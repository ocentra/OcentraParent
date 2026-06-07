use ocentra_parent_agent_protocol::{
    LocalAiChatGenerationResult, LocalAiGenerationState, SCREEN_CATEGORY_UNKNOWN,
    SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE,
    SCREEN_SERVICE_ANALYSIS_MODEL_ID, SCREEN_SERVICE_ANALYSIS_RUNTIME_REF,
    SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID, SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE,
    SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION, SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
};

use super::super::adapter::parsed_generation_output;

pub(super) struct ScreenAiAnalysisParsedFields {
    pub(super) summary: String,
    pub(super) category: String,
    pub(super) confidence: f64,
    pub(super) policy_eligible: bool,
    pub(super) provider_kind: String,
    pub(super) model_runtime_ref: String,
    pub(super) model_id: String,
    pub(super) template_version: String,
    pub(super) ocr_text_snippets: Vec<String>,
    pub(super) redaction_notes: Vec<String>,
}

pub(super) fn parsed_fields_from_generation(
    generation: &LocalAiChatGenerationResult,
) -> ScreenAiAnalysisParsedFields {
    match parsed_generation_output(generation) {
        Some(output) => ScreenAiAnalysisParsedFields {
            summary: output.summary,
            category: output.primary_category,
            confidence: output.confidence,
            policy_eligible: output.policy_eligible
                && output.confidence >= SCREEN_POLICY_CONFIDENCE_READY,
            provider_kind: output.provider_kind,
            model_runtime_ref: output.model_runtime_ref,
            model_id: output.model_id,
            template_version: output.prompt_or_template_version,
            ocr_text_snippets: output.ocr_text_snippets,
            redaction_notes: output.redaction_notes,
        },
        None if generation.generation_state == LocalAiGenerationState::Complete => {
            unavailable_fields(SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID)
        }
        None => unavailable_fields(SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE),
    }
}

fn unavailable_fields(summary: &str) -> ScreenAiAnalysisParsedFields {
    ScreenAiAnalysisParsedFields {
        summary: summary.to_string(),
        category: SCREEN_CATEGORY_UNKNOWN.to_string(),
        confidence: SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
        policy_eligible: false,
        provider_kind: SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE.to_string(),
        model_runtime_ref: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
        model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
        template_version: SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION.to_string(),
        ocr_text_snippets: Vec::new(),
        redaction_notes: Vec::new(),
    }
}
