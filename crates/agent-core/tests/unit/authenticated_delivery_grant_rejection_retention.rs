use ed25519_dalek::{Signer, SigningKey};
use rusqlite::{params, Connection};

use super::authenticated_delivery_grant::{
    expected, open, signed_grant, store_path, trusted_issuer,
};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantConsumer,
    AuthenticatedDeliveryGrantValidationRejection,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DELIVERED_PAYLOAD: &[u8] = b"canonical-delivered-action";

#[test]
fn consumer_keeps_the_latest_validation_rejection_when_an_older_clock_follows_future_audits(
) -> TestResult {
    let key = SigningKey::from_bytes(&[6; 32]);
    let path = store_path("validation-rejection-insertion-order-cap");
    drop(open(&path, trusted_issuer(&key))?);
    let connection = Connection::open(path.as_ref())?;
    for index in 0..1_024 {
        connection.execute(
            "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json, recorded_at_nanos, audit_scope) VALUES (?1, ?2, ?3, ?4, 'validation-rejection')",
            params![format!("future-issuer-{index}"), format!("future-nonce-{index}"), "{}", 9_000_000_000_000_000_000_i64],
        )?;
    }
    drop(connection);

    let mut consumer = AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    )
    .map_err(|error| std::io::Error::other(format!("unexpected error: {error:?}")))?;
    let mut invalid = signed_grant(&key);
    invalid.target_device_id = "different-target-device".to_owned();
    invalid.signature = key.sign(&invalid.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.consume(
            &invalid,
            &expected(),
            DELIVERED_PAYLOAD,
            "older-clock-insert".to_owned(),
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    drop(consumer);

    let connection = Connection::open(path.as_ref())?;
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection'",
        [],
        |row| row.get(0),
    )?;
    let retained: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_json LIKE '%older-clock-insert%'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(count, 1_024);
    assert_eq!(retained, 1);
    Ok(())
}

#[test]
fn consumer_backfills_only_legacy_validation_rejections_once() -> TestResult {
    let key = SigningKey::from_bytes(&[7; 32]);
    let path = store_path("legacy-validation-rejection-scope-once");
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_audits_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL)",
        [],
    )?;
    let validation_audit = AuthenticatedDeliveryGrantAudit {
        correlation_id: "legacy-validation".to_owned(),
        issuer_key_id_digest: "issuer".to_owned(),
        nonce_digest: "nonce".to_owned(),
        grant_digest: "grant".to_owned(),
        outcome: AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            AuthenticatedDeliveryGrantValidationRejection::Expired,
        ),
    };
    let replay_audit = AuthenticatedDeliveryGrantAudit {
        correlation_id: "legacy-replay".to_owned(),
        issuer_key_id_digest: "issuer".to_owned(),
        nonce_digest: "nonce".to_owned(),
        grant_digest: "grant".to_owned(),
        outcome: AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected,
    };
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
        params!["legacy-validation-issuer", "legacy-validation-nonce", serde_json::to_string(&validation_audit)?],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
        params!["legacy-replay-issuer", "legacy-replay-nonce", serde_json::to_string(&replay_audit)?],
    )?;
    drop(connection);

    drop(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:05:00Z",
        )
        .map_err(|error| std::io::Error::other(format!("unexpected error: {error:?}")))?,
    );
    let connection = Connection::open(path.as_ref())?;
    let validation_scope: String = connection.query_row(
        "SELECT audit_scope FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = 'legacy-validation-issuer'",
        [],
        |row| row.get(0),
    )?;
    let validation_recorded_at: i64 = connection.query_row(
        "SELECT recorded_at_nanos FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = 'legacy-validation-issuer'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(validation_scope, "validation-rejection");
    assert_eq!(validation_recorded_at, 1_785_197_100_000_000_000);
    connection.execute_batch(
        "CREATE TRIGGER reject_repeat_audit_scope_backfill BEFORE UPDATE ON authenticated_delivery_grant_audits_v2 BEGIN SELECT RAISE(ABORT, 'unexpected audit rewrite'); END;",
    )?;
    drop(connection);

    drop(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:05:00Z",
        )
        .map_err(|error| std::io::Error::other(format!("unexpected error: {error:?}")))?,
    );
    Ok(())
}

