use super::support::{
    metadata, subscriber, subscriber_for_event, test_event, TestEvent, TestText, OTHER_EVENT_TYPE,
    OTHER_SUBSCRIBER, OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::{DomainEvent, EventContract, EventEnvelope, EventPriority};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CausationId, EventNamespace, EventType, IdempotencyKey, RecordedAt, RequestId,
    SchemaVersion,
};
use ocentra_eventing::request::{EventResponseContract, RequestEvent, RequestOptions};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[tokio::test]
async fn event_bus_dispatches_typed_envelope_and_stores_serialized_boundary() {
    let bus = EventBus::new();
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
    .expect_value("subscriber registers");

    let metadata = metadata(TestText(TEST_TARGET.to_owned()))
        .with_causation_id(
            CausationId::parse("causation-test-1").expect_value("causation id parses"),
        )
        .with_priority(EventPriority::High);
    let report = bus
        .publish(test_event(TestText(TEST_LABEL.to_owned())), metadata)
        .await
        .expect_value("publish succeeds");
    let journal = bus.journal().await;
    let decoded: EventEnvelope<TestEvent> =
        journal[0].decode().expect_value("stored envelope decodes");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert_eq!(decoded.payload().label, TEST_LABEL);
    assert_eq!(
        decoded
            .causation_id()
            .as_ref()
            .expect_value("causation id is stored")
            .as_str(),
        "causation-test-1"
    );
    assert_eq!(decoded.priority(), EventPriority::High);
    assert_eq!(journal.len(), 1);
}

#[tokio::test]
async fn request_completion_rejects_associated_response_type_mismatch() {
    let bus = EventBus::new();
    let mismatch_error = Arc::new(Mutex::new(None));
    let mismatch_error_clone = Arc::clone(&mismatch_error);
    bus.subscribe::<AssociatedResponseRequest, _, _>(
        subscriber_for_event(
            TestText("typed-boundary-request-subscriber".to_owned()),
            TestText(TEST_TARGET.to_owned()),
            TestText(ASSOCIATED_REQUEST_EVENT_TYPE.to_owned()),
        ),
        move |context| {
            let mismatch_error = Arc::clone(&mismatch_error_clone);
            async move {
                let wrong_completion = context
                    .publisher()
                    .complete_request::<MismatchedResponseRequest>(
                        RequestId::parse(ASSOCIATED_REQUEST_ID)
                            .expect_value("associated request id parses"),
                        MismatchedResponse { accepted: true },
                    )
                    .await;
                *mismatch_error.lock().await = wrong_completion.err();
                context
                    .complete_request(AssociatedResponse { accepted: true })
                    .await?;
                Ok(())
            }
        },
    )
    .await
    .expect_value("request subscriber registers");

    let report = bus
        .publish_request(
            AssociatedResponseRequest {
                request_id: RequestId::parse(ASSOCIATED_REQUEST_ID)
                    .expect_value("associated request id parses"),
            },
            metadata(TestText(TEST_TARGET.to_owned())),
            RequestOptions::with_timeout(Duration::from_millis(50))
                .expect_value("request options valid"),
        )
        .await
        .expect_value("associated response request completes");

    assert_eq!(report.request_id.as_str(), ASSOCIATED_REQUEST_ID);
    assert!(report.response.accepted);
    let mismatch_error = mismatch_error
        .lock()
        .await
        .take()
        .expect_value("mismatched completion must return an error");
    assert!(matches!(
        mismatch_error,
        EventingError::RequestTypeMismatch { request_id }
            if request_id.as_str() == ASSOCIATED_REQUEST_ID
    ));
}

#[tokio::test]
async fn target_handler_filter_prevents_wrong_handler_delivery() {
    let bus = EventBus::new();
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
    .expect_value("subscriber registers");
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(OTHER_SUBSCRIBER.to_owned()),
            TestText(OTHER_TARGET.to_owned()),
        ),
        |_| async { Ok(()) },
    )
    .await
    .expect_value("second subscriber registers");

    let report = bus
        .publish(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
        )
        .await
        .expect_value("publish succeeds");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn concurrent_dispatch_records_handler_dead_letter_without_losing_journal() {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(
        subscriber(
            TestText(TEST_SUBSCRIBER.to_owned()),
            TestText(TEST_TARGET.to_owned()),
        ),
        |_| async {
            Err(EventingError::InvalidValue {
                field: "handler_failure",
                value: "handler_failure".to_string(),
            })
        },
    )
    .await
    .expect_value("subscriber registers");

    let report = bus
        .publish_with_mode(
            test_event(TestText(TEST_LABEL.to_owned())),
            metadata(TestText(TEST_TARGET.to_owned())),
            ocentra_eventing::bus::DispatchMode::Concurrent,
        )
        .await
        .expect_value("publish succeeds");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(report.handled_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    assert_eq!(
        dead_letters[0]
            .target_handler
            .as_ref()
            .expect_value("handler dead letter has target")
            .as_str(),
        TEST_TARGET
    );
}

#[tokio::test]
async fn duplicate_subscriber_ids_are_rejected() {
    let bus = EventBus::new();
    let duplicate = subscriber(
        TestText(TEST_SUBSCRIBER.to_owned()),
        TestText(TEST_TARGET.to_owned()),
    );
    bus.subscribe::<TestEvent, _, _>(duplicate.clone(), |_| async { Ok(()) })
        .await
        .expect_value("first subscriber registers");

    let result = bus
        .subscribe::<TestEvent, _, _>(duplicate, |_| async { Ok(()) })
        .await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateSubscriber { .. })
    ));
}

