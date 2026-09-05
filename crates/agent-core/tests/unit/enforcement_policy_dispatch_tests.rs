use ocentra_parent_agent_protocol::activity::policy::ParentActorReference;
use ocentra_parent_agent_protocol::activity::policy::ParentActorRole;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReferenceKind;
use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::activity::policy::PolicyTarget;
use ocentra_parent_agent_protocol::activity::policy::PolicyTargetType;
use ocentra_parent_agent_protocol::activity::policy_context::ParentDeviceReference;
use ocentra_parent_agent_protocol::constants::v08_enforcement_policy_dispatch as dispatch;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementCapabilityState, EnforcementMode, ParentActionReference,
    ParentPlatform,
};
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchApprovalState, EnforcementPolicyDispatchCapabilityMatrixRow,
    EnforcementPolicyDispatchIntent, EnforcementPolicyDispatchOutcomeState,
    EnforcementPolicyDispatchProofLevel, EnforcementPolicyDispatchReadModel,
    EnforcementPolicyDispatchReadModelEntry, EnforcementPolicyDispatchRejectionReason,
    EnforcementPolicyDispatchSourceState, EnforcementPolicyDispatchTimerState,
};
use ocentra_parent_agent_protocol::enforcement_product_control_spine::{
    V08EnforcementProductControlParentAction, V08EnforcementProductControlSurface,
};

use crate::test_text::{test_err as err, test_ok as ok, TestResult, TestText};
use ocentra_parent_agent_core::enforcement_policy_dispatch::validate_enforcement_policy_dispatch_read_model;

#[test]
fn validates_dispatch_ready_dry_run_manual_report_only_rejected_and_recovery_states() -> TestResult
{
    let mut dry_run_entry = entry(
        dispatch::TEST_SUFFIX_DRY_RUN_ONLY,
        EnforcementPolicyDispatchOutcomeState::DryRunOnly,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::NotRequired,
    );
    dry_run_entry.intent.requested_policy_action = PolicyAction::AskParent;
    dry_run_entry.intent.requested_parent_action =
        V08EnforcementProductControlParentAction::AskParent;
    dry_run_entry.matrix_row.requested_action = V08EnforcementProductControlParentAction::AskParent;
    dry_run_entry.intent.dry_run = true;
    dry_run_entry.matrix_row.proof_level = EnforcementPolicyDispatchProofLevel::Scaffold;
    dry_run_entry.matrix_row.child_reason_code =
        dispatch::CHILD_REASON_ASK_PARENT_REVIEW.to_string();
    dry_run_entry.child_reason_code = dispatch::CHILD_REASON_ASK_PARENT_REVIEW.to_string();
    dry_run_entry.reason_codes = vec![dispatch::CHILD_REASON_ASK_PARENT_REVIEW.to_string()];
    dry_run_entry.approval_state = EnforcementPolicyDispatchApprovalState::Pending;
    dry_run_entry.intent.approval_ref =
        Some(approval_reference(dispatch::TEST_SUFFIX_DRY_RUN_ONLY));
    dry_run_entry.dispatched_at = None;
    dry_run_entry.next_check_at = None;

    let mut rejected_stale_entry = entry(
        dispatch::TEST_SUFFIX_STALE_POLICY_VERSION,
        EnforcementPolicyDispatchOutcomeState::Rejected,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::StalePolicyVersion,
        EnforcementPolicyDispatchTimerState::NotRequired,
    );
    rejected_stale_entry.matrix_row.proof_level = EnforcementPolicyDispatchProofLevel::Scaffold;
    rejected_stale_entry.matrix_row.source_state = EnforcementPolicyDispatchSourceState::Stale;
    rejected_stale_entry.intent.source_state = EnforcementPolicyDispatchSourceState::Stale;
    rejected_stale_entry.matrix_row.child_reason_code =
        dispatch::CHILD_REASON_STALE_POLICY_VERSION.to_string();
    rejected_stale_entry.child_reason_code =
        dispatch::CHILD_REASON_STALE_POLICY_VERSION.to_string();
    rejected_stale_entry.reason_codes =
        vec![dispatch::CHILD_REASON_STALE_POLICY_VERSION.to_string()];
    rejected_stale_entry.dispatched_at = None;
    rejected_stale_entry.next_check_at = None;

    let mut manual_required_entry = entry(
        dispatch::TEST_SUFFIX_MANUAL_REQUIRED,
        EnforcementPolicyDispatchOutcomeState::ManualRequired,
        EnforcementCapabilityState::ManualRequired,
        EnforcementPolicyDispatchRejectionReason::AdapterManualRequired,
        EnforcementPolicyDispatchTimerState::NotRequired,
    );
    manual_required_entry.approval_state = EnforcementPolicyDispatchApprovalState::ManualRequired;
    manual_required_entry.intent.approval_ref =
        Some(approval_reference(dispatch::TEST_SUFFIX_MANUAL_REQUIRED));

    let read_model = read_model(vec![
        entry(
            dispatch::TEST_SUFFIX_DISPATCH_READY,
            EnforcementPolicyDispatchOutcomeState::DispatchReady,
            EnforcementCapabilityState::Supported,
            EnforcementPolicyDispatchRejectionReason::None,
            EnforcementPolicyDispatchTimerState::Active,
        ),
        dry_run_entry,
        manual_required_entry,
        entry(
            dispatch::TEST_SUFFIX_REPORT_ONLY,
            EnforcementPolicyDispatchOutcomeState::ReportOnly,
            EnforcementCapabilityState::Degraded,
            EnforcementPolicyDispatchRejectionReason::None,
            EnforcementPolicyDispatchTimerState::RecoveryNeeded,
        ),
        rejected_stale_entry,
    ]);

    let validation = ok(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::READ_MODEL_ID,
    )?;

    assert_eq!(validation.dispatch_ready_count, 1);
    assert_eq!(validation.dry_run_only_count, 1);
    assert_eq!(validation.manual_required_count, 1);
    assert_eq!(validation.report_only_count, 1);
    assert_eq!(validation.rejected_count, 1);
    assert_eq!(validation.recovery_needed_count, 1);

    Ok(())
}

