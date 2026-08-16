use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::SocialAuditExplanationSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_ACCOUNT_APPROVAL;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE;

use super::social_audit_explanation_read_model_payload::{
    social_audit_explanation_read_model_from_service, social_audit_explanation_read_model_payload,
};
use crate::log_payload::{payload_json, payload_number};

#[test]
fn social_audit_explanation_payload_reports_six_honest_service_rows() {
    let read_model = social_audit_explanation_read_model_from_service();
    let payload = social_audit_explanation_read_model_payload(&read_model);
    let decoded: SocialAuditExplanationSnapshot = payload_json(
        &payload,
        constants::field::BROWSER_SOCIAL_AUDIT_EXPLANATION_READ_MODEL,
    );

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
        payload_number(&payload, constants::field::RETURNED),
        decoded.entries.len() as f64
    );
}
