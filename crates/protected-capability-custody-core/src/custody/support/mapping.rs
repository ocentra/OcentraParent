use super::super::CustodyError;
use crate::platform::PlatformError;
use crate::storage::StorageError;

pub(super) fn map_platform_error(error: PlatformError) -> CustodyError {
    match error {
        PlatformError::Unavailable
        | PlatformError::Rejected
        | PlatformError::InvalidAttestation => CustodyError::Unavailable,
        PlatformError::Tampered | PlatformError::AntiRollback => CustodyError::Tampered,
        PlatformError::WrongBinding => CustodyError::WrongBinding,
        PlatformError::Rotated => CustodyError::Rotated,
        PlatformError::Conflict => CustodyError::Conflict,
    }
}

pub(super) fn map_storage_error(error: StorageError) -> CustodyError {
    match error {
        StorageError::Sql(_) => CustodyError::Database,
        StorageError::Tampered => CustodyError::Tampered,
    }
}
