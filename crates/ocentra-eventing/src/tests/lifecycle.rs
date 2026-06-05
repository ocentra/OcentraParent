use std::{sync::Arc, time::Duration};

use tokio::sync::{Barrier, Mutex};

use super::fixtures::{
    metadata, subscriber, subscriber_for_event, test_event, test_event_for_type,
    test_event_with_aggregate, TestEvent, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET,
    TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::{DispatchMode, EventBus, EventRegistrar, EventingError, HandlerOutcome};

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
    .expect("subscriber registers");

    let first = bus.publish_with_mode(
        test_event("first"),
        metadata(TEST_TARGET),
        DispatchMode::OrderedByAggregateKey,
    );
    let second = bus.publish_with_mode(
        test_event("second"),
        metadata(TEST_TARGET),
        DispatchMode::OrderedByAggregateKey,
    );
    let (first_report, second_report) = tokio::join!(first, second);

    assert_eq!(
        first_report.expect("first publish succeeds").handled_count,
        1
    );
    assert_eq!(
        second_report
            .expect("second publish succeeds")
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
    .expect("subscriber registers");

    let first = bus.publish_with_mode(
        test_event_with_aggregate("first", "aggregate-a"),
        metadata(TEST_TARGET),
        DispatchMode::OrderedByAggregateKey,
    );
    let second = bus.publish_with_mode(
        test_event_with_aggregate("second", "aggregate-b"),
        metadata(TEST_TARGET),
        DispatchMode::OrderedByAggregateKey,
    );
    let result = tokio::time::timeout(Duration::from_secs(1), async {
        tokio::join!(first, second)
    })
    .await
    .expect("different aggregate publishes complete without serial deadlock");

    assert_eq!(result.0.expect("first publish succeeds").handled_count, 1);
    assert_eq!(result.1.expect("second publish succeeds").handled_count, 1);
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
                    metadata(OTHER_TARGET),
                )
                .await?;
            Ok(())
        },
    )
    .await
    .expect("publisher subscriber registers");
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
    .expect("nested subscriber registers");

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("outer publish succeeds");

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
    .expect("subscriber registers");

    let report = bus
        .publish_detached(
            test_event(TEST_LABEL),
            metadata(TEST_TARGET),
            DispatchMode::Sequential,
        )
        .await
        .expect("detached task completes")
        .expect("detached publish succeeds");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
}

#[tokio::test]
async fn panicking_handler_isolated_as_dead_letter_report() {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        panic!("eventing test panic");
        #[allow(unreachable_code)]
        Ok(())
    })
    .await
    .expect("subscriber registers");

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("publish survives handler panic");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.handler_reports[0].outcome, HandlerOutcome::Panicked);
    assert_eq!(report.handled_count, 0);
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(
        dead_letters[0]
            .subscriber_id
            .as_ref()
            .expect("handler dead letter has subscriber")
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
        .expect("subscriber registers with handle");

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("first publish succeeds");
    drop(handle);
    let second_report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("second publish succeeds");

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
        .expect("registrar subscribes");

    let dispose_report = registrar.dispose();
    let publish_report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("publish after dispose succeeds");
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
