use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;

use crate::journal::ndjson::NdjsonJournalRecord;
use crate::journal::ndjson::{
    NdjsonJournalSynchronizationActivation, NdjsonJournalSynchronizationCompletion,
};
use crate::{EventingError, JournalDispatchPhase, StoredEventEnvelope};

use super::{JournalAppend, JournalFlushPolicy, NdjsonEventJournal};

impl NdjsonEventJournal {
    pub(crate) async fn write_entry(
        &self,
        append: &JournalAppend,
        envelope: &StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Result<(), EventingError> {
        let entry = NdjsonJournalRecord::Entry(Box::new(super::NdjsonJournalEntry {
            append: append.clone(),
            phase,
            envelope: envelope.clone(),
        }));
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

impl NdjsonEventJournal {
    pub(super) async fn write_synchronization_completion(
        &self,
        append: &JournalAppend,
    ) -> Result<(), EventingError> {
        let Some(synchronization_hash) = &append.synchronization_hash else {
            return Err(EventingError::InvalidValue {
                field: "journal_append.synchronization_hash",
                value: "V3 completion requires authenticated hashes".to_owned(),
            });
        };
        let completion = NdjsonJournalSynchronizationCompletion {
            sequence: append.sequence,
            entry_hash: append.current_hash.clone(),
            synchronization_hash: synchronization_hash.clone(),
        };
        let mut line = serde_json::to_vec(&completion)
            .map_err(|error| EventingError::journal_encode(&error))?;
        line.push(b'\n');
        let mut file = OpenOptions::new()
            .read(true)
            .append(true)
            .open(&self.path)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        file.write_all(&line)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        file.flush()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        let synchronization = if self
            .synchronization_completion_sync_failure_for_debug
            .swap(false, std::sync::atomic::Ordering::AcqRel)
        {
            Err(EventingError::journal_io(
                self.path_string(),
                &std::io::Error::other("injected synchronization completion sync failure"),
            ))
        } else {
            self.sync_file(&file).await
        };
        synchronization?;
        let activation = NdjsonJournalSynchronizationActivation {
            activation: true,
            sequence: completion.sequence,
            entry_hash: completion.entry_hash,
            synchronization_hash: completion.synchronization_hash,
        };
        let mut activation_line = serde_json::to_vec(&activation)
            .map_err(|error| EventingError::journal_encode(&error))?;
        activation_line.push(b'\n');
        file.write_all(&activation_line)
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        file.flush()
            .await
            .map_err(|error| EventingError::journal_io(self.path_string(), &error))?;
        self.sync_file(&file).await?;
        Ok(())
    }
}

#[cfg(unix)]
async fn sync_parent_directory(path: &std::path::Path) -> std::io::Result<()> {
    let parent = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| std::path::Path::new("."));
    File::open(parent).await?.sync_all().await
}

#[cfg(windows)]
async fn sync_parent_directory(_path: &std::path::Path) -> std::io::Result<()> {
    Ok(())
}
