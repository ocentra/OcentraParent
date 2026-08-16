use super::support::{
    test_event, test_event_for_type, TestText as SupportText, OTHER_EVENT_TYPE, TEST_EVENT_TYPE,
};
use ocentra_eventing::contract_registry::EventContractRegistry;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::EventType;

#[test]
fn contract_registry_generates_markdown_in_event_type_order() {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event_for_type(
            SupportText("second".to_owned()),
            SupportText(OTHER_EVENT_TYPE.to_owned()),
        ))
        .expect_value("other event registers");
    registry
        .register_event(&test_event(SupportText("first".to_owned())))
        .expect_value("test event registers");

    let descriptors = registry
        .descriptors()
        .map(|descriptor| descriptor.event_type().as_str().to_string())
        .collect::<Vec<_>>();
    assert_eq!(
        descriptors,
        vec![TEST_EVENT_TYPE.to_string(), OTHER_EVENT_TYPE.to_string()]
    );

    let markdown = registry.render_markdown().into_string();
    let lines = markdown.lines().collect::<Vec<_>>();
    assert_eq!(lines.first().copied(), Some("# Event Contract Registry"));
    assert_eq!(
        lines.get(2).copied(),
        Some("| Event Type | Schema Version | Rust Type |")
    );
    assert!(lines
        .iter()
        .any(|line| line.starts_with("| eventing.test.observed | 1 |")));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("| eventing.test.other | 1 |")));

    let observed_index = markdown
        .find(TEST_EVENT_TYPE)
        .expect_value("observed event appears in markdown");
    let other_index = markdown
        .find(OTHER_EVENT_TYPE)
        .expect_value("other event appears in markdown");
    assert!(observed_index < other_index);
}

#[test]
fn contract_registry_rejects_duplicate_event_type() {
    let mut registry = EventContractRegistry::new();
    registry
        .register_event(&test_event(SupportText("first".to_owned())))
        .expect_value("first event registers");

    let duplicate = match registry.register_event(&test_event(SupportText("duplicate".to_owned())))
    {
        Ok(_) => std::process::abort(),
        Err(error) => error,
    };
    assert_eq!(
        duplicate,
        EventingError::DuplicateEventContract {
            event_type: EventType::parse(TEST_EVENT_TYPE).expect_value("test event type parses")
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
