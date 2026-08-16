use std::path::PathBuf as TestPathBuf;
use std::{
    fs,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CATEGORY_UNKNOWN, SCREEN_PROVIDER_SERVICE_METADATA, SCREEN_SERVICE_EVENT_ID_PREFIX,
    SCREEN_SERVICE_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_MODEL_ID, SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX,
    SCREEN_SERVICE_RESULT_ID_PREFIX, SCREEN_SERVICE_SOURCE_ID, SCREEN_SERVICE_SUMMARY_CAPTURED,
    SCREEN_SERVICE_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureMetadata, ScreenCaptureScope,
};

use super::{
    screen_ai_cadence_runtime::{ScreenAiCadenceRuntimeConfig, ScreenAiCadenceTickClock},
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCapturePaths,
        ScreenAiServiceCaptureRecord,
    },
};
use crate::test_invariants::{require_json_decode, require_ok, require_some};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_cadence_capture_writes_encrypted_queue_and_read_model_event() {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    if root.exists() {
        require_ok(
            fs::remove_dir_all(&root),
            constants::error::ACTIVITY_STORE_INGESTS,
        );
    }
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
        clock: cadence_clock(2, constants::activity_store::TEST_SECOND_OBSERVED_AT),
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
    });
    let queue_job_id = require_ok(queue_job_id, constants::error::ACTIVITY_STORE_INGESTS);

    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record = require_ok(
        fs::read_to_string(queue_file),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let queue_lines = queue_record.lines().collect::<Vec<_>>();
    assert_eq!(queue_lines.len(), 1);
    let queue_entry: serde_json::Value =
        require_json_decode(queue_lines[0], constants::error::AGENT_EVENT_SERIALIZES);
    let ciphertext = require_some(
        queue_entry
            .get(constants::field::CIPHERTEXT)
            .and_then(serde_json::Value::as_str),
        constants::field::CIPHERTEXT,
    );

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

    let store = require_ok(
        ActivityStore::open(&config.store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let summary = require_ok(
        store.screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        ),
        constants::error::ACTIVITY_STORE_QUERIES,
    );

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

fn cadence_clock(
    epoch_seconds: u64,
    timestamp: impl std::fmt::Display,
) -> ScreenAiCadenceTickClock {
    let timestamp = timestamp.to_string();
    ScreenAiCadenceTickClock {
        epoch_seconds,
        timestamp,
    }
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

fn test_path(suffix: impl std::fmt::Display) -> TestPathBuf {
    let suffix = suffix.to_string();
    let mut path = std::env::temp_dir();
    path.push(constants::activity_store::TEST_FILE_PREFIX);
    path.push(std::process::id().to_string());
    path.push(constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE);
    path.push(TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    path.push(suffix);
    path
}
