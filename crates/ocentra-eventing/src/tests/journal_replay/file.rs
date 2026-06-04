use crate::{EventJournal, NdjsonEventJournal, NdjsonJournalEntry, NdjsonJournalOptions};

use super::{
    super::fixtures::{test_event, test_event_for_type, OTHER_EVENT_TYPE, TEST_LABEL},
    support::{cleanup, journal_path, read_lines, stored_event},
};

#[tokio::test]
async fn ndjson_journal_appends_one_object_per_line_with_hash_chain() {
    let path = journal_path("hash-chain");
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let first = stored_event(test_event(TEST_LABEL));
    let second = stored_event(test_event_for_type("second event", OTHER_EVENT_TYPE));

    let first_append = journal.append(&first).await.expect("first append");
    let second_append = journal.append(&second).await.expect("second append");

    let lines = read_lines(&path).await;
    let first_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[0]).expect("first line decodes");
    let second_entry: NdjsonJournalEntry =
        serde_json::from_str(&lines[1]).expect("second line decodes");

    assert_eq!(lines.len(), 2);
    assert_eq!(first_append.sequence, 1);
    assert_eq!(second_append.sequence, 2);
    assert!(first_append.previous_hash.is_none());
    assert!(first_append.current_hash.is_some());
    assert_eq!(second_append.previous_hash, first_append.current_hash);
    assert_eq!(second_entry.append.current_hash, second_append.current_hash);
    assert_eq!(first_entry.envelope.event_id, first.event_id);
    assert_eq!(
        first_entry.envelope.contract.schema_version,
        first.contract.schema_version
    );
    cleanup(&path).await;
}
