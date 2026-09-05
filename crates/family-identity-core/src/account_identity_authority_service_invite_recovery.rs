use std::time::Duration;

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::family_identity::RecoveryId;
use crate::setup_lifecycle::{SetupInvitePurpose, SetupInviteTargetRole};

use super::invite_recovery_repository::{
    DeviceTrustReinstallOwnerReceipt, DeviceTrustRevokeOwnerReceipt,
    HouseholdAuthorityMutationOwnerReceipt, InviteMembershipCommitReceipt,
    InviteMembershipDeliveryAttempt, InviteRecoveryRepositoryError, IssuedSetupInvite,
    ProviderCredentialSessionOwnerReceipt, RecoveryCompletion, RecoveryHandoffDeliveryAttempt,
    RedeemedSetupInvite, SetupInviteCode, VerifiedInviteRecipient, VerifiedRecoveryIdentityProof,
    VerifiedSupportRecoveryAuthorization,
};
use super::AccountIdentityAuthorityService;

impl AccountIdentityAuthorityService {
    /// Persist an invite only after both the inviter authority and the
    /// recipient identity have been verified by their owning boundaries.
    /// Neither opaque input can be constructed from a transport DTO.
    pub fn issue_setup_invite(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        purpose: SetupInvitePurpose,
        target_role: SetupInviteTargetRole,
        recipient: &VerifiedInviteRecipient,
        ttl: Duration,
    ) -> Result<IssuedSetupInvite, InviteRecoveryRepositoryError> {
        self.repository
            .issue_setup_invite(authority, purpose, target_role, recipient, ttl)
    }

    /// Atomically redeem a single-use invite for a verified recipient. The
    /// returned membership handoff is evidence for the membership owner; it
    /// does not itself grant household membership.
    pub fn redeem_setup_invite(
        &mut self,
        recipient: &VerifiedInviteRecipient,
        code: SetupInviteCode,
    ) -> Result<RedeemedSetupInvite, InviteRecoveryRepositoryError> {
        self.repository.redeem_setup_invite(recipient, code)
    }

    /// Claim a pending membership handoff for the membership owner. The
    /// opaque recipient proof keeps the claim bound to the verified account.
    pub fn claim_pending_invite_membership(
        &mut self,
        recipient: &VerifiedInviteRecipient,
    ) -> Result<Option<InviteMembershipDeliveryAttempt>, InviteRecoveryRepositoryError> {
        self.repository.claim_pending_invite_membership(recipient)
    }

    pub fn release_pending_invite_membership(
        &mut self,
        recipient: &VerifiedInviteRecipient,
        attempt: &InviteMembershipDeliveryAttempt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository
            .release_pending_invite_membership(recipient, attempt)
    }

    pub fn acknowledge_pending_invite_membership(
        &mut self,
        recipient: &VerifiedInviteRecipient,
        attempt: &InviteMembershipDeliveryAttempt,
        receipt: &InviteMembershipCommitReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository
            .acknowledge_pending_invite_membership(recipient, attempt, receipt)
    }

    /// Begin recovery from owner-verified identity evidence. Serialized
    /// callers cannot mint either proof type.
    pub fn begin_recovery(
        &mut self,
        proof: &VerifiedRecoveryIdentityProof,
        support_authorization: Option<&VerifiedSupportRecoveryAuthorization>,
    ) -> Result<RecoveryId, InviteRecoveryRepositoryError> {
        self.repository.begin_recovery(proof, support_authorization)
    }

    pub fn approve_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository.approve_recovery(authority, recovery_id)
    }

    pub fn complete_recovery(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        recovery_id: &RecoveryId,
    ) -> Result<RecoveryCompletion, InviteRecoveryRepositoryError> {
        self.repository.complete_recovery(authority, recovery_id)
    }

    pub fn claim_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
    ) -> Result<Option<RecoveryHandoffDeliveryAttempt>, InviteRecoveryRepositoryError> {
        self.repository.claim_recovery_handoff(authority)
    }

    pub fn release_recovery_handoff(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository.release_recovery_handoff(authority, attempt)
    }

    pub fn acknowledge_provider_credential_session(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &ProviderCredentialSessionOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository
            .acknowledge_provider_credential_session(authority, attempt, receipt)
    }

    pub fn acknowledge_device_trust_revoke(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &DeviceTrustRevokeOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository
            .acknowledge_device_trust_revoke(authority, attempt, receipt)
    }

    pub fn acknowledge_device_trust_reinstall(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &DeviceTrustReinstallOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository
            .acknowledge_device_trust_reinstall(authority, attempt, receipt)
    }

    pub fn acknowledge_household_authority_mutation(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        attempt: &RecoveryHandoffDeliveryAttempt,
        receipt: &HouseholdAuthorityMutationOwnerReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        self.repository
            .acknowledge_household_authority_mutation(authority, attempt, receipt)
    }
}
