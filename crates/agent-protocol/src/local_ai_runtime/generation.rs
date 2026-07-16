use serde::{Deserialize, Serialize};

use super::lifecycle::LocalAiGenerationState;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiChatGenerationResult {
    pub local_ai_result_id: String,
    pub runtime_reference_id: String,
    pub provider_id: String,
    pub model_id: String,
    pub model_reference: String,
    pub generation_state: LocalAiGenerationState,
    pub output_text: Option<String>,
    pub prompt_char_count: u64,
    pub max_output_tokens: u32,
    pub timeout_ms: u64,
    pub duration_ms: u64,
    pub exit_code: Option<i32>,
    pub stderr_byte_size: u64,
    pub unavailable_reason: Option<String>,
}
