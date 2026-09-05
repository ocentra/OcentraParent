#[path = "parent_assistant_payload/payload_state_backend.rs"]
mod payload_state_backend;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionConfirmResult;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantActionPreviewResult;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderStatus;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunCancelResult;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantThreadResponse;

use crate::fields::fields_from_pairs;
use crate::parent_assistant_provider_state_value::parent_assistant_provider_state_value;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantTextRef<'a>(&'a str);

pub(crate) fn parent_assistant_thread_payload(
    response: &ParentAssistantThreadResponse,
) -> LogFields {
    let active_thread = response.active_thread.as_ref();
    fields_from_pairs(vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(
                ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                    .to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            LogFieldValue::String(
                payload_state_backend::backend_state_value(response.backend_state)
                    .0
                    .to_string(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_THREAD_ID,
            match active_thread.map(|thread| thread.thread_id.as_str()) {
                Some(text) => LogFieldValue::String(text.to_string()),
                None => LogFieldValue::Null(()),
            },
        ),
        (
            constants::parent_assistant::FIELD_THREAD,
            LogFieldValue::String(serde_json::to_string(&active_thread).unwrap_or_default()),
        ),
        (
            constants::parent_assistant::FIELD_THREADS,
            LogFieldValue::String(serde_json::to_string(&response.threads).unwrap_or_default()),
        ),
        (
            constants::parent_assistant::FIELD_THREAD_RESPONSE,
            LogFieldValue::String(serde_json::to_string(response).unwrap_or_default()),
        ),
        (
            constants::field::REASON,
            optional_string(response.reason.as_deref().map(ParentAssistantTextRef)),
        ),
    ])
}

pub(crate) fn parent_assistant_provider_status_payload(
    status: &ParentAssistantProviderStatus,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(
                ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                    .to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            LogFieldValue::String(
                constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED.to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_PROVIDER_STATE,
            LogFieldValue::String(
                parent_assistant_provider_state_value(status.provider_state).to_string(),
            ),
        ),
        (
            constants::field::LOCAL_AI_PROVIDER_ID,
            LogFieldValue::String(status.provider_id.clone()),
        ),
        (
            constants::field::LOCAL_AI_MODEL_ID,
            LogFieldValue::String(status.model_id.clone()),
        ),
        (
            constants::field::LOCAL_AI_DEGRADED_STATE,
            LogFieldValue::String(status.degraded_state.as_protocol_str().to_string()),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(
                status
                    .unavailable_reason
                    .as_deref()
                    .map(ParentAssistantTextRef),
            ),
        ),
        (
            constants::parent_assistant::FIELD_PROVIDER_STATUS,
            LogFieldValue::String(serde_json::to_string(status).unwrap_or_default()),
        ),
        (
            constants::field::PARENT_ASSISTANT_API_PROVIDER_BOUNDARY,
            LogFieldValue::String(
                serde_json::to_string(&status.api_provider_boundary).unwrap_or_default(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_PROVIDER_ROUTE,
            LogFieldValue::String(
                serde_json::to_string(&status.provider_route).unwrap_or_default(),
            ),
        ),
    ])
}

pub(crate) fn parent_assistant_run_cancel_payload(
    result: &ParentAssistantRunCancelResult,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(
                ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                    .to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            LogFieldValue::String(
                constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED.to_string(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_THREAD_ID,
            LogFieldValue::String(result.thread_id.clone()),
        ),
        (
            constants::parent_assistant::FIELD_RUN_ID,
            LogFieldValue::String(result.run_id.clone()),
        ),
        (
            constants::field::REASON,
            optional_string(
                result
                    .unavailable_reason
                    .as_deref()
                    .map(ParentAssistantTextRef),
            ),
        ),
        (
            constants::parent_assistant::FIELD_RUN_CANCEL_RESULT,
            LogFieldValue::String(serde_json::to_string(result).unwrap_or_default()),
        ),
    ])
}

pub(crate) fn parent_assistant_action_confirm_payload(
    result: &ParentAssistantActionConfirmResult,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(
                ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                    .to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            LogFieldValue::String(
                constants::parent_assistant::BACKEND_STATE_CONTRACT_REQUIRED.to_string(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_ACTION_INTENT_ID,
            LogFieldValue::String(result.action_intent_id.clone()),
        ),
        (
            constants::parent_assistant::FIELD_REQUIRED_CHILD_CONTRACTS,
            LogFieldValue::String(
                constants::parent_assistant::REQUIRED_CHILD_CONTRACT_POLICY_WRITE.to_string(),
            ),
        ),
        (
            constants::field::REASON,
            LogFieldValue::String(result.reason.clone()),
        ),
        (
            constants::parent_assistant::FIELD_ACTION_CONFIRM_RESULT,
            LogFieldValue::String(serde_json::to_string(result).unwrap_or_default()),
        ),
    ])
}

pub(crate) fn parent_assistant_action_preview_payload(
    result: &ParentAssistantActionPreviewResult,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(
                ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                    .to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            LogFieldValue::String(
                constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED.to_string(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_ACTION_INTENT_ID,
            LogFieldValue::String(result.action_intent_id.clone()),
        ),
        (
            constants::parent_assistant::FIELD_REQUIRED_CHILD_CONTRACTS,
            LogFieldValue::String(
                constants::parent_assistant::REQUIRED_CHILD_CONTRACT_POLICY_WRITE.to_string(),
            ),
        ),
        (
            constants::field::REASON,
            LogFieldValue::String(result.reason.clone()),
        ),
        (
            constants::field::PARENT_ASSISTANT_ACTION_PREVIEW,
            LogFieldValue::String(serde_json::to_string(result).unwrap_or_default()),
        ),
    ])
}

fn optional_string(value: Option<ParentAssistantTextRef<'_>>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.0.to_string()),
        None => LogFieldValue::Null(()),
    }
}
