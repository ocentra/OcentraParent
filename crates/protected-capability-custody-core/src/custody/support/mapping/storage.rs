use super::super::super::CustodyError;
use crate::storage::StorageError;

pub(super) fn error(error: &StorageError) -> CustodyError {
    match error {
        StorageError::Unavailable => CustodyError::Unavailable,
        StorageError::Sql(_) => CustodyError::Database,
        StorageError::Tampered => CustodyError::Tampered,
        StorageError::IllegalTransition => CustodyError::Conflict,
    }
}
