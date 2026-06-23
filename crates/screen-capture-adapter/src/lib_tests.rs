use super::*;

#[test]
fn degraded_attempt_reports_status_and_active_window_scope() {
    let attempt = degraded_capture(
        ActivityCaptureCapabilityStatus::AccessDenied,
        ScreenCaptureScope::ActiveWindow,
    );

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

#[test]
fn window_title_query_rejects_blank_input() {
    assert_eq!(
        ScreenCaptureWindowTitleQuery::try_from(String::from("  ")),
        Err(ScreenCaptureWindowTitleQueryError::Empty)
    );
}

#[test]
fn window_title_query_trims_input() {
    assert_eq!(
        ScreenCaptureWindowTitleQuery::try_from(String::from(" Ocentra "))
            .as_ref()
            .map(ScreenCaptureWindowTitleQuery::as_str),
        Ok("Ocentra")
    );
}
