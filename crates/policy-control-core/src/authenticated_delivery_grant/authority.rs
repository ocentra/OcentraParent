use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrantAssertionSnapshot;

use super::{AuthenticatedDeliveryGrantIssuanceError, DeliveryGrantBindings};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedAuthorityBindings {
    pub bindings: DeliveryGrantBindings,
    pub assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
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
        Ok((signed.bindings.clone(), signed.assertions.clone()))
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
    ) -> SignedAuthorityBindings {
        let bytes = serde_json::to_vec(&(bindings.clone(), assertions.clone()))
            .unwrap_or_else(|_error| Vec::new());
        let signature = self.signing_key.sign(&bytes).to_bytes().to_vec();
        SignedAuthorityBindings {
            bindings,
            assertions,
            signature,
        }
    }
}

fn signing_bytes(
    signed: &SignedAuthorityBindings,
) -> Result<Vec<u8>, AuthenticatedDeliveryGrantIssuanceError> {
    serde_json::to_vec(&(signed.bindings.clone(), signed.assertions.clone()))
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
}
