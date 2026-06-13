use ocentra_eventing::DomainEvent;
use ocentra_family_identity_core::{DeviceOwnershipScope, HouseholdMembership};
use ocentra_provisioning_core::{
    evaluate_provisioning_readiness, plan_provisioning_actions, provisioning_action_planned_event,
    provisioning_readiness_evaluated_event, ChildRuntimeReadinessState, PairingTokenState,
    ParentDeviceRegistrationState, ProvisioningAggregateId, ProvisioningChildRuntimeStartAction,
    ProvisioningManualStepState, ProvisioningReadinessEvaluationId, ProvisioningReadinessInput,
    ProvisioningRecoveryAction, RecoveryState, RequiredPermissionState,
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

#[test]
fn ready_provisioning_plan_starts_child_runtime() {
    let plan = plan_provisioning_actions(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        plan.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::Start
    );
    assert_eq!(plan.recovery_action, ProvisioningRecoveryAction::Continue);
}

#[test]
fn expired_pairing_plan_refreshes_token_without_starting_runtime() {
    let plan = plan_provisioning_actions(ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::MissingOrExpired,
        recovery_state: RecoveryState::Normal,
    });

    assert_eq!(
        plan.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::DoNotStart
    );
    assert_eq!(
        plan.recovery_action,
        ProvisioningRecoveryAction::RefreshPairingToken
    );
}

#[test]
fn readiness_event_drives_action_plan_event_for_child_runtime_start() {
    let readiness = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse("provisioning-child-default")
            .expect("provisioning aggregate"),
        ProvisioningReadinessEvaluationId::parse("provisioning-readiness-default")
            .expect("provisioning readiness"),
        ProvisioningReadinessInput {
            household_membership: HouseholdMembership::Member,
            parent_device_registration_state: ParentDeviceRegistrationState::Registered,
            child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
            required_permission_state: RequiredPermissionState::Granted,
            pairing_token_state: PairingTokenState::Valid,
            recovery_state: RecoveryState::Normal,
        },
    );
    let action = provisioning_action_planned_event(readiness.clone());

    assert_eq!(
        readiness.decision.child_runtime_readiness_state,
        ChildRuntimeReadinessState::Ready
    );
    assert_eq!(action.aggregate_id, readiness.aggregate_id);
    assert_eq!(action.source_evaluation_id, readiness.evaluation_id);
    assert_eq!(
        action.action_plan.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::Start
    );
    assert_eq!(
        readiness
            .contract()
            .expect("provisioning readiness contract")
            .event_type
            .as_str(),
        "provisioning.readiness.evaluated"
    );
    assert_eq!(
        action
            .contract()
            .expect("provisioning action contract")
            .event_type
            .as_str(),
        "provisioning.action.planned"
    );
}
