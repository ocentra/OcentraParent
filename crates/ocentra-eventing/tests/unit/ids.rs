use std::error::Error;

use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    CausationId, CorrelationId, EventId, EventNamespace, EventType, JournalHash, RequestId,
    RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};

#[test]
fn event_type_and_namespace_reject_empty_or_malformed_taxonomy() -> Result<(), Box<dyn Error>> {
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
    Ok(())
}

#[test]
fn event_namespace_matches_exact_and_child_event_types_only() -> Result<(), Box<dyn Error>> {
    let namespace = EventNamespace::parse("tracking")?;
    let exact = EventType::parse("tracking")?;
    let child = EventType::parse("tracking.location.observed")?;
    let sibling = EventType::parse("tracking-location.observed")?;

    assert!(namespace.matches_event_type(&exact));
    assert!(namespace.matches_event_type(&child));
    assert!(!namespace.matches_event_type(&sibling));
    Ok(())
}

#[test]
fn schema_version_rejects_zero_and_preserves_nonzero_value() -> Result<(), Box<dyn Error>> {
    assert_eq!(SchemaVersion::new(0), Err(EventingError::InvalidVersion));
    assert_eq!(
        SchemaVersion::new(3)?.value(),
        3
    );
    assert!(serde_json::from_str::<SchemaVersion>("0").is_err());
    assert_eq!(
        serde_json::from_str::<SchemaVersion>("3")?.value(),
        3
    );
    Ok(())
}

#[test]
fn strong_identifier_wrappers_accept_existing_lineage_and_hash_values()
    -> Result<(), Box<dyn Error>>
{
    assert_eq!(
        EventId::parse("event-parity-1")?.as_str(),
        "event-parity-1"
    );
    assert_eq!(
        CorrelationId::parse("correlation-parity-1")?.as_str(),
        "correlation-parity-1"
    );
    assert_eq!(
        CausationId::parse("causation-test-1")?.as_str(),
        "causation-test-1"
    );
    assert_eq!(
        RequestId::parse("request-a-0000")?.as_str(),
        "request-a-0000"
    );
    assert_eq!(
        JournalHash::parse("journal-hash-parity-1")?.as_str(),
        "journal-hash-parity-1"
    );
    assert_eq!(
        SubscriberId::parse("subscriber-parity-1")?.as_str(),
        "subscriber-parity-1"
    );
    Ok(())
}

#[test]
fn strong_identifier_wrappers_reject_whitespace_values() -> Result<(), Box<dyn Error>> {
    assert_invalid(&EventId::parse(" event-parity-1"));
    assert_invalid(&CorrelationId::parse("correlation parity 1"));
    assert_invalid(&CausationId::parse("causation-test-1\t"));
    assert_invalid(&RequestId::parse("request-\nparity-1"));
    assert_invalid(&JournalHash::parse("journal hash parity 1"));
    Ok(())
}

#[test]
fn routing_and_source_wrappers_accept_existing_repo_values() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        SubscriberId::parse("subscriber.app.observer")?.as_str(),
        "subscriber.app.observer"
    );
    assert_eq!(
        SubscriberId::parse("subscriber.child-policy.evaluator")?.as_str(),
        "subscriber.child-policy.evaluator"
    );
    assert_eq!(
        TargetHandler::parse("target.child-domain.observer")?.as_str(),
        "target.child-domain.observer"
    );
    assert_eq!(
        TargetHandler::parse("child-runtime.tracking")?.as_str(),
        "child-runtime.tracking"
    );
    assert_eq!(
        SourceService::parse("eventing-integration-service")?.as_str(),
        "eventing-integration-service"
    );
    assert_eq!(
        SourceComponent::parse("eventing-integration-component")?.as_str(),
        "eventing-integration-component"
    );
    assert_eq!(
        RuntimeInstanceId::parse("eventing-integration-runtime")?.as_str(),
        "eventing-integration-runtime"
    );
    Ok(())
}

#[test]
fn routing_and_source_wrappers_reject_whitespace_values() -> Result<(), Box<dyn Error>> {
    assert_invalid(&SubscriberId::parse("subscriber app observer"));
    assert_invalid(&TargetHandler::parse("target.child-domain observer"));
    assert_invalid(&SourceService::parse("eventing integration service"));
    assert_invalid(&SourceComponent::parse("eventing-integration-component "));
    assert_invalid(&RuntimeInstanceId::parse("\teventing-integration-runtime"));
    Ok(())
}

fn assert_invalid<T>(result: &Result<T, EventingError>) {
    assert!(matches!(result, Err(EventingError::InvalidValue { .. })));
}
