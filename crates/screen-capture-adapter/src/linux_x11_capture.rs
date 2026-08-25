use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use super::super::{degraded_capture, ScreenCaptureAttempt, ScreenCaptureScope};

// Linux xwd/convert capture remains fail-closed. A compile-checked, FD-backed
// handoff that keeps the capture owner attached to both tools is not available
// in this source-only lane; passing replaceable file pathnames would
// reintroduce a name-swap race.

pub(super) fn capture_x11_window_png(scope: ScreenCaptureScope) -> ScreenCaptureAttempt {
    degraded_capture(ActivityCaptureCapabilityStatus::Unavailable, scope)
}

pub(super) fn capture_x11_png(scope: ScreenCaptureScope) -> ScreenCaptureAttempt {
    degraded_capture(ActivityCaptureCapabilityStatus::Unavailable, scope)
}
