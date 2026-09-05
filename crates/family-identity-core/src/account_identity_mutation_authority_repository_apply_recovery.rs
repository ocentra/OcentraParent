use rusqlite::{params, Transaction};

use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::AccountIdentityMutationResult;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn apply(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<AccountIdentityMutationResult, AccountIdentityMutationAuthorityError> {
    let last_transition = transaction
        .query_row(
            "SELECT last_transition_at_epoch_millis
             FROM account_identity_recovery
             WHERE recovery_id = ?1 AND household_id = ?2 LIMIT 1",
            params![envelope.target_id, envelope.household_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::TargetNotCurrent)?;
    let transition_at = now.max(
        last_transition
            .checked_add(1)
            .ok_or(AccountIdentityMutationAuthorityError::ClockUnavailable)?,
    );
    let changed = transaction
        .execute(
            "UPDATE account_identity_recovery
             SET state = 'revoked', last_transition_at_epoch_millis = ?3
             WHERE recovery_id = ?1 AND household_id = ?2
               AND state IN ('owner-approval-required','approved')
               AND identity_proof_state = 'verified'
               AND identity_proof_expires_at_epoch_millis > ?3",
            params![envelope.target_id, envelope.household_id, transition_at],
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    if changed != 1 {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    transaction
        .execute(
            "DELETE FROM account_identity_recovery_custody_handoff
             WHERE recovery_id = ?1 AND household_id = ?2
               AND state IN ('pending','in-flight')",
            params![envelope.target_id, envelope.household_id],
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    let remaining = transaction
        .query_row(
            "SELECT COUNT(*) FROM account_identity_recovery_custody_handoff
             WHERE recovery_id = ?1 AND household_id = ?2",
            params![envelope.target_id, envelope.household_id],
            |row| row.get::<_, i64>(0),
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    if remaining != 0 {
        return Err(AccountIdentityMutationAuthorityError::TargetNotCurrent);
    }
    Ok(AccountIdentityMutationResult::RecoveryRevoked)
}
