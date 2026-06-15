use std::{sync::Arc, sync::Mutex as StdMutex, time::Duration};

use tokio::sync::{Barrier, Mutex};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type, test_event_with_aggregate, TestEvent, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER,
    OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::bus::reports::HandlerOutcome;
use crate::bus::DispatchMode;
use crate::bus::EventBus;
use crate::error::EventingError;
use crate::registrar::EventRegistrar;

fn must_ok<T, E>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    }
}

fn must_some<T>(value: Option<T>) -> T {
    match value {
        Some(value) => value,
        None => std::process::abort(),
    }
}

#[tokio::test]
async fn ordered_dispatch_serializes_same_aggregate_transitions() {
    let bus = EventBus::new();
    let observed = Arc::new(Mutex::new(Vec::new()));
    let observed_clone = Arc::clone(&observed);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
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
    })
    .await
    .unwrap_or_else(|_| std::process::abort());

    let first = bus.publish_with_mode(
        test_event("first"),
        metadata_with_event_id(TEST_TARGET, "ordered-same-aggregate-event-1"),
        DispatchMode::OrderedByAggregateKey,
    );
    let second = bus.publish_with_mode(
        test_event("second"),
        metadata_with_event_id(TEST_TARGET, "ordered-same-aggregate-event-2"),
        DispatchMode::OrderedByAggregateKey,
    );
    let (first_report, second_report) = tokio::join!(first, second);

    let first_report = must_ok(first_report);
    let second_report = must_ok(second_report);
    assert_eq!(first_report.handled_count, 1);
    assert_eq!(second_report.handled_count, 1);
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
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let barrier = Arc::clone(&barrier_clone);
        async move {
            barrier.wait().await;
            Ok(())
        }
    })
    .await
    .unwrap_or_else(|_| std::process::abort());

    let first = bus.publish_with_mode(
        test_event_with_aggregate("first", "aggregate-a"),
        metadata_with_event_id(TEST_TARGET, "ordered-different-aggregate-event-1"),
        DispatchMode::OrderedByAggregateKey,
    );
    let second = bus.publish_with_mode(
        test_event_with_aggregate("second", "aggregate-b"),
        metadata_with_event_id(TEST_TARGET, "ordered-different-aggregate-event-2"),
        DispatchMode::OrderedByAggregateKey,
    );
    let result = must_ok(
        tokio::time::timeout(
            Duration::from_secs(1),
            Box::pin(async { tokio::join!(first, second) }),
        )
        .await,
    );

    let first_report = must_ok(result.0);
    let second_report = must_ok(result.1);
    assert_eq!(first_report.handled_count, 1);
    assert_eq!(second_report.handled_count, 1);
    assert_eq!(bus.clear_for_test().await.aggregate_gate_count, 0);
}

#[tokio::test]
async fn nested_publish_uses_context_publisher_without_deadlock() {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(Vec::new()));
    let nested_handled = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(
        subscriber(TEST_SUBSCRIBER, TEST_TARGET),
        move |context| async move {
            context
                .publisher()
                .publish(
                    test_event_for_type("nested", OTHER_EVENT_TYPE),
                    metadata_with_event_id(OTHER_TARGET, "nested-publish-event-1"),
                )
                .await?;
            Ok(())
        },
    )
    .await
    .unwrap_or_else(|_| std::process::abort());
    bus.subscribe::<TestEvent, _, _>(
        subscriber_for_event(OTHER_SUBSCRIBER, OTHER_TARGET, OTHER_EVENT_TYPE),
        move |context| {
            let handled = Arc::clone(&nested_handled);
            async move {
                handled.lock().await.push(context.payload().label.clone());
                Ok(())
            }
        },
    )
    .await
    .unwrap_or_else(|_| std::process::abort());

    let report = must_ok(bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await);

    assert_eq!(report.handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &["nested".to_string()]);
    assert_eq!(bus.journal().await.len(), 2);
}

#[tokio::test]
async fn detached_publish_returns_observable_report() {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Ok(())
    })
    .await
    .unwrap_or_else(|_| std::process::abort());

    let report = must_ok(
        must_ok(
            bus.publish_detached(
                test_event(TEST_LABEL),
                metadata(TEST_TARGET),
                DispatchMode::Sequential,
            )
            .await,
        ),
    );

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
}

#[tokio::test]
async fn sync_subscriber_adapter_uses_typed_dispatch_path() {
    let bus = EventBus::new();
    let handled = Arc::new(StdMutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    let subscription = bus
        .subscribe_sync::<TestEvent, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
            must_ok(handled_clone.lock()).push(context.payload().label.clone());
            Ok(())
        })
        .await
        .unwrap_or_else(|_| std::process::abort());

    let report = must_ok(bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await);

    assert_eq!(
        subscription.event_type.as_str(),
        super::fixtures::TEST_EVENT_TYPE
    );
    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
    assert_eq!(must_ok(handled.lock()).as_slice(), &[TEST_LABEL.to_string()]);
}

#[tokio::test]
async fn failing_handler_isolated_as_dead_letter_report() {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Err(EventingError::BusShutdown)
    })
    .await
    .unwrap_or_else(|_| std::process::abort());

    let report = must_ok(bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await);
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.handler_reports[0].outcome, HandlerOutcome::Failed);
    assert_eq!(report.handled_count, 0);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(dead_letters[0].reason, crate::bus::reports::DeadLetterReason::HandlerFailed);
    assert_eq!(
        must_some(dead_letters[0].subscriber_id.as_ref()).as_str(),
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
            subscriber(TEST_SUBSCRIBER, TEST_TARGET),
            move |_| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    *handled.lock().await += 1;
                    Ok(())
                }
            },
        )
        .await
        .unwrap_or_else(|_| std::process::abort());

    let report = must_ok(bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await);
    drop(handle);
    let second_report = must_ok(bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await);

    assert_eq!(report.handled_count, 1);
    assert_eq!(second_report.subscriber_count, 0);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn registrar_dispose_removes_all_owned_subscriptions() {
    let bus = EventBus::new();
    let mut registrar = EventRegistrar::new();
    registrar
        .subscribe::<TestEvent, _, _>(&bus, subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
            Ok(())
        })
        .await
        .unwrap_or_else(|_| std::process::abort());

    let dispose_report = registrar.dispose();
    let publish_report = must_ok(bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET)).await);
    let subscribe_after_dispose = registrar
        .subscribe::<TestEvent, _, _>(
            &bus,
            subscriber(OTHER_SUBSCRIBER, OTHER_TARGET),
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
