use rusqlite::{params, OptionalExtension, Transaction};

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

const MAX_ADVANCE: i64 = 366 * 24 * 60 * 60 * 1_000_000_000;

pub(super) fn advance(
    transaction: &Transaction<'_>,
    now: i64,
    independently_confirmed: bool,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let stored = transaction.query_row("SELECT highest_trusted_now_nanos, confirmed, provisional_observed_at_nanos FROM authenticated_delivery_grant_replay_retention_v3 WHERE singleton = 1", [], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, bool>(1)?, row.get::<_, Option<i64>>(2)?))).optional().map_err(storage)?;
    match stored {
        None => write(transaction, now, false, Some(now)),
        Some((highest, false, provisional_observed_at)) => provisional(
            transaction,
            highest,
            provisional_observed_at.unwrap_or(highest),
            now,
            independently_confirmed,
        ),
        Some((highest, true, _)) => confirmed(transaction, highest, now),
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

fn confirmed(
    transaction: &Transaction<'_>,
    highest: i64,
    now: i64,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let plausible = now >= highest && now - highest <= MAX_ADVANCE;
    let effective = if now > highest && !plausible {
        now
    } else {
        highest.max(now)
    };
    if effective == now && plausible {
        write(transaction, now, true, None)?;
    }
    Ok(effective)
}

fn provisional(
    transaction: &Transaction<'_>,
    highest: i64,
    provisional_observed_at: i64,
    now: i64,
    independently_confirmed: bool,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let plausible = now >= highest && now - highest <= MAX_ADVANCE;
    let independently_confirmed = plausible && independently_confirmed;
    let observed_at = if now < highest {
        now
    } else {
        provisional_observed_at
    };
    write(
        transaction,
        now,
        independently_confirmed,
        (!independently_confirmed).then_some(observed_at),
    )
}

fn write(
    transaction: &Transaction<'_>,
    now: i64,
    confirmed: bool,
    provisional_observed_at: Option<i64>,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    transaction.execute("INSERT INTO authenticated_delivery_grant_replay_retention_v3 (singleton, highest_trusted_now_nanos, confirmed, provisional_observed_at_nanos) VALUES (1, ?1, ?2, ?3) ON CONFLICT(singleton) DO UPDATE SET highest_trusted_now_nanos = excluded.highest_trusted_now_nanos, confirmed = excluded.confirmed, provisional_observed_at_nanos = excluded.provisional_observed_at_nanos", params![now, confirmed, provisional_observed_at]).and_then(|_| transaction.execute("INSERT INTO authenticated_delivery_grant_replay_retention_v1 (singleton, highest_trusted_now_nanos) VALUES (1, ?1) ON CONFLICT(singleton) DO UPDATE SET highest_trusted_now_nanos = excluded.highest_trusted_now_nanos", [now])).map(|_| now).map_err(storage)
}

fn storage(_: rusqlite::Error) -> AuthenticatedDeliveryGrantConsumeError {
    AuthenticatedDeliveryGrantConsumeError::StorageUnavailable
}
