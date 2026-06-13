use ocentra_family_identity_core::{DeviceOwnershipScope, HouseholdMembership};
use ocentra_provisioning_core::{
    evaluate_provisioning_readiness, ChildRuntimeReadinessState, PairingTokenState,
    ParentDeviceRegistrationState, ProvisioningManualStepState, ProvisioningReadinessInput,
    RecoveryState, RequiredPermissionState,
};

#[test]
fn provisioning_is_ready_after_household_parent_child_and_permissions_exist() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::NotRequired
    );
}

#[test]
fn provisioning_requires_manual_steps_until_child_device_is_paired() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::OtherDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::Required
    );
}

#[test]
fn provisioning_requires_household_membership_before_child_runtime_ready() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::External,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::Required
    );
}

#[test]
fn provisioning_requires_manual_steps_until_parent_device_is_registered() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Missing,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::Required
    );
}

#[test]
fn provisioning_requires_manual_steps_until_required_permissions_are_granted() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Missing,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::Required
    );
}

#[test]
fn provisioning_requires_valid_pairing_token() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::MissingOrExpired,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::Required
    );
}

#[test]
fn provisioning_requires_recovery_resolution_before_child_runtime_ready() {
    let decision = evaluate_provisioning_readiness(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::RecoveryRequired,
    });

    assert_eq!(
        decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(
        decision.manual_step_state,
        ProvisioningManualStepState::Required
    );
}