#[test]
fn consumer_bounds_distinct_validation_rejection_audits_across_restart() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("bounded-distinct-validation-rejections");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    for index in 0..1_104 {
        let mut invalid = signed_grant(&key);
        invalid.issuer_key_id = format!("untrusted-issuer-{index}");
        invalid.nonce = format!("untrusted-nonce-{index}");
        invalid.signature = key.sign(&invalid.signing_bytes()).to_bytes().to_vec();
        assert_eq!(
            consumer.consume(
                &invalid,
                &expected(),
                DELIVERED_PAYLOAD,
                format!("untrusted-rejection-{index}"),
            ),
            Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
        );
    }
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection'",
        [],
        |row| row.get(0),
    )?;
    let plan: String = connection.query_row(
        "EXPLAIN QUERY PLAN SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' AND recorded_at_nanos <= ?1 ORDER BY recorded_at_nanos LIMIT ?2",
        params![1_i64, 128_i64],
        |row| row.get(3),
    )?;
    assert_eq!(count, 1_024);
    assert_eq!(
        plan,
        "SEARCH authenticated_delivery_grant_audits_v2 USING COVERING INDEX authenticated_delivery_grant_audits_v2_validation_rejection_retention_idx (audit_scope=? AND recorded_at_nanos<?)"
    );
    drop(connection);
    let reopened = open(&path, trusted_issuer(&key))?;
    drop(reopened);
    let connection = Connection::open(path.as_ref())?;
    let retained_after_restart: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(retained_after_restart, 1_024);
    Ok(())
}

#[test]
fn consumer_bounds_post_lock_temporal_rejection_audits_in_their_write_transaction() -> TestResult {
    let key = SigningKey::from_bytes(&[5; 32]);
    let path = store_path("bounded-post-lock-temporal-rejections");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let expired = signed_grant(&key);
    assert_eq!(
        consumer.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:05:00Z"),
        Ok(())
    );
    for index in 0..513 {
        assert_eq!(
            consumer.consume(
                &expired,
                &expected(),
                DELIVERED_PAYLOAD,
                format!("post-lock-expired-{index}"),
            ),
            Err(AuthenticatedDeliveryGrantConsumeError::Expired)
        );
    }

    let mut not_yet_valid = signed_grant(&key);
    not_yet_valid.nonce = "post-lock-not-yet-valid".to_owned();
    not_yet_valid.issued_at = "2026-07-28T00:00:30Z".to_owned();
    not_yet_valid.signature = key.sign(&not_yet_valid.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:00:00Z"),
        Ok(())
    );
    for index in 0..513 {
        assert_eq!(
            consumer.consume(
                &not_yet_valid,
                &expected(),
                DELIVERED_PAYLOAD,
                format!("post-lock-not-yet-valid-{index}"),
            ),
            Err(AuthenticatedDeliveryGrantConsumeError::NotYetValid)
        );
    }
    drop(consumer);

    let connection = Connection::open(path.as_ref())?;
    let audits = connection
        .prepare(
            "SELECT audit_json FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' ORDER BY rowid",
        )?
        .query_map([], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|audit_json| serde_json::from_str::<AuthenticatedDeliveryGrantAudit>(&audit_json))
        .collect::<Result<Vec<_>, _>>()?;
    let expired_count = audits
        .iter()
        .filter(|audit| {
            audit.outcome
                == AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
                    AuthenticatedDeliveryGrantValidationRejection::Expired,
                )
        })
        .count();
    let not_yet_valid_count = audits
        .iter()
        .filter(|audit| {
            audit.outcome
                == AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
                    AuthenticatedDeliveryGrantValidationRejection::NotYetValid,
                )
        })
        .count();
    assert_eq!(audits.len(), 1_024);
    assert_eq!(expired_count, 511);
    assert_eq!(not_yet_valid_count, 513);
    Ok(())
}
