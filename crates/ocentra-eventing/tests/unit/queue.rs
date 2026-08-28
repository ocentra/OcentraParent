use crate::ExpectValue;
use std::{sync::Arc, time::Duration};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::{future::Future, pin::Pin, sync::Mutex as StdMutex};
use tokio::sync::{Mutex, Notify};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type_with_aggregate_and_idempotency, test_event_with_idempotency, TestEvent,
    TestText, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER,
    TEST_TARGET,
};
use crate::{
    DispatchMode, DomainEvent, EventBus, EventJournal, EventQueuePolicy, EventingError,
    HandlerExecutionPolicy, JournalAppend, JournalPolicy, JournalSelector, ManualEventClock,
    QueueDisposition, QueueOverflowPolicy, StoredEventEnvelope,
};
use ocentra_eventing::bus::reports::dead_letter::{DeadLetterReason, DeadLetterRetryState};
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_eventing::journal::{JournalAppendDurability, JournalHashVersion};
use ocentra_eventing::queue::policy::NoSubscriberQueuePolicy;

#[path = "queue/no_subscriber_journal.rs"]
mod no_subscriber_journal;

fn failing_journal_result(
    call: usize,
    fail_once_on: usize,
    hash_version: JournalHashVersion,
) -> Result<JournalAppend, EventingError> {
    if call == fail_once_on {
        return Err(EventingError::JournalIo {
            path: String::from("failing-journal"),
            reason: String::from("intentional one-shot append failure"),
        });
    }

    Ok(JournalAppend {
        sequence: call as u64,
        previous_hash: None,
        current_hash: Some(
            crate::JournalHash::parse(format!("journal-hash-{call}"))
                .expect_value("journal hash parses"),
        ),
        hash_version,
        durability: JournalAppendDurability::Synchronized,
        requested_durability: JournalAppendDurability::Synchronized,
        synchronization_hash: None,
    })
}

fn require_verified_v3_receipt(append: &JournalAppend) -> Result<(), EventingError> {
    if append.has_verified_synchronization_proof() {
        return Ok(());
    }
    Err(EventingError::InvalidHandlerPolicy {
        reason: "test requires a verified V3 synchronization receipt".to_owned(),
    })
}

