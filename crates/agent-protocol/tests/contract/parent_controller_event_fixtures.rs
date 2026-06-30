use crate::{
    constants,
    parent_controller_events::{
        ParentActionReceivedEvent, ParentChildCommandDeliveryState,
        ParentChildCommandForwardRequestedEvent, ParentChildCommandForwardedEvent,
        ParentChildCommandTransportBoundary, ParentCommandRejectedEvent,
        ParentCommandValidatedEvent, ParentCommandValidationState, ParentControllerActionKind,
        ParentControllerSource, ParentProjectionKind, ParentReadModelProjectedEvent,
    },
};

pub(super) fn parent_action_received_event() -> ParentActionReceivedEvent {
    ParentActionReceivedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        parent_action_event_ref: constants::parent_controller::TEST_PARENT_ACTION_EVENT_REF
            .to_string(),
        received_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        parent_intent_ref: constants::parent_controller::TEST_PARENT_INTENT_REF.to_string(),
        parent_profile_ref: constants::parent_controller::TEST_PARENT_PROFILE_REF.to_string(),
        device_ref: constants::parent_controller::TEST_DEVICE_REF.to_string(),
        action_kind: ParentControllerActionKind::Block,
        source: ParentControllerSource::PortalTypedIntent,
        custody: constants::parent_controller::CUSTODY_LOCAL_SERVICE_VALIDATION.to_string(),
        idempotency_key: constants::parent_controller::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

pub(super) fn parent_command_validated_event() -> ParentCommandValidatedEvent {
    ParentCommandValidatedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_validated_event_ref: constants::parent_controller::TEST_COMMAND_VALIDATED_EVENT_REF
            .to_string(),
        parent_action_event_ref: constants::parent_controller::TEST_PARENT_ACTION_EVENT_REF
            .to_string(),
        parent_command_ref: constants::parent_controller::TEST_PARENT_COMMAND_REF.to_string(),
        child_command_ref: Some(constants::parent_controller::TEST_CHILD_COMMAND_REF.to_string()),
        validated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        validation_state: ParentCommandValidationState::Validated,
        causation_event_ref: constants::parent_controller::TEST_PARENT_ACTION_EVENT_REF.to_string(),
        idempotency_key: constants::parent_controller::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

pub(super) fn parent_command_rejected_event() -> ParentCommandRejectedEvent {
    ParentCommandRejectedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_rejected_event_ref: constants::parent_controller::TEST_COMMAND_REJECTED_EVENT_REF
            .to_string(),
        parent_action_event_ref: constants::parent_controller::TEST_PARENT_ACTION_EVENT_REF
            .to_string(),
        rejected_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        rejection_reason_code: constants::parent_controller::TEST_VALIDATION_REJECTION_CODE
            .to_string(),
        causation_event_ref: constants::parent_controller::TEST_PARENT_ACTION_EVENT_REF.to_string(),
        idempotency_key: constants::parent_controller::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

pub(super) fn parent_child_command_forward_requested_event(
) -> ParentChildCommandForwardRequestedEvent {
    ParentChildCommandForwardRequestedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        forward_requested_event_ref: constants::parent_controller::TEST_FORWARD_REQUESTED_EVENT_REF
            .to_string(),
        parent_command_ref: constants::parent_controller::TEST_PARENT_COMMAND_REF.to_string(),
        child_command_ref: constants::parent_controller::TEST_CHILD_COMMAND_REF.to_string(),
        device_ref: constants::parent_controller::TEST_DEVICE_REF.to_string(),
        requested_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        transport_boundary: ParentChildCommandTransportBoundary::TypedLocalServiceTransport,
        causation_event_ref: constants::parent_controller::TEST_COMMAND_VALIDATED_EVENT_REF
            .to_string(),
        idempotency_key: constants::parent_controller::TEST_IDEMPOTENCY_KEY.to_string(),
    }
}

pub(super) fn parent_child_command_forwarded_event() -> ParentChildCommandForwardedEvent {
    ParentChildCommandForwardedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        forwarded_event_ref: constants::parent_controller::TEST_FORWARDED_EVENT_REF.to_string(),
        child_command_ref: constants::parent_controller::TEST_CHILD_COMMAND_REF.to_string(),
        transport_message_ref: constants::parent_controller::TEST_TRANSPORT_MESSAGE_REF.to_string(),
        forwarded_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        delivery_state: ParentChildCommandDeliveryState::Forwarded,
        causation_event_ref: constants::parent_controller::TEST_FORWARD_REQUESTED_EVENT_REF
            .to_string(),
    }
}

pub(super) fn parent_read_model_projected_event() -> ParentReadModelProjectedEvent {
    ParentReadModelProjectedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        read_model_projected_event_ref:
            constants::parent_controller::TEST_READ_MODEL_PROJECTED_EVENT_REF.to_string(),
        read_model_ref: constants::parent_controller::TEST_READ_MODEL_REF.to_string(),
        previous_event_ref: constants::parent_controller::TEST_FORWARDED_EVENT_REF.to_string(),
        projected_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        projection_kind: ParentProjectionKind::ChildCommandStatus,
        visible_to_portal: true,
    }
}
