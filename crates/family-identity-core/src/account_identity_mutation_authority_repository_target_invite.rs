use ocentra_schema::account_identity_authority::AccountIdentityRole;
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::ResolvedAccountIdentityMutationTarget;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use crate::family_identity::SetupInviteId;

struct InviteTargetRow {
    household_id: String,
    inviter_member_id: String,
    state: String,
    expires_at_epoch_millis: i64,
}

pub(super) fn resolve(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    invite_id: &SetupInviteId,
    now: i64,
) -> Result<ResolvedAccountIdentityMutationTarget, AccountIdentityMutationAuthorityError> {
    let row = load(transaction, invite_id.as_str())?;
    if row.household_id != authority.household_id().to_string()
        || row.state != "pending"
        || row.expires_at_epoch_millis <= now
        || (authority.role() != AccountIdentityRole::ParentOwner
            && row.inviter_member_id != authority.member_id().as_str())
    {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    Ok(resolved(invite_id.as_str(), row))
}

pub(super) fn validate_consumed(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let row = load(transaction, &envelope.target_id)?;
    if row.household_id != envelope.target_household_id
        || row.household_id != envelope.household_id
        || row.inviter_member_id != envelope.target_owner_member_id
        || row.state != envelope.target_state
        || row.state != "pending"
        || row.expires_at_epoch_millis != envelope.target_expires_at_epoch_millis
        || row.expires_at_epoch_millis <= now
        || (envelope.role != "parent-owner" && row.inviter_member_id != envelope.member_id)
    {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    Ok(())
}

fn load(
    transaction: &Transaction<'_>,
    invite_id: &str,
) -> Result<InviteTargetRow, AccountIdentityMutationAuthorityError> {
    transaction
        .query_row(
            "SELECT household_id, inviter_member_id, state, expires_at_epoch_millis
             FROM account_identity_setup_invite WHERE invite_id = ?1 LIMIT 1",
            params![invite_id],
            |row| {
                Ok(InviteTargetRow {
                    household_id: row.get(0)?,
                    inviter_member_id: row.get(1)?,
                    state: row.get(2)?,
                    expires_at_epoch_millis: row.get(3)?,
                })
            },
        )
        .optional()
        .map_err(|_| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?
        .ok_or(AccountIdentityMutationAuthorityError::TargetNotCurrent)
}

fn resolved(invite_id: &str, row: InviteTargetRow) -> ResolvedAccountIdentityMutationTarget {
    ResolvedAccountIdentityMutationTarget {
        kind: "setup-invite".to_owned(),
        target_id: invite_id.to_owned(),
        child_profile_id: String::new(),
        child_device_id: String::new(),
        household_id: row.household_id,
        owner_member_id: row.inviter_member_id,
        state: row.state,
        expires_at_epoch_millis: row.expires_at_epoch_millis,
        support_channel: String::new(),
        support_authorization_id: String::new(),
        support_authorization_issuer: String::new(),
        support_authorization_scope: String::new(),
        support_authorization_expires_at_epoch_millis: 0,
    }
}
