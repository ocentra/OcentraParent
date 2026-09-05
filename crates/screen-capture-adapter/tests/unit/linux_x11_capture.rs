#![cfg(target_os = "linux")]

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png, capture_primary_display_png, ScreenCaptureAttempt,
    ScreenCaptureScope,
};

#[test]
fn linux_capture_remains_unavailable_without_fd_owned_handoff() -> Result<(), String> {
    for (attempt, scope) in [
        (
            capture_active_window_png(),
            ScreenCaptureScope::ActiveWindow,
        ),
        (
            capture_primary_display_png(),
            ScreenCaptureScope::PrimaryDisplay,
        ),
    ] {
        let ScreenCaptureAttempt::Degraded(metadata) = attempt else {
            return Err("Linux capture must remain degraded without fd-owned handoff".to_owned());
        };
        assert_eq!(
            metadata.status,
            ActivityCaptureCapabilityStatus::Unavailable
        );
        assert_eq!(metadata.scope, scope);
        assert_eq!(metadata.pid, None);
        assert_eq!(metadata.app_name, None);
        assert_eq!(metadata.title, None);
        assert_eq!(metadata.window_id, None);
        assert_eq!(metadata.monitor_id, None);
        assert_eq!(metadata.monitor_name, None);
    }
    Ok(())
}

#[path = "linux_foreground_source_preflight.rs"]
mod linux_foreground_source_preflight;
