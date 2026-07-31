use rusqlite::{params, OptionalExtension, Transaction};

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

use super::confirmation;

mod confirmed_epoch;

const MAX_ADVANCE: i64 = 366 * 24 * 60 * 60 * 1_000_000_000;
const MAX_BACKWARD_SKEW: i64 = 5 * 60 * 1_000_000_000;

pub(super) fn advance(
    transaction: &Transaction<'_>,
    now: i64,
    authenticated_issued_at_nanos: Option<i64>,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let stored = transaction.query_row("SELECT highest_trusted_now_nanos, confirmed, provisional_observed_at_nanos FROM authenticated_delivery_grant_replay_retention_v3 WHERE singleton = 1", [], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, bool>(1)?, row.get::<_, Option<i64>>(2)?))).optional().map_err(storage)?;
    match stored {
        None => write(transaction, now, false, Some(now), now),
        Some((highest, false, provisional_observed_at)) => provisional(
            transaction,
            highest,
            provisional_observed_at,
            now,
            authenticated_issued_at_nanos,
        ),
        Some((highest, true, _)) => confirmed_epoch::advance(transaction, highest, now),
    }
}

pub(super) fn confirmed_purge_cutoff(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<Option<i64>, AuthenticatedDeliveryGrantConsumeError> {
    transaction
        .query_row(
            "SELECT highest_trusted_now_nanos, confirmed FROM authenticated_delivery_grant_replay_retention_v3 WHERE singleton = 1",
            [],
            |row| Ok((row.get::<_, i64>(0)?, row.get::<_, bool>(1)?)),
        )
        .optional()
        .map_err(storage)
        .and_then(|clock| match clock {
            Some((highest, true)) => Ok(Some(highest.min(now))),
            Some((_highest, false)) => Ok(None),
            None => Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected),
        })
}

fn provisional(
    transaction: &Transaction<'_>,
    highest: i64,
    provisional_observed_at: Option<i64>,
    now: i64,
    authenticated_issued_at_nanos: Option<i64>,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let plausible = now >= highest && now - highest <= MAX_ADVANCE;
    // A second read of the local wall clock cannot establish that the first
    // read was trustworthy. Confirmation instead requires the `issued_at`
    // timestamp from a grant whose signature, bindings, and temporal window
    // have already been verified by the caller. That timestamp is authored by
    // the trusted issuer, not this device's clock.
    let independently_confirmed = provisional_observed_at.is_some()
        && plausible
        && confirmation::is_current(authenticated_issued_at_nanos, now, highest);
    let observed_at = provisional_observed_at
        .map(|observed_at| confirmation::provisional_observed_at(now, highest, observed_at));
    write(
        transaction,
        now,
        independently_confirmed,
        (!independently_confirmed).then_some(observed_at).flatten(),
        now,
    )
}

fn write(
    transaction: &Transaction<'_>,
    highest: i64,
    confirmed: bool,
    provisional_observed_at: Option<i64>,
    effective_now: i64,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    transaction.execute("INSERT INTO authenticated_delivery_grant_replay_retention_v3 (singleton, highest_trusted_now_nanos, confirmed, provisional_observed_at_nanos) VALUES (1, ?1, ?2, ?3) ON CONFLICT(singleton) DO UPDATE SET highest_trusted_now_nanos = excluded.highest_trusted_now_nanos, confirmed = excluded.confirmed, provisional_observed_at_nanos = excluded.provisional_observed_at_nanos", params![highest, confirmed, provisional_observed_at]).and_then(|_| transaction.execute("INSERT INTO authenticated_delivery_grant_replay_retention_v1 (singleton, highest_trusted_now_nanos) VALUES (1, ?1) ON CONFLICT(singleton) DO UPDATE SET highest_trusted_now_nanos = excluded.highest_trusted_now_nanos", [highest])).map(|_| effective_now).map_err(storage)
}

fn storage(_: rusqlite::Error) -> AuthenticatedDeliveryGrantConsumeError {
    AuthenticatedDeliveryGrantConsumeError::StorageUnavailable
}
