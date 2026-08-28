use ocentra_family_identity_core::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use ocentra_family_identity_core::household_authority::{
    HouseholdAuthorityAction, HouseholdAuthorityInput, ParentControllerLeaseState,
};
use ocentra_family_identity_core::session_lifecycle::{
    SessionActivityState, SessionCredentialKind, SessionLifecycleAction, SessionTokenInput,
    TokenReplayState, TokenValidityWindowState,
};
use ocentra_family_identity_core::setup_lifecycle::{
    RecoveryIdentityProofState, RecoveryKind as FamilyRecoveryKind,
    RecoveryOperation as FamilyRecoveryOperation, RecoveryState as FamilyRecoveryState,
    RecoverySupportChannel, SetupInviteInput, SetupInvitePurpose, SetupInviteReplayState,
    SetupInviteState, SetupInviteTargetRole, SetupRecoveryAbuseState,
    SetupRecoveryResponseTimingState,
};
use ocentra_provisioning_core::provisioning_install::{
    derive_provisioning_readiness_input_from_family_context, evaluate_provisioning_readiness,
    plan_provisioning_actions, AccountReadinessState, ChildAppReadinessState, ChildInstallState,
    ChildRuntimeReadinessState, ChildServiceState, DataCustodySyncState, NetworkReachabilityState,
    PairingLifecycleState, ParentAppReadinessState, ParentDeviceRegistrationState,
    PermissionReadinessState, PolicyBaselineState, ProvisioningActionPlan, ProvisioningAuditState,
    ProvisioningBlockerReason, ProvisioningChildRuntimeStartAction, ProvisioningFamilyContextInput,
    ProvisioningManualStepState, ProvisioningOverallState, ProvisioningReadinessDecision,
    ProvisioningReadinessInput, ProvisioningRecoveryAction, RecoveryState,
};

fn ready_input() -> ProvisioningReadinessInput {
    ProvisioningReadinessInput {
        membership_state: HouseholdMembershipState::Active,
        account_readiness_state: AccountReadinessState::Ready,
        parent_app_readiness_state: ParentAppReadinessState::Ready,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_install_state: ChildInstallState::Installed,
        child_service_state: ChildServiceState::ServiceStarted,
        child_app_readiness_state: ChildAppReadinessState::Ready,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        device_trust_state: DeviceTrustState::Trusted,
        permission_readiness_state: PermissionReadinessState::Granted,
        pairing_lifecycle_state: PairingLifecycleState::Trusted,
        policy_baseline_state: PolicyBaselineState::Applied,
        data_custody_sync_state: DataCustodySyncState::Synced,
        network_reachability_state: NetworkReachabilityState::Reachable,
        recovery_state: RecoveryState::Normal,
    }
}

fn ready_family_context() -> ProvisioningFamilyContextInput {
    ProvisioningFamilyContextInput {
        account_matches_invite_target: true,
        setup_invite_input: SetupInviteInput {
            inviter_role: HouseholdRole::ParentOwner,
            same_family: true,
            purpose: SetupInvitePurpose::ChildDevicePairing,
            target_role: SetupInviteTargetRole::ChildDeviceAgent,
            invite_state: SetupInviteState::Accepted,
            single_use: true,
            replay_state: SetupInviteReplayState::Fresh,
            abuse_state: SetupRecoveryAbuseState::WithinLimit,
            response_timing_state: SetupRecoveryResponseTimingState::Uniform,
        },
        pairing_session_input: SessionTokenInput {
            credential_kind: SessionCredentialKind::PairingToken,
            action: SessionLifecycleAction::AcceptPairingToken,
            activity_state: SessionActivityState::Active,
            replay_state: TokenReplayState::Fresh,
            validity_window_state: TokenValidityWindowState::Valid,
            session_freshness_state: SessionFreshnessState::Fresh,
        },
        household_authority_input: HouseholdAuthorityInput {
            actor_role: HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ActorAccountState::Active,
            membership_state: HouseholdMembershipState::Active,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
            device_trust_state: DeviceTrustState::Trusted,
            session_freshness_state: SessionFreshnessState::Fresh,
            capability_granted: true,
            controller_lease_state: Some(ParentControllerLeaseState::Active),
            action: HouseholdAuthorityAction::PairChildDevice,
        },
        recovery_operation: None,
        parent_app_readiness_state: ParentAppReadinessState::Ready,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_install_state: ChildInstallState::Installed,
        child_service_state: ChildServiceState::ServiceStarted,
        child_app_readiness_state: ChildAppReadinessState::Ready,
        permission_readiness_state: PermissionReadinessState::Granted,
        policy_baseline_state: PolicyBaselineState::Applied,
        data_custody_sync_state: DataCustodySyncState::Synced,
        network_reachability_state: NetworkReachabilityState::Reachable,
    }
}

