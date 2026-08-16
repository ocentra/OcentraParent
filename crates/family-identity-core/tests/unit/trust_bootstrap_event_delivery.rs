use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

use ocentra_eventing::ids::CorrelationId;
use ocentra_eventing::journal::ndjson::{NdjsonJournalEntry, NdjsonJournalRecord};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, ParentStepUpAssertionSnapshot,
};
use ocentra_family_identity_core::parent_presence::{
    ParentPresenceChallenge, ParentPresenceCustodyDecisionArtifact,
    ParentPresenceCustodyDecisionDelivery, ParentPresenceCustodyDecisionRedaction,
    ParentPresenceCustodyDecisionResult, ParentPresenceStorageFailureReason,
    ParentPresenceVerificationFailureReason, ParentPresenceVerificationInput,
    ParentPresenceVerificationPort,
};

use super::open_parent_presence_test_port;

#[path = "trust_bootstrap_event_delivery_recovery.rs"]
mod recovery_regressions;

const EXPIRY: &str = "2099-01-01T00:00:00.000Z";
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

type TestResult = Result<(), ParentPresenceStorageFailureReason>;

struct DeliveryStore {
    root: PathBuf,
    store_path: PathBuf,
}

impl DeliveryStore {
    fn new(prefix: &str) -> Self {
        let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "ocentra-parent-presence-delivery-{prefix}-{}-{id}",
            std::process::id()
        ));
        assert!(matches!(fs::create_dir_all(&root), Ok(())));
        let store_path = root.join("parent-presence.sqlite");
        Self { root, store_path }
    }

    fn port(&self) -> Result<ParentPresenceVerificationPort, ParentPresenceStorageFailureReason> {
        open_parent_presence_test_port(&self.store_path)
    }
}

impl Drop for DeliveryStore {
    fn drop(&mut self) {
        let _cleanup = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn custody_decisions_are_correlated_redacted_and_replayable_after_restart() -> TestResult {
    let store = DeliveryStore::new("correlated-replay");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "correlated-replay");

    assert!(port
        .verify_and_consume(input("correlated-replay", "accepted-correlation")?)
        .is_ok());
    assert_eq!(
        port.verify_and_consume(input("correlated-replay", "replay-correlation")?),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
    drop(port);

    let mut restarted = store.port()?;
    assert_eq!(
        restarted.verify_and_consume(input("correlated-replay", "restart-correlation")?),
        Err(ParentPresenceVerificationFailureReason::ReplayRejected)
    );
    let entries = journal_entries(&journal_path)?;
    let artifacts = decode_artifacts(&entries)?;

    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].append.sequence, 1);
    assert_eq!(entries[2].append.sequence, 3);
    assert_eq!(
        artifacts
            .iter()
            .map(|artifact| artifact.correlation_id.as_str())
            .collect::<Vec<_>>(),
        [
            "accepted-correlation",
            "replay-correlation",
            "restart-correlation"
        ]
    );
    assert_eq!(
        artifacts[0].result,
        ParentPresenceCustodyDecisionResult::Accepted
    );
    assert!(artifacts[1..].iter().all(|artifact| {
        artifact.result == ParentPresenceCustodyDecisionResult::ReplayRejected
    }));
    assert!(artifacts.iter().all(|artifact| {
        artifact.delivery == ParentPresenceCustodyDecisionDelivery::EventingJournal
            && artifact.redaction == ParentPresenceCustodyDecisionRedaction::SensitiveInputsOmitted
    }));
    assert_journal_redacted(&journal_path, "correlated-replay")?;
    Ok(())
}

#[test]
fn journal_failure_fails_closed_and_pending_acceptance_delivers_on_restart() -> TestResult {
    let store = DeliveryStore::new("failure-recovery");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    fs::create_dir(&journal_path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    issue_challenge(&mut port, "failure-recovery");

    assert_eq!(
        port.verify_and_consume(input("failure-recovery", "failure-correlation")?),
        Err(ParentPresenceVerificationFailureReason::CustodyUnavailable)
    );
    assert_eq!(outbox_state(&store.store_path)?, "pending");
    assert!(port.take_custody_artifact().is_none());
    drop(port);
    fs::remove_dir(&journal_path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;

    let restarted = store.port()?;
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    let artifacts = decode_artifacts(&journal_entries(&journal_path)?)?;
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0].result,
        ParentPresenceCustodyDecisionResult::Accepted
    );
    assert_eq!(artifacts[0].correlation_id.as_str(), "failure-correlation");
    drop(restarted);
    Ok(())
}

