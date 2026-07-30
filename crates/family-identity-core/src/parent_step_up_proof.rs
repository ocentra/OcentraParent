use chrono::DateTime;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};
use serde::{Deserialize, Serialize};

use crate::household_authority::ParentStepUpValidationInput;

const MAX_PARENT_STEP_UP_PROOF_LIFETIME_SECONDS: i64 = 5 * 60;

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
    pub signature: Vec<u8>,
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
        let bytes = signing_bytes(&validation, &target_device_id, &assertions)?;
        let signature = self.signing_key.sign(&bytes).to_bytes().to_vec();
        Ok(VerifiedParentStepUpProof {
            validation,
            target_device_id,
            assertions,
            signature,
        })
    }
}

fn signing_bytes(
    validation: &ParentStepUpValidationInput,
    target_device_id: &str,
    assertions: &AuthenticatedDeliveryGrantAssertionSnapshot,
) -> Result<Vec<u8>, ParentStepUpProofError> {
    serde_json::to_vec(&(validation, target_device_id, assertions))
        .map_err(|_error| ParentStepUpProofError::Rejected)
}

fn validate_proof_shape(proof: &VerifiedParentStepUpProof) -> Result<(), ParentStepUpProofError> {
    (proof.signature.len() == AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES)
        .then_some(())
        .ok_or(ParentStepUpProofError::Rejected)?;
    validate_unsigned_shape(&proof.validation, &proof.target_device_id)
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
    let lifetime_seconds = (expires_at - observed_at).num_seconds();
    (0..=MAX_PARENT_STEP_UP_PROOF_LIFETIME_SECONDS).contains(&lifetime_seconds)
}
