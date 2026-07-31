use std::{thread, time::Duration};

use ed25519_dalek::SigningKey;
use ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeOutcome;
use rusqlite::Connection;

use super::{
    expected, must, open, signed_grant, store_path, trusted_issuer, TestResult, DELIVERED_PAYLOAD,
};

#[test]
fn consumer_retries_a_held_sqlite_write_lock_beyond_the_legacy_retry_window() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("held-write-lock-consume");
    let grant = signed_grant(&key);
    let expected = expected();
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let (lock_ready, lock_acquired) = std::sync::mpsc::sync_channel(1);
    let lock_path = path;
    let lock_holder = thread::spawn(move || -> TestResult {
        let mut connection = Connection::open(lock_path.as_ref())?;
        let transaction =
            connection.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        lock_ready
            .send(())
            .map_err(|_error| std::io::Error::other("write-lock readiness receiver dropped"))?;
        thread::sleep(Duration::from_millis(150));
        transaction.commit()?;
        Ok(())
    });
    lock_acquired.recv().map_err(|_error| {
        std::io::Error::other("write-lock holder exited before acquiring lock")
    })?;
    let outcome = must(consumer.consume(&grant, &expected, DELIVERED_PAYLOAD, "held-write-lock"))?;
    must(
        lock_holder
            .join()
            .map_err(|_error| std::io::Error::other("write-lock holder panicked"))?,
    )?;
    assert!(matches!(
        outcome,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    Ok(())
}
