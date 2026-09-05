use std::fmt;

use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::{
    LanHouseholdMeshChildDeviceId, LanHouseholdMeshFamilyHash, LanHouseholdMeshIdempotencyKey,
    LanHouseholdMeshInstallId, LanHouseholdMeshLocalEventRef, LanHouseholdMeshMessageId,
    LanHouseholdMeshNonce, LanHouseholdMeshPairingId, LanHouseholdMeshParentDeviceId,
    LanHouseholdMeshPayloadSha256, LanHouseholdMeshPublicKeyId, LanHouseholdMeshPublicKeySha256,
    LanHouseholdMeshRegistryProofDigest, LanHouseholdMeshRouteId, LanHouseholdMeshSequenceDto,
    LanHouseholdMeshTargetDeviceId, LanSignedHouseholdMeshMessageType,
    LanSignedHouseholdMeshTransportClaimDto,
};
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind;

mod accessors;

/// Canonical signed identities that a durable receipt store must reserve in a
/// single transaction before any runtime authorization can be minted.
/// Registry-owned authority generation is deliberately absent from this signed
/// identity and must be joined by trusted custody before reservation.
pub struct LanHouseholdMeshDurableReplayIdentity<'a> {
    family_hash: &'a LanHouseholdMeshFamilyHash,
    child_device_id: &'a LanHouseholdMeshChildDeviceId,
    target_device_id: &'a LanHouseholdMeshTargetDeviceId,
    parent_device_id: &'a LanHouseholdMeshParentDeviceId,
    install_id: &'a LanHouseholdMeshInstallId,
    pairing_id: &'a LanHouseholdMeshPairingId,
    registry_proof_digest: &'a LanHouseholdMeshRegistryProofDigest,
    signer_public_key_id: &'a LanHouseholdMeshPublicKeyId,
    signer_public_key_sha256: &'a LanHouseholdMeshPublicKeySha256,
    message_kind: &'a LanSignedChildAgentMessageKind,
    local_event_ref: &'a LanHouseholdMeshLocalEventRef,
    lan_message_type: &'a LanSignedHouseholdMeshMessageType,
    route_id: &'a LanHouseholdMeshRouteId,
    message_id: &'a LanHouseholdMeshMessageId,
    idempotency_key: &'a LanHouseholdMeshIdempotencyKey,
    nonce: &'a LanHouseholdMeshNonce,
    sequence: LanHouseholdMeshSequenceDto,
    canonical_payload_sha256: &'a LanHouseholdMeshPayloadSha256,
}

impl fmt::Debug for LanHouseholdMeshDurableReplayIdentity<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanHouseholdMeshDurableReplayIdentity")
            .field("identities", &"[redacted]")
            .field("sequence", &self.sequence.value())
            .finish()
    }
}

impl<'a> LanHouseholdMeshDurableReplayIdentity<'a> {
    pub(super) fn from_verified_ingress(
        claim: &'a LanSignedHouseholdMeshTransportClaimDto,
        signer_public_key_id: &'a LanHouseholdMeshPublicKeyId,
        signer_public_key_sha256: &'a LanHouseholdMeshPublicKeySha256,
    ) -> Self {
        Self {
            family_hash: &claim.family_hash,
            child_device_id: &claim.child_device_id,
            target_device_id: &claim.target_device_id,
            parent_device_id: &claim.parent_device_id,
            install_id: &claim.install_id,
            pairing_id: &claim.pairing_id,
            registry_proof_digest: &claim.registry_proof_digest,
            signer_public_key_id,
            signer_public_key_sha256,
            message_kind: &claim.message_kind,
            local_event_ref: &claim.local_event_ref,
            lan_message_type: &claim.lan_message_type,
            route_id: &claim.route_id,
            message_id: &claim.message_id,
            idempotency_key: &claim.idempotency_key,
            nonce: &claim.nonce,
            sequence: claim.sequence,
            canonical_payload_sha256: &claim.canonical_payload_sha256,
        }
    }
}
