use std::{
    thread,
    time::{Duration, Instant},
};

use rusqlite::{Connection, Error as SqliteError, ErrorCode, Transaction, TransactionBehavior};

const CONTENTION_RETRY_DEADLINE: Duration = Duration::from_secs(5);
const RETRY_BACKOFF: Duration = Duration::from_millis(25);

pub(super) fn immediate_transaction_with_contention_retry(
    connection: &Connection,
) -> Result<Transaction<'_>, SqliteError> {
    immediate_transaction_attempt(connection, Instant::now() + CONTENTION_RETRY_DEADLINE)
}

fn immediate_transaction_attempt(
    connection: &Connection,
    deadline: Instant,
) -> Result<Transaction<'_>, SqliteError> {
    match Transaction::new_unchecked(connection, TransactionBehavior::Immediate) {
        Ok(transaction) => Ok(transaction),
        Err(error) if is_sqlite_contention(&error) && Instant::now() < deadline => {
            thread::sleep(RETRY_BACKOFF.min(deadline.saturating_duration_since(Instant::now())));
            immediate_transaction_attempt(connection, deadline)
        }
        Err(error) => Err(error),
    }
}

fn is_sqlite_contention(error: &SqliteError) -> bool {
    matches!(
        error.sqlite_error().map(|sqlite| sqlite.code),
        Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked)
    )
}
