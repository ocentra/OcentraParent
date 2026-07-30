use std::{thread, time::Duration};

use rusqlite::{Connection, Error as SqliteError, ErrorCode, Transaction, TransactionBehavior};

const TRANSACTION_ATTEMPTS: usize = 3;
const RETRY_BACKOFF: Duration = Duration::from_millis(25);

pub(super) fn immediate_transaction_with_contention_retry(
    connection: &Connection,
) -> Result<Transaction<'_>, SqliteError> {
    immediate_transaction_attempt(connection, TRANSACTION_ATTEMPTS)
}

fn immediate_transaction_attempt(
    connection: &Connection,
    remaining_attempts: usize,
) -> Result<Transaction<'_>, SqliteError> {
    match Transaction::new_unchecked(connection, TransactionBehavior::Immediate) {
        Ok(transaction) => Ok(transaction),
        Err(error) if is_sqlite_contention(&error) && remaining_attempts > 1 => {
            let completed_attempts = TRANSACTION_ATTEMPTS - remaining_attempts + 1;
            thread::sleep(RETRY_BACKOFF * completed_attempts as u32);
            immediate_transaction_attempt(connection, remaining_attempts - 1)
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
