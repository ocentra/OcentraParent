use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::{
    LanHouseholdMeshSequenceDto, LanSignedHouseholdMeshMessageType,
};
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind;

use super::LanHouseholdMeshDurableReplayIdentity;

impl<'a> LanHouseholdMeshDurableReplayIdentity<'a> {
    /// Returns the signed family hash.
    pub fn family_hash(&self) -> &'a str {
        self.family_hash.as_str()
    }

    /// Returns the signed child-device identifier.
    pub fn child_device_id(&self) -> &'a str {
        self.child_device_id.as_str()
    }

    /// Returns the signed target-device identifier.
    pub fn target_device_id(&self) -> &'a str {
        self.target_device_id.as_str()
    }

    /// Returns the signed parent-device identifier.
    pub fn parent_device_id(&self) -> &'a str {
        self.parent_device_id.as_str()
    }

    /// Returns the signed install identifier.
    pub fn install_id(&self) -> &'a str {
        self.install_id.as_str()
    }

    /// Returns the signed pairing identifier.
    pub fn pairing_id(&self) -> &'a str {
        self.pairing_id.as_str()
    }

    /// Returns the signed registry proof digest.
    pub fn registry_proof_digest(&self) -> &'a str {
        self.registry_proof_digest.as_str()
    }

    /// Returns the cryptographically verified signer key identifier.
    pub fn signer_public_key_id(&self) -> &'a str {
        self.signer_public_key_id.as_str()
    }

    /// Returns the cryptographically verified signer key digest.
    pub fn signer_public_key_sha256(&self) -> &'a str {
        self.signer_public_key_sha256.as_str()
    }

    /// Returns the signed child-agent message kind.
    pub fn message_kind(&self) -> LanSignedChildAgentMessageKind {
        // CLONE-JUSTIFICATION: the protocol enum is a small owned value and the
        // replay identity must not expose a reference beyond its claim borrow.
        self.message_kind.clone()
    }

    /// Returns the signed local-event reference.
    pub fn local_event_ref(&self) -> &'a str {
        self.local_event_ref.as_str()
    }

    /// Returns the signed LAN message type.
    pub fn lan_message_type(&self) -> LanSignedHouseholdMeshMessageType {
        // CLONE-JUSTIFICATION: the validated bounded scalar stays owned by the
        // verified claim while callers receive an independent typed value.
        self.lan_message_type.clone()
    }

    /// Returns the signed selected-route identifier.
    pub fn route_id(&self) -> &'a str {
        self.route_id.as_str()
    }

    /// Returns the signed message identifier.
    pub fn message_id(&self) -> &'a str {
        self.message_id.as_str()
    }

    /// Returns the signed idempotency key.
    pub fn idempotency_key(&self) -> &'a str {
        self.idempotency_key.as_str()
    }

    /// Returns the signed nonce.
    pub fn nonce(&self) -> &'a str {
        self.nonce.as_str()
    }

    /// Returns the signed monotonic sequence.
    pub fn sequence(&self) -> LanHouseholdMeshSequenceDto {
        self.sequence
    }

    /// Returns the signed canonical payload digest.
    pub fn canonical_payload_sha256(&self) -> &'a str {
        self.canonical_payload_sha256.as_str()
    }
}
