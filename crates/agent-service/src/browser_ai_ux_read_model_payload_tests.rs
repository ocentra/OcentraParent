use ocentra_parent_agent_protocol::{
    constants, BrowserAiUxReadModel, LogFieldValue, BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION,
};

use super::browser_ai_ux_read_model_payload::{
    browser_ai_ux_read_model_payload, modeled_browser_ai_ux_read_model,
};

#[test]
fn browser_ai_ux_read_model_payload_contains_contract_json_and_no_claim_summary() {
    let read_model = modeled_browser_ai_ux_read_model(
        constants::browser_ai_ux_read_model::GENERATED_AT.to_string(),
    );

    let payload = browser_ai_ux_read_model_payload(&read_model);
    let read_model_json = string_payload(&payload, constants::field::BROWSER_AI_UX_READ_MODEL);
    let decoded: BrowserAiUxReadModel =
        serde_json::from_str(read_model_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        decoded.schema_version,
        BROWSER_AI_UX_READ_MODEL_SCHEMA_VERSION
    );
    assert_eq!(decoded.returned, 2);
    assert_eq!(
        decoded.rows[0].adapter_proof_ref.as_deref(),
        Some(constants::browser_ai_ux_read_model::ADAPTER_PROOF_CHECKING_PAGE)
    );
    assert_eq!(decoded.rows[1].manual_fallback_visible, true);
    assert_eq!(
        string_payload(&payload, constants::field::EVIDENCE_REFERENCE_IDS),
        constants::browser_ai_ux_read_model::EVIDENCE_YOUTUBE_VIDEO
    );
    assert_eq!(
        string_payload(&payload, constants::field::BROWSER_AI_CHILD_STATE),
        constants::browser_ai_ux_read_model::CHILD_STATE_CHECKING
    );
    assert_eq!(
        string_payload(&payload, constants::field::BROWSER_AI_PARENT_EXPLANATION_ID),
        constants::browser_ai_ux_read_model::PARENT_EXPLANATION_YOUTUBE_VIDEO
    );
}

fn string_payload<'a>(payload: &'a ocentra_parent_agent_protocol::LogFields, key: &str) -> &'a str {
    match payload.get(key) {
        Some(LogFieldValue::String(value)) => value.as_str(),
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
