use crate::ExpectValue;
use std::{future, sync::Arc, time::Duration};

use tokio::sync::Mutex;

use super::{
    fixtures::{metadata, metadata_with_event_id, subscriber_for_event, TEST_TARGET},
    request_response_support::{
        test_request, test_request_with_id, test_result_event, InvalidContractRequestEvent,
        TestRequestEvent, TestResponse, REQUEST_EVENT_TYPE, REQUEST_ID, RESULT_EVENT_TYPE,
    },
};
use crate::{EventPublisher, EventingError, RequestCompletionOutcome, RequestId, RequestOptions};

const REQUEST_TERMINAL_RETENTION_PROBE_COUNT: usize = 4097;

#[tokio::test]
async fn publish_request_resolves_associated_response_type() {
    let bus = crate::EventBus::new();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("resolve-associated-response"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request resolves");

    assert_eq!(report.request_id.as_str(), REQUEST_ID);
    assert_eq!(report.response.decision, "approved");
    assert_eq!(report.publish_report.handled_count, 1);
}

#[tokio::test]
async fn request_terminal_retention_uses_completion_order_not_request_id_sort_order() {
    let bus = crate::EventBus::new();
    let captured_publisher = Arc::new(Mutex::new(None::<EventPublisher>));
    let captured_publisher_clone = Arc::clone(&captured_publisher);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            "request-retention-subscriber",
            TEST_TARGET,
            REQUEST_EVENT_TYPE,
        ),
        move |context| {
            let captured_publisher = Arc::clone(&captured_publisher_clone);
            async move {
                *captured_publisher.lock().await = Some(context.publisher().clone());
                context.complete_request(TestResponse::approved()).await?;
                Ok(())
            }
        },
    )
    .await
    .expect_value("request retention subscriber registers");

    let oldest = RequestId::parse("request-z-oldest").expect_value("oldest request id parses");
    publish_retention_probe_request(
        &bus,
        "request-z-oldest",
        "request-z-oldest",
        "request-retention-event-oldest",
    )
    .await;

    let first_new = RequestId::parse("request-a-0000").expect_value("first new request id parses");
    for index in 0..(REQUEST_TERMINAL_RETENTION_PROBE_COUNT - 1) {
        let request_id = format!("request-a-{index:04}");
        let event_id = format!("request-retention-event-{index:04}");
        publish_retention_probe_request(&bus, &request_id, &request_id, &event_id).await;
    }

    let publisher = captured_publisher
        .lock()
        .await
        .clone()
        .expect_value("request publisher captured");
    assert_eq!(
        publisher
            .complete_request::<TestRequestEvent>(oldest, TestResponse::approved())
            .await
            .expect_value("evicted oldest reports late")
            .outcome,
        RequestCompletionOutcome::Late
    );
    assert_eq!(
        publisher
            .complete_request::<TestRequestEvent>(first_new, TestResponse::approved())
            .await
            .expect_value("newer low-sorted request remains retained")
            .outcome,
        RequestCompletionOutcome::Duplicate
    );
    assert_eq!(
        bus.metrics_snapshot()
            .await
            .requests
            .completed_request_count,
        4096
    );
}

async fn publish_retention_probe_request(
    bus: &crate::EventBus,
    label: &str,
    request_id: &str,
    event_id: &str,
) {
    bus.publish_request(
        test_request_with_id(label, request_id),
        metadata_with_event_id(TEST_TARGET, event_id),
        RequestOptions::with_timeout(Duration::from_millis(50))
            .expect_value("request options valid"),
    )
    .await
    .expect_value("request retention probe resolves");
}