fn evaluate_family_context(
    input: ProvisioningFamilyContextInput,
) -> (
    ProvisioningReadinessInput,
    ProvisioningReadinessDecision,
    ProvisioningActionPlan,
) {
    let projected_input = derive_provisioning_readiness_input_from_family_context(input);
    let readiness = evaluate_provisioning_readiness(projected_input);
    let actions = plan_provisioning_actions(projected_input);

    (projected_input, readiness, actions)
}

#[test]
fn ready_provisioning_starts_child_runtime_without_manual_step() {
    let input = ready_input();

    let readiness = evaluate_provisioning_readiness(input);
    let actions = plan_provisioning_actions(input);

    assert_eq!(
        readiness.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        readiness.manual_step_state,
        ProvisioningManualStepState::NotRequired
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Ready);
    assert_eq!(readiness.blocker_reason, None);
    assert_eq!(
        actions.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::Start
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::Continue
    );
    assert_eq!(actions.audit_state, ProvisioningAuditState::Record);
}

#[test]
fn custody_sync_pending_keeps_runtime_blocked_but_marks_state_degraded() {
    let input = ProvisioningReadinessInput {
        data_custody_sync_state: DataCustodySyncState::SyncPending,
        ..ready_input()
    };

    let readiness = evaluate_provisioning_readiness(input);
    let actions = plan_provisioning_actions(input);

    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::DataCustodySyncPending)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Degraded);
    assert_eq!(
        actions.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::DoNotStart
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RepairCustodySync
    );
}

#[test]
fn family_context_projects_trusted_pairing_into_ready_provisioning() {
    let (projected_input, readiness, actions) = evaluate_family_context(ready_family_context());

    assert_eq!(
        projected_input.account_readiness_state,
        AccountReadinessState::Ready
    );
    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Trusted
    );
    assert_eq!(projected_input.recovery_state, RecoveryState::Normal);
    assert_eq!(
        readiness.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::Continue
    );
}

#[test]
fn family_context_installed_not_started_projects_explicit_start_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            child_service_state: ChildServiceState::NotStarted,
            child_app_readiness_state: ChildAppReadinessState::Installed,
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.child_install_state,
        ChildInstallState::Installed
    );
    assert_eq!(
        projected_input.child_service_state,
        ChildServiceState::NotStarted
    );
    assert_eq!(
        projected_input.child_app_readiness_state,
        ChildAppReadinessState::Installed
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildServiceNotStarted)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::StartChildService
    );
}

#[test]
fn family_context_offline_service_projects_degraded_runtime_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            child_service_state: ChildServiceState::Offline,
            child_app_readiness_state: ChildAppReadinessState::Offline,
            network_reachability_state: NetworkReachabilityState::OfflineChild,
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.child_install_state,
        ChildInstallState::Installed
    );
    assert_eq!(
        projected_input.child_service_state,
        ChildServiceState::Offline
    );
    assert_eq!(
        projected_input.child_app_readiness_state,
        ChildAppReadinessState::Offline
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppOffline)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Degraded);
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::WaitForChildConnectivity
    );
}

