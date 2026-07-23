use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, IdempotencyKey};
use ocentra_eventing::journal::ndjson::{
    NdjsonEventJournal, NdjsonJournalEntry, NdjsonJournalOptions,
};
use ocentra_eventing::journal::EventJournal;
use std::sync::Arc;
use tokio::sync::Barrier;

use super::{
    super::fixtures::{test_event, test_event_for_type, TestText, OTHER_EVENT_TYPE, TEST_LABEL},
    support::{
        cleanup, journal_path, read_lines, stored_event, tamper_first_journal_payload_label,
        JournalPath,
    },
};

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
    let first_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[0]).expect_value("first line decodes");
    let second_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[1]).expect_value("second line decodes");

    assert_eq!(lines.len(), 2);
    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
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

    assert_eq!(repeated, first);
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
    assert_eq!(repeated, third_append);
    assert_eq!(read_lines(path.clone()).await.len(), 3);
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

async fn cleanup_idempotent_journal(path: JournalPath) {
    let lock_path = std::path::PathBuf::from(format!("{}.append.lock", path.0.display()));
    cleanup(path).await;
    let _cleanup_lock = tokio::fs::remove_file(lock_path).await;
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
