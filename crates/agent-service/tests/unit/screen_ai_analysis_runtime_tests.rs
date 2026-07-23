use std::path::PathBuf as TestPathBuf;
use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::{
    activity_store::ActivityStore,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::ScreenEvidenceQueue,
};
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE, SCREEN_PROVIDER_SERVICE_METADATA,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN, SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV,
    SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE, SCREEN_SERVICE_EVENT_ID_PREFIX,
    SCREEN_SERVICE_EVIDENCE_ID_PREFIX, SCREEN_SERVICE_MODEL_ID, SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX,
    SCREEN_SERVICE_RESULT_ID_PREFIX, SCREEN_SERVICE_SOURCE_ID, SCREEN_SERVICE_SUMMARY_CAPTURED,
    SCREEN_SERVICE_TEMPLATE_VERSION,
};
use ocentra_parent_screen_capture_adapter::{
    CapturedScreenImage, ScreenCaptureMetadata, ScreenCaptureScope,
};

use crate::test_invariants::{require_json_decode, require_ok, require_some};

use crate::test_text::TestText;

use super::{
    screen_ai_analysis_runtime::{
        config::{
            ScreenAiAnalysisCycleClock, ScreenAiAnalysisCycleOutcome, ScreenAiAnalysisRuntimeConfig,
        },
        lease_heartbeat::{
            start_analysis_lease_heartbeat_with_interval, ScreenAnalysisLeaseHeartbeatInput,
        },
        record_screen_ai_analysis_cycle_with_events,
    },
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCaptureClock,
        ScreenAiServiceCapturePaths, ScreenAiServiceCaptureRecord,
    },
    screen_ai_service_event_subscription::ScreenAiServiceEventRuntime,
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

    let outcome = require_ok(
        record_screen_ai_analysis_cycle_with_events(
            &config,
            analysis_clock(3, constants::activity_store::TEST_THIRD_OBSERVED_AT),
            None,
        )
        .await,
        constants::error::ACTIVITY_STORE_INGESTS,
    );

    assert_eq!(outcome, ScreenAiAnalysisCycleOutcome::Suppressed);
    assert_queue_contains(&config, &queue_job_id);
    assert_only_service_metadata_summary(&config, &queue_job_id);
}

#[tokio::test]
async fn screen_analysis_cycle_records_unavailable_result_and_removes_queue_entry() {
    let config = test_analysis_config();
    let queue_job_id = record_test_capture(&config);

    let outcome = require_ok(
        record_screen_ai_analysis_cycle_with_events(
            &config,
            analysis_clock(3, constants::activity_store::TEST_THIRD_OBSERVED_AT),
            None,
        )
        .await,
        constants::error::ACTIVITY_STORE_INGESTS,
    );

    assert_eq!(
        outcome,
        ScreenAiAnalysisCycleOutcome::ProviderUnavailable {
            queue_job_id: queue_job_id.to_string(),
        }
    );
    assert_queue_drained(&config);
    assert_unavailable_analysis_summary(&config, &queue_job_id);
}

#[tokio::test]
async fn screen_analysis_cycle_publishes_row_ready_event_and_gates_missing_policy_refs() {
    let config = test_analysis_config();
    let queue_job_id = record_test_capture(&config);
    let runtime = require_ok(
        ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_SUBSCRIBES,
    );

    let outcome = require_ok(
        record_screen_ai_analysis_cycle_with_events(
            &config,
            analysis_clock(3, constants::activity_store::TEST_THIRD_OBSERVED_AT),
            Some(&runtime),
        )
        .await,
        constants::error::ACTIVITY_STORE_INGESTS,
    );

    assert_eq!(
        outcome,
        ScreenAiAnalysisCycleOutcome::ProviderUnavailable {
            queue_job_id: queue_job_id.to_string(),
        }
    );
    assert_queue_drained(&config);
    assert_unavailable_analysis_summary(&config, &queue_job_id);
}

