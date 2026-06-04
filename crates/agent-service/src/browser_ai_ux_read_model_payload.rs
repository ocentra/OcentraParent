use ocentra_parent_agent_protocol::{
    constants, BrowserAiUxReadModel, BrowserAiUxReadModelRow, LogFieldValue, LogFields,
    BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn modeled_browser_ai_ux_read_model(generated_at: String) -> BrowserAiUxReadModel {
    BrowserAiUxReadModel {
        schema_version: BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
        generated_at,
        custody_label: constants::browser_ai_ux_read_model::CHILD_DEVICE_SERVICE_MODELED
            .to_string(),
        capability_status:
            constants::browser_ai_ux_read_model::STATUS_SERVICE_BACKED_MANUAL_REQUIRED
                .to_string(),
        returned: 2,
        latest_event_id: Some(constants::browser_ai_ux_read_model::ROW_CHECKING_ID.to_string()),
        rows: vec![checking_row(), manual_required_row()],
    }
}

pub fn browser_ai_ux_read_model_payload(read_model: &BrowserAiUxReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(latest_row_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &BrowserAiUxReadModel) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::BROWSER_AI_UX_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model)
                    .expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn latest_row_pairs(row: Option<&BrowserAiUxReadModelRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(join_evidence_ids(row)),
        ),
        (
            constants::field::CHILD_DELIVERY_STATE,
            optional_string(row.map(|value| &value.child_delivery_state)),
        ),
        (
            constants::field::BROWSER_AI_CHILD_STATE,
            optional_string(row.map(|value| &value.child_state)),
        ),
        (
            constants::field::BROWSER_AI_PARENT_EXPLANATION_ID,
            optional_string(row.map(|value| &value.parent_explanation_id)),
        ),
    ]
}

fn checking_row() -> BrowserAiUxReadModelRow {
    BrowserAiUxReadModelRow {
        schema_version: BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
        row_id: constants::browser_ai_ux_read_model::ROW_CHECKING_ID.to_string(),
        source_evidence_ids: vec![
            constants::browser_ai_ux_read_model::EVIDENCE_YOUTUBE_VIDEO.to_string(),
        ],
        child_snapshot_id: constants::browser_ai_ux_read_model::CHILD_SNAPSHOT_YOUTUBE_VIDEO
            .to_string(),
        child_state: constants::browser_ai_ux_read_model::CHILD_STATE_CHECKING.to_string(),
        child_primary_text_token: constants::browser_ai_ux_read_model::CHILD_TOKEN_CHECKING
            .to_string(),
        child_delivery_state:
            constants::browser_ai_ux_read_model::CHILD_DELIVERY_CHECKING_HOLD_RENDERED
                .to_string(),
        adapter_proof_ref: Some(
            constants::browser_ai_ux_read_model::ADAPTER_PROOF_CHECKING_PAGE.to_string(),
        ),
        parent_explanation_id:
            constants::browser_ai_ux_read_model::PARENT_EXPLANATION_YOUTUBE_VIDEO.to_string(),
        parent_explanation_state: constants::browser_ai_ux_read_model::PARENT_STATE_PREVIEW
            .to_string(),
        parent_title_text_token: constants::browser_ai_ux_read_model::PARENT_TOKEN_TITLE
            .to_string(),
        explanation_audit_refs: vec![
            constants::browser_ai_ux_read_model::AUDIT_YOUTUBE_VIDEO.to_string(),
        ],
        model_runtime_visible: true,
        policy_rule_visible: true,
        action_visible: true,
        child_experience_visible: true,
        degraded_state_visible: false,
        manual_fallback_visible: false,
        runtime_delivery_claimed: false,
        rendered_ui_claimed: false,
        direct_enforcement_claimed: false,
    }
}

fn manual_required_row() -> BrowserAiUxReadModelRow {
    BrowserAiUxReadModelRow {
        schema_version: BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
        row_id: constants::browser_ai_ux_read_model::ROW_MANUAL_REQUIRED_ID.to_string(),
        source_evidence_ids: vec![
            constants::browser_ai_ux_read_model::EVIDENCE_GENERIC_VIDEO.to_string(),
        ],
        child_snapshot_id: constants::browser_ai_ux_read_model::CHILD_SNAPSHOT_GENERIC_VIDEO
            .to_string(),
        child_state: constants::browser_ai_ux_read_model::CHILD_STATE_MANUAL_REQUIRED
            .to_string(),
        child_primary_text_token: constants::browser_ai_ux_read_model::CHILD_TOKEN_MANUAL
            .to_string(),
        child_delivery_state: constants::browser_ai_ux_read_model::CHILD_DELIVERY_PORTAL_ROW_ONLY
            .to_string(),
        adapter_proof_ref: None,
        parent_explanation_id:
            constants::browser_ai_ux_read_model::PARENT_EXPLANATION_GENERIC_VIDEO.to_string(),
        parent_explanation_state: constants::browser_ai_ux_read_model::PARENT_STATE_MANUAL_REQUIRED
            .to_string(),
        parent_title_text_token: constants::browser_ai_ux_read_model::PARENT_TOKEN_DEGRADED
            .to_string(),
        explanation_audit_refs: vec![
            constants::browser_ai_ux_read_model::AUDIT_GENERIC_VIDEO.to_string(),
        ],
        model_runtime_visible: true,
        policy_rule_visible: true,
        action_visible: true,
        child_experience_visible: true,
        degraded_state_visible: true,
        manual_fallback_visible: true,
        runtime_delivery_claimed: false,
        rendered_ui_claimed: false,
        direct_enforcement_claimed: false,
    }
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn join_evidence_ids(row: Option<&BrowserAiUxReadModelRow>) -> String {
    let separator = constants::delimiter::LIST.to_string();
    row.map(|value| value.source_evidence_ids.join(&separator))
        .unwrap_or_default()
}
