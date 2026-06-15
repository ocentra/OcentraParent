use serde::{Deserialize, Serialize};

use crate::bus::subscriber::EventSubscriber;
use crate::envelope::{DomainEvent, EventContract, EventMetadata, EventSource};
use crate::error::EventingError;
use crate::ids::{
    AggregateKey, CorrelationId, EventCustody, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, SubscriberId,
    TargetHandler,
};

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
    fixture_or_exit(
        try_test_event_for_type_with_aggregate_and_idempotency(
            label,
            TEST_AGGREGATE,
            TEST_EVENT_TYPE,
            TEST_IDEMPOTENCY,
        ),
        "test_event",
    )
}

pub(super) fn test_event_with_aggregate(label: &str, aggregate_key: &str) -> TestEvent {
    fixture_or_exit(
        try_test_event_for_type_with_aggregate_and_idempotency(
            label,
            aggregate_key,
            TEST_EVENT_TYPE,
            TEST_IDEMPOTENCY,
        ),
        "test_event_with_aggregate",
    )
}

pub(super) fn test_event_for_type(label: &str, event_type: &str) -> TestEvent {
    fixture_or_exit(
        try_test_event_for_type_with_aggregate_and_idempotency(
            label,
            TEST_AGGREGATE,
            event_type,
            TEST_IDEMPOTENCY,
        ),
        "test_event_for_type",
    )
}

pub(super) fn test_event_with_idempotency(label: &str, idempotency_key: &str) -> TestEvent {
    fixture_or_exit(
        try_test_event_for_type_with_aggregate_and_idempotency(
            label,
            TEST_AGGREGATE,
            TEST_EVENT_TYPE,
            idempotency_key,
        ),
        "test_event_with_idempotency",
    )
}

pub(super) fn test_event_for_type_with_aggregate_and_idempotency(
    label: &str,
    aggregate_key: &str,
    event_type: &str,
    idempotency_key: &str,
) -> TestEvent {
    fixture_or_exit(
        try_test_event_for_type_with_aggregate_and_idempotency(
            label,
            aggregate_key,
            event_type,
            idempotency_key,
        ),
        "test_event_for_type_with_aggregate_and_idempotency",
    )
}

fn try_test_event_for_type_with_aggregate_and_idempotency(
    label: &str,
    aggregate_key: &str,
    event_type: &str,
    idempotency_key: &str,
) -> Result<TestEvent, EventingError> {
    Ok(TestEvent {
        label: label.to_string(),
        aggregate_key: AggregateKey::parse(aggregate_key)?,
        idempotency_key: IdempotencyKey::parse(idempotency_key)?,
        event_type: EventType::parse(event_type)?,
    })
}

pub(super) fn metadata(target: &str) -> EventMetadata {
    fixture_or_exit(
        try_metadata_with_event_id(target, TEST_EVENT_ID),
        "metadata",
    )
}

pub(super) fn metadata_with_event_id(target: &str, event_id: &str) -> EventMetadata {
    fixture_or_exit(
        try_metadata_with_event_id(target, event_id),
        "metadata_with_event_id",
    )
}

fn try_metadata_with_event_id(target: &str, event_id: &str) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        crate::ids::EventId::parse(event_id)?,
        CorrelationId::parse(TEST_CORRELATION_ID)?,
        EventSource::new(
            EventCustody::parse(TEST_CUSTODY)?,
            RuntimeRole::parse(TEST_RUNTIME_ROLE)?,
            SourceService::parse(TEST_SOURCE_SERVICE)?,
            SourceComponent::parse(TEST_SOURCE_COMPONENT)?,
            RuntimeInstanceId::parse(TEST_INSTANCE)?,
        ),
        RecordedAt::parse(TEST_OBSERVED_AT)?,
        Some(TargetHandler::parse(target)?),
    ))
}

pub(super) fn subscriber(id: &str, target: &str) -> EventSubscriber {
    fixture_or_exit(
        try_subscriber_for_event(id, target, TEST_EVENT_TYPE),
        "subscriber",
    )
}

pub(super) fn subscriber_for_event(id: &str, target: &str, event_type: &str) -> EventSubscriber {
    fixture_or_exit(
        try_subscriber_for_event(id, target, event_type),
        "subscriber_for_event",
    )
}

fn try_subscriber_for_event(
    id: &str,
    target: &str,
    event_type: &str,
) -> Result<EventSubscriber, EventingError> {
    Ok(EventSubscriber::new(
        SubscriberId::parse(id)?,
        EventType::parse(event_type)?,
        TargetHandler::parse(target)?,
    ))
}

fn fixture_or_exit<T>(result: Result<T, EventingError>, context: &'static str) -> T {
    match result {
        Ok(value) => value,
        Err(error) => invalid_fixture(context, &error),
    }
}

fn invalid_fixture(context: &'static str, error: &EventingError) -> ! {
    let _ = (context, error);
    std::process::exit(1);
}
