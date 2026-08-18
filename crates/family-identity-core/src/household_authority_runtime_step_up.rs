use std::fmt;

use chrono::{DateTime, Utc};

use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, HouseholdAuthorityAction,
    HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::household_authority::requires_parent_step_up;

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
    /// Construct only after the parent step-up owner has verified and consumed its one-time
    /// receipt against the current authority and device binding.
    pub(crate) fn from_owner_consumed(
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        action: HouseholdAuthorityAction,
        expires_at: DateTime<Utc>,
        receipt_epoch: u64,
    ) -> Result<Self, HouseholdAuthorityRuntimeFailure> {
        if !requires_parent_step_up(action) {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch);
        }
        if expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpExpired);
        }
        if receipt_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpReplayRejected);
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
            action,
            authority_generation: authority.authority_generation(),
            expires_at,
            receipt_epoch,
        };
        value.validate_for(authority, device_binding, action)?;
        Ok(value)
    }

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
            &self.household_id,
            &self.account_id,
            &self.parent_device_id,
            &self.child_profile_id,
            &self.child_device_id,
            &self.installation_id,
            &self.pairing_id,
            &self.route_id,
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
