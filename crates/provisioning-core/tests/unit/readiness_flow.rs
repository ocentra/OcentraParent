use ocentra_eventing::DomainEvent;
use ocentra_family_identity_core::{DeviceOwnershipScope, HouseholdMembership};
use ocentra_provisioning_core::{
    evaluate_provisioning_readiness, plan_provisioning_actions,
    provisioning_action_planned_event, provisioning_readiness_evaluated_event,
    ChildRuntimeReadinessState, PairingTokenState, ParentDeviceRegistrationState,
    ProvisioningActionPlanId, ProvisioningAggregateId, ProvisioningAuditState,
    ProvisioningChildRuntimeStartAction, ProvisioningManualStepState,
    ProvisioningReadinessEvaluationId, ProvisioningReadinessInput, ProvisioningRecoveryAction,
    RecoveryState, RequiredPermissionState,
};

const PROVISIONING_AGGREGATE_ID: &str = "provisioning-family-default";
const PROVISIONING_EVALUATION_ID: &str = "provisioning-readiness-default";
const PROVISIONING_READINESS_EVENT_TYPE: &str = "provisioning.readiness.evaluated";
const PROVISIONING_ACTION_EVENT_TYPE: &str = "provisioning.action.planned";

fn ready_input() -> ProvisioningReadinessInput {
    ProvisioningReadinessInput {
        household_membership: HouseholdMembership::Member,
        parent_device_registration_state: ParentDeviceRegistrationState::Registered,
        child_device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        required_permission_state: RequiredPermissionState::Granted,
        pairing_token_state: PairingTokenState::Valid,
        recovery_state: RecoveryState::Normal,
    }
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
    assert_eq!(readiness.manual_step_state, ProvisioningManualStepState::NotRequired);
    assert_eq!(
        actions.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::Start
    );
    assert_eq!(actions.recovery_action, ProvisioningRecoveryAction::Continue);
    assert_eq!(actions.audit_state, ProvisioningAuditState::Record);
}

#[test]
fn missing_permission_blocks_runtime_and_requests_manual_permission_step() {
    let input = ProvisioningReadinessInput {
        required_permission_state: RequiredPermissionState::Missing,
        ..ready_input()
    };

    let readiness = evaluate_provisioning_readiness(input);
    let actions = plan_provisioning_actions(input);

    assert_eq!(
        readiness.child_runtime_readiness_state,
        ChildRuntimeReadinessState::NotReady
    );
    assert_eq!(readiness.manual_step_state, ProvisioningManualStepState::Required);
    assert_eq!(
        actions.child_runtime_start_action,
        ProvisioningChildRuntimeStartAction::DoNotStart
    );
    assert_eq!(
        actions.recovery_action,
        ProvisioningRecoveryAction::RequestMissingPermissions
    );
}

#[test]
fn readiness_event_projects_typed_action_event() {
    let readiness_event = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse(PROVISIONING_AGGREGATE_ID)
            .expect("provisioning aggregate id"),
        ProvisioningReadinessEvaluationId::parse(PROVISIONING_EVALUATION_ID)
            .expect("provisioning evaluation id"),
        ready_input(),
    );

    let action_event = provisioning_action_planned_event(readiness_event.clone());

    assert_eq!(
        readiness_event
            .contract()
            .expect("provisioning readiness contract")
            .event_type
            .as_str(),
        PROVISIONING_READINESS_EVENT_TYPE
    );
    assert_eq!(
        action_event
            .contract()
            .expect("provisioning action contract")
            .event_type
            .as_str(),
        PROVISIONING_ACTION_EVENT_TYPE
    );
    assert_eq!(action_event.aggregate_id, readiness_event.aggregate_id);
    assert_eq!(action_event.source_evaluation_id, readiness_event.evaluation_id);
    assert!(
        ProvisioningActionPlanId::parse(action_event.action_plan_id.as_str()).is_ok(),
        "action plan id remains branded"
    );
}
