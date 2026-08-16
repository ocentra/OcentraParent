use chrono::{DateTime, SecondsFormat};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    json_contract, screen_ai_analysis_runtime::config, screen_ai_cadence_runtime,
    screen_ai_cadence_runtime_event::ScreenAiServiceCaptureClock, screen_ai_foreground_runtime,
    screen_ai_foreground_runtime_config, screen_ai_retention_sweeper_runtime,
    screen_ai_service_event_bridge,
    screen_ai_service_event_subscription::live_view_service_runtime, test_invariants,
};

#[test]
fn screen_ai_runtime_clippy_linkage_keeps_entrypoints_live() {
    let _ = screen_ai_cadence_runtime::spawn_screen_ai_cadence_runtime;
    let _ = screen_ai_foreground_runtime::spawn_screen_ai_foreground_runtime;
    let _ = screen_ai_retention_sweeper_runtime::spawn_screen_ai_retention_sweeper_runtime;
    let _ = live_view_service_runtime::spawn_screen_live_view_worker_runtime;
    let _ = screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job;
    let _ = screen_ai_foreground_runtime_config::pending_queue_record_count;
    let _ =
        screen_ai_foreground_runtime_config::ScreenAiForegroundRuntimeConfig::scheduler_settings;

    let analysis_clock = config::ScreenAiAnalysisCycleClock::from_system_time();
    let analysis_clock_parsed = test_invariants::require_ok(
        DateTime::parse_from_rfc3339(&analysis_clock.timestamp),
        "analysis clock timestamp should parse",
    );
    assert_eq!(
        analysis_clock_parsed.to_rfc3339_opts(SecondsFormat::Millis, true),
        analysis_clock.timestamp
    );

    let capture_clock = ScreenAiServiceCaptureClock::from_system_time();
    let capture_clock_parsed = test_invariants::require_ok(
        DateTime::parse_from_rfc3339(&capture_clock.timestamp),
        "capture clock timestamp should parse",
    );
    assert_eq!(
        capture_clock_parsed.to_rfc3339_opts(SecondsFormat::Millis, true),
        capture_clock.timestamp
    );

    let runtime = crate::screen_ai_analysis_runtime::adapter_runtime_status(
        None::<&std::path::Path>,
        "2026-06-29T00:00:00Z",
    );
    assert_eq!(runtime.last_checked_at, "2026-06-29T00:00:00Z");
    assert_eq!(
        runtime.unavailable_reason.as_deref(),
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_BINARY_UNCONFIGURED)
    );

    let payload = serde_json::json!({"screen": "runtime"});
    assert_eq!(
        json_contract::serialize_json_string(&payload).0.as_str(),
        payload.to_string()
    );
    assert_eq!(
        json_contract::serialize_json_value(payload.clone()),
        payload
    );
    assert_eq!(
        test_invariants::serialize_test_json(&payload),
        payload.to_string()
    );

    let log_field = LogFieldValue::String("value".to_string());
    assert_eq!(
        test_invariants::require_log_string_field(Some(&log_field), "log field"),
        "value"
    );

    assert_eq!(
        crate::activity_capture::ActivityCaptureError::Io
            .reason()
            .to_string(),
        constants::value::ACTIVITY_CAPTURE_IO_ERROR
    );
}
