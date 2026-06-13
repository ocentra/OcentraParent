#![forbid(unsafe_code)]

//! Setup, install, and provisioning ownership boundary.
//!
//! This crate owns install journey state, pairing readiness, permission
//! onboarding, recovery, and provisioning contracts. Binary updater mechanics
//! remain in the updater crate.

use ocentra_family_identity_core::{DeviceOwnershipScope, HouseholdMembership};

pub const CRATE_NAME: &str = "ocentra-provisioning-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentDeviceRegistrationState {
    Registered,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequiredPermissionState {
    Granted,
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairingTokenState {
    Valid,
    MissingOrExpired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryState {
    Normal,
    RecoveryRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildRuntimeReadinessState {
    Ready,
    NotReady,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvisioningManualStepState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvisioningReadinessInput {
    pub household_membership: HouseholdMembership,
    pub parent_device_registration_state: ParentDeviceRegistrationState,
    pub child_device_ownership_scope: DeviceOwnershipScope,
    pub required_permission_state: RequiredPermissionState,
    pub pairing_token_state: PairingTokenState,
    pub recovery_state: RecoveryState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProvisioningReadinessDecision {
    pub child_runtime_readiness_state: ChildRuntimeReadinessState,
    pub manual_step_state: ProvisioningManualStepState,
}

pub fn evaluate_provisioning_readiness(
    input: ProvisioningReadinessInput,
) -> ProvisioningReadinessDecision {
    let ready_for_child_runtime = input.household_membership == HouseholdMembership::Member
        && input.parent_device_registration_state == ParentDeviceRegistrationState::Registered
        && input.child_device_ownership_scope == DeviceOwnershipScope::ChildProfileDevice
        && input.required_permission_state == RequiredPermissionState::Granted
        && input.pairing_token_state == PairingTokenState::Valid
        && input.recovery_state == RecoveryState::Normal;

    ProvisioningReadinessDecision {
        child_runtime_readiness_state: if ready_for_child_runtime {
            ChildRuntimeReadinessState::Ready
        } else {
            ChildRuntimeReadinessState::NotReady
        },
        manual_step_state: if ready_for_child_runtime {
            ProvisioningManualStepState::NotRequired
        } else {
            ProvisioningManualStepState::Required
        },
    }
}