#[tokio::test]
async fn invalid_response_validation_does_not_settle_request() {
    let bus = crate::EventBus::new();
    let invalid_rejected = Arc::new(Mutex::new(false));
    let invalid_rejected_clone = Arc::clone(&invalid_rejected);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        move |context| {
            let invalid_rejected = Arc::clone(&invalid_rejected_clone);
            async move {
                let invalid = context.complete_request(TestResponse::invalid()).await;
                *invalid_rejected.lock().await = invalid.is_err();
                context.complete_request(TestResponse::approved()).await?;
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("validate-before-settle"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request resolves after valid response");

    assert!(*invalid_rejected.lock().await);
    assert_eq!(report.response.decision, "approved");
}

#[tokio::test]
async fn request_timeout_reports_late_response_without_mutating_result() {
    let bus = crate::EventBus::new();
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let outcomes_clone = Arc::clone(&outcomes);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        move |context| {
            let outcomes = Arc::clone(&outcomes_clone);
            async move {
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    let report = context
                        .complete_request(TestResponse::approved())
                        .await
                        .expect_value("late completion reports");
                    outcomes.lock().await.push(report.outcome);
                });
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request("timeout"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(5))
                .expect_value("request options valid"),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert_eq!(
        outcomes.lock().await.as_slice(),
        &[RequestCompletionOutcome::Late]
    );
}

#[tokio::test]
async fn request_timeout_covers_slow_handler_dispatch() {
    let bus = crate::EventBus::new();
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let outcomes_clone = Arc::clone(&outcomes);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        move |context| {
            let outcomes = Arc::clone(&outcomes_clone);
            async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                let report = context.complete_request(TestResponse::approved()).await?;
                outcomes.lock().await.push(report.outcome);
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request("slow-handler-timeout"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(5))
                .expect_value("request options valid"),
        )
        .await;
    tokio::time::sleep(Duration::from_millis(75)).await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert!(outcomes.lock().await.is_empty());
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
}

#[tokio::test]
async fn request_timeout_aborts_never_completing_publish_and_releases_in_flight() {
    let bus = crate::EventBus::new();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        |_context| async move { future::pending::<Result<(), EventingError>>().await },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request("never-completing-handler-timeout"),
            metadata_with_event_id(TEST_TARGET, "request-never-completing-event"),
            RequestOptions::with_timeout(Duration::from_millis(5))
                .expect_value("request options valid"),
        )
        .await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
}

#[tokio::test]
async fn publish_request_cancels_registry_entry_when_publish_fails() {
    let bus = crate::EventBus::new();
    let failed = bus
        .publish_request(
            InvalidContractRequestEvent::new(),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await;
    assert!(matches!(failed, Err(EventingError::InvalidVersion)));

    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");
    let report = bus
        .publish_request(
            test_request("retry-after-publish-failure"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request id can be reused after failed publish");

    assert_eq!(report.response.decision, "approved");
}

#[tokio::test]
async fn double_completion_is_ignored_and_reported() {
    let bus = crate::EventBus::new();
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let outcomes_clone = Arc::clone(&outcomes);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        move |context| {
            let outcomes = Arc::clone(&outcomes_clone);
            async move {
                let first = context.complete_request(TestResponse::approved()).await?;
                let second = context.complete_request(TestResponse::approved()).await?;
                outcomes
                    .lock()
                    .await
                    .extend([first.outcome, second.outcome]);
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("double-completion"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("first request completion resolves");

    assert_eq!(report.response.decision, "approved");
    assert_eq!(
        outcomes.lock().await.as_slice(),
        &[
            RequestCompletionOutcome::Completed,
            RequestCompletionOutcome::Duplicate
        ]
    );
}

#[tokio::test]
async fn durable_result_event_pattern_remains_separate_from_local_completion() {
    let bus = crate::EventBus::new();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        move |context| async move {
            context
                .publisher()
                .publish(
                    test_result_event(),
                    metadata_with_event_id(TEST_TARGET, "request-result-event-1"),
                )
                .await?;
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("durable-result-event"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("request resolves");
    let journal = bus.journal().await;

    assert_eq!(report.response.decision, "approved");
    assert_eq!(journal.len(), 2);
    assert_eq!(journal[0].contract.event_type.as_str(), REQUEST_EVENT_TYPE);
    assert_eq!(journal[1].contract.event_type.as_str(), RESULT_EVENT_TYPE);
}
