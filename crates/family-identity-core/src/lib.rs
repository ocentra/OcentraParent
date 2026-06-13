#![forbid(unsafe_code)]

//! Family identity and device-role ownership boundary.
//!
//! This crate owns household membership, child/profile/device role contracts,
//! local authorization decisions, invite/recovery state, and device-ownership
//! checks shared by parent and child runtimes.

pub const CRATE_NAME: &str = "ocentra-family-identity-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FamilyActorRole {
    Parent,
    Guardian,
    Child,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdMembership {
    Member,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActorAccountState {
    Active,
    Suspended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildProfileBindingState {
    Bound,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceOwnershipScope {
    ChildProfileDevice,
    OtherDevice,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildDisclosureState {
    Disclosed,
    NotDisclosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceScopeAuthorizationState {
    Authorized,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentAuthorityRequirementState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceScopeInput {
    pub actor_role: FamilyActorRole,
    pub actor_account_state: ActorAccountState,
    pub household_membership: HouseholdMembership,
    pub child_profile_binding_state: ChildProfileBindingState,
    pub device_ownership_scope: DeviceOwnershipScope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceScopeDecision {
    pub authorization_state: DeviceScopeAuthorizationState,
    pub parent_authority_requirement_state: ParentAuthorityRequirementState,
}

pub fn authorize_child_device_scope(input: DeviceScopeInput) -> DeviceScopeDecision {
    let has_parent_authority = matches!(
        input.actor_role,
        FamilyActorRole::Parent | FamilyActorRole::Guardian
    );
    let allowed = input.household_membership == HouseholdMembership::Member
        && input.actor_account_state == ActorAccountState::Active
        && input.child_profile_binding_state == ChildProfileBindingState::Bound
        && input.device_ownership_scope == DeviceOwnershipScope::ChildProfileDevice
        && has_parent_authority;

    DeviceScopeDecision {
        authorization_state: if allowed {
            DeviceScopeAuthorizationState::Authorized
        } else {
            DeviceScopeAuthorizationState::Rejected
        },
        parent_authority_requirement_state: if allowed {
            ParentAuthorityRequirementState::NotRequired
        } else {
            ParentAuthorityRequirementState::Required
        },
    }
}
