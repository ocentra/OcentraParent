use super::{
    constants, policy_constants as policy, ChildProfileReference, FamilyReference,
    LocalAiParentRuleContextRef, ParentActorReference, ParentActorRole, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, PolicyAction,
    PolicyAssistantConfirmationState, PolicyDecision, PolicyDecisionHandoffState,
    PolicyPreviewFindingKind, PolicyPreviewNetworkEvidenceMapping, PolicyPreviewReadModel,
    PolicyPreviewReadModelRow, PolicyPreviewTargetState, PolicyRequestOrigin, PolicyRequestStatus,
    PolicyRule, PolicySourceStatus, PolicySourceSurface, PolicyTarget, PolicyTargetType,
    POLICY_DRY_RUN_SCHEMA_VERSION,
};
use crate::activity::policy_preview::{PolicyPreviewManualReviewState, PolicyPreviewSaveState};
use crate::policy_preview_finding_kinds_csv;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn policy_preview_read_model_serializes_stored_evidence_rows() {
    let read_model = policy_preview_read_model();

    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_preview_contract_fields(&serialized);
    assert_preview_state_fields(&serialized);
    assert_preview_decision_fields(&serialized);
    assert_preview_parent_rule_context_fields(&serialized);
    assert_preview_network_mapping_fields(&serialized);
}

fn assert_preview_contract_fields(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["schemaVersion"],
        policy::CONTRACT_SCHEMA_VERSION_V0_6
    );
    assert_eq!(
        serialized["custody"],
        policy::PREVIEW_CUSTODY_ACTIVITY_STORE
    );
    assert_eq!(serialized["rows"][0]["previewId"], policy::TEST_PREVIEW_ID);
}

fn assert_preview_state_fields(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["rows"][0]["policyPreviewSaveState"],
        "preview-required"
    );
    assert_eq!(
        serialized["rows"][0]["policyPreviewManualReviewState"],
        "required"
    );
    assert_eq!(
        serialized["rows"][0]["policyPreviewTargetState"],
        "unsupported"
    );
    assert_eq!(
        serialized["rows"][0]["policyPreviewTargetExplanationCode"],
        constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER
    );
    assert_eq!(
        serialized["rows"][0]["policyPreviewFindingKinds"],
        "unsupported-target"
    );
    assert_eq!(serialized["rows"][0]["policySourceStatus"], "preview");
    assert_eq!(serialized["rows"][0]["policySourceSurface"], "ai-preview");
    assert_eq!(
        serialized["rows"][0]["policyRequestOrigin"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized["rows"][0]["policyAssistantConfirmationState"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized["rows"][0]["policyRequestStatus"],
        serde_json::Value::Null
    );
}

fn assert_preview_decision_fields(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["rows"][0]["policyApprovalId"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized["rows"][0]["policyOverrideId"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized["rows"][0]["policyAuditReferenceId"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized["rows"][0]["decision"]["enforcementHandoffState"],
        policy::HANDOFF_DISABLED
    );
}

fn assert_preview_parent_rule_context_fields(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["rows"][0]["parentRuleContextReferences"][0]["parentRuleRefId"],
        policy::TEST_PARENT_RULE_CONTEXT_REF_ID
    );
    assert_eq!(
        serialized["rows"][0]["parentRuleContextReferences"][0]["targetEvidenceRefs"][0],
        policy::TEST_EVIDENCE_ID
    );
}

fn assert_preview_network_mapping_fields(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["rows"][0]["networkEvidenceMapping"]["evidenceGrade"],
        policy::NETWORK_EVIDENCE_GRADE_B
    );
    assert_eq!(
        serialized["rows"][0]["networkEvidenceMapping"]["requestedAction"],
        policy::ACTION_BLOCK
    );
    assert_eq!(
        serialized["rows"][0]["networkEvidenceMapping"]["mappedAction"],
        policy::ACTION_ASK_PARENT
    );
    assert_eq!(
        serialized["rows"][0]["networkEvidenceMapping"]["mode"],
        policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW
    );
    assert_eq!(
        serialized["rows"][0]["networkEvidenceMapping"]["adapterActionAuthorized"],
        false
    );
    assert_eq!(
        serialized["rows"][0]["networkEvidenceMapping"]["enforcementCommandAuthorized"],
        false
    );
}

fn policy_preview_read_model() -> PolicyPreviewReadModel {
    PolicyPreviewReadModel {
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
            parent_rule_context_references: vec![parent_rule_context()],
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
            policy_preview_save_state: Some(PolicyPreviewSaveState::PreviewRequired),
            policy_preview_manual_review_state: Some(PolicyPreviewManualReviewState::Required),
            policy_preview_target_state: Some(PolicyPreviewTargetState::Unsupported),
            policy_preview_target_explanation_code: Some(
                constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string(),
            ),
            policy_preview_finding_kinds: policy_preview_finding_kinds_csv(&[
                PolicyPreviewFindingKind::UnsupportedTarget,
            ]),
            policy_source_status: Some(PolicySourceStatus::Preview),
            policy_source_surface: Some(PolicySourceSurface::AiPreview),
            policy_request_origin: None::<PolicyRequestOrigin>,
            policy_assistant_confirmation_state: None::<PolicyAssistantConfirmationState>,
            policy_request_status: None::<PolicyRequestStatus>,
            policy_approval_id: None,
            policy_override_id: None,
            policy_replay_of_approval_id: None,
            policy_reviewed_by_actor_id: None,
            policy_reviewed_by_actor_role: None,
            policy_reviewed_at: None,
            policy_audit_reference_id: None,
            network_evidence_mapping: Some(PolicyPreviewNetworkEvidenceMapping {
                evidence_grade: policy::NETWORK_EVIDENCE_GRADE_B.to_string(),
                requested_action: policy::ACTION_BLOCK.to_string(),
                mapped_action: policy::ACTION_ASK_PARENT.to_string(),
                mode: policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW.to_string(),
                adapter_action_authorized: false,
                enforcement_command_authorized: false,
            }),
            // This protocol fixture contains no request/source confirmation fields;
            // production projection therefore emits no confirmation context.
            confirmation_context: None,
        }],
    }
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

