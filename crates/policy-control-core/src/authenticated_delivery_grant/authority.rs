use std::io::{self, Write};

use ocentra_eventing::ids::CorrelationId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityProof;
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAssertionSnapshot, AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES,
};

use super::{AuthenticatedDeliveryGrantIssuanceError, DeliveryGrantBindings};
use crate::policy_authority_resolved_decision::ResolvedPolicyDecision;
use crate::policy_contract_helpers::authority::PolicyContractAuthorityDecision;

const AUTHORITY_BINDINGS_WIRE_OVERHEAD_BYTES: usize = 1_024;

struct BoundedWireLenWriter {
    remaining: usize,
}

impl BoundedWireLenWriter {
    fn new(limit: usize) -> Self {
        Self { remaining: limit }
    }
}

impl Write for BoundedWireLenWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() > self.remaining {
            return Err(io::Error::new(
                io::ErrorKind::WriteZero,
                "authority envelope exceeds its signed wire limit",
            ));
        }
        self.remaining -= bytes.len();
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SignedAuthorityBindings {
    pub bindings: DeliveryGrantBindings,
    pub assertions: AuthenticatedDeliveryGrantAssertionSnapshot,
    /// Household authority is minted and signed by family identity. Policy only
    /// carries the opaque family-owned proof and consumes its verified material.
    pub household_authority_proof: HouseholdAuthorityProof,
    /// The trusted producer's resolved policy identity and decision are signed
    /// before grant issuance.
    pub resolved_policy_decision: ResolvedPolicyDecision,
    /// The trusted producer's policy contract authority is signed before grant issuance.
    pub policy_authority: PolicyContractAuthorityDecision,
    pub signature: Vec<u8>,
}

impl SignedAuthorityBindings {
    /// Derives the issuance audit chain identifier from authority material only
    /// after the caller has verified this envelope's signature. The request's
    /// correlation value is intentionally never an audit authority input.
    pub(crate) fn trusted_issuance_correlation_id(
        &self,
    ) -> Result<CorrelationId, AuthenticatedDeliveryGrantIssuanceError> {
        let mut hasher = Sha256::new();
        hasher.update(signing_bytes(self)?);
        CorrelationId::parse(format!(
            "authenticated-delivery-grant:issuance:v1:{:x}",
            hasher.finalize()
        ))
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
    }
}

fn signing_bytes(
    signed: &SignedAuthorityBindings,
) -> Result<Vec<u8>, AuthenticatedDeliveryGrantIssuanceError> {
    serde_json::to_vec(&(
        &signed.bindings,
        &signed.assertions,
        &signed.household_authority_proof,
        &signed.resolved_policy_decision,
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
        signed.resolved_policy_decision.aggregate_id.as_str(),
        signed.resolved_policy_decision.decision_id.as_str(),
    ];
    let bounded = fields.iter().all(|field| {
        !field.trim().is_empty() && field.len() <= AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
    });
    let payload_length_bounded =
        bindings.payload_length <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES;
    let wire_len = fields.iter().map(|field| field.len()).sum::<usize>()
        + AUTHORITY_BINDINGS_WIRE_OVERHEAD_BYTES;
    let signature_is_bounded = signed.signature.is_empty()
        || signed.signature.len() == AUTHENTICATED_DELIVERY_GRANT_SIGNATURE_BYTES;
    (bounded
        && payload_length_bounded
        && wire_len <= AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES
        && signature_is_bounded)
        .then_some(())
        .ok_or(AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)?;
    let mut wire_len_writer =
        BoundedWireLenWriter::new(AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES);
    serde_json::to_writer(&mut wire_len_writer, signed)
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::AuthorityProvenanceRejected)
}
