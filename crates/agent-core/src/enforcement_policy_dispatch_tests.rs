use ocentra_parent_agent_protocol::{
    constants::v08_enforcement_policy_dispatch as dispatch, EnforcementAdapterKind,
    EnforcementCapabilityState, EnforcementMode, EnforcementPolicyDispatchApprovalState,
    EnforcementPolicyDispatchCapabilityMatrixRow, EnforcementPolicyDispatchIntent,
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchProofLevel,
    EnforcementPolicyDispatchReadModel, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchRejectionReason, EnforcementPolicyDispatchSourceState,
    EnforcementPolicyDispatchTimerState, ParentActionReference, ParentActorReference,
    ParentActorRole, ParentDeviceReference, ParentEvidenceReference, ParentEvidenceReferenceKind,
    ParentPlatform, PolicyAction, PolicyTarget, PolicyTargetType,
    V08EnforcementProductControlParentAction, V08EnforcementProductControlSurface,
};

use crate::validate_enforcement_policy_dispatch_read_model;

#[test]
fn validates_dispatch_ready_manual_report_only_and_recovery_states() {
    let read_model = read_model(vec![
        entry(
            dispatch::TEST_SUFFIX_DISPATCH_READY,
            EnforcementPolicyDispatchOutcomeState::DispatchReady,
            EnforcementCapabilityState::Supported,
            EnforcementPolicyDispatchRejectionReason::None,
            EnforcementPolicyDispatchTimerState::Active,
        ),
        entry(
            dispatch::TEST_SUFFIX_MANUAL_REQUIRED,
            EnforcementPolicyDispatchOutcomeState::ManualRequired,
            EnforcementCapabilityState::ManualRequired,
            EnforcementPolicyDispatchRejectionReason::AdapterManualRequired,
            EnforcementPolicyDispatchTimerState::NotRequired,
        ),
        entry(
            dispatch::TEST_SUFFIX_REPORT_ONLY,
            EnforcementPolicyDispatchOutcomeState::ReportOnly,
            EnforcementCapabilityState::Degraded,
            EnforcementPolicyDispatchRejectionReason::None,
            EnforcementPolicyDispatchTimerState::RecoveryNeeded,
        ),
    ]);

    let validation = validate_enforcement_policy_dispatch_read_model(&read_model).unwrap();

    assert_eq!(validation.dispatch_ready_count, 1);
    assert_eq!(validation.manual_required_count, 1);
    assert_eq!(validation.report_only_count, 1);
    assert_eq!(validation.recovery_needed_count, 1);
}

#[test]
fn rejects_wrong_device_before_dispatch() {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_WRONG_DEVICE,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.device.device_id = dispatch::TEST_DEVICE_OTHER_CHILD.to_string();

    let rejection = validate_enforcement_policy_dispatch_read_model(&read_model).unwrap_err();

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::WrongDevice
    );
}

#[test]
fn rejects_missing_evidence_before_dispatch() {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MISSING_EVIDENCE,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.evidence_references.clear();

    let rejection = validate_enforcement_policy_dispatch_read_model(&read_model).unwrap_err();

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::MissingEvidence
    );
}

fn read_model(
    entries: Vec<EnforcementPolicyDispatchReadModelEntry>,
) -> EnforcementPolicyDispatchReadModel {
    EnforcementPolicyDispatchReadModel {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        read_model_id: dispatch::READ_MODEL_ID.to_string(),
        generated_at: dispatch::GENERATED_AT.to_string(),
        entries,
    }
}

fn entry(
    suffix: &str,
    outcome_state: EnforcementPolicyDispatchOutcomeState,
    capability_state: EnforcementCapabilityState,
    rejection_reason: EnforcementPolicyDispatchRejectionReason,
    timer_state: EnforcementPolicyDispatchTimerState,
) -> EnforcementPolicyDispatchReadModelEntry {
    EnforcementPolicyDispatchReadModelEntry {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        intent: intent(suffix),
        matrix_row: EnforcementPolicyDispatchCapabilityMatrixRow {
            matrix_id: prefixed(dispatch::PREFIX_MATRIX, suffix),
            surface: V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
            platform: ParentPlatform::Windows,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            requested_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
            mode: EnforcementMode::TerminateProcess,
            capability_state,
            proof_level: EnforcementPolicyDispatchProofLevel::Implemented,
            outcome_state,
            rejection_reason,
            source_state: EnforcementPolicyDispatchSourceState::Ready,
            child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT.to_string(),
        },
        approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
        timer_state,
        audit_refs: vec![prefixed(dispatch::PREFIX_AUDIT, suffix)],
        timer_refs: vec![prefixed(dispatch::PREFIX_TIMER, suffix)],
        child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT.to_string(),
        reason_codes: vec![dispatch::CHILD_REASON_TIME_LIMIT.to_string()],
        dispatched_at: Some(dispatch::GENERATED_AT.to_string()),
        next_check_at: Some(dispatch::GENERATED_AT.to_string()),
    }
}

fn intent(suffix: &str) -> EnforcementPolicyDispatchIntent {
    EnforcementPolicyDispatchIntent {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        intent_id: prefixed(dispatch::PREFIX_INTENT, suffix),
        actor: ParentActorReference {
            actor_id: dispatch::PARENT_ACTOR_PRIMARY_ID.to_string(),
            role: ParentActorRole::Parent,
        },
        device: ParentDeviceReference {
            device_id: dispatch::LOCAL_DEV_AGENT_DEVICE_ID.to_string(),
            child_profile_id: Some(dispatch::LOCAL_DEV_CHILD_PROFILE_ID.to_string()),
            label: dispatch::LOCAL_DEV_CHILD_DEVICE_LABEL.to_string(),
            platform: dispatch::WINDOWS_PLATFORM.to_string(),
        },
        policy_decision_id: prefixed(dispatch::PREFIX_POLICY, suffix),
        policy_decision_ref: prefixed(dispatch::PREFIX_DECISION, suffix),
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        target: PolicyTarget {
            target_id: prefixed(dispatch::PREFIX_TARGET, suffix),
            target_type: PolicyTargetType::App,
            target_value: dispatch::TARGET_OWNED_PROCESS_DEMO.to_string(),
        },
        requested_policy_action: PolicyAction::Block,
        requested_parent_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
        schedule_ref: prefixed(dispatch::PREFIX_SCHEDULE, suffix),
        evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: prefixed(dispatch::PREFIX_EVIDENCE, suffix),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: dispatch::GENERATED_AT.to_string(),
        }],
        approval_ref: Some(ParentActionReference {
            action_reference_id: prefixed(dispatch::PREFIX_APPROVAL, suffix),
            actor: ParentActorReference {
                actor_id: dispatch::PARENT_ACTOR_PRIMARY_ID.to_string(),
                role: ParentActorRole::Parent,
            },
            policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
            created_at: dispatch::GENERATED_AT.to_string(),
        }),
        route_ref: dispatch::LOCAL_DEV_AGENT_ROUTE_REF.to_string(),
        source_state: EnforcementPolicyDispatchSourceState::Ready,
        dry_run: false,
        requested_at: dispatch::GENERATED_AT.to_string(),
    }
}

fn prefixed(prefix: &str, value: &str) -> String {
    let mut output = String::from(prefix);
    output.push_str(value);
    output
}
