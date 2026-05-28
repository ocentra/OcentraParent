use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, LogFields, ParentAssistantActionConfirmResult, ParentAssistantAnswer,
    ParentAssistantAnswerState, ParentAssistantBackendState, ParentAssistantProviderState,
    ParentAssistantProviderStatus, ParentAssistantRunCancelResult, ParentAssistantThreadResponse,
};

use crate::fields::fields_from_pairs;

pub(crate) fn parent_assistant_answer_payload(answer: &ParentAssistantAnswer) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::PARENT_ASSISTANT_ANSWER,
            LogFieldValue::String(
                serde_json::to_string(answer).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_REQUEST_ID,
            LogFieldValue::String(answer.request_id.clone()),
        ),
        (
            constants::field::PARENT_ASSISTANT_PROVIDER_STATE,
            LogFieldValue::String(provider_state_value(answer.provider_state).to_string()),
        ),
        (
            constants::field::PARENT_ASSISTANT_ANSWER_STATE,
            LogFieldValue::String(answer_state_value(answer.answer_state).to_string()),
        ),
        (
            constants::field::PARENT_ASSISTANT_ANSWER_TEXT,
            optional_string(answer.answer_text.as_ref()),
        ),
        (
            constants::field::PARENT_ASSISTANT_CITATION_COUNT,
            LogFieldValue::Number(answer.citations.len() as f64),
        ),
        (
            constants::field::PARENT_ASSISTANT_ACTION_PREVIEW,
            LogFieldValue::String(
                serde_json::to_string(&answer.action_preview)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_API_PROVIDER_BOUNDARY,
            LogFieldValue::String(
                serde_json::to_string(&answer.api_provider_boundary)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
        (
            constants::field::LOCAL_AI_RESULT_ID,
            optional_string(answer.local_ai_result_id.as_ref()),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(answer.unavailable_reason.as_ref()),
        ),
    ])
}

