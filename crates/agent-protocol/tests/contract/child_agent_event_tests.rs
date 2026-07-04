use super::constants;
use crate::child_agent::child_agent_events::{
    ChildAgentEventContract, ChildCapabilityStateUpdatedEvent, ChildCommandAcceptedEvent,
    ChildCommandKind, ChildCommandReceivedEvent, ChildCommandRejectedEvent,
    ChildRuntimeHealthUpdatedEvent,
};

#[path = "child_agent_event_fixtures.rs"]
mod child_agent_event_fixtures;

use child_agent_event_fixtures::{
    child_capability_state_updated_event, child_command_accepted_event,
    child_command_received_event, child_command_rejected_event, child_runtime_health_updated_event,
};

macro_rules! serialized_field {
    ($value:expr, $field:literal) => {{
        let serialized =
            serde_json::to_value($value).expect(constants::error::AGENT_EVENT_SERIALIZES);
        serialized[$field].clone()
    }};
    ($value:expr, $field:literal, $nested:literal) => {{
        let serialized =
            serde_json::to_value($value).expect(constants::error::AGENT_EVENT_SERIALIZES);
        serialized[$field][$nested].clone()
    }};
}

#[test]
fn child_agent_contracts_name_exact_event_types() {
    assert_eq!(
        ChildCommandReceivedEvent::EVENT_TYPE,
        constants::child_agent::EVENT_COMMAND_RECEIVED
    );
    assert_eq!(
        ChildCommandAcceptedEvent::EVENT_TYPE,
        constants::child_agent::EVENT_COMMAND_ACCEPTED
    );
    assert_eq!(
        ChildCommandRejectedEvent::EVENT_TYPE,
        constants::child_agent::EVENT_COMMAND_REJECTED
    );
    assert_eq!(
        ChildCapabilityStateUpdatedEvent::EVENT_TYPE,
        constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED
    );
    assert_eq!(
        ChildRuntimeHealthUpdatedEvent::EVENT_TYPE,
        constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED
    );
}

#[test]
fn child_agent_contracts_serialize_command_and_runtime_refs() {
    assert_eq!(
        serialized_field!(&child_command_received_event(), "commandKind"),
        serde_json::json!("apply-policy")
    );
    assert_eq!(
        serialized_field!(&child_command_received_event(), "parentControllerEventRef"),
        constants::child_agent::TEST_PARENT_CONTROLLER_EVENT_REF
    );
    assert_eq!(
        serialized_field!(&child_command_accepted_event(), "decision"),
        serde_json::json!("manual-required")
    );
    assert_eq!(
        serialized_field!(&child_command_rejected_event(), "rejectionReasonCode"),
        constants::child_agent::TEST_REJECTION_CODE
    );
    assert_eq!(
        serialized_field!(&child_capability_state_updated_event(), "capabilityState"),
        serde_json::json!("manual-required")
    );
    assert_eq!(
        serialized_field!(&child_runtime_health_updated_event(), "healthState"),
        serde_json::json!("degraded")
    );
}

#[test]
fn child_agent_contracts_serialize_browser_action_intent_handoff_kind() {
    let mut event = child_command_received_event();
    event.command_kind = ChildCommandKind::BrowserActionIntentHandoff;

    assert_eq!(
        serialized_field!(&event, "commandKind"),
        serde_json::json!("browser-action-intent-handoff")
    );
}

#[test]
fn child_command_contract_rejects_missing_child_command_ref() {
    let event = serde_json::json!({
        "schemaVersion": constants::child_agent::EVENT_SCHEMA_VERSION,
        "commandReceivedEventRef": constants::child_agent::TEST_COMMAND_RECEIVED_EVENT_REF,
        "receivedAt": constants::activity_store::TEST_FIRST_OBSERVED_AT,
        "deviceRef": constants::child_agent::TEST_DEVICE_REF,
        "parentControllerEventRef": constants::child_agent::TEST_PARENT_CONTROLLER_EVENT_REF,
        "transportMessageRef": constants::child_agent::TEST_TRANSPORT_MESSAGE_REF,
        "commandKind": "apply-policy"
    });

    let parsed = serde_json::from_value::<ChildCommandReceivedEvent>(event);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}
