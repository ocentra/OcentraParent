use rusqlite::{params, OptionalExtension, Transaction};

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

const MAX_ADVANCE: i64 = 366 * 24 * 60 * 60 * 1_000_000_000;

pub(super) fn advance(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    let stored = transaction.query_row("SELECT highest_trusted_now_nanos, confirmed FROM authenticated_delivery_grant_replay_retention_v2 WHERE singleton = 1", [], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, bool>(1)?))).optional().map_err(storage)?;
    match stored {
        None => write(transaction, now, false),
        Some((highest, false)) => write(
            transaction,
            now,
            now >= highest && now - highest <= MAX_ADVANCE,
        ),
        Some((highest, true)) => confirmed(transaction, highest, now),
    }
}

pub(super) fn confirmed_purge_cutoff(
    transaction: &Transaction<'_>,
    now: i64,
) -> Result<Option<i64>, AuthenticatedDeliveryGrantConsumeError> {
    transaction
        .query_row(
            "SELECT highest_trusted_now_nanos, confirmed FROM authenticated_delivery_grant_replay_retention_v2 WHERE singleton = 1",
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
        write(transaction, now, true)?;
    }
    Ok(effective)
}

fn write(
    transaction: &Transaction<'_>,
    now: i64,
    confirmed: bool,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    transaction.execute("INSERT INTO authenticated_delivery_grant_replay_retention_v2 (singleton, highest_trusted_now_nanos, confirmed) VALUES (1, ?1, ?2) ON CONFLICT(singleton) DO UPDATE SET highest_trusted_now_nanos = excluded.highest_trusted_now_nanos, confirmed = excluded.confirmed", params![now, confirmed]).and_then(|_| transaction.execute("INSERT INTO authenticated_delivery_grant_replay_retention_v1 (singleton, highest_trusted_now_nanos) VALUES (1, ?1) ON CONFLICT(singleton) DO UPDATE SET highest_trusted_now_nanos = excluded.highest_trusted_now_nanos", [now])).map(|_| now).map_err(storage)
}

fn storage(_: rusqlite::Error) -> AuthenticatedDeliveryGrantConsumeError {
    AuthenticatedDeliveryGrantConsumeError::StorageUnavailable
}
