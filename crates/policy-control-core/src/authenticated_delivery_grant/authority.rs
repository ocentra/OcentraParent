use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

use ocentra_family_identity_core::household_authority::HouseholdAuthorityInput;
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
};

use super::{AuthenticatedDeliveryGrantIssuanceError, DeliveryGrantBindings};
use crate::policy_authority::PolicyControlDecision;
use crate::policy_contract_helpers::authority::PolicyContractAuthorityDecision;

const AUTHORITY_BINDINGS_WIRE_OVERHEAD_BYTES: usize = 1_024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignedAuthorityBindings {
    pub bindings: DeliveryGrantBindings,
    pub assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
    /// Current household authority is signed by the trusted authority producer.
    /// The issuance caller's copy is never used as authorization evidence.
    pub household_authority: HouseholdAuthorityInput,
    /// The trusted producer's policy decision is signed before grant issuance.
    pub policy_decision: PolicyControlDecision,
    /// The trusted producer's policy contract authority is signed before grant issuance.
    pub policy_authority: PolicyContractAuthorityDecision,
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
            PolicyControlDecision,
            PolicyContractAuthorityDecision,
        ),
        AuthenticatedDeliveryGrantIssuanceError,
    > {
        validate_signed_shape(signed)?;
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
            signed.policy_decision,
            signed.policy_authority.clone(),
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
        policy_decision: PolicyControlDecision,
        policy_authority: PolicyContractAuthorityDecision,
    ) -> Result<SignedAuthorityBindings, AuthenticatedDeliveryGrantIssuanceError> {
        let unsigned = SignedAuthorityBindings {
            bindings,
            assertions,
            household_authority,
            policy_decision,
            policy_authority,
            signature: Vec::new(),
        };
        validate_signed_shape(&unsigned)?;
        let bytes = signing_bytes(&unsigned)?;
        let signature = self.signing_key.sign(&bytes).to_bytes().to_vec();
        Ok(SignedAuthorityBindings {
            bindings: unsigned.bindings,
            assertions: unsigned.assertions,
            household_authority: unsigned.household_authority,
            policy_decision: unsigned.policy_decision,
            policy_authority: unsigned.policy_authority,
            signature,
        })
    }
}

fn signing_bytes(
    signed: &SignedAuthorityBindings,
) -> Result<Vec<u8>, AuthenticatedDeliveryGrantIssuanceError> {
    serde_json::to_vec(&(
        &signed.bindings,
        &signed.assertions,
        signed.household_authority,
        signed.policy_decision,
        &signed.policy_authority,
    ))
    .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
}

fn validate_signed_shape(
    signed: &SignedAuthorityBindings,
) -> Result<(), AuthenticatedDeliveryGrantIssuanceError> {
    let bindings = &signed.bindings;
    let fields = [
        bindings.issuer_actor_id.as_str(),
        bindings.household_id.as_str(),
        bindings.parent_device_id.as_str(),
        bindings.child_profile_id.as_str(),
        bindings.target_device_id.as_str(),
        bindings.policy_decision_id.as_str(),
        bindings.policy_version.as_str(),
        bindings.action_id.as_str(),
        bindings.capability_id.as_str(),
        bindings.evidence_digest.as_str(),
        bindings.payload_digest.as_str(),
        bindings.nonce.as_str(),
        bindings.issued_at.as_str(),
        bindings.expires_at.as_str(),
        bindings.revocation_version.as_str(),
    ];
    let bounded = fields.iter().all(|field| {
        !field.trim().is_empty() && field.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
    });
    let payload_length_bounded =
        bindings.payload_length <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES;
    let wire_len = fields.iter().map(|field| field.len()).sum::<usize>()
        + AUTHORITY_BINDINGS_WIRE_OVERHEAD_BYTES;
    (bounded
        && payload_length_bounded
        && wire_len <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
}
