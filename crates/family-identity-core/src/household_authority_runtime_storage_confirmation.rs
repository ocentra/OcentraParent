use chrono::{DateTime, Utc};

use super::{
    ConsumedParentStorageConfirmation, CurrentChildDeviceTrustBinding,
    HouseholdAuthorityRuntimeFailure, VerifiedAccountIdentityAuthority,
};
use ocentra_schema::parent_storage_settings_apply_flow::{
    ParentStorageApplyIntentDigest, ParentStorageHouseholdRef, ParentStoragePreviewId,
};

impl ConsumedParentStorageConfirmation {
    /// Construct only after the Account/family owner has durably consumed its one-time
    /// confirmation record for this exact preview and digest.
    pub(crate) fn from_owner_consumed(
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        preview_id: ParentStoragePreviewId,
        apply_intent_digest: ParentStorageApplyIntentDigest,
        expires_at: DateTime<Utc>,
        receipt_epoch: u64,
    ) -> Result<Self, HouseholdAuthorityRuntimeFailure> {
        if expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationExpired);
        }
        if receipt_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationReplayRejected);
        }

        let value = Self {
            household_id: authority.household_id().to_string(),
            account_id: authority.account_id().to_string(),
            parent_device_id: authority.device_id().as_str().to_owned(),
            child_profile_id: authority.child_profile_id().to_string(),
            child_device_id: authority.child_device_id().as_str().to_owned(),
            installation_id: authority
                .current_binding()
                .installation_id
                .as_str()
                .to_owned(),
            pairing_id: authority.current_binding().pairing_id.as_str().to_owned(),
            route_id: authority
                .current_binding()
                .selected_route_id
                .as_str()
                .to_owned(),
            authority_generation: authority.authority_generation(),
            preview_id,
            apply_intent_digest,
            expires_at,
            receipt_epoch,
        };
        value.validate_for(authority, device_binding)?;
        Ok(value)
    }

    fn validate_for(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        if self.expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationExpired);
        }
        if self.receipt_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationReplayRejected);
        }
        if self.authority_generation != authority.authority_generation()
            || !super::household_authority_runtime_binding::matches(
                authority,
                device_binding,
                &self.household_id,
                &self.account_id,
                &self.parent_device_id,
                &self.child_profile_id,
                &self.child_device_id,
                &self.installation_id,
                &self.pairing_id,
                &self.route_id,
            )
        {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationBindingMismatch);
        }
        Ok(())
    }

    /// Consume the opaque handoff for the exact preview and canonical digest. Moving the value
    /// prevents a caller from reusing it; the owner-issued receipt epoch prevents durable replay.
    pub fn consume_for_storage(
        self,
        preview_id: &ParentStoragePreviewId,
        household_ref: &ParentStorageHouseholdRef,
        apply_intent_digest: &ParentStorageApplyIntentDigest,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        if self.expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationExpired);
        }
        if self.receipt_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationReplayRejected);
        }
        if self.household_id != household_ref.as_str()
            || &self.preview_id != preview_id
            || &self.apply_intent_digest != apply_intent_digest
        {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStorageConfirmationBindingMismatch);
        }
        Ok(())
    }
}