#[tokio::test]
async fn screen_analysis_lease_heartbeat_retries_transient_queue_open_failure() {
    let config = test_analysis_config();
    let queue_job_id = record_test_capture(&config);
    let key_bytes = require_ok(
        fs::read(&config.journal_key_path),
        constants::error::JOURNAL_READS,
    );
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(&key_bytes);
    let key = JournalKey::from_bytes(key);
    let queue = require_ok(
        ScreenEvidenceQueue::open(&config.queue_dir, key.clone()),
        constants::error::JOURNAL_OPENS,
    );
    let claimed = require_some(
        require_ok(
            queue.claim_first_decrypted_entry(
                1,
                constants::activity_store::TEST_FIRST_OBSERVED_AT,
                constants::activity_store::TEST_THIRD_OBSERVED_AT,
            ),
            constants::error::JOURNAL_READS,
        ),
        constants::error::JOURNAL_READS,
    );
    assert_eq!(claimed.queue_job_id, queue_job_id.as_str());
    let lease_path = queue.path().with_extension("analysis-leases");
    let initial_expiry = lease_expiry(&lease_path, queue_job_id.as_str());
    let blocked_queue_dir = config.queue_dir.with_extension("heartbeat-blocked");
    reset_test_path(&blocked_queue_dir);
    require_ok(
        fs::rename(&config.queue_dir, &blocked_queue_dir),
        constants::error::JOURNAL_APPENDS,
    );
    require_ok(
        fs::write(&config.queue_dir, b"blocked queue directory"),
        constants::error::JOURNAL_APPENDS,
    );

    let heartbeat = start_analysis_lease_heartbeat_with_interval(
        ScreenAnalysisLeaseHeartbeatInput {
            queue_dir: config.queue_dir.clone(),
            key,
            queue_job_id: queue_job_id.to_string(),
            adapter_timeout_ms: config.adapter_timeout_ms,
        },
        std::time::Duration::from_millis(100),
    );
    for _ in 0..16 {
        tokio::task::yield_now().await;
    }
    require_ok(
        fs::remove_file(&config.queue_dir),
        constants::error::JOURNAL_APPENDS,
    );
    require_ok(
        fs::rename(&blocked_queue_dir, &config.queue_dir),
        constants::error::JOURNAL_APPENDS,
    );

    let mut renewed_expiry = initial_expiry.clone();
    for _ in 0..20 {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        renewed_expiry = lease_expiry(&lease_path, queue_job_id.as_str());
        if renewed_expiry != initial_expiry {
            break;
        }
    }
    drop(heartbeat);
    reset_test_path(require_some(
        config.queue_dir.parent(),
        constants::error::JOURNAL_APPENDS,
    ));

    assert_ne!(renewed_expiry, initial_expiry);
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
        require_ok(
            fs::remove_dir_all(path),
            constants::error::ACTIVITY_STORE_OPENS,
        );
    }
}

fn lease_expiry(path: &Path, queue_job_id: &str) -> String {
    #[derive(serde::Deserialize)]
    struct LeaseRecord {
        queue_job_id: String,
        lease_expires_at: String,
    }

    let contents = require_ok(fs::read_to_string(path), constants::error::JOURNAL_READS);
    require_some(
        contents
            .lines()
            .map(|line| require_json_decode::<LeaseRecord>(line, constants::error::JOURNAL_READS))
            .find(|lease| lease.queue_job_id == queue_job_id)
            .map(|lease| lease.lease_expires_at),
        constants::error::JOURNAL_READS,
    )
}

fn record_test_capture(config: &ScreenAiAnalysisRuntimeConfig) -> TestText {
    TestText::from_display(require_ok(
        record_captured_screen_image_to_paths(ScreenAiServiceCaptureRecord {
            paths: ScreenAiServiceCapturePaths {
                queue_dir: &config.queue_dir,
                journal_path: &config.journal_path,
                journal_key_path: &config.journal_key_path,
                store_path: &config.store_path,
            },
            image: &captured_test_image(),
            clock: capture_clock(constants::activity_store::TEST_SECOND_OBSERVED_AT),
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
                ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        }),
        constants::error::ACTIVITY_STORE_INGESTS,
    ))
}

fn analysis_clock(
    epoch_seconds: u64,
    timestamp: impl std::fmt::Display,
) -> ScreenAiAnalysisCycleClock {
    let timestamp = timestamp.to_string();
    ScreenAiAnalysisCycleClock {
        epoch_seconds,
        timestamp,
    }
}

fn capture_clock(timestamp: impl std::fmt::Display) -> ScreenAiServiceCaptureClock {
    let timestamp = timestamp.to_string();
    let epoch_seconds = require_ok(
        chrono::DateTime::parse_from_rfc3339(&timestamp),
        constants::error::ACTIVITY_STORE_INGESTS,
    )
    .timestamp() as u64;
    ScreenAiServiceCaptureClock {
        epoch_seconds,
        timestamp,
    }
}

fn assert_queue_drained(config: &ScreenAiAnalysisRuntimeConfig) {
    let queue_file = config
        .queue_dir
        .join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    let queue_record = require_ok(
        fs::read_to_string(queue_file),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    assert!(queue_record.trim().is_empty());
}

fn assert_queue_contains(
    config: &ScreenAiAnalysisRuntimeConfig,
    queue_job_id: impl std::fmt::Display,
) {
    let queue_job_id = queue_job_id.to_string();
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
}

fn assert_only_service_metadata_summary(
    config: &ScreenAiAnalysisRuntimeConfig,
    queue_job_id: impl std::fmt::Display,
) {
    let queue_job_id = queue_job_id.to_string();
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
    let latest = &summary.results[0];

    assert_eq!(summary.returned, 1);
    assert_eq!(latest.queue_job_id, queue_job_id.as_str());
    assert_eq!(latest.provider_kind, SCREEN_PROVIDER_SERVICE_METADATA);
    assert_eq!(
        latest.capture_reason,
        constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE
    );
    assert!(!latest.policy_eligible);
}

fn assert_unavailable_analysis_summary(
    config: &ScreenAiAnalysisRuntimeConfig,
    queue_job_id: impl std::fmt::Display,
) {
    let queue_job_id = queue_job_id.to_string();
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
    let latest = &summary.results[0];

    assert_eq!(summary.returned, 2);
    assert_eq!(latest.queue_job_id, queue_job_id.as_str());
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

fn test_path(suffix: impl std::fmt::Display) -> TestPathBuf {
    let suffix = suffix.to_string();
    let mut path = std::env::temp_dir();
    path.push(constants::activity_store::TEST_FILE_PREFIX);
    path.push(std::process::id().to_string());
    path.push(SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV);
    path.push(TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    path.push(suffix);
    path
}
