use ocentra_eventing::envelope::{DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService, TargetHandler,
};
use std::error::Error;
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
fn event_contract_serde_rejects_zero_schema_version() -> Result<(), Box<dyn Error>> {
    let result = serde_json::from_value::<EventContract>(json!({
        "event_type": TEST_EVENT_TYPE,
        "schema_version": 0
    }));

    let error = match result {
        Ok(_) => {
            return Err(std::io::Error::other("zero schema version should fail").into());
        }
        Err(error) => error,
    };
    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
    Ok(())
}

#[test]
fn live_and_stored_envelopes_preserve_contract_and_metadata() -> Result<(), Box<dyn Error>> {
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata()?,
    )?;
    let stored = live.store()?;
    let decoded: EventEnvelope<EnvelopeBoundaryEvent> = stored.decode()?;

    assert_eq!(stored.contract.event_type.as_str(), TEST_EVENT_TYPE);
    assert_eq!(stored.contract.schema_version.value(), 1);
    assert_eq!(stored.event_id.as_str(), TEST_EVENT_ID);
    assert_eq!(stored.correlation_id.as_str(), TEST_CORRELATION_ID);
    assert_eq!(
        match stored.target_handler.as_ref() {
            Some(target) => target.as_str(),
            None => {
                return Err(std::io::Error::other("target handler missing").into());
            }
        },
        TEST_TARGET
    );
    assert_eq!(decoded.payload.label, "typed-boundary");
    assert_eq!(decoded.contract.schema_version.value(), 1);
    Ok(())
}

#[test]
fn stored_decode_contract_mismatch_reports_event_type_and_schema_version_context()
    -> Result<(), Box<dyn Error>>
{
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata()?,
    )?;
    let mut stored = live.store()?;
    stored.contract.event_type = EventType::parse(OTHER_EVENT_TYPE)?;
    stored.contract.schema_version = SchemaVersion::new(2)?;

    let error = match stored.decode::<EnvelopeBoundaryEvent>() {
        Ok(_) => {
            return Err(std::io::Error::other("expected contract mismatch").into());
        }
        Err(error) => error,
    };

    assert_eq!(
        error,
        EventingError::ContractMismatch {
            expected: EventType::parse(TEST_EVENT_TYPE)?,
            received: EventType::parse(OTHER_EVENT_TYPE)?,
            expected_schema_version: SchemaVersion::new(1)?,
            received_schema_version: SchemaVersion::new(2)?,
        }
    );
    assert_eq!(
        error.to_string(),
        "event contract mismatch: expected eventing.unit.contract-boundary@1, received eventing.unit.contract-boundary.other@2"
    );
    Ok(())
}

fn metadata() -> Result<EventMetadata, Box<dyn Error>> {
    Ok(EventMetadata::from_parts(
        EventId::parse(TEST_EVENT_ID)?,
        CorrelationId::parse(TEST_CORRELATION_ID)?,
        EventSource::new(
            EventCustody::parse(TEST_CUSTODY)?,
            RuntimeRole::parse(TEST_RUNTIME_ROLE)?,
            SourceService::parse(TEST_SOURCE_SERVICE)?,
            SourceComponent::parse(TEST_SOURCE_COMPONENT)?,
            RuntimeInstanceId::parse(TEST_RUNTIME_INSTANCE)?,
        ),
        RecordedAt::parse(TEST_OBSERVED_AT)?,
        Some(TargetHandler::parse(TEST_TARGET)?),
    ))
}
