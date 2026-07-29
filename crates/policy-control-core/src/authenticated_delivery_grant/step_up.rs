use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use ocentra_family_identity_core::household_authority::ParentStepUpValidationInput;
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};
use serde::{Deserialize, Serialize};

use super::AuthenticatedDeliveryGrantIssuanceError;

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
        AuthenticatedDeliveryGrantIssuanceError,
    > {
        validate_proof_shape(proof)?;
        let signature = Signature::from_slice(&proof.signature)
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
        let bytes = signing_bytes(
            &proof.validation,
            &proof.target_device_id,
            &proof.assertions,
        )?;
        self.verifying_key
            .verify_strict(&bytes, &signature)
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
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
    ) -> Result<VerifiedParentStepUpProof, AuthenticatedDeliveryGrantIssuanceError> {
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
) -> Result<Vec<u8>, AuthenticatedDeliveryGrantIssuanceError> {
    serde_json::to_vec(&(validation, target_device_id, assertions))
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
}

fn validate_proof_shape(
    proof: &VerifiedParentStepUpProof,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    (proof.signature.len() == AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
    validate_unsigned_shape(&proof.validation, &proof.target_device_id)
}

fn validate_unsigned_shape(
    validation: &ParentStepUpValidationInput,
    target_device_id: &str,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let assertion = validation
        .assertion
        .as_ref()
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
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
    (!target_device_id.trim().is_empty()
        && bounded
        && wire_len <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
}
