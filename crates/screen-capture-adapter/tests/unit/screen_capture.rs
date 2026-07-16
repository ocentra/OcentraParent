use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureMetadata, ScreenCaptureScope,
};

#[test]
fn degraded_attempt_reports_status_and_active_window_scope() {
    let attempt = ScreenCaptureAttempt::Degraded(ScreenCaptureMetadata {
        status: ActivityCaptureCapabilityStatus::AccessDenied,
        scope: ScreenCaptureScope::ActiveWindow,
        pid: None,
        app_name: None,
        title: None,
        window_id: None,
        monitor_id: None,
        monitor_name: None,
    });

    assert_eq!(
        attempt.status(),
        ActivityCaptureCapabilityStatus::AccessDenied
    );
    assert!(matches!(
        attempt,
        ScreenCaptureAttempt::Degraded(ScreenCaptureMetadata {
            scope: ScreenCaptureScope::ActiveWindow,
            ..
        })
    ));
}

#[test]
fn captured_attempt_reports_available_status() {
    let attempt = ScreenCaptureAttempt::Captured(CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope: ScreenCaptureScope::ActiveWindow,
            pid: None,
            app_name: None,
            title: None,
            window_id: None,
            monitor_id: None,
            monitor_name: None,
        },
        width: 1,
        height: 1,
        png_bytes: vec![1],
    });

    assert_eq!(attempt.status(), ActivityCaptureCapabilityStatus::Available);
}
