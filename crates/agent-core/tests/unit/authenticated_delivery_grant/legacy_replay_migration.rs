use std::{thread, time::Duration};

use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantConsumer,
};
use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;
use ocentra_schema::authenticated_delivery_grant::{
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
};
use rusqlite::{params, Connection};

use super::storage_keys::stored_key;
use super::{must, signed_grant, store_path, trusted_issuer, TestResult};

fn legacy_grant_json(grant: &AuthenticatedDeliveryGrant, key: &SigningKey) -> TestResult<String> {
    let mut legacy = serde_json::to_value(grant)?;
    legacy["schemaVersion"] = serde_json::json!(1);
    legacy
        .as_object_mut()
        .ok_or_else(|| std::io::Error::other("grant must serialize as an object"))?
        .remove("payloadLength");
    legacy["signature"] =
        serde_json::json!(key.sign(&legacy_signing_bytes(grant)).to_bytes().to_vec());
    Ok(serde_json::to_string(&legacy)?)
}

fn legacy_signing_bytes(grant: &AuthenticatedDeliveryGrant) -> Vec<u8> {
    let mut bytes = Vec::new();
    let dry_run = grant.dry_run.to_string();
    for value in [
        "1",
        grant.issuer_key_id.as_str(),
        grant.issuer_actor_id.as_str(),
        grant.household_id.as_str(),
        grant.parent_device_id.as_str(),
        grant.child_profile_id.as_str(),
        grant.target_device_id.as_str(),
        grant.policy_decision_id.as_str(),
        grant.policy_version.as_str(),
        grant.action_id.as_str(),
        grant.capability_id.as_str(),
        grant.evidence_digest.as_str(),
        grant.payload_digest.as_str(),
        dry_run.as_str(),
        grant.nonce.as_str(),
        grant.issued_at.as_str(),
        grant.expires_at.as_str(),
        grant.revocation_version.as_str(),
    ] {
        bytes.extend_from_slice(&(value.len() as u64).to_be_bytes());
        bytes.extend_from_slice(value.as_bytes());
    }
    bytes
}

#[test]
fn consumer_backfills_legacy_microsecond_rows_from_signed_grant_nanos() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("legacy-microsecond-backfill");
    let mut grant = signed_grant(&key);
    grant.expires_at = "2026-07-28T00:05:00.000000001Z".to_owned();
    let grant_json = legacy_grant_json(&grant, &key)?;
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![grant.issuer_key_id, grant.nonce, grant_json, "{}", 1_i64],
    )?;
    drop(connection);
    drop(super::open(&path, trusted_issuer(&key))?);
    let connection = Connection::open(path.as_ref())?;
    let stored_nanos: i64 = connection.query_row(
        "SELECT expires_at_nanos FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [stored_key(&grant.issuer_key_id), stored_key(&grant.nonce)],
        |row| row.get(0),
    )?;
    assert_eq!(stored_nanos.rem_euclid(1_000), 1);
    Ok(())
}

#[test]
fn consumer_preserves_legacy_replay_tombstone_across_trusted_issuer_rotation() -> TestResult {
    let retired_key = SigningKey::from_bytes(&[4; 32]);
    let current_key = SigningKey::from_bytes(&[5; 32]);
    let path = store_path("legacy-replay-issuer-rotation");
    let mut retired_grant = signed_grant(&retired_key);
    retired_grant.issuer_key_id = "parent-key-retired".to_owned();
    let grant_json = legacy_grant_json(&retired_grant, &retired_key)?;
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            retired_grant.issuer_key_id,
            retired_grant.nonce,
            grant_json,
            "{}",
            1_i64
        ],
    )?;
    drop(connection);

    let current_issuer = ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantTrustedIssuer {
        key_id: "parent-key-current".to_owned(),
        verifying_key: current_key.verifying_key(),
    };
    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            current_issuer.clone(),
            "2026-07-28T00:01:00Z",
        ),
    )?);
    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            current_issuer,
            "2026-07-28T00:01:00Z",
        ),
    )?);

    let connection = Connection::open(path.as_ref())?;
    let (fingerprint, expires_at_nanos): (String, i64) = connection.query_row(
        "SELECT grant_fingerprint, expires_at_nanos FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [stored_key(&retired_grant.issuer_key_id), stored_key(&retired_grant.nonce)],
        |row| Ok((row.get(0)?, row.get(1)?)),
    )?;
    assert_eq!(fingerprint.len(), 64);
    assert_eq!(expires_at_nanos, 1_785_197_100_000_000_000);
    Ok(())
}

