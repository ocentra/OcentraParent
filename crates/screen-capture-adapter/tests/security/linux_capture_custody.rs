#![cfg(target_os = "linux")]

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::ScreenCaptureWindowTitleQuery;
use ocentra_parent_screen_capture_adapter::{
    capture_window_title_contains_png, ScreenCaptureAttempt, ScreenCaptureScope,
};

#[test]
fn linux_title_capture_does_not_emit_uncustodied_metadata() -> Result<(), String> {
    let query =
        ScreenCaptureWindowTitleQuery::try_from("terminal".to_owned()).expect("valid query");
    let attempt = capture_window_title_contains_png(&query);
    let ScreenCaptureAttempt::Degraded(metadata) = attempt else {
        return Err("title capture must remain degraded without custodied metadata".to_owned());
    };
    assert_eq!(
        metadata.status,
        ActivityCaptureCapabilityStatus::Unavailable
    );
    assert_eq!(metadata.scope, ScreenCaptureScope::SelectedWindow);
    assert_eq!(metadata.pid, None);
    assert_eq!(metadata.app_name, None);
    assert_eq!(metadata.title, None);
    assert_eq!(metadata.window_id, None);
    assert_eq!(metadata.monitor_id, None);
    assert_eq!(metadata.monitor_name, None);
    Ok(())
}
