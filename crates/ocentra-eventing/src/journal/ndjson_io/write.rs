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
        let created = match tokio::fs::metadata(&self.path).await {
            Ok(_) => false,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
            Err(error) => return Err(EventingError::journal_io(self.path_string(), &error)),
        };
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        if self
            .partial_write_failure_for_debug
            .swap(false, std::sync::atomic::Ordering::AcqRel)
        {
            file.write_all(&line[..line.len() / 2])
                .await
                .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
            file.sync_all()
                .await
                .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
            return Err(EventingError::journal_io(
                self.path_string(),
                &std::io::Error::other("injected partial journal write failure"),
            ));
        }
        file.write_all(&line)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        if self.options.flush == JournalFlushPolicy::Always {
            self.sync_file(&file).await?;
            if created {
                self.sync_parent_directory().await?;
            }
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
        self.sync_file(&file).await?;
        self.sync_parent_directory().await
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

    async fn sync_parent_directory(&self) -> Result<(), EventingError> {
        if self
            .directory_sync_failure_for_debug
            .swap(false, std::sync::atomic::Ordering::AcqRel)
        {
            return Err(EventingError::journal_io(
                self.path_string(),
                &std::io::Error::other("injected journal directory sync failure"),
            ));
        }
        sync_parent_directory(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))
    }
}

#[cfg(unix)]
async fn sync_parent_directory(path: &std::path::Path) -> std::io::Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| std::io::Error::other("journal path has no parent directory"))?;
    File::open(parent).await?.sync_all().await
}

#[cfg(windows)]
async fn sync_parent_directory(_path: &std::path::Path) -> std::io::Result<()> {
    Ok(())
}
