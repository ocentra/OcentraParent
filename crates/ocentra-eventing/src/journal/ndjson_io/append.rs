use std::sync::Arc;

use crate::journal::{EventJournal, JournalAppendFuture};
use crate::{EventingError, ExpectValue, JournalDispatchPhase, StoredEventEnvelope};

use super::NdjsonEventJournal;

impl NdjsonEventJournal {
    async fn append_entry(
        &self,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<crate::JournalAppend, EventingError> {
        let _append_permit = Arc::clone(&self.append_gate)
            .acquire_owned()
            .await
            .expect_value("journal append gate remains open");
        let _append_file_lock = self.acquire_append_file_lock().await?;
        self.prepare_append_state().await?;
        self.append_entry_with_gate(envelope, phase).await
    }
}

impl EventJournal for NdjsonEventJournal {
    fn append<'a>(&'a self, envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            self.append_entry(envelope, JournalDispatchPhase::AfterDispatch)
                .await
        })
    }

    fn append_phase<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> JournalAppendFuture<'a> {
        Box::pin(async move { self.append_entry(envelope, phase).await })
    }

    fn append_phase_idempotent<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            self.append_phase_idempotent_by_event_id(envelope, phase)
                .await
        })
    }
}
