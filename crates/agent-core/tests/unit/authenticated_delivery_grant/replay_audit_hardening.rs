use ed25519_dalek::{Signer, SigningKey};
use rusqlite::{params, Connection};

use super::{
    expected, must, open, signed_grant, store_path, trusted_issuer, TestResult, DELIVERED_PAYLOAD,
};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantConsumeOutcome,
};

#[test]
fn restart_replay_uses_non_reconstructable_fingerprint_without_raw_grant_storage() -> TestResult {
    let key = SigningKey::from_bytes(&[9; 32]);
    let path = store_path("fingerprint-restart-replay");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    must(consumer.consume(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "fingerprint-consume",
    ))?;
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let fingerprint: String = connection.query_row(
        "SELECT grant_fingerprint FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    assert_eq!(fingerprint.len(), 64);
    assert_ne!(fingerprint.as_bytes(), grant.signing_bytes().as_slice());
    assert_eq!(
        fingerprint,
        "18ca55dd5ea42830ff91cd6935bcc87b04eb0d6103e30ebca5354f86223177f8"
    );
    assert!(!fingerprint.contains(&serde_json::to_string(&grant)?));
    drop(connection);
    let mut restarted = open(&path, trusted_issuer(&key))?;
    assert!(matches!(
        must(restarted.consume(&grant, &expected(), DELIVERED_PAYLOAD, "fingerprint-replay"))?,
        AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_)
    ));
    Ok(())
}

#[test]
fn post_lock_temporal_revalidation_rejects_both_newly_future_and_expired_grants() -> TestResult {
    let key = SigningKey::from_bytes(&[10; 32]);
    let path = store_path("post-lock-temporal-window");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let grant = signed_grant(&key);
    must(consumer.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:05:00Z"))?;
    assert_eq!(
        consumer.consume_at_for_debug_test(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "post-lock-expired",
            "2026-07-28T00:04:59Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    let mut future_grant = signed_grant(&key);
    future_grant.nonce = "post-lock-future-nonce".to_owned();
    future_grant.issued_at = "2026-07-28T00:04:59.500000001Z".to_owned();
    future_grant.expires_at = "2026-07-28T00:10:00Z".to_owned();
    future_grant.signature = key.sign(&future_grant.signing_bytes()).to_bytes().to_vec();
    must(consumer.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:04:59Z"))?;
    assert_eq!(
        consumer.consume_at_for_debug_test(
            &future_grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "post-lock-future",
            "2026-07-28T00:05:00Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::NotYetValid)
    );
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(rows, 0);
    Ok(())
}

#[test]
fn restart_backfill_parses_audit_outcomes_without_substring_misclassification() -> TestResult {
    let key = SigningKey::from_bytes(&[11; 32]);
    let path = store_path("structured-audit-backfill");
    drop(must(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "1970-01-01T00:00:00Z",
    ))?);
    let valid = AuthenticatedDeliveryGrantAudit {
        correlation_id: "legacy-validation".to_owned(),
        issuer_key_id_digest: "a".repeat(64),
        nonce_digest: "b".repeat(64),
        grant_digest: "c".repeat(64),
        outcome: AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::Expired,
        ),
    };
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, audit_scope) VALUES (?1, ?2, ?3, 'replay')",
        params!["structured", "valid", serde_json::to_string(&valid)?],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, audit_scope) VALUES (?1, ?2, ?3, 'replay')",
        params!["structured", "malformed", r#"{"outcome":"validation-rejected","missing":}"#],
    )?;
    drop(connection);
    drop(must(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "1970-01-01T00:00:00Z",
    ))?);
    let connection = Connection::open(path.as_ref())?;
    let valid_scope: String = connection.query_row(
        "SELECT audit_scope FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = 'structured' AND nonce = 'valid'", [], |row| row.get(0),
    )?;
    let malformed_scope: String = connection.query_row(
        "SELECT audit_scope FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = 'structured' AND nonce = 'malformed'", [], |row| row.get(0),
    )?;
    assert_eq!(valid_scope, "validation-rejection");
    assert_eq!(malformed_scope, "replay");
    Ok(())
}

#[test]
fn replay_expiry_purge_preserves_validation_audits_for_the_same_grant_identity() -> TestResult {
    let key = SigningKey::from_bytes(&[12; 32]);
    let path = store_path("replay-scope-purge");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    must(consumer.consume(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "replay-scope-consume",
    ))?;
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) VALUES (?1, ?2, ?3, ?4, 'validation-rejection')",
        params![grant.issuer_key_id, grant.nonce, "{}", 1_785_196_799_000_000_000_i64],
    )?;
    drop(connection);
    drop(must(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:05:00Z",
    ))?);
    let connection = Connection::open(path.as_ref())?;
    let replay_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()], |row| row.get(0),
    )?;
    let validation_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'validation-rejection'",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()], |row| row.get(0),
    )?;
    assert_eq!(replay_rows, 0);
    assert_eq!(validation_rows, 1);
    Ok(())
}
