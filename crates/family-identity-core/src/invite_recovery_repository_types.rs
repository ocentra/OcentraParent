use super::*;
impl fmt::Debug for IssuedSetupInvite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IssuedSetupInvite")
            .field("code", &"<redacted>")
            .field("purpose", &self.purpose)
            .field("target_role", &self.target_role)
            .field("expires_at", &self.expires_at)
            .finish()
    }
}
impl IssuedSetupInvite {
    pub fn code(&self) -> &SetupInviteCode {
        &self.code
    }
    pub fn purpose(&self) -> SetupInvitePurpose {
        self.purpose
    }
    pub fn target_role(&self) -> SetupInviteTargetRole {
        self.target_role
    }
    pub fn expires_at(&self) -> &str {
        &self.expires_at
    }
}
impl RedeemedSetupInvite {
    pub fn invite_id(&self) -> &SetupInviteId {
        &self.invite_id
    }
    pub fn household_id(&self) -> &FamilyId {
        &self.household_id
    }
    pub fn target_role(&self) -> SetupInviteTargetRole {
        self.target_role
    }
    pub fn accepted_at(&self) -> &str {
        &self.accepted_at
    }
    pub fn membership_handoff(&self) -> &InviteMembershipHandoff {
        &self.membership_handoff
    }
}
impl RecoveryCompletion {
    pub fn state(&self) -> RecoveryState {
        self.state
    }
    pub fn handoff_enqueued(&self) -> bool {
        self.handoff_enqueued
    }

    pub fn owner_effect(&self) -> RecoveryOwnerEffect {
        self.owner_effect
    }
}
impl fmt::Debug for RecoveryHandoffDeliveryAttempt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecoveryHandoffDeliveryAttempt")
            .field("handoff", &self.handoff)
            .field("attempt_id", &"<redacted>")
            .field("lease_expires_at", &self.lease_expires_at)
            .finish()
    }
}
impl RecoveryHandoffDeliveryAttempt {
    pub fn handoff(&self) -> &RecoveryCustodyHandoff {
        &self.handoff
    }
    pub fn lease_expires_at(&self) -> &str {
        &self.lease_expires_at
    }
}
