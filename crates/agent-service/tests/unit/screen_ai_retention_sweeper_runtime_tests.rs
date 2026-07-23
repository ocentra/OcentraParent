use std::path::PathBuf as TestPathBuf;
use std::string::String as TestString;
use std::{
    fs::{self, remove_dir_all},
    io::Error as IoError,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::replay::ReplayFilter;
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_core::screen_evidence_queue::ScreenEvidenceQueue;
use ocentra_parent_agent_protocol as parent_protocol;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisQueueJob, ScreenEvidenceRecentSummary, SCREEN_DELETION_DELETE_FAILED,
    SCREEN_DELETION_EXPIRED_DELETED,
};

use crate::screen_ai_retention_sweeper_deletion_events::{
    publish_screen_retention_deletion_events, ScreenAiRetentionSweeperDeletionEventOutcome,
};
use crate::screen_ai_retention_sweeper_runtime::{
    acknowledge_published_deletion_outbox, finalize_published_deletion_outbox,
    record_screen_ai_retention_sweeper_tick, ScreenAiRetentionSweeperClock,
    ScreenAiRetentionSweeperOutcome, ScreenAiRetentionSweeperRuntimeConfig,
};
use crate::test_invariants::require_ok;

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_retention_sweeper_runtime_is_enabled_by_default() -> Result<(), IoError> {
    let config = ScreenAiRetentionSweeperRuntimeConfig::from_environment()
        .ok_or_else(|| IoError::other("default retention sweeper configuration is disabled"))?;
    assert_eq!(config.poll_seconds, 5);
    assert_eq!(config.max_sweeps, None);
    assert_eq!(config.max_ticks, None);
    Ok(())
}

