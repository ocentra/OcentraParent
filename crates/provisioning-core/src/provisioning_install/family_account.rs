use super::family_context::{first_some, mapped_projection};
use super::*;

pub(super) fn provisioning_child_app_readiness_state(
    child_install_state: ChildInstallState,
    child_service_state: ChildServiceState,
) -> ChildAppReadinessState {
    mapped_projection(
        child_install_state,
        [
            (
                ChildInstallState::NotInstalled,
                ChildAppReadinessState::NotInstalled,
            ),
            (
                ChildInstallState::ReinstallRequired,
                ChildAppReadinessState::ReinstallRequired,
            ),
        ],
    )
    .or_else(|| {
        mapped_projection(
            child_service_state,
            [
                (
                    ChildServiceState::NotStarted,
                    ChildAppReadinessState::Installed,
                ),
                (
                    ChildServiceState::ServiceStarted,
                    ChildAppReadinessState::Ready,
                ),
                (ChildServiceState::Offline, ChildAppReadinessState::Offline),
                (ChildServiceState::Revoked, ChildAppReadinessState::Revoked),
            ],
        )
    })
    .unwrap_or(ChildAppReadinessState::Installed)
}

pub(super) fn provisioning_account_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    authority_decision: HouseholdAuthorityDecision,
    recovery_decision: Option<RecoveryDecision>,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> AccountReadinessState {
    first_some([
        (!input.account_matches_invite_target).then_some(AccountReadinessState::WrongAccount),
        household_or_session_recovery_required(
            authority_decision.failure_reason,
            session_failure_reason,
        ),
        recovery_operation_requires_account_recovery(input.recovery_operation, recovery_decision),
    ])
    .unwrap_or(AccountReadinessState::Ready)
}

fn household_or_session_recovery_required(
    household_failure_reason: Option<HouseholdAuthorizationFailureReason>,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> Option<AccountReadinessState> {
    matches!(
        household_failure_reason,
        Some(HouseholdAuthorizationFailureReason::AccountNotActive)
    )
    .then_some(AccountReadinessState::RecoveryRequired)
    .or_else(|| {
        matches!(
            session_failure_reason,
            Some(
                SessionTokenFailureReason::SessionLoggedOut
                    | SessionTokenFailureReason::SessionRevoked
                    | SessionTokenFailureReason::SessionGloballyRevoked
                    | SessionTokenFailureReason::SessionNotFresh
            )
        )
        .then_some(AccountReadinessState::RecoveryRequired)
    })
}

fn recovery_operation_requires_account_recovery(
    recovery_operation: Option<FamilyRecoveryOperation>,
    recovery_decision: Option<RecoveryDecision>,
) -> Option<AccountReadinessState> {
    let recovery_operation_requires_recovery = matches!(
        recovery_operation,
        Some(operation) if operation.state != FamilyRecoveryState::Completed
    );
    let recovery_blocks_custody_first = matches!(
        recovery_decision,
        Some(decision)
            if decision.data_custody_handoff_state != RecoveryDataCustodyHandoffState::None
    ) || matches!(
        recovery_operation,
        Some(operation) if operation.support_channel == RecoverySupportChannel::SupportAssisted
    );

    (recovery_operation_requires_recovery && !recovery_blocks_custody_first)
        .then_some(AccountReadinessState::RecoveryRequired)
}
