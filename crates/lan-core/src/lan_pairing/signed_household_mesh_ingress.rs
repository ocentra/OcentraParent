#![forbid(unsafe_code)]

pub mod replay_identity;
mod transport_verification;
mod validation;

use std::fmt;

use ocentra_parent_agent_protocol::{
    household_mesh::HouseholdMeshTransportEnvelope,
    lan_pairing::signed_household_mesh_ingress::transport::{
        LanHouseholdMeshChildDeviceId, LanHouseholdMeshFamilyHash, LanHouseholdMeshInstallId,
        LanHouseholdMeshPairingId, LanHouseholdMeshParentDeviceId, LanHouseholdMeshPayloadSha256,
        LanHouseholdMeshPublicKeyId, LanHouseholdMeshPublicKeySha256,
        LanHouseholdMeshRegistryProofDigest, LanHouseholdMeshRouteId,
        LanHouseholdMeshTargetDeviceId, LanHouseholdMeshTimestamp,
        LanSignedChildBeaconIngressEnvelope, LanSignedHouseholdMeshTransportClaimDto,
    },
};
use sha2::{Digest, Sha256};

use super::{signed_child_agent, LanSignedChildAgentVerificationContext};
use transport_verification::{
    transport_verifying_key, validate_signer, verify_transport_signature,
};
use validation::{
    validate_authority_binding, validate_beacon_binding, validate_payload_window,
    validate_required_fields, validate_safe_payload, validate_schema, validate_time_window,
    validate_transport_binding,
};

const TRANSPORT_PAYLOAD_DIGEST_DOMAIN: &[u8] = b"ocentra.lan.household-mesh.transport-payload.v1\0";

/// Fail-closed reasons why a signed household-mesh ingress cannot be accepted.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum LanSignedHouseholdMeshIngressVerificationError {
    #[error("unsupported signed household-mesh schema version")]
    UnsupportedSchemaVersion,
    #[error("a required signed household-mesh field is empty or malformed")]
    EmptyRequiredField,
    #[error("unsupported signed household-mesh signature algorithm")]
    UnsupportedAlgorithm,
    #[error("malformed signed household-mesh timestamp")]
    MalformedTimestamp,
    #[error("signed household-mesh claim was issued in the future")]
    FutureIssuedAt,
    #[error("signed household-mesh claim has expired")]
    Expired,
    #[error("signed household-mesh claim lifetime exceeds the fixed ingress window")]
    ClaimLifetimeExceeded,
    #[error("invalid signed household-mesh public key")]
    InvalidPublicKey,
    #[error("signed household-mesh public key identifier does not match")]
    PublicKeyIdMismatch,
    #[error("signed household-mesh signer is not the expected signer")]
    UntrustedSigner,
    #[error("invalid signed household-mesh signature encoding")]
    InvalidSignature,
    #[error("signed household-mesh signature verification failed")]
    SignatureRejected,
    #[error("signed household-mesh canonical serialization failed")]
    SerializationFailed,
    #[error("signed household-mesh payload digest does not match")]
    PayloadDigestMismatch,
    #[error("signed household-mesh payload freshness window does not match")]
    PayloadWindowMismatch,
    #[error("signed child beacon and transport claim do not bind identically")]
    BeaconBindingMismatch,
    #[error("signed transport claim and payload do not bind identically")]
    TransportBindingMismatch,
    #[error("signed transport claim does not match the expected registry binding")]
    AuthorityBindingMismatch,
    #[error("signed household-mesh payload requests an unsupported authority path")]
    UnsupportedPayload,
    #[error("signed child beacon verification failed")]
    SignedChildAgentRejected,
}

/// Expected values used only to verify cryptographic binding.
///
/// This context is not authorization and must not be deserialized from a peer
/// request. A later registry-and-custody composer must source these values from
/// durable trusted-device and selected-route state, reserve the returned replay
/// identity, and mint the separate non-forgeable runtime authorization.
#[derive(Clone, PartialEq, Eq)]
pub struct LanSignedHouseholdMeshCryptographicVerificationContext {
    expected_family_hash: LanHouseholdMeshFamilyHash,
    expected_parent_device_id: LanHouseholdMeshParentDeviceId,
    expected_child_device_id: LanHouseholdMeshChildDeviceId,
    expected_target_device_id: LanHouseholdMeshTargetDeviceId,
    expected_install_id: LanHouseholdMeshInstallId,
    expected_route_id: LanHouseholdMeshRouteId,
    expected_pairing_id: LanHouseholdMeshPairingId,
    expected_registry_proof_digest: LanHouseholdMeshRegistryProofDigest,
    expected_signer_public_key_id: LanHouseholdMeshPublicKeyId,
    expected_signer_public_key_sha256: LanHouseholdMeshPublicKeySha256,
}

impl fmt::Debug for LanSignedHouseholdMeshCryptographicVerificationContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanSignedHouseholdMeshCryptographicVerificationContext")
            .field("registry_binding", &"[redacted]")
            .finish()
    }
}

