use ocentra_family_identity_core::family_identity::{DeviceTrustState, HouseholdRole};
use ocentra_family_identity_core::household_authority::AuditRequirementState;
use ocentra_family_identity_core::setup_lifecycle::{
    authorize_setup_invite, device_trust_state_for_recovery_operation,
    device_trust_state_for_recovery_state, evaluate_recovery_operation,
    RecoveryChildEvidenceAccessState, RecoveryDataCustodyHandoffState, RecoveryDecisionState,
    RecoveryFailureReason, RecoveryIdentityProofState, RecoveryKind, RecoveryOperation,
    RecoveryState, RecoverySupportChannel, SetupInviteDecisionState, SetupInviteFailureReason,
    SetupInviteInput, SetupInvitePurpose, SetupInviteReplayState, SetupInviteState,
    SetupInviteTargetRole, SetupRecoveryAbuseState, SetupRecoveryResponseTimingState,
};

fn parent_member_invite(
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
) -> SetupInviteInput {
    SetupInviteInput {
        inviter_role: HouseholdRole::ParentOwner,
        same_family: true,
        purpose,
        target_role,
        invite_state: SetupInviteState::Pending,
        single_use: true,
        replay_state: SetupInviteReplayState::Fresh,
        abuse_state: SetupRecoveryAbuseState::WithinLimit,
        response_timing_state: SetupRecoveryResponseTimingState::Uniform,
    }
}

fn parent_member_recovery(kind: RecoveryKind) -> RecoveryOperation {
    RecoveryOperation {
        requester_role: HouseholdRole::ParentOwner,
        same_family: true,
        kind,
        state: RecoveryState::Approved,
        owner_approval_required: false,
        identity_proof_state: RecoveryIdentityProofState::Verified,
        support_channel: RecoverySupportChannel::SelfServe,
        delete_export_handoff_required: false,
        abuse_state: SetupRecoveryAbuseState::WithinLimit,
        response_timing_state: SetupRecoveryResponseTimingState::Uniform,
    }
}

