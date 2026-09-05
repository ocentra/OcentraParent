use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use ocentra_eventing::bus::{subscriber::EventSubscriber, EventBus};
use ocentra_eventing::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RequestId, RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService,
    SubscriberId, TargetHandler,
};
use ocentra_eventing::request::{EventResponseContract, RequestEvent, RequestOptions};
use serde::{Deserialize, Serialize};

const FIRE_AND_FORGET_EVENT_TYPE: &str = "eventing.integration.fire-and-forget";
const REQUEST_EVENT_TYPE: &str = "eventing.integration.requested";
const SCHEMA_VERSION: u16 = 1;
const AGGREGATE_KEY: &str = "eventing-integration-aggregate";
const FIRE_AND_FORGET_IDEMPOTENCY: &str = "eventing-integration-fire-idempotency";
const REQUEST_IDEMPOTENCY: &str = "eventing-integration-request-idempotency";
const FIRE_AND_FORGET_PAYLOAD_REF: &str = "eventing-integration-fire-payload";
const REQUEST_PAYLOAD_REF: &str = "eventing-integration-request-payload";
const RESPONSE_PAYLOAD_REF: &str = "eventing-integration-response-payload";
const REQUEST_ID: &str = "eventing-integration-request-id";
const EVENT_ID: &str = "eventing-integration-event-id";
const CORRELATION_ID: &str = "eventing-integration-correlation-id";
const OBSERVED_AT: &str = "2026-06-12T12:00:00Z";
const EVENT_CUSTODY: &str = "local-only";
const RUNTIME_ROLE: &str = "child";
const SOURCE_SERVICE: &str = "eventing-integration-service";
const SOURCE_COMPONENT: &str = "eventing-integration-component";
const RUNTIME_INSTANCE_ID: &str = "eventing-integration-runtime";
const FIRE_SUBSCRIBER_ID: &str = "eventing-integration-fire-subscriber";
const REQUEST_SUBSCRIBER_ID: &str = "eventing-integration-request-subscriber";
const TARGET_HANDLER: &str = "eventing-integration-handler";
const PARSE_EXPECTATION: &str = "eventing integration fixture parses";
const RESPONSE_TIMEOUT_MILLIS: u64 = 50;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct IntegrationPayloadRef(String);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct FireAndForgetEvent {
    payload_ref: IntegrationPayloadRef,
}

impl DomainEvent for FireAndForgetEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        event_contract(TestText(FIRE_AND_FORGET_EVENT_TYPE.to_owned()))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        aggregate_key()
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(FIRE_AND_FORGET_IDEMPOTENCY)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AwaitableRequestEvent {
    request_id: RequestId,
    payload_ref: IntegrationPayloadRef,
}

