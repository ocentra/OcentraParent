use crate::{
    DispatchMode, EventingError, JournalDispatchPhase, QueueDisposition, ReplayMode, ReplayRecord,
    StoredEventEnvelope,
};

use super::{
    reports::{empty_publish_report, PublishReport},
    EventBus,
};

const PROJECTION_ONLY_REPLAY_EVENT_TYPE: &str = "projection-only-replay";

impl EventBus {
    pub(super) async fn record_stored_snapshot(&self, stored: &StoredEventEnvelope) {
        self.stored_journal.write().await.push(stored.clone());
    }

    pub(super) async fn append_journal_phase(
        &self,
        stored: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<(), EventingError> {
        if !self.journal_policy.should_append(stored, phase) {
            return Ok(());
        }
        if let Some(journal) = &self.event_journal {
            journal.append(stored).await?;
        }
        Ok(())
    }

    pub async fn replay_to_handlers(
        &self,
        records: Vec<ReplayRecord>,
        mode: ReplayMode,
        dispatch_mode: DispatchMode,
    ) -> Result<Vec<PublishReport>, EventingError> {
        if mode != ReplayMode::ActionHandlersAllowed {
            let event_type = records
                .first()
                .map(|record| record.envelope.contract.event_type.clone())
                .unwrap_or(crate::EventType::parse(PROJECTION_ONLY_REPLAY_EVENT_TYPE)?);
            return Err(EventingError::ReplayActionNotAllowed { event_type });
        }

        let mut reports = Vec::new();
        for record in records {
            let subscribers = self.subscribers_for(&record.envelope);
            if subscribers.is_empty() {
                reports.push(empty_publish_report(
                    &record.envelope,
                    dispatch_mode,
                    self.queue.report(QueueDisposition::Dispatched),
                    0,
                ));
                continue;
            }
            reports.push(
                self.dispatch_stored(
                    record.envelope,
                    subscribers,
                    dispatch_mode,
                    self.queue.report(QueueDisposition::Dispatched),
                    false,
                )
                .await?,
            );
        }
        Ok(reports)
    }
}