pub(crate) fn parent_assistant_thread_payload(
    response: &ParentAssistantThreadResponse,
) -> LogFields {
    let active_thread = response.active_thread.as_ref();
    fields_from_pairs(vec![
        string_field(
            constants::field::SCHEMA_VERSION,
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6,
        ),
        string_field(
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            backend_state_value(response.backend_state),
        ),
        optional_str_field(
            constants::parent_assistant::FIELD_THREAD_ID,
            active_thread.map(|thread| thread.thread_id.as_str()),
        ),
        json_string_field(
            constants::parent_assistant::FIELD_THREAD,
            serde_json::to_string(&active_thread).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
        json_string_field(
            constants::parent_assistant::FIELD_THREADS,
            serde_json::to_string(&response.threads)
                .expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
        json_string_field(
            constants::parent_assistant::FIELD_THREAD_RESPONSE,
            serde_json::to_string(response).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
        (
            constants::field::REASON,
            optional_string(response.reason.as_ref()),
        ),
    ])
}

pub(crate) fn parent_assistant_provider_status_payload(
    status: &ParentAssistantProviderStatus,
) -> LogFields {
    fields_from_pairs(vec![
        string_field(
            constants::field::SCHEMA_VERSION,
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6,
        ),
        string_field(
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED,
        ),
        string_field(
            constants::field::PARENT_ASSISTANT_PROVIDER_STATE,
            provider_state_value(status.provider_state),
        ),
        string_field(constants::field::LOCAL_AI_PROVIDER_ID, &status.provider_id),
        string_field(constants::field::LOCAL_AI_MODEL_ID, &status.model_id),
        string_field(
            constants::field::LOCAL_AI_DEGRADED_STATE,
            status.degraded_state.as_protocol_str(),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(status.unavailable_reason.as_ref()),
        ),
        json_string_field(
            constants::parent_assistant::FIELD_PROVIDER_STATUS,
            serde_json::to_string(status).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
        json_string_field(
            constants::field::PARENT_ASSISTANT_API_PROVIDER_BOUNDARY,
            serde_json::to_string(&status.api_provider_boundary)
                .expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    ])
}

pub(crate) fn parent_assistant_run_cancel_payload(
    result: &ParentAssistantRunCancelResult,
) -> LogFields {
    fields_from_pairs(vec![
        string_field(
            constants::field::SCHEMA_VERSION,
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6,
        ),
        string_field(
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED,
        ),
        string_field(
            constants::parent_assistant::FIELD_THREAD_ID,
            &result.thread_id,
        ),
        string_field(constants::parent_assistant::FIELD_RUN_ID, &result.run_id),
        (
            constants::field::REASON,
            optional_string(result.unavailable_reason.as_ref()),
        ),
        json_string_field(
            constants::parent_assistant::FIELD_RUN_CANCEL_RESULT,
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    ])
}

pub(crate) fn parent_assistant_action_confirm_payload(
    result: &ParentAssistantActionConfirmResult,
) -> LogFields {
    fields_from_pairs(vec![
        string_field(
            constants::field::SCHEMA_VERSION,
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6,
        ),
        string_field(
            constants::field::PARENT_ASSISTANT_BACKEND_STATE,
            constants::parent_assistant::BACKEND_STATE_CONTRACT_REQUIRED,
        ),
        string_field(
            constants::parent_assistant::FIELD_ACTION_INTENT_ID,
            &result.action_intent_id,
        ),
        string_field(
            constants::parent_assistant::FIELD_REQUIRED_CHILD_CONTRACTS,
            constants::parent_assistant::REQUIRED_CHILD_CONTRACT_POLICY_WRITE,
        ),
        string_field(constants::field::REASON, &result.reason),
        json_string_field(
            constants::parent_assistant::FIELD_ACTION_CONFIRM_RESULT,
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    ])
}

fn provider_state_value(state: ParentAssistantProviderState) -> &'static str {
    match state {
        ParentAssistantProviderState::Configured => {
            constants::parent_assistant::PROVIDER_CONFIGURED
        }
        ParentAssistantProviderState::Degraded => constants::parent_assistant::PROVIDER_DEGRADED,
        ParentAssistantProviderState::Unavailable => {
            constants::parent_assistant::PROVIDER_UNAVAILABLE
        }
    }
}

fn answer_state_value(state: ParentAssistantAnswerState) -> &'static str {
    match state {
        ParentAssistantAnswerState::Answered => constants::parent_assistant::ANSWER_ANSWERED,
        ParentAssistantAnswerState::Queued => constants::parent_assistant::ANSWER_QUEUED,
        ParentAssistantAnswerState::Degraded => constants::parent_assistant::ANSWER_DEGRADED,
        ParentAssistantAnswerState::Unavailable => constants::parent_assistant::ANSWER_UNAVAILABLE,
    }
}

fn backend_state_value(state: ParentAssistantBackendState) -> &'static str {
    match state {
        ParentAssistantBackendState::RuntimeBacked => {
            constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED
        }
        ParentAssistantBackendState::DurableLocal => {
            constants::parent_assistant::BACKEND_STATE_DURABLE_LOCAL
        }
        ParentAssistantBackendState::VolatileLocal => {
            constants::parent_assistant::BACKEND_STATE_VOLATILE_LOCAL
        }
        ParentAssistantBackendState::ContractRequired => {
            constants::parent_assistant::BACKEND_STATE_CONTRACT_REQUIRED
        }
        ParentAssistantBackendState::Unavailable => {
            constants::parent_assistant::BACKEND_STATE_UNAVAILABLE
        }
    }
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_str_field(key: &'static str, value: Option<&str>) -> (&'static str, LogFieldValue) {
    match value {
        Some(text) => string_field(key, text),
        None => (key, LogFieldValue::Null(())),
    }
}

fn string_field(key: &'static str, value: &str) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.to_string()))
}

fn json_string_field(key: &'static str, value: String) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value))
}
