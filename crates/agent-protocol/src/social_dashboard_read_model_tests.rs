use super::{
    constants, SocialDashboardClaimBoundaries, SocialDashboardPanel, SocialDashboardUxSnapshot,
    SOCIAL_DASHBOARD_ACTION_OPEN_PARENT_APPROVAL, SOCIAL_DASHBOARD_CHILD_PROFILE_ID,
    SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED, SOCIAL_DASHBOARD_FAMILY_ID,
    SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE, SOCIAL_DASHBOARD_REASON_PARENT_REVIEW_NEEDED,
    SOCIAL_DASHBOARD_SCHEMA_VERSION, SOCIAL_DASHBOARD_SEVERITY_INFO,
    SOCIAL_DASHBOARD_STATUS_READY_FOR_REVIEW,
};

#[test]
fn social_dashboard_snapshot_serializes_without_runtime_claims() {
    let snapshot = SocialDashboardUxSnapshot {
        schema_version: SOCIAL_DASHBOARD_SCHEMA_VERSION.to_string(),
        family_id: SOCIAL_DASHBOARD_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_DASHBOARD_CHILD_PROFILE_ID.to_string(),
        generated_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        panels: vec![SocialDashboardPanel {
            panel_id: SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE.to_string(),
            panel_kind: SOCIAL_DASHBOARD_PANEL_ACCOUNT_APPROVAL_QUEUE.to_string(),
            status: SOCIAL_DASHBOARD_STATUS_READY_FOR_REVIEW.to_string(),
            primary_action: SOCIAL_DASHBOARD_ACTION_OPEN_PARENT_APPROVAL.to_string(),
            severity: SOCIAL_DASHBOARD_SEVERITY_INFO.to_string(),
            sort_order: 0,
            source_evidence_refs: vec![
                constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
            ],
            reasons: vec![SOCIAL_DASHBOARD_REASON_PARENT_REVIEW_NEEDED.to_string()],
            rendered_ui_claimed: false,
            notification_claimed: false,
            runtime_data_fetch_claimed: false,
            policy_decision_claimed: false,
            native_app_control_claimed: false,
            connector_authorization_claimed: false,
            enforcement_claimed: false,
        }],
        claim_boundaries: not_claimed_boundaries(),
    };

    let serialized =
        serde_json::to_value(snapshot).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], SOCIAL_DASHBOARD_SCHEMA_VERSION);
    assert!(serialized["custodyLabel"].is_null());
    assert!(serialized["capabilityStatus"].is_null());
    assert!(serialized["returned"].is_null());
    assert_eq!(serialized["panels"][0]["renderedUiClaimed"], false);
    assert_eq!(serialized["panels"][0]["enforcementClaimed"], false);
    assert_eq!(
        serialized["claimBoundaries"]["policyDecision"],
        SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED
    );
}

fn not_claimed_boundaries() -> SocialDashboardClaimBoundaries {
    SocialDashboardClaimBoundaries {
        rendered_portal_ui: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        notification_delivery: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        runtime_data_fetch: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        policy_decision: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        native_app_control: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        connector_authorization: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
        enforcement: SOCIAL_DASHBOARD_CLAIM_NOT_CLAIMED.to_string(),
    }
}
