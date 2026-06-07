use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, SocialAuditExplanationSnapshot,
    SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED, SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL,
    SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE,
};

use super::social_audit_explanation_read_model_payload::{
    social_audit_explanation_read_model_from_service, social_audit_explanation_read_model_payload,
};

#[test]
fn social_audit_explanation_payload_reports_six_honest_service_rows() {
    let read_model = social_audit_explanation_read_model_from_service();
    let payload = social_audit_explanation_read_model_payload(&read_model);
    let read_model_json = string_payload(
        &payload,
        constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL,
    );
    let decoded: SocialAuditExplanationSnapshot =
        serde_json::from_str(read_model_json).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        decoded.schema_version,
        SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION
    );
    assert_eq!(decoded.entries.len(), 6);
    assert_eq!(
        decoded.entries[0].subject_kind,
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL
    );
    assert_eq!(
        decoded.entries[1].subject_kind,
        SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE
    );
    assert!(!decoded.entries[1].runtime_audit_store_claimed);
    assert!(!decoded.entries[1].final_policy_decision_claimed);
    assert!(!decoded.entries[1].enforcement_claimed);
    assert_eq!(
        decoded.claim_boundaries.enforcement,
        SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED
    );
    assert_eq!(
        number_payload(&payload, constants::field::RETURNED),
        decoded.entries.len() as f64
    );
}

fn string_payload<'a>(
    payload: &'a ocentra_parent_agent_protocol::LogFields,
    field: &str,
) -> &'a str {
    match &payload[field] {
        LogFieldValue::String(text) => text,
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}

fn number_payload(payload: &ocentra_parent_agent_protocol::LogFields, field: &str) -> f64 {
    match &payload[field] {
        LogFieldValue::Number(value) => *value,
        _ => std::panic::panic_any(constants::error::AGENT_EVENT_SERIALIZES),
    }
}
