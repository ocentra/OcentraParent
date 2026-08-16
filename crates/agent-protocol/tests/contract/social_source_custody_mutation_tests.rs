use super::{
    constants, AgentCommandName, AgentEventName, SocialSourceCustodyMutationSnapshot,
    SocialSourceCustodySettingsSnapshot, SOCIAL_SOURCE_CUSTODY_AUDIT_REF,
    SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF, SOCIAL_SOURCE_CUSTODY_MUTATION_SCHEMA_VERSION,
    SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn social_source_custody_mutation_command_event_and_snapshot_serialize() {
    assert_eq!(
        serde_json::to_value(AgentCommandName::AgentBrowserSocialSourceCustodyMutationApply)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        "agent.browser.social-source-custody.mutation.apply"
    );
    assert_eq!(
        serde_json::to_value(AgentEventName::AgentBrowserSocialSourceCustodyMutationApplied)
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        "agent.browser.social-source-custody.mutation.applied"
    );

    let snapshot = SocialSourceCustodyMutationSnapshot {
        schema_version: SOCIAL_SOURCE_CUSTODY_MUTATION_SCHEMA_VERSION.to_string(),
        mutation_id: "mutation".to_string(),
        requested_at: "2026-06-07T03:56:00Z".to_string(),
        applied_at: "2026-06-07T03:56:01Z".to_string(),
        mutation_state: SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED.to_string(),
        settings: SocialSourceCustodySettingsSnapshot {
            schema_version: 1,
            settings_id: "settings".to_string(),
            generated_at: "2026-06-07T03:56:00Z".to_string(),
            child_profile_ref: "child".to_string(),
            device_id: "device".to_string(),
            source_privacy_evidence_ids: vec!["privacy".to_string()],
            evidence_refs: vec![SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF.to_string()],
            setting_scope: "managed-browser-social-route".to_string(),
            permission_state: "enabled".to_string(),
            custody_mode: "local-redacted-refs-only".to_string(),
            retention_mode: "redacted-ref-journal-only".to_string(),
            permitted_downstream_uses: vec!["ai-candidate-input".to_string()],
            disabled_use_reasons: vec![],
            parent_review_refs: vec![],
            connector_authorization_refs: vec![],
            manual_proof_requirements: vec![],
            no_claim_labels: vec!["enforcement-not-claimed".to_string()],
            raw_message_content_allowed: false,
            raw_video_content_allowed: false,
            screenshot_custody_allowed: false,
            connector_token_stored: false,
            connector_api_called: false,
            runtime_settings_ui_claimed: false,
            runtime_custody_mutation_claimed: false,
            final_policy_decision_claimed: false,
            enforcement_claimed: false,
        },
        evidence_refs: vec![SOCIAL_SOURCE_CUSTODY_EVIDENCE_REF.to_string()],
        audit_refs: vec![SOCIAL_SOURCE_CUSTODY_AUDIT_REF.to_string()],
        service_mutation_executed: true,
        runtime_custody_mutation_applied: true,
        raw_content_custody_claimed: false,
        connector_api_called: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
        product_claim_ready: false,
    };
    let value =
        serde_json::to_value(snapshot).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        value["schemaVersion"],
        SOCIAL_SOURCE_CUSTODY_MUTATION_SCHEMA_VERSION
    );
    assert_eq!(
        value["mutationState"],
        SOCIAL_SOURCE_CUSTODY_MUTATION_STATE_APPLIED
    );
    assert_eq!(value["serviceMutationExecuted"], true);
    assert_eq!(value["finalPolicyDecisionClaimed"], false);
    assert_eq!(value["enforcementClaimed"], false);
}