#[test]
fn sync_failure_keeps_outbox_pending_until_restart_durably_redelivers() -> TestResult {
    let store = DeliveryStore::new("sync-failure-recovery");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "sync-failure-recovery");
    port.inject_next_custody_journal_sync_failure_for_debug();

    assert_eq!(
        port.verify_and_consume(input("sync-failure-recovery", "sync-failure-correlation")?),
        Err(ParentPresenceVerificationFailureReason::CustodyUnavailable)
    );
    assert_eq!(outbox_state(&store.store_path)?, "pending");
    assert_eq!(journal_entries(&journal_path)?.len(), 1);
    drop(port);

    let restarted = store.port()?;
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    let artifacts = decode_artifacts(&journal_entries(&journal_path)?)?;
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0].result,
        ParentPresenceCustodyDecisionResult::Accepted
    );
    assert_eq!(
        artifacts[0].correlation_id.as_str(),
        "sync-failure-correlation"
    );
    drop(restarted);
    Ok(())
}

#[test]
fn partial_journal_write_keeps_outbox_pending_and_restart_repairs_tail() -> TestResult {
    let store = DeliveryStore::new("partial-write-recovery");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "partial-write-recovery");
    port.inject_next_custody_journal_partial_write_failure_for_debug();

    assert_eq!(
        port.verify_and_consume(input(
            "partial-write-recovery",
            "partial-write-correlation"
        )?),
        Err(ParentPresenceVerificationFailureReason::CustodyUnavailable)
    );
    assert_eq!(outbox_state(&store.store_path)?, "pending");
    let partial = fs::read(&journal_path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(partial.first(), Some(&b'{'));
    assert_ne!(partial.last(), Some(&b'\n'));
    drop(port);

    let restarted = store.port()?;
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    let artifacts = decode_artifacts(&journal_entries(&journal_path)?)?;
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0].correlation_id.as_str(),
        "partial-write-correlation"
    );
    drop(restarted);
    Ok(())
}

#[test]
fn directory_sync_failure_keeps_outbox_pending_until_durability_retry() -> TestResult {
    let store = DeliveryStore::new("directory-sync-recovery");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "directory-sync-recovery");
    port.inject_next_custody_journal_directory_sync_failure_for_debug();

    assert_eq!(
        port.verify_and_consume(input(
            "directory-sync-recovery",
            "directory-sync-correlation"
        )?),
        Err(ParentPresenceVerificationFailureReason::CustodyUnavailable)
    );
    assert_eq!(outbox_state(&store.store_path)?, "pending");
    assert_eq!(journal_entries(&journal_path)?.len(), 1);
    drop(port);

    let restarted = store.port()?;
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    assert_eq!(journal_entries(&journal_path)?.len(), 1);
    drop(restarted);
    Ok(())
}

#[test]
fn pending_redelivery_is_idempotent_across_restart() -> TestResult {
    let store = DeliveryStore::new("idempotent-restart");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "idempotent-restart");
    assert!(port
        .verify_and_consume(input("idempotent-restart", "idempotent-correlation")?)
        .is_ok());
    drop(port);
    set_outbox_pending(&store.store_path)?;

    let restarted = store.port()?;
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    let entries = journal_entries(&journal_path)?;
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].append.sequence, 1);
    drop(restarted);
    Ok(())
}

#[test]
fn concurrent_recovery_workers_claim_one_outbox_delivery_without_failing_other_ports() -> TestResult
{
    let store = DeliveryStore::new("concurrent-recovery-claim");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "concurrent-recovery-claim");
    assert!(port
        .verify_and_consume(input(
            "concurrent-recovery-claim",
            "concurrent-recovery-correlation"
        )?)
        .is_ok());
    drop(port);
    set_outbox_pending(&store.store_path)?;

    let workers = 8;
    let start = Arc::new(Barrier::new(workers));
    let joins = (0..workers)
        .map(|_| {
            let start = Arc::clone(&start);
            let store_path = store.store_path.clone();
            thread::spawn(move || {
                start.wait();
                open_parent_presence_test_port(store_path).is_ok()
            })
        })
        .collect::<Vec<_>>();
    for join in joins {
        assert!(join.join().unwrap_or(false));
    }

    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    assert_eq!(journal_entries(&journal_path)?.len(), 1);
    Ok(())
}

