use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantConsumer,
};
use ocentra_schema::authenticated_delivery_grant::AuthenticatedDeliveryGrant;
use ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES;
use rusqlite::{params, Connection};

use super::{signed_grant, store_path, trusted_issuer, TestResult};

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
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    assert_eq!(stored_nanos.rem_euclid(1_000), 1);
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
