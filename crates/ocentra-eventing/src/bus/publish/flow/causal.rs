use crate::bus::dispatch_chain::DispatchChain;
use crate::bus::reports::dead_letters_for;
use crate::bus::reports::handler::{HandlerOutcome, PublishReport};
use crate::bus::{DispatchMode, EventBus};
use crate::{DomainEvent, EventEnvelope, EventMetadata, EventingError, QueueDisposition};

use super::dispatching;

pub(super) async fn publish_with_mode<E>(
    bus: &EventBus,
    event: E,
    metadata: EventMetadata,
    dispatch_mode: DispatchMode,
    dispatch_chain: DispatchChain,
) -> Result<PublishReport, EventingError>
where
    E: DomainEvent,
{
    bus.ensure_active()?;
    dispatch_chain.ensure_current_handler_task()?;
    dispatch_chain.ensure_live()?;
    let stored = EventEnvelope::from_event(event, metadata)?.store()?;
    let subscribers = bus.subscribers_for(&stored);
    if stored.is_deadline_expired(bus.clock.now())
        || subscribers.is_empty()
        || bus.causal_publication_requires_root(&stored)
    {
        return Err(EventingError::CausalPublicationRequiresRootAuthority {
            event_type: stored.contract.event_type,
        });
    }

    let _active_dispatch = bus.active_dispatches.enter();
    let ordered_admission = if dispatch_mode == DispatchMode::OrderedByAggregateKey {
        Some(
            dispatch_chain
                .admit_ordered(bus, stored.aggregate_key.clone())
                .await?,
        )
    } else {
        None
    };
    let reservation = bus.queue.reserve_dispatch(&stored)?;
    // CLONE-JUSTIFICATION: dispatch consumes one immutable causal snapshot;
    // the original remains the authority checked by the final commit.
    let handler_reports = dispatching::dispatch(
        bus,
        stored.clone(),
        subscribers.clone(),
        dispatch_mode,
        dispatch_chain.clone(),
        ordered_admission.as_ref(),
    )
    .await;
    let dead_letters = dead_letters_for(&stored, &handler_reports);
    let dead_letter_count = dead_letters.len();
    bus.commit_causal_effects(&dispatch_chain, &stored, dead_letters)
        .await?;
    // CANCEL-SAFE: completion and report construction contain no await after
    // the atomic causal effect commit.
    reservation.complete();
    Ok(PublishReport {
        event_id: stored.event_id,
        event_type: stored.contract.event_type,
        dispatch_mode,
        queue_report: bus.queue.report(QueueDisposition::Dispatched),
        subscriber_count: subscribers.len(),
        handled_count: handler_reports
            .iter()
            .filter(|report| report.outcome == HandlerOutcome::Handled)
            .count(),
        dead_letter_count,
        handler_reports,
        journal_appends: Vec::new(),
    })
}