#[test]
fn recovery_waits_for_the_claimed_head_before_delivering_newer_decisions() -> TestResult {
    let store = DeliveryStore::new("claimed-head-order");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "claimed-head-first");
    issue_challenge(&mut port, "claimed-head-second");
    assert!(port
        .verify_and_consume(input(
            "claimed-head-first",
            "claimed-head-first-correlation"
        )?)
        .is_ok());
    assert!(port
        .verify_and_consume(input(
            "claimed-head-second",
            "claimed-head-second-correlation"
        )?)
        .is_ok());
    drop(port);
    fs::remove_file(&journal_path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    reset_outbox_with_claimed_head(&store.store_path, current_time_millis()?)?;

    let store_path = store.store_path.clone();
    let recovery = thread::spawn(move || open_parent_presence_test_port(store_path).is_ok());
    thread::sleep(std::time::Duration::from_millis(40));

    assert!(!recovery.is_finished());
    assert!(!journal_path.exists());
    release_all_outbox_claims(&store.store_path)?;
    assert!(recovery.join().unwrap_or(false));
    let artifacts = decode_artifacts(&journal_entries(&journal_path)?)?;
    assert_eq!(
        artifacts
            .iter()
            .map(|artifact| artifact.correlation_id.as_str())
            .collect::<Vec<_>>(),
        [
            "claimed-head-first-correlation",
            "claimed-head-second-correlation"
        ]
    );
    Ok(())
}

