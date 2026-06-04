use std::sync::Arc;

use crate::{
    CorrelationId, DispatchMode, EventBus, EventJournal, EventingError, NdjsonEventJournal,
    ReplayCursor, ReplayFilter, ReplayMode,
};

use super::{
    super::fixtures::{
        subscriber, test_event, test_event_for_type, TestEvent, OTHER_EVENT_TYPE, TEST_EVENT_TYPE,
        TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
    },
    support::{cleanup, event_type, journal_path, stored_event},
};

#[tokio::test]
async fn replay_cursor_and_filters_read_ordered_projection_records() {
    let path = journal_path("replay-filters");
    let journal = NdjsonEventJournal::new(&path);
    let first = stored_event(test_event(TEST_LABEL));
    let second = stored_event(test_event_for_type("other", OTHER_EVENT_TYPE));
    let mut third = stored_event(test_event("third"));
    third.correlation_id = CorrelationId::parse("correlation-replay-3").expect("correlation");

    journal.append(&first).await.expect("append first");
    journal.append(&second).await.expect("append second");
    journal.append(&third).await.expect("append third");

    let report = journal
        .replay_projection(
            ReplayFilter::for_event_type(event_type(TEST_EVENT_TYPE))
                .with_correlation_id(third.correlation_id.clone())
                .with_cursor(ReplayCursor::after(1)),
        )
        .await
        .expect("projection replay reads");

    assert_eq!(report.mode, ReplayMode::ProjectionOnly);
    assert_eq!(report.records.len(), 1);
    assert_eq!(report.records[0].sequence, 3);
    assert_eq!(report.records[0].envelope.event_id, third.event_id);
    assert_eq!(report.cursor.next_sequence, 4);
    assert_eq!(report.skipped_count, 2);
    cleanup(&path).await;
}

#[tokio::test]
async fn replay_corrupt_line_is_reported_explicitly() {
    let path = journal_path("corrupt-line");
    tokio::fs::write(&path, "not-json\n")
        .await
        .expect("write corrupt journal");
    let journal = NdjsonEventJournal::new(&path);

    let result = journal.replay_projection(ReplayFilter::all()).await;

    assert!(matches!(
        result,
        Err(EventingError::JournalCorruptLine { line: 1, .. })
    ));
    cleanup(&path).await;
}

#[tokio::test]
async fn projection_replay_cannot_run_handlers_without_action_mode() {
    let path = journal_path("projection-gate");
    let journal = NdjsonEventJournal::new(&path);
    journal
        .append(&stored_event(test_event(TEST_LABEL)))
        .await
        .expect("append event");
    let projection = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect("projection replay");
    let bus = EventBus::new();

    let blocked = bus
        .replay_to_handlers(
            projection.records.clone(),
            projection.mode,
            DispatchMode::Sequential,
        )
        .await;
    assert!(matches!(
        blocked,
        Err(EventingError::ReplayActionNotAllowed { .. })
    ));

    let handled = Arc::new(tokio::sync::Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let handled = Arc::clone(&handled_clone);
        async move {
            *handled.lock().await += 1;
            Ok(())
        }
    })
    .await
    .expect("subscriber registers");
    let action = journal
        .replay_action_records(ReplayFilter::all())
        .await
        .expect("action replay reads");
    let reports = bus
        .replay_to_handlers(action.records, action.mode, DispatchMode::Sequential)
        .await
        .expect("action replay dispatches");

    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].handled_count, 1);
    assert_eq!(*handled.lock().await, 1);
    cleanup(&path).await;
}
