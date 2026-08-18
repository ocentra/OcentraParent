use chrono::{DateTime, FixedOffset, Utc};

use crate::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};
use crate::household_authority::{
    AuditRequirementState, ElevatedConfirmationState, HouseholdActorTargetAuthorityInput,
    HouseholdAuthorityAction, HouseholdAuthorityInput, HouseholdAuthorizationFailureReason,
    ParentControllerLeaseState, ParentStepUpAssertionSnapshot, ParentStepUpValidationFailureReason,
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
            (input.action == HouseholdAuthorityAction::SealParentDeviceTrust
                && !bootstrap_sealing_state_is_allowed(input))
                || (input.action != HouseholdAuthorityAction::SealParentDeviceTrust
                    && input.device_trust_state != DeviceTrustState::Trusted),
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
            requires_parent_controller_device_scope(input.action)
                && input.device_ownership_scope != DeviceOwnershipScope::ParentControllerDevice,
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

pub(crate) fn household_actor_target_authority_failure_reason(
    input: &HouseholdActorTargetAuthorityInput,
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
            (input.action == HouseholdAuthorityAction::SealParentDeviceTrust
                && !matches!(
                    (input.device_trust_state, input.child_profile_binding_state),
                    (
                        DeviceTrustState::Pending | DeviceTrustState::ResetRequired,
                        _
                    ) | (DeviceTrustState::Trusted, ChildProfileBindingState::Missing)
                ))
                || (input.action != HouseholdAuthorityAction::SealParentDeviceTrust
                    && input.device_trust_state != DeviceTrustState::Trusted),
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
            target_actor_device_scope_is_invalid(input),
            HouseholdAuthorizationFailureReason::WrongDeviceScope,
        ),
        (
            requires_target_child_scope(input.action)
                && input.target_device_ownership_scope
                    != Some(DeviceOwnershipScope::ChildProfileDevice),
            HouseholdAuthorizationFailureReason::WrongDeviceScope,
        ),
        (
            requires_capability_grant(input.action),
            HouseholdAuthorizationFailureReason::MissingCapabilityGrant,
        ),
        (
            requires_controller_lease(input.action),
            HouseholdAuthorizationFailureReason::ControllerLeaseRequired,
        ),
        (
            !role_can_authorize(input.actor_role, input.action),
            HouseholdAuthorizationFailureReason::RoleNotAuthorized,
        ),
        (
            crate::household_authority::requires_parent_step_up(input.action),
            HouseholdAuthorizationFailureReason::MissingParentStepUp,
        ),
    ]
    .into_iter()
    .find_map(|(failed, reason)| failed.then_some(reason))
}

fn target_actor_device_scope_is_invalid(input: &HouseholdActorTargetAuthorityInput) -> bool {
    input.actor_device_ownership_scope != DeviceOwnershipScope::ParentControllerDevice
        && !matches!(
            (
                input.actor_device_ownership_scope,
                input.actor_role,
                input.action
            ),
            (
                DeviceOwnershipScope::ParentObserverDevice,
                HouseholdRole::Observer,
                HouseholdAuthorityAction::ViewChildStatus
                    | HouseholdAuthorityAction::StartRemoteView
            )
        )
}

pub(crate) fn parent_step_up_validation_failure_reason(
    input: &ParentStepUpValidationInput,
    assertion: &ParentStepUpAssertionSnapshot,
) -> Option<ParentStepUpValidationFailureReason> {
    let (Some(assertion_expires_at), Some(observed_at)) = (
        parse_rfc3339_utc(&assertion.expires_at),
        parse_rfc3339_utc(&input.observed_at),
    ) else {
        return Some(ParentStepUpValidationFailureReason::Expired);
    };
    [
        (
            assertion_expires_at <= observed_at,
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

fn parse_rfc3339_utc(value: &str) -> Option<DateTime<Utc>> {
    DateTime::<FixedOffset>::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
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
        HouseholdAuthorityAction::SealParentDeviceTrust
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
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
            HouseholdRole::ParentOwner,
            HouseholdAuthorityAction::SealParentDeviceTrust
        ) | (
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian,
            HouseholdAuthorityAction::PairChildDevice
                | HouseholdAuthorityAction::RegisterLanSignerAnchor
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
        HouseholdAuthorityAction::SealParentDeviceTrust
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
            | HouseholdAuthorityAction::ChangePolicy
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
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
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
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::ViewChildStatus
            | HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::StartRemoteView
            | HouseholdAuthorityAction::StartRemoteControl
    )
}

fn requires_parent_controller_device_scope(action: HouseholdAuthorityAction) -> bool {
    matches!(action, HouseholdAuthorityAction::SealParentDeviceTrust)
}

fn requires_target_child_scope(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::PairChildDevice
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
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

fn bootstrap_sealing_state_is_allowed(input: &HouseholdAuthorityInput) -> bool {
    input.action == HouseholdAuthorityAction::SealParentDeviceTrust
        && matches!(
            (input.device_trust_state, input.child_profile_binding_state),
            (DeviceTrustState::Pending | DeviceTrustState::ResetRequired, _)
                // A trusted parent controller with no child binding is the established
                // controller path used when sealing local parent-device custody.
                | (
                    DeviceTrustState::Trusted,
                    ChildProfileBindingState::Missing
                )
        )
}

fn matches_target_child_profile(asserted: Option<&str>, expected: Option<&str>) -> bool {
    asserted == expected
}
