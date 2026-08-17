use super::{CurrentSignerAuthority, DeviceTrustLifecycleState};

impl CurrentSignerAuthority {
    pub fn state(&self) -> DeviceTrustLifecycleState {
        self.state
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
