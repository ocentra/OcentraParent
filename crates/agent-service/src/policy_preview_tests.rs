use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, LogFieldValue, ParentEvidenceReference,
    ParentEvidenceReferenceKind, PolicyAction, PolicyDecision, PolicyDecisionHandoffState,
    PolicyPreviewReadModel, PolicyPreviewReadModelRow, PolicyTarget, PolicyTargetType,
};

use crate::policy_preview_payload::policy_preview_read_model_payload;

#[test]
fn policy_preview_payload_exposes_latest_dry_run_decision_without_enforcement() {
    let read_model = PolicyPreviewReadModel {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        custody: policy::PREVIEW_CUSTODY_ACTIVITY_STORE.to_string(),
        limit: 5,
        returned: 1,
        capability_status: policy::PREVIEW_CAPABILITY_READY.to_string(),
        rows: vec![PolicyPreviewReadModelRow {
            preview_id: policy::TEST_PREVIEW_ID.to_string(),
            source_event_id: policy::TEST_EVIDENCE_ID.to_string(),
            observed_at: policy::TEST_EVALUATED_AT.to_string(),
            target: PolicyTarget {
                target_id: policy::TEST_TARGET_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: policy::TEST_TARGET_VALUE.to_string(),
            },
            evidence_references: vec![ParentEvidenceReference {
                evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: policy::TEST_EVALUATED_AT.to_string(),
            }],
            decision: PolicyDecision {
                schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                decision_id: policy::TEST_DECISION_ID.to_string(),
                action: PolicyAction::Unknown,
                reason_codes: vec![
                    policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
                    policy::REASON_LOCAL_AI_RESULT_MISSING.to_string(),
                ],
                evidence_references: vec![ParentEvidenceReference {
                    evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
                    kind: ParentEvidenceReferenceKind::ActivityEvent,
                    observed_at: policy::TEST_EVALUATED_AT.to_string(),
                }],
                rule_ids: Vec::new(),
                local_ai_result_id: None,
                dry_run: true,
                enforcement_handoff_state: PolicyDecisionHandoffState::Disabled,
                expires_at: None,
            },
        }],
    };

    let payload = policy_preview_read_model_payload(&read_model);

    assert_eq!(
        payload.get(constants::field::POLICY_PREVIEW_ID),
        Some(&LogFieldValue::String(policy::TEST_PREVIEW_ID.to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_ACTION),
        Some(&LogFieldValue::String(policy::ACTION_UNKNOWN.to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_DRY_RUN),
        Some(&LogFieldValue::Boolean(true))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_HANDOFF_STATE),
        Some(&LogFieldValue::String(policy::HANDOFF_DISABLED.to_string()))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_RESULT_ID),
        Some(&LogFieldValue::Null(()))
    );
}
