use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CausationId, CorrelationId, EventCustody, EventId, EventNamespace, EventType,
    IdempotencyKey, JournalHash, RecordedAt, RequestId, RuntimeInstanceId, RuntimeRole,
    SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
use serde::de::DeserializeOwned;

#[test]
fn event_type_and_namespace_reject_empty_or_malformed_taxonomy() {
    assert!(matches!(
        EventType::parse(" "),
        Err(EventingError::EmptyValue {
            field: "event_type"
        })
    ));
    assert!(matches!(
        EventType::parse(".tracking.location"),
        Err(EventingError::InvalidValue { .. })
    ));
    assert!(matches!(
        EventNamespace::parse("tracking..location"),
        Err(EventingError::InvalidValue { .. })
    ));
}

#[test]
fn event_namespace_matches_exact_and_child_event_types_only() {
    let namespace = EventNamespace::parse("tracking").expect_value("namespace parses");
    let exact = EventType::parse("tracking").expect_value("event type parses");
    let child = EventType::parse(format!("{}.{}", namespace.as_str(), "location.observed"))
        .expect_value("event type parses");
    let sibling = EventType::parse("tracking-location.observed").expect_value("event type parses");

    assert!(namespace.matches_event_type(&exact));
    assert!(namespace.matches_event_type(&child));
    assert!(!namespace.matches_event_type(&sibling));
}

#[test]
fn schema_version_rejects_zero_and_preserves_nonzero_value() {
    assert_eq!(SchemaVersion::new(0), Err(EventingError::InvalidVersion));
    assert_eq!(
        SchemaVersion::new(3)
            .expect_value("schema version parses")
            .value(),
        3
    );
    assert!(matches!(
        serde_json::from_str::<SchemaVersion>("0"),
        Err(error) if error.is_data()
    ));
    assert_eq!(
        serde_json::from_str::<SchemaVersion>("3")
            .expect_value("schema version deserializes")
            .value(),
        3
    );
}

#[test]
fn strong_identifier_wrappers_accept_existing_lineage_and_hash_values() {
    assert_eq!(
        EventId::parse("event-parity-1")
            .expect_value("event id parses")
            .as_str(),
        "event-parity-1"
    );
    assert_eq!(
        CorrelationId::parse("correlation-parity-1")
            .expect_value("correlation id parses")
            .as_str(),
        "correlation-parity-1"
    );
    assert_eq!(
        CausationId::parse("causation-test-1")
            .expect_value("causation id parses")
            .as_str(),
        "causation-test-1"
    );
    assert_eq!(
        RequestId::parse("request-a-0000")
            .expect_value("request id parses")
            .as_str(),
        "request-a-0000"
    );
    assert_eq!(
        JournalHash::parse("journal-hash-parity-1")
            .expect_value("journal hash parses")
            .as_str(),
        "journal-hash-parity-1"
    );
    assert_eq!(
        SubscriberId::parse("subscriber-parity-1")
            .expect_value("subscriber id parses")
            .as_str(),
        "subscriber-parity-1"
    );
}

#[test]
fn strong_identifier_wrappers_reject_whitespace_values() {
    assert_invalid(&EventId::parse(" event-parity-1"));
    assert_invalid(&CorrelationId::parse("correlation parity 1"));
    assert_invalid(&CausationId::parse("causation-test-1\t"));
    assert_invalid(&RequestId::parse("request-\nparity-1"));
    assert_invalid(&JournalHash::parse("journal hash parity 1"));
}

#[test]
fn event_type_taxonomy_rejects_unsupported_separators_and_whitespace() {
    for value in [
        "eventing observed",
        "eventing#observed",
        "eventing..observed",
    ] {
        assert!(matches!(
            EventType::parse(value),
            Err(EventingError::InvalidValue {
                field: "event_type",
                ..
            })
        ));
    }
}