#[test]
fn family_context_reinstall_required_projects_reinstall_recovery() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            child_install_state: ChildInstallState::ReinstallRequired,
            child_service_state: ChildServiceState::NotStarted,
            child_app_readiness_state: ChildAppReadinessState::ReinstallRequired,
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.child_install_state,
        ChildInstallState::ReinstallRequired
    );
    assert_eq!(
        projected_input.child_service_state,
        ChildServiceState::NotStarted
    );
    assert_eq!(
        projected_input.recovery_state,
        RecoveryState::ChildReinstall
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppReinstallRequired)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::ReinstallChildApp
    );
}

#[test]
fn family_context_replay_detection_reissues_pairing_code() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            pairing_session_input: SessionTokenInput {
                replay_state: TokenReplayState::ReplayDetected,
                ..ready_family_context().pairing_session_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Replayed
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingReplayRejected)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::ReissuePairingCode
    );
}

#[test]
fn family_context_accepted_pairing_waits_for_parent_trust_confirmation() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            setup_invite_input: SetupInviteInput {
                invite_state: SetupInviteState::Accepted,
                ..ready_family_context().setup_invite_input
            },
            household_authority_input: HouseholdAuthorityInput {
                device_trust_state: DeviceTrustState::Pending,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Accepted
    );
    assert_eq!(
        projected_input.device_trust_state,
        DeviceTrustState::Pending
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildDeviceTrustRequired)
    );
    assert_eq!(readiness.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::TrustChildDevice
    );
}

#[test]
fn family_context_accepted_pairing_revalidates_invite_replay_state() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            setup_invite_input: SetupInviteInput {
                replay_state: SetupInviteReplayState::ReplayDetected,
                ..ready_family_context().setup_invite_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::Replayed
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingReplayRejected)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::ReissuePairingCode
    );
}

#[test]
fn family_context_accepted_pairing_rejects_mis_scoped_invite_metadata() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            setup_invite_input: SetupInviteInput {
                target_role: SetupInviteTargetRole::Observer,
                ..ready_family_context().setup_invite_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::WrongDevice
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingWrongDevice)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RePairChildDevice
    );
}

#[test]
fn family_context_accepted_pairing_rejects_reusable_or_throttled_invites() {
    for setup_invite_input in [
        SetupInviteInput {
            single_use: false,
            ..ready_family_context().setup_invite_input
        },
        SetupInviteInput {
            abuse_state: SetupRecoveryAbuseState::Throttled,
            ..ready_family_context().setup_invite_input
        },
        SetupInviteInput {
            response_timing_state: SetupRecoveryResponseTimingState::Variable,
            ..ready_family_context().setup_invite_input
        },
    ] {
        let (projected_input, readiness, actions) =
            evaluate_family_context(ProvisioningFamilyContextInput {
                setup_invite_input,
                ..ready_family_context()
            });

        assert_eq!(
            projected_input.pairing_lifecycle_state,
            PairingLifecycleState::Revoked
        );
        assert_eq!(
            readiness.blocker_reason,
            Some(ProvisioningBlockerReason::PairingRevoked)
        );
        assert_eq!(
            actions.recovery_action,
            ProvisioningRecoveryAction::RePairChildDevice
        );
    }
}

#[test]
fn family_context_accepted_pairing_requires_authorized_inviter_role() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            setup_invite_input: SetupInviteInput {
                inviter_role: HouseholdRole::Observer,
                ..ready_family_context().setup_invite_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::ParentRoleRequired
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingParentRoleRequired)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::SwitchToCorrectAccount
    );
}

#[test]
fn family_context_expired_and_revoked_invites_remain_rejected() {
    for (invite_state, pairing_state, blocker_reason, recovery_action) in [
        (
            SetupInviteState::Expired,
            PairingLifecycleState::Expired,
            ProvisioningBlockerReason::PairingExpired,
            ProvisioningRecoveryAction::ReissuePairingCode,
        ),
        (
            SetupInviteState::Revoked,
            PairingLifecycleState::Revoked,
            ProvisioningBlockerReason::PairingRevoked,
            ProvisioningRecoveryAction::RePairChildDevice,
        ),
    ] {
        let (projected_input, readiness, actions) =
            evaluate_family_context(ProvisioningFamilyContextInput {
                setup_invite_input: SetupInviteInput {
                    invite_state,
                    ..ready_family_context().setup_invite_input
                },
                ..ready_family_context()
            });

        assert_eq!(projected_input.pairing_lifecycle_state, pairing_state);
        assert_eq!(readiness.blocker_reason, Some(blocker_reason));
        assert_eq!(actions.recovery_action, recovery_action);
    }
}

