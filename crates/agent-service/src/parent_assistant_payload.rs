use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, LogFields, ParentAssistantAnswer, ParentAssistantAnswerState,
    ParentAssistantProviderState,
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

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}
