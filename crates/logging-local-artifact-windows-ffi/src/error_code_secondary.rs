use super::{ArtifactError, ErrorCode};

const TARGET_IDENTITY_DRIFT: &str = "target-identity-drift";
const LOCK_CONFLICT: &str = "lock-conflict";
const NOT_FOUND: &str = "not-found";
const ALREADY_EXISTS: &str = "already-exists";
const SIZE_LIMIT: &str = "size-limit";
const DURABILITY_FAILURE: &str = "durability-failure";
const RECOVERY_REQUIRED: &str = "recovery-required";
const IO_FAILURE: &str = "io-failure";

pub(super) fn secondary(error: &ArtifactError) -> Option<ErrorCode> {
    match error {
        ArtifactError::OwnershipChanged => Some(ErrorCode(TARGET_IDENTITY_DRIFT)),
        ArtifactError::LockConflict => Some(ErrorCode(LOCK_CONFLICT)),
        ArtifactError::NotFound => Some(ErrorCode(NOT_FOUND)),
        ArtifactError::AlreadyExists => Some(ErrorCode(ALREADY_EXISTS)),
        ArtifactError::SizeLimit => Some(ErrorCode(SIZE_LIMIT)),
        ArtifactError::DurabilityFailure | ArtifactError::DurabilityFailureWith(_) => {
            Some(ErrorCode(DURABILITY_FAILURE))
        }
        ArtifactError::RecoveryRequired => Some(ErrorCode(RECOVERY_REQUIRED)),
        ArtifactError::Io(_) => Some(ErrorCode(IO_FAILURE)),
        _ => None,
    }
}
