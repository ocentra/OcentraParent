use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;
use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::ResolvedAccountIdentityMutationTarget;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;
use crate::family_identity::RecoveryId;

struct RecoveryTargetRow {
    household_id: String,
    requester_member_id: String,
    state: String,
    proof_expires_at: i64,
    proof_state: String,
    support_channel: String,
    support_id: Option<String>,
    support_issuer: Option<String>,
    support_scope: Option<String>,
    support_expires_at: Option<i64>,
}

pub(super) fn resolve(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    recovery_id: &RecoveryId,
    now: i64,
) -> Result<ResolvedAccountIdentityMutationTarget, AccountIdentityMutationAuthorityError> {
    if authority.role()
        != ocentra_schema::account_identity_authority::AccountIdentityRole::ParentOwner
    {
        return Err(AccountIdentityMutationAuthorityError::RoleNotAuthorized);
    }
    let row = load(transaction, recovery_id.as_str())?;
    validate_row(&row, &authority.household_id().to_string(), now)?;
    Ok(resolved(recovery_id.as_str(), row))
}

pub(super) fn validate_consumed(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if envelope.role != "parent-owner" {
        return Err(AccountIdentityMutationAuthorityError::RoleNotAuthorized);
    }
    let row = load(transaction, &envelope.target_id)?;
    validate_row(&row, &envelope.household_id, now)?;
    if row.household_id != envelope.target_household_id
        || row.requester_member_id != envelope.target_owner_member_id
        || row.state != envelope.target_state
        || row.proof_expires_at != envelope.target_expires_at_epoch_millis
        || row.support_channel != envelope.target_support_channel
        || row.support_id.as_deref().unwrap_or_default() != envelope.target_support_authorization_id
        || row.support_issuer.as_deref().unwrap_or_default()
            != envelope.target_support_authorization_issuer
        || row.support_scope.as_deref().unwrap_or_default()
            != envelope.target_support_authorization_scope
        || row.support_expires_at.unwrap_or_default()
            != envelope.target_support_authorization_expires_at_epoch_millis
    {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    Ok(())
}

fn validate_row(
    row: &RecoveryTargetRow,
    household_id: &str,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    if row.household_id != household_id
        || !["owner-approval-required", "approved"].contains(&row.state.as_str())
        || row.proof_state != "verified"
        || row.proof_expires_at <= now
    {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    if row.support_channel == "support-assisted" && !valid_support(row, now) {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    Ok(())
}

fn valid_support(row: &RecoveryTargetRow, now: i64) -> bool {
    row.support_id
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
        && row
            .support_issuer
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
        && row
            .support_scope
            .as_deref()
            .is_some_and(|value| ["household", "device-control"].contains(&value))
        && row.support_expires_at.is_some_and(|value| value > now)
}

fn load(
    transaction: &Transaction<'_>,
    recovery_id: &str,
) -> Result<RecoveryTargetRow, AccountIdentityMutationAuthorityError> {
    transaction
        .query_row(
            "SELECT household_id, requester_member_id, state,
                    identity_proof_expires_at_epoch_millis, identity_proof_state,
                    support_channel, support_authorization_id,
                    support_authorization_issuer, support_authorization_scope,
                    support_authorization_expires_at_epoch_millis
             FROM account_identity_recovery WHERE recovery_id = ?1 LIMIT 1",
            params![recovery_id],
            |row| {
                Ok(RecoveryTargetRow {
                    household_id: row.get(0)?,
                    requester_member_id: row.get(1)?,
                    state: row.get(2)?,
                    proof_expires_at: row.get(3)?,
                    proof_state: row.get(4)?,
                    support_channel: row.get(5)?,
                    support_id: row.get(6)?,
                    support_issuer: row.get(7)?,
                    support_scope: row.get(8)?,
                    support_expires_at: row.get(9)?,
                })
            },
        )
        .optional()
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?
        .ok_or(AccountIdentityMutationAuthorityError::TargetNotCurrent)
}

fn resolved(recovery_id: &str, row: RecoveryTargetRow) -> ResolvedAccountIdentityMutationTarget {
    ResolvedAccountIdentityMutationTarget {
        kind: "recovery".to_owned(),
        target_id: recovery_id.to_owned(),
        child_profile_id: String::new(),
        child_device_id: String::new(),
        household_id: row.household_id,
        owner_member_id: row.requester_member_id,
        state: row.state,
        expires_at_epoch_millis: row.proof_expires_at,
        support_channel: row.support_channel,
        support_authorization_id: row.support_id.unwrap_or_default(),
        support_authorization_issuer: row.support_issuer.unwrap_or_default(),
        support_authorization_scope: row.support_scope.unwrap_or_default(),
        support_authorization_expires_at_epoch_millis: row.support_expires_at.unwrap_or_default(),
    }
}
