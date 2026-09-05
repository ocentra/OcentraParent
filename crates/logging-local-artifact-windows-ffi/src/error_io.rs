use super::ArtifactError;

pub(crate) fn io_error(error: std::io::Error) -> ArtifactError {
    if error.raw_os_error() == Some(33) {
        return ArtifactError::LockConflict;
    }
    match error.kind() {
        std::io::ErrorKind::NotFound => ArtifactError::NotFound,
        std::io::ErrorKind::AlreadyExists => ArtifactError::AlreadyExists,
        std::io::ErrorKind::WouldBlock => ArtifactError::LockConflict,
        std::io::ErrorKind::PermissionDenied => ArtifactError::OwnershipChanged,
        _ => ArtifactError::Io(error.to_string()),
    }
}
