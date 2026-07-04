use crate::ExpectValue;
use std::{sync::Arc, sync::Mutex as StdMutex, time::Duration};

use tokio::sync::{Barrier, Mutex};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type, test_event_with_aggregate, TestEvent, TestText, OTHER_EVENT_TYPE,
    OTHER_SUBSCRIBER, OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::{DispatchMode, EventBus, EventRegistrar, EventingError};
use ocentra_eventing::bus::reports::handler::HandlerOutcome;

#[tokio::test]
async fn ordered_dispatch_serializes_same_aggregate_transitions() {
    let bus = EventBus::new();
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
    let bus = EventBus::new();
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
async fn nested_publish_uses_context_publisher_without_deadlock() {
    let bus = EventBus::new();
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
    let bus = EventBus::new();
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
    let bus = EventBus::new();
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
    let bus = EventBus::new();
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

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish survives handler panic");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.handler_reports[0].outcome, HandlerOutcome::Panicked);
    assert_eq!(report.handled_count, 0);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(
        dead_letters[0]
            .subscriber_id
            .as_ref()
            .expect_value("handler dead letter has subscriber")
            .as_str(),
        TEST_SUBSCRIBER
    );
}

#[tokio::test]
async fn subscription_handle_drop_unsubscribes_handler() {
    let bus = EventBus::new();
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
    drop(handle);
    let second_report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("second publish succeeds");

    assert_eq!(report.handled_count, 1);
    assert_eq!(second_report.subscriber_count, 0);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn registrar_dispose_removes_all_owned_subscriptions() {
    let bus = EventBus::new();
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
    assert!(registrar.is_disposed());
    assert_eq!(publish_report.subscriber_count, 0);
    assert!(matches!(
        subscribe_after_dispose,
        Err(EventingError::RegistrarDisposed)
    ));
}
