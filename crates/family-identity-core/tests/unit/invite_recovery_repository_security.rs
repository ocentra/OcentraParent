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

fn invite(purpose: SetupInvitePurpose, target_role: SetupInviteTargetRole) -> SetupInviteInput {
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

fn recovery(kind: RecoveryKind) -> RecoveryOperation {
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
fn purpose_and_target_role_are_bound_one_to_one() {
    let decision = authorize_setup_invite(invite(
        SetupInvitePurpose::CoParentInvite,
        SetupInviteTargetRole::Observer,
    ));

    assert_eq!(decision.decision_state, SetupInviteDecisionState::Rejected);
    assert_eq!(
        decision.failure_reason,
        Some(SetupInviteFailureReason::WrongTargetRole)
    );
}

#[test]
fn cross_household_and_wrong_role_requests_are_rejected() {
    let cross_household = authorize_setup_invite(SetupInviteInput {
        same_family: false,
        ..invite(
            SetupInvitePurpose::ObserverInvite,
            SetupInviteTargetRole::Observer,
        )
    });
    assert_eq!(
        cross_household.failure_reason,
        Some(SetupInviteFailureReason::WrongHousehold)
    );

    let recovery_cross_household = evaluate_recovery_operation(RecoveryOperation {
        same_family: false,
        ..recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(
        recovery_cross_household.failure_reason,
        Some(RecoveryFailureReason::WrongHousehold)
    );

    let wrong_role = evaluate_recovery_operation(RecoveryOperation {
        requester_role: HouseholdRole::Observer,
        ..recovery(RecoveryKind::ChildReinstall)
    });
    assert_eq!(wrong_role.decision_state, RecoveryDecisionState::Rejected);
    assert_eq!(
        wrong_role.failure_reason,
        Some(RecoveryFailureReason::RoleNotAuthorized)
    );
}

#[test]
fn inactive_replayed_throttled_and_non_uniform_invites_are_rejected() {
    let cases = [
        (
            SetupInviteInput {
                invite_state: SetupInviteState::Expired,
                ..invite(
                    SetupInvitePurpose::ObserverInvite,
                    SetupInviteTargetRole::Observer,
                )
            },
            SetupInviteFailureReason::InviteNotActive,
        ),
        (
            SetupInviteInput {
                replay_state: SetupInviteReplayState::ReplayDetected,
                ..invite(
                    SetupInvitePurpose::ObserverInvite,
                    SetupInviteTargetRole::Observer,
                )
            },
            SetupInviteFailureReason::InviteReplayRejected,
        ),
        (
            SetupInviteInput {
                abuse_state: SetupRecoveryAbuseState::Throttled,
                ..invite(
                    SetupInvitePurpose::ObserverInvite,
                    SetupInviteTargetRole::Observer,
                )
            },
            SetupInviteFailureReason::InviteNotActive,
        ),
        (
            SetupInviteInput {
                response_timing_state: SetupRecoveryResponseTimingState::Variable,
                ..invite(
                    SetupInvitePurpose::ObserverInvite,
                    SetupInviteTargetRole::Observer,
                )
            },
            SetupInviteFailureReason::InviteNotActive,
        ),
    ];

    for (input, expected_reason) in cases {
        let decision = authorize_setup_invite(input);
        assert_eq!(decision.decision_state, SetupInviteDecisionState::Rejected);
        assert_eq!(decision.failure_reason, Some(expected_reason));
    }
}

#[test]
fn recovery_requires_verified_identity_and_uniform_anti_abuse_state() {
    let unproved = evaluate_recovery_operation(RecoveryOperation {
        identity_proof_state: RecoveryIdentityProofState::Pending,
        ..recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(unproved.decision_state, RecoveryDecisionState::Rejected);
    assert_eq!(
        unproved.failure_reason,
        Some(RecoveryFailureReason::IdentityProofRequired)
    );

    let throttled = evaluate_recovery_operation(RecoveryOperation {
        abuse_state: SetupRecoveryAbuseState::Throttled,
        ..recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(throttled.decision_state, RecoveryDecisionState::Rejected);
    assert_eq!(
        throttled.failure_reason,
        Some(RecoveryFailureReason::IdentityProofRequired)
    );

    let variable_timing = evaluate_recovery_operation(RecoveryOperation {
        response_timing_state: SetupRecoveryResponseTimingState::Variable,
        ..recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(
        variable_timing.decision_state,
        RecoveryDecisionState::Rejected
    );
    assert_eq!(
        variable_timing.failure_reason,
        Some(RecoveryFailureReason::IdentityProofRequired)
    );
}

#[test]
fn support_recovery_is_audited_and_cannot_read_child_evidence() {
    let decision = evaluate_recovery_operation(RecoveryOperation {
        requester_role: HouseholdRole::SupportAdmin,
        same_family: false,
        support_channel: RecoverySupportChannel::SupportAssisted,
        kind: RecoveryKind::CompromisedAccount,
        state: RecoveryState::OwnerApprovalRequired,
        ..recovery(RecoveryKind::CompromisedAccount)
    });

    assert_eq!(decision.decision_state, RecoveryDecisionState::Authorized);
    assert!(decision.owner_approval_required);
    assert_eq!(
        decision.audit_requirement_state,
        AuditRequirementState::Required
    );
    assert_eq!(
        decision.child_evidence_access_state,
        RecoveryChildEvidenceAccessState::Blocked
    );
}

#[test]
fn recovery_handoff_kind_is_explicit_for_delete_export_and_transfer() {
    let export_delete = evaluate_recovery_operation(RecoveryOperation {
        delete_export_handoff_required: true,
        ..recovery(RecoveryKind::ForgotLogin)
    });
    assert_eq!(
        export_delete.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::ExportDeleteHandoffRequired
    );

    let transfer = evaluate_recovery_operation(recovery(RecoveryKind::HouseholdTransfer));
    assert_eq!(
        transfer.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::HouseholdTransferHandoffRequired
    );
}

#[test]
fn recovery_device_trust_mapping_keeps_terminal_states_monotonic() {
    assert_eq!(
        device_trust_state_for_recovery_state(RecoveryState::PendingIdentityProof),
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
    assert_eq!(
        device_trust_state_for_recovery_operation(RecoveryOperation {
            state: RecoveryState::Completed,
            delete_export_handoff_required: true,
            ..recovery(RecoveryKind::ForgotLogin)
        }),
        DeviceTrustState::ResetRequired
    );
}

#[test]
fn lifecycle_decisions_serialize_stable_wire_labels_without_bearer_values() {
    let decision = evaluate_recovery_operation(RecoveryOperation {
        delete_export_handoff_required: true,
        ..recovery(RecoveryKind::ForgotLogin)
    });
    let encoded = serde_json::to_value(decision).expect("serialize recovery decision");

    assert_eq!(encoded["decision_state"], "authorized");
    assert_eq!(
        encoded["data_custody_handoff_state"],
        "export-delete-handoff-required"
    );
    assert!(encoded.get("token").is_none());
    assert!(encoded.get("recovery_token").is_none());
}
