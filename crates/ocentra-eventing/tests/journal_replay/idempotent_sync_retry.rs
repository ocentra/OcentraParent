use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, IdempotencyKey};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::replay::ReplayFilter;

use super::super::fixtures::{test_event, TestText};
use super::support::{cleanup_idempotent_journal, journal_path, stored_event};

#[tokio::test]
async fn idempotent_retry_after_sync_failure_persists_v3_completion_before_acknowledgement() {
    let path = journal_path(TestText("idempotent-sync-failure-completion".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let event = unique_stored_event("idempotent retry sync failure", 61);
    journal.inject_next_sync_failure_for_debug();

    assert!(matches!(
        journal.append_idempotent(&event).await,
        Err(EventingError::JournalIo { .. })
    ));

    let acknowledged = journal
        .append_idempotent(&event)
        .await
        .expect_value("idempotent retry persists V3 completion before acknowledgement");
    assert!(acknowledged.has_verified_synchronization_proof());
    let raw_records = tokio::fs::read_to_string(&path.0)
        .await
        .expect_value("idempotent retry journal reads");
    assert_eq!(raw_records.lines().count(), 3);

    let restarted = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    let replay = restarted
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("restart replays the durably completed idempotent event");
    assert_eq!(replay.records.len(), 1);
    assert_eq!(replay.skipped_count, 0);
    assert_eq!(replay.records[0].envelope, event);
    cleanup_idempotent_journal(path).await;
}

fn unique_stored_event(label: &str, index: usize) -> StoredEventEnvelope {
    let mut event = stored_event(test_event(TestText(format!("{label} {index}"))));
    event.event_id =
        EventId::parse(format!("idempotent-event-{index}")).expect_value("unique event id");
    event.idempotency_key = IdempotencyKey::parse(format!("idempotent-key-{index}"))
        .expect_value("unique idempotency key");
    event
}
