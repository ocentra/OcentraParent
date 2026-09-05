use std::fmt;

use super::{
    ConsumedParentStepUp, HouseholdAuthorityRuntimeAuthorization,
    HouseholdAuthorityRuntimeConsumedEffect, HouseholdAuthorityRuntimeEffectAuthorization,
    HouseholdAuthorityRuntimeEffectTarget, HouseholdAuthorityRuntimeFailure,
};

impl fmt::Debug for HouseholdAuthorityRuntimeAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("HouseholdAuthorityRuntimeAuthorization")
            .field("action", &self.action)
            .field(
                "account_authority_generation",
                &self.account_authority_generation,
            )
            .field("session_generation", &self.session_generation)
            .field("session_id", &"opaque")
            .field("session_expires_at", &"opaque")
            .field("consumption_nonce", &"opaque")
            .field("capability", &self.capability.is_some())
            .field("controller_lease", &self.controller_lease.is_some())
            .field("parent_step_up", &self.parent_step_up.is_some())
            .field("device_binding", &"opaque")
            .finish()
    }
}

impl HouseholdAuthorityRuntimeAuthorization {
    pub fn action(&self) -> super::HouseholdAuthorityAction {
        self.action
    }

    pub fn account_authority_generation(&self) -> u64 {
        self.account_authority_generation
    }

    pub fn session_generation(&self) -> u64 {
        self.session_generation
    }

    pub(crate) fn parent_step_up(&self) -> Option<&ConsumedParentStepUp> {
        self.parent_step_up.as_ref()
    }

    pub(super) fn consumption_nonce(&self) -> &[u8; 32] {
        &self.consumption_nonce
    }
}

impl HouseholdAuthorityRuntimeAuthorization {}

impl HouseholdAuthorityRuntimeEffectAuthorization {
    /// Consume an already revalidated Account receipt for one exact Data Custody operation.
    ///
    /// Data Custody receives no household, device, session, generation, or revocation fields
    /// from this type. Account retains those fields privately and performs the final target
    /// comparison here before moving the receipt into the terminal consumed effect.
    pub fn consume_for_data_custody(
        self,
        expected_action: super::HouseholdAuthorityAction,
        expected_household_id: &str,
        expected_target_device_id: Option<&str>,
        expected_account_authority_generation: Option<u64>,
    ) -> Result<HouseholdAuthorityRuntimeConsumedEffect, HouseholdAuthorityRuntimeFailure> {
        if self.target.action != expected_action
            || self.target.household_id != expected_household_id
            || expected_target_device_id
                .is_some_and(|expected| self.target.child_device_id != expected)
            || expected_account_authority_generation
                .is_some_and(|expected| self.target.account_authority_generation != expected)
        {
            return Err(HouseholdAuthorityRuntimeFailure::EffectTargetMismatch);
        }
        Ok(HouseholdAuthorityRuntimeConsumedEffect {})
    }

    /// Consume this receipt exactly once with an owner-issued target. Both values move by value,
    /// and every private identity, binding, generation, expiry, and action field must match.
    /// Mismatched targets fail closed; there is no API for pairing arbitrary caller state.
    pub fn consume_for_target(
        self,
        target: HouseholdAuthorityRuntimeEffectTarget,
    ) -> Result<HouseholdAuthorityRuntimeConsumedEffect, HouseholdAuthorityRuntimeFailure> {
        if !self.target.matches(&target) {
            return Err(HouseholdAuthorityRuntimeFailure::EffectTargetMismatch);
        }
        drop(target);
        Ok(HouseholdAuthorityRuntimeConsumedEffect {})
    }
}

impl HouseholdAuthorityRuntimeEffectTarget {
    fn matches(&self, other: &Self) -> bool {
        self.action == other.action
            && self.household_id == other.household_id
            && self.account_id == other.account_id
            && self.parent_device_id == other.parent_device_id
            && self.child_profile_id == other.child_profile_id
            && self.child_device_id == other.child_device_id
            && self.provider == other.provider
            && self.provider_subject == other.provider_subject
            && self.session_id == other.session_id
            && self.session_expires_at == other.session_expires_at
            && self.session_generation == other.session_generation
            && self.account_authority_generation == other.account_authority_generation
            && self.account_binding_authority_generation
                == other.account_binding_authority_generation
            && self.installation_id == other.installation_id
            && self.pairing_id == other.pairing_id
            && self.route_id == other.route_id
            && self.device_trust_subject == other.device_trust_subject
            && self.device_signer_key_id == other.device_signer_key_id
            && self.device_signer_key_sha256 == other.device_signer_key_sha256
            && self.device_state == other.device_state
            && self.device_lifecycle_generation == other.device_lifecycle_generation
            && self.device_installation_binding_generation
                == other.device_installation_binding_generation
            && self.device_authority_generation == other.device_authority_generation
            && self.capability_authority_generation == other.capability_authority_generation
            && self.capability_expires_at == other.capability_expires_at
            && self.capability_revocation_epoch == other.capability_revocation_epoch
            && self.controller_lease_authority_generation
                == other.controller_lease_authority_generation
            && self.controller_lease_expires_at == other.controller_lease_expires_at
            && self.controller_lease_revocation_epoch == other.controller_lease_revocation_epoch
            && self.parent_step_up_authority_generation == other.parent_step_up_authority_generation
            && self.parent_step_up_expires_at == other.parent_step_up_expires_at
            && self.parent_step_up_receipt_epoch == other.parent_step_up_receipt_epoch
    }
}
