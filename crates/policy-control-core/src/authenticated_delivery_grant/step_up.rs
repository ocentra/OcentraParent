use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use ocentra_family_identity_core::household_authority::ParentStepUpValidationInput;
use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrantAssertionSnapshot;
use serde::{Deserialize, Serialize};

use super::AuthenticatedDeliveryGrantIssuanceError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedParentStepUpProof {
    pub validation: ParentStepUpValidationInput,
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
            AuthenticatedDeliveryGrantAssertionSnapshot,
        ),
        AuthenticatedDeliveryGrantIssuanceError,
    > {
        let signature = Signature::from_slice(&proof.signature)
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
        self.verifying_key
            .verify_strict(
                &signing_bytes(&proof.validation, &proof.assertions),
                &signature,
            )
            .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)?;
        Ok((proof.validation.clone(), proof.assertions.clone()))
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
        assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
    ) -> VerifiedParentStepUpProof {
        let signature = self
            .signing_key
            .sign(&signing_bytes(&validation, &assertions))
            .to_bytes()
            .to_vec();
        VerifiedParentStepUpProof {
            validation,
            assertions,
            signature,
        }
    }
}

fn signing_bytes(
    validation: &ParentStepUpValidationInput,
    assertions: &AuthenticatedDeliveryGrantAssertionSnapshot,
) -> Vec<u8> {
    serde_json::to_vec(&(validation, assertions)).unwrap_or_default()
}
