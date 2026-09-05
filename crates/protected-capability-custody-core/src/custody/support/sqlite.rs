use std::sync::MutexGuard;

use rusqlite::Connection;

use super::super::{CustodyError, CustodyStore};
use crate::storage::StorageError;

pub(in crate::custody) fn lock_connection(
    store: &CustodyStore,
) -> Result<MutexGuard<'_, Connection>, CustodyError> {
    store
        .connection
        .lock()
        .map_err(|_poison_error| CustodyError::Conflict)
}

/// Call only after dropping every SQLite guard/transaction. This revalidates
/// the main file, parent, journal identity, absent WAL/SHM, and the required
/// zero-length quiescent PERSIST journal even when the SQLite step failed.
pub(in crate::custody) fn finish_step<T>(
    store: &CustodyStore,
    result: Result<T, CustodyError>,
) -> Result<T, CustodyError> {
    match store
        .secured_path
        .revalidate()
        .map_err(|error| super::map_path_error(&error))
    {
        Err(path_error) => Err(path_error),
        Ok(()) => result,
    }
}

pub(in crate::custody) fn map_error(error: &StorageError) -> CustodyError {
    super::mapping::map_storage_error(error)
}
