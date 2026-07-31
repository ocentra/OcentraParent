use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, IdempotencyKey};
use ocentra_eventing::journal::ndjson::{
    NdjsonEventJournal, NdjsonJournalEntry, NdjsonJournalOptions, NdjsonJournalRecord,
};
use ocentra_eventing::journal::production_file::ProductionFileEventJournal;
use ocentra_eventing::journal::{EventJournal, JournalAppendDurability, JournalHashVersion};
use std::sync::Arc;
use tokio::sync::Barrier;

use super::{
    super::fixtures::{test_event, test_event_for_type, TestText, OTHER_EVENT_TYPE, TEST_LABEL},
    support::{
        cleanup, cleanup_idempotent_journal, journal_path, read_lines, stored_event,
        tamper_first_journal_payload_label, JournalPath,
    },
};

#[cfg(not(target_os = "windows"))]
use super::support::append_lock_path;

#[tokio::test]
async fn ndjson_journal_appends_one_object_per_line_with_hash_chain() {
    let path = journal_path(TestText("hash-chain".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let first = stored_event(test_event(TestText(TEST_LABEL.to_owned())));
    let second = stored_event(test_event_for_type(
        TestText("second event".to_owned()),
        TestText(OTHER_EVENT_TYPE.to_owned()),
    ));

    let first_append = journal.append(&first).await.expect_value("first append");
    let second_append = journal.append(&second).await.expect_value("second append");

    let lines = read_lines(path.clone()).await;
    let entries = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| {
            NdjsonJournalRecord::parse(line, index + 1)
                .expect_value("journal record decodes")
                .entry()
        })
        .collect::<Vec<_>>();
    let first_entry = &entries[0];
    let second_entry = &entries[1];

    assert_eq!(entries.len(), 2);
    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
    assert_eq!(first_append.hash_version, JournalHashVersion::V3);
    assert_eq!(
        first_append.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(first_append.is_synchronized());
    let mut forged_synchronization = first_append.clone();
    forged_synchronization.synchronization_hash = None;
    assert!(
        !forged_synchronization.is_synchronized(),
        "a V3 receipt cannot claim synchronized durability without its authenticated completion marker"
    );
    let mut mismatched_synchronization = first_append.clone();
    mismatched_synchronization.synchronization_hash = first_append.current_hash.clone();
    assert!(
        !mismatched_synchronization.is_synchronized(),
        "a V3 receipt cannot substitute its buffered entry hash for the authenticated completion marker"
    );
    assert_eq!(
        first_entry.append.durability,
        JournalAppendDurability::Buffered,
        "the immutable data line cannot claim sync before it happened"
    );
    assert_eq!(
        first_entry.append.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(
        first_entry.append.synchronization_hash.is_none(),
        "the immutable pre-sync line must not contain a post-sync completion marker"
    );
    assert!(
        !first_entry.append.is_synchronized(),
        "the persisted buffered line remains ineligible as a durable receipt"
    );
    assert!(first_append.previous_hash.is_none());
    assert_eq!(first_entry.append.current_hash, first_append.current_hash);
    assert_eq!(second_append.previous_hash, first_append.current_hash);
    assert_eq!(second_entry.append.current_hash, second_append.current_hash);
    assert_eq!(first_entry.envelope.event_id, first.event_id);
    assert_eq!(
        first_entry.envelope.contract.schema_version,
        first.contract.schema_version
    );
    cleanup(path).await;
}

#[tokio::test]
async fn production_file_journal_synchronizes_a_bare_relative_path() {
    let path = JournalPath(std::path::PathBuf::from(format!(
        "ocentra-production-bare-relative-{}.journal",
        EventId::generated().as_str()
    )));
    let journal = ProductionFileEventJournal::new(&path.0);
    let event = stored_event(test_event(TestText("bare relative journal".to_owned())));

    let append = journal
        .append(&event)
        .await
        .expect_value("bare relative production journal append synchronizes");

    assert!(append.is_synchronized());
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    let mut lock_path = path.0.as_os_str().to_os_string();
    lock_path.push(".append.lock");
    cleanup(path).await;
    let _cleanup_lock = tokio::fs::remove_file(std::path::PathBuf::from(lock_path)).await;
}

#[tokio::test]
async fn production_file_journal_retains_authenticated_tail_between_appends() {
    let path = journal_path(TestText("production-retained-tail".to_owned()));
    let journal = ProductionFileEventJournal::new(&path.0);
    let first = stored_event(test_event(TestText("production first".to_owned())));
    let second = stored_event(test_event(TestText("production second".to_owned())));
    let third = stored_event(test_event(TestText("production third".to_owned())));

    let first_append = journal
        .append(&first)
        .await
        .expect_value("first production append");
    let second_append = journal
        .append(&second)
        .await
        .expect_value("second production append");
    let third_append = journal
        .append(&third)
        .await
        .expect_value("third production append");

    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
    assert_eq!(third_append.sequence, 3);
    assert_eq!(journal.recovery_count_for_debug(), 1);
    assert_eq!(read_lines(path.clone()).await.len(), 3);
    cleanup(path).await;
}

#[tokio::test]
async fn ndjson_journal_reopen_continues_sequence_and_hash_chain() {
    let path = journal_path(TestText("reopen-hash-chain".to_owned()));
    let first_journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let first = stored_event(test_event(TestText(TEST_LABEL.to_owned())));
    let first_append = first_journal
        .append(&first)
        .await
        .expect_value("first append");
    drop(first_journal);

    let reopened = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let second = stored_event(test_event_for_type(
        TestText("second event".to_owned()),
        TestText(OTHER_EVENT_TYPE.to_owned()),
    ));
    let second_append = reopened.append(&second).await.expect_value("second append");
    let lines = read_lines(path.clone()).await;
    let second_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[1]).expect_value("second line decodes");

    assert_eq!(lines.len(), 2);
    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
    assert_eq!(second_append.previous_hash, first_append.current_hash);
    assert_eq!(second_entry.append.previous_hash, first_append.current_hash);
    assert_eq!(second_entry.append.current_hash, second_append.current_hash);
    cleanup(path).await;
}

#[tokio::test]
async fn ndjson_idempotent_append_survives_reopen_without_duplicate_lines() {
    let path = journal_path(TestText("idempotent-reopen".to_owned()));
    let event = stored_event(test_event(TestText(TEST_LABEL.to_owned())));
    let first = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain())
        .append_idempotent(&event)
        .await
        .expect_value("first idempotent append");

    let reopened = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let repeated = reopened
        .append_idempotent(&event)
        .await
        .expect_value("repeated idempotent append");
    let lines = read_lines(path.clone()).await;

    assert_eq!(repeated.sequence, first.sequence);
    assert_eq!(repeated.current_hash, first.current_hash);
    assert_eq!(repeated.durability, JournalAppendDurability::Synchronized);
    assert_eq!(
        repeated.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(repeated.is_synchronized());
    assert!(repeated.has_verified_synchronization_proof());
    assert_eq!(lines.len(), 1);
    cleanup(path).await;
}

#[tokio::test]
async fn ndjson_idempotent_append_rejects_key_reuse_for_a_different_event() {
    let path = journal_path(TestText("idempotent-collision".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let event = stored_event(test_event(TestText(TEST_LABEL.to_owned())));
    journal
        .append_idempotent(&event)
        .await
        .expect_value("first idempotent append");
    let mut collision = event.clone();
    collision.event_id = EventId::parse("different-event-id").expect_value("event id");

    let result = journal.append_idempotent(&collision).await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    cleanup(path).await;
}

#[tokio::test]
async fn idempotent_retry_rejects_a_collision_even_when_an_exact_record_exists_later() {
    use ocentra_eventing::journal::policy::JournalDispatchPhase;

    let path = journal_path(TestText("idempotent-collision-before-exact".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let exact = unique_stored_event("collision before exact", 52);
    let mut collision = exact.clone();
    collision.idempotency_key =
        IdempotencyKey::parse("conflicting-idempotency-key").expect_value("idempotency key");
    journal
        .append_phase(&collision, JournalDispatchPhase::AfterDispatch)
        .await
        .expect_value("conflicting ordinary append");
    journal
        .append_phase(&exact, JournalDispatchPhase::AfterDispatch)
        .await
        .expect_value("later exact ordinary append");

    let result = journal.append_idempotent(&exact).await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateEventId { .. })
    ));
    assert_eq!(read_lines(path.clone()).await.len(), 2);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn idempotent_restart_truncates_only_an_incomplete_trailing_record() {
    let path = journal_path(TestText("idempotent-partial-recovery".to_owned()));
    let event = unique_stored_event("partial recovery", 43);
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    journal.inject_next_partial_write_failure_for_debug();

    let failed = journal.append_idempotent(&event).await;

    assert!(matches!(failed, Err(EventingError::JournalIo { .. })));
    let partial = tokio::fs::read(&path.0)
        .await
        .expect_value("partial record remains");
    assert_eq!(partial.first(), Some(&b'{'));
    assert_ne!(partial.last(), Some(&b'\n'));
    let restarted = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let append = restarted
        .append_idempotent(&event)
        .await
        .expect_value("restart repairs incomplete trailing record");
    assert_eq!(append.sequence, 1);
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn idempotent_recovery_preserves_a_hash_valid_final_record_without_newline() {
    let path = journal_path(TestText("idempotent-valid-tail-no-newline".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let first = unique_stored_event("valid final record", 53);
    let second = unique_stored_event("after valid final record", 54);
    let first_append = journal
        .append_idempotent(&first)
        .await
        .expect_value("first append");
    let length = tokio::fs::metadata(&path.0)
        .await
        .expect_value("journal metadata")
        .len();
    tokio::fs::OpenOptions::new()
        .write(true)
        .open(&path.0)
        .await
        .expect_value("journal opens for one-byte truncation")
        .set_len(length.saturating_sub(1))
        .await
        .expect_value("trailing newline truncates");

    let second_append = journal
        .append_idempotent(&second)
        .await
        .expect_value("append repairs valid final record");
    let lines = read_lines(path.clone()).await;

    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
    assert_eq!(second_append.previous_hash, first_append.current_hash);
    assert_eq!(lines.len(), 2);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn first_creation_directory_sync_failure_prevents_append_acknowledgement() {
    let path = journal_path(TestText("directory-sync-order".to_owned()));
    let event = unique_stored_event("directory sync", 44);
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    journal.inject_next_directory_sync_failure_for_debug();

    let failed = journal.append_idempotent(&event).await;

    assert!(matches!(failed, Err(EventingError::JournalIo { .. })));
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    let append = journal
        .append_idempotent(&event)
        .await
        .expect_value("retry syncs directory before acknowledgement");
    assert_eq!(append.sequence, 1);
    assert_eq!(append.durability, JournalAppendDurability::Synchronized);
    assert_eq!(
        append.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(append.is_synchronized());
    assert!(append.has_verified_synchronization_proof());
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    let restarted = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let recovered = restarted
        .append_idempotent(&event)
        .await
        .expect_value("restart reports only persisted achieved durability");
    assert_eq!(recovered.sequence, 1);
    assert_eq!(recovered.durability, JournalAppendDurability::Synchronized);
    assert_eq!(
        recovered.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(recovered.is_synchronized());
    assert!(recovered.has_verified_synchronization_proof());
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn ordinary_append_refreshes_state_after_a_failed_durable_sync() {
    let path = journal_path(TestText("ordinary-sync-failure-refresh".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let first = unique_stored_event("ordinary failed sync", 45);
    let second = unique_stored_event("ordinary retry", 46);
    journal.inject_next_sync_failure_for_debug();

    let failed = journal.append(&first).await;
    assert!(matches!(failed, Err(EventingError::JournalIo { .. })));
    let second_append = journal
        .append(&second)
        .await
        .expect_value("next ordinary append refreshes the durable tail");

    assert_eq!(second_append.sequence, 2);
    let entries = read_lines(path.clone())
        .await
        .into_iter()
        .map(|line| {
            serde_json::from_str::<NdjsonJournalEntry>(&line)
                .expect_value("ordinary append line decodes")
        })
        .collect::<Vec<_>>();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].append.sequence, 1);
    assert_eq!(entries[0].append.hash_version, JournalHashVersion::V3);
    assert_eq!(
        entries[0].append.durability,
        JournalAppendDurability::Buffered
    );
    assert_eq!(
        entries[0].append.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert_eq!(entries[1].append.sequence, 2);
    assert_eq!(
        entries[1].append.previous_hash,
        entries[0].append.current_hash
    );
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn replay_excludes_a_v3_entry_when_sync_fails_before_completion_marker() {
    use ocentra_eventing::replay::ReplayFilter;

    let path = journal_path(TestText("failed-sync-no-replay".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    journal.inject_next_sync_failure_for_debug();
    let event = unique_stored_event("failed sync cannot replay", 58);
    assert!(matches!(
        journal.append(&event).await,
        Err(EventingError::JournalIo { .. })
    ));
    let restarted = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let replay = restarted
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("replay reads incomplete durable record");
    assert!(
        replay.records.is_empty(),
        "a V3 record without a verified completion marker is not replayable"
    );
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn ordinary_and_idempotent_journal_instances_share_the_append_file_lock() {
    let path = journal_path(TestText("mixed-append-file-lock".to_owned()));
    let barrier = Arc::new(Barrier::new(2));
    let ordinary_path = path.clone();
    let ordinary_barrier = Arc::clone(&barrier);
    let ordinary = tokio::spawn(async move {
        let journal =
            NdjsonEventJournal::with_options(&ordinary_path, NdjsonJournalOptions::hash_chain());
        let event = unique_stored_event("mixed ordinary", 47);
        ordinary_barrier.wait().await;
        journal
            .append(&event)
            .await
            .expect_value("ordinary append succeeds")
    });
    let idempotent_path = path.clone();
    let idempotent_barrier = Arc::clone(&barrier);
    let idempotent = tokio::spawn(async move {
        let journal =
            NdjsonEventJournal::with_options(&idempotent_path, NdjsonJournalOptions::hash_chain());
        let event = unique_stored_event("mixed idempotent", 48);
        idempotent_barrier.wait().await;
        journal
            .append_idempotent(&event)
            .await
            .expect_value("idempotent append succeeds")
    });
    let mut appends = [
        ordinary.await.expect_value("ordinary task joins"),
        idempotent.await.expect_value("idempotent task joins"),
    ];
    appends.sort_by_key(|append| append.sequence);

    assert_eq!(
        appends
            .iter()
            .map(|append| append.sequence)
            .collect::<Vec<_>>(),
        vec![1, 2]
    );
    assert_eq!(appends[1].previous_hash, appends[0].current_hash);
    assert_eq!(read_lines(path.clone()).await.len(), 2);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn alternating_idempotent_journal_instances_refresh_the_hash_chain_tail() {
    let path = journal_path(TestText("idempotent-alternating-instances".to_owned()));
    let first_journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let second_journal =
        NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let first = unique_stored_event("first alternating event", 1);
    let second = unique_stored_event("second alternating event", 2);
    let third = unique_stored_event("third alternating event", 3);

    let first_append = first_journal
        .append_idempotent(&first)
        .await
        .expect_value("first append");
    let second_append = second_journal
        .append_idempotent(&second)
        .await
        .expect_value("second append");
    let third_append = first_journal
        .append_idempotent(&third)
        .await
        .expect_value("third append");

    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
    assert_eq!(third_append.sequence, 3);
    assert_eq!(second_append.previous_hash, first_append.current_hash);
    assert_eq!(third_append.previous_hash, second_append.current_hash);
    let reopened = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let repeated = reopened
        .append_idempotent(&third)
        .await
        .expect_value("chain recovers after alternating instances");
    assert_eq!(repeated.sequence, third_append.sequence);
    assert_eq!(repeated.current_hash, third_append.current_hash);
    assert_eq!(repeated.durability, JournalAppendDurability::Synchronized);
    assert_eq!(
        repeated.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(repeated.is_synchronized());
    assert!(repeated.has_verified_synchronization_proof());
    assert_eq!(read_lines(path.clone()).await.len(), 3);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn idempotent_retry_finds_after_dispatch_past_the_before_dispatch_copy() {
    use ocentra_eventing::journal::policy::JournalDispatchPhase;

    let path = journal_path(TestText("idempotent-after-before-copy".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let event = unique_stored_event("before and after", 49);
    journal
        .append_phase(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("before-dispatch append");
    let after = journal
        .append_phase(&event, JournalDispatchPhase::AfterDispatch)
        .await
        .expect_value("after-dispatch append");

    let repeated = journal
        .append_idempotent(&event)
        .await
        .expect_value("idempotent retry finds after-dispatch copy");

    assert_eq!(repeated.sequence, after.sequence);
    assert_eq!(repeated.current_hash, after.current_hash);
    assert_eq!(repeated.durability, JournalAppendDurability::Synchronized);
    assert_eq!(
        repeated.requested_durability,
        JournalAppendDurability::Synchronized
    );
    assert!(repeated.is_synchronized());
    assert!(repeated.has_verified_synchronization_proof());
    assert_eq!(read_lines(path.clone()).await.len(), 2);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn concurrent_idempotent_journal_instances_serialize_one_valid_hash_chain() {
    let path = journal_path(TestText("idempotent-concurrent-instances".to_owned()));
    let barrier = Arc::new(Barrier::new(8));
    let handles = (0..8)
        .map(|index| {
            let path = path.clone();
            let barrier = Arc::clone(&barrier);
            tokio::spawn(async move {
                let journal =
                    NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
                let event = unique_stored_event("concurrent idempotent event", index);
                barrier.wait().await;
                journal
                    .append_idempotent(&event)
                    .await
                    .expect_value("concurrent append")
            })
        })
        .collect::<Vec<_>>();
    let mut appends = Vec::new();
    for handle in handles {
        appends.push(handle.await.expect_value("append task joins"));
    }
    appends.sort_by_key(|append| append.sequence);

    assert_eq!(
        appends
            .iter()
            .map(|append| append.sequence)
            .collect::<Vec<_>>(),
        (1..=8).collect::<Vec<_>>()
    );
    for pair in appends.windows(2) {
        assert_eq!(pair[1].previous_hash, pair[0].current_hash);
    }
    let lines = read_lines(path.clone()).await;
    assert_eq!(lines.len(), 8);
    let reopened = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let ninth = unique_stored_event("ninth event", 9);
    assert_eq!(
        reopened
            .append_idempotent(&ninth)
            .await
            .expect_value("reopened chain append")
            .sequence,
        9
    );
    cleanup_idempotent_journal(path).await;
}

#[cfg(not(target_os = "windows"))]
#[tokio::test]
async fn non_windows_idempotent_append_waits_for_would_block_lock_contention() {
    use std::fs::OpenOptions;

    let path = journal_path(TestText("idempotent-would-block".to_owned()));
    let lock_path = append_lock_path(&path);
    let lock_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .expect_value("contention lock opens");
    lock_file.try_lock().expect_value("contention lock holds");
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let append = tokio::spawn(async move {
        journal
            .append_idempotent(&unique_stored_event("would-block event", 41))
            .await
    });

    tokio::time::sleep(std::time::Duration::from_millis(25)).await;
    assert!(!append.is_finished());
    lock_file.unlock().expect_value("contention lock releases");
    let append_receipt = append
        .await
        .expect_value("append task joins")
        .expect_value("append succeeds after contention");
    assert_eq!(append_receipt.sequence, 1);
    cleanup_idempotent_journal(path).await;
}

#[cfg(not(target_os = "windows"))]
#[tokio::test]
async fn non_windows_append_lock_open_error_maps_to_journal_io() {
    let path = journal_path(TestText("idempotent-lock-io-error".to_owned()));
    let lock_path = append_lock_path(&path);
    tokio::fs::create_dir(&lock_path)
        .await
        .expect_value("directory blocks lock-file open");
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());

    let result = journal
        .append_idempotent(&unique_stored_event("lock io error", 42))
        .await;

    assert!(matches!(
        result,
        Err(EventingError::JournalIo {
            path: error_path,
            reason,
        }) if error_path == path.0.display().to_string() && !reason.is_empty()
    ));
    let _cleanup_lock = tokio::fs::remove_dir(lock_path).await;
}

#[tokio::test]
async fn ndjson_journal_reopen_rejects_tampered_hash_chain_payload() {
    let path = journal_path(TestText("reopen-tampered-hash-chain".to_owned()));
    let first_journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    first_journal
        .append(&stored_event(test_event(TestText(TEST_LABEL.to_owned()))))
        .await
        .expect_value("first append");
    first_journal
        .append(&stored_event(test_event_for_type(
            TestText("second event".to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
        )))
        .await
        .expect_value("second append");
    drop(first_journal);
    tamper_first_journal_payload_label(path.clone(), TestText("tampered event".to_owned())).await;
    let reopened = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());

    let result = reopened
        .append(&stored_event(test_event_for_type(
            TestText("third event".to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
        )))
        .await;

    match result {
        Err(EventingError::JournalCorruptLine { line: 1, reason }) => {
            assert_eq!(
                reason.split(": expected ").next(),
                Some("journal hash-chain current hash mismatch at sequence 1")
            );
        }
        _other => std::process::abort(),
    }
    cleanup(path).await;
}

#[tokio::test]
async fn live_journal_rejects_same_length_hash_chain_tampering_before_append() {
    let path = journal_path(TestText("live-same-length-tamper".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    journal
        .append(&unique_stored_event("cached alpha", 50))
        .await
        .expect_value("first append");
    let original = tokio::fs::read_to_string(&path.0)
        .await
        .expect_value("journal reads");
    let tampered = original.replace("cached alpha 50", "cached omega 50");
    assert_eq!(tampered.len(), original.len());
    assert_ne!(tampered, original);
    tokio::fs::write(&path.0, tampered)
        .await
        .expect_value("same-length tamper writes");

    let result = journal
        .append(&unique_stored_event("after tamper", 51))
        .await;

    assert!(matches!(
        result,
        Err(EventingError::JournalCorruptLine { line: 1, .. })
    ));
    cleanup(path).await;
}

#[tokio::test]
async fn hash_chain_rejects_tampered_append_durability() {
    let path = journal_path(TestText("durability-hash-chain-tamper".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    journal
        .append(&unique_stored_event("durable event", 56))
        .await
        .expect_value("synchronized append");
    let original = tokio::fs::read_to_string(&path.0)
        .await
        .expect_value("journal reads");
    let tampered = original.replace(
        "\"requested_durability\":\"Synchronized\"",
        "\"requested_durability\":\"Buffered\"",
    );
    assert_ne!(
        tampered, original,
        "fixture must alter the authenticated durability claim"
    );
    tokio::fs::write(&path.0, tampered)
        .await
        .expect_value("durability tamper writes");

    let result = journal
        .append(&unique_stored_event("after durability tamper", 57))
        .await;
    assert!(matches!(
        result,
        Err(EventingError::JournalCorruptLine { line: 1, .. })
    ));
    cleanup(path).await;
}

fn unique_stored_event(
    label: &str,
    index: usize,
) -> ocentra_eventing::envelope::StoredEventEnvelope {
    let mut event = stored_event(test_event(TestText(format!("{label} {index}"))));
    event.event_id =
        EventId::parse(format!("idempotent-event-{index}")).expect_value("unique event id");
    event.idempotency_key = IdempotencyKey::parse(format!("idempotent-key-{index}"))
        .expect_value("unique idempotency key");
    event
}

#[tokio::test]
async fn concurrent_ndjson_appends_do_not_hold_state_lock_across_file_write() {
    let path = journal_path(TestText("concurrent-hash-chain".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let handles = (0..4)
        .map(|index| {
            let journal = journal.clone();
            tokio::spawn(async move {
                let event = stored_event(test_event_for_type(
                    TestText(format!("parallel event {index}")),
                    TestText(OTHER_EVENT_TYPE.to_owned()),
                ));
                journal.append(&event).await.expect_value("append succeeds")
            })
        })
        .collect::<Vec<_>>();

    for handle in handles {
        handle.await.expect_value("append task joins");
    }

    let lines = read_lines(path.clone()).await;
    let entries = lines
        .iter()
        .map(|line| serde_json::from_str::<NdjsonJournalEntry>(line).expect_value("line decodes"))
        .collect::<Vec<_>>();

    assert_eq!(entries.len(), 4);
    for (index, entry) in entries.iter().enumerate() {
        assert_eq!(entry.append.sequence, index as u64 + 1);
        if index == 0 {
            assert!(entry.append.previous_hash.is_none());
        } else {
            assert_eq!(
                entry.append.previous_hash,
                entries[index - 1].append.current_hash
            );
        }
    }
    cleanup(path).await;
}
