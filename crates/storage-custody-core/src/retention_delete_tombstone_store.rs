use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

use crate::storage_custody::{StorageCustodyActionPlannedEvent, StorageTombstoneState};

const STORE_VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionDeleteOutboxRecord {
    pub version: u16,
    pub deletion_ref: String,
    pub proof_ref: String,
    pub terminal_pending: bool,
}

pub struct RetentionDeleteTombstoneStore {
    path: PathBuf,
}

impl RetentionDeleteTombstoneStore {
    pub fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        fs::create_dir_all(directory.as_ref())?;
        if fs::symlink_metadata(directory.as_ref())?
            .file_type()
            .is_symlink()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "custody store directory must not be a symlink",
            ));
        }
        let directory = directory.as_ref().canonicalize()?;
        Ok(Self {
            path: directory.join("retention-delete-tombstones.json"),
        })
    }

    pub fn records(&self) -> io::Result<Vec<RetentionDeleteOutboxRecord>> {
        match fs::read(&self.path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error),
        }
    }

    pub fn persist_intent(&self, deletion_ref: String, proof_ref: String) -> io::Result<()> {
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let mut records = self.records()?;
        if !records
            .iter()
            .any(|record| record.deletion_ref == deletion_ref)
        {
            records.push(RetentionDeleteOutboxRecord {
                version: STORE_VERSION,
                deletion_ref,
                proof_ref,
                terminal_pending: true,
            });
        }
        let result = self.write(&records);
        FileExt::unlock(&lock)?;
        result
    }

    /// Persists the terminal-publish obligation produced by the typed custody
    /// action event. A non-delete action must never create a tombstone record.
    pub fn persist_action_plan_intent(
        &self,
        action: &StorageCustodyActionPlannedEvent,
    ) -> io::Result<()> {
        if action.action_plan.tombstone_state != StorageTombstoneState::Write {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "a non-delete custody action cannot create a tombstone intent",
            ));
        }

        self.persist_intent(
            format!(
                "storage-custody-delete:{}",
                action.source_decision_id.as_str()
            ),
            action.action_plan_id.as_str().to_owned(),
        )
    }

    pub fn mark_terminal_published(&self, deletion_ref: &str) -> io::Result<()> {
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let mut records = self.records()?;
        records.retain(|record| record.deletion_ref != deletion_ref);
        let result = self.write(&records);
        FileExt::unlock(&lock)?;
        result
    }

    fn write(&self, records: &[RetentionDeleteOutboxRecord]) -> io::Result<()> {
        AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                serde_json::to_writer(&mut *file, records).map_err(io::Error::other)?;
                file.sync_all()
            })
            .map_err(|error| io::Error::other(error.to_string()))?;
        sync_parent_directory(&self.path)
    }

    fn lock(&self) -> io::Result<std::fs::File> {
        OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(self.path.with_extension("lock"))
    }
}

#[cfg(not(windows))]
fn sync_parent_directory(path: &Path) -> io::Result<()> {
    std::fs::File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()
}

#[cfg(windows)]
fn sync_parent_directory(_path: &Path) -> io::Result<()> {
    Ok(())
}
