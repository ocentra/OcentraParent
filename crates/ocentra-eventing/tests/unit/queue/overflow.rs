use super::*;

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
