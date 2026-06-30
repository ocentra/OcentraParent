use crate::child_agent::child_agent_events::{
    ChildCapabilityState, ChildCapabilityStateUpdatedEvent, ChildCommandAcceptedEvent,
    ChildCommandDecision, ChildCommandKind, ChildCommandReceivedEvent, ChildCommandRejectedEvent,
    ChildRuntimeHealthState, ChildRuntimeHealthUpdatedEvent,
};
use crate::constants;

pub(super) fn child_command_received_event() -> ChildCommandReceivedEvent {
    ChildCommandReceivedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_received_event_ref: constants::child_agent::TEST_COMMAND_RECEIVED_EVENT_REF
            .to_string(),
        child_command_ref: constants::child_agent::TEST_CHILD_COMMAND_REF.to_string(),
        received_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        device_ref: constants::child_agent::TEST_DEVICE_REF.to_string(),
        parent_controller_event_ref: constants::child_agent::TEST_PARENT_CONTROLLER_EVENT_REF
            .to_string(),
        transport_message_ref: constants::child_agent::TEST_TRANSPORT_MESSAGE_REF.to_string(),
        command_kind: ChildCommandKind::ApplyPolicy,
    }
}

pub(super) fn child_command_accepted_event() -> ChildCommandAcceptedEvent {
    ChildCommandAcceptedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_accepted_event_ref: constants::child_agent::TEST_COMMAND_ACCEPTED_EVENT_REF
            .to_string(),
        child_command_ref: constants::child_agent::TEST_CHILD_COMMAND_REF.to_string(),
        accepted_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        decision: ChildCommandDecision::ManualRequired,
        causation_event_ref: constants::child_agent::TEST_COMMAND_RECEIVED_EVENT_REF.to_string(),
        idempotency_key: constants::child_agent::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

pub(super) fn child_command_rejected_event() -> ChildCommandRejectedEvent {
    ChildCommandRejectedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_rejected_event_ref: constants::child_agent::TEST_COMMAND_REJECTED_EVENT_REF
            .to_string(),
        child_command_ref: constants::child_agent::TEST_CHILD_COMMAND_REF.to_string(),
        rejected_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        rejection_reason_code: constants::child_agent::TEST_REJECTION_CODE.to_string(),
        causation_event_ref: constants::child_agent::TEST_COMMAND_RECEIVED_EVENT_REF.to_string(),
        idempotency_key: constants::child_agent::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

pub(super) fn child_capability_state_updated_event() -> ChildCapabilityStateUpdatedEvent {
    ChildCapabilityStateUpdatedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        capability_state_event_ref: constants::child_agent::TEST_CAPABILITY_STATE_EVENT_REF
            .to_string(),
        device_ref: constants::child_agent::TEST_DEVICE_REF.to_string(),
        updated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        capability_ref: constants::child_agent::TEST_CAPABILITY_REF.to_string(),
        capability_state: ChildCapabilityState::ManualRequired,
        previous_event_ref: constants::child_agent::TEST_COMMAND_ACCEPTED_EVENT_REF.to_string(),
        custody: constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME.to_string(),
    }
}

pub(super) fn child_runtime_health_updated_event() -> ChildRuntimeHealthUpdatedEvent {
    ChildRuntimeHealthUpdatedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        runtime_health_event_ref: constants::child_agent::TEST_RUNTIME_HEALTH_EVENT_REF.to_string(),
        device_ref: constants::child_agent::TEST_DEVICE_REF.to_string(),
        updated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        health_state: ChildRuntimeHealthState::Degraded,
        previous_event_ref: constants::child_agent::TEST_CAPABILITY_STATE_EVENT_REF.to_string(),
        custody: constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME.to_string(),
    }
}