#[tokio::test]
async fn screen_retention_sweeper_tick_deletes_expired_queue_records_only() -> Result<(), IoError> {
    let root = test_path(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let _ = remove_dir_all(&root);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let key_path = root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX);
    let journal_path = root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX);
    let store_path = root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX);
    let key = JournalKey::from_bytes([4; JOURNAL_KEY_BYTES]);
    require_ok(
        fs::create_dir_all(&root),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    require_ok(
        fs::write(&key_path, key.as_bytes()),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let queue = require_ok(
        ScreenEvidenceQueue::open(&queue_dir, key),
        constants::error::JOURNAL_OPENS,
    );
    let expired_job = screen_queue_job_with_expiry(
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    let fresh_job = screen_queue_job_with_expiry(
        constants::activity_store::TEST_SCREEN_RESULT_ID,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    );
    require_ok(
        queue.append_encrypted_image(
            &expired_job,
            constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER.as_bytes(),
        ),
        constants::error::JOURNAL_APPENDS,
    );
    require_ok(
        queue.append_encrypted_image(
            &fresh_job,
            constants::activity_store::TEST_SCREEN_SUMMARY.as_bytes(),
        ),
        constants::error::JOURNAL_APPENDS,
    );
    let config = ScreenAiRetentionSweeperRuntimeConfig {
        poll_seconds: 1,
        max_sweeps: Some(1),
        max_ticks: Some(1),
        queue_dir,
        journal_path,
        journal_key_path: key_path,
        store_path: store_path.clone(),
    };

    let outcome = require_ok(
        record_screen_ai_retention_sweeper_tick(
            &config,
            sweeper_clock(constants::activity_store::TEST_SECOND_OBSERVED_AT),
        ),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let deletion_events = publish_swept_deletion_events(&store_path, &outcome).await;
    let entries = require_ok(
        queue.read_decrypted_entries(4),
        constants::error::JOURNAL_READS,
    );
    let screen_summary = require_ok(
        ActivityStore::open(&store_path),
        constants::error::ACTIVITY_STORE_OPENS,
    )
    .screen_evidence_recent_summary(
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    )
    .map_err(|error| {
        IoError::other(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    let _ = remove_dir_all(&root);

    assert_sweep_outcome(outcome, &expired_job)?;
    assert_sweep_deletion_events(&deletion_events, &expired_job);
    assert_sweep_store(
        entries.len(),
        entries[0].queue_job_id.as_str(),
        &fresh_job,
        &screen_summary,
    );
    Ok(())
}

#[test]
fn screen_retention_sweeper_records_delete_failed_when_outbox_replacement_is_blocked(
) -> Result<(), IoError> {
    let root = test_path("retention-outbox-replacement-failure");
    let _ = remove_dir_all(&root);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let key_path = root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX);
    let journal_path = root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX);
    let store_path = root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX);
    let key = JournalKey::from_bytes([27; JOURNAL_KEY_BYTES]);
    fs::create_dir_all(&root)?;
    fs::write(&key_path, key.as_bytes())?;
    let queue = ScreenEvidenceQueue::open(&queue_dir, key)
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let expired_job = screen_queue_job_with_expiry(
        "screen-retention-outbox-replacement-failure",
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    queue
        .append_encrypted_image(&expired_job, b"retryable-expired-image")
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    fs::create_dir(queue.path().with_extension("deletion-outbox"))?;
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
        sweeper_clock(constants::activity_store::TEST_SECOND_OBSERVED_AT),
    )
    .map_err(|error| IoError::other(format!("{error:?}")))?;
    let remaining = queue
        .read_decrypted_entries(4)
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let summary = ActivityStore::open(&store_path)
        .map_err(|error| IoError::other(format!("{error:?}")))?
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let _ = remove_dir_all(&root);

    assert_eq!(
        outcome,
        ScreenAiRetentionSweeperOutcome::NoExpired { pending_count: 1 }
    );
    assert_eq!(remaining.len(), 1);
    assert_eq!(
        summary.latest_image_deletion_state.as_deref(),
        Some(SCREEN_DELETION_DELETE_FAILED)
    );
    Ok(())
}

#[tokio::test]
async fn screen_retention_sweeper_replays_durable_outbox_after_publication_failure_and_restart(
) -> Result<(), IoError> {
    let root = test_path("retention-outbox-restart");
    let _ = remove_dir_all(&root);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let key_path = root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX);
    let journal_path = root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX);
    let store_path = root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX);
    let delivery_journal_path =
        store_path.with_extension(constants::activity_store::DEFAULT_JOURNAL_FILE_NAME);
    let key = JournalKey::from_bytes([7; JOURNAL_KEY_BYTES]);
    require_ok(
        fs::create_dir_all(&root),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    require_ok(
        fs::write(&key_path, key.as_bytes()),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let queue = require_ok(
        ScreenEvidenceQueue::open(&queue_dir, key),
        constants::error::JOURNAL_OPENS,
    );
    let expired_job = screen_queue_job_with_expiry(
        "screen-retention-outbox-restart",
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    require_ok(
        queue.append_encrypted_image(&expired_job, b"redacted-test-image"),
        constants::error::JOURNAL_APPENDS,
    );
    let config = ScreenAiRetentionSweeperRuntimeConfig {
        poll_seconds: 1,
        max_sweeps: Some(1),
        max_ticks: Some(1),
        queue_dir,
        journal_path,
        journal_key_path: key_path,
        store_path: store_path.clone(),
    };

    let first = require_ok(
        record_screen_ai_retention_sweeper_tick(
            &config,
            sweeper_clock(constants::activity_store::TEST_SECOND_OBSERVED_AT),
        ),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    require_ok(
        fs::create_dir(&delivery_journal_path),
        constants::error::JOURNAL_OPENS,
    );
    let failed_publication = publish_swept_deletion_events(&store_path, &first).await;
    let failed_acknowledgement = finalize_published_deletion_outbox(&config, &failed_publication);
    let after_failure = require_ok(
        queue.read_decrypted_entries(4),
        constants::error::JOURNAL_READS,
    );
    require_ok(
        fs::remove_dir(&delivery_journal_path),
        constants::error::JOURNAL_OPENS,
    );
    let restarted = require_ok(
        record_screen_ai_retention_sweeper_tick(
            &config,
            sweeper_clock(constants::activity_store::TEST_THIRD_OBSERVED_AT),
        ),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let replayed = publish_swept_deletion_events(&store_path, &restarted).await;
    assert_durable_deletion_delivery(&delivery_journal_path).await?;
    let acknowledged = require_ok(
        queue.acknowledge_expired_entries(std::slice::from_ref(&expired_job.queue_job_id)),
        constants::error::JOURNAL_APPENDS,
    );
    let repeat_acknowledgement = require_ok(
        queue.acknowledge_expired_entries(std::slice::from_ref(&expired_job.queue_job_id)),
        constants::error::JOURNAL_APPENDS,
    );
    let after_acknowledgement = require_ok(
        record_screen_ai_retention_sweeper_tick(
            &config,
            sweeper_clock(constants::activity_store::TEST_THIRD_OBSERVED_AT),
        ),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let _ = remove_dir_all(&root);

    assert!(failed_publication.is_empty());
    assert_eq!(failed_acknowledgement, Ok(0));
    assert!(after_failure.is_empty());
    assert_replayed_outbox_outcome(first, &expired_job)?;
    assert_replayed_outbox_outcome(restarted, &expired_job)?;
    assert_sweep_deletion_events(&replayed, &expired_job);
    assert_eq!(acknowledged, 1);
    assert_eq!(repeat_acknowledgement, 0);
    assert_eq!(
        after_acknowledgement,
        ScreenAiRetentionSweeperOutcome::QueueEmpty
    );
    Ok(())
}

async fn assert_durable_deletion_delivery(
    delivery_journal_path: &std::path::Path,
) -> Result<(), IoError> {
    let durable_delivery =
        NdjsonEventJournal::with_options(delivery_journal_path, NdjsonJournalOptions::hash_chain())
            .replay_projection(ReplayFilter::all())
            .await
            .map_err(|error| IoError::other(format!("{error:?}")))?;
    assert_eq!(durable_delivery.records.len(), 1);
    assert_eq!(
        durable_delivery.records[0]
            .envelope
            .contract
            .event_type
            .as_str(),
        constants::screen_flow::EVENT_SCREEN_DELETION_COMMITTED
    );
    Ok(())
}

#[test]
fn screen_retention_acknowledgement_rejects_zero_removal_as_failed_retry() -> Result<(), IoError> {
    let root = test_path("retention-ack-zero");
    let _ = remove_dir_all(&root);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let key_path = root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX);
    let key = JournalKey::from_bytes([15; JOURNAL_KEY_BYTES]);
    fs::create_dir_all(&root)?;
    fs::write(&key_path, key.as_bytes())?;
    ScreenEvidenceQueue::open(&queue_dir, key)
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let config = ScreenAiRetentionSweeperRuntimeConfig {
        poll_seconds: 1,
        max_sweeps: Some(1),
        max_ticks: Some(1),
        queue_dir,
        journal_path: root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX),
        journal_key_path: key_path,
        store_path: root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX),
    };
    let published = vec![ScreenAiRetentionSweeperDeletionEventOutcome {
        queue_job_id: "screen-ack-missing".to_string(),
        downstream_event_count: 1,
        raw_image_escaped: false,
    }];

    let acknowledgement = acknowledge_published_deletion_outbox(&config, &published);
    let _ = remove_dir_all(&root);

    assert_eq!(
        acknowledgement,
        Err(crate::activity_capture::ActivityCaptureError::Io)
    );
    Ok(())
}

#[test]
fn screen_retention_acknowledgement_failure_does_not_relabel_successful_deletion(
) -> Result<(), IoError> {
    let root = test_path("retention-ack-not-delete-failure");
    let _ = remove_dir_all(&root);
    let queue_dir = root.join(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let key_path = root.join(constants::activity_store::TEST_CAPTURE_KEY_SUFFIX);
    let journal_path = root.join(constants::activity_store::TEST_CAPTURE_JOURNAL_SUFFIX);
    let store_path = root.join(constants::activity_store::TEST_CAPTURE_STORE_SUFFIX);
    let key = JournalKey::from_bytes([16; JOURNAL_KEY_BYTES]);
    fs::create_dir_all(&root)?;
    fs::write(&key_path, key.as_bytes())?;
    let queue = ScreenEvidenceQueue::open(&queue_dir, key)
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let job = screen_queue_job_with_expiry(
        "screen-ack-failure-is-not-delete-failure",
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );
    queue
        .append_encrypted_image(&job, b"already-deleted-image")
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let config = ScreenAiRetentionSweeperRuntimeConfig {
        poll_seconds: 1,
        max_sweeps: Some(1),
        max_ticks: Some(1),
        queue_dir,
        journal_path,
        journal_key_path: key_path,
        store_path: store_path.clone(),
    };
    record_screen_ai_retention_sweeper_tick(
        &config,
        sweeper_clock(constants::activity_store::TEST_SECOND_OBSERVED_AT),
    )
    .map_err(|error| IoError::other(format!("{error:?}")))?;
    assert_eq!(
        queue
            .acknowledge_expired_entries(std::slice::from_ref(&job.queue_job_id))
            .map_err(|error| IoError::other(format!("{error:?}")))?,
        1
    );
    let published = vec![ScreenAiRetentionSweeperDeletionEventOutcome {
        queue_job_id: job.queue_job_id,
        downstream_event_count: 1,
        raw_image_escaped: false,
    }];

    assert_eq!(
        finalize_published_deletion_outbox(&config, &published),
        Err(crate::activity_capture::ActivityCaptureError::Io)
    );
    let summary = ActivityStore::open(&store_path)
        .map_err(|error| IoError::other(format!("{error:?}")))?
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| IoError::other(format!("{error:?}")))?;
    let _ = remove_dir_all(&root);

    assert_eq!(
        summary.latest_image_deletion_state.as_deref(),
        Some(SCREEN_DELETION_EXPIRED_DELETED)
    );
    Ok(())
}

fn assert_replayed_outbox_outcome(
    outcome: ScreenAiRetentionSweeperOutcome,
    expired_job: &ScreenAnalysisQueueJob,
) -> Result<(), IoError> {
    match outcome {
        ScreenAiRetentionSweeperOutcome::Swept {
            expired_entries,
            retained_count,
        } => {
            assert_eq!(retained_count, 0);
            assert_eq!(expired_entries.len(), 1);
            assert_eq!(expired_entries[0].queue_job_id, expired_job.queue_job_id);
            Ok(())
        }
        other => Err(IoError::other(format!(
            "expected durable outbox replay, got {other:?}"
        ))),
    }
}

async fn publish_swept_deletion_events(
    store_path: &std::path::Path,
    outcome: &ScreenAiRetentionSweeperOutcome,
) -> Vec<ScreenAiRetentionSweeperDeletionEventOutcome> {
    let runtime = require_ok(
        crate::screen_ai_service_event_subscription::ScreenAiServiceEventRuntime::start().await,
        constants::screen_flow::ERROR_SCREEN_SERVICE_EVENT_BRIDGE_PUBLISHES,
    );
    match outcome {
        ScreenAiRetentionSweeperOutcome::Swept {
            expired_entries, ..
        } => {
            publish_screen_retention_deletion_events(
                &runtime,
                store_path,
                expired_entries,
                crate::screen_ai_service_event_subscription::ObservedAtText(
                    constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
                ),
            )
            .await
        }
        _ => Vec::new(),
    }
}

fn assert_sweep_deletion_events(
    deletion_events: &[ScreenAiRetentionSweeperDeletionEventOutcome],
    expired_job: &ScreenAnalysisQueueJob,
) {
    assert_eq!(deletion_events.len(), 1);
    assert_eq!(deletion_events[0].queue_job_id, expired_job.queue_job_id);
    assert_eq!(deletion_events[0].downstream_event_count, 1);
    assert!(!deletion_events[0].raw_image_escaped);
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

    let outcome = require_ok(
        record_screen_ai_retention_sweeper_tick(
            &config,
            sweeper_clock(constants::activity_store::TEST_SECOND_OBSERVED_AT),
        ),
        constants::error::ACTIVITY_STORE_OPENS,
    );
    let _ = remove_dir_all(&root);

    assert_eq!(outcome, ScreenAiRetentionSweeperOutcome::QueueEmpty);
}

fn sweeper_clock(timestamp: impl std::fmt::Display) -> ScreenAiRetentionSweeperClock {
    let timestamp = timestamp.to_string();
    ScreenAiRetentionSweeperClock { timestamp }
}

fn assert_sweep_outcome(
    outcome: ScreenAiRetentionSweeperOutcome,
    expired_job: &ScreenAnalysisQueueJob,
) -> Result<(), IoError> {
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
                ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX
            ));
            Ok(())
        }
        ScreenAiRetentionSweeperOutcome::QueueEmpty => Err(IoError::other(
            "expected swept retention outcome, got queue empty",
        )),
        ScreenAiRetentionSweeperOutcome::NoExpired { pending_count } => Err(IoError::other(
            format!("expected swept retention outcome, got no expired items: {pending_count}"),
        )),
    }
}

