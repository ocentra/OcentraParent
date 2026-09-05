use ocentra_schema::account_identity_authority::{
    AccountIdentityProvider, AccountIdentityProviderSubject,
};
use rusqlite::{params, OptionalExtension, Transaction};
use sha2::{Digest, Sha256};

use super::support_invite_identity::provider_label;
use super::InviteRecoveryRepositoryError;

pub(crate) fn enforce_recovery_rate_limit(
    transaction: &Transaction<'_>,
    provider: &AccountIdentityProvider,
    subject: &AccountIdentityProviderSubject,
    now: i64,
) -> Result<(), InviteRecoveryRepositoryError> {
    const WINDOW_MILLIS: i64 = 15 * 60 * 1_000;
    const MAX_ATTEMPTS: i64 = 5;
    let subject_digest = digest_subject(
        b"ocentra-account-recovery-rate-v1",
        &format!("{}:{}", provider_label(provider), subject.as_str()),
    );
    let existing = transaction
        .query_row(
            "SELECT window_started_at_epoch_millis, attempt_count
             FROM account_identity_recovery_rate_limit WHERE subject_digest = ?1",
            params![subject_digest],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)),
        )
        .optional()
        .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?;
    match existing {
        None => transaction
            .execute(
                "INSERT INTO account_identity_recovery_rate_limit
                 (subject_digest, window_started_at_epoch_millis, attempt_count)
                 VALUES (?1, ?2, 1)",
                params![subject_digest, now],
            )
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?,
        Some((window, _)) if elapsed_at_least(now, window, WINDOW_MILLIS) => transaction
            .execute(
                "UPDATE account_identity_recovery_rate_limit
                 SET window_started_at_epoch_millis = ?2, attempt_count = 1
                 WHERE subject_digest = ?1",
                params![subject_digest, now],
            )
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?,
        Some((_window, attempts)) if attempts >= MAX_ATTEMPTS => {
            return Err(InviteRecoveryRepositoryError::RecoveryRejected)
        }
        Some((_window, _attempts)) => transaction
            .execute(
                "UPDATE account_identity_recovery_rate_limit
                 SET attempt_count = attempt_count + 1 WHERE subject_digest = ?1",
                params![subject_digest],
            )
            .map_err(|_error| InviteRecoveryRepositoryError::Unavailable)?,
    };
    Ok(())
}

fn elapsed_at_least(now: i64, started: i64, window: i64) -> bool {
    now >= started
        && now
            .checked_sub(started)
            .is_some_and(|elapsed| elapsed >= window)
}

fn digest_subject(domain: &[u8], value: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(domain);
    digest.update(value.as_bytes());
    digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
