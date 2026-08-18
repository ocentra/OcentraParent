use rusqlite::{ErrorCode, Transaction};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::ParentStorageConfirmationStoreError;

pub(super) fn ensure_current_authority(
    transaction: &Transaction<'_>,
    authority: &VerifiedAccountIdentityAuthority,
    now: i64,
) -> Result<(), ParentStorageConfirmationStoreError> {
    crate::invite_recovery_repository::authority::ensure_current_authority(
        transaction,
        authority,
        now,
    )
    .map_err(|error| match error {
        crate::invite_recovery_repository::InviteRecoveryRepositoryError::Unavailable => {
            ParentStorageConfirmationStoreError::Unavailable
        }
        crate::invite_recovery_repository::InviteRecoveryRepositoryError::ClockUnavailable => {
            ParentStorageConfirmationStoreError::ClockUnavailable
        }
        crate::invite_recovery_repository::InviteRecoveryRepositoryError::AuthorityUnavailable => {
            ParentStorageConfirmationStoreError::AccountAuthorityUnavailable
        }
        crate::invite_recovery_repository::InviteRecoveryRepositoryError::AuthorityExpired
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::AuthorityNotCurrent => {
            ParentStorageConfirmationStoreError::AccountAuthorityNotCurrent
        }
        crate::invite_recovery_repository::InviteRecoveryRepositoryError::EntropyUnavailable
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::InvalidValue(_)
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::InvalidInvite
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::InviteRejected
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::RecoveryRejected
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::Missing
        | crate::invite_recovery_repository::InviteRecoveryRepositoryError::HandoffConflict => {
            ParentStorageConfirmationStoreError::IntegrityRejected
        }
    })
}

pub(super) fn trusted_now(
    transaction: &Transaction<'_>,
) -> Result<(i64, String), ParentStorageConfirmationStoreError> {
    crate::invite_recovery_repository::authority::trusted_now_in_transaction(transaction).map_err(
        |error| {
            match error {
            crate::invite_recovery_repository::InviteRecoveryRepositoryError::ClockUnavailable => {
                ParentStorageConfirmationStoreError::ClockUnavailable
            }
            crate::invite_recovery_repository::InviteRecoveryRepositoryError::Unavailable
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::AuthorityUnavailable
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::AuthorityExpired
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::AuthorityNotCurrent
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::EntropyUnavailable
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::InvalidValue(_)
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::InvalidInvite
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::InviteRejected
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::RecoveryRejected
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::Missing
            | crate::invite_recovery_repository::InviteRecoveryRepositoryError::HandoffConflict => {
                ParentStorageConfirmationStoreError::Unavailable
            }
        }
        },
    )
}

pub(super) fn next_receipt_epoch(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<u64, ParentStorageConfirmationStoreError> {
    let previous = transaction
        .query_row(
            "SELECT max(receipt_epoch) FROM account_identity_parent_storage_confirmation",
            [],
            |row| row.get::<_, Option<i64>>(0),
        )
        .map_err(|_| ParentStorageConfirmationStoreError::Unavailable)?
        .unwrap_or(0);
    let next = now.max(
        previous
            .checked_add(1)
            .ok_or(ParentStorageConfirmationStoreError::ClockUnavailable)?,
    );
    u64::try_from(next).map_err(|_| ParentStorageConfirmationStoreError::ClockUnavailable)
}

pub(super) fn map_write_error(error: rusqlite::Error) -> ParentStorageConfirmationStoreError {
    if let rusqlite::Error::SqliteFailure(failure, _) = error {
        if failure.code == ErrorCode::ConstraintViolation {
            return ParentStorageConfirmationStoreError::Duplicate;
        }
    }
    ParentStorageConfirmationStoreError::Unavailable
}
