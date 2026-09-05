use std::time::Duration;

use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::AccountIdentityRole;
use rusqlite::{params, OptionalExtension, Transaction, TransactionBehavior};

use crate::account_identity_authority_repository::SqliteAccountIdentityAuthorityRepository;
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
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
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
            &SetupInvitePersistence {
                authority,
                purpose,
                target_role,
                recipient,
                invite_id: &invite_id,
                token_digest: &token_digest,
                issued_at: now,
                expires_at: expires_at_epoch_millis,
            },
        )?;
        transaction
            .commit()
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
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
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
        let (now, accepted_at) = trusted_now_in_transaction(&transaction)?;
        let row = load_invite_row(&transaction, &code)?;
        let target_role = target_role_from_label(&row.target_role)
            .ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
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
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
        let redeemed = redeemed_invite(&code, row, recipient, target_role, accepted_at);
        drop(code);
        redeemed
    }
}

struct InviteRow {
    household_id: String,
    target_role: String,
    state: String,
    expires_at: i64,
    provider: String,
    provider_subject: String,
    account_id: String,
    email_digest: String,
}

fn load_invite_row(
    transaction: &Transaction<'_>,
    code: &SetupInviteCode,
) -> Result<InviteRow, InviteRecoveryRepositoryError> {
    transaction
        .query_row(
            "SELECT household_id, target_role, state, expires_at_epoch_millis,
                        recipient_provider, recipient_provider_subject, recipient_account_id,
                        invitee_email_digest
                 FROM account_identity_setup_invite
                 WHERE invite_id = ?1 AND token_digest = ?2 LIMIT 1",
            params![code.invite_id().as_str(), digest_token(code.as_str())],
            |row| {
                Ok(InviteRow {
                    household_id: row.get::<_, String>(0)?,
                    target_role: row.get::<_, String>(1)?,
                    state: row.get::<_, String>(2)?,
                    expires_at: row.get::<_, i64>(3)?,
                    provider: row.get::<_, String>(4)?,
                    provider_subject: row.get::<_, String>(5)?,
                    account_id: row.get::<_, String>(6)?,
                    email_digest: row.get::<_, String>(7)?,
                })
            },
        )
        .optional()
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?
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
    let ttl_millis = i64::try_from(ttl.as_millis()).map_err(|_error| {
        InviteRecoveryRepositoryError::InvalidValue(EventingError::InvalidValue {
            field: "family_identity.setup_invite.ttl",
            value: String::from("overflow"),
        })
    })?;
    now.checked_add(ttl_millis)
        .ok_or(InviteRecoveryRepositoryError::InvalidInvite)
}

struct SetupInvitePersistence<'a> {
    authority: &'a VerifiedAccountIdentityAuthority,
    purpose: SetupInvitePurpose,
    target_role: SetupInviteTargetRole,
    recipient: &'a VerifiedInviteRecipient,
    invite_id: &'a SetupInviteId,
    token_digest: &'a str,
    issued_at: i64,
    expires_at: i64,
}

fn persist_setup_invite(
    transaction: &Transaction<'_>,
    input: &SetupInvitePersistence<'_>,
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
                input.invite_id.as_str(),
                input.token_digest,
                input.authority.household_id().to_string(),
                input.authority.account_id().to_string(),
                input.authority.member_id().as_str(),
                input.authority.device_id().as_str(),
                input.authority.authority_generation() as i64,
                input.authority.session_generation() as i64,
                role_label(input.authority.role()),
                purpose_label(input.purpose),
                target_role_label(input.target_role),
                provider_label(&input.recipient.provider),
                input.recipient.provider_subject.as_str(),
                input.recipient.account_id.to_string(),
                input.recipient.email_digest.as_str(),
                input.issued_at,
                input.expires_at,
            ],
        )
        .map(|_| ())
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)
}

fn validate_redeem_row(
    row: &InviteRow,
    recipient: &VerifiedInviteRecipient,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    if row.state != "pending"
        || row.expires_at <= now
        || row.provider != provider_label(&recipient.provider)
        || row.provider_subject != recipient.provider_subject.as_str()
        || row.account_id != recipient.account_id.to_string()
        || row.email_digest != recipient.email_digest
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
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(InviteRecoveryRepositoryError::InviteRejected)
}

fn insert_pending_membership(
    transaction: &Transaction<'_>,
    code: &SetupInviteCode,
    row: &InviteRow,
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
                row.household_id,
                row.provider,
                row.provider_subject,
                row.account_id,
                target_role_label(target_role),
                now,
            ],
        )
        .map(|_| ())
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)
}

fn redeemed_invite(
    code: &SetupInviteCode,
    row: InviteRow,
    recipient: &VerifiedInviteRecipient,
    target_role: SetupInviteTargetRole,
    accepted_at: String,
) -> Result<RedeemedSetupInvite, InviteRecoveryRepositoryError> {
    let household_id =
        FamilyId::parse(row.household_id).ok_or(InviteRecoveryRepositoryError::InvalidInvite)?;
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
