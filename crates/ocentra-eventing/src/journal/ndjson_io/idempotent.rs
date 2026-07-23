use std::sync::Arc;

use crate::journal::JournalAppend;
use crate::{EventingError, ExpectValue, JournalDispatchPhase, StoredEventEnvelope};

use super::idempotent_match::matching_append;
use super::{NdjsonEventJournal, NdjsonJournalEntry};

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
        self.repair_incomplete_trailing_record().await?;
        self.refresh_state_if_unrecovered().await?;
        match self.existing_append(envelope).await? {
            Some(append) => {
                self.sync_existing_journal().await?;
                Ok(append)
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
        for (index, line) in contents
            .lines()
            .enumerate()
            .filter(|(_index, line)| !line.trim().is_empty())
        {
            let entry = decode_entry(line, index + 1)?;
            if let Some(matched) =
                matching_append(entry, envelope, JournalDispatchPhase::AfterDispatch)
            {
                return matched.map(Some);
            }
        }
        Ok(None)
    }
}

async fn read_journal(journal: &NdjsonEventJournal) -> Result<String, EventingError> {
    match tokio::fs::read_to_string(&journal.path).await {
        Ok(contents) => Ok(contents),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(EventingError::journal_io(journal.path_string(), &error)),
    }
}

fn decode_entry(line: &str, line_number: usize) -> Result<NdjsonJournalEntry, EventingError> {
    serde_json::from_str(line).map_err(|error| EventingError::JournalCorruptLine {
        line: line_number,
        reason: error.to_string(),
    })
}
