use std::{
    fmt::Display,
    fs::{read_to_string, remove_dir_all, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    thread,
};

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisQueueJob, SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
    SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CUSTODY_TEMP_QUEUE, SCREEN_DELETION_REQUIRED,
    SCREEN_IMAGE_FORMAT_PNG, SCREEN_QUEUE_STATUS_QUEUED,
    SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
};
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use serde_json::Value;

use ocentra_parent_agent_core::journal_crypto::{JournalKey, JOURNAL_KEY_BYTES};
use ocentra_parent_agent_core::screen_evidence_queue::ScreenEvidenceQueue;

use crate::test_text::TestText;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct TestPath(PathBuf);

impl AsRef<Path> for TestPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn screen_evidence_queue_encrypts_image_bytes_before_durable_write() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([9; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let plaintext = constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER.as_bytes();

    queue
        .append_encrypted_image(&screen_queue_job(), plaintext)
        .expect_value(constants::error::JOURNAL_APPENDS);
    let raw = read_to_string(queue.path()).expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);
    let record: Value =
        serde_json::from_str(raw.trim()).expect_value(constants::error::JOURNAL_READS);

    assert_eq!(
        record[constants::field::SCREEN_QUEUE_JOB_ID].as_str(),
        Some(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID)
    );
    assert_eq!(
        record[constants::field::CREATED_AT].as_str(),
        Some(constants::activity_store::TEST_FIRST_OBSERVED_AT)
    );
    assert_eq!(
        record[constants::field::EXPIRES_AT].as_str(),
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT)
    );
    assert_ne!(
        record[constants::field::CIPHERTEXT].as_str(),
        Some(constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER)
    );
    assert_ne!(
        record[constants::field::CIPHERTEXT].as_str(),
        Some(constants::activity_store::TEST_SCREEN_SUMMARY)
    );
}

#[test]
fn screen_evidence_queue_reads_decrypted_entries_for_local_analysis() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([7; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let plaintext = constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER.as_bytes();

    queue
        .append_encrypted_image(&screen_queue_job(), plaintext)
        .expect_value(constants::error::JOURNAL_APPENDS);
    let raw = read_to_string(queue.path()).expect_value(constants::error::JOURNAL_READS);
    let entries = queue
        .read_decrypted_entries(1)
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);
    let record: Value =
        serde_json::from_str(raw.trim()).expect_value(constants::error::JOURNAL_READS);

    assert_ne!(
        record[constants::field::CIPHERTEXT].as_str(),
        Some(constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER)
    );
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(entry.schema_version, SCREEN_EVIDENCE_SCHEMA_VERSION);
    assert_eq!(
        entry.queue_job_id,
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID
    );
    assert_eq!(
        entry.created_at.as_deref(),
        Some(constants::activity_store::TEST_FIRST_OBSERVED_AT)
    );
    assert_eq!(
        entry.expires_at.as_deref(),
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT)
    );
    assert_eq!(entry.status, SCREEN_QUEUE_STATUS_QUEUED);
    assert!(entry.deletion_required);
    assert_eq!(entry.deletion_status, SCREEN_DELETION_REQUIRED);
    assert_eq!(entry.deletion_proof_ref, None);
    assert_eq!(entry.image_bytes, plaintext);
    assert_eq!(entry.custody_state, SCREEN_CUSTODY_TEMP_QUEUE);
}

#[test]
fn screen_evidence_queue_sweeps_only_expired_entries_with_delete_proof_refs() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([6; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let expired_job = screen_queue_job();
    let mut fresh_queue_job_id = String::from(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID);
    fresh_queue_job_id.push(constants::delimiter::HYPHEN);
    fresh_queue_job_id.push_str(constants::activity_store::TEST_THIRD_OBSERVED_AT);
    let fresh_job = screen_queue_job_with_expiry(
        &fresh_queue_job_id,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    );

    queue
        .append_encrypted_image(
            &expired_job,
            constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER.as_bytes(),
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .append_encrypted_image(
            &fresh_job,
            constants::activity_store::TEST_SCREEN_SUMMARY.as_bytes(),
        )
        .expect_value(constants::error::JOURNAL_APPENDS);

    let sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    let entries = queue
        .read_decrypted_entries(4)
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert_eq!(sweep.expired_entries.len(), 1);
    assert_eq!(sweep.retained_count, 1);
    assert_eq!(
        sweep.expired_entries[0].queue_job_id,
        expired_job.queue_job_id
    );
    assert_eq!(
        sweep.expired_entries[0].expires_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        sweep.expired_entries[0].deletion_proof_ref,
        format!(
            "{SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX}{}",
            expired_job.queue_job_id
        )
    );
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].queue_job_id, fresh_job.queue_job_id);
}

