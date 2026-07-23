use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

use crate::{EventingError, JournalDispatchPhase, StoredEventEnvelope};

use super::{JournalAppend, JournalFlushPolicy, NdjsonEventJournal};

impl NdjsonEventJournal {
    pub(crate) async fn write_entry(
        &self,
        append: &JournalAppend,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<(), EventingError> {
        let entry = super::NdjsonJournalEntry {
            append: append.clone(),
            phase,
            envelope: envelope.clone(),
        };
        let mut line =
            serde_json::to_vec(&entry).map_err(|error| EventingError::journal_encode(&error))?;
        line.push(b'\n');
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        file.write_all(&line)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        if self.options.flush == JournalFlushPolicy::Always {
            self.sync_file(&file).await?;
        }
        Ok(())
    }

    pub(super) async fn sync_existing_journal(&self) -> Result<(), EventingError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        self.sync_file(&file).await
    }

    async fn sync_file(&self, file: &File) -> Result<(), EventingError> {
        if self
            .sync_failure_for_debug
            .swap(false, std::sync::atomic::Ordering::AcqRel)
        {
            return Err(EventingError::journal_io(
                self.path_string(),
                &std::io::Error::other("injected journal sync failure"),
            ));
        }
        file.sync_all()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))
    }
}
