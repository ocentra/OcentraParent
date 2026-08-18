//! Family-owned composition of current household authority.
//!
//! The existing household evaluator is deliberately a pure policy function. It accepts a
//! complete input value so that policy rules can be tested, but that value is not authority.
//! This module is the runtime seam that may turn only owner-issued current values into an
//! opaque authorization. It never accepts caller-assembled ownership, trust, session,
//! capability, lease, or step-up state.
//!
//! Account currentness is resolved again for every composition. Device Trust is resolved from
//! its durable signer authority and consumed into the existing non-serializable child binding.
//! Capability, controller-lease, and parent-step-up owners are explicit ports: until a real
//! owner implements one of those ports, the corresponding action remains unavailable/manual
//! required. A transport DTO or a public proof cannot satisfy any of these ports.

use chrono::{DateTime, Utc};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::AccountIdentityAuthorityService;
use crate::device_trust_current_binding::CurrentChildDeviceTrustBinding;
use crate::household_authority::HouseholdAuthorityAction;

mod household_authority_runtime_account;
mod household_authority_runtime_authorization;
mod household_authority_runtime_binding;
mod household_authority_runtime_capability;
mod household_authority_runtime_device_source;
mod household_authority_runtime_device_validation;
mod household_authority_runtime_lease;
mod household_authority_runtime_ports;
mod household_authority_runtime_requirements;
mod household_authority_runtime_resolution;
mod household_authority_runtime_step_up;

/// Failure from an owner boundary or from the cross-owner binding checks.
///
/// These outcomes intentionally do not expose a caller-controlled policy decision. A caller can
/// request an action, but it cannot choose the state that would make that action succeed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdAuthorityRuntimeFailure {
    AccountAuthorityUnavailable,
    AccountAuthorityRevoked,
    AccountAuthorityStale,
    AccountAuthorityGenerationMismatch,
    SessionStale,
    DeviceTrustUnavailable,
    DeviceTrustRevoked,
    DeviceTrustBindingMismatch,
    DeviceTrustGenerationMismatch,
    CapabilityUnavailable,
    CapabilityExpired,
    CapabilityRevoked,
    CapabilityBindingMismatch,
    ControllerLeaseUnavailable,
    ControllerLeaseExpired,
    ControllerLeaseRevoked,
    ControllerLeaseBindingMismatch,
    ParentStepUpUnavailable,
    ParentStepUpExpired,
    ParentStepUpReplayRejected,
    ParentStepUpBindingMismatch,
    RoleNotAuthorized,
    ManualRequired,
}

/// An owner-issued capability for one exact action and current account/device generation.
///
/// The constructor is crate-private. The value is intentionally neither `Clone` nor serde so a
/// serialized capability or a copied value cannot become a second authority.
pub struct CurrentHouseholdCapability {
    household_id: String,
    account_id: String,
    parent_device_id: String,
    child_profile_id: String,
    child_device_id: String,
    action: HouseholdAuthorityAction,
    authority_generation: u64,
    expires_at: DateTime<Utc>,
    revocation_epoch: u64,
}

/// An owner-issued controller lease for one exact action and current account/device generation.
///
/// This is intentionally distinct from `family_identity::ParentControllerLease`, which is a
/// serializable record/evidence shape and cannot be consumed as runtime authority.
pub struct CurrentHouseholdControllerLease {
    household_id: String,
    account_id: String,
    parent_device_id: String,
    child_profile_id: String,
    child_device_id: String,
    action: HouseholdAuthorityAction,
    authority_generation: u64,
    expires_at: DateTime<Utc>,
    revocation_epoch: u64,
}

/// A one-time parent step-up consumed by its owner for one exact action.
///
/// This type is not a receipt DTO. The owner must consume the durable nonce/replay record before
/// returning it, and the constructor is crate-private so a caller cannot mint a step-up by
/// copying a signed assertion.
pub struct ConsumedParentStepUp {
    household_id: String,
    account_id: String,
    parent_device_id: String,
    child_profile_id: String,
    child_device_id: String,
    action: HouseholdAuthorityAction,
    authority_generation: u64,
    expires_at: DateTime<Utc>,
    receipt_epoch: u64,
}

/// The only positive runtime result from this module.
///
/// The fields are private and the type has no `Clone`, serde, or public constructor. It is a
/// capability owned by this composer, not a transport handoff or a policy decision DTO.
pub struct HouseholdAuthorityRuntimeAuthorization {
    action: HouseholdAuthorityAction,
    account_authority_generation: u64,
    session_generation: u64,
    device_binding: CurrentChildDeviceTrustBinding,
    capability: Option<CurrentHouseholdCapability>,
    controller_lease: Option<CurrentHouseholdControllerLease>,
    parent_step_up: Option<ConsumedParentStepUp>,
}

