use crate::ExpectValue;
use std::{sync::Arc, time::Duration};

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
    JournalAppend, JournalPolicy, JournalSelector, QueueDisposition, StoredEventEnvelope,
};
use ocentra_eventing::bus::reports::dead_letter::DeadLetterReason;
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_eventing::journal::{JournalAppendDurability, JournalHashVersion};

fn failing_journal_result(
    call: usize,
    fail_once_on: usize,
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
        hash_version: JournalHashVersion::V2,
        durability: JournalAppendDurability::Synchronized,
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
async fn before_dispatch_journal_is_durable_without_a_subscriber() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::All),
        journal.clone(),
    );

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("before-dispatch journal persists without a subscriber");

    assert_eq!(report.subscriber_count, 0);
    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::Dispatched
    );
    assert_eq!(report.journal_appends.len(), 1);
    assert!(report.journal_appends[0].is_synchronized());
    assert_eq!(journal.phases(), vec![JournalDispatchPhase::BeforeDispatch]);
}

#[tokio::test]
async fn no_subscriber_after_dispatch_journal_records_the_after_phase() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let bus = EventBus::with_journal(
        JournalPolicy::after_dispatch(JournalSelector::All),
        journal.clone(),
    );

    let report = bus
        .publish(
            test_event(TestText("after-only no-subscriber".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("after-dispatch journal persists without a subscriber");

    assert_eq!(report.subscriber_count, 0);
    assert_eq!(report.journal_appends.len(), 1);
    assert_eq!(journal.phases(), vec![JournalDispatchPhase::AfterDispatch]);
}

#[tokio::test]
async fn no_subscriber_before_and_after_journal_completes_idempotency_after_both_phases() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal.clone(),
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
    .expect_value("both phases persist before idempotency completion");

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
    assert_eq!(
        journal.phases(),
        vec![
            JournalDispatchPhase::BeforeDispatch,
            JournalDispatchPhase::AfterDispatch,
        ]
    );
}

#[tokio::test]
async fn no_subscriber_after_phase_failure_releases_idempotency_for_full_phase_retry() {
    let journal = Arc::new(FailingJournal::fail_once_on(2));
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let bus = EventBus::with_journal_and_queue_policy(
        JournalPolicy::before_and_after_dispatch(JournalSelector::All),
        journal.clone(),
        policy,
    );

    let first = bus
        .publish(
            test_event_with_idempotency(
                TestText("retry both-phase no-subscriber".to_owned()),
                TestText("retry-both-phase-no-subscriber-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("retry-both-phase-no-subscriber-event-1".to_owned()),
            ),
        )
        .await;
    assert!(matches!(first, Err(EventingError::JournalIo { .. })));

    let replay = bus
        .publish(
            test_event_with_idempotency(
                TestText("retry both-phase no-subscriber".to_owned()),
                TestText("retry-both-phase-no-subscriber-key".to_owned()),
            ),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("retry-both-phase-no-subscriber-event-2".to_owned()),
            ),
        )
        .await
        .expect_value("retry persists the previously missing after phase");

    assert_eq!(replay.journal_appends.len(), 2);
    assert_eq!(
        journal.phases(),
        vec![
            JournalDispatchPhase::BeforeDispatch,
            JournalDispatchPhase::AfterDispatch,
            JournalDispatchPhase::BeforeDispatch,
            JournalDispatchPhase::AfterDispatch,
        ]
    );
}

#[tokio::test]
async fn no_subscriber_before_dispatch_reserves_idempotency_without_duplicate_journal_records() {
    let journal = Arc::new(FailingJournal::fail_once_on(usize::MAX));
    let policy = EventQueuePolicy::default().with_idempotency_registry();
    let event_journal: Arc<dyn EventJournal> = journal.clone();
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
    let bus = EventBus::new();
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
async fn failed_subscribe_drain_preserves_queued_event_for_retry() {
    let policy = EventQueuePolicy::no_subscriber_queue(2).expect_value("queue policy is valid");
    let journal = Arc::new(FailingJournal::fail_once_on(1));
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
    let journal = Arc::new(FailingJournal::fail_once_on(2));
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

struct FailingJournal {
    calls: StdMutex<usize>,
    phases: StdMutex<Vec<JournalDispatchPhase>>,
    fail_once_on: usize,
}

impl FailingJournal {
    fn fail_once_on(call: usize) -> Self {
        Self {
            calls: StdMutex::new(0),
            phases: StdMutex::new(Vec::new()),
            fail_once_on: call,
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
            failing_journal_result(call, self.fail_once_on)
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