#[test]
fn recovery_reclaims_a_head_claim_when_it_becomes_stale_without_another_open() -> TestResult {
    let store = DeliveryStore::new("stale-claim-schedule");
    let mut port = store.port()?;
    let journal_path = port.custody_decision_journal_path().to_path_buf();
    issue_challenge(&mut port, "stale-claim-schedule");
    assert!(port
        .verify_and_consume(input("stale-claim-schedule", "stale-claim-correlation")?)
        .is_ok());
    drop(port);
    fs::remove_file(&journal_path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let nearly_stale = current_time_millis()?.saturating_sub(299_850);
    reset_outbox_with_claimed_head(&store.store_path, nearly_stale)?;

    let store_path = store.store_path.clone();
    let recovery = thread::spawn(move || open_parent_presence_test_port(store_path).is_ok());
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(2);
    while !recovery.is_finished() && std::time::Instant::now() < deadline {
        thread::sleep(std::time::Duration::from_millis(20));
    }

    assert!(recovery.is_finished());
    assert!(recovery.join().unwrap_or(false));
    assert_eq!(outbox_state(&store.store_path)?, "delivered");
    let artifacts = decode_artifacts(&journal_entries(&journal_path)?)?;
    assert_eq!(artifacts.len(), 1);
    assert_eq!(
        artifacts[0].correlation_id.as_str(),
        "stale-claim-correlation"
    );
    Ok(())
}

#[test]
fn custody_journal_suffix_preserves_distinct_store_extensions() -> TestResult {
    let store = DeliveryStore::new("extension-collision");
    let db_path = store.root.join("parent-presence.db");
    let sqlite_path = store.root.join("parent-presence.sqlite");
    let mut db_port = open_parent_presence_test_port(&db_path)?;
    let mut sqlite_port = open_parent_presence_test_port(&sqlite_path)?;
    let db_journal = db_port.custody_decision_journal_path().to_path_buf();
    let sqlite_journal = sqlite_port.custody_decision_journal_path().to_path_buf();

    assert_ne!(db_journal, sqlite_journal);
    assert_eq!(
        db_journal.file_name().and_then(|name| name.to_str()),
        Some("parent-presence.db.custody-decisions.ndjson")
    );
    assert_eq!(
        sqlite_journal.file_name().and_then(|name| name.to_str()),
        Some("parent-presence.sqlite.custody-decisions.ndjson")
    );
    issue_challenge(&mut db_port, "extension-db");
    issue_challenge(&mut sqlite_port, "extension-sqlite");
    assert!(db_port
        .verify_and_consume(input("extension-db", "extension-db-correlation")?)
        .is_ok());
    assert!(sqlite_port
        .verify_and_consume(input("extension-sqlite", "extension-sqlite-correlation")?)
        .is_ok());
    assert_eq!(journal_entries(&db_journal)?.len(), 1);
    assert_eq!(journal_entries(&sqlite_journal)?.len(), 1);
    Ok(())
}

fn issue_challenge(port: &mut ParentPresenceVerificationPort, scope: &str) {
    assert_eq!(
        port.issue_challenge(ParentPresenceChallenge {
            challenge_ref: format!("{scope}-challenge"),
            nonce_ref: format!("{scope}-nonce"),
            family_id: format!("{scope}-family"),
            parent_account_id: format!("{scope}-parent"),
            privileged_action: HouseholdAuthorityAction::PairChildDevice,
            action_device_id: format!("{scope}-device"),
            action_device_child_profile_id: Some(format!("{scope}-action-child")),
            target_child_profile_id: Some(format!("{scope}-target-child")),
            expires_at: EXPIRY.to_owned(),
        }),
        Ok(())
    );
}

fn input(
    scope: &str,
    correlation: &str,
) -> Result<ParentPresenceVerificationInput, ParentPresenceStorageFailureReason> {
    Ok(ParentPresenceVerificationInput {
        correlation_id: CorrelationId::parse(correlation)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?,
        challenge_ref: format!("{scope}-challenge"),
        assertion: ParentStepUpAssertionSnapshot {
            family_id: format!("{scope}-family"),
            parent_account_id: format!("{scope}-parent"),
            action_device_id: format!("{scope}-device"),
            action_device_child_profile_id: Some(format!("{scope}-action-child")),
            target_child_profile_id: Some(format!("{scope}-target-child")),
            action: HouseholdAuthorityAction::PairChildDevice,
            nonce: format!("{scope}-nonce"),
            expires_at: EXPIRY.to_owned(),
        },
    })
}

fn journal_entries(
    path: &Path,
) -> Result<Vec<NdjsonJournalEntry>, ParentPresenceStorageFailureReason> {
    let text = fs::read_to_string(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    text.lines()
        .enumerate()
        .map(|(index, line)| {
            NdjsonJournalRecord::parse(line, index + 1)
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
        })
        .filter_map(|record| match record {
            Ok(record) => record.entry().map(Ok),
            Err(error) => Some(Err(error)),
        })
        .collect()
}

fn decode_artifacts(
    entries: &[NdjsonJournalEntry],
) -> Result<Vec<ParentPresenceCustodyDecisionArtifact>, ParentPresenceStorageFailureReason> {
    entries
        .iter()
        .map(|entry| {
            entry
                .envelope
                .decode::<ParentPresenceCustodyDecisionArtifact>()
                .map(|envelope| envelope.payload)
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
        })
        .collect()
}

fn assert_journal_redacted(path: &Path, scope: &str) -> TestResult {
    let text = fs::read_to_string(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    for protected in [
        format!("{scope}-challenge"),
        format!("{scope}-nonce"),
        format!("{scope}-family"),
        format!("{scope}-parent"),
        format!("{scope}-device"),
        format!("{scope}-action-child"),
        format!("{scope}-target-child"),
    ] {
        assert_eq!(text.find(&protected), None);
    }
    Ok(())
}

fn outbox_state(path: &Path) -> Result<String, ParentPresenceStorageFailureReason> {
    let connection = rusqlite::Connection::open(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .query_row(
            "SELECT delivery_state FROM parent_presence_decision_outbox",
            [],
            |row| row.get(0),
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
}

fn set_outbox_pending(path: &Path) -> TestResult {
    let connection = rusqlite::Connection::open(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let changed = connection
        .execute(
            "UPDATE parent_presence_decision_outbox
             SET delivery_state = 'pending', delivery_claim = NULL, delivery_claimed_at = NULL",
            [],
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(changed, 1);
    Ok(())
}

fn reset_outbox_with_claimed_head(path: &Path, claimed_at: i64) -> TestResult {
    let connection = rusqlite::Connection::open(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute(
            "UPDATE parent_presence_decision_outbox
             SET delivery_state = 'pending', delivery_claim = NULL, delivery_claimed_at = NULL",
            [],
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    let changed = connection
        .execute(
            "UPDATE parent_presence_decision_outbox
             SET delivery_state = 'claimed',
                 delivery_claim = 'interrupted-worker',
                 delivery_claimed_at = ?1
             WHERE rowid = (
                 SELECT MIN(rowid) FROM parent_presence_decision_outbox
             )",
            [claimed_at],
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    assert_eq!(changed, 1);
    Ok(())
}

fn release_all_outbox_claims(path: &Path) -> TestResult {
    let connection = rusqlite::Connection::open(path)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    connection
        .execute(
            "UPDATE parent_presence_decision_outbox
             SET delivery_state = 'pending', delivery_claim = NULL, delivery_claimed_at = NULL
             WHERE delivery_state = 'claimed'",
            [],
        )
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
    Ok(())
}

fn current_time_millis() -> Result<i64, ParentPresenceStorageFailureReason> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
        .and_then(|duration| {
            i64::try_from(duration.as_millis())
                .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)
        })
}
