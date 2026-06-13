use ocentra_family_identity_core::{
    authorize_setup_invite, evaluate_recovery_operation, FamilyActorRole, HouseholdMembership,
    RecoveryKind, RecoveryOperation, RecoveryState, SetupInviteDecisionState,
    SetupInviteFailureReason, SetupInviteInput, SetupInvitePurpose, SetupInviteState,
    SetupInviteTargetRole,
};

fn parent_member_invite(
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
) -> SetupInviteInput {
    SetupInviteInput {
        inviter_role: FamilyActorRole::Parent,
        household_membership: HouseholdMembership::Member,
        purpose,
        target_role,
        invite_state: SetupInviteState::Pending,
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
        household_membership: HouseholdMembership::External,
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
        inviter_role: FamilyActorRole::Guardian,
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
fn lost_parent_device_and_household_transfer_recovery_require_owner_approval() {
    let lost_parent_device = evaluate_recovery_operation(RecoveryOperation {
        requester_role: FamilyActorRole::Parent,
        household_membership: HouseholdMembership::Member,
        kind: RecoveryKind::LostParentDevice,
        state: RecoveryState::OwnerApprovalRequired,
        owner_approval_required: false,
    });
    assert!(lost_parent_device.owner_approval_required);

    let child_reinstall = evaluate_recovery_operation(RecoveryOperation {
        requester_role: FamilyActorRole::Parent,
        household_membership: HouseholdMembership::Member,
        kind: RecoveryKind::ChildReinstall,
        state: RecoveryState::Approved,
        owner_approval_required: false,
    });
    assert!(!child_reinstall.owner_approval_required);

    let explicit_owner_gate = evaluate_recovery_operation(RecoveryOperation {
        requester_role: FamilyActorRole::Parent,
        household_membership: HouseholdMembership::Member,
        kind: RecoveryKind::ForgotLogin,
        state: RecoveryState::PendingIdentityProof,
        owner_approval_required: true,
    });
    assert!(explicit_owner_gate.owner_approval_required);
}
