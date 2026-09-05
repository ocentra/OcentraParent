use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CATEGORY_UNKNOWN;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MODEL_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_RUNTIME_REF;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE;

use super::super::{
    adapter::parsed_generation_output_with_policy, config::ScreenOcrRedactionPolicy,
};

pub(super) struct ScreenAiAnalysisParsedFields {
    pub(super) summary: String,
    pub(super) category: String,
    pub(super) confidence: f64,
    pub(super) provider_kind: String,
    pub(super) model_runtime_ref: String,
    pub(super) model_id: String,
    pub(super) template_version: String,
    pub(super) ocr_text_snippets: Vec<String>,
    pub(super) redaction_notes: Vec<String>,
}

pub(super) fn parsed_fields_from_generation(
    generation: &LocalAiChatGenerationResult,
    policy: &ScreenOcrRedactionPolicy,
) -> ScreenAiAnalysisParsedFields {
    match parsed_generation_output_with_policy(generation, policy) {
        Some(output) => ScreenAiAnalysisParsedFields {
            summary: output.summary,
            category: output.primary_category,
            confidence: output.confidence,
            provider_kind: output.provider_kind,
            model_runtime_ref: output.model_runtime_ref,
            model_id: output.model_id,
            template_version: output.prompt_or_template_version,
            ocr_text_snippets: output.ocr_text_snippets,
            redaction_notes: output.redaction_notes,
        },
        None if generation.generation_state == LocalAiGenerationState::Complete => {
            ScreenAiAnalysisParsedFields {
                summary: SCREEN_SERVICE_ANALYSIS_SUMMARY_INVALID.to_string(),
                category: SCREEN_CATEGORY_UNKNOWN.to_string(),
                confidence: SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
                provider_kind: SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE.to_string(),
                model_runtime_ref: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
                model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
                template_version: SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION.to_string(),
                ocr_text_snippets: Vec::new(),
                redaction_notes: Vec::new(),
            }
        }
        None => ScreenAiAnalysisParsedFields {
            summary: SCREEN_SERVICE_ANALYSIS_SUMMARY_UNAVAILABLE.to_string(),
            category: SCREEN_CATEGORY_UNKNOWN.to_string(),
            confidence: SCREEN_SERVICE_UNAVAILABLE_CONFIDENCE,
            provider_kind: SCREEN_PROVIDER_LOCAL_VISION_UNAVAILABLE.to_string(),
            model_runtime_ref: SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string(),
            model_id: SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string(),
            template_version: SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION.to_string(),
            ocr_text_snippets: Vec::new(),
            redaction_notes: Vec::new(),
        },
    }
}
