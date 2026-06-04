use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use super::fixtures::{metadata, subscriber_for_event, TEST_TARGET};
use crate::{
    AggregateKey, DomainEvent, EventContract, EventResponseContract, EventingError, IdempotencyKey,
    RequestCompletionOutcome, RequestEvent, RequestId, RequestOptions, SchemaVersion,
};

const REQUEST_EVENT_TYPE: &str = "eventing.test.requested";
const RESULT_EVENT_TYPE: &str = "eventing.test.completed";
const REQUEST_AGGREGATE: &str = "request-aggregate";
const REQUEST_ID: &str = "request-response-id";
const REQUEST_IDEMPOTENCY: &str = "request-idempotency";
const RESULT_IDEMPOTENCY: &str = "request-result-idempotency";

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
    .expect("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("resolve-associated-response"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50)).expect("request options valid"),
        )
        .await
        .expect("request resolves");

    assert_eq!(report.request_id.as_str(), REQUEST_ID);
    assert_eq!(report.response.decision, "approved");
    assert_eq!(report.publish_report.handled_count, 1);
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
    .expect("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("validate-before-settle"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50)).expect("request options valid"),
        )
        .await
        .expect("request resolves after valid response");

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
                        .expect("late completion reports");
                    outcomes.lock().await.push(report.outcome);
                });
                Ok(())
            }
        },
    )
    .await
    .expect("request subscriber registers");

    let result = bus
        .publish_request(
            test_request("timeout"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(5)).expect("request options valid"),
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
    .expect("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("double-completion"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50)).expect("request options valid"),
        )
        .await
        .expect("first request completion resolves");

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
                .publisher
                .publish(test_result_event(), metadata(TEST_TARGET))
                .await?;
            context.complete_request(TestResponse::approved()).await?;
            Ok(())
        },
    )
    .await
    .expect("request subscriber registers");

    let report = bus
        .publish_request(
            test_request("durable-result-event"),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(50)).expect("request options valid"),
        )
        .await
        .expect("request resolves");
    let journal = bus.journal().await;

    assert_eq!(report.response.decision, "approved");
    assert_eq!(journal.len(), 2);
    assert_eq!(journal[0].contract.event_type.as_str(), REQUEST_EVENT_TYPE);
    assert_eq!(journal[1].contract.event_type.as_str(), RESULT_EVENT_TYPE);
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TestRequestEvent {
    label: String,
    request_id: RequestId,
}

impl DomainEvent for TestRequestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            crate::EventType::parse(REQUEST_EVENT_TYPE)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for TestRequestEvent {
    type Response = TestResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TestResponse {
    decision: String,
}

impl TestResponse {
    fn approved() -> Self {
        Self {
            decision: String::from("approved"),
        }
    }

    fn invalid() -> Self {
        Self {
            decision: String::from(" "),
        }
    }
}

impl EventResponseContract for TestResponse {
    fn validate(&self) -> Result<(), EventingError> {
        if self.decision.trim().is_empty() {
            return Err(EventingError::empty_value("test_response_decision"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TestResultEvent {
    label: String,
}

impl DomainEvent for TestResultEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            crate::EventType::parse(RESULT_EVENT_TYPE)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(RESULT_IDEMPOTENCY)
    }
}

fn test_request(label: &str) -> TestRequestEvent {
    TestRequestEvent {
        label: label.to_string(),
        request_id: RequestId::parse(REQUEST_ID).expect("request id parses"),
    }
}

fn test_result_event() -> TestResultEvent {
    TestResultEvent {
        label: String::from("durable-result"),
    }
}