#[test]
fn active_parent_can_issue_matching_coparent_invite() {
    let decision = authorize_setup_invite(parent_member_invite(
        SetupInvitePurpose::CoParentInvite,
        SetupInviteTargetRole::CoParentGuardian,
    ));

    assert_eq!(
        decision.decision_state,
        SetupInviteDecisionState::Acceptable
    );
    assert_eq!(
        decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(decision.failure_reason, None);
}

#[test]
fn revoked_or_expired_invites_are_rejected() {
    let revoked = authorize_setup_invite(SetupInviteInput {
        invite_state: SetupInviteState::Revoked,
        ..parent_member_invite(
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer,
        )
    });

    assert_eq!(revoked.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        revoked.failure_reason,
        Some(SetupInviteFailureReason::InviteNotActive)
    );

    let expired = authorize_setup_invite(SetupInviteInput {
        invite_state: SetupInviteState::Expired,
        ..parent_member_invite(
            SetupInvitePurpose::CoParentInvite,
            SetupInviteTargetRole::CoParentGuardian,
        )
    });

    assert_eq!(expired.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        expired.failure_reason,
        Some(SetupInviteFailureReason::InviteNotActive)
    );
}

#[test]
fn replayed_or_non_single_use_invites_are_rejected() {
    let replayed = authorize_setup_invite(SetupInviteInput {
        replay_state: SetupInviteReplayState::ReplayDetected,
        ..parent_member_invite(
            SetupInvitePurpose::CoParentInvite,
            SetupInviteTargetRole::CoParentGuardian,
        )
    });

    assert_eq!(replayed.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        replayed.failure_reason,
        Some(SetupInviteFailureReason::InviteReplayRejected)
    );

    let reusable = authorize_setup_invite(SetupInviteInput {
        single_use: false,
        ..parent_member_invite(
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer,
        )
    });

    assert_eq!(reusable.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        reusable.failure_reason,
        Some(SetupInviteFailureReason::InviteNotSingleUse)
    );
}

#[test]
fn observer_and_child_device_pairing_invites_are_accepted_in_scope() {
    let observer = authorize_setup_invite(parent_member_invite(
        SetupInvitePurpose::ObserverInvite,
        SetupInviteTargetRole::Observer,
    ));

    assert_eq!(
        observer.decision_state,
        SetupInviteDecisionState::Acceptable
    );
    assert_eq!(
        observer.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(observer.failure_reason, None);

    let child_device_pairing = authorize_setup_invite(parent_member_invite(
        SetupInvitePurpose::ChildDevicePairing,
        SetupInviteTargetRole::ChildDeviceAgent,
    ));

    assert_eq!(
        child_device_pairing.decision_state,
        SetupInviteDecisionState::Acceptable
    );
    assert_eq!(
        child_device_pairing.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(child_device_pairing.failure_reason, None);
}

#[test]
fn child_device_pairing_invite_rejects_wrong_target_role() {
    let decision = authorize_setup_invite(parent_member_invite(
        SetupInvitePurpose::ChildDevicePairing,
        SetupInviteTargetRole::Observer,
    ));

    assert_eq!(decision.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        decision.failure_reason,
        Some(SetupInviteFailureReason::WrongTargetRole)
    );
}

#[test]
fn non_member_household_cannot_accept_household_transfer_invite() {
    let decision = authorize_setup_invite(SetupInviteInput {
        same_family: false,
        purpose: SetupInvitePurpose::HouseholdTransfer,
        target_role: SetupInviteTargetRole::ParentOwner,
        ..parent_member_invite(
            SetupInvitePurpose::HouseholdTransfer,
            SetupInviteTargetRole::ParentOwner,
        )
    });

    assert_eq!(decision.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        decision.failure_reason,
        Some(SetupInviteFailureReason::WrongHousehold)
    );
}

#[test]
fn guardian_cannot_issue_household_transfer_invite() {
    let decision = authorize_setup_invite(SetupInviteInput {
        inviter_role: HouseholdRole::CoParentGuardian,
        purpose: SetupInvitePurpose::HouseholdTransfer,
        target_role: SetupInviteTargetRole::ParentOwner,
        ..parent_member_invite(
            SetupInvitePurpose::HouseholdTransfer,
            SetupInviteTargetRole::ParentOwner,
        )
    });

    assert_eq!(decision.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        decision.failure_reason,
        Some(SetupInviteFailureReason::InviterNotAuthorized)
    );
}

#[test]
fn lost_parent_device_recovery_requires_owner_approval() {
    let lost_parent_device = evaluate_recovery_operation(RecoveryOperation {
        kind: RecoveryKind::LostParentDevice,
        state: RecoveryState::OwnerApprovalRequired,
        ..parent_member_recovery(RecoveryKind::LostParentDevice)
    });

    assert_eq!(
        lost_parent_device.decision_state,
        RecoveryDecisionState::Authorized
    );
    assert!(lost_parent_device.owner_approval_required);
    assert_eq!(
        lost_parent_device.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        lost_parent_device.child_evidence_access_state,
        RecoveryChildEvidenceAccessState::Allowed
    );
    assert_eq!(
        lost_parent_device.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::None
    );
    assert_eq!(lost_parent_device.failure_reason, None);
}

#[test]
fn child_reinstall_recovery_does_not_require_owner_approval() {
    let child_reinstall =
        evaluate_recovery_operation(parent_member_recovery(RecoveryKind::ChildReinstall));

    assert_eq!(
        child_reinstall.decision_state,
        RecoveryDecisionState::Authorized
    );
    assert!(!child_reinstall.owner_approval_required);
    assert_eq!(child_reinstall.failure_reason, None);
}

#[test]
fn delete_export_recovery_routes_to_data_custody() {
    let decision = evaluate_recovery_operation(RecoveryOperation {
        kind: RecoveryKind::ForgotLogin,
        delete_export_handoff_required: true,
        ..parent_member_recovery(RecoveryKind::ForgotLogin)
    });

    assert_eq!(decision.decision_state, RecoveryDecisionState::Authorized);
    assert_eq!(
        decision.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::ExportDeleteHandoffRequired
    );
    assert_eq!(decision.failure_reason, None);
}

#[test]
fn compromised_account_and_household_transfer_recovery_keep_owner_approval_parity() {
    let compromised_account =
        evaluate_recovery_operation(parent_member_recovery(RecoveryKind::CompromisedAccount));

    assert_eq!(
        compromised_account.decision_state,
        RecoveryDecisionState::Authorized
    );
    assert!(compromised_account.owner_approval_required);
    assert_eq!(
        compromised_account.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        compromised_account.child_evidence_access_state,
        RecoveryChildEvidenceAccessState::Allowed
    );
    assert_eq!(
        compromised_account.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::None
    );
    assert_eq!(compromised_account.failure_reason, None);

    let household_transfer =
        evaluate_recovery_operation(parent_member_recovery(RecoveryKind::HouseholdTransfer));

    assert_eq!(
        household_transfer.decision_state,
        RecoveryDecisionState::Authorized
    );
    assert!(household_transfer.owner_approval_required);
    assert_eq!(
        household_transfer.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        household_transfer.child_evidence_access_state,
        RecoveryChildEvidenceAccessState::Allowed
    );
    assert_eq!(
        household_transfer.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::HouseholdTransferHandoffRequired
    );
    assert_eq!(household_transfer.failure_reason, None);
}

#[test]
fn support_assisted_recovery_is_audited_and_cannot_access_child_evidence() {
    let decision = evaluate_recovery_operation(RecoveryOperation {
        requester_role: HouseholdRole::SupportAdmin,
        same_family: false,
        kind: RecoveryKind::CompromisedAccount,
        support_channel: RecoverySupportChannel::SupportAssisted,
        ..parent_member_recovery(RecoveryKind::CompromisedAccount)
    });

    assert_eq!(decision.decision_state, RecoveryDecisionState::Authorized);
    assert_eq!(
        decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        decision.child_evidence_access_state,
        RecoveryChildEvidenceAccessState::Blocked
    );
    assert_eq!(decision.failure_reason, None);
}

#[test]
fn observer_and_revoked_recovery_paths_are_rejected() {
    let observer = evaluate_recovery_operation(RecoveryOperation {
        requester_role: HouseholdRole::Observer,
        ..parent_member_recovery(RecoveryKind::ForgotLogin)
    });

    assert_eq!(observer.decision_state, RecoveryDecisionState::Rejected);
    assert_eq!(
        observer.failure_reason,
        Some(RecoveryFailureReason::RoleNotAuthorized)
    );

    let revoked = evaluate_recovery_operation(RecoveryOperation {
        state: RecoveryState::Revoked,
        ..parent_member_recovery(RecoveryKind::ForgotLogin)
    });

    assert_eq!(revoked.decision_state, RecoveryDecisionState::Rejected);
    assert_eq!(
        revoked.failure_reason,
        Some(RecoveryFailureReason::RecoveryNotActive)
    );
}

#[test]
fn household_transfer_without_identity_proof_is_rejected_and_keeps_custody_handoff() {
    let decision = evaluate_recovery_operation(RecoveryOperation {
        kind: RecoveryKind::HouseholdTransfer,
        identity_proof_state: RecoveryIdentityProofState::Pending,
        ..parent_member_recovery(RecoveryKind::HouseholdTransfer)
    });

    assert_eq!(decision.decision_state, RecoveryDecisionState::Rejected);
    assert!(decision.owner_approval_required);
    assert_eq!(
        decision.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::HouseholdTransferHandoffRequired
    );
    assert_eq!(
        decision.failure_reason,
        Some(RecoveryFailureReason::IdentityProofRequired)
    );
}

#[test]
fn throttled_or_variable_timing_invite_is_rejected_as_inactive() {
    let throttled = authorize_setup_invite(SetupInviteInput {
        abuse_state: SetupRecoveryAbuseState::Throttled,
        ..parent_member_invite(
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer,
        )
    });
    assert_eq!(
        throttled.failure_reason,
        Some(SetupInviteFailureReason::InviteNotActive)
    );

    let variable_timing = authorize_setup_invite(SetupInviteInput {
        response_timing_state: SetupRecoveryResponseTimingState::Variable,
        ..parent_member_invite(
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer,
        )
    });
    assert_eq!(
        variable_timing.failure_reason,
        Some(SetupInviteFailureReason::InviteNotActive)
    );
}

#[test]
fn throttled_or_variable_timing_recovery_requires_identity_proof() {
    let throttled = evaluate_recovery_operation(RecoveryOperation {
        abuse_state: SetupRecoveryAbuseState::Throttled,
        ..parent_member_recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(
        throttled.failure_reason,
        Some(RecoveryFailureReason::IdentityProofRequired)
    );

    let variable_timing = evaluate_recovery_operation(RecoveryOperation {
        response_timing_state: SetupRecoveryResponseTimingState::Variable,
        ..parent_member_recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(
        variable_timing.failure_reason,
        Some(RecoveryFailureReason::IdentityProofRequired)
    );
}

#[test]
fn recovery_device_trust_projection_matches_canonical_states() {
    assert_eq!(
        device_trust_state_for_recovery_state(RecoveryState::PendingIdentityProof),
        DeviceTrustState::ResetRequired
    );
    assert_eq!(
        device_trust_state_for_recovery_state(RecoveryState::OwnerApprovalRequired),
        DeviceTrustState::ResetRequired
    );
    assert_eq!(
        device_trust_state_for_recovery_state(RecoveryState::Approved),
        DeviceTrustState::ResetRequired
    );
    assert_eq!(
        device_trust_state_for_recovery_state(RecoveryState::Completed),
        DeviceTrustState::Pending
    );
    assert_eq!(
        device_trust_state_for_recovery_state(RecoveryState::Revoked),
        DeviceTrustState::Revoked
    );
}

#[test]
fn completed_delete_export_recovery_keeps_device_reset_required() {
    let trust_state = device_trust_state_for_recovery_operation(RecoveryOperation {
        state: RecoveryState::Completed,
        delete_export_handoff_required: true,
        ..parent_member_recovery(RecoveryKind::ForgotLogin)
    });

    assert_eq!(trust_state, DeviceTrustState::ResetRequired);
}
