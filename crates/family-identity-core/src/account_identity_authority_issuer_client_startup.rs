use super::AccountIdentityAuthorityIssuerStartupState;

impl AccountIdentityAuthorityIssuerStartupState {
    pub fn active_key_count(&self) -> u64 {
        self.active_key_count
    }

    pub fn pending_outbox_count(&self) -> u64 {
        self.pending_outbox_count
    }
}
