use std::{
    fs::{self, remove_dir_all},
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_parent_agent_core::{ActivityStore, JournalKey, ScreenEvidenceQueue, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_protocol::{constants, ScreenAnalysisQueueJob};

use crate::screen_ai_retention_sweeper_runtime::{
    record_screen_ai_retention_sweeper_tick, ScreenAiRetentionSweeperClock,
    ScreenAiRetentionSweeperOutcome, ScreenAiRetentionSweeperRuntimeConfig,
};

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_retention_sweeper_runtime_is_disabled_by_default() {
    assert_eq!(
        ScreenAiRetentionSweeperRuntimeConfig::from_environment(),
        None
    );
}

#[test]
fn screen_retention_sweeper_tick_deletes_expired_queue_records_only() {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let _ = remove_dir_all(&root);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let key_path = root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX);
    let journal_path = root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX);
    let store_path = root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX);
    let key = JournalKey::from_bytes([4; JOURNAL_KEY_BYTES]);
    fs::create_dir_all(&root).expect(constants::error::ACTIVITY_STORE_OPENS);
    fs::write(&key_path, key.as_bytes()).expect(constants::error::ACTIVITY_STORE_OPENS);
    let queue = ScreenEvidenceQueue::open(&queue_dir, key).expect(constants::error::JOURNAL_OPENS);
    let expired_job = screen_queue_job_with_expiry(
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    let fresh_job = screen_queue_job_with_expiry(
        constants::activity_store::TEST_SCREEN_RESULT_ID,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    );
    queue
        .append_encrypted_image(
            &expired_job,
            constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER.as_bytes(),
        )
        .expect(constants::error::JOURNAL_APPENDS);
    queue
        .append_encrypted_image(
            &fresh_job,
            constants::activity_store::TEST_SCREEN_SUMMARY.as_bytes(),
        )
        .expect(constants::error::JOURNAL_APPENDS);
    let config = ScreenAiRetentionSweeperRuntimeConfig {
        poll_seconds: 1,
        max_sweeps: Some(1),
        max_ticks: Some(1),
        queue_dir,
        journal_path,
        journal_key_path: key_path,
        store_path: store_path.clone(),
    };

    let outcome = record_screen_ai_retention_sweeper_tick(
        &config,
        ScreenAiRetentionSweeperClock::from_timestamp(
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        ),
    )
    .expect(constants::error::ACTIVITY_STORE_OPENS);
    let entries = queue
        .read_decrypted_entries(4)
        .expect(constants::error::JOURNAL_READS);
    let screen_summary = ActivityStore::open(&store_path)
        .expect(constants::error::ACTIVITY_STORE_OPENS)
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_OPENS);
    let _ = remove_dir_all(&root);

    match outcome {
        ScreenAiRetentionSweeperOutcome::Swept {
            expired_entries,
            retained_count,
        } => {
            assert_eq!(expired_entries.len(), 1);
            assert_eq!(retained_count, 1);
            assert_eq!(expired_entries[0].queue_job_id, expired_job.queue_job_id);
            assert_eq!(
                expired_entries[0].expires_at,
                constants::activity_store::TEST_SECOND_OBSERVED_AT
            );
            assert!(expired_entries[0].deletion_proof_ref.contains(
                ocentra_parent_agent_protocol::SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX
            ));
        }
        _ => assert!(false),
    }
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].queue_job_id, fresh_job.queue_job_id);
    assert_eq!(screen_summary.returned, 1);
    assert_eq!(
        screen_summary.results[0].queue_job_id,
        expired_job.queue_job_id
    );
    assert_eq!(
        screen_summary.results[0].image_deletion_state,
        ocentra_parent_agent_protocol::SCREEN_DELETION_EXPIRED_DELETED
    );
}

#[test]
fn screen_retention_sweeper_tick_keeps_queue_when_key_is_missing() {
    let root = test_path(constants::activity_store::TEST_CAPTURE_INVALID_KEY_SUFFIX);
    let _ = remove_dir_all(&root);
    let config = ScreenAiRetentionSweeperRuntimeConfig {
        poll_seconds: 1,
        max_sweeps: Some(1),
        max_ticks: Some(1),
        queue_dir: root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX),
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX),
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    };

    let outcome = record_screen_ai_retention_sweeper_tick(
        &config,
        ScreenAiRetentionSweeperClock::from_timestamp(
            constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        ),
    )
    .expect(constants::error::ACTIVITY_STORE_OPENS);
    let _ = remove_dir_all(&root);

    assert_eq!(outcome, ScreenAiRetentionSweeperOutcome::QueueEmpty);
}

fn screen_queue_job_with_expiry(queue_job_id: &str, expires_at: &str) -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: queue_job_id.to_string(),
        created_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        not_before: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at: expires_at.to_string(),
        last_attempt_at: None,
        capture_reason: ocentra_parent_agent_protocol::SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST
            .to_string(),
        capture_scope: ocentra_parent_agent_protocol::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW
            .to_string(),
        source_id: constants::activity_store::TEST_SCREEN_SOURCE_ID.to_string(),
        adapter_id: constants::activity_store::TEST_SCREEN_ADAPTER_ID.to_string(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_string(),
        local_user_ref: constants::activity_store::TEST_SCREEN_LOCAL_USER_REF.to_string(),
        parent_setting_ref: constants::activity_store::TEST_SCREEN_PARENT_SETTING_REF.to_string(),
        setting_version: 1,
        related_evidence_refs: Vec::new(),
        encrypted_image_ref: constants::activity_store::TEST_SCREEN_ENCRYPTED_IMAGE_REF.to_string(),
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        image_byte_size: 32,
        image_format: ocentra_parent_agent_protocol::SCREEN_IMAGE_FORMAT_PNG.to_string(),
        status: ocentra_parent_agent_protocol::SCREEN_QUEUE_STATUS_QUEUED.to_string(),
        attempt_count: 0,
        max_retry_count: 2,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: ocentra_parent_agent_protocol::SCREEN_DELETION_REQUIRED.to_string(),
        deletion_proof_ref: None,
        custody_state: ocentra_parent_agent_protocol::SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
    }
}

fn test_path(suffix: &str) -> PathBuf {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix);
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}
