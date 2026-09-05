use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{EventId, IdempotencyKey};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_eventing::journal::production_file::ProductionFileEventJournal;
use ocentra_eventing::journal::EventJournal;
use ocentra_eventing::replay::ReplayFilter;

use super::super::fixtures::{test_event, TestText};
use super::support::{cleanup_idempotent_journal, journal_path, read_lines, stored_event};

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

#[tokio::test]
async fn production_file_idempotent_retry_after_sync_failure_replays_once() {
    let path = journal_path(TestText("production-idempotent-sync-failure".to_owned()));
    let journal = ProductionFileEventJournal::new(&path.0);
    let event = unique_stored_event("production retry sync failure", 62);
    journal.inject_next_sync_failure_for_debug();

    assert!(matches!(
        journal
            .append_phase_idempotent(
                &event,
                ocentra_eventing::journal::policy::JournalDispatchPhase::BeforeDispatch,
            )
            .await,
        Err(EventingError::JournalIo { .. })
    ));

    let acknowledged = journal
        .append_phase_idempotent(
            &event,
            ocentra_eventing::journal::policy::JournalDispatchPhase::BeforeDispatch,
        )
        .await
        .expect_value("production retry persists completion before acknowledgement");
    assert!(acknowledged.has_verified_synchronization_proof());

    let replay = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("production retry replays the completed event");
    assert_eq!(replay.records.len(), 1);
    assert_eq!(replay.records[0].envelope, event);
    assert_eq!(replay.skipped_count, 0);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn production_file_journal_recovers_and_replays_durable_entries() {
    let path = journal_path(TestText("production-recover-replay".to_owned()));
    let event = stored_event(test_event(TestText("production recover".to_owned())));
    let journal = ProductionFileEventJournal::new(&path.0);
    journal
        .append(&event)
        .await
        .expect_value("production append persists");
    drop(journal);

    let reopened = ProductionFileEventJournal::new(&path.0);
    reopened
        .recover()
        .await
        .expect_value("production journal recovers");
    let replay = reopened
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("production journal replays");

    assert_eq!(replay.records.len(), 1);
    assert_eq!(replay.records[0].envelope, event);
    assert_eq!(replay.skipped_count, 0);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn production_file_journal_keeps_before_and_after_rows_for_one_event_id() {
    let path = journal_path(TestText("production-phase-rows".to_owned()));
    let journal = ProductionFileEventJournal::new(&path.0);
    let event = stored_event(test_event(TestText("production phases".to_owned())));

    let before = journal
        .append_phase_idempotent(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("before-dispatch row persists");
    let after = journal
        .append_phase_idempotent(&event, JournalDispatchPhase::AfterDispatch)
        .await
        .expect_value("after-dispatch row persists");

    assert_eq!(before.sequence, 1);
    assert_eq!(after.sequence, 2);
    assert_eq!(before.sequence + 1, after.sequence);
    assert_eq!(read_lines(path.clone()).await.len(), 2);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn production_file_journal_retries_the_same_phase_without_duplicate_row() {
    let path = journal_path(TestText("production-phase-retry".to_owned()));
    let journal = ProductionFileEventJournal::new(&path.0);
    let event = stored_event(test_event(TestText("production retry".to_owned())));

    let first = journal
        .append_phase_idempotent(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("first idempotent phase append");
    let retry = journal
        .append_phase_idempotent(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("same phase retry returns existing acknowledgement");

    assert_eq!(retry.sequence, first.sequence);
    assert_eq!(retry.current_hash, first.current_hash);
    assert!(retry.has_verified_synchronization_proof());
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    assert_eq!(raw_journal_lines(&path).await.len(), 3);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn production_file_journal_retries_unverified_markers_fail_closed() {
    let path = journal_path(TestText(
        "production-marker-retry-completion-only".to_owned(),
    ));
    let event = unique_stored_event("production marker retry", 70);
    let journal = ProductionFileEventJournal::new(&path.0);
    journal
        .append_phase_idempotent(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("initial marker pair persists");
    let lines = raw_journal_lines(&path).await;
    write_raw_journal_lines(&path, &lines[..2]).await;
    drop(journal);

    let reopened = ProductionFileEventJournal::new(&path.0);
    let retry = reopened
        .append_phase_idempotent(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("unverified marker state retries fail closed");
    assert!(retry.has_verified_synchronization_proof());
    let replay = reopened
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("repaired marker state remains replayable");
    assert_eq!(replay.records.len(), 1);
    assert_eq!(replay.records[0].envelope, event);
    assert_eq!(read_lines(path.clone()).await.len(), 1);
    cleanup_idempotent_journal(path).await;
}

#[tokio::test]
async fn production_file_journal_rejects_conflicting_payload_for_existing_event_id() {
    let path = journal_path(TestText("production-event-id-conflict".to_owned()));
    let journal = ProductionFileEventJournal::new(&path.0);
    let event = stored_event(test_event(TestText("production original".to_owned())));
    journal
        .append_phase_idempotent(&event, JournalDispatchPhase::BeforeDispatch)
        .await
        .expect_value("original phase append");
    let mut conflict = stored_event(test_event(TestText("production conflict".to_owned())));
    conflict.event_id = event.event_id.clone();

    let result = journal
        .append_phase_idempotent(&conflict, JournalDispatchPhase::BeforeDispatch)
        .await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateEventId { event_id }) if event_id == event.event_id
    ));
    assert_eq!(read_lines(path.clone()).await.len(), 1);
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

async fn raw_journal_lines(path: &super::support::JournalPath) -> Vec<String> {
    tokio::fs::read_to_string(&path.0)
        .await
        .expect_value("raw journal reads")
        .lines()
        .map(str::to_owned)
        .collect()
}

async fn write_raw_journal_lines(path: &super::support::JournalPath, lines: &[String]) {
    let content = lines.join("\n");
    tokio::fs::write(&path.0, format!("{content}\n"))
        .await
        .expect_value("raw journal rewrite succeeds");
}
