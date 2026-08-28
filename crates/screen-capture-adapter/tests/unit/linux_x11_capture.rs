#![cfg(target_os = "linux")]

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{capture_active_window_png, capture_primary_display_png, ScreenCaptureAttempt, ScreenCaptureScope};

#[test]
fn linux_capture_remains_unavailable_without_fd_owned_handoff() {
    for (attempt, scope) in [(capture_active_window_png(), ScreenCaptureScope::ActiveWindow), (capture_primary_display_png(), ScreenCaptureScope::PrimaryDisplay)] {
        match attempt { ScreenCaptureAttempt::Degraded(metadata) => { assert_eq!(metadata.status, ActivityCaptureCapabilityStatus::Unavailable); assert_eq!(metadata.scope, scope); assert!(metadata.pid.is_none() && metadata.app_name.is_none() && metadata.title.is_none() && metadata.window_id.is_none() && metadata.monitor_id.is_none() && metadata.monitor_name.is_none()); }, ScreenCaptureAttempt::Captured(_) => panic!("Linux capture must not claim custody") }
    }
}
