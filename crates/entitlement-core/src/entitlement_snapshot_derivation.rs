#![forbid(unsafe_code)]

use crate::entitlement_snapshot::{
    EntitlementSnapshotDerivationError, EntitlementSnapshotDerivationInput,
    EntitlementSnapshotLimitBundle, UnsignedEntitlementSnapshotProjection,
    ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION,
};
use crate::entitlement_snapshot_values::EntitlementSnapshotPlanTier;

pub(super) fn checked_effective_child_device_limit(
    base_child_device_limit: u32,
    active_referral_credits: u32,
    paid_extra_child_device_seats: u32,
) -> Result<u32, EntitlementSnapshotDerivationError> {
    let base_child_device_limit = std::num::NonZeroU32::new(base_child_device_limit)
        .ok_or(EntitlementSnapshotDerivationError::ZeroBaseChildDeviceLimit)?
        .get();
    base_child_device_limit
        .checked_add(active_referral_credits)
        .and_then(|subtotal| subtotal.checked_add(paid_extra_child_device_seats))
        .ok_or(EntitlementSnapshotDerivationError::SeatLimitOverflow)
}

pub(super) fn derive_unsigned_entitlement_snapshot(
    input: EntitlementSnapshotDerivationInput,
) -> Result<UnsignedEntitlementSnapshotProjection, EntitlementSnapshotDerivationError> {
    input
        .provider_state
        .provider_child_device_limit_hint
        .map(|hint| {
            std::num::NonZeroU32::new(hint)
                .ok_or(EntitlementSnapshotDerivationError::ZeroProviderChildDeviceLimitHint)
        })
        .transpose()?;
    if input.billing_ledger_state.plan_tier == EntitlementSnapshotPlanTier::Starter
        && input.billing_ledger_state.base_child_device_limit != 1
    {
        return Err(EntitlementSnapshotDerivationError::InvalidStarterBaseChildDeviceLimit);
    }
    let effective_child_device_limit = checked_effective_child_device_limit(
        input.billing_ledger_state.base_child_device_limit,
        input.referral_ledger_state.active_referral_credits,
        input.billing_ledger_state.paid_extra_child_device_seats,
    )?;

    Ok(UnsignedEntitlementSnapshotProjection {
        schema_version: ENTITLEMENT_SNAPSHOT_SCHEMA_VERSION,
        snapshot_id: input.snapshot_id,
        account_ref: input.entitlement_ledger_state.account_ref,
        household_ref: input.entitlement_ledger_state.household_ref,
        trusted_device_ref: input.entitlement_ledger_state.trusted_device_ref,
        plan_tier: input.billing_ledger_state.plan_tier,
        feature_flags: input.entitlement_ledger_state.feature_flags,
        limits: EntitlementSnapshotLimitBundle {
            child_device_limit: effective_child_device_limit,
        },
        base_child_device_limit: input.billing_ledger_state.base_child_device_limit,
        active_referral_credits: input.referral_ledger_state.active_referral_credits,
        paid_extra_child_device_seats: input.billing_ledger_state.paid_extra_child_device_seats,
        effective_child_device_limit,
        issued_at: input.issued_at,
        expires_at: input.expires_at,
        grace_until: input.grace_until,
        livemode: input.provider_state.livemode,
        revocation_cursor: input.entitlement_ledger_state.revocation_cursor,
        authority_generation: input.authority_generation,
        device_trust_required: input.entitlement_ledger_state.device_trust_required,
        package_build_ref: input.entitlement_ledger_state.package_build_ref,
        release_channel: input.entitlement_ledger_state.release_channel,
    })
}
