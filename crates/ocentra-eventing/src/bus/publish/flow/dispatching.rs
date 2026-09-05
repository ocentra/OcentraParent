use std::sync::Arc;

use crate::bus::dispatch::{dispatch_concurrent, dispatch_sequential};
use crate::bus::dispatch_chain::{DispatchChain, OrderedDispatchAdmission};
use crate::bus::publish::flow::{
    receipt::validate_before_dispatch_receipt, BeforeDispatchReceiptValidator,
};
use crate::bus::publisher::EventPublisher;
use crate::bus::reports::dead_letter::{DeadLetter, DeadLetterReason};
use crate::bus::reports::empty_publish_report;
use crate::bus::{DispatchMode, EventBus, SubscriberRecord};
use crate::journal::policy::JournalDispatchPhase;
use crate::queue::state::NoSubscriberQueueDecision;
use crate::{EventingError, ExpectValue, PublishReport, QueueDisposition, StoredEventEnvelope};

use super::ordered;
use super::queued;

pub(super) async fn publish_without_subscribers(
    bus: &EventBus,
    stored: StoredEventEnvelope,
    dispatch_mode: DispatchMode,
    validator: Option<BeforeDispatchReceiptValidator>,
) -> Result<PublishReport, EventingError> {
    match bus
        .queue
        .enqueue_no_subscriber(stored.clone(), bus.clock.now())?
    {
        NoSubscriberQueueDecision::Dispatch(queue_report) => {
            let reservation = bus.queue.reserve_dispatch(&stored)?;
            bus.record_stored_snapshot(&stored).await;
            let mut report = empty_publish_report(&stored, dispatch_mode, queue_report, 0);
            let append = bus
                .append_journal_phase(&stored, JournalDispatchPhase::BeforeDispatch)
                .await?;
            validate_before_dispatch_receipt(validator, append.as_ref())?;
            if let Some(append) = append {
                report.journal_appends.push(append);
            }
            // No handler observed this event. An AfterDispatch record authorizes
            // action replay, so it must only exist after an actual dispatch.
            reservation.complete();
            Ok(report)
        }
        NoSubscriberQueueDecision::Queued(queue_report) => {
            queued::publish_queued(bus, stored, dispatch_mode, queue_report).await
        }
        NoSubscriberQueueDecision::QueuedWithDeadLetter(queue_report, dropped, reason, error) => {
            queued::publish_with_dropped_dead_letter(
                bus,
                stored,
                dispatch_mode,
                queue_report,
                dropped,
                reason,
                error,
            )
            .await
        }
        NoSubscriberQueueDecision::DeadLetter(queue_report, reason, error) => {
            let reservation = bus.queue.reserve_dispatch(&stored)?;
            bus.record_stored_snapshot(&stored).await;
            let dead_letter = DeadLetter::for_queue(&stored, reason, error);
            reservation.complete();
            bus.record_dead_letter(dead_letter).await;
            Ok(empty_publish_report(
                &stored,
                dispatch_mode,
                queue_report,
                1,
            ))
        }
    }
}

pub(super) async fn dead_letter_expired_deadline(
    bus: &EventBus,
    stored: StoredEventEnvelope,
    dispatch_mode: DispatchMode,
) -> Result<PublishReport, EventingError> {
    let reservation = bus.queue.reserve_dispatch(&stored)?;
    bus.record_stored_snapshot(&stored).await;
    let dead_letter = DeadLetter::for_queue(
        &stored,
        DeadLetterReason::DeadlineExpired,
        EventingError::EventDeadlineExpired {
            event_type: stored.contract.event_type.clone(),
        },
    );
    reservation.complete();
    bus.record_dead_letter(dead_letter).await;
    Ok(empty_publish_report(
        &stored,
        dispatch_mode,
        bus.queue
            .report(QueueDisposition::DeadLetteredDeadlineExpired),
        1,
    ))
}

pub(super) async fn dispatch(
    bus: &EventBus,
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
    dispatch_mode: DispatchMode,
    dispatch_chain: DispatchChain,
    ordered_admission: Option<&OrderedDispatchAdmission>,
) -> Vec<crate::bus::reports::handler::HandlerReport> {
    match dispatch_mode {
        DispatchMode::Sequential => {
            let publisher = EventPublisher::for_dispatch(bus.clone(), dispatch_chain, &stored);
            dispatch_sequential(
                stored,
                subscribers,
                publisher,
                bus.handler_policy.clone(),
                Arc::clone(&bus.clock),
            )
            .await
        }
        DispatchMode::Concurrent => {
            let publisher = EventPublisher::for_dispatch(bus.clone(), dispatch_chain, &stored);
            dispatch_concurrent(
                stored,
                subscribers,
                publisher,
                bus.handler_policy.clone(),
                Arc::clone(&bus.clock),
            )
            .await
        }
        DispatchMode::OrderedByAggregateKey => {
            ordered::dispatch(
                bus,
                stored,
                subscribers,
                ordered_admission.expect_value("ordered dispatch admission"),
            )
            .await
        }
    }
}
