use rusqlite::Transaction;

use crate::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeError;

use super::{write, MAX_ADVANCE, MAX_BACKWARD_SKEW};

pub(super) fn advance(
    transaction: &Transaction<'_>,
    highest: i64,
    now: i64,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    if now.saturating_add(MAX_BACKWARD_SKEW) < highest {
        return write(transaction, now, false, Some(now));
    }
    if now > highest && now - highest > MAX_ADVANCE {
        // A `None` observation marks an epoch created by a forward anomaly.
        // It cannot authorize purges until the local clock falls below it and
        // starts a fresh, independently confirmable provisional epoch.
        return write(transaction, now, false, None);
    }
    write_confirmed_if_advanced(transaction, highest, now)
}

fn write_confirmed_if_advanced(
    transaction: &Transaction<'_>,
    highest: i64,
    now: i64,
) -> Result<i64, AuthenticatedDeliveryGrantConsumeError> {
    if now >= highest {
        return write(transaction, now, true, None);
    }
    Ok(highest)
}