fn parent_rule_context() -> LocalAiParentRuleContextRef {
    LocalAiParentRuleContextRef {
        parent_rule_ref_id: policy::TEST_PARENT_RULE_CONTEXT_REF_ID.to_string(),
        policy_version: policy::TEST_POLICY_VERSION.to_string(),
        family: FamilyReference {
            family_id: policy::TEST_FAMILY_ID.to_string(),
        },
        child_profile: ChildProfileReference {
            child_profile_id: policy::TEST_CHILD_PROFILE_ID.to_string(),
            display_name: policy::TEST_CHILD_PROFILE_DISPLAY_NAME.to_string(),
        },
        device: ParentDeviceReference {
            device_id: policy::TEST_PARENT_DEVICE_ID.to_string(),
            child_profile_id: Some(policy::TEST_CHILD_PROFILE_ID.to_string()),
            label: policy::TEST_PARENT_DEVICE_LABEL.to_string(),
            platform: policy::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
        },
        rule: rule(),
        target_evidence_refs: vec![policy::TEST_EVIDENCE_ID.to_string()],
        custody: policy::TEST_PARENT_RULE_CONTEXT_CUSTODY.to_string(),
        updated_at: policy::TEST_EVALUATED_AT.to_string(),
        expires_at: None,
    }
}

fn rule() -> PolicyRule {
    PolicyRule {
        rule_id: policy::TEST_BLOCK_RULE_ID.to_string(),
        target: target(),
        action: PolicyAction::Block,
        schedule_id: None,
        priority: 10,
        reason_code: policy::TEST_REASON_PARENT_BLOCK.to_string(),
        created_by: ParentActorReference {
            actor_id: policy::TEST_PARENT_ACTOR_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        enabled: true,
        effective_from: None,
        effective_until: None,
    }
}
