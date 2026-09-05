use super::ActivityCaptureError;
use crate::activity_capture_persistence::error::ActivityCapturePersistenceError;

impl From<ActivityCapturePersistenceError> for ActivityCaptureError {
    fn from(error: ActivityCapturePersistenceError) -> Self {
        match error {
            ActivityCapturePersistenceError::Store => Self::Store,
            ActivityCapturePersistenceError::Journal => Self::Journal,
            ActivityCapturePersistenceError::Io => Self::Io,
            ActivityCapturePersistenceError::InvalidKeyLength => Self::InvalidKeyLength,
        }
    }
}
