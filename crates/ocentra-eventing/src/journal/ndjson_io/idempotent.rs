use std::sync::Arc;

use crate::journal::{JournalAppend, JournalHashVersion};
use crate::{EventingError, ExpectValue, JournalDispatchPhase, StoredEventEnvelope};

use super::idempotent_match::matching_append;
use super::idempotent_record::decode_entry;
use super::NdjsonEventJournal;

impl NdjsonEventJournal {
    pub async fn append_idempotent(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> Result<JournalAppend, EventingError> {
        let _append_permit = Arc::clone(&self.append_gate)
            .acquire_owned()
            .await
            .expect_value("journal append gate remains open");
        let _append_file_lock = self.acquire_append_file_lock().await?;
        self.prepare_append_state().await?;
        match self.existing_append(envelope).await? {
            Some(append) => {
                self.sync_existing_journal().await?;
                acknowledgement_after_sync(append)
            }
            None => {
                self.append_entry_with_gate(envelope, JournalDispatchPhase::AfterDispatch)
                    .await
            }
        }
    }

    async fn existing_append(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> Result<Option<JournalAppend>, EventingError> {
        let contents = read_journal(self).await?;
        let mut matches = contents
            .lines()
            .enumerate()
            .filter(|(_index, line)| !line.trim().is_empty())
            .filter_map(|(index, line)| decode_entry(line, index + 1).transpose())
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter(|entry| {
                entry.envelope != *envelope || entry.phase == JournalDispatchPhase::AfterDispatch
            })
            .filter_map(|entry| {
                matching_append(entry, envelope, JournalDispatchPhase::AfterDispatch)
            })
            .collect::<Vec<_>>();
        if let Some(index) = matches.iter().position(Result::is_err) {
            return matches.swap_remove(index).map(Some);
        }
        Ok(matches.into_iter().find_map(Result::ok))
    }
}

fn acknowledgement_after_sync(append: JournalAppend) -> Result<JournalAppend, EventingError> {
    match append.hash_version {
        JournalHashVersion::V3 => append.with_synchronization_proof(),
        JournalHashVersion::LegacyV1 | JournalHashVersion::V2 => Ok(append),
    }
}

async fn read_journal(journal: &NdjsonEventJournal) -> Result<String, EventingError> {
    match tokio::fs::read_to_string(&journal.path).await {
        Ok(contents) => Ok(contents),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(EventingError::journal_io(journal.path_string(), &error)),
    }
}
