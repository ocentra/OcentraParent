use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_PROVIDER_LOCAL_OCR, SCREEN_PROVIDER_LOCAL_VISION, SCREEN_SERVICE_ANALYSIS_MODEL_ID,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_REF, SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION,
};
use serde_json::Value;

use super::super::{
    adapter_redaction::apply_service_ocr_redaction, config::ScreenOcrRedactionPolicy,
};
use super::ScreenAiAnalysisAdapterOutput;

struct ParsedGenerationFields {
    summary: String,
    primary_category: String,
    confidence: f64,
    policy_eligible: bool,
    provider_kind: String,
    model_runtime_ref: String,
    model_id: String,
    prompt_or_template_version: String,
    ocr_text_snippets: Vec<String>,
    redaction_notes: Vec<String>,
}

pub(super) fn parsed_generation_output_with_policy(
    generation: &LocalAiChatGenerationResult,
    policy: &ScreenOcrRedactionPolicy,
) -> Option<ScreenAiAnalysisAdapterOutput> {
    if generation.generation_state != LocalAiGenerationState::Complete {
        return None;
    }
    let output = generation.output_text.as_ref()?;
    let parsed = serde_json::from_str::<Value>(output).ok()?;
    let parsed_fields = parsed_generation_fields(&parsed)?;
    Some(apply_service_ocr_redaction(
        ScreenAiAnalysisAdapterOutput {
            summary: parsed_fields.summary,
            primary_category: parsed_fields.primary_category,
            confidence: parsed_fields.confidence,
            policy_eligible: parsed_fields.policy_eligible,
            provider_kind: parsed_fields.provider_kind,
            model_runtime_ref: parsed_fields.model_runtime_ref,
            model_id: parsed_fields.model_id,
            prompt_or_template_version: parsed_fields.prompt_or_template_version,
            ocr_text_snippets: parsed_fields.ocr_text_snippets,
            redaction_notes: parsed_fields.redaction_notes,
        },
        policy,
    ))
}

fn parsed_generation_fields(parsed: &Value) -> Option<ParsedGenerationFields> {
    let required_string = |field: &str| {
        let value = parsed.get(field)?.as_str()?.trim();
        if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        }
    };
    let optional_string = |field: &str| {
        parsed.get(field).and_then(|field_value| {
            let text = field_value.as_str()?.trim();
            if text.is_empty() {
                None
            } else {
                Some(text.to_string())
            }
        })
    };
    let optional_string_array = |field: &'static str| {
        parsed
            .get(field)
            .and_then(Value::as_array)
            .map(|values| {
                values
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::trim)
                    .filter(|item| !item.is_empty())
                    .map(ToOwned::to_owned)
                    .collect()
            })
            .unwrap_or_default()
    };
    let provider_kind = optional_string(constants::field::SCREEN_PROVIDER_KIND)
        .unwrap_or_else(|| SCREEN_PROVIDER_LOCAL_VISION.to_string());
    if provider_kind != SCREEN_PROVIDER_LOCAL_VISION && provider_kind != SCREEN_PROVIDER_LOCAL_OCR {
        return None;
    }
    let confidence = parsed.get(constants::field::SCREEN_CONFIDENCE)?.as_f64()?;
    if !(0.0..=1.0).contains(&confidence) {
        return None;
    }
    Some(ParsedGenerationFields {
        summary: required_string(constants::field::SCREEN_SUMMARY)?,
        primary_category: required_string(constants::field::SCREEN_PRIMARY_CATEGORY)?,
        confidence,
        policy_eligible: parsed
            .get(constants::field::SCREEN_POLICY_ELIGIBLE)?
            .as_bool()?,
        provider_kind,
        model_runtime_ref: optional_string(constants::field::SCREEN_MODEL_RUNTIME_REF)
            .unwrap_or_else(|| SCREEN_SERVICE_ANALYSIS_RUNTIME_REF.to_string()),
        model_id: optional_string(constants::field::SCREEN_MODEL_ID)
            .unwrap_or_else(|| SCREEN_SERVICE_ANALYSIS_MODEL_ID.to_string()),
        prompt_or_template_version: optional_string(constants::field::SCREEN_TEMPLATE_VERSION)
            .unwrap_or_else(|| SCREEN_SERVICE_ANALYSIS_TEMPLATE_VERSION.to_string()),
        ocr_text_snippets: optional_string_array(constants::field::SCREEN_OCR_TEXT_SNIPPETS),
        redaction_notes: optional_string_array(constants::field::SCREEN_REDACTION_NOTES),
    })
}
