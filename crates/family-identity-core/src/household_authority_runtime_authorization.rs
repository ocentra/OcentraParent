use std::fmt;

use super::{
    ConsumedParentStepUp, CurrentChildDeviceTrustBinding, CurrentHouseholdCapability,
    CurrentHouseholdControllerLease, HouseholdAuthorityRuntimeAuthorization,
    HouseholdAuthorityRuntimeConsumedEffect, HouseholdAuthorityRuntimeEffectAuthorization,
    HouseholdAuthorityRuntimeEffectTarget, HouseholdAuthorityRuntimeFailure,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

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

    pub(crate) fn device_binding(&self) -> &CurrentChildDeviceTrustBinding {
        &self.device_binding
    }

    pub(crate) fn capability(&self) -> Option<&CurrentHouseholdCapability> {
        self.capability.as_ref()
    }

    pub(crate) fn controller_lease(&self) -> Option<&CurrentHouseholdControllerLease> {
        self.controller_lease.as_ref()
    }

    pub(crate) fn parent_step_up(&self) -> Option<&ConsumedParentStepUp> {
        self.parent_step_up.as_ref()
    }

    pub(super) fn consumption_nonce(&self) -> &[u8; 32] {
        &self.consumption_nonce
    }
}

impl HouseholdAuthorityRuntimeAuthorization {
    /// Issue the final effect receipt only from the owner-side current snapshots already
    /// revalidated immediately before the CAS fence. The private authorization nonce is carried
    /// into the receipt; the caller-provided nonce argument on the trait is intentionally not a
    /// source of receipt authority.
    pub(super) fn issue_effect_receipt(
        self,
        current_account: VerifiedAccountIdentityAuthority,
        current_device: CurrentChildDeviceTrustBinding,
        current_capability: Option<CurrentHouseholdCapability>,
        current_controller_lease: Option<CurrentHouseholdControllerLease>,
        current_parent_step_up: Option<ConsumedParentStepUp>,
    ) -> Result<HouseholdAuthorityRuntimeEffectAuthorization, HouseholdAuthorityRuntimeFailure>
    {
        self.validate_current_account(&current_account)?;
        self.validate_current_device(&current_device)?;
        self.validate_current_capability(current_capability.as_ref())?;
        self.validate_current_controller_lease(current_controller_lease.as_ref())?;
        match (
            self.parent_step_up.as_ref(),
            current_parent_step_up.as_ref(),
        ) {
            (None, None) => {}
            (Some(expected), Some(current)) if expected.same_current(current) => {}
            _ => return Err(HouseholdAuthorityRuntimeFailure::ParentStepUpBindingMismatch),
        }

        let target = HouseholdAuthorityRuntimeEffectTarget::from_owner_current(
            self.action,
            &current_account,
            &current_device,
            current_capability.as_ref(),
            current_controller_lease.as_ref(),
            current_parent_step_up.as_ref(),
        );
        Ok(HouseholdAuthorityRuntimeEffectAuthorization {
            target,
            consumption_nonce: self.consumption_nonce,
        })
    }
}

impl HouseholdAuthorityRuntimeEffectAuthorization {
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
        Ok(HouseholdAuthorityRuntimeConsumedEffect {
            target,
            consumption_nonce: self.consumption_nonce,
        })
    }
}

impl HouseholdAuthorityRuntimeEffectTarget {
    pub(super) fn from_owner_current(
        action: super::HouseholdAuthorityAction,
        authority: &VerifiedAccountIdentityAuthority,
        device_binding: &CurrentChildDeviceTrustBinding,
        capability: Option<&CurrentHouseholdCapability>,
        controller_lease: Option<&CurrentHouseholdControllerLease>,
        parent_step_up: Option<&ConsumedParentStepUp>,
    ) -> Self {
        let account_binding = authority.current_binding();
        Self {
            action,
            household_id: authority.household_id().to_string(),
            account_id: authority.account_id().to_string(),
            parent_device_id: authority.device_id().as_str().to_owned(),
            child_profile_id: authority.child_profile_id().to_string(),
            child_device_id: authority.child_device_id().as_str().to_owned(),
            provider: authority.provider().clone(),
            provider_subject: authority.provider_subject().as_str().to_owned(),
            session_id: authority.session_id().as_str().to_owned(),
            session_expires_at: authority.session_expires_at().to_owned(),
            session_generation: authority.session_generation(),
            account_authority_generation: authority.authority_generation(),
            account_binding_authority_generation: account_binding.authority_generation,
            installation_id: account_binding.installation_id.as_str().to_owned(),
            pairing_id: account_binding.pairing_id.as_str().to_owned(),
            route_id: account_binding.selected_route_id.as_str().to_owned(),
            device_trust_subject: device_binding.trust_subject().to_owned(),
            device_signer_key_id: device_binding.signer_key_id().to_owned(),
            device_signer_key_sha256: device_binding.signer_key_sha256().to_owned(),
            device_state: device_binding.state(),
            device_lifecycle_generation: device_binding.lifecycle_generation(),
            device_installation_binding_generation: device_binding
                .installation_binding_generation(),
            device_authority_generation: device_binding.authority_generation(),
            capability_authority_generation: capability.map(|value| value.authority_generation),
            capability_expires_at: capability.map(|value| value.expires_at.clone()),
            capability_revocation_epoch: capability.map(|value| value.revocation_epoch),
            controller_lease_authority_generation: controller_lease
                .map(|value| value.authority_generation),
            controller_lease_expires_at: controller_lease.map(|value| value.expires_at.clone()),
            controller_lease_revocation_epoch: controller_lease.map(|value| value.revocation_epoch),
            parent_step_up_authority_generation: parent_step_up
                .map(|value| value.authority_generation),
            parent_step_up_expires_at: parent_step_up.map(|value| value.expires_at.clone()),
            parent_step_up_receipt_epoch: parent_step_up.map(|value| value.receipt_epoch),
        }
    }

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
