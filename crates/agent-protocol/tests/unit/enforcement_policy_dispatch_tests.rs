use crate::{
    constants::v08_enforcement_policy_dispatch as dispatch, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementMode, EnforcementPolicyDispatchApprovalState,
    EnforcementPolicyDispatchCapabilityMatrixRow, EnforcementPolicyDispatchOutcomeState,
    EnforcementPolicyDispatchProofLevel, EnforcementPolicyDispatchReadModel,
    EnforcementPolicyDispatchReadModelEntry, EnforcementPolicyDispatchRejectionReason,
    EnforcementPolicyDispatchSourceState, EnforcementPolicyDispatchTimerState,
    ParentActionReference, ParentActorReference, ParentActorRole, ParentDeviceReference,
    ParentEvidenceReference, ParentEvidenceReferenceKind, ParentPlatform, PolicyAction,
    PolicyTarget, PolicyTargetType, V08EnforcementProductControlParentAction,
    V08EnforcementProductControlSurface,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn serializes_policy_dispatch_read_model_with_stable_fields() {
    let read_model = proof_read_model();
    let json = serde_json::to_value(&read_model).expect_value("read model serializes: {error:?}");

    assert_eq!(json["readModelId"], dispatch::READ_MODEL_ID);
    assert_eq!(
        json["entries"][0]["matrixRow"]["proofLevel"],
        dispatch::PROOF_IMPLEMENTED
    );
    assert_eq!(
        json["entries"][0]["matrixRow"]["outcomeState"],
        dispatch::OUTCOME_DISPATCH_READY
    );
    assert_eq!(
        json["entries"][0]["intent"]["evidenceReferences"][0]["evidenceReferenceId"],
        "evidence-app-session-owned-process"
    );
    assert_eq!(
        json["entries"][1]["matrixRow"]["proofLevel"],
        dispatch::PROOF_MANUAL_REQUIRED
    );
    assert_eq!(
        json["entries"][1]["childReasonCode"],
        dispatch::CHILD_REASON_MANUAL_REQUIRED
    );
}

#[test]
fn deserializes_report_only_and_scaffold_states_without_claim_upgrade() {
    let read_model = proof_read_model();
    let encoded =
        serde_json::to_string(&read_model).expect_value("read model serializes: {error:?}");
    let decoded: EnforcementPolicyDispatchReadModel =
        serde_json::from_str(&encoded).expect_value("read model deserializes: {error:?}");

    assert_eq!(
        decoded.entries[0].matrix_row.proof_level,
        EnforcementPolicyDispatchProofLevel::Implemented
    );
    assert_eq!(
        decoded.entries[1].matrix_row.rejection_reason,
        EnforcementPolicyDispatchRejectionReason::AdapterManualRequired
    );
    assert_eq!(
        EnforcementPolicyDispatchProofLevel::Scaffold.as_protocol_str(),
        dispatch::PROOF_SCAFFOLD
    );
}

fn proof_read_model() -> EnforcementPolicyDispatchReadModel {
    EnforcementPolicyDispatchReadModel {
        schema_version: "v0.6".to_string(),
        read_model_id: dispatch::READ_MODEL_ID.to_string(),
        generated_at: dispatch::GENERATED_AT.to_string(),
        entries: vec![
            EnforcementPolicyDispatchReadModelEntry {
                schema_version: "v0.6".to_string(),
                intent: dispatch_owned_process_time_limit_intent(),
                matrix_row: dispatch_owned_process_time_limit_matrix_row(),
                approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
                timer_state: EnforcementPolicyDispatchTimerState::Active,
                audit_refs: vec!["audit-dispatch-owned-process-time-limit".to_string()],
                timer_refs: vec!["timer-dispatch-owned-process-time-limit".to_string()],
                child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT.to_string(),
                reason_codes: vec![dispatch::CHILD_REASON_TIME_LIMIT.to_string()],
                dispatched_at: Some(dispatch::GENERATED_AT.to_string()),
                next_check_at: Some(dispatch::GENERATED_AT.to_string()),
            },
            EnforcementPolicyDispatchReadModelEntry {
                schema_version: "v0.6".to_string(),
                intent: dispatch_network_domain_manual_required_intent(),
                matrix_row: dispatch_network_domain_manual_required_matrix_row(),
                approval_state: EnforcementPolicyDispatchApprovalState::ManualRequired,
                timer_state: EnforcementPolicyDispatchTimerState::NotRequired,
                audit_refs: vec!["audit-dispatch-network-domain-manual-required".to_string()],
                timer_refs: vec!["timer-dispatch-network-domain-manual-required".to_string()],
                child_reason_code: dispatch::CHILD_REASON_MANUAL_REQUIRED.to_string(),
                reason_codes: vec![dispatch::CHILD_REASON_MANUAL_REQUIRED.to_string()],
                dispatched_at: Some(dispatch::GENERATED_AT.to_string()),
                next_check_at: Some(dispatch::GENERATED_AT.to_string()),
            },
        ],
    }
}

fn dispatch_device() -> ParentDeviceReference {
    ParentDeviceReference {
        device_id: "local-dev-agent".to_string(),
        child_profile_id: Some("child-profile-v0-8-dispatch".to_string()),
        label: "Local dev child device".to_string(),
        platform: "windows".to_string(),
    }
}

fn dispatch_evidence() -> ParentEvidenceReference {
    ParentEvidenceReference {
        evidence_reference_id: "evidence-app-session-owned-process".to_string(),
        kind: ParentEvidenceReferenceKind::ActivityEvent,
        observed_at: dispatch::GENERATED_AT.to_string(),
    }
}

fn approval_reference(
    approval_state: EnforcementPolicyDispatchApprovalState,
) -> Option<ParentActionReference> {
    if approval_state == EnforcementPolicyDispatchApprovalState::NotRequired {
        return None;
    }

    Some(ParentActionReference {
        action_reference_id: "approval-dispatch".to_string(),
        actor: parent_actor(),
        policy_version: "policy-version-v0-8-dispatch".to_string(),
        created_at: dispatch::GENERATED_AT.to_string(),
    })
}

fn dispatch_owned_process_time_limit_intent() -> crate::EnforcementPolicyDispatchIntent {
    crate::EnforcementPolicyDispatchIntent {
        schema_version: "v0.6".to_string(),
        intent_id: "dispatch-owned-process-time-limit".to_string(),
        actor: parent_actor(),
        device: dispatch_device(),
        policy_decision_id: "policy-dispatch-owned-process-time-limit".to_string(),
        policy_decision_ref: "decision-dispatch-owned-process-time-limit".to_string(),
        policy_version: "policy-version-v0-8-dispatch".to_string(),
        target: dispatch_owned_process_time_limit_target(),
        requested_policy_action: PolicyAction::Block,
        requested_parent_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
        schedule_ref: "schedule-dispatch-owned-process-time-limit".to_string(),
        evidence_references: vec![dispatch_evidence()],
        approval_ref: None,
        route_ref: "route-localhost-agent-service".to_string(),
        source_state: EnforcementPolicyDispatchSourceState::Ready,
        dry_run: false,
        requested_at: dispatch::GENERATED_AT.to_string(),
    }
}

fn dispatch_network_domain_manual_required_intent() -> crate::EnforcementPolicyDispatchIntent {
    crate::EnforcementPolicyDispatchIntent {
        schema_version: "v0.6".to_string(),
        intent_id: "dispatch-network-domain-manual-required".to_string(),
        actor: parent_actor(),
        device: dispatch_device(),
        policy_decision_id: "policy-dispatch-network-domain-manual-required".to_string(),
        policy_decision_ref: "decision-dispatch-network-domain-manual-required".to_string(),
        policy_version: "policy-version-v0-8-dispatch".to_string(),
        target: dispatch_network_domain_manual_required_target(),
        requested_policy_action: PolicyAction::Block,
        requested_parent_action: V08EnforcementProductControlParentAction::ReportOnly,
        schedule_ref: "schedule-dispatch-network-domain-manual-required".to_string(),
        evidence_references: vec![dispatch_evidence()],
        approval_ref: approval_reference(EnforcementPolicyDispatchApprovalState::ManualRequired),
        route_ref: "route-localhost-agent-service".to_string(),
        source_state: EnforcementPolicyDispatchSourceState::Ready,
        dry_run: false,
        requested_at: dispatch::GENERATED_AT.to_string(),
    }
}

fn dispatch_owned_process_time_limit_target() -> PolicyTarget {
    PolicyTarget {
        target_id: "target-dispatch-owned-process-time-limit".to_string(),
        target_type: PolicyTargetType::App,
        target_value: "owned-process:ocentra-child-demo.exe".to_string(),
    }
}

fn dispatch_network_domain_manual_required_target() -> PolicyTarget {
    PolicyTarget {
        target_id: "target-dispatch-network-domain-manual-required".to_string(),
        target_type: PolicyTargetType::App,
        target_value: "owned-process:ocentra-child-demo.exe".to_string(),
    }
}

fn dispatch_owned_process_time_limit_matrix_row() -> EnforcementPolicyDispatchCapabilityMatrixRow {
    EnforcementPolicyDispatchCapabilityMatrixRow {
        matrix_id: "matrix-owned-process-implemented".to_string(),
        surface: V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::ProcessControl,
        requested_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
        mode: EnforcementMode::TerminateProcess,
        capability_state: EnforcementCapabilityState::Supported,
        proof_level: EnforcementPolicyDispatchProofLevel::Implemented,
        outcome_state: EnforcementPolicyDispatchOutcomeState::DispatchReady,
        rejection_reason: EnforcementPolicyDispatchRejectionReason::None,
        source_state: EnforcementPolicyDispatchSourceState::Ready,
        child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT.to_string(),
    }
}

fn dispatch_network_domain_manual_required_matrix_row(
) -> EnforcementPolicyDispatchCapabilityMatrixRow {
    EnforcementPolicyDispatchCapabilityMatrixRow {
        matrix_id: "matrix-network-domain-manual-required".to_string(),
        surface: V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
        platform: ParentPlatform::Windows,
        adapter_kind: EnforcementAdapterKind::NetworkControl,
        requested_action: V08EnforcementProductControlParentAction::ReportOnly,
        mode: EnforcementMode::TemporaryBlock,
        capability_state: EnforcementCapabilityState::ManualRequired,
        proof_level: EnforcementPolicyDispatchProofLevel::ManualRequired,
        outcome_state: EnforcementPolicyDispatchOutcomeState::ManualRequired,
        rejection_reason: EnforcementPolicyDispatchRejectionReason::AdapterManualRequired,
        source_state: EnforcementPolicyDispatchSourceState::Ready,
        child_reason_code: dispatch::CHILD_REASON_MANUAL_REQUIRED.to_string(),
    }
}

fn parent_actor() -> ParentActorReference {
    ParentActorReference {
        actor_id: "parent-actor-primary".to_string(),
        role: ParentActorRole::Parent,
    }
}