#[test]
fn screen_evidence_queue_durably_replaces_outbox_across_updates_acknowledgements_and_restart() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let key = JournalKey::from_bytes([5; JOURNAL_KEY_BYTES]);
    let queue = ScreenEvidenceQueue::open(&directory, key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let first_job = screen_queue_job();
    let second_job = screen_queue_job_with_id("screen-outbox-second-expired-job");

    queue
        .append_encrypted_image(&first_job, b"first-expired-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    let first_sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .append_encrypted_image(&second_job, b"second-expired-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    let updated_sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    let acknowledged = queue
        .acknowledge_expired_entries(std::slice::from_ref(&first_job.queue_job_id))
        .expect_value(constants::error::JOURNAL_APPENDS);

    let restarted = ScreenEvidenceQueue::open(&directory, key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let recovered = restarted
        .remove_expired_entries(
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    let second_acknowledged = restarted
        .acknowledge_expired_entries(std::slice::from_ref(&second_job.queue_job_id))
        .expect_value(constants::error::JOURNAL_APPENDS);
    let final_restart =
        ScreenEvidenceQueue::open(&directory, key).expect_value(constants::error::JOURNAL_OPENS);
    let final_sweep = final_restart
        .remove_expired_entries(
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    let _ = remove_dir_all(&directory);

    assert_eq!(first_sweep.expired_entries.len(), 1);
    assert_eq!(updated_sweep.expired_entries.len(), 2);
    assert_eq!(acknowledged, 1);
    assert_eq!(recovered.expired_entries.len(), 1);
    assert_eq!(
        recovered.expired_entries[0].queue_job_id,
        second_job.queue_job_id
    );
    assert_eq!(second_acknowledged, 1);
    assert_eq!(final_sweep.expired_entries.len(), 0);
}

#[test]
fn screen_evidence_queue_removes_processed_entries_without_touching_pending_entries() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([8; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let first_job = screen_queue_job();
    let mut second_queue_job_id = String::from(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID);
    second_queue_job_id.push(constants::delimiter::HYPHEN);
    second_queue_job_id.push_str(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let second_job = screen_queue_job_with_id(&second_queue_job_id);

    queue
        .append_encrypted_image(
            &first_job,
            constants::activity_store::TEST_SCREEN_PLAINTEXT_MARKER.as_bytes(),
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .append_encrypted_image(
            &second_job,
            constants::activity_store::TEST_SCREEN_SUMMARY.as_bytes(),
        )
        .expect_value(constants::error::JOURNAL_APPENDS);

    let removed = queue
        .remove_entries(std::slice::from_ref(&first_job.queue_job_id))
        .expect_value(constants::error::JOURNAL_APPENDS);
    let entries = queue
        .read_decrypted_entries(4)
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert_eq!(removed, 1);
    assert_eq!(entries.len(), 1);
    let entry = &entries[0];
    assert_eq!(entry.queue_job_id, second_job.queue_job_id);
    assert_eq!(
        entry.image_bytes,
        constants::activity_store::TEST_SCREEN_SUMMARY.as_bytes()
    );
}

#[test]
fn screen_evidence_queue_sweeps_malformed_expiry_fail_closed() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([4; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let malformed_job = screen_queue_job_with_expiry(
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
        "not-a-timestamp",
    );
    queue
        .append_encrypted_image(&malformed_job, b"malformed-expiry")
        .expect_value(constants::error::JOURNAL_APPENDS);

    let sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    let _ = remove_dir_all(&directory);

    assert_eq!(sweep.expired_entries.len(), 1);
    assert_eq!(sweep.retained_count, 0);
    assert_eq!(
        sweep.expired_entries[0].queue_job_id,
        malformed_job.queue_job_id
    );
}

#[test]
fn screen_evidence_queue_serializes_concurrent_appends() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue = Arc::new(
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([3; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS),
    );
    let mut workers = Vec::new();
    for index in 0..8 {
        let queue = Arc::clone(&queue);
        workers.push(thread::spawn(move || {
            let queue_job_id = format!(
                "{}-{index}",
                constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID
            );
            let job = screen_queue_job_with_id(queue_job_id);
            queue.append_encrypted_image(&job, b"concurrent-image")
        }));
    }
    for worker in workers {
        worker
            .join()
            .expect_value(constants::error::JOURNAL_APPENDS)
            .expect_value(constants::error::JOURNAL_APPENDS);
    }
    let entries = queue
        .read_decrypted_entries(16)
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert_eq!(entries.len(), 8);
}

#[test]
fn screen_evidence_queue_missing_file_is_an_idempotent_removal() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([2; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    std::fs::remove_file(queue.path()).expect_value(constants::error::JOURNAL_APPENDS);

    let removed = queue
        .remove_entries(&[String::from(
            constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID,
        )])
        .expect_value(constants::error::JOURNAL_APPENDS);
    let _ = remove_dir_all(&directory);

    assert_eq!(removed, 0);
}

#[test]
fn screen_evidence_queue_active_analysis_lease_blocks_sweep_and_restart_recovers_expired_lease() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let key = JournalKey::from_bytes([11; JOURNAL_KEY_BYTES]);
    let queue = ScreenEvidenceQueue::open(&directory, key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let job = screen_queue_job();
    queue
        .append_encrypted_image(&job, b"leased-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    let claimed = queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let racing_claim = queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let protected = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);

    let restarted =
        ScreenEvidenceQueue::open(&directory, key).expect_value(constants::error::JOURNAL_OPENS);
    let lease_path = restarted.path().with_extension("analysis-leases");
    let recovered = restarted
        .remove_expired_entries(
            "2099-01-01T00:00:00Z",
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let remaining_leases = std::fs::read_to_string(&lease_path).unwrap_or_default();
    let _ = remove_dir_all(&directory);

    assert_eq!(
        claimed.map(|entry| entry.queue_job_id),
        Some(job.queue_job_id.clone())
    );
    assert!(racing_claim.is_none());
    assert!(protected.expired_entries.is_empty());
    assert_eq!(protected.retained_count, 1);
    assert_eq!(recovered.expired_entries.len(), 1);
    assert_eq!(recovered.expired_entries[0].queue_job_id, job.queue_job_id);
    assert!(remaining_leases.trim().is_empty());
}

#[test]
fn screen_evidence_queue_refuses_to_lease_already_expired_entries() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([18; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let job = screen_queue_job_with_expiry(
        "screen-expired-before-claim",
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    );
    queue
        .append_encrypted_image(&job, b"expired-before-claim")
        .expect_value(constants::error::JOURNAL_APPENDS);

    let claimed = queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert!(claimed.is_none());
    assert_eq!(sweep.expired_entries.len(), 1);
}

#[test]
fn screen_evidence_queue_quarantines_malformed_leases_and_keeps_claiming() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([19; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    queue
        .append_encrypted_image(&screen_queue_job(), b"lease-recovery")
        .expect_value(constants::error::JOURNAL_APPENDS);
    let lease_path = queue.path().with_extension("analysis-leases");
    std::fs::write(&lease_path, b"{malformed-lease\n")
        .expect_value(constants::error::JOURNAL_APPENDS);

    let claimed = queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let quarantine_path = lease_path.with_extension("analysis-leases.quarantine");
    let quarantine_exists = quarantine_path.is_file();
    let _ = remove_dir_all(&directory);

    assert_eq!(
        claimed.map(|entry| entry.queue_job_id),
        Some(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string())
    );
    assert!(quarantine_exists);
}

#[test]
fn screen_evidence_queue_renewed_lease_stays_valid_for_the_running_job() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([20; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let job = screen_queue_job_with_expiry(
        "screen-renewed-analysis-lease",
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    );
    queue
        .append_encrypted_image(&job, b"renewed-lease")
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS)
        .expect_value(constants::error::JOURNAL_READS);
    let renewed = queue
        .renew_claimed_entry(
            "screen-renewed-analysis-lease",
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_APPENDS);
    let sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert!(renewed);
    assert!(sweep.expired_entries.is_empty());
    assert_eq!(sweep.retained_count, 1);
}

#[test]
fn screen_evidence_queue_claim_completion_removes_all_duplicate_job_records() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([12; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let job = screen_queue_job();
    queue
        .append_encrypted_image(&job, b"claimed-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .append_encrypted_image(&job, b"duplicate-claimed-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS)
        .expect_value(constants::error::JOURNAL_READS);

    queue
        .complete_claimed_entry(&job.queue_job_id)
        .expect_value(constants::error::JOURNAL_APPENDS);
    let remaining = queue
        .read_decrypted_entries(4)
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert!(remaining.is_empty());
}

#[test]
fn screen_evidence_queue_release_claim_keeps_entry_available_for_retry() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([23; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let job = screen_queue_job();
    queue
        .append_encrypted_image(&job, b"retryable-claimed-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS)
        .expect_value(constants::error::JOURNAL_READS);
    queue
        .release_claimed_entry(&job.queue_job_id)
        .expect_value(constants::error::JOURNAL_APPENDS);
    let reclaimed = queue
        .claim_first_decrypted_entry(
            1,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert_eq!(
        reclaimed.map(|entry| entry.queue_job_id),
        Some(job.queue_job_id)
    );
}

#[test]
fn screen_evidence_queue_quarantines_corrupt_outbox_and_continues_valid_deletions() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([13; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    let first = screen_queue_job();
    let second = screen_queue_job_with_id("screen-corrupt-outbox-second");
    queue
        .append_encrypted_image(&first, b"first-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let outbox_path = queue.path().with_extension("deletion-outbox");
    let mut outbox = OpenOptions::new()
        .append(true)
        .open(&outbox_path)
        .expect_value(constants::error::JOURNAL_OPENS);
    outbox
        .write_all(b"{not-valid-json}\n")
        .expect_value(constants::error::JOURNAL_APPENDS);
    outbox
        .sync_all()
        .expect_value(constants::error::JOURNAL_APPENDS);
    drop(outbox);
    queue
        .append_encrypted_image(&second, b"second-image")
        .expect_value(constants::error::JOURNAL_APPENDS);

    let sweep = queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let retry = queue
        .remove_expired_entries(
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let acknowledged = queue
        .acknowledge_outbox_failures(&retry.outbox_failures)
        .expect_value(constants::error::JOURNAL_APPENDS);
    let projected = queue
        .remove_expired_entries(
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let quarantine = read_to_string(outbox_path.with_extension("deletion-outbox-quarantine"))
        .expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert_eq!(sweep.expired_entries.len(), 2);
    assert_eq!(sweep.outbox_failures.len(), 1);
    assert_eq!(retry.outbox_failures, sweep.outbox_failures);
    assert_eq!(acknowledged, 1);
    assert!(projected.outbox_failures.is_empty());
    let quarantined: Value =
        serde_json::from_str(quarantine.trim()).expect_value(constants::error::JOURNAL_READS);
    assert_eq!(quarantined["rawRecord"].as_str(), Some("{not-valid-json}"));
    assert_eq!(quarantined["lineNumber"].as_u64(), Some(2));
}

#[test]
fn screen_evidence_queue_noop_poll_preserves_outbox_identity_and_mtime() {
    let directory = temp_queue_dir();
    let _ = remove_dir_all(&directory);
    let queue =
        ScreenEvidenceQueue::open(&directory, JournalKey::from_bytes([14; JOURNAL_KEY_BYTES]))
            .expect_value(constants::error::JOURNAL_OPENS);
    queue
        .append_encrypted_image(&screen_queue_job(), b"expired-image")
        .expect_value(constants::error::JOURNAL_APPENDS);
    queue
        .remove_expired_entries(
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let outbox_path = queue.path().with_extension("deletion-outbox");
    let before_metadata =
        std::fs::metadata(&outbox_path).expect_value(constants::error::JOURNAL_READS);
    let before_contents =
        read_to_string(&outbox_path).expect_value(constants::error::JOURNAL_READS);

    let replay = queue
        .remove_expired_entries(
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
            SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
        )
        .expect_value(constants::error::JOURNAL_READS);
    let after_metadata =
        std::fs::metadata(&outbox_path).expect_value(constants::error::JOURNAL_READS);
    let after_contents = read_to_string(&outbox_path).expect_value(constants::error::JOURNAL_READS);
    let _ = remove_dir_all(&directory);

    assert_eq!(replay.expired_entries.len(), 1);
    assert_eq!(before_contents, after_contents);
    assert_eq!(
        before_metadata
            .modified()
            .expect_value(constants::error::JOURNAL_READS),
        after_metadata
            .modified()
            .expect_value(constants::error::JOURNAL_READS)
    );
    assert_eq!(before_metadata.len(), after_metadata.len());
}

fn screen_queue_job() -> ScreenAnalysisQueueJob {
    screen_queue_job_with_id(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID)
}

fn screen_queue_job_with_id(queue_job_id: impl Display) -> ScreenAnalysisQueueJob {
    screen_queue_job_with_expiry(
        queue_job_id,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
}

fn screen_queue_job_with_expiry(
    queue_job_id: impl Display,
    expires_at: impl Display,
) -> ScreenAnalysisQueueJob {
    let queue_job_id = TestText::from_display(queue_job_id);
    let expires_at = TestText::from_display(expires_at);
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: queue_job_id.to_string(),
        created_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        not_before: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at: expires_at.to_string(),
        last_attempt_at: None,
        capture_reason: SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_string(),
        capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
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
        image_format: SCREEN_IMAGE_FORMAT_PNG.to_string(),
        status: SCREEN_QUEUE_STATUS_QUEUED.to_string(),
        attempt_count: 0,
        max_retry_count: 2,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: SCREEN_DELETION_REQUIRED.to_string(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
    }
}

fn temp_queue_dir() -> TestPath {
    let mut name = String::from(constants::activity_store::TEST_FILE_PREFIX);
    name.push_str(&std::process::id().to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(&TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed).to_string());
    name.push(constants::delimiter::HYPHEN);
    name.push_str(constants::activity_store::TEST_SCREEN_QUEUE_SUFFIX);
    let mut path = std::env::temp_dir();
    path.push(name);
    TestPath(path)
}
