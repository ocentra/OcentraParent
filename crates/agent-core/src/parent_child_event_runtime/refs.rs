use ocentra_parent_agent_protocol::constants;

use super::ParentChildRuntimeInput;

pub(super) fn event_ref(input: &ParentChildRuntimeInput, event_type: &str) -> String {
    let mut value = parent_child_correlation_id(input);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(event_type);
    value
}

pub(super) fn parent_command_ref(input: &ParentChildRuntimeInput) -> String {
    suffixed_runtime_ref(
        input,
        constants::parent_controller::REF_PARENT_COMMAND_SUFFIX,
    )
}

pub(super) fn child_command_ref(input: &ParentChildRuntimeInput) -> String {
    suffixed_runtime_ref(
        input,
        constants::parent_controller::REF_CHILD_COMMAND_SUFFIX,
    )
}

pub(super) fn transport_message_ref(input: &ParentChildRuntimeInput) -> String {
    suffixed_runtime_ref(
        input,
        constants::parent_controller::REF_TRANSPORT_MESSAGE_SUFFIX,
    )
}

pub(super) fn read_model_ref(input: &ParentChildRuntimeInput) -> String {
    suffixed_runtime_ref(
        input,
        constants::parent_controller::REF_PARENT_READ_MODEL_SUFFIX,
    )
}

pub(super) fn parent_child_correlation_id(input: &ParentChildRuntimeInput) -> String {
    let mut value =
        String::from(constants::parent_controller::CORRELATION_PARENT_CHILD_RUNTIME_PREFIX);
    value.push_str(&input.parent_intent_ref);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(&input.observed_at);
    value
}

pub(super) fn event_idempotency_key(input: &ParentChildRuntimeInput, event_type: &str) -> String {
    parent_child_idempotency_key(&event_ref(input, event_type))
}

pub(super) fn parent_child_idempotency_key(event_ref: &str) -> String {
    let mut value =
        String::from(constants::parent_controller::IDEMPOTENCY_PARENT_CHILD_RUNTIME_PREFIX);
    value.push_str(event_ref);
    value
}

fn suffixed_runtime_ref(input: &ParentChildRuntimeInput, suffix: &str) -> String {
    let mut value = parent_child_correlation_id(input);
    value.push(constants::delimiter::HYPHEN);
    value.push_str(suffix);
    value
}
