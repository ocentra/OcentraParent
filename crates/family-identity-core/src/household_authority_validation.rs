use crate::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use crate::household_authority::{
    AuditRequirementState, ElevatedConfirmationState, HouseholdAuthorityAction,
    HouseholdAuthorityInput, HouseholdAuthorizationFailureReason, ParentControllerLeaseState,
    ParentStepUpAssertionSnapshot, ParentStepUpValidationFailureReason,
    ParentStepUpValidationInput,
};

pub(crate) fn household_authority_failure_reason(
    input: &HouseholdAuthorityInput,
) -> Option<HouseholdAuthorizationFailureReason> {
    [
        (
            !input.same_family,
            HouseholdAuthorizationFailureReason::ExternalHousehold,
        ),
        (
            input.membership_state != HouseholdMembershipState::Active,
            HouseholdAuthorizationFailureReason::MembershipNotActive,
        ),
        (
            input.actor_account_state != ActorAccountState::Active,
            HouseholdAuthorizationFailureReason::AccountNotActive,
        ),
        (
            input.device_trust_state != DeviceTrustState::Trusted,
            HouseholdAuthorizationFailureReason::DeviceNotTrusted,
        ),
        (
            requires_fresh_session(input.action)
                && input.session_freshness_state != SessionFreshnessState::Fresh,
            HouseholdAuthorizationFailureReason::SessionNotFresh,
        ),
        (
            requires_bound_child_scope(input.action)
                && input.child_profile_binding_state != ChildProfileBindingState::Bound,
            HouseholdAuthorizationFailureReason::ChildProfileNotBound,
        ),
        (
            requires_child_profile_device_scope(input.action)
                && input.device_ownership_scope != DeviceOwnershipScope::ChildProfileDevice,
            HouseholdAuthorizationFailureReason::WrongDeviceScope,
        ),
        (
            requires_capability_grant(input.action) && !input.capability_granted,
            HouseholdAuthorizationFailureReason::MissingCapabilityGrant,
        ),
        (
            !role_can_authorize(input.actor_role, input.action),
            HouseholdAuthorizationFailureReason::RoleNotAuthorized,
        ),
    ]
    .into_iter()
    .find_map(|(failed, reason)| failed.then_some(reason))
    .or_else(|| controller_lease_failure_reason(input.action, input.controller_lease_state))
}

pub(crate) fn parent_step_up_validation_failure_reason(
    input: &ParentStepUpValidationInput,
    assertion: &ParentStepUpAssertionSnapshot,
) -> Option<ParentStepUpValidationFailureReason> {
    [
        (
            assertion.expires_at < input.observed_at,
            ParentStepUpValidationFailureReason::Expired,
        ),
        (
            assertion.family_id != input.family_id,
            ParentStepUpValidationFailureReason::WrongHousehold,
        ),
        (
            assertion.parent_account_id != input.parent_account_id,
            ParentStepUpValidationFailureReason::WrongAccount,
        ),
        (
            assertion.action != input.action,
            ParentStepUpValidationFailureReason::WrongAction,
        ),
        (
            assertion.action_device_id != input.action_device_id
                || assertion.action_device_child_profile_id != input.action_device_child_profile_id,
            ParentStepUpValidationFailureReason::WrongDevice,
        ),
        (
            !matches_target_child_profile(
                assertion.target_child_profile_id.as_deref(),
                input.target_child_profile_id.as_deref(),
            ),
            ParentStepUpValidationFailureReason::WrongTarget,
        ),
        (
            assertion.target_child_device_id != input.target_child_device_id,
            ParentStepUpValidationFailureReason::WrongTarget,
        ),
        (
            input
                .expected_nonce
                .as_ref()
                .is_some_and(|nonce| assertion.nonce != *nonce),
            ParentStepUpValidationFailureReason::ReplayRejected,
        ),
    ]
    .into_iter()
    .find_map(|(failed, reason)| failed.then_some(reason))
}

pub(crate) fn audit_requirement_state(action: HouseholdAuthorityAction) -> AuditRequirementState {
    if matches!(action, HouseholdAuthorityAction::ViewChildStatus) {
        AuditRequirementState::NotRequired
    } else {
        AuditRequirementState::Required
    }
}

pub(crate) fn elevated_confirmation_state(
    action: HouseholdAuthorityAction,
) -> ElevatedConfirmationState {
    if matches!(
        action,
        HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::StartRemoteControl
            | HouseholdAuthorityAction::ExportDeleteData
            | HouseholdAuthorityAction::ManageBilling
    ) {
        ElevatedConfirmationState::Required
    } else {
        ElevatedConfirmationState::NotRequired
    }
}

fn role_can_authorize(role: HouseholdRole, action: HouseholdAuthorityAction) -> bool {
    matches!(
        (role, action),
        (
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian,
            HouseholdAuthorityAction::PairChildDevice
                | HouseholdAuthorityAction::RevokeChildDevice
                | HouseholdAuthorityAction::ChangePolicy
        ) | (
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian | HouseholdRole::Observer,
            HouseholdAuthorityAction::ViewChildStatus | HouseholdAuthorityAction::StartRemoteView
        ) | (
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian,
            HouseholdAuthorityAction::StartRemoteControl
        ) | (
            HouseholdRole::ParentOwner,
            HouseholdAuthorityAction::ExportDeleteData | HouseholdAuthorityAction::ManageBilling
        )
    )
}

fn requires_capability_grant(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::StartRemoteView | HouseholdAuthorityAction::StartRemoteControl
    )
}

fn controller_lease_failure_reason(
    action: HouseholdAuthorityAction,
    controller_lease_state: Option<ParentControllerLeaseState>,
) -> Option<HouseholdAuthorizationFailureReason> {
    if !requires_controller_lease(action) {
        return None;
    }

    match controller_lease_state {
        Some(ParentControllerLeaseState::Active) => None,
        Some(ParentControllerLeaseState::Expired) => {
            Some(HouseholdAuthorizationFailureReason::ControllerLeaseExpired)
        }
        Some(ParentControllerLeaseState::Revoked) => {
            Some(HouseholdAuthorizationFailureReason::ControllerLeaseRevoked)
        }
        None => Some(HouseholdAuthorizationFailureReason::ControllerLeaseRequired),
    }
}

fn requires_fresh_session(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::StartRemoteView
            | HouseholdAuthorityAction::StartRemoteControl
            | HouseholdAuthorityAction::ExportDeleteData
            | HouseholdAuthorityAction::ManageBilling
    )
}

fn requires_bound_child_scope(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::ViewChildStatus
            | HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::StartRemoteView
            | HouseholdAuthorityAction::StartRemoteControl
    )
}

fn requires_child_profile_device_scope(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::ViewChildStatus
            | HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::StartRemoteView
            | HouseholdAuthorityAction::StartRemoteControl
    )
}

fn requires_controller_lease(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::StartRemoteView | HouseholdAuthorityAction::StartRemoteControl
    )
}

fn matches_target_child_profile(asserted: Option<&str>, expected: Option<&str>) -> bool {
    asserted == expected
}
