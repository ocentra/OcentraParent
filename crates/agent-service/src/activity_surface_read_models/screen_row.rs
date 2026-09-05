use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivityScreenReadModelRow,
};
use ocentra_parent_agent_protocol::constants;

pub(crate) fn activity_screen_row_from_result(
    result: ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult,
) -> ActivityScreenReadModelRow {
    ActivityScreenReadModelRow {
        row_id: result.screen_analysis_result_id,
        label: result.summary,
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        state: ActivityReadModelState::Ready,
        total_ms: 0,
        foreground_ms: 0,
        background_ms: 0,
        capture_reason: result.capture_reason,
        capture_scope: result.capture_scope,
        capability_status: result.capability_status,
        queue_job_id: result.queue_job_id,
        model_runtime_ref: result.model_runtime_ref,
        model_id: result.model_id,
        provider_kind: result.provider_kind,
        prompt_or_template_version: result.prompt_or_template_version,
        primary_category: result.primary_category,
        confidence: result.confidence,
        image_deletion_state: result.image_deletion_state,
        raw_image_retained: result.raw_image_retained,
        policy_eligible: result.policy_eligible,
        image_digest: result.image_digest,
        custody_state: result.custody_state,
        evidence: result.source_evidence_refs,
        policy_decision_ref: result.policy_decision_ref,
        policy_action: result.policy_action,
        policy_reason_codes: result.policy_reason_codes,
        parent_rule_refs: result.parent_rule_refs,
        local_model_runtime_refs: result.local_model_runtime_refs,
        parent_explanation_refs: result.parent_explanation_refs,
        explanation_reasons: result.explanation_reasons,
        deletion_reasons: result.deletion_reasons,
        ocr_text_snippets: result.ocr_text_snippets,
        redaction_notes: result.redaction_notes,
    }
}
