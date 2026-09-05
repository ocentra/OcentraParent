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
use ocentra_schema::account_identity_authority::AccountIdentityProvider;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::AccountIdentityAuthorityService;
use crate::device_trust_current_binding::CurrentChildDeviceTrustBinding;
use crate::device_trust_lifecycle::DeviceTrustLifecycleState;
use crate::household_authority::HouseholdAuthorityAction;

#[path = "household_authority_runtime_account.rs"]
mod household_authority_runtime_account;
#[path = "household_authority_runtime_authorization.rs"]
mod household_authority_runtime_authorization;
#[path = "household_authority_runtime_binding.rs"]
mod household_authority_runtime_binding;
#[path = "household_authority_runtime_capability.rs"]
mod household_authority_runtime_capability;
#[path = "household_authority_runtime_consume.rs"]
mod household_authority_runtime_consume;
#[path = "household_authority_runtime_device_source.rs"]
mod household_authority_runtime_device_source;
#[path = "household_authority_runtime_device_validation.rs"]
mod household_authority_runtime_device_validation;
#[path = "household_authority_runtime_lease.rs"]
mod household_authority_runtime_lease;
#[path = "household_authority_runtime_ports.rs"]
mod household_authority_runtime_ports;
#[path = "household_authority_runtime_requirements.rs"]
mod household_authority_runtime_requirements;
#[path = "household_authority_runtime_resolution.rs"]
mod household_authority_runtime_resolution;
#[path = "household_authority_runtime_revalidation.rs"]
mod household_authority_runtime_revalidation;
#[path = "household_authority_runtime_step_up.rs"]
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
    EffectTargetMismatch,
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

/// The exact typed target bound by the owner-issued execution receipt.
///
/// This type has no public constructor and keeps every target identity and currentness snapshot
/// private. A downstream caller cannot pair a receipt with caller-assembled household, account,
/// child, device, provider, session, route, or generation scalars. A real effect owner either
/// constructs this target inside the CAS owner or receives it from that owner and consumes the
/// receipt by value.
pub struct HouseholdAuthorityRuntimeEffectTarget {
    action: HouseholdAuthorityAction,
    household_id: String,
    account_id: String,
    parent_device_id: String,
    child_profile_id: String,
    child_device_id: String,
    provider: AccountIdentityProvider,
    provider_subject: String,
    session_id: String,
    session_expires_at: String,
    session_generation: u64,
    account_authority_generation: u64,
    account_binding_authority_generation: u64,
    installation_id: String,
    pairing_id: String,
    route_id: String,
    device_trust_subject: String,
    device_signer_key_id: String,
    device_signer_key_sha256: String,
    device_state: DeviceTrustLifecycleState,
    device_lifecycle_generation: u64,
    device_installation_binding_generation: u64,
    device_authority_generation: u64,
    capability_authority_generation: Option<u64>,
    capability_expires_at: Option<DateTime<Utc>>,
    capability_revocation_epoch: Option<u64>,
    controller_lease_authority_generation: Option<u64>,
    controller_lease_expires_at: Option<DateTime<Utc>>,
    controller_lease_revocation_epoch: Option<u64>,
    parent_step_up_authority_generation: Option<u64>,
    parent_step_up_expires_at: Option<DateTime<Utc>>,
    parent_step_up_receipt_epoch: Option<u64>,
}

/// The only value a downstream effect owner may receive after an authorization has been
/// revalidated and atomically consumed. It is a target-bound, single-use receipt rather than an
/// action flag: its private target cannot be inspected, copied, serialized, or paired with another
/// target by a caller. The CAS nonce is consumed by the owner fence that issues this value.
pub struct HouseholdAuthorityRuntimeEffectAuthorization {
    target: HouseholdAuthorityRuntimeEffectTarget,
}

/// Terminal effect handoff produced by consuming the owner-issued receipt with the exact typed
/// target. It remains opaque and non-reusable; the receipt and target are both moved by value.
#[non_exhaustive]
pub struct HouseholdAuthorityRuntimeConsumedEffect {}

/// Current owner snapshots presented to the final execution-time CAS fence.
///
/// The composer, not its caller, resolves every value in this input immediately before the
/// authorization is consumed. Grouping those values keeps the fence boundary explicit without
/// turning any snapshot into caller-supplied authority.
pub struct HouseholdAuthorityRuntimeCasInput {
    pub authorization: HouseholdAuthorityRuntimeAuthorization,
    pub current_account_authority: VerifiedAccountIdentityAuthority,
    pub current_device_binding: CurrentChildDeviceTrustBinding,
    pub current_capability: Option<CurrentHouseholdCapability>,
    pub current_controller_lease: Option<CurrentHouseholdControllerLease>,
    pub current_parent_step_up: Option<ConsumedParentStepUp>,
    pub consumption_nonce: [u8; 32],
}

/// Owner seam for the final execution-time CAS/revocation fence.
///
/// The caller supplies no current state. `consume_household_authority` resolves current Account,
/// Device Trust, capability, lease, and step-up state immediately before invoking this seam. An
/// implementation must atomically compare the private authorization nonce and all supplied owner
/// snapshots against its revocation/currentness store before issuing the target-bound receipt.
/// The owner must either consume that receipt inside the same CAS owner or use the exact opaque
/// target returned by its own owner boundary and call `consume_for_target` by value. Returning an
/// effect without that owner CAS is an authority bug.
pub trait HouseholdAuthorityRuntimeCasFence {
    fn compare_and_consume(
        &mut self,
        input: HouseholdAuthorityRuntimeCasInput,
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

/// Owner ports and one-shot authorization consumed at the execution boundary.
pub struct HouseholdAuthorityRuntimeConsumeInput<
    'a,
    DeviceTrustSource,
    CapabilitySource,
    ControllerLeaseSource,
    ParentStepUpSource,
    CasFence,
> {
    pub account_service: &'a AccountIdentityAuthorityService,
    pub presented_account_authority: &'a VerifiedAccountIdentityAuthority,
    pub device_trust_source: &'a DeviceTrustSource,
    pub capability_source: &'a CapabilitySource,
    pub controller_lease_source: &'a ControllerLeaseSource,
    pub parent_step_up_source: &'a ParentStepUpSource,
    pub cas_fence: &'a mut CasFence,
    pub authorization: HouseholdAuthorityRuntimeAuthorization,
}

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
pub fn consume_household_authority<
    DeviceTrustSource: HouseholdAuthorityDeviceTrustSource,
    CapabilitySource: HouseholdAuthorityCapabilitySource,
    ControllerLeaseSource: HouseholdAuthorityControllerLeaseSource,
    ParentStepUpSource: HouseholdAuthorityParentStepUpSource,
    CasFence: HouseholdAuthorityRuntimeCasFence,
>(
    input: HouseholdAuthorityRuntimeConsumeInput<
        '_,
        DeviceTrustSource,
        CapabilitySource,
        ControllerLeaseSource,
        ParentStepUpSource,
        CasFence,
    >,
) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure> {
    household_authority_runtime_consume::consume(input)
}
