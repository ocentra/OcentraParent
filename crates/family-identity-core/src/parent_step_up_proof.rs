use chrono::{DateTime, TimeDelta};
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::family_identity::DeviceTrustState;
use crate::household_authority::ParentStepUpValidationInput;

const MAX_PARENT_STEP_UP_PROOF_LIFETIME_SECONDS: i64 = 5 * 60;
const AUTHORIZATION_DIGEST_PREFIX: &str = "sha256:";
const AUTHORIZATION_DIGEST_HEX_BYTES: usize = 64;
const AUTHORIZATION_DIGEST_BYTES: usize =
    AUTHORIZATION_DIGEST_PREFIX.len() + AUTHORIZATION_DIGEST_HEX_BYTES;
const UNBOUND_PARENT_STEP_UP_PROOF_DOMAIN: &[u8] = b"ocentra-parent-step-up-proof:unbound";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentStepUpProofError {
    Rejected,
}

impl std::fmt::Display for ParentStepUpProofError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("parent step-up proof rejected")
    }
}

impl std::error::Error for ParentStepUpProofError {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedParentStepUpProof {
    pub validation: ParentStepUpValidationInput,
    pub target_device_id: String,
    pub assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
    pub authorization_digest: String,
    pub parent_device_trust_revocation_epoch: u64,
    pub signature: Vec<u8>,
}

/// Family-identity-owned current trust snapshot for the parent device that
/// performed a high-risk step-up.  The policy consumer compares this at issue
/// time so a proof cannot outlive a device revocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParentDeviceTrustCurrentState {
    pub parent_device_id: String,
    pub trust_state: DeviceTrustState,
    pub revocation_epoch: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct ParentStepUpAuthorizationBinding<'a> {
    pub household_id: &'a str,
    pub parent_actor_id: &'a str,
    pub parent_device_id: &'a str,
    pub child_profile_id: &'a str,
    pub target_device_id: &'a str,
    pub action_id: &'a str,
    pub capability_id: &'a str,
    pub evidence_digest: &'a str,
    pub payload_digest: &'a str,
}

pub fn authorization_digest(binding: ParentStepUpAuthorizationBinding<'_>) -> String {
    let input = [
        binding.household_id,
        binding.parent_actor_id,
        binding.parent_device_id,
        binding.child_profile_id,
        binding.target_device_id,
        binding.action_id,
        binding.capability_id,
        binding.evidence_digest,
        binding.payload_digest,
    ];
    let mut bytes = Vec::new();
    for field in input {
        let field_len = field.len() as u64;
        bytes.extend_from_slice(&field_len.to_be_bytes());
        bytes.extend_from_slice(field.as_bytes());
    }
    format!("{AUTHORIZATION_DIGEST_PREFIX}{:x}", Sha256::digest(bytes))
}

pub struct ParentStepUpProofVerifier {
    verifying_key: VerifyingKey,
}

impl ParentStepUpProofVerifier {
    pub fn new(verifying_key: VerifyingKey) -> Self {
        Self { verifying_key }
    }

    pub fn verify(
        &self,
        proof: &VerifiedParentStepUpProof,
    ) -> Result<
        (
            ParentStepUpValidationInput,
            String,
            AuthenticatedDeliveryGrantAssertionSnapshot,
        ),
        ParentStepUpProofError,
    > {
        validate_proof_shape(proof)?;
        let signature = Signature::from_slice(&proof.signature)
            .map_err(|_error| ParentStepUpProofError::Rejected)?;
        let bytes = signing_bytes(
            &proof.validation,
            &proof.target_device_id,
            &proof.assertions,
            &proof.authorization_digest,
            proof.parent_device_trust_revocation_epoch,
        )?;
        self.verifying_key
            .verify_strict(&bytes, &signature)
            .map_err(|_error| ParentStepUpProofError::Rejected)?;
        Ok((
            proof.validation.clone(),
            proof.target_device_id.clone(),
            proof.assertions.clone(),
        ))
    }

    pub fn verify_against_current_device_trust_state(
        &self,
        proof: &VerifiedParentStepUpProof,
        current_state: &ParentDeviceTrustCurrentState,
    ) -> Result<
        (
            ParentStepUpValidationInput,
            String,
            AuthenticatedDeliveryGrantAssertionSnapshot,
        ),
        ParentStepUpProofError,
    > {
        let verified = self.verify(proof)?;
        (current_state.trust_state == DeviceTrustState::Trusted
            && current_state.parent_device_id == verified.0.action_device_id
            && current_state.revocation_epoch == proof.parent_device_trust_revocation_epoch)
            .then_some(verified)
            .ok_or(ParentStepUpProofError::Rejected)
    }
}

pub struct ParentStepUpProofSigner {
    signing_key: SigningKey,
}

