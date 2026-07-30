use ed25519_dalek::{Signer, SigningKey};
use rusqlite::{params, Connection};

use super::authenticated_delivery_grant::{
    expected, must, open, signed_grant, store_path, stored_key, trusted_issuer,
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
        "SELECT audit_scope FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1",
        [stored_key("legacy-validation-issuer")],
        |row| row.get(0),
    )?;
    let validation_recorded_at: i64 = connection.query_row(
        "SELECT recorded_at_nanos FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1",
        [stored_key("legacy-validation-issuer")],
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
fn consumer_backfills_legacy_validation_rejections_in_bounded_batches() -> TestResult {
    let key = SigningKey::from_bytes(&[25; 32]);
    let path = store_path("legacy-validation-rejection-batches");
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_audits_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, audit_json TEXT NOT NULL)",
        [],
    )?;
    let audit = AuthenticatedDeliveryGrantAudit {
        correlation_id: "legacy-batch".to_owned(),
        issuer_key_id_digest: "issuer".to_owned(),
        nonce_digest: "nonce".to_owned(),
        grant_digest: "grant".to_owned(),
        outcome: AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            AuthenticatedDeliveryGrantValidationRejection::Expired,
        ),
    };
    let audit_json = serde_json::to_string(&audit)?;
    for index in 0..129 {
        connection.execute(
            "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
            params![format!("legacy-batch-issuer-{index}"), format!("legacy-batch-nonce-{index}"), audit_json],
        )?;
    }
    drop(connection);

    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2026-07-28T00:05:00Z",
        ),
    )?);
    let connection = Connection::open(path.as_ref())?;
    let backfilled: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' AND recorded_at_nanos = ?1",
        [1_785_197_100_000_000_000_i64],
        |row| row.get(0),
    )?;
    assert_eq!(backfilled, 129);
    Ok(())
}

#[test]
fn consumer_keeps_rejection_audits_across_forward_clock_correction() -> TestResult {
    let key = SigningKey::from_bytes(&[26; 32]);
    let path = store_path("validation-rejection-forward-clock-correction");
    let mut consumer = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    let mut invalid = signed_grant(&key);
    invalid.target_device_id = "other-device".to_owned();
    invalid.signature = key.sign(&invalid.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.consume(
            &invalid,
            &expected(),
            DELIVERED_PAYLOAD,
            "forward-clock-audit"
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    drop(consumer);

    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2030-07-28T00:01:00Z",
        ),
    )?);
    let connection = Connection::open(path.as_ref())?;
    let retained_after_forward_jump: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(retained_after_forward_jump, 1);
    Ok(())
}

#[test]
fn consumer_recovers_from_a_future_clock_jump_without_dropping_replay_tombstones() -> TestResult {
    let key = SigningKey::from_bytes(&[27; 32]);
    let path = store_path("recoverable-future-replay-retention-clock");
    let mut consumer = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    let mut tombstone = signed_grant(&key);
    tombstone.nonce = "future-clock-tombstone".to_owned();
    tombstone.expires_at = "2027-07-28T00:10:00Z".to_owned();
    tombstone.signature = key.sign(&tombstone.signing_bytes()).to_bytes().to_vec();
    let tombstone_outcome = consumer.consume_at_for_debug_test(
        &tombstone,
        &expected(),
        DELIVERED_PAYLOAD,
        "tombstone",
        "2026-07-28T00:01:00Z",
    );
    assert!(matches!(
        tombstone_outcome,
        Ok(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_))
    ), "{tombstone_outcome:?}");
    let mut future_grant = signed_grant(&key);
    future_grant.nonce = "future-clock-probe".to_owned();
    future_grant.issued_at = "2030-07-28T00:00:00Z".to_owned();
    future_grant.expires_at = "2030-07-28T00:05:00Z".to_owned();
    future_grant.signature = key.sign(&future_grant.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.inject_trusted_now_after_transaction_for_debug("2030-07-28T00:01:00Z"),
        Ok(())
    );
    let future_outcome = consumer.consume_at_for_debug_test(
        &future_grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "future-clock-probe",
        "2030-07-28T00:01:00Z",
    );
    assert!(matches!(
        future_outcome,
        Ok(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_))
    ), "{future_outcome:?}");
    drop(consumer);

    let mut recovered = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:02:00Z",
    ))?;
    let mut ordinary_grant = signed_grant(&key);
    ordinary_grant.nonce = "recovered-ordinary-grant".to_owned();
    ordinary_grant.expires_at = "2027-07-28T00:30:00Z".to_owned();
    ordinary_grant.signature = key
        .sign(&ordinary_grant.signing_bytes())
        .to_bytes()
        .to_vec();
    assert!(matches!(
        recovered.consume_at_for_debug_test(
            &ordinary_grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "recovered",
            "2026-07-28T00:02:00Z",
        ),
        Ok(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_))
    ));
    assert!(matches!(
        recovered.consume_at_for_debug_test(
            &tombstone,
            &expected(),
            DELIVERED_PAYLOAD,
            "replay",
            "2026-07-28T00:02:00Z",
        ),
        Ok(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_))
    ));
    Ok(())
}

#[test]
fn consumer_recovers_when_the_first_startup_clock_is_in_the_future() -> TestResult {
    let key = SigningKey::from_bytes(&[28; 32]);
    let path = store_path("future-first-startup-recovery");
    drop(must(
        AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
            &path,
            trusted_issuer(&key),
            "2030-07-28T00:01:00Z",
        ),
    )?);
    let mut recovered = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:02:00Z",
    ))?;
    let mut grant = signed_grant(&key);
    grant.nonce = "future-first-recovered-grant".to_owned();
    grant.expires_at = "2027-07-28T00:30:00Z".to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    let outcome = recovered.consume_at_for_debug_test(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "future-first-recovered",
        "2026-07-28T00:02:00Z",
    );
    assert!(matches!(
        outcome,
        Ok(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_))
    ), "{outcome:?}");
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
fn consumer_bounds_temporal_rejection_audits_without_allowing_clock_rollback() -> TestResult {
    let key = SigningKey::from_bytes(&[5; 32]);
    let path = store_path("bounded-post-lock-temporal-rejections");
    let mut consumer = AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:04:59Z",
    )
    .map_err(|error| std::io::Error::other(format!("open failed: {error:?}")))?;
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
    not_yet_valid.issued_at = "2026-07-28T00:05:30Z".to_owned();
    not_yet_valid.expires_at = "2026-07-28T00:10:00Z".to_owned();
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
    let persisted_trusted_now_nanos: i64 = connection.query_row(
        "SELECT highest_trusted_now_nanos FROM authenticated_delivery_grant_replay_retention_v1 WHERE singleton = 1",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(persisted_trusted_now_nanos, 1_785_197_100_000_000_000);
    Ok(())
}