#[test]
fn family_context_wrong_household_surfaces_pairing_blocker() {
    let (projected_input, readiness, _) = evaluate_family_context(ProvisioningFamilyContextInput {
        setup_invite_input: SetupInviteInput {
            same_family: false,
            invite_state: SetupInviteState::Pending,
            ..ready_family_context().setup_invite_input
        },
        ..ready_family_context()
    });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::WrongHousehold
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingWrongHousehold)
    );
}

#[test]
fn family_context_wrong_device_scope_surfaces_explicit_pairing_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            household_authority_input: HouseholdAuthorityInput {
                device_ownership_scope: DeviceOwnershipScope::OtherDevice,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::WrongDevice
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingWrongDevice)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RePairChildDevice
    );
}

#[test]
fn family_context_unbound_child_profile_surfaces_anonymous_device_pairing_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            household_authority_input: HouseholdAuthorityInput {
                child_profile_binding_state: ChildProfileBindingState::Missing,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::AnonymousDevice
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingAnonymousDevice)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RePairChildDevice
    );
}

#[test]
fn family_context_parent_role_required_surfaces_explicit_pairing_blocker() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            household_authority_input: HouseholdAuthorityInput {
                actor_role: HouseholdRole::Observer,
                ..ready_family_context().household_authority_input
            },
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.pairing_lifecycle_state,
        PairingLifecycleState::ParentRoleRequired
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::PairingParentRoleRequired)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::SwitchToCorrectAccount
    );
}

#[test]
fn family_context_stale_signed_hello_reissues_pairing_code() {
    for validity_window_state in [
        TokenValidityWindowState::Expired,
        TokenValidityWindowState::NotYetValid,
    ] {
        let (projected_input, readiness, actions) =
            evaluate_family_context(ProvisioningFamilyContextInput {
                pairing_session_input: SessionTokenInput {
                    validity_window_state,
                    ..ready_family_context().pairing_session_input
                },
                ..ready_family_context()
            });

        assert_eq!(
            projected_input.pairing_lifecycle_state,
            PairingLifecycleState::StaleSignedHello
        );
        assert_eq!(
            readiness.blocker_reason,
            Some(ProvisioningBlockerReason::PairingStaleSignedHello)
        );
        assert_eq!(
            actions.recovery_action,
            ProvisioningRecoveryAction::ReissuePairingCode
        );
    }
}

#[test]
fn family_context_support_recovery_handoff_projects_reset_required_device_trust() {
    let (projected_input, readiness, actions) =
        evaluate_family_context(ProvisioningFamilyContextInput {
            recovery_operation: Some(FamilyRecoveryOperation {
                requester_role: HouseholdRole::ParentOwner,
                same_family: true,
                kind: FamilyRecoveryKind::CompromisedAccount,
                state: FamilyRecoveryState::Approved,
                owner_approval_required: false,
                identity_proof_state: RecoveryIdentityProofState::Verified,
                support_channel: RecoverySupportChannel::SupportAssisted,
                delete_export_handoff_required: true,
                abuse_state: SetupRecoveryAbuseState::WithinLimit,
                response_timing_state: SetupRecoveryResponseTimingState::Uniform,
            }),
            ..ready_family_context()
        });

    assert_eq!(
        projected_input.data_custody_sync_state,
        DataCustodySyncState::Blocked
    );
    assert_eq!(
        projected_input.recovery_state,
        RecoveryState::PermissionLoss
    );
    assert_eq!(
        projected_input.device_trust_state,
        DeviceTrustState::ResetRequired
    );
    assert_eq!(
        readiness.blocker_reason,
        Some(ProvisioningBlockerReason::ChildDeviceTrustRequired)
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::TrustChildDevice
    );
}
