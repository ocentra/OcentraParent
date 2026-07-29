use ed25519_dalek::{Signer, SigningKey};
use rusqlite::{params, Connection};

use super::authenticated_delivery_grant::{
    expected, open, signed_grant, store_path, trusted_issuer,
};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantValidationRejection,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DELIVERED_PAYLOAD: &[u8] = b"canonical-delivered-action";

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
    assert_eq!(expired_count, 513);
    assert_eq!(not_yet_valid_count, 511);
    Ok(())
}