#[test]
fn rejects_wrong_device_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_WRONG_DEVICE,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.device.device_id = dispatch::TEST_DEVICE_OTHER_CHILD.to_string();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_WRONG_DEVICE,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::WrongDevice
    );

    Ok(())
}

#[test]
fn rejects_dry_run_dispatch_ready_before_execution() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_DRY_RUN_ONLY,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.dry_run = true;

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_DRY_RUN_ONLY,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved
    );

    Ok(())
}

#[test]
fn rejects_unapproved_dispatch_ready_before_execution() -> TestResult {
    for approval_state in [
        EnforcementPolicyDispatchApprovalState::Pending,
        EnforcementPolicyDispatchApprovalState::Denied,
        EnforcementPolicyDispatchApprovalState::Expired,
        EnforcementPolicyDispatchApprovalState::ManualRequired,
    ] {
        let mut read_model = read_model(vec![entry(
            dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
            EnforcementPolicyDispatchOutcomeState::DispatchReady,
            EnforcementCapabilityState::Supported,
            EnforcementPolicyDispatchRejectionReason::None,
            EnforcementPolicyDispatchTimerState::Active,
        )]);
        read_model.entries[0].approval_state = approval_state;
        read_model.entries[0].intent.approval_ref = Some(approval_reference(
            dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
        ));

        let rejection = err(
            validate_enforcement_policy_dispatch_read_model(&read_model),
            dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
        )?;

        assert_eq!(
            rejection,
            EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved
        );
    }

    Ok(())
}

#[test]
fn accepts_approved_dispatch_ready_states_with_valid_approval_reference() -> TestResult {
    for approval_state in [
        EnforcementPolicyDispatchApprovalState::Approved,
        EnforcementPolicyDispatchApprovalState::OverrideActive,
    ] {
        let mut read_model = read_model(vec![entry(
            dispatch::TEST_SUFFIX_DISPATCH_READY,
            EnforcementPolicyDispatchOutcomeState::DispatchReady,
            EnforcementCapabilityState::Supported,
            EnforcementPolicyDispatchRejectionReason::None,
            EnforcementPolicyDispatchTimerState::Active,
        )]);
        read_model.entries[0].approval_state = approval_state;
        read_model.entries[0].intent.approval_ref =
            Some(approval_reference(dispatch::TEST_SUFFIX_DISPATCH_READY));

        let validation = ok(
            validate_enforcement_policy_dispatch_read_model(&read_model),
            dispatch::READ_MODEL_ID,
        )?;

        assert_eq!(validation.dispatch_ready_count, 1);
    }

    Ok(())
}

#[test]
fn rejects_missing_evidence_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MISSING_EVIDENCE,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.evidence_references.clear();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MISSING_EVIDENCE,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::MissingEvidence
    );

    Ok(())
}

