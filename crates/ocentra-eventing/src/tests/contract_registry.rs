use crate::{
    tests::fixtures::{test_event, test_event_for_type, OTHER_EVENT_TYPE, TEST_EVENT_TYPE},
    EventContractRegistry, EventType, EventingError,
};

#[test]
fn contract_registry_generates_markdown_in_event_type_order() {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event_for_type("second", OTHER_EVENT_TYPE))
        .expect("other event registers");
    registry
        .register_event(&test_event("first"))
        .expect("test event registers");

    let descriptors = registry
        .descriptors()
        .map(|descriptor| descriptor.event_type().as_str().to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        descriptors,
        vec![TEST_EVENT_TYPE.to_string(), OTHER_EVENT_TYPE.to_string()]
    );

    let markdown = registry.render_markdown().into_string();
    assert!(markdown.starts_with("# Event Contract Registry"));
    assert!(markdown.contains("| Event Type | Schema Version | Rust Type |"));
    assert!(markdown.contains("| eventing.test.observed | 1 |"));
    assert!(markdown.contains("| eventing.test.other | 1 |"));
    assert!(markdown.contains("ocentra_eventing::tests::fixtures::TestEvent"));

    let observed_index = markdown
        .find(TEST_EVENT_TYPE)
        .expect("observed event appears in markdown");
    let other_index = markdown
        .find(OTHER_EVENT_TYPE)
        .expect("other event appears in markdown");
    assert!(observed_index < other_index);
}

#[test]
fn contract_registry_rejects_duplicate_event_type() {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event("first"))
        .expect("first event registers");

    let duplicate = registry
        .register_event(&test_event("duplicate"))
        .expect_err("duplicate event type rejects");
    assert_eq!(
        duplicate,
        EventingError::DuplicateEventContract {
            event_type: EventType::parse(TEST_EVENT_TYPE).expect("test event type parses")
        }
    );
}

#[test]
fn empty_contract_registry_docs_are_explicit() {
    let markdown = EventContractRegistry::new().render_markdown();

    assert_eq!(
        markdown.as_str(),
        "# Event Contract Registry\n\n_No event contracts registered._\n"
    );
}
