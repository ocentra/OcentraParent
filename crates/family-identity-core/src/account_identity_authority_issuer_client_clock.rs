use chrono::DateTime;
use rusqlite::Transaction;

use super::AccountIdentityAuthorityIssuerClientError;

/// Use the Account repository's durable monotonic runtime clock.  This keeps
/// issuer verification, expiry, lease recovery, and receipt transitions on
/// one trusted SQLite time source rather than accepting a caller timestamp.
pub(super) fn now(
    transaction: &Transaction<'_>,
) -> Result<(i64, String), AccountIdentityAuthorityIssuerClientError> {
    crate::account_identity_authority_repository::trusted_runtime_now_in_transaction(transaction)
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::ClockUnavailable)
}

pub(super) fn parse_timestamp(
    value: &str,
) -> Result<DateTime<chrono::Utc>, AccountIdentityAuthorityIssuerClientError> {
    DateTime::parse_from_rfc3339(value)
        .map(|parsed| parsed.with_timezone(&chrono::Utc))
        .map_err(|_error| AccountIdentityAuthorityIssuerClientError::InvalidSchema)
}
