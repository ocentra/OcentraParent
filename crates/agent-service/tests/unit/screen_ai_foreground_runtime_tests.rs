use std::path::PathBuf as TestPathBuf;
use std::string::String as TestString;
use std::{
    fs,
    io::Error as IoError,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore, window_capture::ForegroundWindowObservation,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CATEGORY_UNKNOWN, SCREEN_PROVIDER_SERVICE_METADATA,
    SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX, SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX, SCREEN_SERVICE_FOREGROUND_MODEL_ID,
    SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX, SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED_ENV, SCREEN_SERVICE_FOREGROUND_SOURCE_ID,
    SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED, SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::ScreenCaptureScheduleTrigger, CapturedScreenImage, ScreenCaptureMetadata,
    ScreenCaptureScope,
};

use super::{
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCapturePaths,
        ScreenAiServiceCaptureRecord,
    },
    screen_ai_foreground_runtime::types::ScreenAiForegroundTickClock,
    screen_ai_foreground_runtime_config::{
        foreground_key, pending_queue_record_count, ScreenAiForegroundRuntimeConfig,
    },
};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

type TestResult = Result<(), IoError>;

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
    let mut expected = TestString::from(SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX);
    expected.push_str(constants::activity_store::TEST_WINDOW_ID);

    assert_eq!(foreground_key(&active).map(|key| key.0), Some(expected));
    assert_eq!(
        foreground_key(&ForegroundWindowObservation::degraded(
            ActivityCaptureCapabilityStatus::Unavailable
        )),
        None
    );
}

#[test]
fn screen_foreground_config_exposes_scheduler_and_empty_queue_contract() -> TestResult {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    let config = foreground_config(&root);
    let scheduler = config.scheduler_settings();

    assert!(scheduler.screen_analysis_enabled);
    assert!(scheduler.trigger_capture_enabled);
    assert!(!scheduler.cadence_capture_enabled);
    assert_eq!(scheduler.allowed_scope, ScreenCaptureScope::ActiveWindow);
    assert_eq!(
        scheduler.enabled_triggers,
        &[ScreenCaptureScheduleTrigger::NativeAppForegroundStart]
    );
    assert_eq!(pending_queue_record_count(&config.queue_dir), Ok(0));

    Ok(())
}

#[test]
fn screen_foreground_capture_writes_native_trigger_queue_and_read_model_event() -> TestResult {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    let config = foreground_config(&root);
    let image = captured_test_image();

    let queue_job_id = record_foreground_capture_for_test(
        &config,
        &image,
        foreground_clock(4, constants::activity_store::TEST_SECOND_OBSERVED_AT),
        1,
    )
    .map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_INGESTS
        ))
    })?;

    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record = fs::read_to_string(queue_file).map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    let queue_lines = queue_record.lines().collect::<Vec<_>>();
    assert_eq!(queue_lines.len(), 1);
    assert_eq!(pending_queue_record_count(&config.queue_dir), Ok(1));
    let queue_entry: serde_json::Value = serde_json::from_str(queue_lines[0]).map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::AGENT_EVENT_SERIALIZES
        ))
    })?;
    let ciphertext = queue_entry
        .get(constants::field::CIPHERTEXT)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| IoError::other(format!("missing {}", constants::field::CIPHERTEXT)))?;

    assert_eq!(
        queue_entry
            .get(constants::field::SCREEN_QUEUE_JOB_ID)
            .and_then(serde_json::Value::as_str),
        Some(queue_job_id.as_str())
    );
    assert_eq!(
        queue_entry
            .get(constants::field::STATUS)
            .and_then(serde_json::Value::as_str),
        Some(ocentra_parent_agent_protocol::screen_evidence::SCREEN_QUEUE_STATUS_QUEUED)
    );
    assert_ne!(
        ciphertext,
        constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER
    );

    let store = ActivityStore::open(&config.store_path).map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .map_err(|error| {
            IoError::other(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

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

    Ok(())
}

fn foreground_config(root: &std::path::Path) -> ScreenAiForegroundRuntimeConfig {
    ScreenAiForegroundRuntimeConfig {
        screen_analysis_enabled: true,
        foreground_capture_enabled: true,
        poll_seconds: 1,
        min_trigger_gap_seconds: 1,
        max_captures: Some(1),
        max_ticks: Some(1),
        max_pending_queue_records: 3,
        temporary_image_ttl_seconds:
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        queue_dir: root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX),
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    }
}

fn record_foreground_capture_for_test(
    config: &ScreenAiForegroundRuntimeConfig,
    image: &CapturedScreenImage,
    clock: ScreenAiForegroundTickClock,
    sequence_index: u64,
) -> Result<String, crate::activity_capture::ActivityCaptureError> {
    record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
        paths: ScreenAiServiceCapturePaths {
            queue_dir: &config.queue_dir,
            journal_path: &config.journal_path,
            journal_key_path: &config.journal_key_path,
            store_path: &config.store_path,
        },
        image,
        clock,
        sequence_index,
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_NATIVE_APP_FOREGROUND_START,
        source_id: SCREEN_SERVICE_FOREGROUND_SOURCE_ID,
        queue_job_id_prefix: SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX,
        result_id_prefix: SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX,
        event_id_prefix: SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX,
        evidence_id_prefix: SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX,
        summary: SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED,
        model_id: SCREEN_SERVICE_FOREGROUND_MODEL_ID,
        template_version: SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION,
        temporary_image_ttl_seconds: config.temporary_image_ttl_seconds,
    })
    .map(|queue_job_id| queue_job_id.0)
}

fn foreground_clock(
    epoch_seconds: u64,
    timestamp: impl std::fmt::Display,
) -> ScreenAiForegroundTickClock {
    let timestamp = timestamp.to_string();
    ScreenAiForegroundTickClock {
        epoch_seconds,
        timestamp,
    }
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

fn test_path(suffix: impl std::fmt::Display) -> TestPathBuf {
    let suffix = suffix.to_string();
    let mut path = std::env::temp_dir();
    path.push(constants::activity_store::TEST_FILE_PREFIX);
    path.push(std::process::id().to_string());
    path.push(constants::activity_capture::SCREEN_TRIGGER_NATIVE_APP_FOREGROUND_START);
    path.push(TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    path.push(suffix);
    path
}
