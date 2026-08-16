use crate::bus::reports::dead_letter::{DeadLetter, DeadLetterReason};
use crate::bus::reports::empty_publish_report;
use crate::bus::{DispatchMode, EventBus};
use crate::queue::state::QueuedEnvelope;
use crate::{EventingError, JournalDispatchPhase, PublishReport, StoredEventEnvelope};

pub(super) async fn publish_queued(
    bus: &EventBus,
    stored: StoredEventEnvelope,
    dispatch_mode: DispatchMode,
    queue_report: crate::QueueReport,
) -> Result<PublishReport, EventingError> {
    let journal_append = match bus
        .append_journal_phase(&stored, JournalDispatchPhase::BeforeDispatch)
        .await
    {
        Ok(append) => append,
        Err(error) => {
            bus.queue.rollback_queued(&stored.event_id);
            return Err(error);
        }
    };
    bus.record_stored_snapshot(&stored).await;
    let mut report = empty_publish_report(&stored, dispatch_mode, queue_report, 0);
    if let Some(append) = journal_append {
        report.journal_appends.push(append);
    }
    Ok(report)
}

pub(super) async fn publish_with_dropped_dead_letter(
    bus: &EventBus,
    stored: StoredEventEnvelope,
    dispatch_mode: DispatchMode,
    queue_report: crate::QueueReport,
    dropped: Box<QueuedEnvelope>,
    reason: DeadLetterReason,
    error: EventingError,
) -> Result<PublishReport, EventingError> {
    let dropped = *dropped;
    let journal_append = match bus
        .append_journal_phase(&stored, JournalDispatchPhase::BeforeDispatch)
        .await
    {
        Ok(append) => append,
        Err(error) => {
            bus.queue.rollback_overflow(&stored.event_id, dropped);
            return Err(error);
        }
    };
    bus.record_stored_snapshot(&stored).await;
    let dead_letter = DeadLetter::for_queue(&dropped.stored, reason, error);
    bus.queue.mark_completed(
        &dropped.stored.event_id,
        dropped.stored.idempotency_key.clone(),
    );
    bus.record_dead_letter(dead_letter).await;
    let mut report = empty_publish_report(&stored, dispatch_mode, queue_report, 1);
    if let Some(append) = journal_append {
        report.journal_appends.push(append);
    }
    Ok(report)
}
