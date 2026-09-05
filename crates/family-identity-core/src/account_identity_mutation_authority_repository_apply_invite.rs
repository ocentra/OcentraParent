use rusqlite::{params, Transaction};

use crate::account_identity_mutation_authority::envelope::CanonicalMutationEnvelope;
use crate::account_identity_mutation_authority::AccountIdentityMutationResult;
use crate::account_identity_mutation_authority_error::AccountIdentityMutationAuthorityError;

pub(super) fn apply(
    transaction: &Transaction<'_>,
    envelope: &CanonicalMutationEnvelope,
    now: i64,
) -> Result<AccountIdentityMutationResult, AccountIdentityMutationAuthorityError> {
    let changed = transaction
        .execute(
            "UPDATE account_identity_setup_invite
             SET state = 'revoked', revoked_at_epoch_millis = ?2
             WHERE invite_id = ?1 AND household_id = ?3 AND state = 'pending'
               AND expires_at_epoch_millis > ?2
               AND inviter_member_id = ?4
               AND (?5 = 'parent-owner' OR inviter_member_id = ?6)",
            params![
                envelope.target_id,
                now,
                envelope.household_id,
                envelope.target_owner_member_id,
                envelope.role,
                envelope.member_id,
            ],
        )
        .map_err(|_error| AccountIdentityMutationAuthorityError::RepositoryUnavailable)?;
    (changed == 1)
        .then_some(AccountIdentityMutationResult::SetupInviteRevoked)
        .ok_or(AccountIdentityMutationAuthorityError::TargetNotCurrent)
}
