use ocentra_eventing::{
    AggregateKey, CorrelationId, DomainEvent, EventContract, EventCustody, EventEnvelope,
    EventMetadata, EventSource, EventType, EventingError, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, TargetHandler,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

const TEST_EVENT_TYPE: &str = "eventing.unit.contract-boundary";
const TEST_EVENT_ID: &str = "eventing-unit-envelope-id";
const TEST_CORRELATION_ID: &str = "eventing-unit-envelope-correlation";
const TEST_AGGREGATE_KEY: &str = "eventing-unit-envelope-aggregate";
const TEST_IDEMPOTENCY_KEY: &str = "eventing-unit-envelope-idempotency";
const TEST_CUSTODY: &str = "local-only";
const TEST_RUNTIME_ROLE: &str = "parent";
const TEST_SOURCE_SERVICE: &str = "eventing-unit-envelope-service";
const TEST_SOURCE_COMPONENT: &str = "eventing-unit-envelope-component";
const TEST_RUNTIME_INSTANCE: &str = "eventing-unit-envelope-runtime";
const TEST_TARGET: &str = "eventing-unit-envelope-target";
const TEST_OBSERVED_AT: &str = "2026-06-13T20:15:00Z";
const OTHER_EVENT_TYPE: &str = "eventing.unit.contract-boundary.other";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct EnvelopeBoundaryEvent {
    label: String,
}

impl DomainEvent for EnvelopeBoundaryEvent {
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
fn event_contract_serde_rejects_zero_schema_version() {
    let result = serde_json::from_value::<EventContract>(json!({
        "event_type": TEST_EVENT_TYPE,
        "schema_version": 0
    }));

    let error = result.expect_err("zero schema version must fail");
    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn live_and_stored_envelopes_preserve_contract_and_metadata() {
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata(),
    )
    .expect("live envelope builds");
    let stored = live.store().expect("stored envelope builds");
    let decoded: EventEnvelope<EnvelopeBoundaryEvent> =
        stored.decode().expect("stored envelope decodes");

    assert_eq!(stored.contract.event_type.as_str(), TEST_EVENT_TYPE);
    assert_eq!(stored.contract.schema_version.value(), 1);
    assert_eq!(stored.event_id.as_str(), TEST_EVENT_ID);
    assert_eq!(stored.correlation_id.as_str(), TEST_CORRELATION_ID);
    assert_eq!(
        stored
            .target_handler
            .as_ref()
            .expect("target handler stored")
            .as_str(),
        TEST_TARGET
    );
    assert_eq!(decoded.payload.label, "typed-boundary");
    assert_eq!(decoded.contract.schema_version.value(), 1);
}

#[test]
fn stored_decode_contract_mismatch_reports_event_type_and_schema_version_context() {
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata(),
    )
    .expect("live envelope builds");
    let mut stored = live.store().expect("stored envelope builds");
    stored.contract.event_type = EventType::parse(OTHER_EVENT_TYPE).expect("other event parses");
    stored.contract.schema_version = SchemaVersion::new(2).expect("received schema version parses");

    let error = stored
        .decode::<EnvelopeBoundaryEvent>()
        .expect_err("contract mismatch must fail closed");

    assert_eq!(
        error,
        EventingError::ContractMismatch {
            expected: EventType::parse(TEST_EVENT_TYPE).expect("expected event parses"),
            received: EventType::parse(OTHER_EVENT_TYPE).expect("received event parses"),
            expected_schema_version: SchemaVersion::new(1).expect("expected schema version parses"),
            received_schema_version: SchemaVersion::new(2).expect("received schema version parses"),
        }
    );
    assert_eq!(
        error.to_string(),
        "event contract mismatch: expected eventing.unit.contract-boundary@1, received eventing.unit.contract-boundary.other@2"
    );
}

fn metadata() -> EventMetadata {
    EventMetadata::from_parts(
        ocentra_eventing::EventId::parse(TEST_EVENT_ID).expect("event id parses"),
        CorrelationId::parse(TEST_CORRELATION_ID).expect("correlation id parses"),
        EventSource::new(
            EventCustody::parse(TEST_CUSTODY).expect("custody parses"),
            RuntimeRole::parse(TEST_RUNTIME_ROLE).expect("runtime role parses"),
            SourceService::parse(TEST_SOURCE_SERVICE).expect("source service parses"),
            SourceComponent::parse(TEST_SOURCE_COMPONENT).expect("source component parses"),
            RuntimeInstanceId::parse(TEST_RUNTIME_INSTANCE).expect("runtime instance parses"),
        ),
        RecordedAt::parse(TEST_OBSERVED_AT).expect("observed at parses"),
        Some(TargetHandler::parse(TEST_TARGET).expect("target handler parses")),
    )
}
