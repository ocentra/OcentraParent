use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::SocialAuditExplanationClaimBoundaries;
use ocentra_parent_agent_protocol::SocialAuditExplanationEntry;
use ocentra_parent_agent_protocol::SocialAuditExplanationEvidenceLink;
use ocentra_parent_agent_protocol::SocialAuditExplanationSnapshot;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_ACTION_WARN;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_CHILD_PROFILE_ID;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_FAMILY_ID;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SNAPSHOT_ID;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT;
use ocentra_parent_agent_protocol::SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE;

#[test]
fn social_audit_explanation_snapshot_serializes_without_runtime_claims() {
    let snapshot = SocialAuditExplanationSnapshot {
        schema_version: SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION.to_string(),
        snapshot_id: SOCIAL_AUDIT_EXPLANATION_SNAPSHOT_ID.to_string(),
        family_id: SOCIAL_AUDIT_EXPLANATION_FAMILY_ID.to_string(),
        child_profile_id: SOCIAL_AUDIT_EXPLANATION_CHILD_PROFILE_ID.to_string(),
        captured_at: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_OBSERVED_AT
            .to_string(),
        entries: vec![social_audit_explanation_entry()],
        claim_boundaries: not_claimed_boundaries(),
    };

    let serialized =
        serde_json::to_value(snapshot).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["schemaVersion"],
        SOCIAL_AUDIT_EXPLANATION_SCHEMA_VERSION
    );
    assert_eq!(
        serialized["entries"][0]["evidenceLinks"][0]["evidenceKind"],
        SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE
    );
    assert_eq!(
        serialized["claimBoundaries"]["enforcement"],
        SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED
    );
    assert_eq!(serialized["entries"][0]["runtimeAuditStoreClaimed"], false);
}

fn not_claimed_boundaries() -> SocialAuditExplanationClaimBoundaries {
    SocialAuditExplanationClaimBoundaries {
        runtime_audit_store: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        rendered_explanation_ui: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        notification_delivery: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        raw_account_video_message_content: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        connector_authorization: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        native_app_control: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        final_policy_decision: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
        enforcement: SOCIAL_AUDIT_EXPLANATION_CLAIM_NOT_CLAIMED.to_string(),
    }
}

fn social_audit_explanation_entry() -> SocialAuditExplanationEntry {
    SocialAuditExplanationEntry {
        event_id: SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE.to_string(),
        subject_kind: SOCIAL_AUDIT_EXPLANATION_SUBJECT_FEED_VIDEO_GATE.to_string(),
        status: SOCIAL_AUDIT_EXPLANATION_STATUS_READY_FOR_PARENT.to_string(),
        decision_state: SOCIAL_AUDIT_EXPLANATION_DECISION_CANDIDATE_ONLY.to_string(),
        audience: SOCIAL_AUDIT_EXPLANATION_AUDIENCE_PARENT.to_string(),
        policy_version_ref: Some(SOCIAL_AUDIT_EXPLANATION_POLICY_VERSION.to_string()),
        action_candidate: SOCIAL_AUDIT_EXPLANATION_ACTION_WARN.to_string(),
        policy_reason_codes: vec![
            SOCIAL_AUDIT_EXPLANATION_POLICY_REASON_SOCIAL_RISK_HIGH.to_string()
        ],
        explanation_reasons: vec![SOCIAL_AUDIT_EXPLANATION_REASON_EVIDENCE_LINKED.to_string()],
        evidence_links: social_audit_explanation_evidence_links(),
        audit_refs: vec![
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ],
        parent_approval_request_ref: None,
        parent_approval_decision_ref: None,
        decision_memory_ref: None,
        connector_boundary_ref: None,
        native_capability_ref: None,
        manual_required_ref: None,
        runtime_audit_store_claimed: false,
        rendered_explanation_ui_claimed: false,
        notification_delivered_claimed: false,
        raw_account_data_included: false,
        raw_video_content_included: false,
        raw_message_content_included: false,
        connector_authorization_claimed: false,
        native_app_control_claimed: false,
        final_policy_decision_claimed: false,
        enforcement_claimed: false,
    }
}

fn social_audit_explanation_evidence_links() -> Vec<SocialAuditExplanationEvidenceLink> {
    vec![
        SocialAuditExplanationEvidenceLink {
            evidence_kind: SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE.to_string(),
            evidence_ref: format!(
                "parent-evidence-{}",
                SOCIAL_AUDIT_EXPLANATION_EVIDENCE_ROUTE_EVIDENCE
            ),
        },
        SocialAuditExplanationEvidenceLink {
            evidence_kind: SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE.to_string(),
            evidence_ref: format!(
                "parent-evidence-{}",
                SOCIAL_AUDIT_EXPLANATION_EVIDENCE_POLICY_CANDIDATE
            ),
        },
    ]
}
