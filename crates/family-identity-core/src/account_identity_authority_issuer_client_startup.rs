use super::AccountIdentityAuthorityIssuerStartupState;

impl AccountIdentityAuthorityIssuerStartupState {
    pub fn active_key_count(&self) -> u64 {
        self.active_key_count
    }

    pub fn pending_outbox_count(&self) -> u64 {
        self.pending_outbox_count
    }

    /// Whether the last bounded recovery pass found more expired work to
    /// process. A false value means no expired reservation or outbox lease
    /// remained when startup recovery stopped.
    pub fn recovery_backlog(&self) -> bool {
        self.recovery_backlog
    }
}
