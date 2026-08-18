use chrono::{DateTime, Utc};
use ocentra_schema::parent_storage_settings_apply_flow::{
    ParentStorageApplyIntentDigest, ParentStoragePreviewId,
};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_authority_repository::{
    parent_storage_confirmation_store::{
        ParentStorageConfirmationBinding, ParentStorageConfirmationStoreError,
    },
    AccountIdentityAuthorityService,
};
use crate::household_authority::HouseholdAuthorityAction;

use super::{
    compose_household_authority, consume_household_authority, ConsumedParentStorageConfirmation,
    HouseholdAuthorityCapabilitySource, HouseholdAuthorityControllerLeaseSource,
    HouseholdAuthorityDeviceTrustSource, HouseholdAuthorityParentStepUpSource,
    HouseholdAuthorityParentStorageConfirmationFailure,
    HouseholdAuthorityParentStorageStoreFailure, HouseholdAuthorityRuntimeCasFence,
    HouseholdAuthorityRuntimeConsumedEffect, HouseholdAuthorityRuntimeFailure,
    HouseholdAuthorityRuntimeParentStorageConfirmation,
    HouseholdAuthorityRuntimeParentStorageExecutorHandoff,
};

/// Compose and durably stage the family-owned confirmation for one canonical storage intent.
///
/// The only positive path starts with the existing Account/Device Trust/action composer and
/// consumes its opaque effect authorization by value. The durable Account store then records the
/// private receipt identity and the exact owner-bound target. Preview and digest are intent-only
/// inputs; they never carry a receipt, lifecycle state, or currentness snapshot.
pub fn compose_parent_storage_apply_confirmation(
    account_service: &mut AccountIdentityAuthorityService,
    presented_account_authority: &VerifiedAccountIdentityAuthority,
    device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    capability_source: &impl HouseholdAuthorityCapabilitySource,
    controller_lease_source: &impl HouseholdAuthorityControllerLeaseSource,
    parent_step_up_source: &mut impl HouseholdAuthorityParentStepUpSource,
    cas_fence: &mut impl HouseholdAuthorityRuntimeCasFence,
    preview_id: ParentStoragePreviewId,
    apply_intent_digest: ParentStorageApplyIntentDigest,
) -> Result<HouseholdAuthorityRuntimeParentStorageConfirmation, HouseholdAuthorityRuntimeFailure> {
    let authorization = compose_household_authority(
        account_service,
        presented_account_authority,
        device_trust_source,
        capability_source,
        controller_lease_source,
        parent_step_up_source,
        HouseholdAuthorityAction::ImportRestoreData,
    )?;
    let effect_authorization = consume_household_authority(
        account_service,
        presented_account_authority,
        device_trust_source,
        capability_source,
        controller_lease_source,
        parent_step_up_source,
        cas_fence,
        authorization,
    )?;
    let effect = effect_authorization
        .consume_for_data_custody(
            HouseholdAuthorityAction::ImportRestoreData,
            presented_account_authority.household_id().as_str(),
            Some(presented_account_authority.child_device_id().as_str()),
            Some(presented_account_authority.authority_generation()),
        )
        .map_err(|_| HouseholdAuthorityRuntimeFailure::EffectTargetMismatch)?;
    let current_authority = super::household_authority_runtime_resolution::account_authority(
        account_service,
        presented_account_authority,
    )?;
    let staged = {
        let binding = effect_binding(&effect);
        account_service
            .stage_parent_storage_confirmation(
                &current_authority,
                device_trust_source,
                binding,
                &preview_id,
                &apply_intent_digest,
            )
            .map_err(map_store_failure)?
    };
    let expires_at = DateTime::<Utc>::from_timestamp_millis(staged.expires_at_epoch_millis).ok_or(
        HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(
            HouseholdAuthorityParentStorageStoreFailure::ClockUnavailable,
        ),
    )?;
    Ok(HouseholdAuthorityRuntimeParentStorageConfirmation {
        effect,
        authority: current_authority,
        preview_id,
        apply_intent_digest,
        receipt_id: staged.receipt_id,
        nonce_id: staged.nonce_id,
        receipt_epoch: staged.receipt_epoch,
        expires_at,
    })
}

