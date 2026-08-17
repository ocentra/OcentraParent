use chrono::{DateTime, FixedOffset, Utc};
use ed25519_dalek::{SigningKey, VerifyingKey};
use ocentra_schema::authenticated_delivery_grant::{
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};
use serde::{Deserialize, Serialize};

use crate::household_authority::HouseholdAuthorityInput;

mod household_authority_proof_signer;
mod household_authority_proof_verifier;

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

/// A current-state household authority that is consumed by one privileged
/// transition.  Unlike `VerifiedHouseholdAuthority`, this value can only be
/// produced by `verify_against_current_state`; it retains freshness,
/// revocation-epoch, and proof-nonce bindings and is intentionally not
/// cloneable or serializable.
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

pub struct HouseholdAuthorityProofVerifier {
    verifying_key: VerifyingKey,
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

fn validate_proof_shape(
    proof: &HouseholdAuthorityProof,
) -> Result<(), HouseholdAuthorityProofError> {
    (proof.signature.len() == AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES)
        .then_some(())
        .ok_or(HouseholdAuthorityProofError::Rejected)?;
    validate_unsigned_shape(
        proof.identity_binding.as_ref(),
        proof.issued_at.as_deref(),
        proof.expires_at.as_deref(),
    )
}

fn validate_unsigned_shape(
    identity_binding: Option<&HouseholdAuthorityProofIdentityBinding>,
    issued_at: Option<&str>,
    expires_at: Option<&str>,
) -> Result<(), HouseholdAuthorityProofError> {
    let mut fields = Vec::with_capacity(7);
    if let Some(identity) = identity_binding {
        fields.extend([
            identity.household_id.as_str(),
            identity.parent_actor_id.as_str(),
            identity.parent_device_id.as_str(),
            identity.child_profile_id.as_str(),
            identity.target_device_id.as_str(),
        ]);
    }
    if let Some(issued_at) = issued_at {
        fields.push(issued_at);
    }
    if let Some(expires_at) = expires_at {
        fields.push(expires_at);
    }
    (fields.iter().all(|field| {
        !field.trim().is_empty() && field.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
    }) && fields.iter().map(|field| field.len()).sum::<usize>() + 512
        <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES)
        .then_some(())
        .ok_or(HouseholdAuthorityProofError::Rejected)
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
