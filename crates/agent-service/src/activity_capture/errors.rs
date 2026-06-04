use ocentra_parent_agent_core::{
    ActivityStoreError, AppGameLiveForegroundWindowError, AppGameLiveInventorySourceError,
    AppGameLiveProcessSnapshotError, AppGameLiveStorePackageSourceError, JournalError,
};
use ocentra_parent_agent_protocol::constants;

#[derive(Debug, PartialEq, Eq)]
pub enum ActivityCaptureError {
    Store,
    Journal,
    Io,
    InvalidKeyLength,
    AppGameRuntime,
}

impl ActivityCaptureError {
    pub fn reason(&self) -> &'static str {
        match self {
            Self::Store => constants::value::ACTIVITY_CAPTURE_STORE_ERROR,
            Self::Journal => constants::value::ACTIVITY_CAPTURE_JOURNAL_ERROR,
            Self::Io => constants::value::ACTIVITY_CAPTURE_IO_ERROR,
            Self::InvalidKeyLength => constants::value::ACTIVITY_CAPTURE_INVALID_KEY_LENGTH,
            Self::AppGameRuntime => constants::value::ACTIVITY_CAPTURE_APP_GAME_ERROR,
        }
    }
}

impl From<ActivityStoreError> for ActivityCaptureError {
    fn from(_: ActivityStoreError) -> Self {
        Self::Store
    }
}

impl From<JournalError> for ActivityCaptureError {
    fn from(_: JournalError) -> Self {
        Self::Journal
    }
}

impl From<std::io::Error> for ActivityCaptureError {
    fn from(_: std::io::Error) -> Self {
        Self::Io
    }
}

impl From<AppGameLiveProcessSnapshotError> for ActivityCaptureError {
    fn from(_: AppGameLiveProcessSnapshotError) -> Self {
        Self::AppGameRuntime
    }
}

impl From<AppGameLiveForegroundWindowError> for ActivityCaptureError {
    fn from(_: AppGameLiveForegroundWindowError) -> Self {
        Self::AppGameRuntime
    }
}

impl From<AppGameLiveInventorySourceError> for ActivityCaptureError {
    fn from(_: AppGameLiveInventorySourceError) -> Self {
        Self::AppGameRuntime
    }
}

impl From<AppGameLiveStorePackageSourceError> for ActivityCaptureError {
    fn from(_: AppGameLiveStorePackageSourceError) -> Self {
        Self::AppGameRuntime
    }
}
