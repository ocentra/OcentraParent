use std::{future, sync::Arc, time::Duration};

use tokio::sync::Mutex;

use super::{
    fixtures::{metadata, metadata_with_event_id, subscriber_for_event, TEST_TARGET},
    request_response_support::{
        test_request, test_result_event, InvalidContractRequestEvent, TestRequestEvent,
        TestResponse, REQUEST_EVENT_TYPE, REQUEST_ID, RESULT_EVENT_TYPE,
    },
};
use crate::error::EventingError;
use crate::ids::RequestId;
use crate::request::{RequestCompletionOutcome, RequestOptions, RequestRegistry};

const REQUEST_TERMINAL_RETENTION_PROBE_COUNT: usize = 4097;

#[tokio::test]
async fn publish_request_resolves_associated_response_type() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        |context| async move {
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    ?;

    let report = bus
        .publish_request(
            test_request("resolve-associated-response")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))?,
        )
        .await
        ?;

    assert_eq!(report.request_id.as_str(), REQUEST_ID);
    assert_eq!(report.response.decision, "approved");
    assert_eq!(report.publish_report.handled_count, 1);
    Ok(())
}

#[test]
fn request_terminal_retention_uses_completion_order_not_request_id_sort_order() -> Result<(), EventingError> {
    let registry = RequestRegistry::default();
    let oldest = RequestId::parse("request-z-oldest")?;
    registry.register(oldest.clone())?;
    registry.complete(oldest.clone(), TestResponse::approved())?;

    let first_new = RequestId::parse("request-a-0000")?;
    for index in 0..(REQUEST_TERMINAL_RETENTION_PROBE_COUNT - 1) {
        let request_id = RequestId::parse(format!("request-a-{index:04}"))?;
        registry.register(request_id.clone())?;
        registry.complete(request_id, TestResponse::approved())?;
    }

    assert_eq!(
        registry.complete(oldest, TestResponse::approved())?.outcome,
        RequestCompletionOutcome::Late
    );
    assert_eq!(
        registry.complete(first_new, TestResponse::approved())?.outcome,
        RequestCompletionOutcome::Duplicate
    );
    assert_eq!(registry.metrics().completed_request_count, 4096);
    Ok(())
}

#[tokio::test]
async fn invalid_response_validation_does_not_settle_request() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
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
    ?;

    let report = bus
        .publish_request(
            test_request("validate-before-settle")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))?,
        )
        .await
        ?;

    assert!(*invalid_rejected.lock().await);
    assert_eq!(report.response.decision, "approved");
    Ok(())
}

#[tokio::test]
async fn request_timeout_reports_late_response_without_mutating_result() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
    let outcomes = Arc::new(Mutex::new(Vec::new()));
    let outcomes_clone = Arc::clone(&outcomes);
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        move |context| {
            let outcomes = Arc::clone(&outcomes_clone);
            async move {
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_millis(20)).await;
                    let report = match context.complete_request(TestResponse::approved()).await {
                        Ok(report) => report,
                        Err(_err) => return,
                    };
                    outcomes.lock().await.push(report.outcome);
                });
                Ok(())
            }
        },
    )
    .await
    ?;

    let result = bus
        .publish_request(
            test_request("timeout")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(5))?,
        )
        .await;
    tokio::time::sleep(Duration::from_millis(50)).await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert_eq!(
        outcomes.lock().await.as_slice(),
        &[RequestCompletionOutcome::Late]
    );
    Ok(())
}

#[tokio::test]
async fn request_timeout_covers_slow_handler_dispatch() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
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
    ?;

    let result = bus
        .publish_request(
            test_request("slow-handler-timeout")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(5))?,
        )
        .await;
    tokio::time::sleep(Duration::from_millis(75)).await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert!(outcomes.lock().await.is_empty());
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
    Ok(())
}

#[tokio::test]
async fn request_timeout_aborts_never_completing_publish_and_releases_in_flight() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
    bus.subscribe::<TestRequestEvent, _, _>(
        subscriber_for_event("request-subscriber", TEST_TARGET, REQUEST_EVENT_TYPE),
        |_context| async move { future::pending::<Result<(), EventingError>>().await },
    )
    .await
    ?;

    let result = bus
        .publish_request(
            test_request("never-completing-handler-timeout")?,
            metadata_with_event_id(TEST_TARGET, "request-never-completing-event"),
            RequestOptions::with_timeout(Duration::from_millis(5))?,
        )
        .await;

    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    let metrics = bus.metrics_snapshot().await;
    assert_eq!(metrics.queue.in_flight_event_id_count, 0);
    assert_eq!(metrics.queue.in_flight_idempotency_key_count, 0);
    assert_eq!(metrics.requests.timed_out_request_count, 1);
    Ok(())
}

#[tokio::test]
async fn publish_request_cancels_registry_entry_when_publish_fails() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
    let failed = bus
        .publish_request(
            InvalidContractRequestEvent::new()?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))?,
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
    ?;
    let report = bus
        .publish_request(
            test_request("retry-after-publish-failure")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))?,
        )
        .await
        ?;

    assert_eq!(report.response.decision, "approved");
    Ok(())
}

#[tokio::test]
async fn double_completion_is_ignored_and_reported() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
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
    ?;

    let report = bus
        .publish_request(
            test_request("double-completion")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))?,
        )
        .await
        ?;

    assert_eq!(report.response.decision, "approved");
    assert_eq!(
        outcomes.lock().await.as_slice(),
        &[
            RequestCompletionOutcome::Completed,
            RequestCompletionOutcome::Duplicate
        ]
    );
    Ok(())
}

#[tokio::test]
async fn durable_result_event_pattern_remains_separate_from_local_completion() -> Result<(), EventingError> {
    let bus = crate::bus::EventBus::new();
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
    ?;

    let report = bus
        .publish_request(
            test_request("durable-result-event")?,
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50))?,
        )
        .await
        ?;
    let journal = bus.journal().await;

    assert_eq!(report.response.decision, "approved");
    assert_eq!(journal.len(), 2);
    assert_eq!(journal[0].contract.event_type.as_str(), REQUEST_EVENT_TYPE);
    assert_eq!(journal[1].contract.event_type.as_str(), RESULT_EVENT_TYPE);
    Ok(())
}