#[test]
fn rejects_malformed_evidence_reference_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MALFORMED_POLICY_DECISION,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.evidence_references[0].evidence_reference_id =
        dispatch::TEST_MALFORMED_POLICY_DECISION_REF.to_string();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MALFORMED_POLICY_DECISION,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::MissingEvidence
    );

    Ok(())
}

#[test]
fn rejects_policy_decision_id_and_ref_mismatch_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.policy_decision_ref =
        prefixed(dispatch::PREFIX_DECISION, dispatch::TEST_DEVICE_OTHER_CHILD).to_string();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision
    );

    Ok(())
}

#[test]
fn rejects_missing_audit_provenance_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MISSING_EVIDENCE,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].audit_refs.clear();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MISSING_EVIDENCE,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved
    );

    Ok(())
}

#[test]
fn rejects_matrix_action_mismatch_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MALFORMED_POLICY_DECISION,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].matrix_row.requested_action =
        V08EnforcementProductControlParentAction::AskParent;

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MALFORMED_POLICY_DECISION,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved
    );

    Ok(())
}

#[test]
fn rejects_stale_policy_version_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_STALE_POLICY_VERSION,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.policy_version =
        dispatch::POLICY_VERSION_V0_8_DISPATCH_STALE.to_string();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_STALE_POLICY_VERSION,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::StalePolicyVersion
    );

    Ok(())
}

#[test]
fn rejects_missing_policy_decision_reference_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.policy_decision_ref.clear();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MISSING_POLICY_DECISION,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision
    );

    Ok(())
}

#[test]
fn rejects_malformed_policy_decision_reference_before_dispatch() -> TestResult {
    let mut read_model = read_model(vec![entry(
        dispatch::TEST_SUFFIX_MALFORMED_POLICY_DECISION,
        EnforcementPolicyDispatchOutcomeState::DispatchReady,
        EnforcementCapabilityState::Supported,
        EnforcementPolicyDispatchRejectionReason::None,
        EnforcementPolicyDispatchTimerState::Active,
    )]);
    read_model.entries[0].intent.policy_decision_ref =
        dispatch::TEST_MALFORMED_POLICY_DECISION_REF.to_string();

    let rejection = err(
        validate_enforcement_policy_dispatch_read_model(&read_model),
        dispatch::TEST_SUFFIX_MALFORMED_POLICY_DECISION,
    )?;

    assert_eq!(
        rejection,
        EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision
    );

    Ok(())
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
    suffix: impl std::fmt::Display,
    outcome_state: EnforcementPolicyDispatchOutcomeState,
    capability_state: EnforcementCapabilityState,
    rejection_reason: EnforcementPolicyDispatchRejectionReason,
    timer_state: EnforcementPolicyDispatchTimerState,
) -> EnforcementPolicyDispatchReadModelEntry {
    let suffix = suffix.to_string();
    EnforcementPolicyDispatchReadModelEntry {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        intent: intent(&suffix),
        matrix_row: EnforcementPolicyDispatchCapabilityMatrixRow {
            matrix_id: prefixed(dispatch::PREFIX_MATRIX, &suffix).to_string(),
            surface: V08EnforcementProductControlSurface::WindowsOwnedProcessTimeLimit,
            platform: ParentPlatform::Windows,
            adapter_kind: EnforcementAdapterKind::ProcessControl,
            requested_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
            mode: EnforcementMode::TerminateProcess,
            capability_state,
            proof_level: proof_level_for(outcome_state),
            outcome_state,
            rejection_reason,
            source_state: EnforcementPolicyDispatchSourceState::Ready,
            child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT.to_string(),
        },
        approval_state: EnforcementPolicyDispatchApprovalState::NotRequired,
        timer_state,
        audit_refs: vec![prefixed(dispatch::PREFIX_AUDIT, &suffix).to_string()],
        timer_refs: if timer_state == EnforcementPolicyDispatchTimerState::NotRequired {
            Vec::new()
        } else {
            vec![prefixed(dispatch::PREFIX_TIMER, &suffix).to_string()]
        },
        child_reason_code: dispatch::CHILD_REASON_TIME_LIMIT.to_string(),
        reason_codes: vec![dispatch::CHILD_REASON_TIME_LIMIT.to_string()],
        dispatched_at: if outcome_state == EnforcementPolicyDispatchOutcomeState::DispatchReady {
            Some(dispatch::GENERATED_AT.to_string())
        } else {
            None
        },
        next_check_at: if matches!(
            timer_state,
            EnforcementPolicyDispatchTimerState::Active
                | EnforcementPolicyDispatchTimerState::RestartRecovered
                | EnforcementPolicyDispatchTimerState::RecoveryNeeded
        ) {
            Some(dispatch::GENERATED_AT.to_string())
        } else {
            None
        },
    }
}

