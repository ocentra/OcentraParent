use std::fmt;

use chrono::{DateTime, Utc};

use super::{
    CurrentChildDeviceTrustBinding, CurrentHouseholdControllerLease, HouseholdAuthorityAction,
    HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

impl fmt::Debug for CurrentHouseholdControllerLease {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("CurrentHouseholdControllerLease")
            .field("action", &self.action)
            .field("authority_generation", &self.authority_generation)
            .field("revocation_epoch", &self.revocation_epoch)
            .field("sensitive_binding", &"omitted")
            .finish()
    }
}

impl CurrentHouseholdControllerLease {
    /// Construct only after the lease owner has resolved an active, unrevoked lease.
    pub(crate) fn from_owner_current(
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        action: HouseholdAuthorityAction,
        expires_at: DateTime<Utc>,
        revocation_epoch: u64,
    ) -> Result<Self, HouseholdAuthorityRuntimeFailure> {
        if !super::household_authority_runtime_requirements::requires_controller_lease(action) {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch);
        }
        if expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseExpired);
        }
        if revocation_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseRevoked);
        }
        let value = Self {
            household_id: authority.household_id().to_string(),
            account_id: authority.account_id().to_string(),
            parent_device_id: authority.device_id().as_str().to_owned(),
            child_profile_id: authority.child_profile_id().to_string(),
            child_device_id: authority.child_device_id().as_str().to_owned(),
            action,
            authority_generation: authority.authority_generation(),
            expires_at,
            revocation_epoch,
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
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch);
        }
        if self.expires_at <= Utc::now() {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseExpired);
        }
        if self.revocation_epoch == 0 {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseRevoked);
        }
        if self.authority_generation != authority.authority_generation() {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch);
        }
        if !super::household_authority_runtime_binding::matches(
            authority,
            device_binding,
            &self.household_id,
            &self.account_id,
            &self.parent_device_id,
            &self.child_profile_id,
            &self.child_device_id,
        ) {
            return Err(HouseholdAuthorityRuntimeFailure::ControllerLeaseBindingMismatch);
        }
        Ok(())
    }
}
