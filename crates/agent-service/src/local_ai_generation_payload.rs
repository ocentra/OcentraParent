#[path = "local_ai_generation_payload/field_pairs.rs"]
mod field_pairs;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::logging::LogFields;

use self::field_pairs::{
    local_ai_generation_fields_from_pairs, number_field, optional_exit_code_field,
    optional_text_field, protocol_field, text_field, LocalAiGenerationFieldKey,
    LocalAiGenerationOwnedText, LocalAiGenerationTextRef,
};

pub fn local_ai_chat_generation_payload(result: &LocalAiChatGenerationResult) -> LogFields {
    local_ai_generation_fields_from_pairs(vec![
        text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_RESULT_ID),
            LocalAiGenerationOwnedText(result.local_ai_result_id.clone()),
        ),
        text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_RUNTIME_REFERENCE_ID),
            LocalAiGenerationOwnedText(result.runtime_reference_id.clone()),
        ),
        text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_PROVIDER_ID),
            LocalAiGenerationOwnedText(result.provider_id.clone()),
        ),
        text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_MODEL_ID),
            LocalAiGenerationOwnedText(result.model_id.clone()),
        ),
        text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_MODEL_REFERENCE),
            LocalAiGenerationOwnedText(result.model_reference.clone()),
        ),
        protocol_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_GENERATION_STATE),
            LocalAiGenerationTextRef(result.generation_state.as_protocol_str()),
        ),
        optional_text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_OUTPUT_TEXT),
            result.output_text.as_deref().map(LocalAiGenerationTextRef),
        ),
        number_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_PROMPT_CHAR_COUNT),
            result.prompt_char_count,
        ),
        number_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_MAX_OUTPUT_TOKENS),
            u64::from(result.max_output_tokens),
        ),
        number_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_TIMEOUT_MS),
            result.timeout_ms,
        ),
        number_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_DURATION_MS),
            result.duration_ms,
        ),
        optional_exit_code_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_EXIT_CODE),
            result.exit_code,
        ),
        number_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_STDERR_BYTE_SIZE),
            result.stderr_byte_size,
        ),
        optional_text_field(
            LocalAiGenerationFieldKey(constants::field::LOCAL_AI_UNAVAILABLE_REASON),
            result
                .unavailable_reason
                .as_deref()
                .map(LocalAiGenerationTextRef),
        ),
    ])
}