fn intent(suffix: impl std::fmt::Display) -> EnforcementPolicyDispatchIntent {
    let suffix = suffix.to_string();
    EnforcementPolicyDispatchIntent {
        schema_version:
            ocentra_parent_agent_protocol::policy_constants::CONTRACT_SCHEMA_VERSION_V0_6
                .to_string(),
        intent_id: suffix.clone(),
        actor: parent_actor(),
        device: ParentDeviceReference {
            device_id: dispatch::LOCAL_DEV_AGENT_DEVICE_ID.to_string(),
            child_profile_id: Some(dispatch::LOCAL_DEV_CHILD_PROFILE_ID.to_string()),
            label: dispatch::LOCAL_DEV_CHILD_DEVICE_LABEL.to_string(),
            platform: dispatch::WINDOWS_PLATFORM.to_string(),
        },
        policy_decision_id: prefixed(dispatch::PREFIX_POLICY, &suffix).to_string(),
        policy_decision_ref: prefixed(dispatch::PREFIX_DECISION, &suffix).to_string(),
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        target: PolicyTarget {
            target_id: prefixed(dispatch::PREFIX_TARGET, &suffix).to_string(),
            target_type: PolicyTargetType::App,
            target_value: dispatch::TARGET_OWNED_PROCESS_DEMO.to_string(),
        },
        requested_policy_action: PolicyAction::Block,
        requested_parent_action: V08EnforcementProductControlParentAction::BlockScopedProcess,
        schedule_ref: prefixed(dispatch::PREFIX_SCHEDULE, &suffix).to_string(),
        evidence_references: vec![ParentEvidenceReference {
            evidence_reference_id: prefixed(dispatch::PREFIX_EVIDENCE, &suffix).to_string(),
            kind: ParentEvidenceReferenceKind::ActivityEvent,
            observed_at: dispatch::GENERATED_AT.to_string(),
        }],
        approval_ref: None,
        route_ref: dispatch::LOCAL_DEV_AGENT_ROUTE_REF.to_string(),
        source_state: EnforcementPolicyDispatchSourceState::Ready,
        dry_run: false,
        requested_at: dispatch::GENERATED_AT.to_string(),
    }
}

fn parent_actor() -> ParentActorReference {
    ParentActorReference {
        actor_id: dispatch::PARENT_ACTOR_PRIMARY_ID.to_string(),
        role: ParentActorRole::Parent,
    }
}

fn approval_reference(suffix: impl std::fmt::Display) -> ParentActionReference {
    ParentActionReference {
        action_reference_id: prefixed(dispatch::PREFIX_APPROVAL, &suffix).to_string(),
        actor: parent_actor(),
        policy_version: dispatch::POLICY_VERSION_V0_8_DISPATCH.to_string(),
        created_at: dispatch::GENERATED_AT.to_string(),
    }
}

fn proof_level_for(
    outcome_state: EnforcementPolicyDispatchOutcomeState,
) -> EnforcementPolicyDispatchProofLevel {
    match outcome_state {
        EnforcementPolicyDispatchOutcomeState::DispatchReady => {
            EnforcementPolicyDispatchProofLevel::Implemented
        }
        EnforcementPolicyDispatchOutcomeState::ManualRequired => {
            EnforcementPolicyDispatchProofLevel::ManualRequired
        }
        EnforcementPolicyDispatchOutcomeState::ReportOnly => {
            EnforcementPolicyDispatchProofLevel::ReportOnly
        }
        EnforcementPolicyDispatchOutcomeState::DryRunOnly
        | EnforcementPolicyDispatchOutcomeState::Degraded
        | EnforcementPolicyDispatchOutcomeState::Unavailable
        | EnforcementPolicyDispatchOutcomeState::Rejected => {
            EnforcementPolicyDispatchProofLevel::Scaffold
        }
    }
}

fn prefixed(prefix: impl std::fmt::Display, value: impl std::fmt::Display) -> TestText {
    let mut output = prefix.to_string();
    output.push_str(&value.to_string());
    TestText::from_display(output)
}
