use crate::ExpectValue;
use std::{future, sync::Arc, time::Duration};

use tokio::sync::Mutex;

use super::{
    fixtures::{metadata, metadata_with_event_id, subscriber_for_event, TestText, TEST_TARGET},
    request_response_support::{
        test_request, test_request_with_id, test_result_event, InvalidContractRequestEvent,
        TestRequestEvent, TestResponse, TestText as RequestText, REQUEST_EVENT_TYPE, REQUEST_ID,
        RESULT_EVENT_TYPE,
    },
};
use crate::{EventPublisher, EventingError, RequestCompletionOutcome, RequestId, RequestOptions};

const REQUEST_TERMINAL_RETENTION_PROBE_COUNT: usize = 4097;

#[tokio::test]
async fn publish_request_resolves_associated_response_type() {
    let bus = crate::EventBus::new();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            test_request(RequestText("resolve-associated-response".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
            TestText("request-retention-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
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
        TestText("request-z-oldest".to_owned()),
        TestText("request-z-oldest".to_owned()),
        TestText("request-retention-event-oldest".to_owned()),
    )
    .await;

    let first_new = RequestId::parse("request-a-0000").expect_value("first new request id parses");
    for index in 0..(REQUEST_TERMINAL_RETENTION_PROBE_COUNT - 1) {
        let request_id = format!("request-a-{index:04}");
        let event_id = format!("request-retention-event-{index:04}");
        publish_retention_probe_request(
            &bus,
            TestText(request_id.clone()),
            TestText(request_id.clone()),
            TestText(event_id.clone()),
        )
        .await;
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
    label: TestText,
    request_id: TestText,
    event_id: TestText,
) {
    let label = label.0;
    let request_id = request_id.0;
    let event_id = event_id.0;
    bus.publish_request(
        test_request_with_id(
            RequestText(label.to_owned()),
            RequestText(request_id.to_owned()),
        ),
        metadata_with_event_id(
            TestText(TEST_TARGET.to_owned()),
            TestText(event_id.to_owned()),
        ),
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
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
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
            test_request(RequestText("validate-before-settle".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
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
            test_request(RequestText("timeout".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
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
            test_request(RequestText("slow-handler-timeout".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |_context| async move { future::pending::<Result<(), EventingError>>().await },
    )
    .await
    .expect_value("request subscriber registers");

    let result = bus
        .publish_request(
            test_request(RequestText("never-completing-handler-timeout".to_owned())),
            metadata_with_event_id(
                TestText(TEST_TARGET.to_owned()),
                TestText("request-never-completing-event".to_owned()),
            ),
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
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await;
    assert!(matches!(failed, Err(EventingError::InvalidVersion)));

    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect_value("request subscriber registers");
    let report = bus
        .publish_request(
            test_request(RequestText("retry-after-publish-failure".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
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
            test_request(RequestText("double-completion".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
        subscriber_for_event(
            TestText("request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| async move {
            context
                .publisher()
                .publish(
                    test_result_event(),
                    metadata_with_event_id(
                        TestText(TEST_TARGET.to_owned()),
                        TestText("request-result-event-1".to_owned()),
                    ),
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
            test_request(RequestText("durable-result-event".to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
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
