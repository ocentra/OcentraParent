use ocentra_eventing::{EventNamespace, EventType, EventingError, SchemaVersion};

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
    let namespace = EventNamespace::parse("tracking").expect("namespace parses");
    let exact = EventType::parse("tracking").expect("event type parses");
    let child = EventType::parse("tracking.location.observed").expect("event type parses");
    let sibling = EventType::parse("tracking-location.observed").expect("event type parses");

    assert!(namespace.matches_event_type(&exact));
    assert!(namespace.matches_event_type(&child));
    assert!(!namespace.matches_event_type(&sibling));
}

#[test]
fn schema_version_rejects_zero_and_preserves_nonzero_value() {
    assert_eq!(SchemaVersion::new(0), Err(EventingError::InvalidVersion));
    assert_eq!(SchemaVersion::new(3).expect("schema version parses").value(), 3);
}
