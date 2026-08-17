use super::{CurrentSignerAuthority, DeviceTrustLifecycleState};

impl CurrentSignerAuthority {
    pub fn state(&self) -> DeviceTrustLifecycleState {
        self.state
    }

    pub fn family_id(&self) -> &str {
        &self.family_id
    }

    pub fn trust_subject(&self) -> &str {
        &self.trust_subject
    }

    pub fn parent_device_id(&self) -> &str {
        &self.parent_device_id
    }

    pub fn child_device_id(&self) -> &str {
        &self.child_device_id
    }

    pub fn installation_id(&self) -> &str {
        &self.installation_id
    }

    pub fn signer_public_key(&self) -> &[u8; 32] {
        &self.signer_public_key
    }

    pub fn signer_key_id(&self) -> &str {
        &self.signer_key_id
    }

    pub fn signer_key_sha256(&self) -> &str {
        &self.signer_key_sha256
    }

    pub fn lifecycle_generation(&self) -> u64 {
        self.lifecycle_generation
    }

    pub fn installation_binding_generation(&self) -> u64 {
        self.installation_binding_generation
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub fn credential_id(&self) -> &str {
        &self.credential_id
    }

    pub fn credential_algorithm(&self) -> i32 {
        self.credential_algorithm
    }

    pub fn credential_sign_count(&self) -> u32 {
        self.credential_sign_count
    }

    pub(crate) fn matches_registration(
        &self,
        parent_intent_digest: &str,
        parent_presence_receipt: &str,
        parent_route_id: &str,
        signer_public_key: [u8; 32],
        installation_id: &str,
        credential_id: &str,
        credential_algorithm: i32,
        credential_sign_count: u32,
    ) -> bool {
        self.parent_intent_digest == parent_intent_digest
            && self.parent_presence_receipt == parent_presence_receipt
            && self.parent_route_id == parent_route_id
            && self.signer_public_key == signer_public_key
            && self.installation_id == installation_id
            && self.credential_id == credential_id
            && self.credential_algorithm == credential_algorithm
            && self.credential_sign_count == credential_sign_count
    }
}
