use crate::{
    SocialAlertReportClaimBoundaries, SocialAlertReportReadModelSnapshot,
    SOCIAL_ALERT_REPORT_CAPABILITY_READY, SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID,
    SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED, SOCIAL_ALERT_REPORT_FAMILY_ID,
    SOCIAL_ALERT_REPORT_SCHEMA_VERSION,
};

#[test]
fn social_alert_report_snapshot_serializes_no_claim_boundaries() {
    let snapshot = SocialAlertReportReadModelSnapshot {
        schema_version: SOCIAL_ALERT_REPORT_SCHEMA_VERSION.to_string(),
        family_id: SOCIAL_ALERT_REPORT_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_ALERT_REPORT_CHILD_PROFILE_ID.to_string(),
        generated_at: SOCIAL_ALERT_REPORT_CAPABILITY_READY.to_string(),
        intents: Vec::new(),
        claim_boundaries: SocialAlertReportClaimBoundaries {
            provider_delivery: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            report_delivery: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            parent_notification_ui: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            final_policy_decision: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
            enforcement: SOCIAL_ALERT_REPORT_CLAIM_NOT_CLAIMED.to_string(),
        },
    };

    let json =
        serde_json::to_string(&snapshot).expect(crate::constants::error::AGENT_EVENT_SERIALIZES);

    assert!(json.contains("social-alert-report-read-model"));
    assert!(json.contains("providerDelivery"));
    assert!(json.contains("not-claimed"));
}
