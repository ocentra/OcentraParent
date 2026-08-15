use std::sync::Arc;

use crate::{EventingError, ExpectValue, JournalAppend, JournalDispatchPhase, StoredEventEnvelope};

use super::NdjsonEventJournal;

impl NdjsonEventJournal {
    /// Append an after-dispatch envelope once per event id.
    pub async fn append_idempotent_by_event_id(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> Result<JournalAppend, EventingError> {
        self.append_phase_idempotent_by_event_id(envelope, JournalDispatchPhase::AfterDispatch)
            .await
    }

    pub async fn append_phase_idempotent_by_event_id(
        &self,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<JournalAppend, EventingError> {
        let _append_permit = Arc::clone(&self.append_gate)
            .acquire_owned()
            .await
            .expect_value("journal append gate remains open");
        let _append_file_lock = self.acquire_append_file_lock().await?;
        self.prepare_append_state().await?;
        match self.existing_append_by_event_id(envelope, phase).await? {
            Some(append) => {
                self.sync_existing_journal().await?;
                self.acknowledgement_after_sync(append).await
            }
            None => self.append_entry_with_gate(envelope, phase).await,
        }
    }
}