#[test]
fn routing_and_source_wrappers_accept_existing_repo_values() {
    assert_eq!(
        SubscriberId::parse("subscriber.app.observer")
            .expect_value("dotted subscriber id parses")
            .as_str(),
        "subscriber.app.observer"
    );
    assert_eq!(
        SubscriberId::parse("subscriber.child-policy.evaluator")
            .expect_value("mixed dotted subscriber id parses")
            .as_str(),
        "subscriber.child-policy.evaluator"
    );
    assert_eq!(
        TargetHandler::parse("target.child-domain.observer")
            .expect_value("dotted target handler parses")
            .as_str(),
        "target.child-domain.observer"
    );
    assert_eq!(
        TargetHandler::parse("child-runtime.tracking")
            .expect_value("existing fixture target handler parses")
            .as_str(),
        "child-runtime.tracking"
    );
    assert_eq!(
        SourceService::parse("eventing-integration-service")
            .expect_value("source service parses")
            .as_str(),
        "eventing-integration-service"
    );
    assert_eq!(
        SourceComponent::parse("eventing-integration-component")
            .expect_value("source component parses")
            .as_str(),
        "eventing-integration-component"
    );
    assert_eq!(
        RuntimeInstanceId::parse("eventing-integration-runtime")
            .expect_value("runtime instance parses")
            .as_str(),
        "eventing-integration-runtime"
    );
}

#[test]
fn routing_and_source_wrappers_reject_whitespace_values() {
    assert_invalid(&SubscriberId::parse("subscriber app observer"));
    assert_invalid(&TargetHandler::parse("target.child-domain observer"));
    assert_invalid(&SourceService::parse("eventing integration service"));
    assert_invalid(&SourceComponent::parse("eventing-integration-component "));
    assert_invalid(&RuntimeInstanceId::parse("\teventing-integration-runtime"));
}

#[test]
fn runtime_boundary_wrappers_serde_roundtrip_and_reject_empty_values() {
    let aggregate = AggregateKey::parse("aggregate-parity-1").expect_value("aggregate parses");
    let idempotency =
        IdempotencyKey::parse("idempotency-parity-1").expect_value("idempotency parses");
    let custody = EventCustody::parse("local-only").expect_value("custody parses");
    let role = RuntimeRole::parse("parent").expect_value("runtime role parses");
    let recorded_at =
        RecordedAt::parse("2026-08-28T09:00:00Z").expect_value("recorded timestamp parses");

    assert_eq!(
        serde_json::to_string(&aggregate).expect_value("aggregate serializes"),
        "\"aggregate-parity-1\""
    );
    assert_eq!(
        serde_json::from_str::<AggregateKey>("\"aggregate-parity-1\"")
            .expect_value("aggregate deserializes"),
        aggregate
    );
    assert_eq!(
        serde_json::to_string(&idempotency).expect_value("idempotency serializes"),
        "\"idempotency-parity-1\""
    );
    assert_eq!(
        serde_json::from_str::<IdempotencyKey>("\"idempotency-parity-1\"")
            .expect_value("idempotency deserializes"),
        idempotency
    );
    assert_eq!(
        serde_json::from_str::<EventCustody>("\"local-only\"").expect_value("custody deserializes"),
        custody
    );
    assert_eq!(
        serde_json::from_str::<RuntimeRole>("\"parent\"").expect_value("runtime role deserializes"),
        role
    );
    assert_eq!(
        serde_json::from_str::<RecordedAt>("\"2026-08-28T09:00:00Z\"")
            .expect_value("recorded timestamp deserializes"),
        recorded_at
    );

    assert_serde_data_error::<AggregateKey>("\"\"");
    assert_serde_data_error::<IdempotencyKey>("\"\"");
    assert_serde_data_error::<EventCustody>("\"\"");
    assert_serde_data_error::<RuntimeRole>("\"\"");
    assert_serde_data_error::<RecordedAt>("\"not-a-timestamp\"");
}

fn assert_invalid<T>(result: &Result<T, EventingError>) {
    assert!(matches!(result, Err(EventingError::InvalidValue { .. })));
}

fn assert_serde_data_error<T: DeserializeOwned>(value: &str) {
    assert!(matches!(
        serde_json::from_str::<T>(value),
        Err(error) if error.is_data()
    ));
}
