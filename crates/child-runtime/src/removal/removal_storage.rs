#[cfg(not(windows))]
use std::fs;
use std::fs::OpenOptions;
use std::io;

use atomicwrites::{AllowOverwrite, AtomicFile};
use fs2::FileExt;

use super::{removal_record::ChildAgentRemovalRecord, ChildAgentRemovalBoundary};

impl ChildAgentRemovalBoundary {
    pub(super) fn with_locked_record<T>(
        &self,
        operation: impl FnOnce(&mut ChildAgentRemovalRecord) -> io::Result<T>,
    ) -> io::Result<T> {
        let lock_path = self.path.with_extension("lock");
        let lock = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .truncate(false)
            .open(lock_path)?;
        lock.lock_exclusive()?;
        let mut record = self.read_record_unlocked()?;
        let result = match operation(&mut record) {
            Ok(value) => self.write_record(&record).map(|()| value),
            Err(error) => Err(error),
        };
        let unlock_result = FileExt::unlock(&lock);
        match (result, unlock_result) {
            (Ok(value), Ok(())) => Ok(value),
            (Err(error), _) => Err(error),
            (Ok(_), Err(error)) => Err(error),
        }
    }

    fn write_record(&self, record: &ChildAgentRemovalRecord) -> io::Result<()> {
        AtomicFile::new(&self.path, AllowOverwrite)
            .write(|file| {
                serde_json::to_writer(&mut *file, record).map_err(io::Error::other)?;
                file.sync_all()
            })
            .map_err(|error| io::Error::other(error.to_string()))?;
        #[cfg(not(windows))]
        if let Some(parent) = self.path.parent() {
            fs::File::open(parent)?.sync_all()?;
        }
        Ok(())
    }
}