impl ParentStepUpProofSigner {
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
        validation: ParentStepUpValidationInput,
        target_device_id: String,
        assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
    ) -> Result<VerifiedParentStepUpProof, ParentStepUpProofError> {
        validate_unsigned_shape(&validation, &target_device_id)?;
        self.sign_bound(
            validation,
            target_device_id,
            assertions,
            format!(
                "{AUTHORIZATION_DIGEST_PREFIX}{:x}",
                Sha256::digest(UNBOUND_PARENT_STEP_UP_PROOF_DOMAIN)
            ),
        )
    }

    pub fn sign_bound(
        &self,
        validation: ParentStepUpValidationInput,
        target_device_id: String,
        assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
        authorization_digest: String,
    ) -> Result<VerifiedParentStepUpProof, ParentStepUpProofError> {
        self.sign_bound_at_device_trust_revocation_epoch(
            validation,
            target_device_id,
            assertions,
            authorization_digest,
            0,
        )
    }

    pub fn sign_bound_for_current_device_trust_state(
        &self,
        validation: ParentStepUpValidationInput,
        target_device_id: String,
        assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
        authorization_digest: String,
        current_state: &ParentDeviceTrustCurrentState,
    ) -> Result<VerifiedParentStepUpProof, ParentStepUpProofError> {
        (current_state.trust_state == DeviceTrustState::Trusted
            && current_state.parent_device_id == validation.action_device_id)
            .then_some(())
            .ok_or(ParentStepUpProofError::Rejected)?;
        self.sign_bound_at_device_trust_revocation_epoch(
            validation,
            target_device_id,
            assertions,
            authorization_digest,
            current_state.revocation_epoch,
        )
    }

    fn sign_bound_at_device_trust_revocation_epoch(
        &self,
        validation: ParentStepUpValidationInput,
        target_device_id: String,
        assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
        authorization_digest: String,
        parent_device_trust_revocation_epoch: u64,
    ) -> Result<VerifiedParentStepUpProof, ParentStepUpProofError> {
        validate_unsigned_shape(&validation, &target_device_id)?;
        validate_authorization_digest(&authorization_digest)?;
        let bytes = signing_bytes(
            &validation,
            &target_device_id,
            &assertions,
            &authorization_digest,
            parent_device_trust_revocation_epoch,
        )?;
        let signature = self.signing_key.sign(&bytes).to_bytes().to_vec();
        Ok(VerifiedParentStepUpProof {
            validation,
            target_device_id,
            assertions,
            authorization_digest,
            parent_device_trust_revocation_epoch,
            signature,
        })
    }
}

fn signing_bytes(
    validation: &ParentStepUpValidationInput,
    target_device_id: &str,
    assertions: &AuthenticatedDeliveryGrantAssertionSnapshot,
    authorization_digest: &str,
    parent_device_trust_revocation_epoch: u64,
) -> Result<Vec<u8>, ParentStepUpProofError> {
    let bytes = serde_json::to_vec(&(
        validation,
        target_device_id,
        assertions,
        authorization_digest,
        parent_device_trust_revocation_epoch,
    ))
    .map_err(|_error| ParentStepUpProofError::Rejected)?;
    (bytes.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES)
        .then_some(bytes)
        .ok_or(ParentStepUpProofError::Rejected)
}

fn validate_proof_shape(proof: &VerifiedParentStepUpProof) -> Result<(), ParentStepUpProofError> {
    (proof.signature.len() == AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES)
        .then_some(())
        .ok_or(ParentStepUpProofError::Rejected)?;
    validate_unsigned_shape(&proof.validation, &proof.target_device_id)?;
    validate_authorization_digest(&proof.authorization_digest)
}

fn validate_authorization_digest(authorization_digest: &str) -> Result<(), ParentStepUpProofError> {
    let digest = authorization_digest
        .strip_prefix(AUTHORIZATION_DIGEST_PREFIX)
        .ok_or(ParentStepUpProofError::Rejected)?;
    (authorization_digest.len() == AUTHORIZATION_DIGEST_BYTES
        && authorization_digest.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)))
    .then_some(())
    .ok_or(ParentStepUpProofError::Rejected)
}

fn validate_unsigned_shape(
    validation: &ParentStepUpValidationInput,
    target_device_id: &str,
) -> Result<(), ParentStepUpProofError> {
    let assertion = validation
        .assertion
        .as_ref()
        .ok_or(ParentStepUpProofError::Rejected)?;
    let fields = [
        validation.family_id.as_str(),
        validation.parent_account_id.as_str(),
        validation.action_device_id.as_str(),
        validation
            .action_device_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        validation
            .target_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        validation.observed_at.as_str(),
        validation.expected_nonce.as_deref().unwrap_or_default(),
        assertion.family_id.as_str(),
        assertion.parent_account_id.as_str(),
        assertion.action_device_id.as_str(),
        assertion
            .action_device_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        assertion
            .target_child_profile_id
            .as_deref()
            .unwrap_or_default(),
        assertion.nonce.as_str(),
        assertion.expires_at.as_str(),
        target_device_id,
    ];
    let bounded = fields
        .iter()
        .all(|field| field.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES);
    let wire_len = fields.iter().map(|field| field.len()).sum::<usize>();
    let lifetime_is_bounded = parent_step_up_lifetime_is_bounded(validation);
    (!target_device_id.trim().is_empty()
        && bounded
        && lifetime_is_bounded
        && wire_len <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES)
        .then_some(())
        .ok_or(ParentStepUpProofError::Rejected)
}

fn parent_step_up_lifetime_is_bounded(validation: &ParentStepUpValidationInput) -> bool {
    let Some(assertion) = validation.assertion.as_ref() else {
        return false;
    };
    let Ok(observed_at) = DateTime::parse_from_rfc3339(&validation.observed_at) else {
        return false;
    };
    let Ok(expires_at) = DateTime::parse_from_rfc3339(&assertion.expires_at) else {
        return false;
    };
    let lifetime = expires_at - observed_at;
    lifetime >= TimeDelta::zero()
        && lifetime <= TimeDelta::seconds(MAX_PARENT_STEP_UP_PROOF_LIFETIME_SECONDS)
}
