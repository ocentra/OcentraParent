use std::sync::Arc;

use crate::journal::{
    hash_chain::{hash_entry_v3, synchronization_receipt_hash},
    EventJournal, JournalAppendDurability, JournalAppendFuture, JournalHashVersion,
};
use crate::{EventingError, ExpectValue, JournalDispatchPhase, JournalHash, StoredEventEnvelope};

use super::{
    JournalAppend, JournalFlushPolicy, JournalHashChain, NdjsonEventJournal, NdjsonJournalOptions,
};

impl NdjsonEventJournal {
    async fn append_entry(
        &self,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<JournalAppend, EventingError> {
        let _append_permit = Arc::clone(&self.append_gate)
            .acquire_owned()
            .await
            .expect_value("journal append gate remains open");
        let _append_file_lock = self.acquire_append_file_lock().await?;
        self.repair_incomplete_trailing_record().await?;
        self.refresh_state_if_unrecovered().await?;
        self.append_entry_with_gate(envelope, phase).await
    }

    pub(super) async fn append_entry_with_gate(
        &self,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<JournalAppend, EventingError> {
        let append = {
            let state = self.state.lock().expect_value("journal state lock");
            let next_sequence = state.next_sequence.saturating_add(1);
            let previous_hash = previous_hash(&self.options, &state);
            let requested_durability = append_durability(self.options.flush);
            // The line is serialized before any file or directory sync can
            // succeed. Its persisted result must therefore fail closed. The
            // returned acknowledgement below may report synchronization only
            // after write_entry has completed its required syncs.
            let durability = JournalAppendDurability::Buffered;
            let current_hash = current_hash(
                &self.options,
                next_sequence,
                &previous_hash,
                envelope,
                phase,
                requested_durability,
                durability,
            )?;
            JournalAppend {
                sequence: next_sequence,
                previous_hash,
                current_hash,
                hash_version: JournalHashVersion::V3,
                durability,
                requested_durability,
                synchronization_hash: None,
            }
        };
        self.write_entry(&append, envelope, phase).await?;
        let file_len = tokio::fs::metadata(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?
            .len();
        {
            let mut state = self.state.lock().expect_value("journal state lock");
            state.next_sequence = append.sequence;
            state.previous_hash = append.current_hash.clone();
            state.recovered = true;
            state.file_len = file_len;
        }
        let mut acknowledgement = append;
        if self.options.flush == JournalFlushPolicy::Always {
            acknowledgement.durability = JournalAppendDurability::Synchronized;
            acknowledgement.synchronization_hash =
                Some(synchronization_receipt_hash(&acknowledgement)?);
        }
        Ok(acknowledgement)
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
}

fn append_durability(flush: JournalFlushPolicy) -> JournalAppendDurability {
    match flush {
        JournalFlushPolicy::Always => JournalAppendDurability::Synchronized,
        JournalFlushPolicy::Buffered => JournalAppendDurability::Buffered,
    }
}

fn previous_hash(
    options: &NdjsonJournalOptions,
    state: &super::super::ndjson_state::NdjsonJournalState,
) -> Option<JournalHash> {
    match options.hash_chain {
        JournalHashChain::Disabled => None,
        JournalHashChain::Enabled => state.previous_hash.clone(),
    }
}

fn current_hash(
    options: &NdjsonJournalOptions,
    sequence: u64,
    previous_hash: &Option<JournalHash>,
    envelope: &StoredEventEnvelope,
    phase: JournalDispatchPhase,
    requested_durability: JournalAppendDurability,
    durability: JournalAppendDurability,
) -> Result<Option<JournalHash>, EventingError> {
    match options.hash_chain {
        JournalHashChain::Disabled => Ok(None),
        JournalHashChain::Enabled => hash_entry_v3(
            sequence,
            previous_hash.as_ref(),
            envelope,
            phase,
            requested_durability,
            durability,
        )
        .map(Some),
    }
}
