use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::linux_foreground_source::{
    foreground_source_preflight_with_deadline,
    linux_tools::{probe_xdotool, probe_xprop},
};
use crate::{
    degraded_capture, ScreenCaptureAttempt, ScreenCaptureScope, ScreenCaptureWindowTitleQuery,
};

#[path = "linux_x11_capture.rs"]
mod capture;

const CAPTURE_DEADLINE: Duration = Duration::from_secs(2);

pub(super) fn capture_active_window_png() -> ScreenCaptureAttempt {
    let deadline = Instant::now() + CAPTURE_DEADLINE;
    let preflight = foreground_source_preflight_with_deadline(deadline);
    if !preflight.source_ready() {
        return degraded_capture(
            ActivityCaptureCapabilityStatus::Unavailable,
            ScreenCaptureScope::ActiveWindow,
        );
    }

    let (_, _, xprop_window) = probe_xprop(deadline);
    let window = xprop_window.or_else(|| {
        let (_, _, xdotool_window) = probe_xdotool(deadline);
        xdotool_window
    });
    let Some(window) = window else {
        return degraded_capture(
            ActivityCaptureCapabilityStatus::NoActiveWindow,
            ScreenCaptureScope::ActiveWindow,
        );
    };

    capture::capture_x11_window_png(&window, ScreenCaptureScope::ActiveWindow, deadline)
}

pub(super) fn capture_window_title_contains_png(
    _title_query: &ScreenCaptureWindowTitleQuery,
) -> ScreenCaptureAttempt {
    degraded_capture(
        ActivityCaptureCapabilityStatus::Unavailable,
        ScreenCaptureScope::SelectedWindow,
    )
}

pub(super) fn capture_primary_display_png() -> ScreenCaptureAttempt {
    capture::capture_x11_png(
        ScreenCaptureScope::PrimaryDisplay,
        Instant::now() + CAPTURE_DEADLINE,
    )
}
