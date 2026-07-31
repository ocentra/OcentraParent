use crate::{
    DomainEvent, EventEnvelope, EventMetadata, EventingError, ExpectValue, JournalDispatchPhase,
    PublishReport, QueueDisposition, StoredEventEnvelope,
};

use super::{DispatchMode, DispatchStoredError, EventBus, SubscriberRecord};
use crate::bus::reports::dead_letters_for;
use crate::bus::reports::handler::{HandlerOutcome, HandlerReport};
use receipt::validate_before_dispatch_receipt;

mod dispatching;
mod receipt;

type BeforeDispatchReceiptValidator = fn(&crate::JournalAppend) -> Result<(), EventingError>;

pub(super) async fn publish_with_mode<E>(
    bus: &EventBus,
    event: E,
    metadata: EventMetadata,
    dispatch_mode: DispatchMode,
) -> Result<PublishReport, EventingError>
where
    E: DomainEvent,
{
    bus.ensure_active()?;
    let stored = EventEnvelope::from_event(event, metadata)?.store()?;
    if stored.is_deadline_expired(bus.clock.now()) {
        return dispatching::dead_letter_expired_deadline(bus, stored, dispatch_mode).await;
    }
    let subscribers = bus.subscribers_for(&stored);
    if subscribers.is_empty() {
        return dispatching::publish_without_subscribers(bus, stored, dispatch_mode, None).await;
    }
    bus.dispatch_stored(
        stored,
        subscribers,
        dispatch_mode,
        bus.queue.report(QueueDisposition::Dispatched),
        true,
    )
    .await
}

pub(super) async fn publish_with_mode_and_before_dispatch_receipt_validator<E>(
    bus: &EventBus,
    event: E,
    metadata: EventMetadata,
    dispatch_mode: DispatchMode,
    validator: BeforeDispatchReceiptValidator,
) -> Result<PublishReport, EventingError>
where
    E: DomainEvent,
{
    bus.ensure_active()?;
    let stored = EventEnvelope::from_event(event, metadata)?.store()?;
    if stored.is_deadline_expired(bus.clock.now()) {
        return dispatching::dead_letter_expired_deadline(bus, stored, dispatch_mode).await;
    }
    let subscribers = bus.subscribers_for(&stored);
    if subscribers.is_empty() {
        return dispatching::publish_without_subscribers(
            bus,
            stored,
            dispatch_mode,
            Some(validator),
        )
        .await;
    }
    bus.dispatch_stored_with_before_dispatch_receipt_validator(
        stored,
        subscribers,
        dispatch_mode,
        bus.queue.report(QueueDisposition::Dispatched),
        true,
        validator,
    )
    .await
}

impl EventBus {
    pub(crate) async fn dispatch_stored(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::QueueReport,
        write_journal: bool,
    ) -> Result<PublishReport, EventingError> {
        self.dispatch_stored_checked(
            stored,
            subscribers,
            dispatch_mode,
            queue_report,
            write_journal,
        )
        .await
        .map_err(DispatchStoredError::into_error)
    }

    async fn dispatch_stored_with_before_dispatch_receipt_validator(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::QueueReport,
        write_journal: bool,
        validator: BeforeDispatchReceiptValidator,
    ) -> Result<PublishReport, EventingError> {
        self.dispatch_stored_checked_with_before_dispatch_receipt_validator(
            stored,
            subscribers,
            dispatch_mode,
            queue_report,
            write_journal,
            Some(validator),
        )
        .await
        .map_err(DispatchStoredError::into_error)
    }

    pub(crate) async fn dispatch_stored_checked(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::QueueReport,
        write_journal: bool,
    ) -> Result<PublishReport, DispatchStoredError> {
        self.dispatch_stored_checked_with_before_dispatch_receipt_validator(
            stored,
            subscribers,
            dispatch_mode,
            queue_report,
            write_journal,
            None,
        )
        .await
    }

    async fn dispatch_stored_checked_with_before_dispatch_receipt_validator(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::QueueReport,
        write_journal: bool,
        validator: Option<BeforeDispatchReceiptValidator>,
    ) -> Result<PublishReport, DispatchStoredError> {
        let reservation = self.queue.reserve_dispatch(&stored)?;
        let _active_dispatch = self.active_dispatches.enter();
        if write_journal {
            self.record_stored_snapshot(&stored).await;
        }
        let mut journal_appends = Vec::new();
        let append = self
            .append_journal_phase(&stored, JournalDispatchPhase::BeforeDispatch)
            .await
            .map_err(DispatchStoredError::BeforeDispatch)?;
        validate_before_dispatch_receipt(validator, append.as_ref())
            .map_err(DispatchStoredError::BeforeDispatch)?;
        if let Some(append) = append {
            journal_appends.push(append);
        }
        let handler_reports = self
            .dispatch(stored.clone(), subscribers.clone(), dispatch_mode)
            .await;
        reservation.complete();
        let dead_letters = dead_letters_for(&stored, &handler_reports);
        if !dead_letters.is_empty() {
            self.record_dead_letters(dead_letters.clone()).await;
        }
        if let Some(append) = self
            .append_journal_phase(&stored, JournalDispatchPhase::AfterDispatch)
            .await
            .map_err(DispatchStoredError::AfterDispatch)?
        {
            journal_appends.push(append);
        }
        Ok(PublishReport {
            event_id: stored.event_id,
            event_type: stored.contract.event_type,
            dispatch_mode,
            queue_report,
            subscriber_count: subscribers.len(),
            handled_count: handler_reports
                .iter()
                .filter(|report| report.outcome == HandlerOutcome::Handled)
                .count(),
            dead_letter_count: dead_letters.len(),
            handler_reports,
            journal_appends,
        })
    }

    pub(crate) fn subscribers_for(&self, stored: &StoredEventEnvelope) -> Vec<SubscriberRecord> {
        let registry = self.registry.lock().expect_value("event registry lock");
        let subscribers = registry
            .get(&stored.contract.event_type)
            .cloned()
            .unwrap_or_default();
        match &stored.target_handler {
            Some(target) => subscribers
                .into_iter()
                .filter(|subscriber| &subscriber.target_handler == target)
                .collect(),
            None => subscribers,
        }
    }

    async fn dispatch(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
    ) -> Vec<HandlerReport> {
        dispatching::dispatch(self, stored, subscribers, dispatch_mode).await
    }
}
