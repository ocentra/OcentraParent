use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    CausationId, CorrelationId, EventId, EventNamespace, EventType, JournalHash, RequestId,
    RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};

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

fn assert_invalid<T>(result: &Result<T, EventingError>) {
    assert!(matches!(result, Err(EventingError::InvalidValue { .. })));
}
