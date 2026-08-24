use std::{
    fs::OpenOptions,
    io::ErrorKind,
    thread,
    time::{Duration, Instant},
};

use fs2::FileExt;

use super::super::{BrowserManagedProfileStoreError, BrowserManagedProfileStorePaths};

const LOCK_WAIT_LIMIT: Duration = Duration::from_secs(5);
const LOCK_RETRY_INTERVAL: Duration = Duration::from_millis(10);

pub(crate) fn with_profile_store_lock<T>(
    paths: &BrowserManagedProfileStorePaths,
    operation: impl FnOnce() -> Result<T, BrowserManagedProfileStoreError>,
) -> Result<T, BrowserManagedProfileStoreError> {
    super::validation::validate_path_chain_for_lock(&paths.lock_path)?;
    let lock = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(&paths.lock_path)
        .map_err(|_error| BrowserManagedProfileStoreError::Io)?;
    acquire_with_deadline(&lock)?;
    let result = operation();
    match (result, FileExt::unlock(&lock)) {
        (Ok(value), Ok(())) => Ok(value),
        (Ok(_), Err(_)) => Err(BrowserManagedProfileStoreError::Io),
        (Err(error), _) => Err(error),
    }
}

fn acquire_with_deadline(lock: &std::fs::File) -> Result<(), BrowserManagedProfileStoreError> {
    let deadline = Instant::now()
        .checked_add(LOCK_WAIT_LIMIT)
        .unwrap_or_else(Instant::now);
    while Instant::now() < deadline {
        if try_acquire(lock)? {
            return Ok(());
        }
        thread::sleep(LOCK_RETRY_INTERVAL);
    }
    if try_acquire(lock)? {
        Ok(())
    } else {
        Err(BrowserManagedProfileStoreError::StoreBusy)
    }
}

fn try_acquire(lock: &std::fs::File) -> Result<bool, BrowserManagedProfileStoreError> {
    match lock.try_lock_exclusive() {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == ErrorKind::WouldBlock => Ok(false),
        Err(_) => Err(BrowserManagedProfileStoreError::Io),
    }
}
