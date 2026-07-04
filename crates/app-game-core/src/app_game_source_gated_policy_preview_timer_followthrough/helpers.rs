use super::*;

const PROTOCOL_HANDOFF_PROOF_REQUIRED: &str = "protocol-proof-required";

pub(super) fn next_state(
    source_state: &str,
    source_required_state: &str,
    target_required_state: &'static str,
) -> &'static str {
    match source_state {
        state if state == source_required_state => target_required_state,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

pub(super) fn protocol_read_model_state_for_handoff(protocol_handoff_state: &str) -> &'static str {
    next_state(
        protocol_handoff_state,
        PROTOCOL_HANDOFF_PROOF_REQUIRED,
        PROTOCOL_READ_MODEL_PROOF_REQUIRED,
    )
}

pub(super) fn protocol_command_handoff_state_for_read_model(
    read_model_state: &str,
) -> &'static str {
    next_state(
        read_model_state,
        PROTOCOL_READ_MODEL_PROOF_REQUIRED,
        PROTOCOL_COMMAND_HANDOFF_PROOF_REQUIRED,
    )
}

pub(super) fn service_handler_state_for_command_handoff(
    command_handoff_state: &str,
) -> &'static str {
    next_state(
        command_handoff_state,
        PROTOCOL_COMMAND_HANDOFF_PROOF_REQUIRED,
        SERVICE_HANDLER_PROOF_REQUIRED,
    )
}

pub(super) fn service_read_api_state_for_service_handler_handoff(
    handler_handoff_state: &str,
) -> &'static str {
    next_state(
        handler_handoff_state,
        SERVICE_HANDLER_PROOF_REQUIRED,
        SERVICE_READ_API_PROOF_REQUIRED,
    )
}

pub(super) fn read_api_response_state_for_read_api_handoff(
    read_api_handoff_state: &str,
) -> &'static str {
    next_state(
        read_api_handoff_state,
        SERVICE_READ_API_PROOF_REQUIRED,
        READ_API_RESPONSE_PROOF_REQUIRED,
    )
}

pub(super) fn read_api_response_consumer_state_for_response_handoff(
    response_handoff_state: &str,
) -> &'static str {
    next_state(
        response_handoff_state,
        READ_API_RESPONSE_PROOF_REQUIRED,
        READ_API_RESPONSE_CONSUMER_PROOF_REQUIRED,
    )
}

pub(super) fn response_consumer_parent_surface_state_for_read_api_response_consumer_handoff(
    consumer_handoff_state: &str,
) -> &'static str {
    next_state(
        consumer_handoff_state,
        READ_API_RESPONSE_CONSUMER_PROOF_REQUIRED,
        PARENT_SURFACE_PROOF_REQUIRED,
    )
}

pub(super) fn response_consumer_parent_surface_read_model_state_for_parent_surface_handoff(
    parent_surface_handoff_state: &str,
) -> &'static str {
    next_state(
        parent_surface_handoff_state,
        PARENT_SURFACE_PROOF_REQUIRED,
        PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
    )
}

pub(super) fn response_consumer_parent_surface_status_state_for_read_model_handoff(
    read_model_handoff_state: &str,
) -> &'static str {
    next_state(
        read_model_handoff_state,
        PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
        PARENT_SURFACE_STATUS_PROOF_REQUIRED,
    )
}

pub(super) fn response_consumer_parent_surface_status_read_model_state_for_status_handoff(
    status_handoff_state: &str,
) -> &'static str {
    next_state(
        status_handoff_state,
        PARENT_SURFACE_STATUS_PROOF_REQUIRED,
        PARENT_SURFACE_STATUS_READ_MODEL_PROOF_REQUIRED,
    )
}

pub(super) fn parent_surface_status_read_model_parent_surface_state_for_status_read_model_handoff(
    status_read_model_handoff_state: &str,
) -> &'static str {
    next_state(
        status_read_model_handoff_state,
        PARENT_SURFACE_STATUS_READ_MODEL_PROOF_REQUIRED,
        PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_PROOF_REQUIRED,
    )
}

pub(super) fn parent_surface_status_read_model_parent_surface_read_model_handoff_state_for_parent_surface_handoff(
    parent_surface_handoff_state: &str,
) -> &'static str {
    next_state(
        parent_surface_handoff_state,
        PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_PROOF_REQUIRED,
        PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
    )
}

pub(super) fn parent_surface_status_read_model_parent_surface_read_model_state_for_handoff(
    parent_surface_read_model_handoff_state: &str,
) -> &'static str {
    next_state(
        parent_surface_read_model_handoff_state,
        PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
        READY_FOR_PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL,
    )
}

pub(super) fn count_state<T, F>(rows: &[T], state_of: F, needle: &str) -> usize
where
    F: Fn(&T) -> &str,
{
    rows.iter().filter(|row| state_of(row) == needle).count()
}
