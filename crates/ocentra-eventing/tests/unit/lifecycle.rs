use crate::ExpectValue;
use std::{sync::Arc, sync::Mutex as StdMutex, time::Duration};

use tokio::sync::{Barrier, Mutex, Notify};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type, test_event_with_aggregate, TestEvent, TestText, OTHER_EVENT_TYPE,
    OTHER_SUBSCRIBER, OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::{DispatchMode, EventBus, EventRegistrar, EventingError};
use ocentra_eventing::bus::reports::{
    dead_letter::{DeadLetterReason, DeadLetterRetryState},
    handler::{EventConsumerOutcome, HandlerOutcome},
};
use ocentra_eventing::queue::policy::EventQueuePolicy;

#[tokio::test]
async fn ordered_dispatch_serializes_same_aggregate_transitions() {
    let bus = EventBus::root();
    let observed = Arc::new(Mutex::new(Vec::new()));
    let observed_clone = Arc::clone(&observed);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| {
            let observed = Arc::clone(&observed_clone);
            async move {
                observed
                    .lock()
                    .await
                    .push(format!("{}:start", context.payload().label));
                tokio::time::sleep(Duration::from_millis(10)).await;
                observed
                    .lock()
                    .await
                    .push(format!("{}:end", context.payload().label));
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");

    let first = bus.publish_with_mode(
        test_event(TestText("first".to_owned())),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("ordered-same-aggregate-event-1".to_owned()),
        ),
        DispatchMode::OrderedByAggregateKey,
    );
    let second = bus.publish_with_mode(
        test_event(TestText("second".to_owned())),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("ordered-same-aggregate-event-2".to_owned()),
        ),
        DispatchMode::OrderedByAggregateKey,
    );
    let (first_report, second_report) = tokio::join!(first, second);

    assert_eq!(
        first_report
            .expect_value("first publish succeeds")
            .handled_count,
        1
    );
    assert_eq!(
        second_report
            .expect_value("second publish succeeds")
            .handled_count,
        1
    );
    assert_eq!(
        observed.lock().await.as_slice(),
        &[
            "first:start".to_string(),
            "first:end".to_string(),
            "second:start".to_string(),
            "second:end".to_string()
        ]
    );
    assert_eq!(bus.clear_for_test().await.aggregate_gate_count, 0);
}

