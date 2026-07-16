use super::*;

pub(super) fn provisioning_blocker_reason(
    input: ProvisioningReadinessInput,
) -> Option<ProvisioningBlockerReason> {
    first_some([
        membership_blocker_reason(input.membership_state),
        account_blocker_reason(input.account_readiness_state),
        parent_app_blocker_reason(input.parent_app_readiness_state),
        parent_device_registration_blocker_reason(input.parent_device_registration_state),
        pairing_guard_blocker_reason(input.pairing_lifecycle_state),
        child_device_scope_blocker_reason(input.child_device_ownership_scope),
        device_trust_blocker_reason(input.device_trust_state),
        permission_blocker_reason(input.permission_readiness_state),
        pairing_lifecycle_blocker_reason(input.pairing_lifecycle_state),
        policy_baseline_blocker_reason(input.policy_baseline_state),
        data_custody_sync_blocker_reason(input.data_custody_sync_state),
        child_install_blocker_reason(input.child_install_state),
        child_service_blocker_reason(input.child_service_state),
        network_reachability_blocker_reason(input.network_reachability_state),
        recovery_blocker_reason(input.recovery_state),
    ])
}

pub(super) fn first_some<T, const N: usize>(values: [Option<T>; N]) -> Option<T> {
    values.into_iter().flatten().next()
}

fn mapped_blocker_reason<T: Copy + PartialEq, const N: usize>(
    state: T,
    mappings: [(T, ProvisioningBlockerReason); N],
) -> Option<ProvisioningBlockerReason> {
    mappings
        .into_iter()
        .find_map(|(candidate, reason)| (state == candidate).then_some(reason))
}

fn membership_blocker_reason(
    membership_state: HouseholdMembershipState,
) -> Option<ProvisioningBlockerReason> {
    (membership_state != HouseholdMembershipState::Active)
        .then_some(ProvisioningBlockerReason::HouseholdMemberRequired)
}

fn account_blocker_reason(
    account_readiness_state: AccountReadinessState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        account_readiness_state,
        [
            (
                AccountReadinessState::WrongAccount,
                ProvisioningBlockerReason::WrongAccount,
            ),
            (
                AccountReadinessState::RecoveryRequired,
                ProvisioningBlockerReason::AccountRecoveryRequired,
            ),
        ],
    )
}

fn parent_app_blocker_reason(
    parent_app_readiness_state: ParentAppReadinessState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        parent_app_readiness_state,
        [
            (
                ParentAppReadinessState::Missing,
                ProvisioningBlockerReason::ParentAppMissing,
            ),
            (
                ParentAppReadinessState::Offline,
                ProvisioningBlockerReason::ParentAppOffline,
            ),
            (
                ParentAppReadinessState::Stale,
                ProvisioningBlockerReason::ParentAppStale,
            ),
        ],
    )
}

fn parent_device_registration_blocker_reason(
    parent_device_registration_state: ParentDeviceRegistrationState,
) -> Option<ProvisioningBlockerReason> {
    (parent_device_registration_state == ParentDeviceRegistrationState::Missing)
        .then_some(ProvisioningBlockerReason::ParentDeviceRegistrationRequired)
}

fn pairing_guard_blocker_reason(
    pairing_lifecycle_state: PairingLifecycleState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        pairing_lifecycle_state,
        [
            (
                PairingLifecycleState::WrongDevice,
                ProvisioningBlockerReason::PairingWrongDevice,
            ),
            (
                PairingLifecycleState::AnonymousDevice,
                ProvisioningBlockerReason::PairingAnonymousDevice,
            ),
            (
                PairingLifecycleState::ParentRoleRequired,
                ProvisioningBlockerReason::PairingParentRoleRequired,
            ),
            (
                PairingLifecycleState::StaleSignedHello,
                ProvisioningBlockerReason::PairingStaleSignedHello,
            ),
        ],
    )
}

fn child_device_scope_blocker_reason(
    child_device_ownership_scope: DeviceOwnershipScope,
) -> Option<ProvisioningBlockerReason> {
    (child_device_ownership_scope != DeviceOwnershipScope::ChildProfileDevice)
        .then_some(ProvisioningBlockerReason::ChildDeviceScopeRequired)
}

fn device_trust_blocker_reason(
    device_trust_state: DeviceTrustState,
) -> Option<ProvisioningBlockerReason> {
    (device_trust_state != DeviceTrustState::Trusted)
        .then_some(ProvisioningBlockerReason::ChildDeviceTrustRequired)
}

fn permission_blocker_reason(
    permission_readiness_state: PermissionReadinessState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        permission_readiness_state,
        [
            (
                PermissionReadinessState::Missing,
                ProvisioningBlockerReason::PermissionMissing,
            ),
            (
                PermissionReadinessState::Revoked,
                ProvisioningBlockerReason::PermissionRevoked,
            ),
        ],
    )
}

