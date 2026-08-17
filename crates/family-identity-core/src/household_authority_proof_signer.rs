use ed25519_dalek::Signer;

use super::{
    signing_bytes, validate_freshness, validate_proof_shape, validate_unsigned_shape,
    HouseholdAuthorityCurrentState, HouseholdAuthorityInput, HouseholdAuthorityProof,
    HouseholdAuthorityProofError, HouseholdAuthorityProofIdentityBinding,
    HouseholdAuthorityProofSigner, HouseholdAuthorizationState,
};
use crate::household_authority::authorize_household_action;

impl HouseholdAuthorityProofSigner {
    pub fn from_platform_key(platform_protected_key: [u8; 32]) -> Self {
        Self {
            signing_key: ed25519_dalek::SigningKey::from_bytes(&platform_protected_key),
        }
    }

    pub fn verifying_key(&self) -> ed25519_dalek::VerifyingKey {
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
        validate_unsigned_shape(None, None, None)?;
        let bytes = signing_bytes(authority, None, None, None, None)?;
        let proof = HouseholdAuthorityProof {
            authority,
            identity_binding: None,
            issued_at: None,
            expires_at: None,
            family_revocation_epoch: None,
            signature: self.signing_key.sign(&bytes).to_bytes().to_vec(),
        };
        validate_proof_shape(&proof)?;
        Ok(proof)
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
        validate_unsigned_shape(Some(&identity_binding), None, None)?;
        let bytes = signing_bytes(authority, Some(&identity_binding), None, None, None)?;
        let proof = HouseholdAuthorityProof {
            authority,
            identity_binding: Some(identity_binding),
            issued_at: None,
            expires_at: None,
            family_revocation_epoch: None,
            signature: self.signing_key.sign(&bytes).to_bytes().to_vec(),
        };
        validate_proof_shape(&proof)?;
        Ok(proof)
    }

    pub fn sign_bound_at(
        &self,
        state: &HouseholdAuthorityCurrentState,
        identity_binding: HouseholdAuthorityProofIdentityBinding,
        issued_at: impl Into<String>,
        expires_at: impl Into<String>,
    ) -> Result<HouseholdAuthorityProof, HouseholdAuthorityProofError> {
        (authorize_household_action(state.authority).authorization_state
            == HouseholdAuthorizationState::Authorized)
            .then_some(())
            .ok_or(HouseholdAuthorityProofError::Rejected)?;
        (state.identity_binding == identity_binding)
            .then_some(())
            .ok_or(HouseholdAuthorityProofError::Rejected)?;
        let issued_at = issued_at.into();
        let expires_at = expires_at.into();
        validate_unsigned_shape(Some(&identity_binding), Some(&issued_at), Some(&expires_at))?;
        validate_freshness(&issued_at, &expires_at, &issued_at)?;
        let bytes = signing_bytes(
            state.authority,
            Some(&identity_binding),
            Some(&issued_at),
            Some(&expires_at),
            Some(state.family_revocation_epoch),
        )?;
        let proof = HouseholdAuthorityProof {
            authority: state.authority,
            identity_binding: Some(identity_binding),
            issued_at: Some(issued_at),
            expires_at: Some(expires_at),
            family_revocation_epoch: Some(state.family_revocation_epoch),
            signature: self.signing_key.sign(&bytes).to_bytes().to_vec(),
        };
        validate_proof_shape(&proof)?;
        Ok(proof)
    }
}
