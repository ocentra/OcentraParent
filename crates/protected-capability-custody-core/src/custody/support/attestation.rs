use super::super::CustodyError;
use crate::path_security::{PathSecurityError, SecuredPath};
use crate::platform::{
    DatabasePathSecurity, PlatformAttestation, PlatformDatabaseGuard, SecurityLevel,
};

pub(super) fn attest_path(
    platform: &dyn PlatformDatabaseGuard,
    path: &SecuredPath,
) -> Result<PlatformAttestation, CustodyError> {
    path.revalidate().map_err(super::map_path_error)?;
    let attestation = platform
        .attest_database(path.canonical(), path.identity())
        .map_err(super::map_platform_error)?;
    if attestation.security_level() != SecurityLevel::SameUserIsolated
        || attestation.database_path_security()
            != DatabasePathSecurity::BrokerExclusiveWriterNoFollowRollbackJournal
        || attestation.key_epoch == 0
        || attestation.writer_epoch == 0
        || attestation.database_identity != path.identity()
    {
        return Err(CustodyError::Unavailable);
    }
    Ok(attestation)
}

pub(super) fn map_path_error(error: PathSecurityError) -> CustodyError {
    match error {
        PathSecurityError::UnsupportedPlatform => CustodyError::UnsupportedPlatform,
        PathSecurityError::Unavailable => CustodyError::Unavailable,
        PathSecurityError::UnsafePath => CustodyError::UnsafeDatabasePath,
        PathSecurityError::Replaced => CustodyError::DatabaseReplaced,
    }
}
