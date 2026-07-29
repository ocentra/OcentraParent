use ed25519_dalek::{Signer, SigningKey};
use rusqlite::{params, Connection};

use super::{
    expected, must, open, signed_grant, store_path, trusted_issuer, TestResult, DELIVERED_PAYLOAD,
};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantConsumeOutcome,
    AuthenticatedDeliveryGrantConsumer,
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
        "81450ad3d6642b99eba5cb6a61d0792af4d422d42c9138f68685fe014bd44485"
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
    let audits: Vec<(String, String, i64)> = {
        let mut statement = connection.prepare(
            "SELECT audit_scope, audit_json, recorded_at_nanos FROM authenticated_delivery_grant_audits_v2 ORDER BY rowid",
        )?;
        let rows = statement
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        rows
    };
    assert_eq!(audits.len(), 2);
    let recorded_at_nanos: Vec<i64> = audits
        .iter()
        .map(|(_, _, recorded_at_nanos)| *recorded_at_nanos)
        .collect();
    assert_eq!(
        recorded_at_nanos,
        vec![1_785_197_100_000_000_000, 1_785_197_099_000_000_000]
    );
    for (scope, audit_json, _) in audits {
        assert_eq!(scope, "validation-rejection");
        let audit: AuthenticatedDeliveryGrantAudit = serde_json::from_str(&audit_json)?;
        assert!(matches!(
            audit.outcome,
            AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
                ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::Expired
                    | ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::NotYetValid
            )
        ));
    }
    Ok(())
}

#[test]
fn debug_consume_refreshes_post_lock_clock_without_an_explicit_post_lock_override() -> TestResult {
    let key = SigningKey::from_bytes(&[15; 32]);
    let path = store_path("debug-post-lock-clock-refresh");
    let grant = signed_grant(&key);
    let mut consumer = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00.500Z",
    ))?;

    assert_eq!(
        consumer.consume(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "debug-post-lock-clock-refresh",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    drop(consumer);

    let connection = Connection::open(path.as_ref())?;
    let consumed_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(consumed_rows, 0);
    Ok(())
}

#[test]
fn oversized_direct_grant_is_audited_with_bounded_data_without_storing_the_untrusted_field(
) -> TestResult {
    let key = SigningKey::from_bytes(&[13; 32]);
    let path = store_path("bounded-shape-rejection-audit");
    let mut grant = signed_grant(&key);
    grant.issuer_key_id = "x".repeat(8 * 1024 * 1024);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    assert_eq!(
        consumer.consume(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "bounded-shape-rejection"
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::InvalidGrant)
    );
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let (issuer_key_id, nonce, audit_json, scope): (String, String, String, String) = connection.query_row(
        "SELECT issuer_key_id, nonce, audit_json, audit_scope FROM authenticated_delivery_grant_audits_v2",
        [],
        |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
    )?;
    assert_eq!(issuer_key_id.len(), 64);
    assert_eq!(nonce.len(), 64);
    assert!(audit_json.len() < 1_024);
    assert_eq!(scope, "validation-rejection");
    let audit: AuthenticatedDeliveryGrantAudit = serde_json::from_str(&audit_json)?;
    assert_eq!(audit.issuer_key_id_digest.len(), 64);
    assert_eq!(audit.nonce_digest.len(), 64);
    assert_eq!(audit.grant_digest.len(), 64);
    assert_eq!(
        audit.outcome,
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::InvalidGrant
        )
    );
    let consumed_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(consumed_rows, 0);
    Ok(())
}

#[test]
fn malformed_shape_rejections_are_retained_without_blocking_valid_consumption() -> TestResult {
    let key = SigningKey::from_bytes(&[13; 32]);
    let path = store_path("malformed-shape-rejection-retention");
    let valid_grant = signed_grant(&key);
    let mut malformed_grant = valid_grant.clone();
    malformed_grant.schema_version = 0;
    let mut consumer = open(&path, trusted_issuer(&key))?;
    for attempt in 0..1_025 {
        assert_eq!(
            consumer.consume(
                &malformed_grant,
                &expected(),
                DELIVERED_PAYLOAD,
                format!("malformed-shape-retention-{attempt}"),
            ),
            Err(AuthenticatedDeliveryGrantConsumeError::InvalidGrant)
        );
    }
    let consumed = must(consumer.consume(
        &valid_grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "valid-after-malformed-shape-rejections",
    ))?;
    assert!(matches!(
        consumed,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let retained_rejections: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(retained_rejections, 1_024);
    let consumed_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        params![valid_grant.issuer_key_id, valid_grant.nonce],
        |row| row.get(0),
    )?;
    assert_eq!(consumed_rows, 1);
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
        params![grant.issuer_key_id, grant.nonce, "{}", 1_785_283_499_000_000_000_i64],
    )?;
    drop(connection);
    drop(must(ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-29T00:05:00Z",
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

#[test]
fn replay_audit_trim_never_evicts_validation_evidence_for_the_same_grant() -> TestResult {
    let key = SigningKey::from_bytes(&[13; 32]);
    let path = store_path("replay-audit-trim-validation-evidence");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let mut rejected_expected = expected();
    rejected_expected.action_id = "different-action".to_owned();
    assert_eq!(
        consumer.consume(
            &grant,
            &rejected_expected,
            DELIVERED_PAYLOAD,
            "validation-evidence",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    must(consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, "initial-consume"))?;
    for index in 0..24 {
        assert!(matches!(
            must(consumer.consume(
                &grant,
                &expected(),
                DELIVERED_PAYLOAD,
                format!("replay-{index}"),
            ))?,
            AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_)
        ));
    }
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let validation_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'validation-rejection'",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    let replay_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 AND audit_scope = 'replay'",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    assert_eq!(validation_rows, 1);
    assert_eq!(replay_rows, 16);
    Ok(())
}

#[test]
fn replay_tombstone_survives_bounded_backward_wall_clock_correction() -> TestResult {
    let key = SigningKey::from_bytes(&[14; 32]);
    let path = store_path("replay-tombstone-clock-correction");
    let grant = signed_grant(&key);
    let mut trigger_grant = signed_grant(&key);
    trigger_grant.nonce = "clock-trigger-nonce".to_owned();
    trigger_grant.expires_at = "2026-07-30T00:05:00Z".to_owned();
    trigger_grant.signature = key.sign(&trigger_grant.signing_bytes()).to_bytes().to_vec();
    let mut consumer = open(&path, trusted_issuer(&key))?;
    must(consumer.consume_at_for_debug_test(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "consume-before-expiry",
        "2026-07-28T00:04:59Z",
    ))?;
    must(consumer.consume_at_for_debug_test(
        &trigger_grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "advance-wall-clock",
        "2026-07-28T00:05:01Z",
    ))?;
    assert!(matches!(
        must(consumer.consume_at_for_debug_test(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "backward-clock-replay",
            "2026-07-28T00:04:59Z",
        ))?,
        AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_)
    ));
    Ok(())
}
