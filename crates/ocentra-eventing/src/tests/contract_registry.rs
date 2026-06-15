use crate::contract_registry::EventContractRegistry;
use crate::error::EventingError;
use crate::ids::EventType;
use crate::tests::fixtures::{test_event, test_event_for_type, OTHER_EVENT_TYPE, TEST_EVENT_TYPE};
use std::error::Error;

#[test]
fn contract_registry_generates_markdown_in_event_type_order() -> Result<(), Box<dyn Error>> {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event_for_type("second", OTHER_EVENT_TYPE))
        ?;
    registry
        .register_event(&test_event("first"))
        ?;

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

    let observed_index = markdown.find(TEST_EVENT_TYPE).ok_or_else(|| {
        std::io::Error::other(format!("markdown missing event type {TEST_EVENT_TYPE}"))
    })?;
    let other_index = markdown.find(OTHER_EVENT_TYPE).ok_or_else(|| {
        std::io::Error::other(format!("markdown missing event type {OTHER_EVENT_TYPE}"))
    })?;
    assert!(observed_index < other_index);
    Ok(())
}

#[test]
fn contract_registry_rejects_duplicate_event_type() -> Result<(), Box<dyn Error>> {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event("first"))
        ?;

    let duplicate = match registry.register_event(&test_event("duplicate")) {
        Ok(_) => {
            return Err(std::io::Error::other("expected duplicate event type rejection").into());
        }
        Err(error) => error,
    };
    assert_eq!(
        duplicate,
        EventingError::DuplicateEventContract {
            event_type: EventType::parse(TEST_EVENT_TYPE)?
        }
    );
    Ok(())
}

#[test]
fn empty_contract_registry_docs_are_explicit() -> Result<(), Box<dyn Error>> {
    let markdown = EventContractRegistry::new().render_markdown();

    assert_eq!(
        markdown.as_str(),
        "# Event Contract Registry\n\n_No event contracts registered._\n"
    );
    Ok(())
}
