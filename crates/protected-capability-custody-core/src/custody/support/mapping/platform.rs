use super::super::super::CustodyError;
use crate::authority::AuthorityError;
use crate::platform::PlatformError;

pub(super) fn platform_error(error: PlatformError) -> CustodyError {
    match error {
        PlatformError::Unavailable
        | PlatformError::DeploymentRequired
        | PlatformError::InvalidAttestation => CustodyError::Unavailable,
        PlatformError::Rejected => CustodyError::BrokerRejected,
        PlatformError::Tampered | PlatformError::AntiRollback => CustodyError::Tampered,
        PlatformError::WrongBinding => CustodyError::WrongBinding,
        PlatformError::Rotated => CustodyError::Rotated,
        PlatformError::Conflict => CustodyError::Conflict,
    }
}

pub(super) fn authority_error(error: AuthorityError) -> CustodyError {
    match error {
        AuthorityError::Unavailable => CustodyError::Unavailable,
        AuthorityError::Rejected => CustodyError::WrongBinding,
    }
}
