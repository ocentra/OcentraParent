use std::fmt;

use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind;

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LanTrustedDeviceSignerAnchor {
    pub(crate) public_key_id: String,
    pub(crate) public_key_sha256: String,
    pub(crate) install_id: String,
    pub(crate) family_hash: String,
    pub(crate) parent_device_id: String,
    pub(crate) authority_generation: u64,
}

impl fmt::Debug for LanTrustedDeviceSignerAnchor {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanTrustedDeviceSignerAnchor")
            .field("redacted", &true)
            .finish()
    }
}

#[derive(PartialEq, Eq)]
pub struct LanRegisteredSignedChildAuthority {
    pub(crate) pairing_id: String,
    pub(crate) child_device_id: String,
    pub(crate) target_device_id: String,
    pub(crate) install_id: String,
    pub(crate) family_hash: String,
    pub(crate) parent_device_id: String,
    pub(crate) route_id: String,
    pub(crate) registry_proof_digest: String,
    pub(crate) message_kind: LanSignedChildAgentMessageKind,
    pub(crate) message_id: String,
    pub(crate) idempotency_key: String,
    pub(crate) nonce: String,
    pub(crate) sequence: u64,
    pub(crate) authority_generation: u64,
    pub(crate) public_key_id: String,
    pub(crate) public_key_sha256: String,
}

impl fmt::Debug for LanRegisteredSignedChildAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanRegisteredSignedChildAuthority")
            .field("redacted", &true)
            .finish()
    }
}

impl LanRegisteredSignedChildAuthority {
    pub fn pairing_id(&self) -> &str {
        &self.pairing_id
    }
    pub fn child_device_id(&self) -> &str {
        &self.child_device_id
    }
    pub fn target_device_id(&self) -> &str {
        &self.target_device_id
    }
    pub fn install_id(&self) -> &str {
        &self.install_id
    }
    pub fn family_hash(&self) -> &str {
        &self.family_hash
    }
    pub fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }
    pub fn route_id(&self) -> &str {
        &self.route_id
    }
    pub fn registry_proof_digest(&self) -> &str {
        &self.registry_proof_digest
    }
    pub fn message_kind(&self) -> LanSignedChildAgentMessageKind {
        self.message_kind.clone()
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
    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }
    pub fn public_key_id(&self) -> &str {
        &self.public_key_id
    }
    pub fn public_key_sha256(&self) -> &str {
        &self.public_key_sha256
    }
}

pub(crate) struct LanSignedChildAuthorityBindingRef<'a> {
    pub(crate) pairing_id: &'a str,
    pub(crate) child_device_id: &'a str,
    pub(crate) target_device_id: &'a str,
    pub(crate) install_id: &'a str,
    pub(crate) family_hash: &'a str,
    pub(crate) parent_device_id: &'a str,
    pub(crate) route_id: &'a str,
    pub(crate) registry_proof_digest: &'a str,
    pub(crate) authority_generation: u64,
    pub(crate) public_key_id: &'a str,
    pub(crate) public_key_sha256: &'a str,
}