impl DomainEvent for AwaitableRequestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        event_contract(TestText(REQUEST_EVENT_TYPE.to_owned()))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        aggregate_key()
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for AwaitableRequestEvent {
    type Response = AwaitableResponseEvent;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct AwaitableResponseEvent {
    payload_ref: IntegrationPayloadRef,
    accepted: bool,
}

impl EventResponseContract for AwaitableResponseEvent {}

#[tokio::test]
async fn publish_and_wait_dispatches_typed_fire_and_forget_event() {
    let bus = EventBus::root();
    let observed_payload = Arc::new(Mutex::new(None));
    let captured_payload = Arc::clone(&observed_payload);

    bus.subscribe::<FireAndForgetEvent, _, _>(fire_subscriber(), move |context| {
        let captured_payload = Arc::clone(&captured_payload);
        async move {
            captured_payload
                .lock()
                .expect_value(PARSE_EXPECTATION)
                .replace(context.payload().payload_ref.clone());
            Ok(())
        }
    })
    .await
    .expect_value(PARSE_EXPECTATION);

    let report = bus
        .publish_and_wait(fire_and_forget_event(), metadata())
        .await
        .expect_value(PARSE_EXPECTATION);

    assert_eq!(report.handled_count, 1);
    assert_eq!(
        observed_payload
            .lock()
            .expect_value(PARSE_EXPECTATION)
            .clone(),
        Some(IntegrationPayloadRef(
            FIRE_AND_FORGET_PAYLOAD_REF.to_owned()
        ))
    );
}

#[tokio::test]
async fn publish_request_waits_for_typed_subscriber_response() {
    let bus = EventBus::root();

    bus.subscribe::<AwaitableRequestEvent, _, _>(request_subscriber(), |context| async move {
        context.complete_request(awaitable_response_event()).await?;
        Ok(())
    })
    .await
    .expect_value(PARSE_EXPECTATION);

    let report = bus
        .publish_request(
            awaitable_request_event(),
            metadata(),
            RequestOptions::with_timeout(Duration::from_millis(RESPONSE_TIMEOUT_MILLIS))
                .expect_value(PARSE_EXPECTATION),
        )
        .await
        .expect_value(PARSE_EXPECTATION);

    assert_eq!(
        report.request_id,
        RequestId::parse(REQUEST_ID).expect_value(PARSE_EXPECTATION)
    );
    assert_eq!(report.response, awaitable_response_event());
    assert_eq!(report.publish_report.handled_count, 1);
}

fn fire_and_forget_event() -> FireAndForgetEvent {
    FireAndForgetEvent {
        payload_ref: IntegrationPayloadRef(FIRE_AND_FORGET_PAYLOAD_REF.to_owned()),
    }
}

fn awaitable_request_event() -> AwaitableRequestEvent {
    AwaitableRequestEvent {
        request_id: RequestId::parse(REQUEST_ID).expect_value(PARSE_EXPECTATION),
        payload_ref: IntegrationPayloadRef(REQUEST_PAYLOAD_REF.to_owned()),
    }
}

fn awaitable_response_event() -> AwaitableResponseEvent {
    AwaitableResponseEvent {
        payload_ref: IntegrationPayloadRef(RESPONSE_PAYLOAD_REF.to_owned()),
        accepted: true,
    }
}

fn fire_subscriber() -> EventSubscriber {
    subscriber(
        TestText(FIRE_SUBSCRIBER_ID.to_owned()),
        TestText(FIRE_AND_FORGET_EVENT_TYPE.to_owned()),
    )
}

fn request_subscriber() -> EventSubscriber {
    subscriber(
        TestText(REQUEST_SUBSCRIBER_ID.to_owned()),
        TestText(REQUEST_EVENT_TYPE.to_owned()),
    )
}

#[derive(Clone)]
pub(super) struct TestText(pub(super) String);

fn subscriber(id: TestText, event_type: TestText) -> EventSubscriber {
    EventSubscriber::new(
        SubscriberId::parse(id.0).expect_value(PARSE_EXPECTATION),
        EventType::parse(event_type.0).expect_value(PARSE_EXPECTATION),
        TargetHandler::parse(TARGET_HANDLER).expect_value(PARSE_EXPECTATION),
    )
}

fn metadata() -> EventMetadata {
    EventMetadata::from_parts(
        EventId::parse(EVENT_ID).expect_value(PARSE_EXPECTATION),
        CorrelationId::parse(CORRELATION_ID).expect_value(PARSE_EXPECTATION),
        EventSource::new(
            EventCustody::parse(EVENT_CUSTODY).expect_value(PARSE_EXPECTATION),
            RuntimeRole::parse(RUNTIME_ROLE).expect_value(PARSE_EXPECTATION),
            SourceService::parse(SOURCE_SERVICE).expect_value(PARSE_EXPECTATION),
            SourceComponent::parse(SOURCE_COMPONENT).expect_value(PARSE_EXPECTATION),
            RuntimeInstanceId::parse(RUNTIME_INSTANCE_ID).expect_value(PARSE_EXPECTATION),
        ),
        RecordedAt::parse(OBSERVED_AT).expect_value(PARSE_EXPECTATION),
        Some(TargetHandler::parse(TARGET_HANDLER).expect_value(PARSE_EXPECTATION)),
    )
}

fn aggregate_key() -> Result<AggregateKey, EventingError> {
    AggregateKey::parse(AGGREGATE_KEY)
}

fn event_contract(event_type: TestText) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type.0)?,
        SchemaVersion::new(SCHEMA_VERSION)?,
    ))
}
