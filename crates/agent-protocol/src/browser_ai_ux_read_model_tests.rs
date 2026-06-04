use super::{
    constants, AgentCommandName, AgentEventName, BrowserAiUxReadModel,
    BrowserAiUxReadModelRow, BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
};

#[test]
fn browser_ai_ux_read_model_serializes_without_runtime_or_ui_claims() {
    let read_model = BrowserAiUxReadModel {
        schema_version: BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
        generated_at: constants::browser_ai_ux_read_model::GENERATED_AT.to_string(),
        custody_label: constants::browser_ai_ux_read_model::CHILD_DEVICE_SERVICE_MODELED
            .to_string(),
        capability_status:
            constants::browser_ai_ux_read_model::STATUS_SERVICE_BACKED_MANUAL_REQUIRED
                .to_string(),
        returned: 2,
        latest_event_id: Some(constants::browser_ai_ux_read_model::ROW_CHECKING_ID.to_string()),
        rows: vec![checking_row(), manual_required_row()],
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["schemaVersion"],
        BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION
    );
    assert_eq!(
        serialized["custodyLabel"],
        constants::browser_ai_ux_read_model::CHILD_DEVICE_SERVICE_MODELED
    );
    assert_eq!(serialized["returned"], 2);
    assert_eq!(
        serialized["rows"][0]["childDeliveryState"],
        constants::browser_ai_ux_read_model::CHILD_DELIVERY_CHECKING_HOLD_RENDERED
    );
    assert_eq!(serialized["rows"][0]["runtimeDeliveryClaimed"], false);
    assert_eq!(serialized["rows"][0]["renderedUiClaimed"], false);
    assert_eq!(serialized["rows"][0]["directEnforcementClaimed"], false);
}

#[test]
fn browser_ai_ux_read_model_command_and_event_names_match_typescript_protocol() {
    let command = serde_json::to_value(AgentCommandName::AgentBrowserAiUxReadModelGet)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event = serde_json::to_value(AgentEventName::AgentBrowserAiUxReadModelReported)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(command, constants::browser_ai_ux_read_model::COMMAND_GET);
    assert_eq!(event, constants::browser_ai_ux_read_model::EVENT_REPORTED);
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
