use super::{
    constants,
    parent_controller_events::{
        ParentActionReceivedEvent, ParentChildCommandForwardRequestedEvent,
        ParentChildCommandForwardedEvent, ParentCommandRejectedEvent, ParentCommandValidatedEvent,
        ParentControllerEventContract, ParentReadModelProjectedEvent,
    },
};
use ocentra_eventing::expect_value::ExpectValue;

#[path = "parent_controller_event_fixtures.rs"]
mod parent_controller_event_fixtures;

use parent_controller_event_fixtures::{
    parent_action_received_event, parent_child_command_forward_requested_event,
    parent_child_command_forwarded_event, parent_command_rejected_event,
    parent_command_validated_event, parent_read_model_projected_event,
};

macro_rules! serialized_field {
    ($value:expr, $field:expr) => {{
        let serialized =
            serde_json::to_value($value).expect(constants::error::AGENT_EVENT_SERIALIZES);
        serialized[$field].clone()
    }};
    ($value:expr, $field:expr, $nested:expr) => {{
        let serialized =
            serde_json::to_value($value).expect(constants::error::AGENT_EVENT_SERIALIZES);
        serialized[$field][$nested].clone()
    }};
}

macro_rules! assert_namespace {
    ($events:expr, $namespace:expr) => {{
        for event in $events {
            assert!(event.starts_with($namespace));
        }
    }};
}

macro_rules! assert_unique {
    ($events:expr) => {{
        for (index, event) in $events.iter().enumerate() {
            assert_eq!(
                $events[..index]
                    .iter()
                    .filter(|candidate| *candidate == event)
                    .count(),
                0
            );
        }
    }};
}

#[test]
fn parent_and_child_event_namespace_constants_are_unique_and_prefixed() {
    let parent_events = constants::parent_controller::EVENT_TYPES;
    let child_events = constants::child_agent::EVENT_TYPES;

    assert_namespace!(&parent_events, constants::parent_controller::NAMESPACE);
    assert_namespace!(&child_events, constants::child_agent::NAMESPACE);
    assert_unique!(&parent_events);
    assert_unique!(&child_events);

    for event in parent_events {
        assert!(child_events.iter().all(|child_event| *child_event != event));
    }
}

#[test]
fn parent_controller_contracts_name_exact_event_types() {
    assert_eq!(
        ParentActionReceivedEvent::EVENT_TYPE,
        constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED
    );
    assert_eq!(
        ParentCommandValidatedEvent::EVENT_TYPE,
        constants::parent_controller::EVENT_COMMAND_VALIDATED
    );
    assert_eq!(
        ParentCommandRejectedEvent::EVENT_TYPE,
        constants::parent_controller::EVENT_COMMAND_REJECTED
    );
    assert_eq!(
        ParentChildCommandForwardRequestedEvent::EVENT_TYPE,
        constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED
    );
    assert_eq!(
        ParentChildCommandForwardedEvent::EVENT_TYPE,
        constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED
    );
    assert_eq!(
        ParentReadModelProjectedEvent::EVENT_TYPE,
        constants::parent_controller::EVENT_READ_MODEL_PROJECTED
    );
}

#[test]
fn parent_controller_contracts_serialize_validation_and_transport_refs() {
    assert_eq!(
        serialized_field!(&parent_action_received_event(), "actionKind"),
        serde_json::json!("block")
    );
    assert_eq!(
        serialized_field!(&parent_action_received_event(), "source"),
        serde_json::json!("portal-typed-intent")
    );
    assert_eq!(
        serialized_field!(&parent_command_validated_event(), "childCommandRef"),
        constants::parent_controller::TEST_CHILD_COMMAND_REF
    );
    assert_eq!(
        serialized_field!(&parent_command_rejected_event(), "rejectionReasonCode"),
        constants::parent_controller::TEST_VALIDATION_REJECTION_CODE
    );
    assert_eq!(
        serialized_field!(
            &parent_child_command_forward_requested_event(),
            "transportBoundary"
        ),
        serde_json::json!("typed-local-service-transport")
    );
    assert_eq!(
        serialized_field!(
            &parent_child_command_forwarded_event(),
            "transportMessageRef"
        ),
        constants::parent_controller::TEST_TRANSPORT_MESSAGE_REF
    );
    assert_eq!(
        serialized_field!(&parent_read_model_projected_event(), "visibleToPortal"),
        true
    );
}

#[test]
fn parent_action_contract_rejects_missing_parent_intent_ref() {
    let event = serde_json::json!({
        "schemaVersion": constants::parent_controller::EVENT_SCHEMA_VERSION,
        "parentActionEventRef": constants::parent_controller::TEST_PARENT_ACTION_EVENT_REF,
        "receivedAt": constants::activity_store::TEST_FIRST_OBSERVED_AT,
        "parentProfileRef": constants::parent_controller::TEST_PARENT_PROFILE_REF,
        "deviceRef": constants::parent_controller::TEST_DEVICE_REF,
        "actionKind": "block",
        "source": "portal-typed-intent",
        "custody": constants::parent_controller::CUSTODY_LOCAL_SERVICE_VALIDATION,
        "idempotencyKey": constants::parent_controller::TEST_IDEMPOTENCY_KEY
    });

    let parsed = serde_json::from_value::<ParentActionReceivedEvent>(event);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}

#[test]
fn parent_controller_contracts_validate_schema_and_owned_refs() {
    let event = parent_action_received_event();
    assert_eq!(event.validate(), Ok(()));

    let mut schema_skew = event.clone();
    schema_skew.schema_version = constants::parent_controller::EVENT_SCHEMA_VERSION + 1;
    assert_eq!(
        schema_skew.validate(),
        Err(ocentra_eventing::error::EventingError::InvalidVersion)
    );

    let mut blank_ref = event;
    blank_ref.parent_intent_ref = "  ".to_string();
    assert_eq!(
        blank_ref.validate(),
        Err(ocentra_eventing::error::EventingError::EmptyValue {
            field: "parent_intent_ref"
        })
    );
}

#[test]
fn parent_controller_contracts_reject_schema_skew_blank_text_and_unknown_fields() {
    let valid = serde_json::to_value(parent_action_received_event())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    let mut schema_skew = valid.clone();
    schema_skew["schemaVersion"] =
        serde_json::json!(constants::parent_controller::EVENT_SCHEMA_VERSION + 1);
    let mut blank_ref = valid.clone();
    blank_ref["parentIntentRef"] = serde_json::json!(" ");
    let mut unknown_field = valid;
    unknown_field["futureField"] = serde_json::json!(true);

    for invalid in [schema_skew, blank_ref, unknown_field] {
        assert_eq!(
            serde_json::from_value::<ParentActionReceivedEvent>(invalid)
                .err()
                .map(|error| error.classify()),
            Some(serde_json::error::Category::Data)
        );
    }
}
