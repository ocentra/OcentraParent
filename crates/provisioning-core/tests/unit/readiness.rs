use ocentra_eventing::{envelope::DomainEvent, error::EventingError};
use ocentra_family_identity_core::family_identity::{
    DeviceOwnershipScope, DeviceTrustState, HouseholdMembershipState,
};
use ocentra_provisioning_core::provisioning_install::{
    evaluate_provisioning_readiness, plan_provisioning_actions, provisioning_action_planned_event,
    provisioning_readiness_evaluated_event, AccountReadinessState, ChildAppReadinessState,
    ChildInstallState, ChildRuntimeReadinessState, ChildServiceState, DataCustodySyncState,
    NetworkReachabilityState, PairingLifecycleState, ParentAppReadinessState,
    ParentDeviceRegistrationState, PermissionReadinessState, PolicyBaselineState,
    ProvisioningActionPlan, ProvisioningAggregateId, ProvisioningBlockerReason,
    ProvisioningChildRuntimeStartAction, ProvisioningManualStepState, ProvisioningOverallState,
    ProvisioningReadinessDecision, ProvisioningReadinessEvaluationId, ProvisioningReadinessInput,
    ProvisioningRecoveryAction, RecoveryState,
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

fn evaluate_and_plan(
    input: ProvisioningReadinessInput,
) -> (ProvisioningReadinessDecision, ProvisioningActionPlan) {
    (
        evaluate_provisioning_readiness(input),
        plan_provisioning_actions(input),
    )
}

type FailClosedReadinessCase = (
    ProvisioningReadinessInput,
    ProvisioningBlockerReason,
    ProvisioningRecoveryAction,
    ProvisioningOverallState,
);

fn assert_fail_closed_readiness_cases(cases: impl IntoIterator<Item = FailClosedReadinessCase>) {
    for (input, blocker_reason, recovery_action, overall_state) in cases {
        let (decision, plan) = evaluate_and_plan(input);
        assert_eq!(decision.blocker_reason, Some(blocker_reason));
        assert_eq!(decision.overall_state, overall_state);
        assert_eq!(
            decision.child_runtime_readiness_state,
            ChildRuntimeReadinessState::NotReady
        );
        assert_eq!(
            decision.manual_step_state,
            ProvisioningManualStepState::Required
        );
        assert_eq!(
            plan.child_runtime_start_action,
            ProvisioningChildRuntimeStartAction::DoNotStart
        );
        assert_eq!(plan.recovery_action, recovery_action);
    }
}

#[test]
fn provisioning_blocks_installed_child_until_service_starts() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        child_install_state: ChildInstallState::Installed,
        child_service_state: ChildServiceState::NotStarted,
        child_app_readiness_state: ChildAppReadinessState::Installed,
        ..ready_input()
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildServiceNotStarted)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::StartChildService
    );
}

#[test]
fn provisioning_is_ready_after_household_pairing_permissions_and_policy_are_satisfied() {
    let decision = evaluate_provisioning_readiness(ready_input());

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::NotRequired
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Ready);
    assert_eq!(decision.blocker_reason, None);
}

#[test]
fn provisioning_requires_repair_when_pairing_code_was_replayed() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        pairing_lifecycle_state: PairingLifecycleState::Replayed,
        ..ready_input()
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::PairingReplayRejected)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::ReissuePairingCode
    );
}

#[test]
fn provisioning_rejects_wrong_household_pairing_and_requires_repair() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        pairing_lifecycle_state: PairingLifecycleState::WrongHousehold,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::PairingWrongHousehold)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::RePairChildDevice
    );
}

#[test]
fn provisioning_maps_wrong_device_and_anonymous_device_pairing_states_into_repair_actions() {
    for (pairing_lifecycle_state, blocker_reason) in [
        (
            PairingLifecycleState::WrongDevice,
            ProvisioningBlockerReason::PairingWrongDevice,
        ),
        (
            PairingLifecycleState::AnonymousDevice,
            ProvisioningBlockerReason::PairingAnonymousDevice,
        ),
    ] {
        let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
            pairing_lifecycle_state,
            ..ready_input()
        });

        assert_eq!(decision.blocker_reason, Some(blocker_reason));
        assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
        assert_eq!(
            plan.recovery_action,
            ProvisioningRecoveryAction::RePairChildDevice
        );
    }
}

#[test]
fn provisioning_requires_parent_role_recovery_when_pairing_needs_parent_authority() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        pairing_lifecycle_state: PairingLifecycleState::ParentRoleRequired,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::PairingParentRoleRequired)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::SwitchToCorrectAccount
    );
}

#[test]
fn provisioning_reissues_pairing_code_for_stale_signed_hello_pairing_state() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        pairing_lifecycle_state: PairingLifecycleState::StaleSignedHello,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::PairingStaleSignedHello)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::ReissuePairingCode
    );
}

#[test]
fn provisioning_marks_offline_child_service_as_degraded_not_fake_ready() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        child_install_state: ChildInstallState::Installed,
        child_service_state: ChildServiceState::Offline,
        child_app_readiness_state: ChildAppReadinessState::Offline,
        network_reachability_state: NetworkReachabilityState::OfflineChild,
        ..ready_input()
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppOffline)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Degraded);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::WaitForChildConnectivity
    );
}

