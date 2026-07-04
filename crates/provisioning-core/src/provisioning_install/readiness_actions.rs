use super::*;

pub(super) fn provisioning_overall_state(
    blocker_reason: Option<ProvisioningBlockerReason>,
) -> ProvisioningOverallState {
    blocker_reason
        .and_then(degraded_overall_state)
        .unwrap_or_else(|| {
            blocker_reason
                .map(|_| ProvisioningOverallState::Blocked)
                .unwrap_or(ProvisioningOverallState::Ready)
        })
}

pub(super) fn provisioning_recovery_action(
    blocker_reason: Option<ProvisioningBlockerReason>,
) -> ProvisioningRecoveryAction {
    blocker_reason
        .and_then(recovery_action_for_blocker_reason)
        .unwrap_or(ProvisioningRecoveryAction::Continue)
}

fn first_some<T, const N: usize>(values: [Option<T>; N]) -> Option<T> {
    values.into_iter().flatten().next()
}

fn degraded_overall_state(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningOverallState> {
    [
        ProvisioningBlockerReason::DataCustodySyncPending,
        ProvisioningBlockerReason::ChildAppOffline,
        ProvisioningBlockerReason::NetworkOfflineChild,
        ProvisioningBlockerReason::OfflineDeviceRecovery,
    ]
    .contains(&blocker_reason)
    .then_some(ProvisioningOverallState::Degraded)
}

fn recovery_action_for_blocker_reason(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        membership_recovery_action(blocker_reason),
        account_recovery_action(blocker_reason),
        parent_app_recovery_action(blocker_reason),
        trust_and_permission_recovery_action(blocker_reason),
        pairing_recovery_action(blocker_reason),
        policy_and_custody_recovery_action(blocker_reason),
        child_runtime_recovery_action(blocker_reason),
        network_recovery_action(blocker_reason),
    ])
}

fn membership_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    blocker_action(
        blocker_reason,
        [ProvisioningBlockerReason::HouseholdMemberRequired],
        ProvisioningRecoveryAction::CompleteHouseholdMembership,
    )
}

fn account_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::WrongAccount,
                ProvisioningBlockerReason::WrongAccountRecovery,
                ProvisioningBlockerReason::PairingParentRoleRequired,
            ],
            ProvisioningRecoveryAction::SwitchToCorrectAccount,
        ),
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::AccountRecoveryRequired,
                ProvisioningBlockerReason::LostParentDeviceRecovery,
            ],
            ProvisioningRecoveryAction::RestoreParentSession,
        ),
    ])
}

fn parent_app_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::ParentAppMissing,
                ProvisioningBlockerReason::ParentAppOffline,
                ProvisioningBlockerReason::ParentAppStale,
            ],
            ProvisioningRecoveryAction::RepairParentApp,
        ),
        blocker_action(
            blocker_reason,
            [ProvisioningBlockerReason::ParentDeviceRegistrationRequired],
            ProvisioningRecoveryAction::ReRegisterParentDevice,
        ),
    ])
}

fn trust_and_permission_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::ChildDeviceTrustRequired,
                ProvisioningBlockerReason::PairingTrustRequired,
                ProvisioningBlockerReason::PairingPendingAcceptance,
            ],
            ProvisioningRecoveryAction::TrustChildDevice,
        ),
        blocker_action(
            blocker_reason,
            [ProvisioningBlockerReason::PermissionMissing],
            ProvisioningRecoveryAction::RequestMissingPermissions,
        ),
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::PermissionRevoked,
                ProvisioningBlockerReason::PermissionLossRecovery,
            ],
            ProvisioningRecoveryAction::RegrantPermissions,
        ),
    ])
}

fn pairing_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::ChildDeviceScopeRequired,
                ProvisioningBlockerReason::PairingRevoked,
                ProvisioningBlockerReason::PairingWrongHousehold,
                ProvisioningBlockerReason::PairingWrongDevice,
                ProvisioningBlockerReason::PairingAnonymousDevice,
                ProvisioningBlockerReason::RevokedChildRecovery,
                ProvisioningBlockerReason::ChildAppRevoked,
            ],
            ProvisioningRecoveryAction::RePairChildDevice,
        ),
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::PairingPendingDisplay,
                ProvisioningBlockerReason::PairingExpired,
                ProvisioningBlockerReason::PairingReplayRejected,
                ProvisioningBlockerReason::PairingStaleSignedHello,
                ProvisioningBlockerReason::StaleCodeRecovery,
            ],
            ProvisioningRecoveryAction::ReissuePairingCode,
        ),
    ])
}

fn policy_and_custody_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::PolicyBaselineMissing,
                ProvisioningBlockerReason::PolicyBaselineStale,
            ],
            ProvisioningRecoveryAction::ApplyPolicyBaseline,
        ),
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::DataCustodySyncPending,
                ProvisioningBlockerReason::DataCustodySyncBlocked,
            ],
            ProvisioningRecoveryAction::RepairCustodySync,
        ),
    ])
}

fn child_runtime_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [ProvisioningBlockerReason::ChildInstallNotInstalled],
            ProvisioningRecoveryAction::InstallChildApp,
        ),
        blocker_action(
            blocker_reason,
            [ProvisioningBlockerReason::ChildServiceNotStarted],
            ProvisioningRecoveryAction::StartChildService,
        ),
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::ChildAppReinstallRequired,
                ProvisioningBlockerReason::ChildReinstallRecovery,
            ],
            ProvisioningRecoveryAction::ReinstallChildApp,
        ),
    ])
}

fn network_recovery_action(
    blocker_reason: ProvisioningBlockerReason,
) -> Option<ProvisioningRecoveryAction> {
    first_some([
        blocker_action(
            blocker_reason,
            [
                ProvisioningBlockerReason::ChildAppOffline,
                ProvisioningBlockerReason::NetworkOfflineChild,
                ProvisioningBlockerReason::NetworkLanUnavailable,
                ProvisioningBlockerReason::OfflineDeviceRecovery,
            ],
            ProvisioningRecoveryAction::WaitForChildConnectivity,
        ),
        blocker_action(
            blocker_reason,
            [ProvisioningBlockerReason::NetworkDirectEntryRequired],
            ProvisioningRecoveryAction::EnterDirectChildAddress,
        ),
    ])
}

fn blocker_action<const N: usize>(
    blocker_reason: ProvisioningBlockerReason,
    reasons: [ProvisioningBlockerReason; N],
    action: ProvisioningRecoveryAction,
) -> Option<ProvisioningRecoveryAction> {
    reasons.contains(&blocker_reason).then_some(action)
}
