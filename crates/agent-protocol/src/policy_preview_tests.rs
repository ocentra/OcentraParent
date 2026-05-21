use super::{
    constants, policy_constants as policy, ParentEvidenceReference, ParentEvidenceReferenceKind,
    PolicyAction, PolicyDecision, PolicyDecisionHandoffState, PolicyPreviewReadModel,
    PolicyPreviewReadModelRow, PolicyTarget, PolicyTargetType, POLICY_DRY_RUN_SCHEMA_VERSION,
};

#[test]
fn policy_preview_read_model_serializes_stored_evidence_rows() {
    let read_model = PolicyPreviewReadModel {
        schema_version: POLICY_DRY_RUN_SCHEMA_VERSION.to_string(),
        generated_at: policy::TEST_EVALUATED_AT.to_string(),
        custody: policy::PREVIEW_CUSTODY_ACTIVITY_STORE.to_string(),
        limit: 5,
        returned: 1,
        capability_status: policy::PREVIEW_CAPABILITY_READY.to_string(),
        rows: vec![PolicyPreviewReadModelRow {
            preview_id: policy::TEST_PREVIEW_ID.to_string(),
            source_event_id: policy::TEST_EVIDENCE_ID.to_string(),
            observed_at: policy::TEST_EVALUATED_AT.to_string(),
            target: target(),
            evidence_references: vec![evidence()],
            decision: PolicyDecision {
                schema_version: POLICY_DRY_RUN_SCHEMA_VERSION.to_string(),
                decision_id: policy::TEST_DECISION_ID.to_string(),
                action: PolicyAction::Unknown,
                reason_codes: vec![policy::REASON_LOCAL_AI_RESULT_MISSING.to_string()],
                evidence_references: vec![evidence()],
                rule_ids: Vec::new(),
                local_ai_result_id: None,
                dry_run: true,
                enforcement_handoff_state: PolicyDecisionHandoffState::Disabled,
                expires_at: None,
            },
        }],
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["schemaVersion"],
        policy::CONTRACT_SCHEMA_VERSION_V0_6
    );
    assert_eq!(
        serialized["custody"],
        policy::PREVIEW_CUSTODY_ACTIVITY_STORE
    );
    assert_eq!(serialized["rows"][0]["previewId"], policy::TEST_PREVIEW_ID);
    assert_eq!(
        serialized["rows"][0]["decision"]["enforcementHandoffState"],
        policy::HANDOFF_DISABLED
    );
}

fn target() -> PolicyTarget {
    PolicyTarget {
        target_id: policy::TEST_TARGET_ID.to_string(),
        target_type: PolicyTargetType::Domain,
        target_value: policy::TEST_TARGET_VALUE.to_string(),
    }
}

fn evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: policy::TEST_EVIDENCE_ID.to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: policy::TEST_EVALUATED_AT.to_string(),
    }
}