#[test]
fn provisioning_requires_reinstall_when_install_state_is_reinstall_required() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        child_install_state: ChildInstallState::ReinstallRequired,
        child_service_state: ChildServiceState::NotStarted,
        child_app_readiness_state: ChildAppReadinessState::ReinstallRequired,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildAppReinstallRequired)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::ReinstallChildApp
    );
}

#[test]
fn provisioning_requires_permission_regrant_when_permission_was_revoked() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        permission_readiness_state: PermissionReadinessState::Revoked,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::PermissionRevoked)
    );
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::RegrantPermissions
    );
}

#[test]
fn provisioning_keeps_accepted_pairing_blocked_until_parent_trusts_child_device() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        device_trust_state: DeviceTrustState::Pending,
        pairing_lifecycle_state: PairingLifecycleState::Accepted,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::ChildDeviceTrustRequired)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::TrustChildDevice
    );
}

#[test]
fn provisioning_requires_policy_baseline_before_runtime_start() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        policy_baseline_state: PolicyBaselineState::Stale,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::PolicyBaselineStale)
    );
    assert_eq!(decision.overall_state, ProvisioningOverallState::Blocked);
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::ApplyPolicyBaseline
    );
}

#[test]
fn provisioning_requires_direct_entry_when_lan_discovery_is_unavailable() {
    let (decision, plan) = evaluate_and_plan(ProvisioningReadinessInput {
        network_reachability_state: NetworkReachabilityState::DirectEntryRequired,
        ..ready_input()
    });

    assert_eq!(
        decision.blocker_reason,
        Some(ProvisioningBlockerReason::NetworkDirectEntryRequired)
    );
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::EnterDirectChildAddress
    );
}

#[test]
fn provisioning_readiness_keeps_account_and_device_owner_inputs_fail_closed() {
    assert_fail_closed_readiness_cases([
        (
            ProvisioningReadinessInput {
                membership_state: HouseholdMembershipState::Pending,
                ..ready_input()
            },
            ProvisioningBlockerReason::HouseholdMemberRequired,
            ProvisioningRecoveryAction::CompleteHouseholdMembership,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                account_readiness_state: AccountReadinessState::RecoveryRequired,
                ..ready_input()
            },
            ProvisioningBlockerReason::AccountRecoveryRequired,
            ProvisioningRecoveryAction::RestoreParentSession,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                parent_app_readiness_state: ParentAppReadinessState::Stale,
                ..ready_input()
            },
            ProvisioningBlockerReason::ParentAppStale,
            ProvisioningRecoveryAction::RepairParentApp,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                parent_device_registration_state: ParentDeviceRegistrationState::Missing,
                ..ready_input()
            },
            ProvisioningBlockerReason::ParentDeviceRegistrationRequired,
            ProvisioningRecoveryAction::ReRegisterParentDevice,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                child_device_ownership_scope: DeviceOwnershipScope::OtherDevice,
                ..ready_input()
            },
            ProvisioningBlockerReason::ChildDeviceScopeRequired,
            ProvisioningRecoveryAction::RePairChildDevice,
            ProvisioningOverallState::Blocked,
        ),
    ]);
}

#[test]
fn provisioning_readiness_keeps_runtime_owner_inputs_fail_closed() {
    assert_fail_closed_readiness_cases([
        (
            ProvisioningReadinessInput {
                permission_readiness_state: PermissionReadinessState::Missing,
                ..ready_input()
            },
            ProvisioningBlockerReason::PermissionMissing,
            ProvisioningRecoveryAction::RequestMissingPermissions,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                pairing_lifecycle_state: PairingLifecycleState::Expired,
                ..ready_input()
            },
            ProvisioningBlockerReason::PairingExpired,
            ProvisioningRecoveryAction::ReissuePairingCode,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                policy_baseline_state: PolicyBaselineState::Missing,
                ..ready_input()
            },
            ProvisioningBlockerReason::PolicyBaselineMissing,
            ProvisioningRecoveryAction::ApplyPolicyBaseline,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                data_custody_sync_state: DataCustodySyncState::Blocked,
                ..ready_input()
            },
            ProvisioningBlockerReason::DataCustodySyncBlocked,
            ProvisioningRecoveryAction::RepairCustodySync,
            ProvisioningOverallState::Blocked,
        ),
        (
            ProvisioningReadinessInput {
                recovery_state: RecoveryState::LostParentDevice,
                ..ready_input()
            },
            ProvisioningBlockerReason::LostParentDeviceRecovery,
            ProvisioningRecoveryAction::RestoreParentSession,
            ProvisioningOverallState::Blocked,
        ),
    ]);
}

#[test]
fn readiness_event_drives_action_plan_event_for_recovered_pairing_state(
) -> Result<(), EventingError> {
    let readiness = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse("provisioning-child-default")?,
        ProvisioningReadinessEvaluationId::parse("provisioning-readiness-default")?,
        ProvisioningReadinessInput {
            pairing_lifecycle_state: PairingLifecycleState::Recovered,
            recovery_state: RecoveryState::Recovered,
            ..ready_input()
        },
    );
    let action = provisioning_action_planned_event(readiness.clone());

    assert_eq!(
        readiness.decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        action.action_plan.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::Start
    );
    assert_eq!(
        action.action_plan.recovery_action,
        ProvisioningRecoveryAction::Continue
    );
    assert_eq!(
        readiness.contract()?.event_type.as_str(),
        "provisioning.readiness.evaluated"
    );
    assert_eq!(
        action.contract()?.event_type.as_str(),
        "provisioning.action.planned"
    );

    Ok(())
}