#[test]
fn concurrent_open_migrates_the_legacy_schema_once() -> TestResult {
    let key = SigningKey::from_bytes(&[6; 32]);
    let path = store_path("concurrent-privacy-marker-migration");
    let grant = signed_grant(&key);
    let grant_json = legacy_grant_json(&grant, &key)?;
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![grant.issuer_key_id, grant.nonce, grant_json, "{}", 1_i64],
    )?;
    drop(connection);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let verifying_key = key.verifying_key();
    let workers = [(), ()].map(|_| {
        let path = path.clone();
        let barrier = std::sync::Arc::clone(&barrier);
        let issuer = ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantTrustedIssuer {
            key_id: "parent-key-1".to_owned(),
            verifying_key,
        };
        std::thread::spawn(move || {
            barrier.wait();
            AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
                path,
                issuer,
                "2026-07-28T00:01:00Z",
            )
        })
    });
    for worker in workers {
        drop(must(worker.join().map_err(|_error| {
            std::io::Error::other("concurrent opener panicked")
        })?)?);
    }
    let connection = Connection::open(path.as_ref())?;
    let marker_count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_storage_privacy_v1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(marker_count, 1);
    let migrated_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [stored_key(&grant.issuer_key_id), stored_key(&grant.nonce)],
        |row| row.get(0),
    )?;
    assert_eq!(migrated_rows, 1);
    Ok(())
}

#[test]
fn consumer_open_retries_a_held_sqlite_write_lock_beyond_the_legacy_retry_window() -> TestResult {
    let key = SigningKey::from_bytes(&[6; 32]);
    let path = store_path("held-write-lock-open");
    let (lock_ready, lock_acquired) = std::sync::mpsc::sync_channel(1);
    let lock_path = path.clone();
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
    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:01:00Z",
        ),
    )?);
    must(
        lock_holder
            .join()
            .map_err(|_error| std::io::Error::other("write-lock holder panicked"))?,
    )?;
    Ok(())
}

#[test]
fn consumer_rejects_tampered_legacy_replay_rows_during_migration() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("tampered-legacy-replay-row");
    let grant = signed_grant(&key);
    let mut grant_json: serde_json::Value =
        serde_json::from_str(&legacy_grant_json(&grant, &key)?)?;
    grant_json["signature"] = serde_json::json!(vec![0_u8; 64]);
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![grant.issuer_key_id, grant.nonce, serde_json::to_string(&grant_json)?, "{}", 1_i64],
    )?;
    drop(connection);
    assert!(matches!(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:01:00Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
    ));
    Ok(())
}

#[test]
fn consumer_rejects_oversized_legacy_replay_json_before_migration() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let grant = signed_grant(&key);
    let path = store_path("oversized-legacy-replay-json");
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            grant.issuer_key_id,
            grant.nonce,
            "x".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES * 8),
            "{}",
            1_i64
        ],
    )?;
    drop(connection);
    assert!(matches!(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:01:00Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
    ));
    Ok(())
}

#[test]
fn consumer_rejects_oversized_legacy_replay_audit_before_materializing_it() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let grant = signed_grant(&key);
    let path = store_path("oversized-legacy-replay-audit");
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            grant.issuer_key_id,
            grant.nonce,
            legacy_grant_json(&grant, &key)?,
            "x".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES * 8 + 1),
            1_i64
        ],
    )?;
    drop(connection);
    assert!(matches!(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:01:00Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::IntegrityRejected)
    ));
    Ok(())
}
