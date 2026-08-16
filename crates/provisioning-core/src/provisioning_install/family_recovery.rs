use super::family_account::provisioning_child_app_readiness_state;
use super::family_context::{first_some, mapped_projection};
use super::*;

pub(super) fn provisioning_custody_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    recovery_decision: Option<RecoveryDecision>,
) -> DataCustodySyncState {
    first_some([
        matches!(
            recovery_decision,
            Some(decision)
                if decision.data_custody_handoff_state != RecoveryDataCustodyHandoffState::None
        )
        .then_some(DataCustodySyncState::Blocked),
        matches!(
            input.recovery_operation,
            Some(operation)
                if operation.support_channel == RecoverySupportChannel::SupportAssisted
                    && operation.state != FamilyRecoveryState::Completed
        )
        .then_some(DataCustodySyncState::Blocked),
    ])
    .unwrap_or(input.data_custody_sync_state)
}

pub(super) fn provisioning_recovery_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> RecoveryState {
    let child_app_readiness_state = provisioning_child_app_readiness_state(
        input.child_install_state,
        input.child_service_state,
    );

    input
        .recovery_operation
        .map(recovery_state_for_operation)
        .or_else(|| {
            first_some([
                stale_code_recovery_state(
                    session_failure_reason,
                    input.setup_invite_input.invite_state,
                ),
                (!input.account_matches_invite_target).then_some(RecoveryState::WrongAccount),
                child_app_recovery_state(child_app_readiness_state),
                (input.permission_readiness_state != PermissionReadinessState::Granted)
                    .then_some(RecoveryState::PermissionLoss),
                (input.network_reachability_state != NetworkReachabilityState::Reachable)
                    .then_some(RecoveryState::OfflineDevice),
            ])
        })
        .unwrap_or(RecoveryState::Normal)
}

fn recovery_state_for_operation(recovery_operation: FamilyRecoveryOperation) -> RecoveryState {
    first_some([
        (recovery_operation.state == FamilyRecoveryState::Completed)
            .then_some(RecoveryState::Recovered),
        (recovery_operation.kind == FamilyRecoveryKind::ForgotLogin)
            .then_some(RecoveryState::WrongAccount),
        (recovery_operation.kind == FamilyRecoveryKind::LostParentDevice)
            .then_some(RecoveryState::LostParentDevice),
        (recovery_operation.kind == FamilyRecoveryKind::CompromisedAccount)
            .then_some(RecoveryState::PermissionLoss),
        (recovery_operation.kind == FamilyRecoveryKind::ChildReinstall)
            .then_some(RecoveryState::ChildReinstall),
        (recovery_operation.kind == FamilyRecoveryKind::HouseholdTransfer)
            .then_some(RecoveryState::RevokedChild),
    ])
    .unwrap_or(RecoveryState::Normal)
}

fn stale_code_recovery_state(
    session_failure_reason: Option<SessionTokenFailureReason>,
    invite_state: SetupInviteState,
) -> Option<RecoveryState> {
    (matches!(
        session_failure_reason,
        Some(SessionTokenFailureReason::TokenExpired | SessionTokenFailureReason::TokenNotYetValid)
    ) || invite_state == SetupInviteState::Expired)
        .then_some(RecoveryState::StaleCode)
}

fn child_app_recovery_state(
    child_app_readiness_state: ChildAppReadinessState,
) -> Option<RecoveryState> {
    mapped_projection(
        child_app_readiness_state,
        [
            (ChildAppReadinessState::Revoked, RecoveryState::RevokedChild),
            (
                ChildAppReadinessState::ReinstallRequired,
                RecoveryState::ChildReinstall,
            ),
            (
                ChildAppReadinessState::Offline,
                RecoveryState::OfflineDevice,
            ),
        ],
    )
}
