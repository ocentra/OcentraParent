use chrono::DateTime;
use rusqlite::{params, OptionalExtension, Transaction};

use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::AccountIdentityMutationResult;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

// Completed outcomes remain retryable for thirty days after token expiry.
// Every consume transaction removes older rows, so replay custody is bounded
// by an explicit expiry window instead of growing without a lifecycle.
const EFFECT_RETENTION_MILLIS: i64 = 30 * 24 * 60 * 60 * 1_000;

struct StoredEffect {
    account_id: String,
    household_id: String,
    action: String,
    target_kind: String,
    target_id: String,
    idempotency_key: String,
    payload_digest: String,
    key_id: String,
    token_expires_at: i64,
    status: String,
    result_code: Option<String>,
}

pub(super) fn purge_expired(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    transaction
        .execute(
            "DELETE FROM account_identity_mutation_effect
             WHERE retain_until_epoch_millis <= ?1",
            params![now],
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    Ok(())
}

pub(super) fn recorded_result(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    digest: &str,
) -> Result<Option<AccountIdentityMutationResult>, AccountIdentityMutationAuthorityError> {
    let Some(row) = load(transaction, envelope, digest)? else {
        return Ok(None);
    };
    if !row.matches_scope(envelope) {
        return Err(AccountIdentityMutationAuthorityError::EffectStateInvalid);
    }
    if row.payload_digest != digest {
        return Err(AccountIdentityMutationAuthorityError::IdempotencyConflict);
    }
    if row.key_id != envelope.key_id || row.token_expires_at != token_expiry(envelope)? {
        return Err(AccountIdentityMutationAuthorityError::EffectStateInvalid);
    }
    if row.status != "completed" {
        return Err(AccountIdentityMutationAuthorityError::EffectPending);
    }
    row.result_code
        .as_deref()
        .and_then(AccountIdentityMutationResult::parse)
        .map(Some)
        .ok_or(AccountIdentityMutationAuthorityError::EffectStateInvalid)
}

pub(super) fn reserve(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    digest: &str,
    token_expires_at: i64,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let retain_until = token_expires_at
        .checked_add(EFFECT_RETENTION_MILLIS)
        .ok_or(AccountIdentityMutationAuthorityError::InvalidEnvelope)?;
    transaction
        .execute(
            "INSERT INTO account_identity_mutation_effect (
                account_id, household_id, action, target_kind, target_id,
                idempotency_key, payload_digest, key_id,
                token_expires_at_epoch_millis, status, result_code,
                created_at_epoch_millis, updated_at_epoch_millis,
                completed_at_epoch_millis, retain_until_epoch_millis
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,
                'pending', NULL, ?10, ?10, NULL, ?11
             )",
            params![
                envelope.account_id,
                envelope.household_id,
                envelope.action,
                envelope.target_kind,
                envelope.target_id,
                envelope.idempotency_key,
                digest,
                envelope.key_id,
                token_expires_at,
                now,
                retain_until,
            ],
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    Ok(())
}

pub(super) fn complete(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    result: AccountIdentityMutationResult,
    now: i64,
) -> Result<(), AccountIdentityMutationAuthorityError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_mutation_effect
             SET status = 'completed', result_code = ?7,
                 updated_at_epoch_millis = ?8, completed_at_epoch_millis = ?8
             WHERE account_id = ?1 AND household_id = ?2 AND action = ?3
               AND target_kind = ?4 AND target_id = ?5 AND idempotency_key = ?6
               AND status = 'pending' AND result_code IS NULL
               AND completed_at_epoch_millis IS NULL",
            params![
                envelope.account_id,
                envelope.household_id,
                envelope.action,
                envelope.target_kind,
                envelope.target_id,
                envelope.idempotency_key,
                result.as_str(),
                now,
            ],
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    (changed == 1)
        .then_some(())
        .ok_or(AccountIdentityMutationAuthorityError::EffectStateInvalid)
}

fn load(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    digest: &str,
) -> Result<Option<StoredEffect>, AccountIdentityMutationAuthorityError> {
    transaction
        .query_row(
            "SELECT account_id, household_id, action, target_kind, target_id,
                    idempotency_key, payload_digest, key_id,
                    token_expires_at_epoch_millis, status, result_code
             FROM account_identity_mutation_effect
             WHERE payload_digest = ?1 OR (
                account_id = ?2 AND household_id = ?3 AND action = ?4
                AND target_kind = ?5 AND target_id = ?6 AND idempotency_key = ?7
             ) LIMIT 1",
            params![
                digest,
                envelope.account_id,
                envelope.household_id,
                envelope.action,
                envelope.target_kind,
                envelope.target_id,
                envelope.idempotency_key,
            ],
            |row| {
                Ok(StoredEffect {
                    account_id: row.get(0)?,
                    household_id: row.get(1)?,
                    action: row.get(2)?,
                    target_kind: row.get(3)?,
                    target_id: row.get(4)?,
                    idempotency_key: row.get(5)?,
                    payload_digest: row.get(6)?,
                    key_id: row.get(7)?,
                    token_expires_at: row.get(8)?,
                    status: row.get(9)?,
                    result_code: row.get(10)?,
                })
            },
        )
        .optional()
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)
}

fn token_expiry(
    envelope: &CanonicalMutationEnvelope,
) -> Result<i64, AccountIdentityMutationAuthorityError> {
    DateTime::parse_from_rfc3339(&envelope.expires_at)
        .map(|value| value.timestamp_millis())
        .map_err(|_error| AccountIdentityMutationAuthorityError::InvalidEnvelope)
}

impl StoredEffect {
    fn matches_scope(&self, envelope: &CanonicalMutationEnvelope) -> bool {
        self.account_id == envelope.account_id
            && self.household_id == envelope.household_id
            && self.action == envelope.action
            && self.target_kind == envelope.target_kind
            && self.target_id == envelope.target_id
            && self.idempotency_key == envelope.idempotency_key
    }
}
