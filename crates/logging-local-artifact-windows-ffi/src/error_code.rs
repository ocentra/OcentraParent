use super::{ArtifactError, ErrorCode};

const UNSUPPORTED_PROVIDER: &str = "unsupported-provider";
const UNSUPPORTED_OPERATION: &str = "unsupported-operation";
const CONTAINMENT_FAILURE: &str = "containment-failure";
const INVALID_REQUEST_ID: &str = "invalid-request-id";
const REQUEST_ID_CONFLICT: &str = "request-id-conflict";
const ROOT_IDENTITY_DRIFT: &str = "root-identity-drift";
const ANCESTOR_IDENTITY_DRIFT: &str = "ancestor-identity-drift";
const REPARSE_OR_SYMLINK: &str = "reparse-or-symlink";
const HARDLINK_DETECTED: &str = "hardlink-detected";

pub(super) fn primary(error: &ArtifactError) -> Option<ErrorCode> {
    match error {
        ArtifactError::UnsupportedPlatform => Some(ErrorCode(UNSUPPORTED_PROVIDER)),
        ArtifactError::UnsupportedOperation(_) => Some(ErrorCode(UNSUPPORTED_OPERATION)),
        ArtifactError::InvalidPath(_) => Some(ErrorCode(CONTAINMENT_FAILURE)),
        ArtifactError::InvalidRequestId => Some(ErrorCode(INVALID_REQUEST_ID)),
        ArtifactError::RequestIdConflict => Some(ErrorCode(REQUEST_ID_CONFLICT)),
        ArtifactError::RootIdentityChanged => Some(ErrorCode(ROOT_IDENTITY_DRIFT)),
        ArtifactError::AncestorIdentityChanged => Some(ErrorCode(ANCESTOR_IDENTITY_DRIFT)),
        ArtifactError::LinkOrReparseDetected => Some(ErrorCode(REPARSE_OR_SYMLINK)),
        ArtifactError::HardlinkDetected => Some(ErrorCode(HARDLINK_DETECTED)),
        _ => None,
    }
}
