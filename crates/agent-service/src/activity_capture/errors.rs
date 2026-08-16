use ocentra_parent_agent_core::{
    activity_store_app_game::{
        AppGameLiveForegroundWindowError, AppGameLiveInventorySourceError,
        AppGameLiveProcessSnapshotError, AppGameLiveRegistryInventorySourceError,
        AppGameLiveStorePackageSourceError,
    },
    activity_store_error::ActivityStoreError,
    journal_error::JournalError,
};
use ocentra_parent_agent_protocol::constants;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ActivityCaptureError {
    Store,
    Journal,
    Io,
    InvalidKeyLength,
    AppGameRuntime,
    ScreenAiEventRuntime,
}

impl ActivityCaptureError {
    pub fn reason(&self) -> ActivityCaptureReasonText {
        const REASONS: &[(ActivityCaptureError, &str)] = &[
            (
                ActivityCaptureError::Store,
                constants::value::ACTIVITY_CAPTURE_STORE_ERROR,
            ),
            (
                ActivityCaptureError::Journal,
                constants::value::ACTIVITY_CAPTURE_JOURNAL_ERROR,
            ),
            (
                ActivityCaptureError::Io,
                constants::value::ACTIVITY_CAPTURE_IO_ERROR,
            ),
            (
                ActivityCaptureError::InvalidKeyLength,
                constants::value::ACTIVITY_CAPTURE_INVALID_KEY_LENGTH,
            ),
            (
                ActivityCaptureError::AppGameRuntime,
                constants::value::ACTIVITY_CAPTURE_APP_GAME_ERROR,
            ),
            (
                ActivityCaptureError::ScreenAiEventRuntime,
                constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBER_REJECTS,
            ),
        ];

        REASONS
            .iter()
            .find(|(error, _)| error == self)
            .map(|(_, reason)| ActivityCaptureReasonText(reason))
            .unwrap_or(ActivityCaptureReasonText(
                constants::value::ACTIVITY_CAPTURE_APP_GAME_ERROR,
            ))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActivityCaptureReasonText(pub &'static str);

impl fmt::Display for ActivityCaptureReasonText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
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

impl From<AppGameLiveRegistryInventorySourceError> for ActivityCaptureError {
    fn from(_: AppGameLiveRegistryInventorySourceError) -> Self {
        Self::AppGameRuntime
    }
}