#[tokio::test]
async fn no_subscriber_queue_drains_after_subscriber_registers() {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(2).expect_value("queue policy is valid"),
    );
    let queued_report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("no-subscriber publish queues");
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let subscription = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await
        .expect_value("subscriber registers and drains queue");
    let empty_drain = bus
        .drain_queued(DispatchMode::Sequential)
        .await
        .expect_value("queue is already drained");

    assert_eq!(
        queued_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(queued_report.queue_report.queued_count, 1);
    assert_eq!(queued_report.subscriber_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    assert_eq!(
        subscription.event_type.as_str(),
        super::fixtures::TEST_EVENT_TYPE
    );
    assert_eq!(empty_drain.queued_before, 0);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
}

#[tokio::test]
async fn no_subscriber_validated_publish_rejects_an_invalid_v3_receipt_before_completion() {
    let journal = Arc::new(FailingJournal::with_invalid_v3_receipt());
    let bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::All),
        Arc::<FailingJournal>::clone(&journal),
    );

    let result = bus
        .publish_with_mode_and_before_dispatch_receipt_validator(
            test_event(TestText("invalid V3 no-subscriber receipt".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            DispatchMode::Sequential,
            require_verified_v3_receipt,
        )
        .await;

    assert_eq!(
        result,
        Err(EventingError::InvalidHandlerPolicy {
            reason: "test requires a verified V3 synchronization receipt".to_owned(),
        })
    );
    assert_eq!(journal.phases(), vec![JournalDispatchPhase::BeforeDispatch]);
    assert_eq!(journal.calls(), 1);
}

#[tokio::test]
async fn validated_publish_rejects_when_selector_omits_the_before_dispatch_receipt() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![crate::EventType::parse(
            OTHER_EVENT_TYPE,
        )
        .expect_value("other event type parses")])),
        Arc::<FailingJournal>::clone(&journal),
    );
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_by_subscription = Arc::clone(&handled);
    let _subscription = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                let handled = Arc::clone(&handled_by_subscription);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await
        .expect_value("subscriber registers");

    let result = bus
        .publish_with_mode_and_before_dispatch_receipt_validator(
            test_event(TestText("missing selected receipt".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            DispatchMode::Sequential,
            require_verified_v3_receipt,
        )
        .await;

    assert_eq!(
        result,
        Err(EventingError::InvalidHandlerPolicy {
            reason: "before-dispatch receipt validation requires a before-dispatch journal append"
                .to_owned(),
        })
    );
    assert!(handled.lock().await.is_empty());
    assert_eq!(journal.calls(), 0);
}

#[tokio::test]
async fn no_subscriber_after_dispatch_journal_does_not_record_an_action_phase() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let bus = EventBus::with_journal(
        JournalPolicy::after_dispatch(JournalSelector::All),
        Arc::<FailingJournal>::clone(&journal),
    );

    let report = bus
        .publish(
            test_event(TestText("after-only no-subscriber".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("no-subscriber dispatch completes without action-replay evidence");

    assert_eq!(report.subscriber_count, 0);
    assert!(report.journal_appends.is_empty());
    assert!(journal.phases().is_empty());
}

#[tokio::test]
async fn no_subscriber_before_and_after_journal_completes_idempotency_without_action_phase() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        Arc::<FailingJournal>::clone(&journal),
        policy,
    );

    bus.publish(
        test_event_with_idempotency(
            TestText("both-phase no-subscriber".to_owned()),
            TestText("both-phase-no-subscriber-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("both-phase-no-subscriber-event-1".to_owned()),
        ),
    )
    .await
    .expect_value("before-dispatch evidence persists before idempotency completion");

    let duplicate = bus
        .publish(
            test_event_with_idempotency(
                TestText("both-phase duplicate".to_owned()),
                TestText("both-phase-no-subscriber-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("both-phase-no-subscriber-event-2".to_owned()),
            ),
        )
        .await;

    assert!(matches!(
        duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));
    assert_eq!(journal.phases(), vec![JournalDispatchPhase::BeforeDispatch]);
}

#[tokio::test]
async fn no_subscriber_before_phase_failure_releases_idempotency_for_retry() {
    let journal = Arc::new(FailingJournal::fail_once_on(1));
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        Arc::<FailingJournal>::clone(&journal),
        policy,
    );

    let first = bus
        .publish(
            test_event_with_idempotency(
                TestText("retry before-phase no-subscriber".to_owned()),
                TestText("retry-before-phase-no-subscriber-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("retry-before-phase-no-subscriber-event-1".to_owned()),
            ),
        )
        .await;
    assert!(matches!(first, Err(EventingError::JournalIo { .. })));

    let replay = bus
        .publish(
            test_event_with_idempotency(
                TestText("retry before-phase no-subscriber".to_owned()),
                TestText("retry-before-phase-no-subscriber-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("retry-before-phase-no-subscriber-event-2".to_owned()),
            ),
        )
        .await
        .expect_value("retry persists the missing before-dispatch evidence");

    assert_eq!(replay.journal_appends.len(), 1);
    assert_eq!(
        journal.phases(),
        vec![
            JournalDispatchPhase::BeforeDispatch,
            JournalDispatchPhase::BeforeDispatch,
        ]
    );
}

#[tokio::test]
async fn no_subscriber_before_dispatch_reserves_idempotency_without_duplicate_journal_records() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let event_journal: Arc<dyn EventJournal> = Arc::<FailingJournal>::clone(&journal);
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_dispatch(JournalSelector::All),
        event_journal,
        policy,
    );

    bus.publish(
        test_event_with_idempotency(
            TestText("first no-subscriber dispatch".to_owned()),
            TestText("no-subscriber-idempotency-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("no-subscriber-idempotency-event-1".to_owned()),
        ),
    )
    .await
    .expect_value("first no-subscriber dispatch journals once");

    let duplicate = bus
        .publish(
            test_event_with_idempotency(
                TestText("duplicate no-subscriber dispatch".to_owned()),
                TestText("no-subscriber-idempotency-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("no-subscriber-idempotency-event-2".to_owned()),
            ),
        )
        .await;

    assert!(matches!(
        duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));
    assert_eq!(journal.calls(), 1);

    bus.publish(
        test_event_with_idempotency(
            TestText("later no-subscriber dispatch".to_owned()),
            TestText("no-subscriber-idempotency-key-later".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("no-subscriber-idempotency-event-3".to_owned()),
        ),
    )
    .await
    .expect_value("later no-subscriber dispatch continues after duplicate rejection");

    assert_eq!(journal.calls(), 2);
}

#[tokio::test]
async fn subscriber_auto_drain_only_drains_matching_event_type() {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(4).expect_value("queue policy is valid"),
    );
    bus.publish(
        test_event_with_idempotency(
            TestText("primary queued".to_owned()),
            TestText("queue-scope-primary-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("queue-scope-primary-event".to_owned()),
        ),
    )
    .await
    .expect_value("primary event queues");
    bus.publish(
        test_event_for_type_with_aggregate_and_idempotency(
            TestText("other queued".to_owned()),
            TestText("queue-scope-other-aggregate".to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
            TestText("queue-scope-other-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(OTHER_TARGET.to_owned()),
            TestText("queue-scope-other-event".to_owned()),
        ),
    )
    .await
    .expect_value("other event queues");

    let handled_other = Arc::new(Mutex::new(Vec::new()));
    let handled_other_clone = Arc::clone(&handled_other);
    let other_subscription = bus
        .subscribe::<TestEvent, _, _>(
            subscriber_for_event(
                TestText(OTHER_SUBSCRIBER.to_owned()),
                TestText(OTHER_TARGET.to_owned()),
                TestText(OTHER_EVENT_TYPE.to_owned()),
            ),
            move |context| {
                let handled = Arc::clone(&handled_other_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await
        .expect_value("other subscriber drains only matching queued event");
    let metrics_after_other = bus.metrics_snapshot().await;

    assert_eq!(other_subscription.drain_report.queued_before, 1);
    assert_eq!(other_subscription.drain_report.dispatched_count, 1);
    assert_eq!(other_subscription.drain_report.remaining_count, 0);
    assert_eq!(
        handled_other.lock().await.as_slice(),
        &["other queued".to_string()]
    );
    assert_eq!(metrics_after_other.queue.queued_event_count, 1);

    let handled_primary = Arc::new(Mutex::new(Vec::new()));
    let handled_primary_clone = Arc::clone(&handled_primary);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&handled_primary_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("primary subscriber drains remaining primary queued event");

    assert_eq!(
        handled_primary.lock().await.as_slice(),
        &["primary queued".to_string()]
    );
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 0);
}

#[tokio::test]
async fn bounded_queue_overflow_dead_letters_oldest_event_and_keeps_newest() {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(1).expect_value("queue policy is valid"),
    );
    bus.publish(
        test_event_with_idempotency(
            TestText("first".to_owned()),
            TestText("queue-overflow-first".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("queue-overflow-event-1".to_owned()),
        ),
    )
    .await
    .expect_value("first event queues");
    let report = bus
        .publish(
            test_event_with_idempotency(
                TestText("second".to_owned()),
                TestText("queue-overflow-second".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("queue-overflow-event-2".to_owned()),
            ),
        )
        .await
        .expect_value("overflow drops oldest and queues newest");
    let dead_letters = bus.dead_letters().await;
    let dead_letter_event = dead_letters[0].as_event();
    let expected_dead_letter_type =
        crate::dead_letter_recorded_event_type().expect_value("dead-letter event type parses");

    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::DeadLetteredQueueOverflow
    );
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(report.queue_report.queued_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueOverflow);
    assert_eq!(
        dead_letters[0].envelope.event_id.as_str(),
        "queue-overflow-event-1"
    );
    assert!(dead_letters[0].subscriber_id.is_none());
    assert!(dead_letters[0].target_handler.is_none());
    assert_eq!(dead_letter_event.reason, DeadLetterReason::QueueOverflow);
    assert_eq!(
        dead_letter_event
            .contract()
            .expect_value("dead-letter event contract exists")
            .event_type,
        expected_dead_letter_type
    );

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber drains newest queued event");
    assert_eq!(handled.lock().await.as_slice(), &["second".to_string()]);
}

#[tokio::test]
async fn queued_event_expires_before_dispatch_when_ttl_elapsed() {
    let policy = EventQueuePolicy::no_subscriber_queue(2)
        .expect_value("queue policy is valid")
        .with_ttl(Duration::from_millis(5))
        .expect_value("ttl policy is valid");
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(
        test_event(TestText(TEST_LABEL.to_owned())),
        metadata(TestText(TEST_TARGET.to_owned())),
    )
    .await
    .expect_value("event queues");
    tokio::time::sleep(Duration::from_millis(20)).await;
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async { Ok(()) },
    )
    .await
    .expect_value("subscriber registration drains expired queue");
    let drain = bus
        .drain_queued(DispatchMode::Sequential)
        .await
        .expect_value("queue stays empty");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(drain.queued_before, 0);
    assert_eq!(drain.dispatched_count, 0);
    assert_eq!(drain.remaining_count, 0);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueExpired);
}

#[tokio::test]
async fn idempotency_registry_rejects_queued_and_completed_duplicates() {
    let policy = EventQueuePolicy::no_subscriber_queue(2)
        .expect_value("queue policy is valid")
        .with_idempotency_registry();
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(
        test_event_with_idempotency(
            TestText(TEST_LABEL.to_owned()),
            TestText("idempotency-queue-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("idempotency-queued-event-1".to_owned()),
        ),
    )
    .await
    .expect_value("first event queues");

    let queued_duplicate = bus
        .publish(
            test_event_with_idempotency(
                TestText("duplicate".to_owned()),
                TestText("idempotency-queue-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("idempotency-queued-event-2".to_owned()),
            ),
        )
        .await;
    assert!(matches!(
        queued_duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));

    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async { Ok(()) },
    )
    .await
    .expect_value("subscriber drains queued event");

    let completed_duplicate = bus
        .publish(
            test_event_with_idempotency(
                TestText("completed".to_owned()),
                TestText("idempotency-queue-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("idempotency-completed-event".to_owned()),
            ),
        )
        .await;
    assert!(matches!(
        completed_duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));
}

#[tokio::test]
async fn in_flight_duplicate_guard_rejects_concurrent_event_id() {
    let bus = EventBus::root();
    let started = Arc::new(Notify::new());
    let started_clone = Arc::clone(&started);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let started = Arc::clone(&started_clone);
            async move {
                started.notify_waiters();
                tokio::time::sleep(Duration::from_millis(50)).await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");

    let first_bus = bus.clone();
    let first = tokio::spawn(async move {
        first_bus
            .publish(
                test_event_with_idempotency(
                    TestText(TEST_LABEL.to_owned()),
                    TestText("in-flight-idempotency-key-1".to_owned()),
                ),
                metadata(TestText(TEST_TARGET.to_owned())),
            )
            .await
    });
    started.notified().await;
    let duplicate = bus
        .publish(
            test_event_with_idempotency(
                TestText("duplicate".to_owned()),
                TestText("in-flight-idempotency-key-2".to_owned()),
            ),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await;
    let first_report = first
        .await
        .expect_value("first task joins")
        .expect_value("first publish completes");

    assert!(matches!(
        duplicate,
        Err(EventingError::DuplicateEventId { .. })
    ));
    assert_eq!(first_report.handled_count, 1);
}

#[tokio::test]
async fn failed_initial_queued_publish_rolls_back_for_same_event_retry() {
    let policy = EventQueuePolicy::no_subscriber_queue(2)
        .expect_value("queue policy is valid")
        .with_idempotency_registry();
    let journal = Arc::new(FailingJournal::fail_once_on(1));
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal,
        policy,
    );
    let event_id = TestText("initial-queued-journal-failure-event".to_owned());
    let key = TestText("initial-queued-journal-failure-key".to_owned());
    let first = bus
        .publish(
            test_event_with_idempotency(TestText(TEST_LABEL.to_owned()), key.clone()),
            metadata_with_event_id(TestText(TEST_TARGET.to_owned()), event_id.clone()),
        )
        .await;
    assert!(matches!(first, Err(EventingError::JournalIo { .. })));
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 0);

    bus.publish(
        test_event_with_idempotency(TestText(TEST_LABEL.to_owned()), key),
        metadata_with_event_id(TestText(TEST_TARGET.to_owned()), event_id),
    )
    .await
    .expect_value("same event retries after queue rollback");
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 1);
}

#[tokio::test]
async fn failed_overflow_journal_append_restores_dropped_event_without_dead_letter() {
    let policy = EventQueuePolicy::no_subscriber_queue(1)
        .expect_value("queue policy is valid")
        .with_idempotency_registry();
    let journal = Arc::new(FailingJournal::fail_once_on(2));
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal,
        policy,
    );
    bus.publish(
        test_event_with_idempotency(
            TestText("first preserved".to_owned()),
            TestText("overflow-rollback-first-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("overflow-rollback-first-event".to_owned()),
        ),
    )
    .await
    .expect_value("first event queues");

    let failed = bus
        .publish(
            test_event_with_idempotency(
                TestText("second rejected".to_owned()),
                TestText("overflow-rollback-second-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("overflow-rollback-second-event".to_owned()),
            ),
        )
        .await;
    assert!(matches!(failed, Err(EventingError::JournalIo { .. })));
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 1);
    assert!(bus.dead_letters().await.is_empty());

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber drains restored oldest event");
    assert_eq!(
        handled.lock().await.as_slice(),
        &["first preserved".to_owned()]
    );
}

#[tokio::test]
async fn failed_subscribe_drain_preserves_queued_event_for_retry() {
    let policy = EventQueuePolicy::no_subscriber_queue(2).expect_value("queue policy is valid");
    // The queued publish records its before-dispatch phase first; fail the
    // drain's before phase so the event is requeued for the retry subscriber.
    let journal = Arc::new(FailingJournal::fail_once_on(2));
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal,
        policy,
    );
    bus.publish(
        test_event_with_idempotency(
            TestText(TEST_LABEL.to_owned()),
            TestText("drain-preserve-idempotency".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("drain-preserve-event-1".to_owned()),
        ),
    )
    .await
    .expect_value("event queues");

    let failed_subscribe = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            |_| async { Ok(()) },
        )
        .await;
    assert!(matches!(
        failed_subscribe,
        Err(EventingError::JournalIo { .. })
    ));

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("retry subscriber drains preserved event");

    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
}

#[tokio::test]
async fn after_dispatch_journal_failure_does_not_replay_handler_work() {
    let policy = EventQueuePolicy::no_subscriber_queue(2).expect_value("queue policy is valid");
    // The queued publish records its before-dispatch phase first; the drain's
    // before phase is call two, so call three is the after-dispatch failure.
    let journal = Arc::new(FailingJournal::fail_once_on(3));
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal,
        policy,
    );
    bus.publish(
        test_event_with_idempotency(
            TestText(TEST_LABEL.to_owned()),
            TestText("drain-after-dispatch-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("drain-after-dispatch-event".to_owned()),
        ),
    )
    .await
    .expect_value("event queues");

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let failed_subscribe = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await;
    assert!(matches!(
        failed_subscribe,
        Err(EventingError::JournalIo { .. })
    ));

    let retry_handled = Arc::new(Mutex::new(Vec::new()));
    let retry_handled_clone = Arc::clone(&retry_handled);
    let retry_subscription = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                let handled = Arc::clone(&retry_handled_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await
        .expect_value("retry subscriber registers without replaying completed work");

    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert!(retry_handled.lock().await.is_empty());
    assert_eq!(retry_subscription.drain_report.queued_before, 0);
    assert_eq!(retry_subscription.drain_report.dispatched_count, 0);
}

#[tokio::test]
async fn no_subscriber_dead_letter_policy_records_typed_metadata() {
    let policy = EventQueuePolicy::default()
        .with_no_subscriber_policy(NoSubscriberQueuePolicy::DeadLetter)
        .expect_value("dead-letter policy is valid");
    let bus = EventBus::with_queue_policy(policy);
    let metadata = metadata(TestText(TEST_TARGET.to_owned()));
    let expected_correlation_id = metadata.correlation_id.clone();

    let report = bus
        .publish(test_event(TestText(TEST_LABEL.to_owned())), metadata)
        .await
        .expect_value("no-subscriber event is dead-lettered");
    let dead_letters = bus.dead_letters().await;
    let dead_letter = &dead_letters[0];
    let dead_letter_event = dead_letter.as_event();
    let expected_dead_letter_type =
        crate::dead_letter_recorded_event_type().expect_value("dead-letter event type parses");

    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::DeadLetteredNoSubscriber
    );
    assert_eq!(report.queue_report.queued_count, 0);
    assert_eq!(report.queue_report.capacity, None);
    assert_eq!(report.subscriber_count, 0);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letter.reason, DeadLetterReason::NoSubscriber);
    assert_eq!(dead_letter.retry_state, DeadLetterRetryState::NotAttempted);
    assert_eq!(
        dead_letter.error,
        EventingError::NoSubscriber {
            event_type: report.event_type.clone(),
        }
    );
    assert_eq!(dead_letter_event.original_event_id, report.event_id);
    assert_eq!(dead_letter_event.original_event_type, report.event_type);
    assert_eq!(
        dead_letter_event.original_correlation_id,
        expected_correlation_id
    );
    assert_eq!(
        dead_letter_event.custody,
        dead_letter.envelope.source.custody
    );
    assert_eq!(dead_letter_event.source, dead_letter.envelope.source);
    assert_eq!(dead_letter_event.subscriber_id, None);
    assert_eq!(dead_letter_event.target_handler, None);
    assert_eq!(
        dead_letter_event
            .contract()
            .expect_value("dead-letter event contract exists")
            .event_type,
        expected_dead_letter_type
    );

    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.dead_letter_count, 1);
    assert_eq!(metrics.queue.queued_event_count, 0);
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
}

#[tokio::test]
async fn bounded_queue_reject_overflow_preserves_queued_event() {
    let policy = EventQueuePolicy::no_subscriber_queue(1)
        .expect_value("queue policy is valid")
        .with_overflow_policy(QueueOverflowPolicy::RejectPublish);
    let bus = EventBus::with_queue_policy(policy);
    let first = bus
        .publish(
            test_event_with_idempotency(
                TestText("reject-overflow-first".to_owned()),
                TestText("reject-overflow-first-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("reject-overflow-first-event".to_owned()),
            ),
        )
        .await
        .expect_value("first event queues");
    let overflow = bus
        .publish(
            test_event_with_idempotency(
                TestText("reject-overflow-second".to_owned()),
                TestText("reject-overflow-second-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("reject-overflow-second-event".to_owned()),
            ),
        )
        .await;

    assert_eq!(
        first.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(first.queue_report.queued_count, 1);
    match overflow {
        Err(EventingError::QueueCapacityExceeded {
            event_type,
            capacity,
        }) => {
            assert_eq!(event_type.as_str(), super::fixtures::TEST_EVENT_TYPE);
            assert_eq!(capacity, 1);
        }
        other => panic!("unexpected overflow result: {other:?}"),
    }
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.queued_event_count, 1);
    assert_eq!(metrics.dead_letter_count, 0);

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let subscription = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await
        .expect_value("subscriber drains preserved event");

    assert_eq!(subscription.drain_report.queued_before, 1);
    assert_eq!(subscription.drain_report.dispatched_count, 1);
    assert_eq!(subscription.drain_report.remaining_count, 0);
    assert_eq!(
        handled.lock().await.as_slice(),
        &["reject-overflow-first".to_owned()]
    );
}

#[tokio::test]
async fn bounded_queue_dead_letter_rejected_overflow_records_incoming_event() {
    let policy = EventQueuePolicy::no_subscriber_queue(1)
        .expect_value("queue policy is valid")
        .with_overflow_policy(QueueOverflowPolicy::DeadLetterRejected);
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(
        test_event_with_idempotency(
            TestText("dead-letter-overflow-first".to_owned()),
            TestText("dead-letter-overflow-first-key".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("dead-letter-overflow-first-event".to_owned()),
        ),
    )
    .await
    .expect_value("first event queues");

    let report = bus
        .publish(
            test_event_with_idempotency(
                TestText("dead-letter-overflow-second".to_owned()),
                TestText("dead-letter-overflow-second-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("dead-letter-overflow-second-event".to_owned()),
            ),
        )
        .await
        .expect_value("full queue dead-letters incoming event");
    let dead_letters = bus.dead_letters().await;
    let dead_letter = &dead_letters[0];
    let dead_letter_event = dead_letter.as_event();

    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::DeadLetteredQueueOverflow
    );
    assert_eq!(report.queue_report.queued_count, 1);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(
        dead_letter.envelope.event_id.as_str(),
        "dead-letter-overflow-second-event"
    );
    assert_eq!(dead_letter.reason, DeadLetterReason::QueueOverflow);
    assert_eq!(dead_letter.retry_state, DeadLetterRetryState::NotAttempted);
    match &dead_letter.error {
        EventingError::QueueCapacityExceeded {
            event_type,
            capacity,
        } => {
            assert_eq!(event_type.as_str(), super::fixtures::TEST_EVENT_TYPE);
            assert_eq!(*capacity, 1);
        }
        other => panic!("unexpected dead-letter error: {other:?}"),
    }
    assert_eq!(
        dead_letter_event.original_event_id.as_str(),
        "dead-letter-overflow-second-event"
    );
    assert_eq!(dead_letter_event.reason, DeadLetterReason::QueueOverflow);
    assert_eq!(
        dead_letter_event.retry_state,
        DeadLetterRetryState::NotAttempted
    );
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 1);

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&handled_clone);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber drains retained event");
    assert_eq!(
        handled.lock().await.as_slice(),
        &["dead-letter-overflow-first".to_owned()]
    );
}

#[tokio::test]
async fn manual_clock_queue_ttl_boundary_dead_letters_without_wall_sleep() {
    let clock = ManualEventClock::new();
    let policy = EventQueuePolicy::no_subscriber_queue(1)
        .expect_value("queue policy is valid")
        .with_ttl(Duration::from_millis(10))
        .expect_value("ttl policy is valid");
    let bus = EventBus::with_queue_policy_and_clock(policy, clock.shared());
    bus.publish(
        test_event(TestText("manual-ttl-boundary".to_owned())),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("manual-ttl-boundary-event".to_owned()),
        ),
    )
    .await
    .expect_value("event queues");
    clock.advance(Duration::from_millis(10));

    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_clone = Arc::clone(&attempts);
    let subscription = bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |_| {
                let attempts = Arc::clone(&attempts_clone);
                async move {
                    attempts.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            },
        )
        .await
        .expect_value("subscriber registration drains expired queue");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(subscription.drain_report.queued_before, 1);
    assert_eq!(subscription.drain_report.dispatched_count, 0);
    assert_eq!(subscription.drain_report.expired_count, 1);
    assert_eq!(subscription.drain_report.remaining_count, 0);
    assert_eq!(attempts.load(Ordering::SeqCst), 0);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueExpired);
    assert_eq!(
        dead_letters[0].retry_state,
        DeadLetterRetryState::NotAttempted
    );
    assert_eq!(bus.metrics_snapshot().await.queue.queued_event_count, 0);
}

#[tokio::test]
async fn in_flight_idempotency_guard_rejects_same_key_across_event_ids() {
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let bus = EventBus::with_queue_policy(policy);
    let started = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let attempts = Arc::new(AtomicUsize::new(0));
    let started_clone = Arc::clone(&started);
    let release_clone = Arc::clone(&release);
    let attempts_clone = Arc::clone(&attempts);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let started = Arc::clone(&started_clone);
            let release = Arc::clone(&release_clone);
            let attempts = Arc::clone(&attempts_clone);
            async move {
                attempts.fetch_add(1, Ordering::SeqCst);
                started.notify_one();
                release.notified().await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");

    let first_bus = bus.clone();
    let first = tokio::spawn(async move {
        first_bus
            .publish(
                test_event_with_idempotency(
                    TestText("in-flight-key-first".to_owned()),
                    TestText("in-flight-shared-key".to_owned()),
                ),
                metadata_with_event_id(
                    TestText(TEST_TARGET.to_owned()),
                    TestText("in-flight-key-event-1".to_owned()),
                ),
            )
            .await
    });
    started.notified().await;

    let duplicate = bus
        .publish(
            test_event_with_idempotency(
                TestText("in-flight-key-second".to_owned()),
                TestText("in-flight-shared-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("in-flight-key-event-2".to_owned()),
            ),
        )
        .await;
    match duplicate {
        Err(EventingError::DuplicateInFlight { idempotency_key }) => {
            assert_eq!(idempotency_key.as_str(), "in-flight-shared-key");
        }
        other => panic!("unexpected duplicate result: {other:?}"),
    }

    release.notify_one();
    let first_report = first
        .await
        .expect_value("first task joins")
        .expect_value("first publish completes");
    assert_eq!(first_report.handled_count, 1);
    assert_eq!(first_report.dead_letter_count, 0);
    assert_eq!(attempts.load(Ordering::SeqCst), 1);
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.queue.completed_idempotency_key_count, 1);
}

#[tokio::test]
async fn retry_storm_guard_dead_letters_after_exact_max_attempts() {
    let bus = EventBus::with_handler_policy(
        HandlerExecutionPolicy::new(None, 3).expect_value("handler policy is valid"),
    );
    let attempts = Arc::new(AtomicUsize::new(0));
    let attempts_clone = Arc::clone(&attempts);
    let handler_error = EventingError::InvalidValue {
        field: "retry-storm",
        value: "handler failed".to_owned(),
    };
    let handler_error_clone = handler_error.clone();
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let attempts = Arc::clone(&attempts_clone);
            let error = handler_error_clone.clone();
            async move {
                attempts.fetch_add(1, Ordering::SeqCst);
                Err(error)
            }
        },
    )
    .await
    .expect_value("failing subscriber registers");

    let report = bus
        .publish(
            test_event(TestText("retry-storm-event".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("retry-storm-event-id".to_owned()),
            ),
        )
        .await
        .expect_value("retry exhaustion reports");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(attempts.load(Ordering::SeqCst), 3);
    assert_eq!(report.handler_reports.len(), 1);
    assert_eq!(report.handler_reports[0].attempts, 3);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::HandlerFailed);
    assert_eq!(
        dead_letters[0].retry_state,
        DeadLetterRetryState::Exhausted { attempts: 3 }
    );
    assert_eq!(dead_letters[0].error, handler_error);
}

struct FailingJournal {
    calls: StdMutex<usize>,
    phases: StdMutex<Vec<JournalDispatchPhase>>,
    fail_once_on: usize,
    hash_version: JournalHashVersion,
}

impl FailingJournal {
    fn fail_once_on(call: usize) -> Self {
        Self {
            calls: StdMutex::new(0),
            phases: StdMutex::new(Vec::new()),
            fail_once_on: call,
            hash_version: JournalHashVersion::V2,
        }
    }

    fn with_invalid_v3_receipt() -> Self {
        Self {
            calls: StdMutex::new(0),
            phases: StdMutex::new(Vec::new()),
            fail_once_on: usize::MAX,
            hash_version: JournalHashVersion::V3,
        }
    }

    fn calls(&self) -> usize {
        *self.calls.lock().expect_value("failing journal lock")
    }

    fn phases(&self) -> Vec<JournalDispatchPhase> {
        self.phases
            .lock()
            .expect_value("failing journal phase lock")
            .clone()
    }
}

impl EventJournal for FailingJournal {
    fn append<'a>(
        &'a self,
        _envelope: &'a StoredEventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        Box::pin(async move {
            let call = {
                let mut calls = self.calls.lock().expect_value("failing journal lock");
                *calls += 1;
                *calls
            };
            failing_journal_result(call, self.fail_once_on, self.hash_version)
        })
    }

    fn append_phase<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        self.phases
            .lock()
            .expect_value("failing journal phase lock")
            .push(phase);
        self.append(envelope)
    }
}