fn pairing_lifecycle_blocker_reason(
    pairing_lifecycle_state: PairingLifecycleState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        pairing_lifecycle_state,
        [
            (
                PairingLifecycleState::Generated,
                ProvisioningBlockerReason::PairingPendingDisplay,
            ),
            (
                PairingLifecycleState::Displayed,
                ProvisioningBlockerReason::PairingPendingDisplay,
            ),
            (
                PairingLifecycleState::Accepted,
                ProvisioningBlockerReason::PairingPendingAcceptance,
            ),
            (
                PairingLifecycleState::Expired,
                ProvisioningBlockerReason::PairingExpired,
            ),
            (
                PairingLifecycleState::Revoked,
                ProvisioningBlockerReason::PairingRevoked,
            ),
            (
                PairingLifecycleState::Replayed,
                ProvisioningBlockerReason::PairingReplayRejected,
            ),
            (
                PairingLifecycleState::WrongHousehold,
                ProvisioningBlockerReason::PairingWrongHousehold,
            ),
            (
                PairingLifecycleState::WrongDevice,
                ProvisioningBlockerReason::PairingWrongDevice,
            ),
            (
                PairingLifecycleState::AnonymousDevice,
                ProvisioningBlockerReason::PairingAnonymousDevice,
            ),
            (
                PairingLifecycleState::ParentRoleRequired,
                ProvisioningBlockerReason::PairingParentRoleRequired,
            ),
            (
                PairingLifecycleState::StaleSignedHello,
                ProvisioningBlockerReason::PairingStaleSignedHello,
            ),
            (
                PairingLifecycleState::Untrusted,
                ProvisioningBlockerReason::PairingTrustRequired,
            ),
        ],
    )
}

fn policy_baseline_blocker_reason(
    policy_baseline_state: PolicyBaselineState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        policy_baseline_state,
        [
            (
                PolicyBaselineState::Missing,
                ProvisioningBlockerReason::PolicyBaselineMissing,
            ),
            (
                PolicyBaselineState::Stale,
                ProvisioningBlockerReason::PolicyBaselineStale,
            ),
        ],
    )
}

fn data_custody_sync_blocker_reason(
    data_custody_sync_state: DataCustodySyncState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        data_custody_sync_state,
        [
            (
                DataCustodySyncState::SyncPending,
                ProvisioningBlockerReason::DataCustodySyncPending,
            ),
            (
                DataCustodySyncState::Blocked,
                ProvisioningBlockerReason::DataCustodySyncBlocked,
            ),
        ],
    )
}

fn child_install_blocker_reason(
    child_install_state: ChildInstallState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        child_install_state,
        [
            (
                ChildInstallState::NotInstalled,
                ProvisioningBlockerReason::ChildInstallNotInstalled,
            ),
            (
                ChildInstallState::ReinstallRequired,
                ProvisioningBlockerReason::ChildAppReinstallRequired,
            ),
        ],
    )
}

fn child_service_blocker_reason(
    child_service_state: ChildServiceState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        child_service_state,
        [
            (
                ChildServiceState::NotStarted,
                ProvisioningBlockerReason::ChildServiceNotStarted,
            ),
            (
                ChildServiceState::Offline,
                ProvisioningBlockerReason::ChildAppOffline,
            ),
            (
                ChildServiceState::Revoked,
                ProvisioningBlockerReason::ChildAppRevoked,
            ),
        ],
    )
}

fn network_reachability_blocker_reason(
    network_reachability_state: NetworkReachabilityState,
) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        network_reachability_state,
        [
            (
                NetworkReachabilityState::OfflineChild,
                ProvisioningBlockerReason::NetworkOfflineChild,
            ),
            (
                NetworkReachabilityState::LanUnavailable,
                ProvisioningBlockerReason::NetworkLanUnavailable,
            ),
            (
                NetworkReachabilityState::DirectEntryRequired,
                ProvisioningBlockerReason::NetworkDirectEntryRequired,
            ),
        ],
    )
}

fn recovery_blocker_reason(recovery_state: RecoveryState) -> Option<ProvisioningBlockerReason> {
    mapped_blocker_reason(
        recovery_state,
        [
            (
                RecoveryState::LostParentDevice,
                ProvisioningBlockerReason::LostParentDeviceRecovery,
            ),
            (
                RecoveryState::ChildReinstall,
                ProvisioningBlockerReason::ChildReinstallRecovery,
            ),
            (
                RecoveryState::RevokedChild,
                ProvisioningBlockerReason::RevokedChildRecovery,
            ),
            (
                RecoveryState::WrongAccount,
                ProvisioningBlockerReason::WrongAccountRecovery,
            ),
            (
                RecoveryState::OfflineDevice,
                ProvisioningBlockerReason::OfflineDeviceRecovery,
            ),
            (
                RecoveryState::PermissionLoss,
                ProvisioningBlockerReason::PermissionLossRecovery,
            ),
            (
                RecoveryState::StaleCode,
                ProvisioningBlockerReason::StaleCodeRecovery,
            ),
        ],
    )
}
