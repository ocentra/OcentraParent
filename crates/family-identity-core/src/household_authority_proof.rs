use chrono::{DateTime, FixedOffset, Utc};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use crate::household_authority::{
    authorize_household_action, HouseholdAuthorityInput, HouseholdAuthorizationState,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAuthorityProof {
    authority: HouseholdAuthorityInput,
    identity_binding: Option<HouseholdAuthorityProofIdentityBinding>,
    issued_at: Option<String>,
    expires_at: Option<String>,
    family_revocation_epoch: Option<u64>,
    signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HouseholdAuthorityCurrentState {
    pub authority: HouseholdAuthorityInput,
    /// The household and actor/device/child identity that owns this authority
    /// state.  Authority flags alone are not sufficient to authorize a
    /// different household with identical values.
    pub identity_binding: HouseholdAuthorityProofIdentityBinding,
    pub family_revocation_epoch: u64,
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
        let bytes = signing_bytes(authority, None, None, None, None)?;
        Ok(HouseholdAuthorityProof {
            authority,
            identity_binding: None,
            issued_at: None,
            expires_at: None,
            family_revocation_epoch: None,
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
        let bytes = signing_bytes(authority, Some(&identity_binding), None, None, None)?;
        Ok(HouseholdAuthorityProof {
            authority,
            identity_binding: Some(identity_binding),
            issued_at: None,
            expires_at: None,
            family_revocation_epoch: None,
            signature: self.signing_key.sign(&bytes).to_bytes().to_vec(),
        })
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
        validate_freshness(&issued_at, &expires_at, &issued_at)?;
        let bytes = signing_bytes(
            state.authority,
            Some(&identity_binding),
            Some(&issued_at),
            Some(&expires_at),
            Some(state.family_revocation_epoch),
        )?;
        Ok(HouseholdAuthorityProof {
            authority: state.authority,
            identity_binding: Some(identity_binding),
            issued_at: Some(issued_at),
            expires_at: Some(expires_at),
            family_revocation_epoch: Some(state.family_revocation_epoch),
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
    ) -> Result<VerifiedHouseholdAuthority, HouseholdAuthorityProofError> {
        let verified = self.verify(proof)?;
        let (Some(issued_at), Some(expires_at), Some(revocation_epoch)) = (
            proof.issued_at.as_deref(),
            proof.expires_at.as_deref(),
            proof.family_revocation_epoch,
        ) else {
            return Err(HouseholdAuthorityProofError::Rejected);
        };
        validate_freshness(issued_at, expires_at, trusted_now)?;
        (verified.authority == current_state.authority
            && verified.identity_binding.as_ref() == Some(&current_state.identity_binding)
            && revocation_epoch == current_state.family_revocation_epoch)
            .then_some(verified)
            .ok_or(HouseholdAuthorityProofError::Rejected)
    }
}

fn signing_bytes(
    authority: HouseholdAuthorityInput,
    identity_binding: Option<&HouseholdAuthorityProofIdentityBinding>,
    issued_at: Option<&str>,
    expires_at: Option<&str>,
    family_revocation_epoch: Option<u64>,
) -> Result<Vec<u8>, HouseholdAuthorityProofError> {
    serde_json::to_vec(&(
        authority,
        identity_binding,
        issued_at,
        expires_at,
        family_revocation_epoch,
    ))
    .map_err(|_error| HouseholdAuthorityProofError::Rejected)
}

fn validate_freshness(
    issued_at: &str,
    expires_at: &str,
    trusted_now: &str,
) -> Result<(), HouseholdAuthorityProofError> {
    let (Some(issued_at), Some(expires_at), Some(trusted_now)) = (
        parse_utc(issued_at),
        parse_utc(expires_at),
        parse_utc(trusted_now),
    ) else {
        return Err(HouseholdAuthorityProofError::Rejected);
    };
    (issued_at <= trusted_now
        && trusted_now < expires_at
        && expires_at - issued_at <= chrono::Duration::minutes(5))
    .then_some(())
    .ok_or(HouseholdAuthorityProofError::Rejected)
}

fn parse_utc(value: &str) -> Option<DateTime<Utc>> {
    DateTime::<FixedOffset>::parse_from_rfc3339(value)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}
