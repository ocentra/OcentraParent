use rusqlite::{params, OptionalExtension, TransactionBehavior};

use crate::family_identity::SetupInviteId;
use crate::setup_lifecycle::SetupInviteTargetRole;

use super::authority::{timestamp, trusted_now_in_transaction};
use super::security_entropy::opaque_id;
use super::support_invite_identity::provider_label;
use super::support_invite_target_role::{target_role_from_label, target_role_label};
use super::{
    InviteMembershipCommitReceipt, InviteMembershipDeliveryAttempt, InviteRecoveryRepositoryError,
    SqliteAccountIdentityAuthorityRepository, VerifiedInviteRecipient, HANDOFF_LEASE_MILLIS,
};

struct PendingMembershipRow {
    invite_id: String,
    household_id: String,
    provider: String,
    provider_subject: String,
    account_id: String,
    target_role: SetupInviteTargetRole,
}

impl SqliteAccountIdentityAuthorityRepository {
    /// Claims one pending invite membership for the real membership owner.
    /// Expired in-flight leases are replayable after restart. This source seam
    /// is crate-private until Account has a provider/membership owner capable
    /// of producing the typed commit receipt.
    pub(crate) fn claim_pending_invite_membership(
        &mut self,
        recipient: &VerifiedInviteRecipient,
    ) -> Result<Option<InviteMembershipDeliveryAttempt>, InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        let lease_expires = now
            .checked_add(HANDOFF_LEASE_MILLIS)
            .ok_or(InviteRecoveryRepositoryError::HandoffConflict)?;
        let Some(row) = load_pending_membership(&transaction, recipient, now)? else {
            transaction
                .commit()
                .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
            return Ok(None);
        };
        let attempt_id = claim_membership(&transaction, &row, lease_expires, now)?;
        let attempt = build_attempt(recipient, row, attempt_id, lease_expires)?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(Some(attempt))
    }

    pub(crate) fn release_pending_invite_membership(
        &mut self,
        recipient: &VerifiedInviteRecipient,
        attempt: &InviteMembershipDeliveryAttempt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        validate_membership_attempt(recipient, attempt)?;
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_pending_invite_membership
                 SET state = 'pending', active_attempt_id = NULL,
                     lease_expires_at_epoch_millis = NULL
                 WHERE invite_id = ?1 AND household_id = ?2
                   AND recipient_provider = ?3 AND recipient_provider_subject = ?4
                   AND recipient_account_id = ?5 AND target_role = ?6
                   AND state = 'in-flight' AND active_attempt_id = ?7
                   AND lease_expires_at_epoch_millis > ?8",
                params![
                    attempt.invite_id.as_str(),
                    attempt.household_id.to_string(),
                    provider_label(&attempt.recipient_provider),
                    attempt.recipient_provider_subject.as_str(),
                    attempt.recipient_account_id.to_string(),
                    target_role_label(attempt.target_role),
                    attempt.attempt_id,
                    now,
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }

    pub(crate) fn acknowledge_pending_invite_membership(
        &mut self,
        recipient: &VerifiedInviteRecipient,
        attempt: &InviteMembershipDeliveryAttempt,
        receipt: &InviteMembershipCommitReceipt,
    ) -> Result<(), InviteRecoveryRepositoryError> {
        validate_membership_attempt(recipient, attempt)?;
        if receipt.invite_id != attempt.invite_id
            || receipt.household_id != attempt.household_id
            || receipt.recipient_provider != attempt.recipient_provider
            || receipt.recipient_provider_subject != attempt.recipient_provider_subject
            || receipt.recipient_account_id != attempt.recipient_account_id
            || receipt.target_role != attempt.target_role
            || receipt.attempt_id != attempt.attempt_id
        {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        let changed = transaction
            .execute(
                "UPDATE account_identity_pending_invite_membership
                 SET state = 'committed', active_attempt_id = NULL,
                     lease_expires_at_epoch_millis = NULL
                 WHERE invite_id = ?1 AND household_id = ?2
                   AND recipient_provider = ?3 AND recipient_provider_subject = ?4
                   AND recipient_account_id = ?5 AND target_role = ?6
                   AND state = 'in-flight' AND active_attempt_id = ?7
                   AND lease_expires_at_epoch_millis > ?8",
                params![
                    attempt.invite_id.as_str(),
                    attempt.household_id.to_string(),
                    provider_label(&attempt.recipient_provider),
                    attempt.recipient_provider_subject.as_str(),
                    attempt.recipient_account_id.to_string(),
                    target_role_label(attempt.target_role),
                    attempt.attempt_id,
                    now,
                ],
            )
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        if changed != 1 {
            return Err(InviteRecoveryRepositoryError::HandoffConflict);
        }
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
    }
}

fn load_pending_membership(
    transaction: &rusqlite::Transaction<'_>,
    recipient: &VerifiedInviteRecipient,
    now: i64,
) -> Result<Option<PendingMembershipRow>, InviteRecoveryRepositoryError> {
    transaction
        .query_row(
            "SELECT invite_id, household_id, recipient_provider,
                    recipient_provider_subject, recipient_account_id, target_role
             FROM account_identity_pending_invite_membership
             WHERE recipient_provider = ?1 AND recipient_provider_subject = ?2
               AND recipient_account_id = ?3
               AND (state = 'pending'
                    OR (state = 'in-flight' AND lease_expires_at_epoch_millis <= ?4))
             ORDER BY created_at_epoch_millis, invite_id LIMIT 1",
            params![
                provider_label(&recipient.provider),
                recipient.provider_subject.as_str(),
                recipient.account_id.to_string(),
                now,
            ],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                ))
            },
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
        .map(|row| {
            let target_role = target_role_from_label(&row.5)
                .ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
            Ok(PendingMembershipRow {
                invite_id: row.0,
                household_id: row.1,
                provider: row.2,
                provider_subject: row.3,
                account_id: row.4,
                target_role,
            })
        })
        .transpose()
}

