use ocentra_parent_agent_protocol::{
    constants, LocalAiChatGenerationResult, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

pub fn local_ai_chat_generation_payload(result: &LocalAiChatGenerationResult) -> LogFields {
    fields_from_pairs(vec![
        string_field(
            constants::field::LOCAL_AI_RESULT_ID,
            result.local_ai_result_id.clone(),
        ),
        string_field(
            constants::field::LOCAL_AI_RUNTIME_REFERENCE_ID,
            result.runtime_reference_id.clone(),
        ),
        string_field(
            constants::field::LOCAL_AI_PROVIDER_ID,
            result.provider_id.clone(),
        ),
        string_field(constants::field::LOCAL_AI_MODEL_ID, result.model_id.clone()),
        string_field(
            constants::field::LOCAL_AI_MODEL_REFERENCE,
            result.model_reference.clone(),
        ),
        protocol_field(
            constants::field::LOCAL_AI_GENERATION_STATE,
            result.generation_state.as_protocol_str(),
        ),
        (
            constants::field::LOCAL_AI_OUTPUT_TEXT,
            optional_string(&result.output_text),
        ),
        number_field(
            constants::field::LOCAL_AI_PROMPT_CHAR_COUNT,
            result.prompt_char_count,
        ),
        number_field(
            constants::field::LOCAL_AI_MAX_OUTPUT_TOKENS,
            u64::from(result.max_output_tokens),
        ),
        number_field(constants::field::LOCAL_AI_TIMEOUT_MS, result.timeout_ms),
        number_field(constants::field::LOCAL_AI_DURATION_MS, result.duration_ms),
        (
            constants::field::LOCAL_AI_EXIT_CODE,
            optional_exit_code(result.exit_code),
        ),
        number_field(
            constants::field::LOCAL_AI_STDERR_BYTE_SIZE,
            result.stderr_byte_size,
        ),
        (
            constants::field::LOCAL_AI_UNAVAILABLE_REASON,
            optional_string(&result.unavailable_reason),
        ),
    ])
}

fn string_field(key: &'static str, value: String) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value))
}

fn protocol_field(key: &'static str, value: &'static str) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.to_string()))
}

fn number_field(key: &'static str, value: u64) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Number(value as f64))
}

fn optional_string(value: &Option<String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_exit_code(value: Option<i32>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(f64::from(number)),
        None => LogFieldValue::Null(()),
    }
}
