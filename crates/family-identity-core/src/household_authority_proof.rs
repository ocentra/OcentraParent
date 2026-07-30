use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityInput, HouseholdAuthorizationState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityProof {
    authority: HouseholdAuthorityInput,
    signature: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifiedHouseholdAuthority {
    authority: HouseholdAuthorityInput,
}

impl VerifiedHouseholdAuthority {
    pub fn input(&self) -> HouseholdAuthorityInput {
        self.authority
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
        let bytes = signing_bytes(authority)?;
        Ok(HouseholdAuthorityProof {
            authority,
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
        let bytes = signing_bytes(proof.authority)?;
        self.verifying_key
            .verify_strict(&bytes, &signature)
            .map_err(|_error| HouseholdAuthorityProofError::Rejected)?;
        (authorize_household_action(proof.authority).authorization_state
            == HouseholdAuthorizationState::Authorized)
            .then_some(VerifiedHouseholdAuthority {
                authority: proof.authority,
            })
            .ok_or(HouseholdAuthorityProofError::Rejected)
    }
}

fn signing_bytes(
    authority: HouseholdAuthorityInput,
) -> Result<Vec<u8>, HouseholdAuthorityProofError> {
    serde_json::to_vec(&authority).map_err(|_error| HouseholdAuthorityProofError::Rejected)
}
