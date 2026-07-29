use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use ocentra_family_identity_core::household_authority::HouseholdAuthorityInput;
use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrantAssertionSnapshot;

use super::{AuthenticatedDeliveryGrantIssuanceError, DeliveryGrantBindings};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedAuthorityBindings {
    pub bindings: DeliveryGrantBindings,
    pub assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
    /// Current household authority is signed by the trusted authority producer.
    /// The issuance caller's copy is never used as authorization evidence.
    pub household_authority: HouseholdAuthorityInput,
    pub signature: Vec<u8>,
}

pub struct AuthenticatedDeliveryGrantAuthorityVerifier {
    verifying_key: VerifyingKey,
}

impl AuthenticatedDeliveryGrantAuthorityVerifier {
    pub fn new(verifying_key: VerifyingKey) -> Self {
        Self { verifying_key }
    }

    pub fn verify(
        &self,
        signed: &SignedAuthorityBindings,
    ) -> Result<
        (
            DeliveryGrantBindings,
            AuthenticatedDeliveryGrantAssertionSnapshot,
            HouseholdAuthorityInput,
        ),
        AuthenticatedDeliveryGrantIssuanceError,
    > {
        let signature = Signature::from_slice(&signed.signature).map_err(|_error| {
            AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected
        })?;
        let bytes = signing_bytes(signed)?;
        self.verifying_key
            .verify_strict(&bytes, &signature)
            .map_err(|_error| {
                AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected
            })?;
        Ok((
            signed.bindings.clone(),
            signed.assertions.clone(),
            signed.household_authority,
        ))
    }
}

pub struct AuthenticatedDeliveryGrantAuthoritySigner {
    signing_key: SigningKey,
}

impl AuthenticatedDeliveryGrantAuthoritySigner {
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
        bindings: DeliveryGrantBindings,
        assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
        household_authority: HouseholdAuthorityInput,
    ) -> SignedAuthorityBindings {
        let bytes =
            serde_json::to_vec(&(bindings.clone(), assertions.clone(), household_authority))
                .unwrap_or_else(|_error| Vec::new());
        let signature = self.signing_key.sign(&bytes).to_bytes().to_vec();
        SignedAuthorityBindings {
            bindings,
            assertions,
            household_authority,
            signature,
        }
    }
}

fn signing_bytes(
    signed: &SignedAuthorityBindings,
) -> Result<Vec<u8>, AuthenticatedDeliveryGrantIssuanceError> {
    serde_json::to_vec(&(
        signed.bindings.clone(),
        signed.assertions.clone(),
        signed.household_authority,
    ))
    .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
}
