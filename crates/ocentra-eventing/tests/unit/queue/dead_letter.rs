use super::*;

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
    assert!(matches!(
        &overflow,
        Err(EventingError::QueueCapacityExceeded { .. })
    ));
    if let Err(EventingError::QueueCapacityExceeded {
        event_type,
        capacity,
    }) = overflow
    {
        assert_eq!(event_type.as_str(), crate::fixtures::TEST_EVENT_TYPE);
        assert_eq!(capacity, 1);
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
    assert!(matches!(
        &dead_letter.error,
        EventingError::QueueCapacityExceeded { .. }
    ));
    if let EventingError::QueueCapacityExceeded {
        event_type,
        capacity,
    } = &dead_letter.error
    {
        assert_eq!(event_type.as_str(), crate::fixtures::TEST_EVENT_TYPE);
        assert_eq!(*capacity, 1);
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
    assert!(matches!(
        &duplicate,
        Err(EventingError::DuplicateInFlight { .. })
    ));
    if let Err(EventingError::DuplicateInFlight { idempotency_key }) = duplicate {
        assert_eq!(idempotency_key.as_str(), "in-flight-shared-key");
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
