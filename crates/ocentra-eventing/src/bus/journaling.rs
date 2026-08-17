use crate::{
    DispatchMode, EventingError, JournalAppend, JournalDispatchPhase, QueueDisposition,
    ReplayActionReport, StoredEventEnvelope,
};

use super::{
    reports::{dead_letter::DeadLetter, empty_publish_report, handler::PublishReport},
    EventBus,
};

const IN_MEMORY_STORED_EVENT_LIMIT: usize = 4096;
const IN_MEMORY_DEAD_LETTER_LIMIT: usize = 4096;

impl EventBus {
    pub(super) async fn record_stored_snapshot(&self, stored: &StoredEventEnvelope) {
        let mut stored_journal = self.stored_journal.write().await;
        stored_journal.push(stored.clone());
        trim_retained(&mut stored_journal, IN_MEMORY_STORED_EVENT_LIMIT);
    }

    pub(super) async fn record_dead_letter(&self, dead_letter: DeadLetter) {
        let mut dead_letters = self.dead_letters.write().await;
        dead_letters.push(dead_letter);
        trim_retained(&mut dead_letters, IN_MEMORY_DEAD_LETTER_LIMIT);
    }

    pub(super) async fn record_dead_letters(&self, new_dead_letters: Vec<DeadLetter>) {
        let mut dead_letters = self.dead_letters.write().await;
        dead_letters.extend(new_dead_letters);
        trim_retained(&mut dead_letters, IN_MEMORY_DEAD_LETTER_LIMIT);
    }

    pub(super) async fn append_journal_phase(
        &self,
        stored: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<Option<JournalAppend>, EventingError> {
        if !self.journal_policy.should_append(stored, phase) {
            return Ok(None);
        }
        if let Some(journal) = &self.event_journal {
            return journal
                .append_phase_idempotent(stored, phase)
                .await
                .map(Some);
        }
        Ok(None)
    }

    pub async fn replay_to_handlers(
        &self,
        report: ReplayActionReport,
        dispatch_mode: DispatchMode,
    ) -> Result<Vec<PublishReport>, EventingError> {
        let mut reports = Vec::new();
        for record in report.records() {
            let envelope = record.envelope.clone();
            let subscribers = self.subscribers_for(&envelope);
            if subscribers.is_empty() {
                reports.push(empty_publish_report(
                    &envelope,
                    dispatch_mode,
                    self.queue.report(QueueDisposition::Dispatched),
                    0,
                ));
                continue;
            }
            reports.push(
                self.dispatch_stored(
                    envelope,
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

fn trim_retained<T>(values: &mut Vec<T>, limit: usize) {
    if values.len() > limit {
        values.drain(0..(values.len() - limit));
    }
}
