use crate::{ScreenCaptureAttempt, ScreenCaptureWindowTitleQuery};

#[path = "desktop_xcap_display.rs"]
mod display;
#[path = "desktop_xcap_window.rs"]
mod window;

pub(super) fn capture_active_window_png() -> ScreenCaptureAttempt {
    window::capture_active_window_png()
}

pub(super) fn capture_window_title_contains_png(
    title_query: &ScreenCaptureWindowTitleQuery,
) -> ScreenCaptureAttempt {
    window::capture_window_title_contains_png(title_query)
}

pub(super) fn capture_primary_display_png() -> ScreenCaptureAttempt {
    display::capture_primary_display_png()
}
