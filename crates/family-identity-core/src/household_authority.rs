use serde::{Deserialize, Serialize};

use crate::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAuthorityAction {
    #[serde(rename = "pair-child-device")]
    PairChildDevice,
    #[serde(rename = "revoke-child-device")]
    RevokeChildDevice,
    #[serde(rename = "view-child-status")]
    ViewChildStatus,
    #[serde(rename = "change-policy")]
    ChangePolicy,
    #[serde(rename = "start-remote-view")]
    StartRemoteView,
    #[serde(rename = "start-remote-control")]
    StartRemoteControl,
    #[serde(rename = "export-delete-data")]
    ExportDeleteData,
    #[serde(rename = "manage-billing")]
    ManageBilling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAuthorizationState {
    #[serde(rename = "authorized")]
    Authorized,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditRequirementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ElevatedConfirmationState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAuthorizationFailureReason {
    #[serde(rename = "external-household")]
    ExternalHousehold,
    #[serde(rename = "membership-not-active")]
    MembershipNotActive,
    #[serde(rename = "account-not-active")]
    AccountNotActive,
    #[serde(rename = "device-not-trusted")]
    DeviceNotTrusted,
    #[serde(rename = "session-not-fresh")]
    SessionNotFresh,
    #[serde(rename = "child-profile-not-bound")]
    ChildProfileNotBound,
    #[serde(rename = "wrong-device-scope")]
    WrongDeviceScope,
    #[serde(rename = "missing-capability-grant")]
    MissingCapabilityGrant,
    #[serde(rename = "controller-lease-required")]
    ControllerLeaseRequired,
    #[serde(rename = "controller-lease-expired")]
    ControllerLeaseExpired,
    #[serde(rename = "controller-lease-revoked")]
    ControllerLeaseRevoked,
    #[serde(rename = "role-not-authorized")]
    RoleNotAuthorized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentControllerLeaseState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityInput {
    pub actor_role: HouseholdRole,
    pub same_family: bool,
    pub actor_account_state: ActorAccountState,
    pub membership_state: HouseholdMembershipState,
    pub child_profile_binding_state: ChildProfileBindingState,
    pub device_ownership_scope: DeviceOwnershipScope,
    pub device_trust_state: DeviceTrustState,
    pub session_freshness_state: SessionFreshnessState,
    pub capability_granted: bool,
    pub controller_lease_state: Option<ParentControllerLeaseState>,
    pub action: HouseholdAuthorityAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityDecision {
    pub authorization_state: HouseholdAuthorizationState,
    pub audit_requirement_state: AuditRequirementState,
    pub elevated_confirmation_state: ElevatedConfirmationState,
    pub failure_reason: Option<HouseholdAuthorizationFailureReason>,
}

pub fn authorize_household_action(input: HouseholdAuthorityInput) -> HouseholdAuthorityDecision {
    if !input.same_family {
        return rejected(
            HouseholdAuthorizationFailureReason::ExternalHousehold,
            input.action,
        );
    }

    if input.membership_state != HouseholdMembershipState::Active {
        return rejected(
            HouseholdAuthorizationFailureReason::MembershipNotActive,
            input.action,
        );
    }

    if input.actor_account_state != ActorAccountState::Active {
        return rejected(
            HouseholdAuthorizationFailureReason::AccountNotActive,
            input.action,
        );
    }

    if input.device_trust_state != DeviceTrustState::Trusted {
        return rejected(
            HouseholdAuthorizationFailureReason::DeviceNotTrusted,
            input.action,
        );
    }

    if requires_fresh_session(input.action)
        && input.session_freshness_state != SessionFreshnessState::Fresh
    {
        return rejected(
            HouseholdAuthorizationFailureReason::SessionNotFresh,
            input.action,
        );
    }

    if requires_bound_child_scope(input.action)
        && input.child_profile_binding_state != ChildProfileBindingState::Bound
    {
        return rejected(
            HouseholdAuthorizationFailureReason::ChildProfileNotBound,
            input.action,
        );
    }

    if requires_child_profile_device_scope(input.action)
        && input.device_ownership_scope != DeviceOwnershipScope::ChildProfileDevice
    {
        return rejected(
            HouseholdAuthorizationFailureReason::WrongDeviceScope,
            input.action,
        );
    }

    if requires_capability_grant(input.action) && !input.capability_granted {
        return rejected(
            HouseholdAuthorizationFailureReason::MissingCapabilityGrant,
            input.action,
        );
    }

    if !role_can_authorize(input.actor_role, input.action) {
        return rejected(
            HouseholdAuthorizationFailureReason::RoleNotAuthorized,
            input.action,
        );
    }

    if let Some(failure_reason) =
        controller_lease_failure_reason(input.action, input.controller_lease_state)
    {
        return rejected(failure_reason, input.action);
    }

    HouseholdAuthorityDecision {
        authorization_state: HouseholdAuthorizationState::Authorized,
        audit_requirement_state: audit_requirement_state(input.action),
        elevated_confirmation_state: elevated_confirmation_state(input.action),
        failure_reason: None,
    }
}

fn rejected(
    failure_reason: HouseholdAuthorizationFailureReason,
    action: HouseholdAuthorityAction,
) -> HouseholdAuthorityDecision {
    HouseholdAuthorityDecision {
        authorization_state: HouseholdAuthorizationState::Rejected,
        audit_requirement_state: audit_requirement_state(action),
        elevated_confirmation_state: elevated_confirmation_state(action),
        failure_reason: Some(failure_reason),
    }
}

fn role_can_authorize(role: HouseholdRole, action: HouseholdAuthorityAction) -> bool {
    match action {
        HouseholdAuthorityAction::PairChildDevice
        | HouseholdAuthorityAction::RevokeChildDevice
        | HouseholdAuthorityAction::ChangePolicy => {
            matches!(
                role,
                HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian
            )
        }
        HouseholdAuthorityAction::ViewChildStatus => matches!(
            role,
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian | HouseholdRole::Observer
        ),
        HouseholdAuthorityAction::StartRemoteView => matches!(
            role,
            HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian | HouseholdRole::Observer
        ),
        HouseholdAuthorityAction::StartRemoteControl => {
            matches!(
                role,
                HouseholdRole::ParentOwner | HouseholdRole::CoParentGuardian
            )
        }
        HouseholdAuthorityAction::ExportDeleteData | HouseholdAuthorityAction::ManageBilling => {
            matches!(role, HouseholdRole::ParentOwner)
        }
    }
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

fn audit_requirement_state(action: HouseholdAuthorityAction) -> AuditRequirementState {
    match action {
        HouseholdAuthorityAction::ViewChildStatus => AuditRequirementState::NotRequired,
        HouseholdAuthorityAction::PairChildDevice
        | HouseholdAuthorityAction::RevokeChildDevice
        | HouseholdAuthorityAction::ChangePolicy
        | HouseholdAuthorityAction::StartRemoteView
        | HouseholdAuthorityAction::StartRemoteControl
        | HouseholdAuthorityAction::ExportDeleteData
        | HouseholdAuthorityAction::ManageBilling => AuditRequirementState::Required,
    }
}

fn elevated_confirmation_state(action: HouseholdAuthorityAction) -> ElevatedConfirmationState {
    match action {
        HouseholdAuthorityAction::RevokeChildDevice
        | HouseholdAuthorityAction::StartRemoteControl
        | HouseholdAuthorityAction::ExportDeleteData
        | HouseholdAuthorityAction::ManageBilling => ElevatedConfirmationState::Required,
        HouseholdAuthorityAction::PairChildDevice
        | HouseholdAuthorityAction::ViewChildStatus
        | HouseholdAuthorityAction::ChangePolicy
        | HouseholdAuthorityAction::StartRemoteView => ElevatedConfirmationState::NotRequired,
    }
}
