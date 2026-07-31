use ocentra_eventing::bus::{DispatchMode, EventBus};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::CorrelationId;
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::EventJournal;
use ocentra_eventing::replay::{ReplayCursor, ReplayFilter, ReplayMode};
use std::sync::Arc;

use super::{
    super::fixtures::{
        metadata, subscriber, test_event, test_event_for_type, test_event_with_idempotency,
        TestEvent, TestText, OTHER_EVENT_TYPE, TEST_EVENT_TYPE, TEST_LABEL, TEST_SUBSCRIBER,
        TEST_TARGET,
    },
    support::{
        cleanup, event_type, journal_path, stored_event, tamper_first_journal_payload_label,
    },
};

#[tokio::test]
async fn replay_cursor_and_filters_read_ordered_projection_records() {
    let path = journal_path(TestText("replay-filters".to_owned()));
    let journal = NdjsonEventJournal::new(&path);
    let first = stored_event(test_event(TestText(TEST_LABEL.to_owned())));
    let second = stored_event(test_event_for_type(
        TestText("other".to_owned()),
        TestText(OTHER_EVENT_TYPE.to_owned()),
    ));
    let mut third = stored_event(test_event(TestText("third".to_owned())));
    third.correlation_id = CorrelationId::parse("correlation-replay-3").expect_value("correlation");

    journal.append(&first).await.expect_value("append first");
    journal.append(&second).await.expect_value("append second");
    journal.append(&third).await.expect_value("append third");

    let report = journal
        .replay_projection(
            ReplayFilter::for_event_type(event_type(TestText(TEST_EVENT_TYPE.to_owned())))
                .with_correlation_id(third.correlation_id.clone())
                .with_cursor(ReplayCursor::after(1)),
        )
        .await
        .expect_value("projection replay reads");

    assert_eq!(report.mode, ReplayMode::ProjectionOnly);
    assert_eq!(report.records.len(), 1);
    assert_eq!(report.records[0].sequence, 3);
    assert_eq!(report.records[0].envelope.event_id, third.event_id);
    assert_eq!(report.cursor.next_sequence, 4);
    assert_eq!(report.skipped_count, 2);
    cleanup(path).await;
}

#[tokio::test]
async fn replay_corrupt_line_is_reported_explicitly() {
    let path = journal_path(TestText("corrupt-line".to_owned()));
    tokio::fs::write(&path, "not-json\n")
        .await
        .expect_value("write corrupt journal");
    let journal = NdjsonEventJournal::new(&path);

    let result = journal.replay_projection(ReplayFilter::all()).await;

    assert!(matches!(
        result,
        Err(EventingError::JournalCorruptLine { line: 1, .. })
    ));
    cleanup(path).await;
}

