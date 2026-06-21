use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore, window_capture::ForegroundWindowObservation,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, SCREEN_CATEGORY_UNKNOWN,
    SCREEN_PROVIDER_SERVICE_METADATA, SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX,
    SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED_ENV,
};
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureMetadata, ScreenCaptureScope,
};

use super::{
    screen_ai_foreground_runtime::{
        record_screen_ai_foreground_captured_image, ScreenAiForegroundTickClock,
    },
    screen_ai_foreground_runtime_config::{foreground_key, ScreenAiForegroundRuntimeConfig},
};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_foreground_runtime_is_disabled_without_explicit_parent_setting() {
    std::env::remove_var(SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED_ENV);

    assert_eq!(ScreenAiForegroundRuntimeConfig::from_environment(), None);
}

#[test]
fn screen_foreground_key_prefers_real_window_id_and_rejects_degraded_state() {
    let active = ForegroundWindowObservation::active(
        41,
        constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string(),
        constants::activity_store::TEST_APP_GAME_PROCESS_PATH.to_string(),
        constants::activity_store::TEST_WINDOW_TITLE.to_string(),
        constants::activity_store::TEST_WINDOW_ID.to_string(),
    );
    let mut expected = String::from(SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX);
    expected.push_str(constants::activity_store::TEST_WINDOW_ID);

    assert_eq!(foreground_key(&active), Some(expected));
    assert_eq!(
        foreground_key(&ForegroundWindowObservation::degraded(
            ActivityCaptureCapabilityStatus::Unavailable
        )),
        None
    );
}

#[test]
fn screen_foreground_capture_writes_native_trigger_queue_and_read_model_event() {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let config = ScreenAiForegroundRuntimeConfig {
        screen_analysis_enabled: true,
        foreground_capture_enabled: true,
        poll_seconds: 1,
        min_trigger_gap_seconds: 1,
        max_captures: Some(1),
        max_ticks: Some(1),
        max_pending_queue_records: 3,
        temporary_image_ttl_seconds:
            ocentra_parent_agent_protocol::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        queue_dir: root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX),
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    };
    let image = captured_test_image();

    let queue_job_id = record_screen_ai_foreground_captured_image(
        &config,
        &image,
        ScreenAiForegroundTickClock::from_parts(
            4,
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        ),
        1,
    )
    .expect(constants::error::ACTIVITY_STORE_INGESTS);

    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record =
        fs::read_to_string(queue_file).expect(constants::error::ACTIVITY_STORE_OPENS);
    assert!(queue_record.contains(&queue_job_id));
    assert!(!queue_record.contains(constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER));

    let store =
        ActivityStore::open(&config.store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.returned, 1);
    assert_eq!(
        summary.latest_primary_category,
        Some(SCREEN_CATEGORY_UNKNOWN.to_string())
    );
    assert_eq!(summary.latest_policy_eligible, Some(false));
    assert_eq!(
        summary.results[0].provider_kind,
        SCREEN_PROVIDER_SERVICE_METADATA
    );
    assert_eq!(
        summary.results[0].capture_reason,
        constants::activity_capture::SCREEN_TRIGGER_NATIVE_APP_FOREGROUND_START
    );
}

fn captured_test_image() -> CapturedScreenImage {
    CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope: ScreenCaptureScope::ActiveWindow,
            pid: Some(41),
            app_name: Some(constants::activity_store::TEST_APP_GAME_PROCESS_NAME.to_string()),
            title: Some(constants::activity_store::TEST_WINDOW_TITLE.to_string()),
            window_id: Some(41),
            monitor_id: None,
            monitor_name: None,
        },
        width: 1,
        height: 1,
        png_bytes: constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER
            .as_bytes()
            .to_vec(),
    }
}

fn test_path(suffix: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(constants::activity_store::TEST_FILE_PREFIX);
    path.push(std::process::id().to_string());
    path.push(constants::activity_capture::SCREEN_TRIGGER_NATIVE_APP_FOREGROUND_START);
    path.push(TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    path.push(suffix);
    path
}
