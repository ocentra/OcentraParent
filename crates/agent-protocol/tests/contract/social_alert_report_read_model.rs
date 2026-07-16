use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::social_alert_report_read_model::{
    SocialAlertReportClaimBoundaries, SocialAlertReportProviderStatusRow,
    SocialAlertReportReadModelSnapshot, SOCIAL_ALERT_REPORT_CAPABILITY_READY,
    SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID, SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED,
    SOCIAL_ALERT_REPORT_FAMILY_ID, SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_MANUAL,
    SOCIAL_ALERT_REPORT_PROVIDER_DELIVERY_NOT_OBSERVED,
    SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL,
    SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED,
    SOCIAL_ALERT_REPORT_PROVIDER_STATUS_PROOF_MANUAL_ACTION, SOCIAL_ALERT_REPORT_SCHEMA_VERSION,
};

#[test]
fn social_alert_report_snapshot_serializes_no_claim_boundaries() {
    let snapshot = SocialAlertReportReadModelSnapshot {
        schema_version: SOCIAL_ALERT_REPORT_SCHEMA_VERSION.to_string(),
        family_id: SOCIAL_ALERT_REPORT_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID.to_string(),
        generated_at: SOCIAL_ALERT_REPORT_CAPABILITY_READY.to_string(),
        intents: Vec::new(),
        provider_status_rows: vec![SocialAlertReportProviderStatusRow {
            status_entry_id: SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL.to_string(),
            source_intent_ref: SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED.to_string(),
            source_preflight_status: SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED
                .to_string(),
            provider_status: SOCIAL_ALERT_REPORT_PROVIDER_STATUS_MANUAL_REQUIRED.to_string(),
            status_proof_state: SOCIAL_ALERT_REPORT_PROVIDER_STATUS_PROOF_MANUAL_ACTION.to_string(),
            delivery_claim_state: SOCIAL_ALERT_REPORT_PROVIDER_DELIVERY_NOT_OBSERVED.to_string(),
            provider_attempt_ref: SOCIAL_ALERT_REPORT_PROVIDER_ATTEMPT_MANUAL.to_string(),
            readiness_refs: vec![SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED.to_string()],
            provider_receipt_refs: Vec::new(),
            manual_proof_requirements: vec![
                SOCIAL_ALERT_REPORT_PROVIDER_PREFLIGHT_MANUAL_REQUIRED.to_string()
            ],
            provider_delivery_implemented: false,
            provider_delivery_observed: false,
            delivered_notification_claimed: false,
            sensitive_provider_payload_claimed: false,
            provider_stores_child_evidence_claimed: false,
            last_checked_at: SOCIAL_ALERT_REPORT_CAPABILITY_READY.to_string(),
        }],
        claim_boundaries: SocialAlertReportClaimBoundaries {
            provider_delivery: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            report_delivery: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            parent_notification_ui: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            final_policy_decision: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            enforcement: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
        },
    };

    let json = serde_json::to_value(&snapshot)
        .expect_value(ocentra_parent_agent_protocol::constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(json["familyId"], SOCIAL_ALERT_REPORT_FAMILY_ID);
    assert_eq!(
        json["claimBoundaries"]["providerDelivery"],
        SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED
    );
    assert_eq!(
        json["providerStatusRows"][0]["providerReceiptRefs"],
        serde_json::json!([])
    );
    assert_eq!(
        json["providerStatusRows"][0]["deliveryClaimState"],
        SOCIAL_ALERT_REPORT_PROVIDER_DELIVERY_NOT_OBSERVED
    );
    assert_eq!(
        json["claimBoundaries"]["finalPolicyDecision"],
        SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED
    );
}
