use serde::{Deserialize, Serialize};

use crate::household_authority::HouseholdAuthorityInput;

/// Transport-only proof material. The production family owner does not yet
/// expose an issuer or verifier, so this value cannot be promoted to current
/// authority by any shipped caller.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityProof {
    authority: HouseholdAuthorityInput,
    identity_binding: Option<HouseholdAuthorityProofIdentityBinding>,
    issued_at: Option<String>,
    expires_at: Option<String>,
    family_revocation_epoch: Option<u64>,
    signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityProofIdentityBinding {
    household_id: String,
    parent_actor_id: String,
    parent_device_id: String,
    child_profile_id: String,
    target_device_id: String,
}

impl HouseholdAuthorityProofIdentityBinding {
    pub fn household_id(&self) -> &str {
        &self.household_id
    }

    pub fn parent_actor_id(&self) -> &str {
        &self.parent_actor_id
    }

    pub fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub fn child_profile_id(&self) -> &str {
        &self.child_profile_id
    }

    pub fn target_device_id(&self) -> &str {
        &self.target_device_id
    }
}

/// A current-state household authority consumed by one privileged transition.
///
/// This type intentionally has no public constructor, serializer, clone, or
/// verifier path. It remains a consumer-facing boundary for the future
/// family-owned ceremony/provider, but no current production caller can mint
/// one while that provider is absent.
#[derive(Debug, PartialEq, Eq)]
pub struct CurrentVerifiedHouseholdAuthority {
    authority: HouseholdAuthorityInput,
    identity_binding: HouseholdAuthorityProofIdentityBinding,
    issued_at: String,
    expires_at: String,
    family_revocation_epoch: u64,
    proof_nonce: String,
}

impl CurrentVerifiedHouseholdAuthority {
    pub fn input(&self) -> HouseholdAuthorityInput {
        self.authority
    }

    pub fn identity_binding(&self) -> &HouseholdAuthorityProofIdentityBinding {
        &self.identity_binding
    }

    pub fn issued_at(&self) -> &str {
        &self.issued_at
    }

    pub fn expires_at(&self) -> &str {
        &self.expires_at
    }

    pub fn family_revocation_epoch(&self) -> u64 {
        self.family_revocation_epoch
    }

    pub fn proof_nonce(&self) -> &str {
        &self.proof_nonce
    }
}
