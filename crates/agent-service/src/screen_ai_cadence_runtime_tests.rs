use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CATEGORY_UNKNOWN, SCREEN_PROVIDER_SERVICE_METADATA,
    SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV, SCREEN_SERVICE_EVENT_ID_PREFIX,
    SCREEN_SERVICE_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_MODEL_ID, SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX,
    SCREEN_SERVICE_RESULT_ID_PREFIX, SCREEN_SERVICE_SOURCE_ID, SCREEN_SERVICE_SUMMARY_CAPTURED,
    SCREEN_SERVICE_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureMetadata, ScreenCaptureScope,
};

use super::{
    screen_ai_cadence_runtime::{
        record_screen_ai_cadence_tick, ScreenAiCadenceRuntimeConfig, ScreenAiCadenceTickClock,
        ScreenAiCadenceTickOutcome,
    },
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCapturePaths,
        ScreenAiServiceCaptureRecord,
    },
};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);
const SCREEN_SERVICE_TEST_QUEUE_RECORD_LINE: &str = "{}\n";

#[test]
fn screen_cadence_runtime_is_disabled_without_explicit_parent_setting() {
    std::env::remove_var(SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV);

    assert_eq!(ScreenAiCadenceRuntimeConfig::from_environment(), None);
}

#[test]
fn screen_cadence_tick_respects_disabled_screen_analysis_setting() {
    let config = ScreenAiCadenceRuntimeConfig {
        screen_analysis_enabled: false,
        cadence_capture_enabled: true,
        cadence_seconds: 1,
        max_captures: Some(1),
        max_ticks: Some(1),
        max_pending_queue_records: 1,
        temporary_image_ttl_seconds:
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        queue_dir: test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX),
        journal_path: test_path(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: test_path(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: test_path(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    };

    let outcome = record_screen_ai_cadence_tick(
        &config,
        ocentra_parent_screen_capture_adapter::trigger_scheduler::ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: None,
        },
        ScreenAiCadenceTickClock::from_parts(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
        1,
    )
    .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_INGESTS));

    assert_eq!(outcome, ScreenAiCadenceTickOutcome::Suppressed);
    assert!(!config.queue_dir.exists());
    assert!(!config.store_path.exists());
}

#[test]
fn screen_cadence_capture_writes_encrypted_queue_and_read_model_event() {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let config = ScreenAiCadenceRuntimeConfig {
        screen_analysis_enabled: true,
        cadence_capture_enabled: true,
        cadence_seconds: 1,
        max_captures: Some(1),
        max_ticks: Some(1),
        max_pending_queue_records: 3,
        temporary_image_ttl_seconds:
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        queue_dir: root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX),
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    };
    let image = captured_test_image();

    let queue_job_id = record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
        paths: ScreenAiServiceCapturePaths {
            queue_dir: &config.queue_dir,
            journal_path: &config.journal_path,
            journal_key_path: &config.journal_key_path,
            store_path: &config.store_path,
        },
        image: &image,
        clock: ScreenAiCadenceTickClock::from_parts(
            2,
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        ),
        sequence_index: 1,
        capture_reason: constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE,
        source_id: SCREEN_SERVICE_SOURCE_ID,
        queue_job_id_prefix: SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX,
        result_id_prefix: SCREEN_SERVICE_RESULT_ID_PREFIX,
        event_id_prefix: SCREEN_SERVICE_EVENT_ID_PREFIX,
        evidence_id_prefix: SCREEN_SERVICE_EVIDENCE_ID_PREFIX,
        summary: SCREEN_SERVICE_SUMMARY_CAPTURED,
        model_id: SCREEN_SERVICE_MODEL_ID,
        template_version: SCREEN_SERVICE_TEMPLATE_VERSION,
        temporary_image_ttl_seconds: config.temporary_image_ttl_seconds,
    })
    .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_INGESTS));

    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record = fs::read_to_string(queue_file)
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_OPENS));
    let queue_lines = queue_record.lines().collect::<Vec<_>>();
    assert_eq!(queue_lines.len(), 1);
    let queue_entry: serde_json::Value = serde_json::from_str(queue_lines[0])
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES));
    let ciphertext = queue_entry
        .get(constants::field::CIPHERTEXT)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| panic!("missing {}", constants::field::CIPHERTEXT));

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

    let store = ActivityStore::open(&config.store_path)
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_OPENS));
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_QUERIES));

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
        constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE
    );
}

#[test]
fn screen_cadence_tick_suppresses_when_pending_queue_is_full() {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    fs::create_dir_all(&queue_dir)
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_OPENS));
    fs::write(
        queue_dir.join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME),
        SCREEN_SERVICE_TEST_QUEUE_RECORD_LINE,
    )
    .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_OPENS));
    let config = ScreenAiCadenceRuntimeConfig {
        screen_analysis_enabled: true,
        cadence_capture_enabled: true,
        cadence_seconds: 1,
        max_captures: Some(1),
        max_ticks: Some(1),
        max_pending_queue_records: 1,
        temporary_image_ttl_seconds:
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        queue_dir,
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    };

    let outcome = record_screen_ai_cadence_tick(
        &config,
        ocentra_parent_screen_capture_adapter::trigger_scheduler::ScreenCaptureSchedulerState {
            last_capture_at_epoch_seconds: None,
        },
        ScreenAiCadenceTickClock::from_parts(
            3,
            constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        ),
        1,
    )
    .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::ACTIVITY_STORE_INGESTS));

    assert_eq!(
        outcome,
        ScreenAiCadenceTickOutcome::QueueBackpressure {
            pending_count: 1,
            max_pending_queue_records: 1,
        }
    );
    assert!(!config.store_path.exists());
}

fn captured_test_image() -> CapturedScreenImage {
    CapturedScreenImage {
        metadata: ScreenCaptureMetadata {
            status: ActivityCaptureCapabilityStatus::Available,
            scope: ScreenCaptureScope::ActiveWindow,
            pid: Some(1),
            app_name: None,
            title: Some(constants::activity_store::TEST_WINDOW_TITLE.to_string()),
            window_id: Some(1),
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
    path.push(constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE);
    path.push(TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    path.push(suffix);
    path
}
