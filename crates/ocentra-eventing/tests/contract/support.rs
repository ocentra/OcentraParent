use ocentra_eventing::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
};
use ocentra_eventing::bus::subscriber::EventSubscriber;
use serde::{Deserialize, Serialize};

pub(super) const TEST_EVENT_TYPE: &str = "eventing.test.observed";
pub(super) const OTHER_EVENT_TYPE: &str = "eventing.test.other";
const TEST_EVENT_ID: &str = "event-test-1";
const TEST_CORRELATION_ID: &str = "correlation-test-1";
const TEST_AGGREGATE: &str = "aggregate-test-1";
const TEST_IDEMPOTENCY: &str = "idempotency-test-1";
const TEST_SOURCE_SERVICE: &str = "eventing-test-service";
const TEST_SOURCE_COMPONENT: &str = "eventing-test-component";
const TEST_INSTANCE: &str = "eventing-test-instance";
const TEST_CUSTODY: &str = "local-only";
const TEST_RUNTIME_ROLE: &str = "agent";
pub(super) const TEST_TARGET: &str = "eventing-test-handler";
pub(super) const OTHER_TARGET: &str = "eventing-other-handler";
pub(super) const TEST_SUBSCRIBER: &str = "eventing-test-subscriber";
pub(super) const OTHER_SUBSCRIBER: &str = "eventing-other-subscriber";
const TEST_OBSERVED_AT: &str = "2026-06-03T22:30:00Z";
pub(super) const TEST_LABEL: &str = "typed envelope proof";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct TestEvent {
    pub(super) label: String,
    aggregate_key: AggregateKey,
    idempotency_key: IdempotencyKey,
    event_type: EventType,
}

impl DomainEvent for TestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            self.event_type.clone(),
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        Ok(self.aggregate_key.clone())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        Ok(self.idempotency_key.clone())
    }
}

pub(super) fn test_event(label: &str) -> TestEvent {
    test_event_with_aggregate(label, TEST_AGGREGATE)
}

pub(super) fn test_event_with_aggregate(label: &str, aggregate_key: &str) -> TestEvent {
    test_event_for_type_with_aggregate(label, aggregate_key, TEST_EVENT_TYPE)
}

pub(super) fn test_event_for_type(label: &str, event_type: &str) -> TestEvent {
    test_event_for_type_with_aggregate(label, TEST_AGGREGATE, event_type)
}

fn test_event_for_type_with_aggregate(
    label: &str,
    aggregate_key: &str,
    event_type: &str,
) -> TestEvent {
    test_event_for_type_with_aggregate_and_idempotency(
        label,
        aggregate_key,
        event_type,
        TEST_IDEMPOTENCY,
    )
}

pub(super) fn test_event_for_type_with_aggregate_and_idempotency(
    label: &str,
    aggregate_key: &str,
    event_type: &str,
    idempotency_key: &str,
) -> TestEvent {
    TestEvent {
        label: label.to_string(),
        aggregate_key: AggregateKey::parse(aggregate_key).expect_value("aggregate key parses"),
        idempotency_key: IdempotencyKey::parse(idempotency_key)
            .expect_value("idempotency key parses"),
        event_type: EventType::parse(event_type).expect_value("event type parses"),
    }
}

pub(super) fn metadata(target: &str) -> EventMetadata {
    metadata_with_event_id(target, TEST_EVENT_ID)
}

pub(super) fn metadata_with_event_id(target: &str, event_id: &str) -> EventMetadata {
    EventMetadata::from_parts(
        EventId::parse(event_id).expect_value("event id parses"),
        CorrelationId::parse(TEST_CORRELATION_ID).expect_value("correlation id parses"),
        source(),
        RecordedAt::parse(TEST_OBSERVED_AT).expect_value("recorded at parses"),
        Some(TargetHandler::parse(target).expect_value("target handler parses")),
    )
}

fn source() -> EventSource {
    EventSource::new(
        EventCustody::parse(TEST_CUSTODY).expect_value("event custody parses"),
        RuntimeRole::parse(TEST_RUNTIME_ROLE).expect_value("runtime role parses"),
        SourceService::parse(TEST_SOURCE_SERVICE).expect_value("source service parses"),
        SourceComponent::parse(TEST_SOURCE_COMPONENT).expect_value("source component parses"),
        RuntimeInstanceId::parse(TEST_INSTANCE).expect_value("runtime instance parses"),
    )
}

pub(super) fn subscriber(id: &str, target: &str) -> EventSubscriber {
    subscriber_for_event(id, target, TEST_EVENT_TYPE)
}

pub(super) fn subscriber_for_event(id: &str, target: &str, event_type: &str) -> EventSubscriber {
    EventSubscriber::new(
        SubscriberId::parse(id).expect_value("subscriber id parses"),
        EventType::parse(event_type).expect_value("event type parses"),
        TargetHandler::parse(target).expect_value("target handler parses"),
    )
}
