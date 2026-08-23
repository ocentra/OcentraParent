use ocentra_eventing::envelope::{
    DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::{ExpectErrValue, ExpectValue};
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, RecordedAt,
    RequestId, RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent, SourceService,
    TargetHandler,
};
use ocentra_eventing::request::{RequestCompletionOutcome, RequestCompletionReport};
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
        "eventType": TEST_EVENT_TYPE,
        "schemaVersion": 0
    }));

    let error = match result {
        Ok(_) => std::process::abort(),
        Err(error) => error,
    };
    assert!(error
        .to_string()
        .contains("event schema version must be nonzero"));
}

#[test]
fn stored_envelope_serde_uses_canonical_eventing_keys() {
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata(),
    )
    .expect_value("live envelope builds");
    let stored_json = serde_json::to_value(live.store().expect_value("stored envelope builds"))
        .expect_value("stored envelope serializes");

    assert_eq!(stored_json["contract"]["eventType"], json!(TEST_EVENT_TYPE));
    assert_eq!(stored_json["contract"]["schemaVersion"], json!(1));
    assert_eq!(
        stored_json["source"]["instanceId"],
        json!(TEST_RUNTIME_INSTANCE)
    );
    assert_eq!(stored_json["eventId"], json!(TEST_EVENT_ID));
    assert_eq!(stored_json["correlationId"], json!(TEST_CORRELATION_ID));
    assert_eq!(stored_json["aggregateKey"], json!(TEST_AGGREGATE_KEY));
    assert_eq!(stored_json["idempotencyKey"], json!(TEST_IDEMPOTENCY_KEY));
    assert_eq!(stored_json["observedAt"], json!(TEST_OBSERVED_AT));
    assert_eq!(stored_json["targetHandler"], json!(TEST_TARGET));
    assert!(stored_json.get("event_id").is_none());
    assert!(stored_json["contract"].get("event_type").is_none());
    assert!(stored_json["source"].get("instance_id").is_none());
}

#[test]
fn request_completion_report_serde_uses_canonical_eventing_keys() {
    let report = RequestCompletionReport {
        request_id: RequestId::parse("request-completion-1").expect_value("request id parses"),
        outcome: RequestCompletionOutcome::Late,
    };
    let report_json = serde_json::to_value(report).expect_value("request report serializes");

    assert_eq!(report_json["requestId"], json!("request-completion-1"));
    assert_eq!(report_json["outcome"], json!("late"));
    assert!(report_json.get("request_id").is_none());
}

#[test]
fn live_and_stored_envelopes_preserve_contract_and_metadata() {
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata(),
    )
    .expect_value("live envelope builds");
    let stored = live.store().expect_value("stored envelope builds");
    let decoded: EventEnvelope<EnvelopeBoundaryEvent> =
        stored.decode().expect_value("stored envelope decodes");

    assert_eq!(stored.contract.event_type.as_str(), TEST_EVENT_TYPE);
    assert_eq!(stored.contract.schema_version.value(), 1);
    assert_eq!(stored.event_id.as_str(), TEST_EVENT_ID);
    assert_eq!(stored.correlation_id.as_str(), TEST_CORRELATION_ID);
    assert_eq!(
        stored
            .target_handler
            .as_ref()
            .expect_value("target handler stored")
            .as_str(),
        TEST_TARGET
    );
    assert_eq!(decoded.payload().label, "typed-boundary");
    assert_eq!(decoded.contract().schema_version.value(), 1);
}

#[test]
fn stored_decode_contract_mismatch_reports_event_type_and_schema_version_context() {
    let live = EventEnvelope::from_event(
        EnvelopeBoundaryEvent {
            label: String::from("typed-boundary"),
        },
        metadata(),
    )
    .expect_value("live envelope builds");
    let mut stored = live.store().expect_value("stored envelope builds");
    stored.contract.event_type =
        EventType::parse(OTHER_EVENT_TYPE).expect_value("other event parses");
    stored.contract.schema_version =
        SchemaVersion::new(2).expect_value("received schema version parses");

    let error = stored
        .decode::<EnvelopeBoundaryEvent>()
        .expect_err_value("contract mismatch must fail closed");

    assert_eq!(
        error,
        EventingError::ContractMismatch {
            expected: EventType::parse(TEST_EVENT_TYPE).expect_value("expected event parses"),
            received: EventType::parse(OTHER_EVENT_TYPE).expect_value("received event parses"),
            expected_schema_version: SchemaVersion::new(1)
                .expect_value("expected schema version parses"),
            received_schema_version: SchemaVersion::new(2)
                .expect_value("received schema version parses"),
        }
    );
    assert_eq!(
        error.to_string(),
        "event contract mismatch: expected eventing.unit.contract-boundary@1, received eventing.unit.contract-boundary.other@2"
    );
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
