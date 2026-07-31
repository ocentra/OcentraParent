use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityInput, HouseholdAuthorizationState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityProof {
    authority: HouseholdAuthorityInput,
    identity_binding: Option<HouseholdAuthorityProofIdentityBinding>,
    signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityProofIdentityBinding {
    pub household_id: String,
    pub parent_actor_id: String,
    pub parent_device_id: String,
    pub child_profile_id: String,
    pub target_device_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedHouseholdAuthority {
    authority: HouseholdAuthorityInput,
    identity_binding: Option<HouseholdAuthorityProofIdentityBinding>,
}

impl VerifiedHouseholdAuthority {
    pub fn input(&self) -> HouseholdAuthorityInput {
        self.authority
    }

    pub fn identity_binding(&self) -> Option<&HouseholdAuthorityProofIdentityBinding> {
        self.identity_binding.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HouseholdAuthorityProofError {
    Rejected,
}

pub struct HouseholdAuthorityProofSigner {
    signing_key: SigningKey,
}

impl HouseholdAuthorityProofSigner {
    pub fn from_platform_key(platform_protected_key: [u8; 32]) -> Self {
        Self {
            signing_key: SigningKey::from_bytes(&platform_protected_key),
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn sign(
        &self,
        authority: HouseholdAuthorityInput,
    ) -> Result<HouseholdAuthorityProof, HouseholdAuthorityProofError> {
        (authorize_household_action(authority).authorization_state
            == HouseholdAuthorizationState::Authorized)
            .then_some(())
            .ok_or(HouseholdAuthorityProofError::Rejected)?;
        let bytes = signing_bytes(authority, None)?;
        Ok(HouseholdAuthorityProof {
            authority,
            identity_binding: None,
            signature: self.signing_key.sign(&bytes).to_bytes().to_vec(),
        })
    }

    pub fn sign_bound(
        &self,
        authority: HouseholdAuthorityInput,
        identity_binding: HouseholdAuthorityProofIdentityBinding,
    ) -> Result<HouseholdAuthorityProof, HouseholdAuthorityProofError> {
        (authorize_household_action(authority).authorization_state
            == HouseholdAuthorizationState::Authorized)
            .then_some(())
            .ok_or(HouseholdAuthorityProofError::Rejected)?;
        let bytes = signing_bytes(authority, Some(&identity_binding))?;
        Ok(HouseholdAuthorityProof {
            authority,
            identity_binding: Some(identity_binding),
            signature: self.signing_key.sign(&bytes).to_bytes().to_vec(),
        })
    }
}

pub struct HouseholdAuthorityProofVerifier {
    verifying_key: VerifyingKey,
}

impl HouseholdAuthorityProofVerifier {
    pub fn new(verifying_key: VerifyingKey) -> Self {
        Self { verifying_key }
    }

    pub fn verify(
        &self,
        proof: &HouseholdAuthorityProof,
    ) -> Result<VerifiedHouseholdAuthority, HouseholdAuthorityProofError> {
        let signature = Signature::from_slice(&proof.signature)
            .map_err(|_error| HouseholdAuthorityProofError::Rejected)?;
        let bytes = signing_bytes(proof.authority, proof.identity_binding.as_ref())?;
        self.verifying_key
            .verify_strict(&bytes, &signature)
            .map_err(|_error| HouseholdAuthorityProofError::Rejected)?;
        (authorize_household_action(proof.authority).authorization_state
            == HouseholdAuthorizationState::Authorized)
            .then_some(VerifiedHouseholdAuthority {
                authority: proof.authority,
                identity_binding: proof.identity_binding.clone(),
            })
            .ok_or(HouseholdAuthorityProofError::Rejected)
    }
}

fn signing_bytes(
    authority: HouseholdAuthorityInput,
    identity_binding: Option<&HouseholdAuthorityProofIdentityBinding>,
) -> Result<Vec<u8>, HouseholdAuthorityProofError> {
    serde_json::to_vec(&(authority, identity_binding))
        .map_err(|_error| HouseholdAuthorityProofError::Rejected)
}