#[tokio::test]
async fn ordered_dispatch_allows_different_aggregates_to_run_concurrently() {
    let bus = EventBus::root();
    let barrier = Arc::new(Barrier::new(2));
    let barrier_clone = Arc::clone(&barrier);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let barrier = Arc::clone(&barrier_clone);
            async move {
                barrier.wait().await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("subscriber registers");

    let first = bus.publish_with_mode(
        test_event_with_aggregate(
            TestText("first".to_owned()),
            TestText("aggregate-a".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("ordered-different-aggregate-event-1".to_owned()),
        ),
        DispatchMode::OrderedByAggregateKey,
    );
    let second = bus.publish_with_mode(
        test_event_with_aggregate(
            TestText("second".to_owned()),
            TestText("aggregate-b".to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("ordered-different-aggregate-event-2".to_owned()),
        ),
        DispatchMode::OrderedByAggregateKey,
    );
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        Box::pin(async { tokio::join!(first, second) }),
    )
    .await
    .expect_value("different aggregate publishes complete without serial deadlock");

    assert_eq!(
        result
            .0
            .expect_value("first publish succeeds")
            .handled_count,
        1
    );
    assert_eq!(
        result
            .1
            .expect_value("second publish succeeds")
            .handled_count,
        1
    );
    assert_eq!(bus.clear_for_test().await.aggregate_gate_count, 0);
}

#[tokio::test]
async fn sequential_dispatch_runs_subscribers_in_registration_order() {
    let bus = EventBus::root();
    let observed = Arc::new(Mutex::new(Vec::new()));
    let first_observed = Arc::clone(&observed);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText("eventing-sequential-first".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let observed = Arc::clone(&first_observed);
            async move {
                observed.lock().await.push("first".to_owned());
                Ok(())
            }
        },
    )
    .await
    .expect_value("first sequential subscriber registers");
    let second_observed = Arc::clone(&observed);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText("eventing-sequential-second".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let observed = Arc::clone(&second_observed);
            async move {
                observed.lock().await.push("second".to_owned());
                Ok(())
            }
        },
    )
    .await
    .expect_value("second sequential subscriber registers");

    let report = bus
        .publish_with_mode(
            test_event(TestText("sequential-dispatch".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("sequential-dispatch-event-1".to_owned()),
            ),
            DispatchMode::Sequential,
        )
        .await
        .expect_value("sequential publish succeeds");

    assert_eq!(report.dispatch_mode, DispatchMode::Sequential);
    assert_eq!(report.subscriber_count, 2);
    assert_eq!(report.handled_count, 2);
    assert_eq!(
        observed.lock().await.as_slice(),
        &["first".to_owned(), "second".to_owned()]
    );
}

#[tokio::test]
async fn concurrent_dispatch_runs_handlers_in_parallel_and_reports_each_result() {
    let bus = EventBus::root();
    let barrier = Arc::new(Barrier::new(2));
    let first_barrier = Arc::clone(&barrier);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText("eventing-concurrent-first".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let barrier = Arc::clone(&first_barrier);
            async move {
                barrier.wait().await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("first concurrent subscriber registers");
    let second_barrier = Arc::clone(&barrier);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText("eventing-concurrent-second".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let barrier = Arc::clone(&second_barrier);
            async move {
                barrier.wait().await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("second concurrent subscriber registers");

    let report = tokio::time::timeout(
        Duration::from_secs(1),
        bus.publish_with_mode(
            test_event(TestText("concurrent-dispatch".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("concurrent-dispatch-event-1".to_owned()),
            ),
            DispatchMode::Concurrent,
        ),
    )
    .await
    .expect_value("concurrent handlers complete together")
    .expect_value("concurrent publish succeeds");

    assert_eq!(report.dispatch_mode, DispatchMode::Concurrent);
    assert_eq!(report.subscriber_count, 2);
    assert_eq!(report.handled_count, 2);
    assert_eq!(report.handler_reports.len(), 2);
    assert!(report
        .handler_reports
        .iter()
        .all(|handler| handler.outcome == HandlerOutcome::Handled));
}

#[tokio::test]
async fn separate_root_publishers_do_not_share_subscriber_registry() {
    let first_bus = EventBus::root();
    let second_bus = EventBus::root();
    first_bus
        .subscribe::<TestEvent, _, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            |_| async { Ok(()) },
        )
        .await
        .expect_value("first root subscriber registers");

    let first_report = first_bus
        .publish(
            test_event(TestText("first-root".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("first-root-event-1".to_owned()),
            ),
        )
        .await
        .expect_value("first root publish succeeds");
    let second_report = second_bus
        .publish(
            test_event(TestText("second-root".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("second-root-event-1".to_owned()),
            ),
        )
        .await
        .expect_value("second root publish succeeds");

    assert_eq!(first_report.subscriber_count, 1);
    assert_eq!(first_report.handled_count, 1);
    assert_eq!(second_report.subscriber_count, 0);
    assert_eq!(
        second_report.consumer_outcome(),
        EventConsumerOutcome::Unregistered
    );
}

#[tokio::test]
async fn wrong_target_reports_unregistered_without_invoking_handler() {
    let bus = EventBus::root();
    let handled = Arc::new(Mutex::new(0_usize));
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
    .expect_value("targeted subscriber registers");

    let report = bus
        .publish(
            test_event(TestText("wrong-target".to_owned())),
            metadata_with_event_id(
                TestText(OTHER_TARGET.to_owned()),
                TestText("wrong-target-event-1".to_owned()),
            ),
        )
        .await
        .expect_value("wrong-target publish succeeds without delivery");

    assert_eq!(report.subscriber_count, 0);
    assert_eq!(report.handled_count, 0);
    assert!(report.handler_reports.is_empty());
    assert_eq!(
        report.consumer_outcome(),
        EventConsumerOutcome::Unregistered
    );
    assert_eq!(*handled.lock().await, 0);
}

#[tokio::test]
async fn subscription_drain_does_not_hold_registry_lock_across_handler_await() {
    let policy = EventQueuePolicy::no_subscriber_queue(1).expect_value("queue policy is valid");
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(
        test_event(TestText("queued-before-subscribe".to_owned())),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText("queued-before-subscribe-event-1".to_owned()),
        ),
    )
    .await
    .expect_value("event queues before subscriber registration");

    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let first_bus = bus.clone();
    let first_entered = Arc::clone(&entered);
    let first_release = Arc::clone(&release);
    let first_subscribe = tokio::spawn(async move {
        first_bus
            .subscribe::<TestEvent, _, _>(
                subscriber(
                    TestText("eventing-drain-first".to_owned()),
                    TestText(TEST_TARGET.to_owned()),
                ),
                move |_| {
                    let entered = Arc::clone(&first_entered);
                    let release = Arc::clone(&first_release);
                    async move {
                        entered.notify_one();
                        release.notified().await;
                        Ok(())
                    }
                },
            )
            .await
    });

    tokio::time::timeout(Duration::from_secs(1), entered.notified())
        .await
        .expect_value("queued handler starts");
    let second_result = tokio::time::timeout(
        Duration::from_secs(1),
        bus.subscribe::<TestEvent, _, _>(
            subscriber(
                TestText("eventing-drain-second".to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            |_| async { Ok(()) },
        ),
    )
    .await;
    release.notify_one();

    let first_report = first_subscribe
        .await
        .expect_value("first subscribe task joins")
        .expect_value("first subscriber drains queued event");
    let second_report = second_result
        .expect_value("second subscription is not blocked by handler await")
        .expect_value("second subscriber registers");

    assert_eq!(first_report.drain_report.dispatched_count, 1);
    assert_eq!(first_report.drain_report.remaining_count, 0);
    assert_eq!(second_report.drain_report.queued_before, 0);
}

#[tokio::test]
async fn publish_and_wait_completes_only_after_handler_work_finishes() {
    let bus = EventBus::root();
    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let handler_entered = Arc::clone(&entered);
    let handler_release = Arc::clone(&release);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText("eventing-publish-wait-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |_| {
            let entered = Arc::clone(&handler_entered);
            let release = Arc::clone(&handler_release);
            async move {
                entered.notify_one();
                release.notified().await;
                Ok(())
            }
        },
    )
    .await
    .expect_value("publish-and-wait subscriber registers");

    let publish_bus = bus.clone();
    let mut publish = tokio::spawn(async move {
        publish_bus
            .publish_and_wait(
                test_event(TestText("publish-and-wait".to_owned())),
                metadata_with_event_id(
                    TestText(TEST_TARGET.to_owned()),
                    TestText("publish-and-wait-event-1".to_owned()),
                ),
            )
            .await
    });
    tokio::time::timeout(Duration::from_secs(1), entered.notified())
        .await
        .expect_value("publish-and-wait handler starts");
    let completed_early = tokio::select! {
        _ = &mut publish => true,
        _ = tokio::task::yield_now() => false,
    };
    assert!(
        !completed_early,
        "publish-and-wait completed before handler release"
    );
    release.notify_one();

    let report = publish
        .await
        .expect_value("publish-and-wait task joins")
        .expect_value("publish-and-wait succeeds");
    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
}

#[tokio::test]
async fn nested_publish_uses_context_publisher_without_deadlock() {
    let bus = EventBus::root();
    let handled = Arc::new(Mutex::new(Vec::new()));
    let nested_handled = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        move |context| async move {
            context
                .publisher()
                .publish(
                    test_event_for_type(
                        TestText("nested".to_owned()),
                        TestText(OTHER_EVENT_TYPE.to_owned()),
                    ),
                    metadata_with_event_id(
                        TestText(OTHER_TARGET.to_owned()),
                        TestText("nested-publish-event-1".to_owned()),
                    ),
                )
                .await?;
            Ok(())
        },
    )
    .await
    .expect_value("publisher subscriber registers");
    bus.subscribe::<TestEvent, _, _>(
        subscriber_for_event(
            TestText(OTHER_SUBSCRIBER.to_owned()),
            TestText(OTHER_TARGET.to_owned()),
            TestText(OTHER_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let handled = Arc::clone(&nested_handled);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .expect_value("nested subscriber registers");

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("outer publish succeeds");

    assert_eq!(report.handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &["nested".to_string()]);
    assert_eq!(bus.journal().await.len(), 2);
}

#[tokio::test]
async fn detached_publish_returns_observable_report() {
    let bus = EventBus::root();
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async { Ok(()) },
    )
    .await
    .expect_value("subscriber registers");

    let report = bus
        .publish_detached(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            DispatchMode::Sequential,
        )
        .await
        .expect_value("detached task completes")
        .expect_value("detached publish succeeds");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
}

#[tokio::test]
async fn sync_subscriber_adapter_uses_typed_dispatch_path() {
    let bus = EventBus::root();
    let handled = Arc::new(StdMutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let subscription = bus
        .subscribe_sync::<TestEvent, _>(
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            move |context| {
                handled_clone
                    .lock()
                    .expect_value("sync handled lock")
                    .push(context.payload().label.clone());
                Ok(())
            },
        )
        .await
        .expect_value("sync subscriber registers");

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish reaches sync subscriber");

    assert_eq!(
        subscription.event_type.as_str(),
        super::fixtures::TEST_EVENT_TYPE
    );
    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
    assert_eq!(
        handled.lock().expect_value("sync handled lock").as_slice(),
        &[TEST_LABEL.to_string()]
    );
}

#[tokio::test]
async fn panicking_handler_isolated_as_dead_letter_report() {
    let bus = EventBus::root();
    let handled = Arc::new(Mutex::new(0_usize));
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async {
            std::panic::resume_unwind(Box::new("eventing test panic"));
        },
    )
    .await
    .expect_value("subscriber registers");
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(OTHER_SUBSCRIBER.to_owned()),
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
    .expect_value("sibling subscriber registers");

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish survives handler panic");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.handler_reports.len(), 2);
    assert_eq!(report.handler_reports[0].outcome, HandlerOutcome::Panicked);
    assert_eq!(report.handler_reports[1].outcome, HandlerOutcome::Handled);
    assert_eq!(report.handled_count, 1);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(*handled.lock().await, 1);
    let dead_letter = &dead_letters[0];
    assert_eq!(dead_letter.envelope.event_id, report.event_id);
    assert_eq!(dead_letter.envelope.contract.event_type, report.event_type);
    assert_eq!(
        dead_letter
            .subscriber_id
            .as_ref()
            .expect_value("handler dead letter has subscriber")
            .as_str(),
        TEST_SUBSCRIBER
    );
    assert_eq!(
        dead_letter
            .target_handler
            .as_ref()
            .expect_value("handler dead letter has target")
            .as_str(),
        TEST_TARGET
    );
    assert_eq!(dead_letter.reason, DeadLetterReason::HandlerPanicked);
    assert_eq!(
        dead_letter.retry_state,
        DeadLetterRetryState::Exhausted { attempts: 1 }
    );
    assert!(matches!(
        &dead_letter.error,
        EventingError::HandlerPanicked { subscriber_id }
            if subscriber_id.as_str() == TEST_SUBSCRIBER
    ));
}

#[tokio::test]
async fn subscription_handle_unsubscribe_is_idempotent() {
    let bus = EventBus::root();
    let handled = Arc::new(Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    let handle = bus
        .subscribe_with_handle::<TestEvent, _, _>(
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
        .expect_value("subscriber registers with handle");

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("first publish succeeds");
    let first_unsubscribe = handle.unsubscribe();
    let second_unsubscribe = handle.unsubscribe();
    let second_report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("second publish succeeds");

    assert_eq!(report.handled_count, 1);
    assert!(first_unsubscribe.removed);
    assert!(!second_unsubscribe.removed);
    assert_eq!(second_report.subscriber_count, 0);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn subscription_handle_drop_unsubscribes_handler() {
    let bus = EventBus::root();
    let handled = Arc::new(Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    {
        let _handle = bus
            .subscribe_with_handle::<TestEvent, _, _>(
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
            .expect_value("subscriber registers with handle");

        let report = bus
            .publish(
                test_event(TestText(TEST_LABEL.to_owned())),
                metadata(TestText(TEST_TARGET.to_owned())),
            )
            .await
            .expect_value("publish succeeds before handle drop");
        assert_eq!(report.subscriber_count, 1);
        assert_eq!(report.handled_count, 1);
    }

    let report_after_drop = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish succeeds after handle drop");

    assert_eq!(report_after_drop.subscriber_count, 0);
    assert_eq!(report_after_drop.handled_count, 0);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn registrar_dispose_removes_all_owned_subscriptions() {
    let bus = EventBus::root();
    let mut registrar = EventRegistrar::new();
    registrar
        .subscribe::<TestEvent, _, _>(
            &bus,
            subscriber(
                TestText(TEST_SUBSCRIBER.to_owned()),
                TestText(TEST_TARGET.to_owned()),
            ),
            |_| async { Ok(()) },
        )
        .await
        .expect_value("registrar subscribes");

    let dispose_report = registrar.dispose();
    let second_dispose_report = registrar.dispose();
    let publish_report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish after dispose succeeds");
    let subscribe_after_dispose = registrar
        .subscribe::<TestEvent, _, _>(
            &bus,
            subscriber(
                TestText(OTHER_SUBSCRIBER.to_owned()),
                TestText(OTHER_TARGET.to_owned()),
            ),
            |_| async { Ok(()) },
        )
        .await;

    assert_eq!(dispose_report.reports.len(), 1);
    assert!(dispose_report.reports[0].removed);
    assert!(second_dispose_report.reports.is_empty());
    assert!(registrar.is_disposed());
    assert_eq!(publish_report.subscriber_count, 0);
    assert!(matches!(
        subscribe_after_dispose,
        Err(EventingError::RegistrarDisposed)
    ));
}
