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

#[test]
fn serializes_policy_dispatch_read_model_with_stable_fields() {
    let read_model = proof_read_model();
    let json = serde_json::to_value(&read_model)
        .unwrap_or_else(|error| unreachable!("read model serializes: {error:?}"));

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
    let encoded = serde_json::to_string(&read_model)
        .unwrap_or_else(|error| unreachable!("read model serializes: {error:?}"));
    let decoded: EnforcementPolicyDispatchReadModel = serde_json::from_str(&encoded)
        .unwrap_or_else(|error| unreachable!("read model deserializes: {error:?}"));

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
            entry(
                "dispatch-owned-process-time-limit",
                "matrix-owned-process-implemented",
                V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
                EnforcementAdapterKind::ProcessControl,
                V08EnforcementProductControlParentAction::BlockScopedProcess,
                (
                    EnforcementMode::TerminateProcess,
                    EnforcementCapabilityState::Supported,
                    EnforcementPolicyDispatchProofLevel::Implemented,
                    EnforcementPolicyDispatchOutcomeState::DispatchReady,
                    EnforcementPolicyDispatchRejectionReason::None,
                    EnforcementPolicyDispatchSourceState::Ready,
                    EnforcementPolicyDispatchApprovalState::NotRequired,
                    EnforcementPolicyDispatchTimerState::Active,
                ),
                dispatch::CHILD_REASON_TIME_LIMIT,
            ),
            entry(
                "dispatch-network-domain-manual-required",
                "matrix-network-domain-manual-required",
                V08EnforcementProductControlSurface::WindowsNetworkDomainBlocking,
                EnforcementAdapterKind::NetworkControl,
                V08EnforcementProductControlParentAction::ReportOnly,
                (
                    EnforcementMode::TemporaryBlock,
                    EnforcementCapabilityState::ManualRequired,
                    EnforcementPolicyDispatchProofLevel::ManualRequired,
                    EnforcementPolicyDispatchOutcomeState::ManualRequired,
                    EnforcementPolicyDispatchRejectionReason::AdapterManualRequired,
                    EnforcementPolicyDispatchSourceState::Ready,
                    EnforcementPolicyDispatchApprovalState::ManualRequired,
                    EnforcementPolicyDispatchTimerState::NotRequired,
                ),
                dispatch::CHILD_REASON_MANUAL_REQUIRED,
            ),
        ],
    }
}

fn entry(
    intent_id: &str,
    matrix_id: &str,
    surface: V08EnforcementProductControlSurface,
    adapter_kind: EnforcementAdapterKind,
    requested_action: V08EnforcementProductControlParentAction,
    states: (
        EnforcementMode,
        EnforcementCapabilityState,
        EnforcementPolicyDispatchProofLevel,
        EnforcementPolicyDispatchOutcomeState,
        EnforcementPolicyDispatchRejectionReason,
        EnforcementPolicyDispatchSourceState,
        EnforcementPolicyDispatchApprovalState,
        EnforcementPolicyDispatchTimerState,
    ),
    child_reason_code: &str,
) -> EnforcementPolicyDispatchReadModelEntry {
    let (
        mode,
        capability_state,
        proof_level,
        outcome_state,
        rejection_reason,
        source_state,
        approval_state,
        timer_state,
    ) = states;
    EnforcementPolicyDispatchReadModelEntry {
        schema_version: "v0.6".to_string(),
        intent: crate::EnforcementPolicyDispatchIntent {
            schema_version: "v0.6".to_string(),
            intent_id: intent_id.to_string(),
            actor: parent_actor(),
            device: ParentDeviceReference {
                device_id: "local-dev-agent".to_string(),
                child_profile_id: Some("child-profile-v0-8-dispatch".to_string()),
                label: "Local dev child device".to_string(),
                platform: "windows".to_string(),
            },
            policy_decision_id: format!("policy-{intent_id}"),
            policy_decision_ref: format!("decision-{intent_id}"),
            policy_version: "policy-version-v0-8-dispatch".to_string(),
            target: PolicyTarget {
                target_id: format!("target-{intent_id}"),
                target_type: PolicyTargetType::App,
                target_value: "owned-process:ocentra-child-demo.exe".to_string(),
            },
            requested_policy_action: PolicyAction::Block,
            requested_parent_action: requested_action,
            schedule_ref: format!("schedule-{intent_id}"),
            evidence_references: vec![ParentEvidenceReference {
                evidence_reference_id: "evidence-app-session-owned-process".to_string(),
                kind: ParentEvidenceReferenceKind::ActivityEvent,
                observed_at: dispatch::GENERATED_AT.to_string(),
            }],
            approval_ref: approval_reference(approval_state),
            route_ref: "route-localhost-agent-service".to_string(),
            source_state,
            dry_run: false,
            requested_at: dispatch::GENERATED_AT.to_string(),
        },
        matrix_row: EnforcementPolicyDispatchCapabilityMatrixRow {
            matrix_id: matrix_id.to_string(),
            surface,
            platform: ParentPlatform::Windows,
            adapter_kind,
            requested_action,
            mode,
            capability_state,
            proof_level,
            outcome_state,
            rejection_reason,
            source_state,
            child_reason_code: child_reason_code.to_string(),
        },
        approval_state,
        timer_state,
        audit_refs: vec![format!("audit-{intent_id}")],
        timer_refs: vec![format!("timer-{intent_id}")],
        child_reason_code: child_reason_code.to_string(),
        reason_codes: vec![child_reason_code.to_string()],
        dispatched_at: Some(dispatch::GENERATED_AT.to_string()),
        next_check_at: Some(dispatch::GENERATED_AT.to_string()),
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

fn parent_actor() -> ParentActorReference {
    ParentActorReference {
        actor_id: "parent-actor-primary".to_string(),
        role: ParentActorRole::Parent,
    }
}
