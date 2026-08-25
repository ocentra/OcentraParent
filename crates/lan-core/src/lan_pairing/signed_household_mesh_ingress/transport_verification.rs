use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use ocentra_parent_agent_protocol::{
    constants,
    lan_pairing::signed_household_mesh_ingress::transport::{
        lan_signed_household_mesh_transport_signing_bytes, LanHouseholdMeshPublicKeySha256,
        LanSignedChildBeaconIngressEnvelope, LanSignedHouseholdMeshTransportEnvelope,
    },
};

use super::super::signed_child_agent;
use super::{
    LanSignedHouseholdMeshCryptographicVerificationContext,
    LanSignedHouseholdMeshIngressVerificationError,
};

// BOUNDARY-INVARIANT: only Ed25519 keys and signatures that match both the
// signed child beacon and the registry-sourced cryptographic context can leave
// this decoder as verified key material.
pub(super) fn transport_verifying_key(
    envelope: &LanSignedHouseholdMeshTransportEnvelope,
) -> Result<VerifyingKey, LanSignedHouseholdMeshIngressVerificationError> {
    if envelope.signature_algorithm.as_str()
        != constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::UnsupportedAlgorithm);
    }
    let public_key_bytes = STANDARD
        .decode(envelope.public_key_base64.as_str())
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::InvalidPublicKey)?;
    let key_bytes: [u8; 32] = public_key_bytes
        .as_slice()
        .try_into()
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::InvalidPublicKey)?;
    VerifyingKey::from_bytes(&key_bytes)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::InvalidPublicKey)
}

pub(super) fn validate_signer(
    packet: &LanSignedChildBeaconIngressEnvelope,
    context: &LanSignedHouseholdMeshCryptographicVerificationContext,
    verifying_key: &VerifyingKey,
    envelope: &LanSignedHouseholdMeshTransportEnvelope,
) -> Result<LanHouseholdMeshPublicKeySha256, LanSignedHouseholdMeshIngressVerificationError> {
    let public_key_id = signed_child_agent::signed_child_agent_public_key_id(verifying_key);
    if envelope.public_key_id.as_str() != public_key_id {
        return Err(LanSignedHouseholdMeshIngressVerificationError::PublicKeyIdMismatch);
    }
    let public_key_sha256 = signed_child_agent::signed_child_agent_public_key_sha256(verifying_key);
    if public_key_id != context.expected_signer_public_key_id.as_str()
        || public_key_sha256 != context.expected_signer_public_key_sha256.as_str()
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::UntrustedSigner);
    }
    if packet.signed_child_agent.public_key_id != envelope.public_key_id.as_str()
        || packet.signed_child_agent.public_key_base64 != envelope.public_key_base64.as_str()
        || packet.signed_child_agent.signature_algorithm != envelope.signature_algorithm.as_str()
    {
        return Err(LanSignedHouseholdMeshIngressVerificationError::BeaconBindingMismatch);
    }
    LanHouseholdMeshPublicKeySha256::try_new(public_key_sha256)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::InvalidPublicKey)
}

pub(super) fn verify_transport_signature(
    envelope: &LanSignedHouseholdMeshTransportEnvelope,
    verifying_key: &VerifyingKey,
) -> Result<(), LanSignedHouseholdMeshIngressVerificationError> {
    let signature_bytes = STANDARD
        .decode(envelope.signature_base64.as_str())
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::InvalidSignature)?;
    let signature = Signature::from_slice(&signature_bytes)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::InvalidSignature)?;
    let payload = lan_signed_household_mesh_transport_signing_bytes(&envelope.claim)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::SerializationFailed)?;
    verifying_key
        .verify(&payload, &signature)
        .map_err(|_error| LanSignedHouseholdMeshIngressVerificationError::SignatureRejected)
}
