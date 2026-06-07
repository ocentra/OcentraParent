use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE,
    SCREEN_PROVIDER_SERVICE_METADATA, SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN, SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV, SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE,
    SCREEN_SERVICE_EVENT_ID_PREFIX, SCREEN_SERVICE_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_MODEL_ID,
    SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX, SCREEN_SERVICE_RESULT_ID_PREFIX, SCREEN_SERVICE_SOURCE_ID,
    SCREEN_SERVICE_SUMMARY_CAPTURED, SCREEN_SERVICE_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureMetadata, ScreenCaptureScope,
};

use super::{
    screen_ai_analysis_runtime::{
        record_screen_ai_analysis_cycle, record_screen_ai_analysis_cycle_with_events,
        ScreenAiAnalysisCycleClock, ScreenAiAnalysisCycleOutcome, ScreenAiAnalysisRuntimeConfig,
    },
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCaptureClock,
        ScreenAiServiceCapturePaths, ScreenAiServiceCaptureRecord,
    },
    screen_ai_service_event_bridge::ScreenAiServiceEventBridgeError,
    screen_ai_service_event_subscription::{
        ScreenAiServiceEventRuntime, ScreenAiServiceEventSubscriptionDispatch,
    },
};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_analysis_runtime_is_disabled_without_explicit_parent_setting() {
    std::env::remove_var(SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV);

    assert_eq!(ScreenAiAnalysisRuntimeConfig::from_environment(), None);
}

#[tokio::test]
async fn screen_analysis_cycle_respects_disabled_screen_analysis_setting() {
    let config = ScreenAiAnalysisRuntimeConfig {
        screen_analysis_enabled: false,
        ..test_analysis_config()
    };
    let queue_job_id = record_test_capture(&config);

    let outcome = record_screen_ai_analysis_cycle(
        &config,
        ScreenAiAnalysisCycleClock::from_parts(
            3,
            constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        ),
    )
    .await
    .expect(constants::error::ACTIVITY_STORE_INGESTS);

    assert_eq!(outcome, ScreenAiAnalysisCycleOutcome::Suppressed);
    assert_queue_contains(&config, &queue_job_id);
    assert_only_service_metadata_summary(&config, &queue_job_id);
}

#[tokio::test]
async fn screen_analysis_cycle_records_unavailable_result_and_removes_queue_entry() {
    let config = test_analysis_config();
    let queue_job_id = record_test_capture(&config);

    let outcome = record_screen_ai_analysis_cycle(
        &config,
        ScreenAiAnalysisCycleClock::from_parts(
            3,
            constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        ),
    )
    .await
    .expect(constants::error::ACTIVITY_STORE_INGESTS);

    assert_eq!(
        outcome,
        ScreenAiAnalysisCycleOutcome::ProviderUnavailable {
            queue_job_id: queue_job_id.clone(),
        }
    );
    assert_queue_drained(&config);
    assert_unavailable_analysis_summary(&config, &queue_job_id);
}

#[tokio::test]
async fn screen_analysis_cycle_publishes_row_ready_event_and_gates_missing_policy_refs() {
    let config = test_analysis_config();
    let queue_job_id = record_test_capture(&config);
    let runtime = ScreenAiServiceEventRuntime::start()
        .await
        .expect(constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES);

    let outcome = record_screen_ai_analysis_cycle_with_events(
        &config,
        ScreenAiAnalysisCycleClock::from_parts(
            3,
            constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string(),
        ),
        Some(&runtime),
    )
    .await
    .expect(constants::error::ACTIVITY_STORE_INGESTS);

    assert_eq!(
        outcome,
        ScreenAiAnalysisCycleOutcome::ProviderUnavailable {
            queue_job_id: queue_job_id.clone(),
        }
    );
    assert_queue_drained(&config);
    assert_unavailable_analysis_summary(&config, &queue_job_id);
    assert_eq!(
        runtime.dispatches(),
        vec![ScreenAiServiceEventSubscriptionDispatch::Rejected {
            screen_analysis_result_id: service_analysis_result_id(&queue_job_id),
            queue_job_id,
            reason: ScreenAiServiceEventBridgeError::MissingPolicyDecision,
        }]
    );
}

fn service_analysis_result_id(queue_job_id: &str) -> String {
    let mut id = String::from(SCREEN_SERVICE_ANALYSIS_RESULT_ID_PREFIX);
    id.push_str(queue_job_id);
    id
}

fn test_analysis_config() -> ScreenAiAnalysisRuntimeConfig {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    reset_test_path(&root);
    ScreenAiAnalysisRuntimeConfig {
        screen_analysis_enabled: true,
        poll_seconds: 1,
        max_jobs: Some(1),
        max_ticks: Some(1),
        max_queue_scan: SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN,
        adapter_timeout_ms: SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
        adapter_command: None,
        ocr_redaction_policy: Default::default(),
        queue_dir: root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX),
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    }
}

fn reset_test_path(path: &Path) {
    if path.exists() {
        fs::remove_dir_all(path).expect(constants::error::ACTIVITY_STORE_OPENS);
    }
}

fn record_test_capture(config: &ScreenAiAnalysisRuntimeConfig) -> String {
    record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
        paths: ScreenAiServiceCapturePaths {
            queue_dir: &config.queue_dir,
            journal_path: &config.journal_path,
            journal_key_path: &config.journal_key_path,
            store_path: &config.store_path,
        },
        image: &captured_test_image(),
        clock: ScreenAiServiceCaptureClock::from_parts(
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
        temporary_image_ttl_seconds:
            ocentra_parent_agent_protocol::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
    })
    .expect(constants::error::ACTIVITY_STORE_INGESTS)
}

fn assert_queue_drained(config: &ScreenAiAnalysisRuntimeConfig) {
    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record =
        fs::read_to_string(queue_file).expect(constants::error::ACTIVITY_STORE_OPENS);
    assert!(queue_record.trim().is_empty());
}

fn assert_queue_contains(config: &ScreenAiAnalysisRuntimeConfig, queue_job_id: &str) {
    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record =
        fs::read_to_string(queue_file).expect(constants::error::ACTIVITY_STORE_OPENS);
    assert!(queue_record.contains(queue_job_id));
}

fn assert_only_service_metadata_summary(
    config: &ScreenAiAnalysisRuntimeConfig,
    queue_job_id: &str,
) {
    let store =
        ActivityStore::open(&config.store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let latest = &summary.results[0];

    assert_eq!(summary.returned, 1);
    assert_eq!(latest.queue_job_id, queue_job_id);
    assert_eq!(latest.provider_kind, SCREEN_PROVIDER_SERVICE_METADATA);
    assert_eq!(
        latest.capture_reason,
        constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE
    );
    assert!(!latest.policy_eligible);
}

fn assert_unavailable_analysis_summary(config: &ScreenAiAnalysisRuntimeConfig, queue_job_id: &str) {
    let store =
        ActivityStore::open(&config.store_path).expect(constants::error::ACTIVITY_STORE_OPENS);
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);
    let latest = &summary.results[0];

    assert_eq!(summary.returned, 2);
    assert_eq!(latest.queue_job_id, queue_job_id);
    assert_eq!(
        latest.provider_kind,
        SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE
    );
    assert_eq!(latest.summary, SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE);
    assert!(!latest.policy_eligible);
    assert_eq!(
        latest.capture_reason,
        constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE
    );
    assert_eq!(
        latest.capability_status,
        ActivityCaptureCapabilityStatus::Available.as_protocol_str()
    );
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
    path.push(SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV);
    path.push(TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    path.push(suffix);
    path
}
