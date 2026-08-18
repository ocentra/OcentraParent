use std::fmt;

use super::*;

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

impl fmt::Debug for InviteMembershipHandoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InviteMembershipHandoff")
            .field("invite_id", &self.invite_id)
            .field("household_id", &self.household_id)
            .field("recipient_provider", &self.recipient_provider)
            .field("recipient_provider_subject", &"<redacted>")
            .field("recipient_account_id", &self.recipient_account_id)
            .field("target_role", &self.target_role)
            .finish()
    }
}

impl InviteMembershipDeliveryAttempt {
    pub fn invite_id(&self) -> &SetupInviteId {
        &self.invite_id
    }

    pub fn household_id(&self) -> &FamilyId {
        &self.household_id
    }

    pub fn recipient_provider(&self) -> &AccountIdentityProvider {
        &self.recipient_provider
    }

    pub fn recipient_provider_subject(&self) -> &AccountIdentityProviderSubject {
        &self.recipient_provider_subject
    }

    pub fn recipient_account_id(&self) -> &ParentAccountId {
        &self.recipient_account_id
    }

    pub fn target_role(&self) -> SetupInviteTargetRole {
        self.target_role
    }

    pub fn lease_expires_at(&self) -> &str {
        &self.lease_expires_at
    }

    pub fn attempt_id(&self) -> &str {
        &self.attempt_id
    }
}

impl fmt::Debug for InviteMembershipDeliveryAttempt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InviteMembershipDeliveryAttempt")
            .field("invite_id", &self.invite_id)
            .field("household_id", &self.household_id)
            .field("recipient_provider", &self.recipient_provider)
            .field("recipient_provider_subject", &"<redacted>")
            .field("recipient_account_id", &self.recipient_account_id)
            .field("target_role", &self.target_role)
            .field("attempt_id", &"<redacted>")
            .field("lease_expires_at", &self.lease_expires_at)
            .finish()
    }
}

impl fmt::Debug for InviteMembershipCommitReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("InviteMembershipCommitReceipt")
            .field("invite_id", &self.invite_id)
            .field("household_id", &self.household_id)
            .field("recipient_provider", &self.recipient_provider)
            .field("recipient_provider_subject", &"<redacted>")
            .field("recipient_account_id", &self.recipient_account_id)
            .field("target_role", &self.target_role)
            .field("attempt_id", &"<redacted>")
            .finish()
    }
}
