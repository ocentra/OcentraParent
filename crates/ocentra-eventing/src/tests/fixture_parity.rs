use serde::Deserialize;
use std::error::Error;

use crate::ids::{
    AggregateKey, CorrelationId, EventId, EventNamespace, EventType, IdempotencyKey, JournalHash,
    RecordedAt, RequestId, RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService,
    SubscriberId, TargetHandler,
};

const PARITY_FIXTURE: &str = include_str!("../../fixtures/branded_scalar_parity.json");
const EVENT_TYPE_FIELD: &str = "eventType";
const EVENT_NAMESPACE_FIELD: &str = "eventNamespace";
const EVENT_ID_FIELD: &str = "eventId";
const CORRELATION_ID_FIELD: &str = "correlationId";
const REQUEST_ID_FIELD: &str = "requestId";
const JOURNAL_HASH_FIELD: &str = "journalHash";
const AGGREGATE_KEY_FIELD: &str = "aggregateKey";
const IDEMPOTENCY_KEY_FIELD: &str = "idempotencyKey";
const SUBSCRIBER_ID_FIELD: &str = "subscriberId";
const TARGET_HANDLER_FIELD: &str = "targetHandler";
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
    request_id: String,
    journal_hash: String,
    aggregate_key: String,
    idempotency_key: String,
    subscriber_id: String,
    target_handler: String,
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
fn rust_newtypes_accept_shared_valid_parity_fixture() -> Result<(), Box<dyn Error>> {
    let valid = fixture()?.valid;

    assert_eq!(
        EventType::parse(valid.event_type)?.as_str(),
        "network.domain.observed"
    );
    assert_eq!(
        EventNamespace::parse(valid.event_namespace)?.as_str(),
        "network"
    );
    EventId::parse(valid.event_id)?;
    CorrelationId::parse(valid.correlation_id)?;
    RequestId::parse(valid.request_id)?;
    JournalHash::parse(valid.journal_hash)?;
    AggregateKey::parse(valid.aggregate_key)?;
    IdempotencyKey::parse(valid.idempotency_key)?;
    SubscriberId::parse(valid.subscriber_id)?;
    TargetHandler::parse(valid.target_handler)?;
    SourceService::parse(valid.source_service)?;
    SourceComponent::parse(valid.source_component)?;
    RuntimeInstanceId::parse(valid.runtime_instance_id)?;
    RecordedAt::parse(valid.recorded_at)?;
    assert_eq!(
        SchemaVersion::new(valid.schema_version)?.value(),
        1
    );
    Ok(())
}

#[test]
fn rust_newtypes_reject_shared_invalid_text_fixture_values() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;

    for invalid in fixture.invalid_text {
        assert!(
            rejects_text_scalar(&invalid.field, invalid.value),
            "expected Rust newtype rejection for {}",
            invalid.field
        );
    }
    Ok(())
}

#[test]
fn rust_schema_version_rejects_shared_invalid_versions() -> Result<(), Box<dyn Error>> {
    let fixture = fixture()?;

    for value in fixture.invalid_schema_versions {
        assert!(SchemaVersion::new(value).is_err());
    }
    Ok(())
}

fn fixture() -> Result<ParityFixture, Box<dyn Error>> {
    Ok(serde_json::from_str(PARITY_FIXTURE)?)
}

fn rejects_text_scalar(field: &str, value: String) -> bool {
    match field {
        EVENT_TYPE_FIELD => EventType::parse(value).is_err(),
        EVENT_NAMESPACE_FIELD => EventNamespace::parse(value).is_err(),
        EVENT_ID_FIELD => EventId::parse(value).is_err(),
        CORRELATION_ID_FIELD => CorrelationId::parse(value).is_err(),
        REQUEST_ID_FIELD => RequestId::parse(value).is_err(),
        JOURNAL_HASH_FIELD => JournalHash::parse(value).is_err(),
        AGGREGATE_KEY_FIELD => AggregateKey::parse(value).is_err(),
        IDEMPOTENCY_KEY_FIELD => IdempotencyKey::parse(value).is_err(),
        SUBSCRIBER_ID_FIELD => SubscriberId::parse(value).is_err(),
        TARGET_HANDLER_FIELD => TargetHandler::parse(value).is_err(),
        SOURCE_SERVICE_FIELD => SourceService::parse(value).is_err(),
        SOURCE_COMPONENT_FIELD => SourceComponent::parse(value).is_err(),
        RUNTIME_INSTANCE_ID_FIELD => RuntimeInstanceId::parse(value).is_err(),
        RECORDED_AT_FIELD => RecordedAt::parse(value).is_err(),
        _ => false,
    }
}
