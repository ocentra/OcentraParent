use ocentra_eventing::{envelope::DomainEvent, error::EventingError};
use ocentra_family_identity_core::family_identity::{
    DeviceOwnershipScope, DeviceTrustState, HouseholdMembershipState,
};
use ocentra_provisioning_core::provisioning_install::{
    provisioning_action_planned_event, provisioning_readiness_evaluated_event,
    AccountReadinessState, ChildAppReadinessState, ChildInstallState, ChildServiceState,
    DataCustodySyncState, NetworkReachabilityState, PairingLifecycleState, ParentAppReadinessState,
    ParentDeviceRegistrationState, PermissionReadinessState, PolicyBaselineState,
    ProvisioningActionPlanId, ProvisioningAggregateId, ProvisioningReadinessEvaluationId,
    ProvisioningReadinessInput, RecoveryState,
};

const PROVISIONING_AGGREGATE_ID: &str = "provisioning-family-default";
const PROVISIONING_EVALUATION_ID: &str = "provisioning-readiness-default";
const PROVISIONING_READINESS_EVENT_TYPE: &str = "provisioning.readiness.evaluated";
const PROVISIONING_ACTION_EVENT_TYPE: &str = "provisioning.action.planned";

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

#[test]
fn readiness_event_projects_typed_action_event_contract() -> Result<(), EventingError> {
    let readiness_event = provisioning_readiness_evaluated_event(
        ProvisioningAggregateId::parse(PROVISIONING_AGGREGATE_ID)?,
        ProvisioningReadinessEvaluationId::parse(PROVISIONING_EVALUATION_ID)?,
        ready_input(),
    );

    let action_event = provisioning_action_planned_event(readiness_event.clone());

    assert_eq!(
        readiness_event.contract()?.event_type.as_str(),
        PROVISIONING_READINESS_EVENT_TYPE
    );
    assert_eq!(
        action_event.contract()?.event_type.as_str(),
        PROVISIONING_ACTION_EVENT_TYPE
    );
    assert_eq!(action_event.aggregate_id, readiness_event.aggregate_id);
    assert_eq!(
        action_event.source_evaluation_id,
        readiness_event.evaluation_id
    );
    assert!(
        ProvisioningActionPlanId::parse(action_event.action_plan_id.as_str()).is_ok(),
        "action plan id remains branded"
    );

    Ok(())
}
