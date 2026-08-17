use ocentra_family_identity_core::{
    DeviceOwnershipScope, DeviceTrustState, HouseholdMembershipState,
};
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_provisioning_core::{
    evaluate_provisioning_readiness, plan_provisioning_actions, AccountReadinessState,
    ChildAppReadinessState, ChildInstallState, ChildServiceState, DataCustodySyncState,
    NetworkReachabilityState, PairingLifecycleState, ParentAppReadinessState,
    ParentDeviceRegistrationState, PermissionReadinessState, PolicyBaselineState,
    ProvisioningActionPlan, ProvisioningReadinessDecision, ProvisioningReadinessInput,
    RecoveryState,
};
use serde::Serialize;

#[derive(Clone, Debug)]
pub(crate) struct SetupFirstRunRuntimeSnapshot {
    pub(crate) input: ProvisioningReadinessInput,
    pub(crate) decision: ProvisioningReadinessDecision,
    pub(crate) action_plan: ProvisioningActionPlan,
    pub(crate) lan_source_state: &'static str,
}

pub(crate) fn load_setup_first_run_runtime_snapshot(
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> SetupFirstRunRuntimeSnapshot {
    let input = ProvisioningReadinessInput {
        // Account and household authority are not supplied by the current
        // parent route. Pending/recovery-required is the conservative state;
        // it must never be interpreted as an authenticated household.
        membership_state: HouseholdMembershipState::Pending,
        account_readiness_state: AccountReadinessState::RecoveryRequired,
        parent_app_readiness_state: ParentAppReadinessState::Ready,
        parent_device_registration_state: ParentDeviceRegistrationState::Missing,
        child_install_state: ChildInstallState::NotInstalled,
        child_service_state: ChildServiceState::NotStarted,
        child_app_readiness_state: ChildAppReadinessState::NotInstalled,
        child_device_ownership_scope: child_device_ownership_scope(lan_read_model),
        device_trust_state: device_trust_state(lan_read_model),
        permission_readiness_state: PermissionReadinessState::Missing,
        pairing_lifecycle_state: pairing_lifecycle_state(lan_read_model),
        policy_baseline_state: PolicyBaselineState::Missing,
        data_custody_sync_state: DataCustodySyncState::Blocked,
        network_reachability_state: network_reachability_state(lan_read_model),
        recovery_state: RecoveryState::Normal,
    };
    let decision = evaluate_provisioning_readiness(input);
    let action_plan = plan_provisioning_actions(input);

    SetupFirstRunRuntimeSnapshot {
        input,
        decision,
        action_plan,
        lan_source_state: if lan_read_model.is_some() {
            "live-lan-read-model"
        } else {
            "unavailable"
        },
    }
}

pub(crate) fn serialized_label<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value)
        .unwrap_or_else(|_| "unavailable".to_string())
        .trim_matches('"')
        .to_string()
}

fn child_device_ownership_scope(
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> DeviceOwnershipScope {
    lan_read_model
        .and_then(|read_model| {
            read_model
                .selected_device_readiness
                .selected_child_device_id
                .as_ref()
        })
        .map(|_| DeviceOwnershipScope::ChildProfileDevice)
        .unwrap_or(DeviceOwnershipScope::OtherDevice)
}

fn device_trust_state(lan_read_model: Option<&LanBrowserAddDeviceReadModel>) -> DeviceTrustState {
    lan_read_model
        .map(|read_model| read_model.selected_device_readiness.trust_state)
        .and_then(|trust_state| {
            [
                (LanPairingTrustState::Paired, DeviceTrustState::Trusted),
                (LanPairingTrustState::Revoked, DeviceTrustState::Revoked),
                (
                    LanPairingTrustState::Expired,
                    DeviceTrustState::ResetRequired,
                ),
            ]
            .into_iter()
            .find_map(|(candidate, mapped)| (trust_state == candidate).then_some(mapped))
        })
        .unwrap_or(DeviceTrustState::Pending)
}

fn pairing_lifecycle_state(
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> PairingLifecycleState {
    lan_read_model
        .map(|read_model| read_model.selected_device_readiness.trust_state)
        .and_then(|trust_state| {
            [
                (LanPairingTrustState::Paired, PairingLifecycleState::Trusted),
                (
                    LanPairingTrustState::Revoked,
                    PairingLifecycleState::Revoked,
                ),
                (
                    LanPairingTrustState::Expired,
                    PairingLifecycleState::Expired,
                ),
                (
                    LanPairingTrustState::Pairing,
                    PairingLifecycleState::Displayed,
                ),
                (
                    LanPairingTrustState::Unpaired,
                    PairingLifecycleState::Generated,
                ),
            ]
            .into_iter()
            .find_map(|(candidate, mapped)| (trust_state == candidate).then_some(mapped))
        })
        .unwrap_or(PairingLifecycleState::Untrusted)
}

fn network_reachability_state(
    lan_read_model: Option<&LanBrowserAddDeviceReadModel>,
) -> NetworkReachabilityState {
    lan_read_model
        .map(|read_model| read_model.selected_device_readiness.reachability)
        .map(|reachability| match reachability {
            LanPairingDeviceReachability::Online => NetworkReachabilityState::Reachable,
            LanPairingDeviceReachability::Offline | LanPairingDeviceReachability::Stale => {
                NetworkReachabilityState::OfflineChild
            }
        })
        .unwrap_or(NetworkReachabilityState::LanUnavailable)
}
