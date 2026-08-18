use super::*;

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
impl VerifiedInviteRecipient {
    pub(crate) fn from_provider_account(input: VerifiedInviteRecipientInput) -> Option<Self> {
        let canonical_email = input.canonical_email.trim().to_ascii_lowercase();
        (canonical_email.len() >= 4 && canonical_email.contains('@')).then_some(Self {
            provider: input.provider,
            provider_subject: input.provider_subject,
            account_id: input.account_id,
            email_digest: super::support_security::digest_email(&canonical_email),
        })
    }
}
impl InviteMembershipHandoff {
    pub fn invite_id(&self) -> &SetupInviteId {
        &self.invite_id
    }
    pub fn household_id(&self) -> &FamilyId {
        &self.household_id
    }
    pub fn recipient_account_id(&self) -> &ParentAccountId {
        &self.recipient_account_id
    }
    pub fn target_role(&self) -> SetupInviteTargetRole {
        self.target_role
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
impl VerifiedRecoveryIdentityProof {
    pub(crate) fn from_account_provider(input: VerifiedRecoveryIdentityProofInput) -> Self {
        Self {
            proof_id: input.proof_id,
            provider: input.provider,
            provider_subject: input.provider_subject,
            account_id: input.account_id,
            household_id: input.household_id,
            member_id: input.member_id,
            device_id: input.device_id,
            role: input.role,
            kind: input.kind,
            support_channel: input.support_channel,
            expires_at_epoch_millis: input.expires_at_epoch_millis,
        }
    }
}

impl RecoveryCustodyDeliveryReceipt {
    pub(crate) fn from_custody_owner(
        handoff_id: String,
        correlation_id: String,
        attempt_id: String,
    ) -> Self {
        Self {
            handoff_id,
            correlation_id,
            attempt_id,
        }
    }
}
