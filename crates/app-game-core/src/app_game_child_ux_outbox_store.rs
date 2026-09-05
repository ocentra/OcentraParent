use std::{
    fs::{self, OpenOptions},
    io,
    path::{Path, PathBuf},
};

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;
use ocentra_parent_agent_protocol::schema_domain_mirrors::notification::NotificationLocalOutboxRecord;

use crate::app_game_child_ux_outbox_types::AppGameChildUxOutboxPersistResult;

#[derive(Clone, Debug)]
pub struct AppGameChildUxLocalOutboxStore {
    path: PathBuf,
}

impl AppGameChildUxLocalOutboxStore {
    pub fn open(directory: impl AsRef<Path>) -> io::Result<Self> {
        fs::create_dir_all(directory.as_ref())?;
        if fs::symlink_metadata(directory.as_ref())?
            .file_type()
            .is_symlink()
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "child UX outbox directory must not be a symlink",
            ));
        }
        let directory = directory.as_ref().canonicalize()?;
        Ok(Self {
            path: directory.join("app-game-child-ux-local-outbox.json"),
        })
    }

    pub fn records(&self) -> io::Result<Vec<NotificationLocalOutboxRecord>> {
        match fs::read(&self.path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Vec::new()),
            Err(error) => Err(error),
        }
    }

    pub fn persist(
        &self,
        record: NotificationLocalOutboxRecord,
    ) -> io::Result<AppGameChildUxOutboxPersistResult> {
        let lock = self.lock()?;
        lock.lock_exclusive()?;
        let result = self.persist_locked(record);
        FileExt::unlock(&lock)?;
        result
    }

    fn persist_locked(
        &self,
        record: NotificationLocalOutboxRecord,
    ) -> io::Result<AppGameChildUxOutboxPersistResult> {
        let mut records = self.records()?;
        if let Some(existing) = records
            .iter()
            .find(|existing| existing.entry_id == record.entry_id)
        {
            return if existing == &record {
                Ok(AppGameChildUxOutboxPersistResult::AlreadyPresent)
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "conflicting child UX outbox entry id",
                ))
            };
        }
        records.push(record);
        self.write(&records)?;
        Ok(AppGameChildUxOutboxPersistResult::Inserted)
    }

    fn write(&self, records: &[NotificationLocalOutboxRecord]) -> io::Result<()> {
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
