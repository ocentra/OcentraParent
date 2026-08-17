use ed25519_dalek::Signature;
use sha2::{Digest, Sha256};

use super::{
    signing_bytes, validate_freshness, validate_proof_shape, CurrentVerifiedHouseholdAuthority,
    HouseholdAuthorityCurrentState, HouseholdAuthorityProof, HouseholdAuthorityProofError,
    HouseholdAuthorityProofVerifier, HouseholdAuthorizationState, VerifiedHouseholdAuthority,
};
use crate::household_authority::authorize_household_action;

impl HouseholdAuthorityProofVerifier {
    pub fn new(verifying_key: ed25519_dalek::VerifyingKey) -> Self {
        Self { verifying_key }
    }

    pub fn verify(
        &self,
        proof: &HouseholdAuthorityProof,
    ) -> Result<VerifiedHouseholdAuthority, HouseholdAuthorityProofError> {
        validate_proof_shape(proof)?;
        let signature = Signature::from_slice(&proof.signature)
            .map_err(|_error| HouseholdAuthorityProofError::Rejected)?;
        let bytes = signing_bytes(
            proof.authority,
            proof.identity_binding.as_ref(),
            proof.issued_at.as_deref(),
            proof.expires_at.as_deref(),
            proof.family_revocation_epoch,
        )?;
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

    pub fn verify_against_current_state(
        &self,
        proof: &HouseholdAuthorityProof,
        current_state: &HouseholdAuthorityCurrentState,
        trusted_now: &str,
    ) -> Result<CurrentVerifiedHouseholdAuthority, HouseholdAuthorityProofError> {
        let verified = self.verify(proof)?;
        let (Some(issued_at), Some(expires_at), Some(revocation_epoch)) = (
            proof.issued_at.as_deref(),
            proof.expires_at.as_deref(),
            proof.family_revocation_epoch,
        ) else {
            return Err(HouseholdAuthorityProofError::Rejected);
        };
        validate_freshness(issued_at, expires_at, trusted_now)?;
        let Some(identity_binding) = verified.identity_binding else {
            return Err(HouseholdAuthorityProofError::Rejected);
        };
        (verified.authority == current_state.authority
            && identity_binding == current_state.identity_binding
            && revocation_epoch == current_state.family_revocation_epoch
            && revocation_epoch > 0)
            .then_some(CurrentVerifiedHouseholdAuthority {
                authority: verified.authority,
                identity_binding,
                issued_at: issued_at.to_owned(),
                expires_at: expires_at.to_owned(),
                family_revocation_epoch: revocation_epoch,
                proof_nonce: proof_nonce(&proof.signature),
            })
            .ok_or(HouseholdAuthorityProofError::Rejected)
    }
}

fn proof_nonce(signature: &[u8]) -> String {
    format!("{:x}", Sha256::digest(signature))
}
