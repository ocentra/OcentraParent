use super::{
    constants,
    parent_controller_events::{
        ParentActionReceivedEvent, ParentChildCommandForwardRequestedEvent,
        ParentChildCommandForwardedEvent, ParentCommandRejectedEvent, ParentCommandValidatedEvent,
        ParentControllerEventContract, ParentReadModelProjectedEvent,
    },
};

#[path = "parent_controller_event_fixtures.rs"]
mod parent_controller_event_fixtures;

use parent_controller_event_fixtures::{
    parent_action_received_event, parent_child_command_forward_requested_event,
    parent_child_command_forwarded_event, parent_command_rejected_event,
    parent_command_validated_event, parent_read_model_projected_event,
};

#[test]
fn parent_and_child_event_namespace_constants_are_unique_and_prefixed() {
    let parent_events = [
        constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED,
        constants::parent_controller::EVENT_COMMAND_VALIDATED,
        constants::parent_controller::EVENT_COMMAND_REJECTED,
        constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED,
        constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED,
        constants::parent_controller::EVENT_READ_MODEL_PROJECTED,
    ];
    let child_events = [
        constants::child_agent::EVENT_COMMAND_RECEIVED,
        constants::child_agent::EVENT_COMMAND_ACCEPTED,
        constants::child_agent::EVENT_COMMAND_REJECTED,
        constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED,
        constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED,
    ];

    assert_namespace(&parent_events, constants::parent_controller::NAMESPACE);
    assert_namespace(&child_events, constants::child_agent::NAMESPACE);
    assert_unique(&parent_events);
    assert_unique(&child_events);

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
        serialized_field(&parent_action_received_event(), "actionKind", ""),
        serde_json::json!("block")
    );
    assert_eq!(
        serialized_field(&parent_action_received_event(), "source", ""),
        serde_json::json!("portal-typed-intent")
    );
    assert_eq!(
        serialized_field(&parent_command_validated_event(), "childCommandRef", ""),
        constants::parent_controller::TEST_CHILD_COMMAND_REF
    );
    assert_eq!(
        serialized_field(&parent_command_rejected_event(), "rejectionReasonCode", ""),
        constants::parent_controller::TEST_VALIDATION_REJECTION_CODE
    );
    assert_eq!(
        serialized_field(
            &parent_child_command_forward_requested_event(),
            "transportBoundary",
            ""
        ),
        serde_json::json!("typed-local-service-transport")
    );
    assert_eq!(
        serialized_field(
            &parent_child_command_forwarded_event(),
            "transportMessageRef",
            ""
        ),
        constants::parent_controller::TEST_TRANSPORT_MESSAGE_REF
    );
    assert_eq!(
        serialized_field(&parent_read_model_projected_event(), "visibleToPortal", ""),
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
        parsed.unwrap_err().classify(),
        serde_json::error::Category::Data
    );
}

fn assert_namespace(events: &[&str], namespace: &str) {
    for event in events {
        assert!(event.starts_with(namespace));
    }
}

fn assert_unique(events: &[&str]) {
    for (index, event) in events.iter().enumerate() {
        assert_eq!(
            events[..index]
                .iter()
                .filter(|candidate| *candidate == event)
                .count(),
            0
        );
    }
}

fn serialized_field<T>(value: &T, field: &str, nested: &str) -> serde_json::Value
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_value(value).unwrap_or_else(|error| {
        unreachable!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
    });
    if nested.is_empty() {
        return serialized[field].clone();
    }
    serialized[field][nested].clone()
}