fn assert_sweep_store(
    entry_count: usize,
    retained_queue_job_id: impl std::fmt::Display,
    fresh_job: &ScreenAnalysisQueueJob,
    screen_summary: &ScreenEvidenceRecentSummary,
) {
    let retained_queue_job_id = retained_queue_job_id.to_string();
    assert_eq!(entry_count, 1);
    assert_eq!(retained_queue_job_id, fresh_job.queue_job_id);
    assert_eq!(screen_summary.returned, 1);
    assert_eq!(
        screen_summary.results[0].queue_job_id,
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID
    );
    assert_eq!(
        screen_summary.results[0].image_deletion_state,
        ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_EXPIRED_DELETED
    );
}

fn screen_queue_job_with_expiry(
    queue_job_id: impl std::fmt::Display,
    expires_at: impl std::fmt::Display,
) -> ScreenAnalysisQueueJob {
    let queue_job_id = queue_job_id.to_string();
    let expires_at = expires_at.to_string();
    ScreenAnalysisQueueJob {
        schema_version: parent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id,
        created_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        not_before: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at,
        last_attempt_at: None,
        capture_reason:
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST
                .to_string(),
        capture_scope:
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW
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
        image_format: ocentra_parent_agent_protocol::screen_evidence::SCREEN_IMAGE_FORMAT_PNG
            .to_string(),
        status: ocentra_parent_agent_protocol::screen_evidence::SCREEN_QUEUE_STATUS_QUEUED
            .to_string(),
        attempt_count: 0,
        max_retry_count: 2,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_REQUIRED
            .to_string(),
        deletion_proof_ref: None,
        custody_state: ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_TEMP_QUEUE
            .to_string(),
    }
}

fn test_path(suffix: impl std::fmt::Display) -> TestPathBuf {
    let suffix = suffix.to_string();
    let mut name = TestString::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(suffix.as_str());
    let mut path = std::env::temp_dir();
    path.push(name);
    path
}
