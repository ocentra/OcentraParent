use ocentra_eventing::envelope::{
    DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource, StoredEventEnvelope,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, TargetHandler,
};
use serde::{Deserialize, Serialize};

const TEST_EVENT_TYPE: &str = "eventing.version-skew.roundtrip";
const TEST_EVENT_ID: &str = "eventing-version-skew-event-id";
const TEST_CORRELATION_ID: &str = "eventing-version-skew-correlation-id";
const TEST_AGGREGATE_KEY: &str = "eventing-version-skew-aggregate";
const TEST_IDEMPOTENCY_KEY: &str = "eventing-version-skew-idempotency";
const TEST_CUSTODY: &str = "local-only";
const TEST_RUNTIME_ROLE: &str = "parent";
const TEST_SOURCE_SERVICE: &str = "eventing-version-skew-service";
const TEST_SOURCE_COMPONENT: &str = "eventing-version-skew-component";
const TEST_RUNTIME_INSTANCE: &str = "eventing-version-skew-runtime";
const TEST_TARGET: &str = "eventing-version-skew-target";
const TEST_OBSERVED_AT: &str = "2026-06-13T20:20:00Z";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct VersionedRoundtripEvent {
    label: String,
}

impl DomainEvent for VersionedRoundtripEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(TEST_EVENT_TYPE)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(TEST_AGGREGATE_KEY)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(TEST_IDEMPOTENCY_KEY)
    }
}

#[test]
fn stored_envelope_rejects_newer_schema_version_without_silent_decode() {
    let live = EventEnvelope::from_event(
        VersionedRoundtripEvent {
            label: String::from("current-contract"),
        },
        metadata(),
    )
    .expect_value("live envelope builds");
    let mut stored = live.store().expect_value("stored envelope builds");
    stored.contract.schema_version =
        SchemaVersion::new(2).expect_value("newer schema version parses");

    let error = stored
        .decode::<VersionedRoundtripEvent>()
        .expect_err_value("newer stored schema version must fail closed");

    assert_eq!(
        error,
        EventingError::ContractMismatch {
            expected: EventType::parse(TEST_EVENT_TYPE).expect_value("event type parses"),
            received: EventType::parse(TEST_EVENT_TYPE).expect_value("event type parses"),
            expected_schema_version: SchemaVersion::new(1)
                .expect_value("expected schema version parses"),
            received_schema_version: SchemaVersion::new(2)
                .expect_value("received schema version parses"),
        }
    );
    assert_eq!(
        error.to_string(),
        "event contract mismatch: expected eventing.version-skew.roundtrip@1, received eventing.version-skew.roundtrip@2"
    );
}

#[test]
fn stored_envelope_rejects_older_schema_version_without_silent_decode() {
    let live = EventEnvelope::from_event(
        VersionedRoundtripEvent {
            label: String::from("current-contract"),
        },
        metadata(),
    )
    .expect_value("live envelope builds");
    let mut stored = live.store().expect_value("stored envelope builds");
    stored.contract.schema_version =
        SchemaVersion::new(1).expect_value("stored current schema version parses");

    let stored_json = serde_json::to_value(&stored).expect_value("stored envelope serializes");
    let mut skewed_json = stored_json;
    skewed_json["contract"]["schemaVersion"] = serde_json::Value::from(0);

    let error = serde_json::from_value::<StoredEventEnvelope>(skewed_json)
        .expect_err_value("zero stored schema version must fail during deserialize");

    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn current_schema_roundtrips_without_contract_or_payload_drift() {
    let live = EventEnvelope::from_event(
        VersionedRoundtripEvent {
            label: String::from("current-contract"),
        },
        metadata(),
    )
    .expect_value("live envelope builds");
    let stored = live.store().expect_value("stored envelope builds");
    let stored_json = serde_json::to_value(&stored).expect_value("stored envelope serializes");

    assert_eq!(stored_json["contract"]["eventType"], TEST_EVENT_TYPE);
    assert_eq!(stored_json["contract"]["schemaVersion"], 1);

    let restored = serde_json::from_value::<StoredEventEnvelope>(stored_json)
        .expect_value("current stored envelope deserializes");
    let decoded = restored
        .decode::<VersionedRoundtripEvent>()
        .expect_value("current stored envelope decodes");

    assert_eq!(decoded.contract().event_type.as_str(), TEST_EVENT_TYPE);
    assert_eq!(decoded.contract().schema_version.value(), 1);
    assert_eq!(decoded.payload().label, "current-contract");
}

fn metadata() -> EventMetadata {
    EventMetadata::from_parts(
        EventId::parse(TEST_EVENT_ID).expect_value("event id parses"),
        CorrelationId::parse(TEST_CORRELATION_ID).expect_value("correlation id parses"),
        EventSource::new(
            EventCustody::parse(TEST_CUSTODY).expect_value("custody parses"),
            RuntimeRole::parse(TEST_RUNTIME_ROLE).expect_value("runtime role parses"),
            SourceService::parse(TEST_SOURCE_SERVICE).expect_value("source service parses"),
            SourceComponent::parse(TEST_SOURCE_COMPONENT).expect_value("source component parses"),
            RuntimeInstanceId::parse(TEST_RUNTIME_INSTANCE).expect_value("runtime instance parses"),
        ),
        RecordedAt::parse(TEST_OBSERVED_AT).expect_value("observed at parses"),
        Some(TargetHandler::parse(TEST_TARGET).expect_value("target handler parses")),
    )
}
