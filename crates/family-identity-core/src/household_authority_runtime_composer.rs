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
mod household_authority_runtime_consume;
mod household_authority_runtime_device_source;
mod household_authority_runtime_device_validation;
mod household_authority_runtime_lease;
mod household_authority_runtime_ports;
mod household_authority_runtime_requirements;
mod household_authority_runtime_resolution;
mod household_authority_runtime_revalidation;
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
    RuntimeFenceUnavailable,
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
    installation_id: String,
    pairing_id: String,
    route_id: String,
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
    installation_id: String,
    pairing_id: String,
    route_id: String,
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
    installation_id: String,
    pairing_id: String,
    route_id: String,
    action: HouseholdAuthorityAction,
    authority_generation: u64,
    expires_at: DateTime<Utc>,
    receipt_epoch: u64,
}

/// The only positive composition result from this module.
///
/// The fields are private and the type has no `Clone`, serde, or public constructor. It is a
/// one-time input to the owner-issued execution fence, not an effect permission, transport
/// handoff, or policy decision DTO. Downstream must move it into `consume_household_authority`.
pub struct HouseholdAuthorityRuntimeAuthorization {
    action: HouseholdAuthorityAction,
    account_authority_generation: u64,
    session_generation: u64,
    session_id: String,
    session_expires_at: String,
    installation_id: String,
    pairing_id: String,
    route_id: String,
    consumption_nonce: [u8; 32],
    device_binding: CurrentChildDeviceTrustBinding,
    capability: Option<CurrentHouseholdCapability>,
    controller_lease: Option<CurrentHouseholdControllerLease>,
    parent_step_up: Option<ConsumedParentStepUp>,
}

/// The only value a downstream effect owner may receive after an authorization has been
/// revalidated and atomically consumed. It is deliberately distinct from the composer's
/// positive authorization: the latter is a one-time input to the owner-issued CAS fence, never
/// an effect permission by itself.
pub struct HouseholdAuthorityRuntimeEffectAuthorization {
    action: HouseholdAuthorityAction,
}

/// Owner seam for the final execution-time CAS/revocation fence.
///
/// The caller supplies no current state. `consume_household_authority` resolves current Account,
/// Device Trust, capability, lease, and step-up state immediately before invoking this seam. An
/// implementation must atomically compare the private authorization nonce and all supplied owner
/// snapshots against its revocation/currentness store before returning the opaque effect value.
/// Returning an effect without that owner CAS is an authority bug.
pub trait HouseholdAuthorityRuntimeCasFence {
    fn compare_and_consume(
        &mut self,
        authorization: HouseholdAuthorityRuntimeAuthorization,
        current_account_authority: VerifiedAccountIdentityAuthority,
        current_device_binding: CurrentChildDeviceTrustBinding,
        current_capability: Option<CurrentHouseholdCapability>,
        current_controller_lease: Option<CurrentHouseholdControllerLease>,
        current_parent_step_up: Option<ConsumedParentStepUp>,
        consumption_nonce: &[u8; 32],
    ) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure>;
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

    /// Re-resolve the already-consumed one-time step-up at effect time without minting or
    /// consuming a second receipt. A durable owner returns a fresh opaque currentness snapshot;
    /// the CAS fence must compare its receipt generation and replay/revocation state. The default
    /// is fail-closed/manual-required.
    fn revalidate_current_parent_step_up(
        &self,
        _account_authority: &VerifiedAccountIdentityAuthority,
        _device_binding: &CurrentChildDeviceTrustBinding,
        _action: HouseholdAuthorityAction,
        _expected: &ConsumedParentStepUp,
    ) -> Result<ConsumedParentStepUp, HouseholdAuthorityRuntimeFailure> {
        Err(HouseholdAuthorityRuntimeFailure::ParentStepUpUnavailable)
    }
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

/// Explicit fail-closed execution fence until a durable owner wires a CAS/revocation store for
/// the composed nonce and all dependency generations.
#[derive(Debug, Default)]
pub struct ManualRequiredHouseholdAuthorityRuntimeCasFence;

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

    let mut consumption_nonce = [0_u8; 32];
    getrandom::fill(&mut consumption_nonce)
        .map_err(|_error| HouseholdAuthorityRuntimeFailure::RuntimeFenceUnavailable)?;

    Ok(HouseholdAuthorityRuntimeAuthorization {
        action,
        account_authority_generation: account_authority.authority_generation(),
        session_generation: account_authority.session_generation(),
        session_id: account_authority.session_id().as_str().to_owned(),
        session_expires_at: account_authority.session_expires_at().to_owned(),
        installation_id: account_authority
            .current_binding()
            .installation_id
            .as_str()
            .to_owned(),
        pairing_id: account_authority
            .current_binding()
            .pairing_id
            .as_str()
            .to_owned(),
        route_id: account_authority
            .current_binding()
            .selected_route_id
            .as_str()
            .to_owned(),
        consumption_nonce,
        device_binding,
        capability,
        controller_lease,
        parent_step_up,
    })
}

/// Re-resolve every owner boundary immediately before an effect and consume the authorization by
/// value through the owner-issued CAS/revocation fence. A failed fence consumes the value without
/// yielding a reusable positive authorization.
pub fn consume_household_authority(
    account_service: &AccountIdentityAuthorityService,
    presented_account_authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    capability_source: &impl HouseholdAuthorityCapabilitySource,
    controller_lease_source: &impl HouseholdAuthorityControllerLeaseSource,
    parent_step_up_source: &impl HouseholdAuthorityParentStepUpSource,
    cas_fence: &mut impl HouseholdAuthorityRuntimeCasFence,
    authorization: HouseholdAuthorityRuntimeAuthorization,
) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure> {
    household_authority_runtime_consume::consume(
        account_service,
        presented_account_authority,
        device_trust_source,
        capability_source,
        controller_lease_source,
        parent_step_up_source,
        cas_fence,
        authorization,
    )
}
