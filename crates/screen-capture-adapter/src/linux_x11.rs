use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::{
    degraded_capture, ScreenCaptureAttempt, ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};

#[path = "linux_x11_capture.rs"]
mod capture;

pub(super) fn capture_active_window_png() -> ScreenCaptureAttempt {
    // Active-window ownership is intentionally not composed by this adapter.
    capture::capture_x11_window_png(ScreenCaptureScope::ActiveWindow)
}

pub(super) fn capture_window_title_contains_png(
    _title_query: &ScreenCaptureWindowTitleQuery,
) -> ScreenCaptureAttempt {
    // Raw-title matching would cross the metadata boundary, so selected-window
    // capture is explicitly unavailable on Linux.
    degraded_capture(
        ActivityCaptureCapabilityStatus::Unavailable,
        ScreenCaptureScope::SelectedWindow,
    )
}

pub(super) fn capture_primary_display_png() -> ScreenCaptureAttempt {
    capture::capture_x11_png(ScreenCaptureScope::PrimaryDisplay)
}
