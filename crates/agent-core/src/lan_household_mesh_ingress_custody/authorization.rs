use super::LanHouseholdMeshIngressAuthorization;
use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::LanSignedHouseholdMeshMessageType;

mod consume;
mod identity_accessors;

impl LanHouseholdMeshIngressAuthorization {
    pub fn signer_public_key_id(&self) -> &str {
        &self.signer_public_key_id
    }

    pub fn signer_public_key_sha256(&self) -> &str {
        &self.signer_public_key_sha256
    }

    pub fn message_kind(
        &self,
    ) -> ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind {
        self.message_kind.clone()
    }

    pub fn local_event_ref(&self) -> &str {
        &self.local_event_ref
    }

    pub fn lan_message_type(&self) -> LanSignedHouseholdMeshMessageType {
        self.lan_message_type.clone()
    }

    pub fn route_id(&self) -> &str {
        &self.route_id
    }

    pub fn message_id(&self) -> &str {
        &self.message_id
    }

    pub fn idempotency_key(&self) -> &str {
        &self.idempotency_key
    }

    pub fn nonce(&self) -> &str {
        &self.nonce
    }

    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn payload_digest(&self) -> &str {
        &self.payload_digest
    }

    pub fn install_id(&self) -> &str {
        &self.install_id
    }

    pub fn pairing_id(&self) -> &str {
        &self.pairing_id
    }

    pub fn registry_proof_digest(&self) -> &str {
        &self.registry_proof_digest
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn issued_at(&self) -> &str {
        &self.issued_at
    }

    pub fn expires_at(&self) -> &str {
        &self.expires_at
    }

    pub fn reserved_at(&self) -> &str {
        &self.reserved_at
    }
}
