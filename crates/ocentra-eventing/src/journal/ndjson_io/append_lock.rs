use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::EventingError;

use super::NdjsonEventJournal;

const LOCK_SUFFIX: &str = ".append.lock";
const LOCK_RETRY_COUNT: usize = 2_000;
const LOCK_RETRY_DELAY: Duration = Duration::from_millis(5);

pub(super) struct JournalAppendLock {
    _file: File,
}

impl NdjsonEventJournal {
    pub(super) async fn acquire_append_file_lock(
        &self,
    ) -> Result<JournalAppendLock, EventingError> {
        acquire(&append_lock_path(&self.path), self).await
    }
}

fn append_lock_path(path: &Path) -> PathBuf {
    let mut file_name = path
        .file_name()
        .map_or_else(|| OsString::from("journal"), OsString::from);
    file_name.push(LOCK_SUFFIX);
    path.with_file_name(file_name)
}

#[cfg(target_os = "windows")]
async fn acquire(
    lock_path: &Path,
    journal: &NdjsonEventJournal,
) -> Result<JournalAppendLock, EventingError> {
    use std::os::windows::fs::OpenOptionsExt;

    for _ in 0..LOCK_RETRY_COUNT {
        let lock_path = lock_path.to_owned();
        let opened = tokio::task::spawn_blocking(move || {
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .share_mode(0)
                .open(lock_path)
        })
        .await
        .map_err(|error| {
            EventingError::journal_io(
                journal.path_string(),
                &std::io::Error::other(error.to_string()),
            )
        })?;
        match opened {
            Ok(file) => return Ok(JournalAppendLock { _file: file }),
            Err(error) if matches!(error.raw_os_error(), Some(32 | 33)) => {
                tokio::time::sleep(LOCK_RETRY_DELAY).await;
            }
            Err(error) => return Err(EventingError::journal_io(journal.path_string(), &error)),
        }
    }
    Err(lock_timeout(journal))
}

#[cfg(not(target_os = "windows"))]
async fn acquire(
    lock_path: &Path,
    journal: &NdjsonEventJournal,
) -> Result<JournalAppendLock, EventingError> {
    let lock_path = lock_path.to_owned();
    let file = tokio::task::spawn_blocking(move || {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(lock_path)
    })
    .await
    .map_err(|error| {
        EventingError::journal_io(
            journal.path_string(),
            &std::io::Error::other(error.to_string()),
        )
    })?
    .map_err(|error| EventingError::journal_io(journal.path_string(), &error))?;
    for _ in 0..LOCK_RETRY_COUNT {
        match file.try_lock() {
            Ok(()) => return Ok(JournalAppendLock { _file: file }),
            Err(std::fs::TryLockError::WouldBlock) => {
                tokio::time::sleep(LOCK_RETRY_DELAY).await;
            }
            Err(std::fs::TryLockError::Error(error)) => {
                return Err(EventingError::journal_io(journal.path_string(), &error));
            }
        }
    }
    Err(lock_timeout(journal))
}

fn lock_timeout(journal: &NdjsonEventJournal) -> EventingError {
    EventingError::journal_io(
        journal.path_string(),
        &std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "journal append lock timed out",
        ),
    )
}
