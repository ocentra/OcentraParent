use std::time::Duration;

use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::AccountIdentityRole;
use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};

use crate::family_identity::SetupInviteId;
use crate::setup_lifecycle::{SetupInvitePurpose, SetupInviteTargetRole};

use super::authority::{ensure_current_authority, timestamp, trusted_now_in_transaction};
use super::security_entropy::{digest_token, opaque_id};
use super::security_rate_invite::enforce_invite_rate_limit;
use super::support_invite_identity::{provider_label, role_label};
use super::support_invite_policy::{inviter_can_issue, purpose_matches_target_role};
use super::support_invite_purpose::purpose_label;
use super::support_invite_target_role::{target_role_from_label, target_role_label};
use super::{
    FamilyId, InviteMembershipHandoff, InviteRecoveryRepositoryError, IssuedSetupInvite,
    RedeemedSetupInvite, SetupInviteCode, VerifiedInviteRecipient, MAX_INVITE_TTL,
};
use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

impl SqliteAccountIdentityAuthorityRepository {
    pub(crate) fn issue_setup_invite(
        &mut self,
        authority: &VerifiedAccountIdentityAuthority,
        purpose: SetupInvitePurpose,
        target_role: SetupInviteTargetRole,
        recipient: &VerifiedInviteRecipient,
        ttl: Duration,
    ) -> Result<IssuedSetupInvite, InviteRecoveryRepositoryError> {
        if !valid_issue_request(authority, purpose, target_role, ttl) {
            return Err(InviteRecoveryRepositoryError::InvalidInvite);
        }
        let invite_id = SetupInviteId::parse(opaque_id("invite-")?)
            .map_err(InviteRecoveryRepositoryError::InvalidValue)?;
        let token = opaque_id("token-")?;
        let token_digest = digest_token(&token);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, _) = trusted_now_in_transaction(&transaction)?;
        ensure_current_authority(&transaction, authority, now)?;
        enforce_invite_rate_limit(
            &transaction,
            &format!(
                "{}:{}:{}:{}",
                authority.household_id(),
                provider_label(&recipient.provider),
                recipient.provider_subject.as_str(),
                recipient.account_id
            ),
            now,
        )?;
        let expires_at_epoch_millis = invite_expiry(now, ttl)?;
        let expires_at = timestamp(expires_at_epoch_millis)?;
        persist_setup_invite(
            &transaction,
            authority,
            purpose,
            target_role,
            recipient,
            &invite_id,
            &token_digest,
            now,
            expires_at_epoch_millis,
        )?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        Ok(IssuedSetupInvite {
            code: SetupInviteCode { invite_id, token },
            purpose,
            target_role,
            expires_at,
        })
    }

    pub(crate) fn redeem_setup_invite(
        &mut self,
        recipient: &VerifiedInviteRecipient,
        code: SetupInviteCode,
    ) -> Result<RedeemedSetupInvite, InviteRecoveryRepositoryError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, accepted_at) = trusted_now_in_transaction(&transaction)?;
        let row = load_invite_row(&transaction, &code)?;
        let target_role =
            target_role_from_label(&row.1).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
        validate_redeem_row(&row, recipient, now)?;
        accept_invite(&transaction, &code, now)?;
        insert_pending_membership(&transaction, &code, &row, target_role, now)?;
        enforce_invite_rate_limit(
            &transaction,
            &format!(
                "{}:{}:{}",
                provider_label(&recipient.provider),
                recipient.provider_subject.as_str(),
                recipient.account_id
            ),
            now,
        )?;
        transaction
            .commit()
            .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
        redeemed_invite(code, row, recipient, target_role, accepted_at)
    }
}

fn load_invite_row(
    transaction: &Transaction<'_>,
    code: &SetupInviteCode,
) -> Result<
    (String, String, String, i64, String, String, String, String),
    InviteRecoveryRepositoryError,
> {
    transaction
        .query_row(
            "SELECT household_id, target_role, state, expires_at_epoch_millis,
                        recipient_provider, recipient_provider_subject, recipient_account_id,
                        invitee_email_digest
                 FROM account_identity_setup_invite
                 WHERE invite_id = ?1 AND token_digest = ?2 LIMIT 1",
            params![code.invite_id().as_str(), digest_token(code.as_str())],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, String>(5)?,
                    row.get::<_, String>(6)?,
                    row.get::<_, String>(7)?,
                ))
            },
        )
        .optional()
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?
        .ok_or(InviteRecoveryRepositoryError::InviteRejected)
}

