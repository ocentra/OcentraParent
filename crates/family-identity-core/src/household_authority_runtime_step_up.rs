use std::fmt;

use chrono::Utc;

use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, HouseholdAuthorityAction,
    HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

impl fmt::Debug for ConsumedParentStepUp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ConsumedParentStepUp")
            .field("action", &self.action)
            .field("authority_generation", &self.authority_generation)
            .field("receipt_epoch", &self.receipt_epoch)
            .field("sensitive_binding", &"omitted")
            .finish()
    }
}

impl ConsumedParentStepUp {
    pub(super) fn validate_for(
        &self,
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        action: HouseholdAuthorityAction,
    ) -> Result<(), HouseholdAuthorityRuntimeFailure> {
        if self.action != action {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch);
        }
        if self.expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpExpired);
        }
        if self.receipt_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected);
        }
        if self.authority_generation != authority.authority_generation() {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch);
        }
        if !super::household_authority_runtime_binding::matches(
            authority,
            device_binding,
            &super::household_authority_runtime_binding::HouseholdAuthorityRuntimeBinding {
                household_id: &self.household_id,
                account_id: &self.account_id,
                parent_device_id: &self.parent_device_id,
                child_profile_id: &self.child_profile_id,
                child_device_id: &self.child_device_id,
                installation_id: &self.installation_id,
                pairing_id: &self.pairing_id,
                route_id: &self.route_id,
            },
        ) {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch);
        }
        Ok(())
    }

    pub(super) fn same_current(&self, other: &Self) -> bool {
        self.household_id == other.household_id
            && self.account_id == other.account_id
            && self.parent_device_id == other.parent_device_id
            && self.child_profile_id == other.child_profile_id
            && self.child_device_id == other.child_device_id
            && self.installation_id == other.installation_id
            && self.pairing_id == other.pairing_id
            && self.route_id == other.route_id
            && self.action == other.action
            && self.authority_generation == other.authority_generation
            && self.expires_at == other.expires_at
            && self.receipt_epoch == other.receipt_epoch
    }
}
