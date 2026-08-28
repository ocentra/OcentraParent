use super::family_context::first_some;
use super::*;

pub(super) fn provisioning_pairing_state_from_family_context(
    input: ProvisioningFamilyContextInput,
    authority_decision: HouseholdAuthorityDecision,
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> PairingLifecycleState {
    let invite_failure_reason = provisioning_setup_invite_failure_reason(input.setup_invite_input);
    let awaiting_parent_trust_confirmation = input.setup_invite_input.invite_state
        == SetupInviteState::Accepted
        && input.household_authority_input.device_trust_state == DeviceTrustState::Pending;

    first_some([
        replay_rejected_pairing_state(session_failure_reason, invite_failure_reason),
        stale_signed_hello_pairing_state(session_failure_reason),
        (input.setup_invite_input.invite_state == SetupInviteState::Expired)
            .then_some(PairingLifecycleState::Expired),
        revoked_pairing_state(
            input.setup_invite_input.invite_state,
            authority_decision.failure_reason,
            awaiting_parent_trust_confirmation,
        ),
        (!input.account_matches_invite_target).then_some(PairingLifecycleState::Untrusted),
        wrong_household_pairing_state(invite_failure_reason, authority_decision.failure_reason),
        setup_invite_guard_pairing_state(invite_failure_reason),
        household_pairing_failure_state(
            authority_decision.failure_reason,
            HouseholdAuthorizationFailureReason::WrongDeviceScope,
            PairingLifecycleState::WrongDevice,
        ),
        household_pairing_failure_state(
            authority_decision.failure_reason,
            HouseholdAuthorizationFailureReason::ChildProfileNotBound,
            PairingLifecycleState::AnonymousDevice,
        ),
        household_pairing_failure_state(
            authority_decision.failure_reason,
            HouseholdAuthorizationFailureReason::RoleNotAuthorized,
            PairingLifecycleState::ParentRoleRequired,
        ),
        rejected_authority_pairing_state(
            authority_decision.authorization_state,
            awaiting_parent_trust_confirmation,
        ),
        recovery_completed_pairing_state(input.recovery_operation),
        recovery_in_progress_pairing_state(input.recovery_operation),
        (input.setup_invite_input.invite_state == SetupInviteState::Accepted)
            .then_some(PairingLifecycleState::Trusted),
    ])
    .unwrap_or(PairingLifecycleState::Displayed)
}

fn replay_rejected_pairing_state(
    session_failure_reason: Option<SessionTokenFailureReason>,
    invite_failure_reason: Option<SetupInviteFailureReason>,
) -> Option<PairingLifecycleState> {
    (matches!(
        session_failure_reason,
        Some(SessionTokenFailureReason::TokenReplayRejected)
    ) || matches!(
        invite_failure_reason,
        Some(SetupInviteFailureReason::InviteReplayRejected)
    ))
    .then_some(PairingLifecycleState::Replayed)
}

fn stale_signed_hello_pairing_state(
    session_failure_reason: Option<SessionTokenFailureReason>,
) -> Option<PairingLifecycleState> {
    matches!(
        session_failure_reason,
        Some(SessionTokenFailureReason::TokenExpired | SessionTokenFailureReason::TokenNotYetValid)
    )
    .then_some(PairingLifecycleState::StaleSignedHello)
}

fn revoked_pairing_state(
    invite_state: SetupInviteState,
    household_failure_reason: Option<HouseholdAuthorizationFailureReason>,
    awaiting_parent_trust_confirmation: bool,
) -> Option<PairingLifecycleState> {
    (invite_state == SetupInviteState::Revoked
        || (matches!(
            household_failure_reason,
            Some(HouseholdAuthorizationFailureReason::DeviceNotTrusted)
        ) && !awaiting_parent_trust_confirmation))
        .then_some(PairingLifecycleState::Revoked)
}

fn wrong_household_pairing_state(
    invite_failure_reason: Option<SetupInviteFailureReason>,
    household_failure_reason: Option<HouseholdAuthorizationFailureReason>,
) -> Option<PairingLifecycleState> {
    (matches!(
        invite_failure_reason,
        Some(SetupInviteFailureReason::WrongHousehold)
    ) || matches!(
        household_failure_reason,
        Some(HouseholdAuthorizationFailureReason::ExternalHousehold)
    ))
    .then_some(PairingLifecycleState::WrongHousehold)
}

fn setup_invite_guard_pairing_state(
    invite_failure_reason: Option<SetupInviteFailureReason>,
) -> Option<PairingLifecycleState> {
    first_some([
        matches!(
            invite_failure_reason,
            Some(
                SetupInviteFailureReason::InviteNotActive
                    | SetupInviteFailureReason::InviteNotSingleUse
            )
        )
        .then_some(PairingLifecycleState::Revoked),
        matches!(
            invite_failure_reason,
            Some(SetupInviteFailureReason::WrongTargetRole)
        )
        .then_some(PairingLifecycleState::WrongDevice),
        matches!(
            invite_failure_reason,
            Some(SetupInviteFailureReason::InviterNotAuthorized)
        )
        .then_some(PairingLifecycleState::ParentRoleRequired),
    ])
}

fn household_pairing_failure_state(
    household_failure_reason: Option<HouseholdAuthorizationFailureReason>,
    candidate: HouseholdAuthorizationFailureReason,
    pairing_state: PairingLifecycleState,
) -> Option<PairingLifecycleState> {
    matches!(household_failure_reason, Some(reason) if reason == candidate).then_some(pairing_state)
}

fn rejected_authority_pairing_state(
    authorization_state: HouseholdAuthorizationState,
    awaiting_parent_trust_confirmation: bool,
) -> Option<PairingLifecycleState> {
    first_some([
        (authorization_state == HouseholdAuthorizationState::Rejected
            && awaiting_parent_trust_confirmation)
            .then_some(PairingLifecycleState::Accepted),
        (authorization_state == HouseholdAuthorizationState::Rejected
            && !awaiting_parent_trust_confirmation)
            .then_some(PairingLifecycleState::Untrusted),
    ])
}

fn recovery_completed_pairing_state(
    recovery_operation: Option<FamilyRecoveryOperation>,
) -> Option<PairingLifecycleState> {
    matches!(
        recovery_operation,
        Some(operation) if operation.state == FamilyRecoveryState::Completed
    )
    .then_some(PairingLifecycleState::Recovered)
}

fn recovery_in_progress_pairing_state(
    recovery_operation: Option<FamilyRecoveryOperation>,
) -> Option<PairingLifecycleState> {
    matches!(
        recovery_operation,
        Some(operation)
            if operation.state == FamilyRecoveryState::PendingIdentityProof
                || operation.state == FamilyRecoveryState::OwnerApprovalRequired
                || operation.state == FamilyRecoveryState::Approved
    )
    .then_some(PairingLifecycleState::Trusted)
}

fn provisioning_setup_invite_failure_reason(
    input: SetupInviteInput,
) -> Option<SetupInviteFailureReason> {
    if !matches!(
        input.invite_state,
        SetupInviteState::Pending | SetupInviteState::Accepted
    ) {
        return None;
    }

    // Accepted is a lifecycle state, not a bypass of the invite guards. Re-run
    // the family-owned validation with the active-state marker normalized so
    // replay, abuse/timing, single-use, role, household, and inviter checks
    // still apply after a child bootstrap presents an accepted invite.
    authorize_setup_invite(SetupInviteInput {
        invite_state: SetupInviteState::Pending,
        ..input
    })
    .failure_reason
}