impl LanSignedHouseholdMeshCryptographicVerificationContext {
    /// Constructs the cryptographic expectation only inside LAN's trusted
    /// registry adapter. Peer request code cannot construct or deserialize it.
    pub(super) fn from_registry_source(
        expected_family_hash: LanHouseholdMeshFamilyHash,
        expected_parent_device_id: LanHouseholdMeshParentDeviceId,
        expected_child_device_id: LanHouseholdMeshChildDeviceId,
        expected_target_device_id: LanHouseholdMeshTargetDeviceId,
        expected_install_id: LanHouseholdMeshInstallId,
        expected_route_id: LanHouseholdMeshRouteId,
        expected_pairing_id: LanHouseholdMeshPairingId,
        expected_registry_proof_digest: LanHouseholdMeshRegistryProofDigest,
        expected_signer_public_key_id: LanHouseholdMeshPublicKeyId,
        expected_signer_public_key_sha256: LanHouseholdMeshPublicKeySha256,
    ) -> Self {
        Self {
            expected_family_hash,
            expected_parent_device_id,
            expected_child_device_id,
            expected_target_device_id,
            expected_install_id,
            expected_route_id,
            expected_pairing_id,
            expected_registry_proof_digest,
            expected_signer_public_key_id,
            expected_signer_public_key_sha256,
        }
    }
}

/// A cryptographic verification result, deliberately distinct from LAN runtime
/// authorization. It is non-serializable and has no public constructor.
pub struct LanCryptographicallyVerifiedHouseholdMeshIngress {
    claim: LanSignedHouseholdMeshTransportClaimDto,
    payload: HouseholdMeshTransportEnvelope,
    signer_public_key_id: LanHouseholdMeshPublicKeyId,
    signer_public_key_sha256: LanHouseholdMeshPublicKeySha256,
}

impl fmt::Debug for LanCryptographicallyVerifiedHouseholdMeshIngress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanCryptographicallyVerifiedHouseholdMeshIngress")
            .field("message_type", &self.claim.lan_message_type)
            .field("sequence", &self.claim.sequence.value())
            .finish_non_exhaustive()
    }
}

impl LanCryptographicallyVerifiedHouseholdMeshIngress {
    /// Returns the complete signed identity statement.
    pub fn claim(&self) -> &LanSignedHouseholdMeshTransportClaimDto {
        &self.claim
    }

    /// Returns the canonical payload whose digest was signed.
    pub fn payload(&self) -> &HouseholdMeshTransportEnvelope {
        &self.payload
    }

    /// Returns the verified signer key identifier.
    pub fn signer_public_key_id(&self) -> &str {
        self.signer_public_key_id.as_str()
    }

    /// Returns the verified signer key SHA-256 digest.
    pub fn signer_public_key_sha256(&self) -> &str {
        self.signer_public_key_sha256.as_str()
    }

    /// Returns the signed identities that durable custody must reserve atomically.
    pub fn durable_replay_identity(
        &self,
    ) -> replay_identity::LanHouseholdMeshDurableReplayIdentity<'_> {
        replay_identity::LanHouseholdMeshDurableReplayIdentity::from_verified_ingress(
            &self.claim,
            &self.signer_public_key_id,
            &self.signer_public_key_sha256,
        )
    }
}

/// Verifies the existing signed hello/heartbeat and the separately signed
/// household-mesh transport claim. This produces no runtime authority.
pub fn verify_lan_signed_household_mesh_ingress(
    packet: LanSignedChildBeaconIngressEnvelope,
    observed_at: &LanHouseholdMeshTimestamp,
    context: &LanSignedHouseholdMeshCryptographicVerificationContext,
) -> Result<
    LanCryptographicallyVerifiedHouseholdMeshIngress,
    LanSignedHouseholdMeshIngressVerificationError,
> {
    validate_schema(&packet)?;
    validate_required_fields(&packet.signed_transport)?;

    // ALLOC-JUSTIFICATION: the established W18 verifier owns this bounded
    // context; copying registry-derived brands does not create authority.
    let beacon_context = LanSignedChildAgentVerificationContext {
        expected_parent_device_id: context.expected_parent_device_id.as_str().to_owned(),
        expected_family_hash: context.expected_family_hash.as_str().to_owned(),
        // ALLOC-JUSTIFICATION: W18 also owns the route and optional child text.
        expected_route_id: context.expected_route_id.as_str().to_owned(),
        expected_child_device_id: Some(context.expected_child_device_id.as_str().to_owned()),
    };
    let beacon_claim = signed_child_agent::verify_lan_signed_child_agent_authenticity(
        &packet.signed_child_agent,
        observed_at.as_str(),
        &beacon_context,
    )
    .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::SignedChildAgentRejected)?;

    let verifying_key = transport_verifying_key(&packet.signed_transport)?;
    let signer_public_key_sha256 =
        validate_signer(&packet, context, &verifying_key, &packet.signed_transport)?;
    verify_transport_signature(&packet.signed_transport, &verifying_key)?;
    validate_time_window(&packet.signed_transport.claim, observed_at)?;
    validate_beacon_binding(&beacon_claim, &packet.signed_transport.claim)?;
    validate_authority_binding(&packet.signed_transport.claim, context)?;
    validate_transport_binding(&packet.signed_transport)?;
    validate_payload_window(&packet.signed_transport, observed_at)?;
    validate_safe_payload(&packet.signed_transport.payload)?;

    let signed_transport = packet.signed_transport;
    Ok(LanCryptographicallyVerifiedHouseholdMeshIngress {
        claim: signed_transport.claim,
        payload: signed_transport.payload,
        signer_public_key_id: signed_transport.public_key_id,
        signer_public_key_sha256,
    })
}

pub(super) fn lan_household_mesh_payload_sha256(
    payload: &HouseholdMeshTransportEnvelope,
) -> Result<LanHouseholdMeshPayloadSha256, LanSignedHouseholdMeshIngressVerificationError> {
    let payload = serde_json::to_vec(payload)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::SerializationFailed)?;
    let mut digest = Sha256::new();
    digest.update(TRANSPORT_PAYLOAD_DIGEST_DOMAIN);
    digest.update(payload);
    LanHouseholdMeshPayloadSha256::try_new(format!("{:x}", digest.finalize()))
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::SerializationFailed)
}
