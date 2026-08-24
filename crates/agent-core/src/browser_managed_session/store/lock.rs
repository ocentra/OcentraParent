use std::{
    io::ErrorKind,
    thread,
    time::{Duration, Instant},
};

use fs2::FileExt;

use super::super::{BrowserManagedProfileStoreError, BrowserManagedProfileStorePaths};
use super::path_guards::ProfileStorePathGuards;

const LOCK_WAIT_LIMIT: Duration = Duration::from_secs(5);
const LOCK_RETRY_INTERVAL: Duration = Duration::from_millis(10);

pub(crate) fn with_profile_store_lock<T>(
    paths: &BrowserManagedProfileStorePaths,
    operation: impl FnOnce(&ProfileStorePathGuards) -> Result<T, BrowserManagedProfileStoreError>,
) -> Result<T, BrowserManagedProfileStoreError> {
    let guards = ProfileStorePathGuards::open(paths)?;
    acquire_with_deadline(guards.lock_file())?;
    let result = operation(&guards);
    let validation = guards.validate();
    let unlock = FileExt::unlock(guards.lock_file());
    match (result, validation, unlock) {
        (Ok(value), Ok(()), Ok(())) => Ok(value),
        (Ok(_), Err(error), _) => Err(error),
        (Ok(_), Ok(()), Err(_)) => Err(BrowserManagedProfileStoreError::Io),
        (Err(error), _, _) => Err(error),
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
