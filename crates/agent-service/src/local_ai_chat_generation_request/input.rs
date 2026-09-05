#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LocalAiChatGenerationRequest {
    pub(crate) model_id: String,
    pub(crate) prompt: String,
    pub(crate) max_output_tokens: u32,
    pub(crate) timeout_ms: u64,
}