fn valid_issue_request(
    authority: &VerifiedAccountIdentityAuthority,
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
    ttl: Duration,
) -> bool {
    matches!(
        authority.role(),
        AccountIdentityRole::ParentOwner | AccountIdentityRole::CoParentGuardian
    ) && purpose_matches_target_role(purpose, target_role)
        && inviter_can_issue(authority.role(), purpose)
        && !ttl.is_zero()
        && ttl <= MAX_INVITE_TTL
}

fn invite_expiry(now: i64, ttl: Duration) -> Result<i64, InviteRecoveryRepositoryError> {
    let ttl_millis = i64::try_from(ttl.as_millis()).map_err(|_| {
        InviteRecoveryRepositoryError::InvalidValue(EventingError::InvalidValue {
            field: "family_identity.setup_invite.ttl",
            value: String::from("overflow"),
        })
    })?;
    now.checked_add(ttl_millis)
        .ok_or(InviteRecoveryRepositoryError::InvalidInvite)
}

fn persist_setup_invite(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
    recipient: &VerifiedInviteRecipient,
    invite_id: &SetupInviteId,
    token_digest: &str,
    now: i64,
    expires_at: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    transaction
        .execute(
            "INSERT INTO account_identity_setup_invite (
                 invite_id, token_digest, household_id, inviter_account_id,
                 inviter_member_id, inviter_device_id, inviter_authority_generation,
                 inviter_session_generation, inviter_role, purpose, target_role,
                 recipient_provider, recipient_provider_subject, recipient_account_id,
                 invitee_email_digest, issued_at_epoch_millis, expires_at_epoch_millis,
                 state, use_count
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, 'pending', 0)",
            params![
                invite_id.as_str(),
                token_digest,
                authority.household_id().to_string(),
                authority.account_id().to_string(),
                authority.member_id().as_str(),
                authority.device_id().as_str(),
                authority.authority_generation() as i64,
                authority.session_generation() as i64,
                role_label(authority.role()),
                purpose_label(purpose),
                target_role_label(target_role),
                provider_label(&recipient.provider),
                recipient.provider_subject.as_str(),
                recipient.account_id.to_string(),
                recipient.email_digest.as_str(),
                now,
                expires_at,
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
}

fn validate_redeem_row(
    row: &(String, String, String, i64, String, String, String, String),
    recipient: &VerifiedInviteRecipient,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    if row.2 != "pending"
        || row.3 <= now
        || row.4 != provider_label(&recipient.provider)
        || row.5 != recipient.provider_subject.as_str()
        || row.6 != recipient.account_id.to_string()
        || row.7 != recipient.email_digest
    {
        return Err(InviteRecoveryRepositoryError::InviteRejected);
    }
    Ok(())
}

fn accept_invite(
    transaction: &Transaction<'_>,
    code: &SetupInviteCode,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_setup_invite
             SET state = 'accepted', accepted_at_epoch_millis = ?3, use_count = 1
             WHERE invite_id = ?1 AND token_digest = ?2 AND state = 'pending'
               AND expires_at_epoch_millis > ?3 AND use_count = 0",
            params![code.invite_id().as_str(), digest_token(code.as_str()), now],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(InviteRecoveryRepositoryError::InviteRejected)
}

fn insert_pending_membership(
    transaction: &Transaction<'_>,
    code: &SetupInviteCode,
    row: &(String, String, String, i64, String, String, String, String),
    target_role: SetupInviteTargetRole,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    transaction
        .execute(
            "INSERT INTO account_identity_pending_invite_membership (
                 invite_id, household_id, recipient_provider,
                 recipient_provider_subject, recipient_account_id, target_role,
                 state, created_at_epoch_millis, active_attempt_id,
                 lease_expires_at_epoch_millis, attempt_count
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'pending', ?7, NULL, NULL, 0)",
            params![
                code.invite_id().as_str(),
                row.0,
                row.4,
                row.5,
                row.6,
                target_role_label(target_role),
                now,
            ],
        )
        .map_err(|_| InviteRecoveryRepositoryError::Unavailable)
}

fn redeemed_invite(
    code: SetupInviteCode,
    row: (String, String, String, i64, String, String, String, String),
    recipient: &VerifiedInviteRecipient,
    target_role: SetupInviteTargetRole,
    accepted_at: String,
) -> Result<RedeemedSetupInvite, InviteRecoveryRepositoryError> {
    let household_id =
        FamilyId::parse(row.0).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
    Ok(RedeemedSetupInvite {
        invite_id: code.invite_id().clone(),
        household_id: household_id.clone(),
        target_role,
        accepted_at,
        membership_handoff: InviteMembershipHandoff {
            invite_id: code.invite_id().clone(),
            household_id,
            recipient_provider: recipient.provider.clone(),
            recipient_provider_subject: recipient.provider_subject.clone(),
            recipient_account_id: recipient.account_id.clone(),
            target_role,
        },
    })
}
