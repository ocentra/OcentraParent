use super::*;

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
        crate::fixtures::TEST_EVENT_TYPE
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