/// Current Device Trust owner seam.
pub trait HouseholdAuthorityDeviceTrustSource {
    fn current_device_trust_binding(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<CurrentChildDeviceTrustBinding, HouseholdAuthorityRuntimeFailure>;
}

/// Current capability owner seam. Implementations must return only an owner-issued opaque
/// capability for the exact authority/device/action tuple.
pub trait HouseholdAuthorityCapabilitySource {
    fn current_capability(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        action: HouseholdAuthorityAction,
    ) -> Result<CurrentHouseholdCapability, HouseholdAuthorityRuntimeFailure>;
}

/// Current controller-lease owner seam. A serializable `ParentControllerLease` is not a valid
/// implementation result; the owner must issue `CurrentHouseholdControllerLease` itself.
pub trait HouseholdAuthorityControllerLeaseSource {
    fn current_controller_lease(
        &self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        action: HouseholdAuthorityAction,
    ) -> Result<CurrentHouseholdControllerLease, HouseholdAuthorityRuntimeFailure>;
}

/// One-time parent step-up owner seam. Implementations must consume durable replay/nonce state
/// before returning the opaque value; callers do not provide a receipt or assertion snapshot.
pub trait HouseholdAuthorityParentStepUpSource {
    fn consume_current_parent_step_up(
        &mut self,
        account_authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        action: HouseholdAuthorityAction,
    ) -> Result<ConsumedParentStepUp, HouseholdAuthorityRuntimeFailure>;
}

/// Explicit fail-closed Device Trust adapter for deployments that have not wired the owner.
#[derive(Debug, Default)]
pub struct ManualRequiredHouseholdAuthorityDeviceTrustSource;

/// Explicit fail-closed capability adapter until a current capability owner is integrated.
#[derive(Debug, Default)]
pub struct ManualRequiredHouseholdAuthorityCapabilitySource;

/// Explicit fail-closed controller-lease adapter until a current lease owner is integrated.
#[derive(Debug, Default)]
pub struct ManualRequiredHouseholdAuthorityControllerLeaseSource;

/// Explicit fail-closed step-up adapter until a durable one-time receipt owner is integrated.
#[derive(Debug, Default)]
pub struct ManualRequiredHouseholdAuthorityParentStepUpSource;

/// Resolve and compose a current runtime authorization.
///
/// The presented account value is only an opaque provider-verified identity handle. It is used
/// to select the owner lookup key; the composer immediately re-resolves current Account state
/// and composes from that fresh value. No public argument carries role, household, trust,
/// freshness, capability, lease, or step-up scalars.
pub fn compose_household_authority(
    account_service: &AccountIdentityAuthorityService,
    presented_account_authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    capability_source: &impl HouseholdAuthorityCapabilitySource,
    controller_lease_source: &impl HouseholdAuthorityControllerLeaseSource,
    parent_step_up_source: &mut impl HouseholdAuthorityParentStepUpSource,
    action: HouseholdAuthorityAction,
) -> Result<HouseholdAuthorityRuntimeAuthorization, HouseholdAuthorityRuntimeFailure> {
    let account_authority = household_authority_runtime_resolution::account_authority(
        account_service,
        presented_account_authority,
    )?;
    household_authority_runtime_account::validate_current(&account_authority)?;
    if !household_authority_runtime_account::role_can_authorize(account_authority.role(), action) {
        return Err(HouseholdAuthorityRuntimeFailure::RoleNotAuthorized);
    }

    let device_binding = device_trust_source.current_device_trust_binding(&account_authority)?;
    household_authority_runtime_device_validation::validate_current(
        &account_authority,
        &device_binding,
    )?;

    let capability = household_authority_runtime_resolution::capability(
        capability_source,
        &account_authority,
        &device_binding,
        action,
    )?;
    let controller_lease = household_authority_runtime_resolution::controller_lease(
        controller_lease_source,
        &account_authority,
        &device_binding,
        action,
    )?;
    let parent_step_up = household_authority_runtime_resolution::parent_step_up(
        parent_step_up_source,
        &account_authority,
        &device_binding,
        action,
    )?;

    Ok(HouseholdAuthorityRuntimeAuthorization {
        action,
        account_authority_generation: account_authority.authority_generation(),
        session_generation: account_authority.session_generation(),
        device_binding,
        capability,
        controller_lease,
        parent_step_up,
    })
}
