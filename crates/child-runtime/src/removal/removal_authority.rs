use std::io;

use ocentra_family_identity_core::{
    device_trust_current_binding::CurrentChildDeviceTrustBinding,
    household_authority::HouseholdAuthorityAction,
    household_authority_proof::CurrentVerifiedHouseholdAuthority,
};

use super::{
    removal_validation::non_empty_ref, ChildAgentRemovalAuthorizationAction,
    ChildAgentServiceIdentity, VerifiedParentRemovalAuthorization,
};

impl ChildAgentServiceIdentity {
    pub(crate) fn from_trust_binding(binding: &CurrentChildDeviceTrustBinding) -> io::Result<Self> {
        Ok(Self {
            household_id: non_empty_ref(binding.family_id())?,
            child_profile_id: non_empty_ref(binding.child_device_id())?,
            target_device_id: non_empty_ref(binding.installation_id())?,
        })
    }
}

impl VerifiedParentRemovalAuthorization {
    pub fn for_revocation(
        authority: CurrentVerifiedHouseholdAuthority,
        reference: impl Into<String>,
    ) -> io::Result<Self> {
        Self::from_verified_authority(
            authority,
            reference,
            HouseholdAuthorityAction::RevokeChildDevice,
            ChildAgentRemovalAuthorizationAction::Revoke,
        )
    }

    pub fn for_reauthorization(
        authority: CurrentVerifiedHouseholdAuthority,
        reference: impl Into<String>,
    ) -> io::Result<Self> {
        Self::from_verified_authority(
            authority,
            reference,
            HouseholdAuthorityAction::PairChildDevice,
            ChildAgentRemovalAuthorizationAction::Reauthorize,
        )
    }

    fn from_verified_authority(
        authority: CurrentVerifiedHouseholdAuthority,
        reference: impl Into<String>,
        required_action: HouseholdAuthorityAction,
        action: ChildAgentRemovalAuthorizationAction,
    ) -> io::Result<Self> {
        if authority.input().action != required_action {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "current household authority is not scoped to child removal or reauthorization",
            ));
        }
        let binding = authority.identity_binding();
        let identity = ChildAgentServiceIdentity {
            household_id: non_empty_ref(binding.household_id())?,
            child_profile_id: non_empty_ref(binding.child_profile_id())?,
            target_device_id: non_empty_ref(binding.target_device_id())?,
        };
        let reference = non_empty_ref(&reference.into())?;
        let authority_nonce = authority.proof_nonce().to_owned();
        let authority_generation = authority.family_revocation_epoch();
        drop(authority);
        Ok(Self {
            reference,
            action,
            identity,
            authority_nonce,
            authority_generation,
        })
    }

    pub fn action(&self) -> ChildAgentRemovalAuthorizationAction {
        self.action
    }

    pub(super) fn identity(&self) -> &ChildAgentServiceIdentity {
        &self.identity
    }

    pub(super) fn into_audit_parts(self) -> (String, ChildAgentServiceIdentity) {
        (self.reference, self.identity)
    }
}