impl HouseholdAuthorityRuntimeParentStorageConfirmation {
    /// Re-resolve Account and Device Trust inside the Account transaction and consume the staged
    /// row with a compare-and-swap. The returned value is the only family-owned handoff to the
    /// storage boundary; the input receipt and durable row cannot be reused.
    pub fn consume_for_storage(
        self,
        account_service: &mut AccountIdentityAuthorityService,
        device_trust_source: &impl HouseholdAuthorityDeviceTrustSource,
    ) -> Result<
        HouseholdAuthorityRuntimeParentStorageExecutorHandoff,
        HouseholdAuthorityParentStorageConfirmationFailure,
    > {
        let Self {
            effect,
            authority,
            preview_id,
            apply_intent_digest,
            receipt_id,
            nonce_id,
            receipt_epoch,
            expires_at,
        } = self;
        let stored = {
            let binding = effect_binding(&effect);
            account_service
                .consume_parent_storage_confirmation(
                    &authority,
                    device_trust_source,
                    binding,
                    &receipt_id,
                    &nonce_id,
                    receipt_epoch,
                    &preview_id,
                    &apply_intent_digest,
                )
                .map_err(map_store_consume_failure)?
        };
        if stored.receipt_id != receipt_id
            || stored.nonce_id != nonce_id
            || stored.receipt_epoch != receipt_epoch
            || stored.expires_at_epoch_millis != expires_at.timestamp_millis()
        {
            return Err(HouseholdAuthorityParentStorageConfirmationFailure::Store(
                HouseholdAuthorityParentStorageStoreFailure::BindingMismatch,
            ));
        }
        let target = &effect.target;
        let confirmation = ConsumedParentStorageConfirmation {
            household_id: target.household_id.clone(),
            account_id: target.account_id.clone(),
            parent_device_id: target.parent_device_id.clone(),
            child_profile_id: target.child_profile_id.clone(),
            child_device_id: target.child_device_id.clone(),
            installation_id: target.installation_id.clone(),
            pairing_id: target.pairing_id.clone(),
            route_id: target.route_id.clone(),
            authority_generation: target.account_authority_generation,
            receipt_id: stored.receipt_id,
            nonce_id: stored.nonce_id,
            preview_id,
            apply_intent_digest,
            expires_at,
            receipt_epoch: stored.receipt_epoch,
        };
        Ok(HouseholdAuthorityRuntimeParentStorageExecutorHandoff {
            effect,
            confirmation,
        })
    }
}

fn effect_binding(
    effect: &HouseholdAuthorityRuntimeConsumedEffect,
) -> ParentStorageConfirmationBinding<'_> {
    let target = &effect.target;
    ParentStorageConfirmationBinding {
        provider: &target.provider,
        provider_subject: &target.provider_subject,
        household_id: &target.household_id,
        account_id: &target.account_id,
        parent_device_id: &target.parent_device_id,
        child_profile_id: &target.child_profile_id,
        child_device_id: &target.child_device_id,
        installation_id: &target.installation_id,
        pairing_id: &target.pairing_id,
        route_id: &target.route_id,
        authority_generation: target.account_authority_generation,
        session_generation: target.session_generation,
        device_trust_subject: &target.device_trust_subject,
        device_lifecycle_generation: target.device_lifecycle_generation,
        device_authority_generation: target.device_authority_generation,
    }
}

fn map_store_failure(
    failure: ParentStorageConfirmationStoreError,
) -> HouseholdAuthorityRuntimeFailure {
    HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationStore(failure)
}

fn map_store_consume_failure(
    failure: ParentStorageConfirmationStoreError,
) -> HouseholdAuthorityParentStorageConfirmationFailure {
    match failure {
        store @ (HouseholdAuthorityParentStorageStoreFailure::Unavailable
        | HouseholdAuthorityParentStorageStoreFailure::IntegrityRejected
        | HouseholdAuthorityParentStorageStoreFailure::ClockUnavailable
        | HouseholdAuthorityParentStorageStoreFailure::EntropyUnavailable
        | HouseholdAuthorityParentStorageStoreFailure::Duplicate
        | HouseholdAuthorityParentStorageStoreFailure::Missing
        | HouseholdAuthorityParentStorageStoreFailure::Expired
        | HouseholdAuthorityParentStorageStoreFailure::ReplayRejected
        | HouseholdAuthorityParentStorageStoreFailure::BindingMismatch
        | HouseholdAuthorityParentStorageStoreFailure::Conflict) => {
            HouseholdAuthorityParentStorageConfirmationFailure::Store(store)
        }
        HouseholdAuthorityParentStorageStoreFailure::Owner(owner) => {
            HouseholdAuthorityParentStorageConfirmationFailure::Store(
                HouseholdAuthorityParentStorageStoreFailure::Owner(owner),
            )
        }
        HouseholdAuthorityParentStorageStoreFailure::AccountAuthorityUnavailable => {
            HouseholdAuthorityParentStorageConfirmationFailure::AccountAuthorityUnavailable
        }
        HouseholdAuthorityParentStorageStoreFailure::AccountAuthorityNotCurrent => {
            HouseholdAuthorityParentStorageConfirmationFailure::AccountAuthorityNotCurrent
        }
        HouseholdAuthorityParentStorageStoreFailure::DeviceTrustUnavailable => {
            HouseholdAuthorityParentStorageConfirmationFailure::DeviceTrustUnavailable
        }
        HouseholdAuthorityParentStorageStoreFailure::DeviceTrustNotCurrent => {
            HouseholdAuthorityParentStorageConfirmationFailure::DeviceTrustNotCurrent
        }
    }
}
