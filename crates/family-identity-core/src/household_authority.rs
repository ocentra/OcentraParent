use serde::{Deserialize, Serialize};

use crate::family_identity::{
    ActorAccountState, ChildProfileBindingState, DeviceOwnershipScope, DeviceTrustState,
    HouseholdMembershipState, HouseholdRole, SessionFreshnessState,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HouseholdAuthorityAction {
    #[serde(rename = "seal-parent-device-trust")]
    SealParentDeviceTrust,
    #[serde(rename = "pair-child-device")]
    PairChildDevice,
    #[serde(rename = "register-lan-signer-anchor")]
    RegisterLanSignerAnchor,
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
    #[serde(rename = "missing-parent-step-up")]
    MissingParentStepUp,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HouseholdActorTargetAuthorityInput {
    pub(crate) actor_role: HouseholdRole,
    pub(crate) same_family: bool,
    pub(crate) actor_account_state: ActorAccountState,
    pub(crate) membership_state: HouseholdMembershipState,
    pub(crate) child_profile_binding_state: ChildProfileBindingState,
    pub(crate) actor_device_ownership_scope: DeviceOwnershipScope,
    pub(crate) target_device_ownership_scope: Option<DeviceOwnershipScope>,
    pub(crate) device_trust_state: DeviceTrustState,
    pub(crate) session_freshness_state: SessionFreshnessState,
    pub(crate) action: HouseholdAuthorityAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityDecision {
    pub authorization_state: HouseholdAuthorizationState,
    pub audit_requirement_state: AuditRequirementState,
    pub elevated_confirmation_state: ElevatedConfirmationState,
    pub failure_reason: Option<HouseholdAuthorizationFailureReason>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentStepUpValidationFailureReason {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "wrong-household")]
    WrongHousehold,
    #[serde(rename = "wrong-account")]
    WrongAccount,
    #[serde(rename = "wrong-action")]
    WrongAction,
    #[serde(rename = "wrong-device")]
    WrongDevice,
    #[serde(rename = "wrong-target")]
    WrongTarget,
    #[serde(rename = "replay-rejected")]
    ReplayRejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentStepUpAssertionSnapshot {
    pub family_id: String,
    pub parent_account_id: String,
    pub action_device_id: String,
    pub action_device_child_profile_id: Option<String>,
    pub target_child_profile_id: Option<String>,
    pub action: HouseholdAuthorityAction,
    pub nonce: String,
    pub expires_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentStepUpValidationInput {
    pub assertion: Option<ParentStepUpAssertionSnapshot>,
    pub family_id: String,
    pub parent_account_id: String,
    pub action_device_id: String,
    pub action_device_child_profile_id: Option<String>,
    pub target_child_profile_id: Option<String>,
    pub action: HouseholdAuthorityAction,
    pub observed_at: String,
    pub expected_nonce: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentStepUpValidationDecision {
    pub valid: bool,
    pub failure_reason: Option<ParentStepUpValidationFailureReason>,
}

pub fn authorize_household_action(input: HouseholdAuthorityInput) -> HouseholdAuthorityDecision {
    if let Some(failure_reason) =
        crate::household_authority_validation::household_authority_failure_reason(&input)
    {
        return rejected(failure_reason, input.action);
    }

    HouseholdAuthorityDecision {
        authorization_state: HouseholdAuthorizationState::Authorized,
        audit_requirement_state: crate::household_authority_validation::audit_requirement_state(
            input.action,
        ),
        elevated_confirmation_state:
            crate::household_authority_validation::elevated_confirmation_state(input.action),
        failure_reason: None,
    }
}

pub(crate) fn authorize_household_actor_target_action(
    input: HouseholdActorTargetAuthorityInput,
) -> HouseholdAuthorityDecision {
    if let Some(failure_reason) =
        crate::household_authority_validation::household_actor_target_authority_failure_reason(
            &input,
        )
    {
        return rejected(failure_reason, input.action);
    }

    HouseholdAuthorityDecision {
        authorization_state: HouseholdAuthorizationState::Authorized,
        audit_requirement_state: crate::household_authority_validation::audit_requirement_state(
            input.action,
        ),
        elevated_confirmation_state:
            crate::household_authority_validation::elevated_confirmation_state(input.action),
        failure_reason: None,
    }
}

pub fn requires_parent_step_up(action: HouseholdAuthorityAction) -> bool {
    matches!(
        action,
        HouseholdAuthorityAction::SealParentDeviceTrust
            | HouseholdAuthorityAction::PairChildDevice
            | HouseholdAuthorityAction::RegisterLanSignerAnchor
            | HouseholdAuthorityAction::RevokeChildDevice
            | HouseholdAuthorityAction::ChangePolicy
            | HouseholdAuthorityAction::StartRemoteControl
            | HouseholdAuthorityAction::ExportDeleteData
            | HouseholdAuthorityAction::ManageBilling
    )
}

pub fn validate_parent_step_up_assertion(
    input: &ParentStepUpValidationInput,
) -> ParentStepUpValidationDecision {
    let Some(assertion) = input.assertion.as_ref() else {
        return rejected_parent_step_up_validation(ParentStepUpValidationFailureReason::Required);
    };

    if let Some(failure_reason) =
        crate::household_authority_validation::parent_step_up_validation_failure_reason(
            input, assertion,
        )
    {
        return rejected_parent_step_up_validation(failure_reason);
    }

    ParentStepUpValidationDecision {
        valid: true,
        failure_reason: None,
    }
}

fn rejected(
    failure_reason: HouseholdAuthorizationFailureReason,
    action: HouseholdAuthorityAction,
) -> HouseholdAuthorityDecision {
    HouseholdAuthorityDecision {
        authorization_state: HouseholdAuthorizationState::Rejected,
        audit_requirement_state: crate::household_authority_validation::audit_requirement_state(
            action,
        ),
        elevated_confirmation_state:
            crate::household_authority_validation::elevated_confirmation_state(action),
        failure_reason: Some(failure_reason),
    }
}

fn rejected_parent_step_up_validation(
    failure_reason: ParentStepUpValidationFailureReason,
) -> ParentStepUpValidationDecision {
    ParentStepUpValidationDecision {
        valid: false,
        failure_reason: Some(failure_reason),
    }
}
