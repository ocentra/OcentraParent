use super::{
    constants, tracking_retention_accepted_at, AgentCommandEnvelope,
    TrackingRetentionSettingsWriteRequest,
};
use ocentra_parent_agent_protocol::child_agent::child_agent_events::{
    ChildCommandKind, ChildCommandReceivedEvent,
};
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentActionReceivedEvent, ParentControllerActionKind, ParentControllerSource,
};
use ocentra_parent_agent_protocol::parent_controller_events::{
    ParentChildCommandForwardRequestedEvent, ParentChildCommandTransportBoundary,
    ParentCommandRejectedEvent, ParentCommandValidatedEvent, ParentCommandValidationState,
};
use ocentra_parent_agent_protocol::tracking::config_update_event::ParentTrackingConfigUpdatedEvent;

type TrackingRetentionCommandId =
    ocentra_parent_agent_protocol::tracking::identifiers::TrackingRetentionCommandId;

const TRACKING_PARENT_ACTION_SUFFIX: &str = "parent-action";
const TRACKING_PARENT_INTENT_SUFFIX: &str = "parent-intent";
const TRACKING_PARENT_PROFILE_SUFFIX: &str = "parent-profile";
const TRACKING_COMMAND_VALIDATED_SUFFIX: &str = "command-validated";
const TRACKING_COMMAND_REJECTED_SUFFIX: &str = "command-rejected";
const TRACKING_FORWARD_REQUESTED_SUFFIX: &str = "forward-requested";
const TRACKING_CHILD_COMMAND_RECEIVED_SUFFIX: &str = "child-command-received";
const TRACKING_RETENTION_EVENT_REF_PREFIX: &str = "event.tracking-retention-settings-write.";
const TRACKING_REF_SEPARATOR: &str = ".";

pub(super) fn tracking_parent_action_received_event(
    command: &AgentCommandEnvelope,
    request: &TrackingRetentionSettingsWriteRequest,
) -> ParentActionReceivedEvent {
    ParentActionReceivedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        parent_action_event_ref: tracking_service_event_ref(
            &request.command_id,
            TRACKING_PARENT_ACTION_SUFFIX,
        ),
        received_at: tracking_retention_accepted_at().to_string(),
        parent_intent_ref: tracking_service_ref(&request.command_id, TRACKING_PARENT_INTENT_SUFFIX),
        parent_profile_ref: tracking_service_ref(
            &request.command_id,
            TRACKING_PARENT_PROFILE_SUFFIX,
        ),
        device_ref: command.target.device_id.clone(),
        action_kind: ParentControllerActionKind::UpdateTrackingConfig,
        source: ParentControllerSource::PortalTypedIntent,
        custody: constants::parent_controller::CUSTODY_LOCAL_SERVICE_VALIDATION.to_string(),
        idempotency_key: tracking_service_idempotency_key(
            &request.command_id,
            TRACKING_PARENT_ACTION_SUFFIX,
        ),
    }
}

pub(super) fn tracking_parent_command_validated_event(
    command_id: &TrackingRetentionCommandId,
    parent_action_received: &ParentActionReceivedEvent,
) -> ParentCommandValidatedEvent {
    ParentCommandValidatedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_validated_event_ref: tracking_service_event_ref(
            command_id,
            TRACKING_COMMAND_VALIDATED_SUFFIX,
        ),
        parent_action_event_ref: parent_action_received.parent_action_event_ref.clone(),
        parent_command_ref: tracking_parent_command_ref(command_id),
        child_command_ref: Some(tracking_child_command_ref(command_id)),
        validated_at: tracking_retention_accepted_at().to_string(),
        validation_state: ParentCommandValidationState::Validated,
        causation_event_ref: parent_action_received.parent_action_event_ref.clone(),
        idempotency_key: tracking_service_idempotency_key(
            command_id,
            TRACKING_COMMAND_VALIDATED_SUFFIX,
        ),
    }
}

pub(super) fn tracking_parent_command_rejected_event(
    command_id: &TrackingRetentionCommandId,
    parent_action_received: &ParentActionReceivedEvent,
    rejection_reason_code: impl Into<String>,
) -> ParentCommandRejectedEvent {
    ParentCommandRejectedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        command_rejected_event_ref: tracking_service_event_ref(
            command_id,
            TRACKING_COMMAND_REJECTED_SUFFIX,
        ),
        parent_action_event_ref: parent_action_received.parent_action_event_ref.clone(),
        rejected_at: tracking_retention_accepted_at().to_string(),
        rejection_reason_code: rejection_reason_code.into(),
        causation_event_ref: parent_action_received.parent_action_event_ref.clone(),
        idempotency_key: tracking_service_idempotency_key(
            command_id,
            TRACKING_COMMAND_REJECTED_SUFFIX,
        ),
    }
}

