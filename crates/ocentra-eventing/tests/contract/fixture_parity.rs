use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CausationId, CorrelationId, EventCustody, EventId, EventNamespace, EventType,
    IdempotencyKey, JournalHash, RecordedAt, RequestId, RuntimeInstanceId, RuntimeRole,
    SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
use serde::Deserialize;

const PARITY_FIXTURE: &str = include_str!("../../fixtures/branded_scalar_parity.json");
const EVENT_TYPE_FIELD: &str = "eventType";
const EVENT_NAMESPACE_FIELD: &str = "eventNamespace";
const EVENT_ID_FIELD: &str = "eventId";
const CORRELATION_ID_FIELD: &str = "correlationId";
const CAUSATION_ID_FIELD: &str = "causationId";
const REQUEST_ID_FIELD: &str = "requestId";
const JOURNAL_HASH_FIELD: &str = "journalHash";
const AGGREGATE_KEY_FIELD: &str = "aggregateKey";
const IDEMPOTENCY_KEY_FIELD: &str = "idempotencyKey";
const SUBSCRIBER_ID_FIELD: &str = "subscriberId";
const TARGET_HANDLER_FIELD: &str = "targetHandler";
const EVENT_CUSTODY_FIELD: &str = "eventCustody";
const RUNTIME_ROLE_FIELD: &str = "runtimeRole";
const SOURCE_SERVICE_FIELD: &str = "sourceService";
const SOURCE_COMPONENT_FIELD: &str = "sourceComponent";
const RUNTIME_INSTANCE_ID_FIELD: &str = "runtimeInstanceId";
const RECORDED_AT_FIELD: &str = "recordedAt";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ParityFixture {
    valid: ValidScalars,
    invalid_text: Vec<InvalidTextScalar>,
    invalid_schema_versions: Vec<u16>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValidScalars {
    event_type: String,
    event_namespace: String,
    event_id: String,
    correlation_id: String,
    causation_id: String,
    request_id: String,
    journal_hash: String,
    aggregate_key: String,
    idempotency_key: String,
    subscriber_id: String,
    target_handler: String,
    event_custody: String,
    runtime_role: String,
    source_service: String,
    source_component: String,
    runtime_instance_id: String,
    recorded_at: String,
    schema_version: u16,
}

#[derive(Deserialize)]
struct InvalidTextScalar {
    field: String,
    value: String,
}

#[test]
fn rust_newtypes_accept_shared_valid_parity_fixture() {
    let valid = fixture().valid;

    assert_eq!(
        EventType::parse(valid.event_type)
            .expect_value("event type parses")
            .as_str(),
        "network.domain.observed"
    );
    assert_eq!(
        EventNamespace::parse(valid.event_namespace)
            .expect_value("event namespace parses")
            .as_str(),
        "network"
    );
    EventId::parse(valid.event_id).expect_value("event id parses");
    CorrelationId::parse(valid.correlation_id).expect_value("correlation id parses");
    CausationId::parse(valid.causation_id).expect_value("causation id parses");
    RequestId::parse(valid.request_id).expect_value("request id parses");
    JournalHash::parse(valid.journal_hash).expect_value("journal hash parses");
    AggregateKey::parse(valid.aggregate_key).expect_value("aggregate key parses");
    IdempotencyKey::parse(valid.idempotency_key).expect_value("idempotency key parses");
    SubscriberId::parse(valid.subscriber_id).expect_value("subscriber id parses");
    TargetHandler::parse(valid.target_handler).expect_value("target handler parses");
    EventCustody::parse(valid.event_custody).expect_value("event custody parses");
    RuntimeRole::parse(valid.runtime_role).expect_value("runtime role parses");
    SourceService::parse(valid.source_service).expect_value("source service parses");
    SourceComponent::parse(valid.source_component).expect_value("source component parses");
    RuntimeInstanceId::parse(valid.runtime_instance_id).expect_value("runtime instance parses");
    RecordedAt::parse(valid.recorded_at).expect_value("recorded at parses");
    assert_eq!(
        SchemaVersion::new(valid.schema_version)
            .expect_value("schema version parses")
            .value(),
        1
    );
}

#[test]
fn rust_newtypes_reject_shared_invalid_text_fixture_values() {
    for invalid in fixture().invalid_text {
        let field = invalid.field;
        let value = invalid.value;
        let field = FixtureField(field);
        let rejected = invalid_text_rejects(&field, FixtureText(value));
        assert!(rejected, "expected Rust newtype rejection for {}", field.0);
    }
}

#[test]
fn rust_schema_version_rejects_shared_invalid_versions() {
    for value in fixture().invalid_schema_versions {
        assert_eq!(
            SchemaVersion::new(value),
            Err(EventingError::InvalidVersion)
        );
    }
}

fn fixture() -> ParityFixture {
    serde_json::from_str(PARITY_FIXTURE).expect_value("parity fixture parses")
}

#[derive(Clone)]
struct FixtureField(String);

#[derive(Clone)]
struct FixtureText(String);

type TextParser = fn(FixtureText) -> Result<(), EventingError>;

fn invalid_text_rejects(field: &FixtureField, value: FixtureText) -> bool {
    invalid_text_checkers()
        .iter()
        .find(|(candidate, _)| candidate.0.as_str() == field.0.as_str())
        .is_some_and(|(_, parser)| parser(value).is_err())
}

fn invalid_text_checkers() -> [(FixtureField, TextParser); 17] {
    [
        (FixtureField(EVENT_TYPE_FIELD.to_owned()), parse_event_type),
        (
            FixtureField(EVENT_NAMESPACE_FIELD.to_owned()),
            parse_event_namespace,
        ),
        (FixtureField(EVENT_ID_FIELD.to_owned()), parse_event_id),
        (
            FixtureField(CORRELATION_ID_FIELD.to_owned()),
            parse_correlation_id,
        ),
        (
            FixtureField(CAUSATION_ID_FIELD.to_owned()),
            parse_causation_id,
        ),
        (FixtureField(REQUEST_ID_FIELD.to_owned()), parse_request_id),
        (
            FixtureField(JOURNAL_HASH_FIELD.to_owned()),
            parse_journal_hash,
        ),
        (
            FixtureField(AGGREGATE_KEY_FIELD.to_owned()),
            parse_aggregate_key,
        ),
        (
            FixtureField(IDEMPOTENCY_KEY_FIELD.to_owned()),
            parse_idempotency_key,
        ),
        (
            FixtureField(SUBSCRIBER_ID_FIELD.to_owned()),
            parse_subscriber_id,
        ),
        (
            FixtureField(TARGET_HANDLER_FIELD.to_owned()),
            parse_target_handler,
        ),
        (
            FixtureField(EVENT_CUSTODY_FIELD.to_owned()),
            parse_event_custody,
        ),
        (
            FixtureField(RUNTIME_ROLE_FIELD.to_owned()),
            parse_runtime_role,
        ),
        (
            FixtureField(SOURCE_SERVICE_FIELD.to_owned()),
            parse_source_service,
        ),
        (
            FixtureField(SOURCE_COMPONENT_FIELD.to_owned()),
            parse_source_component,
        ),
        (
            FixtureField(RUNTIME_INSTANCE_ID_FIELD.to_owned()),
            parse_runtime_instance_id,
        ),
        (
            FixtureField(RECORDED_AT_FIELD.to_owned()),
            parse_recorded_at,
        ),
    ]
}

macro_rules! text_parser {
    ($name:ident, $ty:ty) => {
        fn $name(value: FixtureText) -> Result<(), EventingError> {
            <$ty>::parse(value.0.as_str()).map(|_| ())
        }
    };
}

text_parser!(parse_event_type, EventType);
text_parser!(parse_event_namespace, EventNamespace);
text_parser!(parse_event_id, EventId);
text_parser!(parse_correlation_id, CorrelationId);
text_parser!(parse_causation_id, CausationId);
text_parser!(parse_request_id, RequestId);
text_parser!(parse_journal_hash, JournalHash);
text_parser!(parse_aggregate_key, AggregateKey);
text_parser!(parse_idempotency_key, IdempotencyKey);
text_parser!(parse_subscriber_id, SubscriberId);
text_parser!(parse_target_handler, TargetHandler);
text_parser!(parse_event_custody, EventCustody);
text_parser!(parse_runtime_role, RuntimeRole);
text_parser!(parse_source_service, SourceService);
text_parser!(parse_source_component, SourceComponent);
text_parser!(parse_runtime_instance_id, RuntimeInstanceId);
text_parser!(parse_recorded_at, RecordedAt);
