use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCapabilityState, ChildCapabilityStateUpdatedEvent, ChildCommandAcceptedEvent,
    ChildCommandDecision, ChildCommandReceivedEvent, ChildRuntimeHealthState,
    ChildRuntimeHealthUpdatedEvent,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentActionReceivedEvent, ParentChildCommandDeliveryState,
    ParentChildCommandForwardRequestedEvent, ParentChildCommandForwardedEvent,
    ParentChildCommandTransportBoundary, ParentCommandValidatedEvent, ParentCommandValidationState,
    ParentProjectionKind, ParentReadModelProjectedEvent,
};

use super::{refs, ParentChildRuntimeEventPayload, ParentChildRuntimeInput};

pub(super) fn runtime_events_for_input(
    input: &ParentChildRuntimeInput,
) -> Vec<ParentChildRuntimeEventPayload> {
    vec![
        ParentChildRuntimeEventPayload::ParentActionReceived(parent_action_received_event(input)),
        ParentChildRuntimeEventPayload::ParentCommandValidated(parent_command_validated_event(
            input,
        )),
        ParentChildRuntimeEventPayload::ParentChildCommandForwardRequested(
            parent_child_command_forward_requested_event(input),
        ),
        ParentChildRuntimeEventPayload::ParentChildCommandForwarded(
            parent_child_command_forwarded_event(input),
        ),
        ParentChildRuntimeEventPayload::ChildCommandReceived(child_command_received_event(input)),
        ParentChildRuntimeEventPayload::ChildCommandAccepted(child_command_accepted_event(input)),
        ParentChildRuntimeEventPayload::ChildCapabilityStateUpdated(
            child_capability_state_updated_event(input),
        ),
        ParentChildRuntimeEventPayload::ChildRuntimeHealthUpdated(
            child_runtime_health_updated_event(input),
        ),
        ParentChildRuntimeEventPayload::ParentReadModelProjected(
            parent_read_model_projected_event(input),
        ),
    ]
}

fn parent_action_received_event(input: &ParentChildRuntimeInput) -> ParentActionReceivedEvent {
    ParentActionReceivedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        parent_action_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED,
        ),
        received_at: input.observed_at.clone(),
        parent_intent_ref: input.parent_intent_ref.clone(),
        parent_profile_ref: input.parent_profile_ref.clone(),
        device_ref: input.device_ref.clone(),
        action_kind: input.action_kind,
        source: input.source,
        custody: constants::parent_controller::CUSTODY_LOCAL_SERVICE_VALIDATION.to_string(),
        idempotency_key: refs::event_idempotency_key(
            input,
            constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED,
        ),
    }
}

fn parent_command_validated_event(input: &ParentChildRuntimeInput) -> ParentCommandValidatedEvent {
    ParentCommandValidatedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_validated_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_COMMAND_VALIDATED,
        ),
        parent_action_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED,
        ),
        parent_command_ref: refs::parent_command_ref(input),
        child_command_ref: Some(refs::child_command_ref(input)),
        validated_at: input.observed_at.clone(),
        validation_state: ParentCommandValidationState::Validated,
        causation_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED,
        ),
        idempotency_key: refs::event_idempotency_key(
            input,
            constants::parent_controller::EVENT_COMMAND_VALIDATED,
        ),
    }
}

fn parent_child_command_forward_requested_event(
    input: &ParentChildRuntimeInput,
) -> ParentChildCommandForwardRequestedEvent {
    ParentChildCommandForwardRequestedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        forward_requested_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED,
        ),
        parent_command_ref: refs::parent_command_ref(input),
        child_command_ref: refs::child_command_ref(input),
        device_ref: input.device_ref.clone(),
        requested_at: input.observed_at.clone(),
        transport_boundary: ParentChildCommandTransportBoundary::TypedLocalServiceTransport,
        causation_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_COMMAND_VALIDATED,
        ),
        idempotency_key: refs::event_idempotency_key(
            input,
            constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED,
        ),
    }
}

fn parent_child_command_forwarded_event(
    input: &ParentChildRuntimeInput,
) -> ParentChildCommandForwardedEvent {
    ParentChildCommandForwardedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        forwarded_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED,
        ),
        child_command_ref: refs::child_command_ref(input),
        transport_message_ref: refs::transport_message_ref(input),
        forwarded_at: input.observed_at.clone(),
        delivery_state: ParentChildCommandDeliveryState::Forwarded,
        causation_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED,
        ),
    }
}

fn child_command_received_event(input: &ParentChildRuntimeInput) -> ChildCommandReceivedEvent {
    ChildCommandReceivedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_received_event_ref: refs::event_ref(
            input,
            constants::child_agent::EVENT_COMMAND_RECEIVED,
        ),
        child_command_ref: refs::child_command_ref(input),
        received_at: input.observed_at.clone(),
        device_ref: input.device_ref.clone(),
        parent_controller_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED,
        ),
        transport_message_ref: refs::transport_message_ref(input),
        command_kind: input.child_command_kind,
    }
}

fn child_command_accepted_event(input: &ParentChildRuntimeInput) -> ChildCommandAcceptedEvent {
    ChildCommandAcceptedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_accepted_event_ref: refs::event_ref(
            input,
            constants::child_agent::EVENT_COMMAND_ACCEPTED,
        ),
        child_command_ref: refs::child_command_ref(input),
        accepted_at: input.observed_at.clone(),
        decision: ChildCommandDecision::Accepted,
        causation_event_ref: refs::event_ref(input, constants::child_agent::EVENT_COMMAND_RECEIVED),
        idempotency_key: refs::event_idempotency_key(
            input,
            constants::child_agent::EVENT_COMMAND_ACCEPTED,
        ),
    }
}

fn child_capability_state_updated_event(
    input: &ParentChildRuntimeInput,
) -> ChildCapabilityStateUpdatedEvent {
    ChildCapabilityStateUpdatedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        capability_state_event_ref: refs::event_ref(
            input,
            constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED,
        ),
        device_ref: input.device_ref.clone(),
        updated_at: input.observed_at.clone(),
        capability_ref: constants::child_agent::TEST_CAPABILITY_REF.to_string(),
        capability_state: ChildCapabilityState::Ready,
        previous_event_ref: refs::event_ref(input, constants::child_agent::EVENT_COMMAND_ACCEPTED),
        custody: constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME.to_string(),
    }
}

fn child_runtime_health_updated_event(
    input: &ParentChildRuntimeInput,
) -> ChildRuntimeHealthUpdatedEvent {
    ChildRuntimeHealthUpdatedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        runtime_health_event_ref: refs::event_ref(
            input,
            constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED,
        ),
        device_ref: input.device_ref.clone(),
        updated_at: input.observed_at.clone(),
        health_state: ChildRuntimeHealthState::Healthy,
        previous_event_ref: refs::event_ref(
            input,
            constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED,
        ),
        custody: constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME.to_string(),
    }
}

fn parent_read_model_projected_event(
    input: &ParentChildRuntimeInput,
) -> ParentReadModelProjectedEvent {
    ParentReadModelProjectedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        read_model_projected_event_ref: refs::event_ref(
            input,
            constants::parent_controller::EVENT_READ_MODEL_PROJECTED,
        ),
        read_model_ref: refs::read_model_ref(input),
        previous_event_ref: refs::event_ref(
            input,
            constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED,
        ),
        projected_at: input.observed_at.clone(),
        projection_kind: ParentProjectionKind::CapabilityState,
        visible_to_portal: true,
    }
}