#[tokio::test]
async fn replay_rejects_tampered_hash_chain_payload() {
    let path = journal_path(TestText("replay-tampered-hash-chain".to_owned()));
    let journal = NdjsonEventJournal::with_options(&path, NdjsonJournalOptions::hash_chain());
    journal
        .append(&stored_event(test_event(TestText(TEST_LABEL.to_owned()))))
        .await
        .expect_value("first append");
    journal
        .append(&stored_event(test_event_for_type(
            TestText("second event".to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
        )))
        .await
        .expect_value("second append");
    tamper_first_journal_payload_label(path.clone(), TestText("tampered event".to_owned())).await;

    let result = journal.replay_projection(ReplayFilter::all()).await;

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
async fn action_replay_skips_two_before_dispatch_records_and_replays_later_actions() {
    let path = journal_path(TestText("queued-drain-action-replay".to_owned()));
    let journal = NdjsonEventJournal::new(&path);
    let before_dispatch_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::All),
        journal.clone().shared(),
    );
    before_dispatch_bus
        .publish(
            test_event_with_idempotency(
                TestText(TEST_LABEL.to_owned()),
                TestText("queued-drain-replay-key".to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("first no-subscriber event records only before-dispatch evidence");
    before_dispatch_bus
        .publish(
            test_event_with_idempotency(
                TestText("queued-drain-replay-second".to_owned()),
                TestText("queued-drain-replay-key-second".to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("second no-subscriber event records only before-dispatch evidence");
    let handled = Arc::new(tokio::sync::Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    let action_bus = EventBus::with_journal(
        JournalPolicy::after_dispatch(JournalSelector::All),
        journal.clone().shared(),
    );
    action_bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |_| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    *handled.lock().await += 1;
                    Ok(())
                }
            },
        )
        .await
        .expect_value("subscriber registers after non-actionable journal entries");
    action_bus
        .publish(
            test_event_with_idempotency(
                TestText("later-actionable-replay".to_owned()),
                TestText("later-actionable-replay-key".to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("later subscriber-backed event records actionable after-dispatch evidence");
    assert_eq!(*handled.lock().await, 1);

    let action = journal
        .replay_action_records(ReplayFilter::all())
        .await
        .expect_value(
            "action replay verifies skipped before-dispatch records before reading actions",
        );
    let projection = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("projection replay reads all journal records");
    let replay_bus = EventBus::new();
    let replay_handled = Arc::new(tokio::sync::Mutex::new(0_usize));
    let replay_handled_clone = Arc::clone(&replay_handled);
    replay_bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText("replay-subscriber".to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |_| {
                let handled = Arc::clone(&replay_handled_clone);
                async move {
                    *handled.lock().await += 1;
                    Ok(())
                }
            },
        )
        .await
        .expect_value("replay subscriber registers");
    let reports = replay_bus
        .replay_to_handlers(action.records, action.mode, DispatchMode::Sequential)
        .await
        .expect_value("action replay dispatches the later actionable record");

    assert_eq!(projection.records.len(), 3);
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].handled_count, 1);
    assert_eq!(*replay_handled.lock().await, 1);
    cleanup(path).await;
}

#[tokio::test]
async fn dropped_no_subscriber_event_never_becomes_an_after_dispatch_replay_action() {
    let path = journal_path(TestText("dropped-no-subscriber-action-replay".to_owned()));
    let journal = NdjsonEventJournal::new(&path);
    let dropped_bus = EventBus::with_journal(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal.clone().shared(),
    );

    let dropped = dropped_bus
        .publish(
            test_event_with_idempotency(
                TestText("dropped-no-subscriber".to_owned()),
                TestText("dropped-no-subscriber-key".to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("no-subscriber event completes without action evidence");

    assert_eq!(dropped.subscriber_count, 0);
    assert_eq!(dropped.journal_appends.len(), 1);
    let dropped_actions = journal
        .replay_action_records(ReplayFilter::all())
        .await
        .expect_value("dropped event never enters action replay");
    assert!(dropped_actions.records.is_empty());

    let handled = Arc::new(tokio::sync::Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    let action_bus = EventBus::with_journal(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal.clone().shared(),
    );
    action_bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |_| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    *handled.lock().await += 1;
                    Ok(())
                }
            },
        )
        .await
        .expect_value("subscriber registers for real action");
    action_bus
        .publish(
            test_event_with_idempotency(
                TestText("handled-after-dispatch".to_owned()),
                TestText("handled-after-dispatch-key".to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("subscriber-backed event creates action replay evidence");

    let actions = journal
        .replay_action_records(ReplayFilter::all())
        .await
        .expect_value("only handled event enters action replay");
    assert_eq!(actions.records.len(), 1);
    assert_eq!(
        actions.records[0].envelope.contract.event_type.as_str(),
        TEST_EVENT_TYPE
    );
    assert_eq!(*handled.lock().await, 1);
    cleanup(path).await;
}

#[tokio::test]
async fn projection_replay_cannot_run_handlers_without_action_mode() {
    let path = journal_path(TestText("projection-gate".to_owned()));
    let journal = NdjsonEventJournal::new(&path);
    journal
        .append(&stored_event(test_event(TestText(TEST_LABEL.to_owned()))))
        .await
        .expect_value("append event");
    let projection = journal
        .replay_projection(ReplayFilter::all())
        .await
        .expect_value("projection replay");
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
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let handled = Arc::clone(&handled_clone);
            async move {
                *handled.lock().await += 1;
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");
    let action = journal
        .replay_action_records(ReplayFilter::all())
        .await
        .expect_value("action replay reads");
    let reports = bus
        .replay_to_handlers(action.records, action.mode, DispatchMode::Sequential)
        .await
        .expect_value("action replay dispatches");

    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0].handled_count, 1);
    assert_eq!(*handled.lock().await, 1);
    cleanup(path).await;
}