pub(super) fn tracking_parent_child_command_forward_requested_event(
    command_id: &TrackingRetentionCommandId,
    parent_command_validated: &ParentCommandValidatedEvent,
    parent_event: &ParentTrackingConfigUpdatedEvent,
) -> ParentChildCommandForwardRequestedEvent {
    ParentChildCommandForwardRequestedEvent {
        schema_version: constants::parent_controller::EVENT_SCHEMA_VERSION,
        forward_requested_event_ref: tracking_service_event_ref(
            command_id,
            TRACKING_FORWARD_REQUESTED_SUFFIX,
        ),
        parent_command_ref: parent_command_validated.parent_command_ref.clone(),
        child_command_ref: tracking_child_command_ref(command_id),
        device_ref: parent_event.target.device_id.as_str().to_string(),
        requested_at: tracking_retention_accepted_at().to_string(),
        transport_boundary: ParentChildCommandTransportBoundary::TypedLocalServiceTransport,
        causation_event_ref: parent_command_validated.command_validated_event_ref.clone(),
        idempotency_key: tracking_service_idempotency_key(
            command_id,
            TRACKING_FORWARD_REQUESTED_SUFFIX,
        ),
    }
}

pub(super) fn tracking_child_command_received_event(
    command_id: &TrackingRetentionCommandId,
    forward_requested_event: &ParentChildCommandForwardRequestedEvent,
) -> ChildCommandReceivedEvent {
    ChildCommandReceivedEvent {
        schema_version: constants::child_agent::EVENT_SCHEMA_VERSION,
        command_received_event_ref: tracking_service_event_ref(
            command_id,
            TRACKING_CHILD_COMMAND_RECEIVED_SUFFIX,
        ),
        child_command_ref: forward_requested_event.child_command_ref.clone(),
        received_at: tracking_retention_accepted_at().to_string(),
        device_ref: forward_requested_event.device_ref.clone(),
        parent_controller_event_ref: forward_requested_event.forward_requested_event_ref.clone(),
        transport_message_ref: tracking_transport_message_ref(command_id),
        command_kind: ChildCommandKind::ApplyTrackingConfig,
    }
}

fn tracking_parent_command_ref(command_id: &TrackingRetentionCommandId) -> String {
    tracking_service_ref(
        command_id,
        constants::parent_controller::REF_PARENT_COMMAND_SUFFIX,
    )
}

fn tracking_child_command_ref(command_id: &TrackingRetentionCommandId) -> String {
    tracking_service_ref(
        command_id,
        constants::parent_controller::REF_CHILD_COMMAND_SUFFIX,
    )
}

fn tracking_transport_message_ref(command_id: &TrackingRetentionCommandId) -> String {
    tracking_service_ref(
        command_id,
        constants::parent_controller::REF_TRANSPORT_MESSAGE_SUFFIX,
    )
}

fn tracking_service_ref(command_id: &TrackingRetentionCommandId, suffix: &str) -> String {
    build_tracking_ref(
        constants::parent_controller::CORRELATION_PARENT_CHILD_RUNTIME_PREFIX,
        command_id,
        suffix,
    )
}

fn tracking_service_event_ref(command_id: &TrackingRetentionCommandId, suffix: &str) -> String {
    build_tracking_ref(TRACKING_RETENTION_EVENT_REF_PREFIX, command_id, suffix)
}

fn tracking_service_idempotency_key(
    command_id: &TrackingRetentionCommandId,
    suffix: &str,
) -> String {
    build_tracking_ref(
        constants::parent_controller::IDEMPOTENCY_PARENT_CHILD_RUNTIME_PREFIX,
        command_id,
        suffix,
    )
}

fn build_tracking_ref(
    prefix: &str,
    command_id: &TrackingRetentionCommandId,
    suffix: &str,
) -> String {
    let mut value = prefix.to_string();
    value.push_str(&command_id.to_string());
    value.push_str(TRACKING_REF_SEPARATOR);
    value.push_str(suffix);
    value
}
