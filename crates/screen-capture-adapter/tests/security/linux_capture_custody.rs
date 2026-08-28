#![cfg(target_os = "linux")]

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{capture_window_title_contains_png, ScreenCaptureAttempt, ScreenCaptureScope};
use ocentra_parent_screen_capture_adapter::ScreenCaptureWindowTitleQuery;

#[test]
fn linux_title_capture_does_not_emit_uncustodied_metadata() {
    let query = ScreenCaptureWindowTitleQuery::try_from("terminal".to_owned()).expect("valid query");
    let attempt = capture_window_title_contains_png(&query);
    match attempt { ScreenCaptureAttempt::Degraded(metadata) => { assert_eq!(metadata.status, ActivityCaptureCapabilityStatus::Unavailable); assert_eq!(metadata.scope, ScreenCaptureScope::SelectedWindow); assert!(metadata.pid.is_none() && metadata.app_name.is_none() && metadata.title.is_none() && metadata.window_id.is_none() && metadata.monitor_id.is_none() && metadata.monitor_name.is_none()); }, ScreenCaptureAttempt::Captured(_) => panic!("title must not be echoed") }
}
