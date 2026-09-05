use super::*;

impl fmt::Debug for SetupInviteCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SetupInviteCode")
            .field("invite_id", &self.invite_id)
            .field("token", &"<redacted>")
            .finish()
    }
}
impl Drop for SetupInviteCode {
    fn drop(&mut self) {
        self.token.clear();
    }
}
impl SetupInviteCode {
    pub fn invite_id(&self) -> &SetupInviteId {
        &self.invite_id
    }
    pub fn as_str(&self) -> &str {
        &self.token
    }
}
impl fmt::Debug for VerifiedInviteRecipient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedInviteRecipient")
            .field("provider", &self.provider)
            .field("provider_subject", &"<redacted>")
            .field("account_id", &self.account_id)
            .finish()
    }
}
impl fmt::Debug for VerifiedRecoveryIdentityProof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedRecoveryIdentityProof")
            .field("proof_id", &self.proof_id)
            .field("provider", &self.provider)
            .field("provider_subject", &"<redacted>")
            .field("account_id", &self.account_id)
            .field("household_id", &self.household_id)
            .field("kind", &self.kind)
            .field("support_channel", &self.support_channel)
            .field("expires_at_epoch_millis", &self.expires_at_epoch_millis)
            .finish()
    }
}
