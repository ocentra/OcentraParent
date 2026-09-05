#[path = "parent_assistant_payload/payload_state_answer.rs"]
mod payload_state_answer;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswer;

use crate::fields::fields_from_pairs;
use crate::parent_assistant_provider_state_value::parent_assistant_provider_state_value;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ParentAssistantTextRef<'a>(&'a str);

pub(crate) fn parent_assistant_answer_payload(answer: &ParentAssistantAnswer) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::PARENT_ASSISTANT_ANSWER,
            LogFieldValue::String(serde_json::to_string(answer).unwrap_or_default()),
        ),
        (
            constants::field::PARENT_ASSISTANT_REQUEST_ID,
            LogFieldValue::String(answer.request_id.clone()),
        ),
        (
            constants::field::PARENT_ASSISTANT_PROVIDER_STATE,
            LogFieldValue::String(
                parent_assistant_provider_state_value(answer.provider_state).to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_ANSWER_STATE,
            LogFieldValue::String(
                payload_state_answer::answer_state_value(answer.answer_state)
                    .0
                    .to_string(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_ANSWER_TEXT,
            optional_string(answer.answer_text.as_deref().map(ParentAssistantTextRef)),
        ),
        (
            constants::field::PARENT_ASSISTANT_CITATION_COUNT,
            LogFieldValue::Number(answer.citations.len() as f64),
        ),
        (
            constants::field::PARENT_ASSISTANT_ACTION_PREVIEW,
            LogFieldValue::String(
                serde_json::to_string(&answer.action_preview).unwrap_or_default(),
            ),
        ),
        (
            constants::field::PARENT_ASSISTANT_API_PROVIDER_BOUNDARY,
            LogFieldValue::String(
                serde_json::to_string(&answer.api_provider_boundary).unwrap_or_default(),
            ),
        ),
        (
            constants::parent_assistant::FIELD_PROVIDER_ROUTE,
            LogFieldValue::String(
                serde_json::to_string(&answer.provider_route).unwrap_or_default(),
            ),
        ),
        (
            constants::field::LOCAL_AI_RESULT_ID,
            optional_string(
                answer
                    .local_ai_result_id
                    .as_deref()
                    .map(ParentAssistantTextRef),
            ),
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(
                answer
                    .unavailable_reason
                    .as_deref()
                    .map(ParentAssistantTextRef),
            ),
        ),
    ])
}

fn optional_string(value: Option<ParentAssistantTextRef<'_>>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.0.to_string()),
        None => LogFieldValue::Null(()),
    }
}