#[test]
fn eventing_newtypes_reject_empty_values_and_zero_versions() {
    assert_eq!(
        EventType::parse(""),
        Err(EventingError::EmptyValue {
            field: "event_type"
        })
    );
    assert_eq!(
        EventType::parse(".leading"),
        Err(EventingError::InvalidValue {
            field: "event_type",
            value: ".leading".to_owned(),
        })
    );
    assert_eq!(
        EventType::parse("trailing."),
        Err(EventingError::InvalidValue {
            field: "event_type",
            value: "trailing.".to_owned(),
        })
    );
    assert_eq!(
        EventType::parse("empty..segment"),
        Err(EventingError::InvalidValue {
            field: "event_type",
            value: "empty..segment".to_owned(),
        })
    );
    assert_eq!(
        EventType::parse("eventing/slash-taxonomy/observed")
            .expect_value("slash taxonomy event type parses")
            .as_str(),
        "eventing/slash-taxonomy/observed"
    );
    assert_eq!(
        RecordedAt::parse(" "),
        Err(EventingError::EmptyValue {
            field: "recorded_at"
        })
    );
    assert_eq!(SchemaVersion::new(0), Err(EventingError::InvalidVersion));
}

#[test]
fn event_namespaces_match_dot_and_slash_event_taxonomy() {
    let slash_event =
        EventType::parse("network/transport/observed").expect_value("slash event type parses");
    let dot_event =
        EventType::parse("network.transport.observed").expect_value("dot event type parses");
    let network_namespace = EventNamespace::parse("network").expect_value("namespace parses");

    assert_eq!(
        EventNamespace::from_event_type(&slash_event)
            .expect_value("slash namespace derives")
            .as_str(),
        "network"
    );
    assert!(network_namespace.matches_event_type(&slash_event));
    assert!(network_namespace.matches_event_type(&dot_event));
}

#[test]
fn stored_decode_rejects_contract_mismatch() {
    let envelope = EventEnvelope::from_event(
        test_event(TestText(TEST_LABEL.to_owned())),
        metadata(TestText(TEST_TARGET.to_owned())),
    )
    .expect_value("envelope builds");
    let mut stored = envelope.store().expect_value("stored envelope builds");
    stored.contract.event_type =
        EventType::parse(OTHER_EVENT_TYPE).expect_value("other event parses");

    let decoded = stored.decode::<super::support::TestEvent>();

    assert!(matches!(
        decoded,
        Err(EventingError::ContractMismatch { .. })
    ));
}

const ASSOCIATED_REQUEST_EVENT_TYPE: &str = "eventing.contract.associated-request";
const ASSOCIATED_REQUEST_AGGREGATE: &str = "eventing-contract-associated-aggregate";
const ASSOCIATED_REQUEST_IDEMPOTENCY: &str = "eventing-contract-associated-idempotency";
const ASSOCIATED_REQUEST_ID: &str = "eventing-contract-associated-request-id";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AssociatedResponseRequest {
    request_id: RequestId,
}

impl DomainEvent for AssociatedResponseRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        associated_request_contract()
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(ASSOCIATED_REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(ASSOCIATED_REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for AssociatedResponseRequest {
    type Response = AssociatedResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AssociatedResponse {
    accepted: bool,
}

impl EventResponseContract for AssociatedResponse {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct MismatchedResponseRequest {
    request_id: RequestId,
}

impl DomainEvent for MismatchedResponseRequest {
    fn contract(&self) -> Result<EventContract, EventingError> {
        associated_request_contract()
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(ASSOCIATED_REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(ASSOCIATED_REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for MismatchedResponseRequest {
    type Response = MismatchedResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct MismatchedResponse {
    accepted: bool,
}

impl EventResponseContract for MismatchedResponse {}

fn associated_request_contract() -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(ASSOCIATED_REQUEST_EVENT_TYPE)?,
        SchemaVersion::new(1)?,
    ))
}
