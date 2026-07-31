use super::storage_keys::stored_key;
use super::{
    expected, must, open, signed_grant, store_path, trusted_issuer, TestResult, DELIVERED_PAYLOAD,
};
use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumer,
};
use rusqlite::{params, Connection};

#[test]
fn first_consume_does_not_confirm_a_startup_future_clock() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("startup-future-clock-first-consume");
    let mut grant = signed_grant(&key);
    grant.nonce = "startup-future-clock-grant".to_owned();
    grant.expires_at = "2027-07-28T00:05:00Z".to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    let mut future = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2030-07-28T00:01:00Z",
    ))?;
    assert_eq!(
        future.consume(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "future-clock-attempt"
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    drop(future);
    let mut corrected = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    assert!(matches!(
        must(corrected.consume(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "corrected-clock-attempt"
        ))?,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    Ok(())
}

#[test]
fn accepted_caller_correlation_is_hashed_before_audit_persistence() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("accepted-correlation-audit-redaction");
    let correlation = "child-identifier/private-url/token";
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let outcome = must(consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, correlation))?;
    let AuthenticatedDeliveryGrantConsumeOutcome::Consumed(audit) = outcome else {
        return Err(std::io::Error::other("first consume must apply").into());
    };
    assert_eq!(audit.correlation_id, stored_key(correlation));
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let audit_json: String = connection.query_row(
        "SELECT audit_json FROM authenticated_delivery_grant_audits_v2",
        [],
        |row| row.get(0),
    )?;
    let persisted: AuthenticatedDeliveryGrantAudit = serde_json::from_str(&audit_json)?;
    assert_eq!(persisted.correlation_id, stored_key(correlation));
    Ok(())
}

#[test]
fn privacy_migration_redacts_legacy_caller_correlation_in_every_audit_copy() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("privacy-migration-audit-correlation-redaction");
    let raw_correlation = "child-identifier/private-url/token";
    let raw_audit = serde_json::json!({
        "correlation_id": raw_correlation,
        "outcome": "consumed"
    })
    .to_string();
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_fingerprint TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_nanos INTEGER NOT NULL, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_audits_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL, recorded_at_nanos INTEGER, audit_scope TEXT NOT NULL DEFAULT 'replay')",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["legacy-issuer", "legacy-nonce", "legacy-fingerprint", raw_audit, 1_i64],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["legacy-issuer", "legacy-nonce", raw_audit, 1_i64, "replay"],
    )?;
    drop(connection);

    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:01:00Z",
        ),
    )?);

    let connection = Connection::open(path.as_ref())?;
    let copied_audits = [
        connection.query_row(
            "SELECT audit_json FROM authenticated_delivery_grant_consumes_v2",
            [],
            |row| row.get::<_, String>(0),
        )?,
        connection.query_row(
            "SELECT audit_json FROM authenticated_delivery_grant_audits_v2",
            [],
            |row| row.get::<_, String>(0),
        )?,
    ];
    for copied_audit in copied_audits {
        let audit = serde_json::from_str::<serde_json::Value>(&copied_audit)?;
        let correlation = audit
            .get("correlation_id")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| std::io::Error::other("migrated audit must retain a correlation"))?;
        assert_eq!(correlation, stored_key(raw_correlation));
    }
    Ok(())
}

#[test]
fn confirmed_clock_large_forward_jump_starts_a_persisted_provisional_epoch() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("confirmed-clock-large-forward-jump-recovery");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    drop(must(consumer.consume(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "confirm-initial-epoch",
    ))?);
    drop(consumer);

    let advanced = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2027-07-30T00:01:00Z",
    ))?;
    drop(advanced);

    let connection = Connection::open(path.as_ref())?;
    let (highest, confirmed, observed_at): (i64, bool, Option<i64>) = connection.query_row(
        "SELECT highest_trusted_now_nanos, confirmed, provisional_observed_at_nanos FROM authenticated_delivery_grant_replay_retention_v3 WHERE singleton = 1",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
    )?;
    let expected_nanos = chrono::DateTime::parse_from_rfc3339("2027-07-30T00:01:00Z")?
        .timestamp_nanos_opt()
        .ok_or_else(|| std::io::Error::other("test timestamp must fit nanos"))?;
    assert_eq!(highest, expected_nanos);
    assert!(!confirmed);
    assert_eq!(observed_at, None);
    Ok(())
}

#[test]
fn confirmed_clock_rollback_preserves_purged_replay_tombstones_without_blocking_new_recovery_grants(
) -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("confirmed-clock-rollback-replay-floor");
    let mut consumed = signed_grant(&key);
    consumed.nonce = "rollback-purged-nonce".to_owned();
    consumed.expires_at = "2026-07-28T00:02:00Z".to_owned();
    consumed.signature = key.sign(&consumed.signing_bytes()).to_bytes().to_vec();

    let mut initial = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    must(initial.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:01:00Z"))?;
    let initial_outcome = initial
        .consume(&consumed, &expected(), DELIVERED_PAYLOAD, "initial-consume")
        .map_err(|error| std::io::Error::other(format!("initial consume: {error:?}")))?;
    assert!(matches!(
        initial_outcome,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    drop(initial);

    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:10:00Z",
        ),
    )?);
    let connection = Connection::open(path.as_ref())?;
    let purged: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE nonce = ?1",
        [stored_key(&consumed.nonce)],
        |row| row.get(0),
    )?;
    assert_eq!(purged, 0);
    drop(connection);

    let mut corrected = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    must(corrected.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:01:00Z"))?;
    let replay_outcome = corrected
        .consume(
            &consumed,
            &expected(),
            DELIVERED_PAYLOAD,
            "replay-after-rollback",
        )
        .map_err(|error| std::io::Error::other(format!("rollback replay: {error:?}")))?;
    assert!(matches!(
        replay_outcome,
        AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_)
    ));
    let mut recovery = signed_grant(&key);
    recovery.nonce = "rollback-recovery-nonce".to_owned();
    recovery.expires_at = "2026-07-28T00:11:00Z".to_owned();
    recovery.signature = key.sign(&recovery.signing_bytes()).to_bytes().to_vec();
    let recovery_outcome = corrected
        .consume(
            &recovery,
            &expected(),
            DELIVERED_PAYLOAD,
            "new-after-rollback",
        )
        .map_err(|error| std::io::Error::other(format!("rollback recovery: {error:?}")))?;
    assert!(matches!(
        recovery_outcome,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    Ok(())
}