fn claim_membership(
    transaction: &rusqlite::Transaction<'_>,
    row: &PendingMembershipRow,
    lease_expires: i64,
    now: i64,
) -> Result<String, InviteRecoveryRepositoryError> {
    let attempt_id = opaque_id("membership-attempt-")?;
    let changed = transaction
        .execute(
            "UPDATE account_identity_pending_invite_membership
             SET state = 'in-flight', active_attempt_id = ?2,
                 lease_expires_at_epoch_millis = ?3,
                 attempt_count = attempt_count + 1
             WHERE invite_id = ?1 AND recipient_provider = ?4
               AND recipient_provider_subject = ?5 AND recipient_account_id = ?6
               AND target_role = ?7
               AND (state = 'pending'
                    OR (state = 'in-flight' AND lease_expires_at_epoch_millis <= ?8))",
            params![
                row.invite_id,
                attempt_id,
                lease_expires,
                row.provider,
                row.provider_subject,
                row.account_id,
                target_role_label(row.target_role),
                now,
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    (changed == 1)
        .then_some(attempt_id)
        .ok_or(InviteRecoveryRepositoryError::HandoffConflict)
}

fn build_attempt(
    recipient: &VerifiedInviteRecipient,
    row: PendingMembershipRow,
    attempt_id: String,
    lease_expires: i64,
) -> Result<InviteMembershipDeliveryAttempt, InviteRecoveryRepositoryError> {
    let invite_id =
        SetupInviteId::parse(row.invite_id).map_err(InviteRecoveryRepositoryError::InvalidValue)?;
    let household_id = ocentra_schema::report_query_custody::FamilyId::parse(row.household_id)
        .ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    let recipient_account_id =
        ocentra_schema::report_query_custody::ParentAccountId::parse(row.account_id)
            .ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    Ok(InviteMembershipDeliveryAttempt {
        invite_id,
        household_id,
        recipient_provider: recipient.provider.clone(),
        recipient_provider_subject: recipient.provider_subject.clone(),
        recipient_account_id,
        target_role: row.target_role,
        attempt_id,
        lease_expires_at: timestamp(lease_expires)?,
    })
}

fn validate_membership_attempt(
    recipient: &VerifiedInviteRecipient,
    attempt: &InviteMembershipDeliveryAttempt,
) -> Result<(), InviteRecoveryRepositoryError> {
    if attempt.recipient_provider != recipient.provider
        || attempt.recipient_provider_subject != recipient.provider_subject
        || attempt.recipient_account_id != recipient.account_id
    {
        return Err(InviteRecoveryRepositoryError::HandoffConflict);
    }
    Ok(())
}
