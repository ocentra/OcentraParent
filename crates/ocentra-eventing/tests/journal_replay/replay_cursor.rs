use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::journal::EventJournal;
use ocentra_eventing::replay::ReplayFilter;

use super::{
    super::fixtures::TestText,
    support::{cleanup_idempotent_journal, journal_path, stored_event},
};

#[tokio::test]
async fn replay_cursor_retries_a_v3_entry_after_its_later_activation() {
    let path = journal_path(TestText("replay-cursor-awaits-v3-activation".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let event = stored_event(super::super::fixtures::test_event(TestText(
        "activation follows first replay poll".to_owned(),
    )));
    journal
        .append(&event)
        .await
        .expect_value("journal appends a complete V3 record");

    let lines = tokio::fs::read_to_string(&path.0)
        .await
        .expect_value("complete V3 journal reads")
        .lines()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert_eq!(
        lines.len(),
        3,
        "V3 append writes entry, completion, activation"
    );
    tokio::fs::write(&path.0, format!("{}\n{}\n", lines[0], lines[1]))
        .await
        .expect_value("journal exposes the entry before activation arrives");

    let first_poll = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("first poll keeps its cursor before incomplete V3 entry");
    assert!(first_poll.records.is_empty());
    assert_eq!(first_poll.cursor.next_sequence, 1);

    tokio::fs::write(
        &path.0,
        format!("{}\n{}\n{}\n", lines[0], lines[1], lines[2]),
    )
    .await
    .expect_value("activation reaches the journal after the first poll");
    let second_poll = journal
        .replay_projection(ReplayFilter::all().with_cursor(first_poll.cursor))
        .await
        .expect_value("second poll retries the now activated V3 entry");

    assert_eq!(second_poll.records.len(), 1);
    assert_eq!(second_poll.records[0].sequence, 1);
    assert_eq!(second_poll.records[0].envelope.event_id, event.event_id);
    assert_eq!(second_poll.cursor.next_sequence, 2);
    cleanup_idempotent_journal(path).await;
}
