use ocentra_parent_agent_protocol::activity::policy::{
    ParentActorReference, ParentActorRole, ParentEvidenceReference, ParentEvidenceReferenceKind,
    PolicyAction, PolicyDecision, PolicyDecisionHandoffState, PolicyRule, PolicyTarget,
    PolicyTargetType,
};
use ocentra_parent_agent_protocol::activity::policy_context::{
    ChildProfileReference, FamilyReference, LocalAiParentRuleContextRef, ParentDeviceReference,
};
use ocentra_parent_agent_protocol::activity::policy_preview::{
    PolicyAssistantConfirmationState, PolicyPreviewFindingKind, PolicyPreviewManualReviewState,
    PolicyPreviewNetworkEvidenceMapping, PolicyPreviewReadModel, PolicyPreviewReadModelRow,
    PolicyPreviewSaveState, PolicyPreviewTargetState, PolicyRequestOrigin, PolicyRequestStatus,
    PolicySourceStatus, PolicySourceSurface,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::policy_constants as policy;
use ocentra_parent_agent_protocol::policy_preview_finding_kinds_csv;
use std::string::String as TestString;

use crate::policy_preview_payload::policy_preview_read_model_payload;

#[test]
fn policy_preview_payload_exposes_latest_dry_run_decision_without_enforcement() {
    let read_model = read_model_with_network_mapping();

    let payload = policy_preview_read_model_payload(&read_model);

    assert_preview_payload_core(&payload);
    assert_preview_payload_review(&payload);
    assert_preview_payload_network_mapping(&payload);
}

fn assert_preview_payload_core(payload: &std::collections::BTreeMap<TestString, LogFieldValue>) {
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
        payload.get(constants::field::POLICY_PREVIEW_SAVE_STATE),
        Some(&LogFieldValue::String("preview-required".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_PREVIEW_MANUAL_REVIEW_STATE),
        Some(&LogFieldValue::String("required".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_PREVIEW_TARGET_STATE),
        Some(&LogFieldValue::String("unsupported".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_PREVIEW_TARGET_EXPLANATION_CODE),
        Some(&LogFieldValue::String(
            constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_PREVIEW_FINDING_KINDS),
        Some(&LogFieldValue::String("unsupported-target".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_SOURCE_STATUS),
        Some(&LogFieldValue::String("preview".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_SOURCE_SURFACE),
        Some(&LogFieldValue::String("ai-preview".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_REQUEST_ORIGIN),
        Some(&LogFieldValue::Null(()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE),
        Some(&LogFieldValue::Null(()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_REQUEST_STATUS),
        Some(&LogFieldValue::Null(()))
    );
}

fn assert_preview_payload_review(payload: &std::collections::BTreeMap<TestString, LogFieldValue>) {
    assert_eq!(
        payload.get(constants::field::POLICY_APPROVAL_ID),
        Some(&LogFieldValue::String("approval-1".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_OVERRIDE_ID),
        Some(&LogFieldValue::String("override-1".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_REPLAY_OF_APPROVAL_ID),
        Some(&LogFieldValue::String("approval-0".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_REVIEWED_BY_ACTOR_ID),
        Some(&LogFieldValue::String("parent-1".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE),
        Some(&LogFieldValue::String("parent".to_string()))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_REVIEWED_AT),
        Some(&LogFieldValue::String(
            "2026-06-18T10:00:00.000Z".to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::POLICY_AUDIT_REFERENCE_ID),
        Some(&LogFieldValue::String("audit-1".to_string()))
    );
    assert_eq!(
        payload.get(policy::PARENT_RULE_CONTEXT_REFERENCE_COUNT_FIELD),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(policy::PARENT_RULE_CONTEXT_REF_IDS_FIELD),
        Some(&LogFieldValue::String(
            policy::TEST_PARENT_RULE_CONTEXT_REF_ID.to_string()
        ))
    );
}

fn assert_preview_payload_network_mapping(
    payload: &std::collections::BTreeMap<TestString, LogFieldValue>,
) {
    assert_eq!(
        payload.get(constants::field::NETWORK_EVIDENCE_GRADE),
        Some(&LogFieldValue::String(
            policy::NETWORK_EVIDENCE_GRADE_B.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_REQUESTED_POLICY_ACTION),
        Some(&LogFieldValue::String(policy::ACTION_BLOCK.to_string()))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_MAPPED_POLICY_ACTION),
        Some(&LogFieldValue::String(
            policy::ACTION_ASK_PARENT.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_POLICY_MAPPING_MODE),
        Some(&LogFieldValue::String(
            policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW.to_string()
        ))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_ADAPTER_ACTION_AUTHORIZED),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        payload.get(constants::field::NETWORK_ENFORCEMENT_COMMAND_AUTHORIZED),
        Some(&LogFieldValue::Boolean(false))
    );
    assert_eq!(
        payload.get(constants::field::LOCAL_AI_RESULT_ID),
        Some(&LogFieldValue::Null(()))
    );
}

fn read_model_with_network_mapping() -> PolicyPreviewReadModel {
    PolicyPreviewReadModel {
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
            evidence_references: vec![evidence()],
            parent_rule_context_references: vec![parent_rule_context()],
            decision: PolicyDecision {
                schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
                decision_id: policy::TEST_DECISION_ID.to_string(),
                action: PolicyAction::Unknown,
                reason_codes: vec![
                    policy::REASON_NO_MATCHING_PARENT_RULE.to_string(),
                    policy::REASON_LOCAL_AI_RESULT_MISSING.to_string(),
                ],
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
            policy_approval_id: Some("approval-1".to_string()),
            policy_override_id: Some("override-1".to_string()),
            policy_replay_of_approval_id: Some("approval-0".to_string()),
            policy_reviewed_by_actor_id: Some("parent-1".to_string()),
            policy_reviewed_by_actor_role: Some("parent".to_string()),
            policy_reviewed_at: Some("2026-06-18T10:00:00.000Z".to_string()),
            policy_audit_reference_id: Some("audit-1".to_string()),
            network_evidence_mapping: Some(PolicyPreviewNetworkEvidenceMapping {
                evidence_grade: policy::NETWORK_EVIDENCE_GRADE_B.to_string(),
                requested_action: policy::ACTION_BLOCK.to_string(),
                mapped_action: policy::ACTION_ASK_PARENT.to_string(),
                mode: policy::NETWORK_POLICY_MAPPING_MODE_PARENT_REVIEW.to_string(),
                adapter_action_authorized: false,
                enforcement_command_authorized: false,
            }),
            confirmation_context: None,
        }],
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
        rule: PolicyRule {
            rule_id: policy::TEST_BLOCK_RULE_ID.to_string(),
            target: PolicyTarget {
                target_id: policy::TEST_TARGET_ID.to_string(),
                target_type: PolicyTargetType::Domain,
                target_value: policy::TEST_TARGET_VALUE.to_string(),
            },
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
        },
        target_evidence_refs: vec![policy::TEST_EVIDENCE_ID.to_string()],
        custody: policy::TEST_PARENT_RULE_CONTEXT_CUSTODY.to_string(),
        updated_at: policy::TEST_EVALUATED_AT.to_string(),
        expires_at: None,
    }
}
