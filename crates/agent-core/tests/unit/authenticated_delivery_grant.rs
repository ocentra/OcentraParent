use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantAuditOutcome,
    AuthenticatedDeliveryGrantConsumeError, AuthenticatedDeliveryGrantConsumeOutcome,
    AuthenticatedDeliveryGrantConsumer, AuthenticatedDeliveryGrantExpectation,
    AuthenticatedDeliveryGrantTrustedIssuer,
};
use ocentra_schema::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};
use rusqlite::{params, Connection};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const DELIVERED_PAYLOAD: &[u8] = b"canonical-delivered-action";
const DELIVERED_PAYLOAD_DIGEST: &str =
    "6406b5682ab324971384904f5d776f211b8133cc7bb42910d55a3deff7a13303";

mod ordering;
#[path = "authenticated_delivery_grant/replay_audit_hardening.rs"]
mod replay_audit_hardening;

fn must<T, E: Debug>(result: Result<T, E>) -> TestResult<T> {
    result.map_err(|error| std::io::Error::other(format!("unexpected error: {error:?}")).into())
}

#[derive(Clone)]
pub(super) struct TestDatabase(Arc<TestDatabasePath>);

struct TestDatabasePath(PathBuf);

impl AsRef<Path> for TestDatabase {
    fn as_ref(&self) -> &Path {
        &self.0 .0
    }
}

impl Drop for TestDatabasePath {
    fn drop(&mut self) {
        for suffix in ["", "-journal", "-shm", "-wal"] {
            let path = if suffix.is_empty() {
                self.0.clone()
            } else {
                PathBuf::from(format!("{}{}", self.0.display(), suffix))
            };
            let _ = std::fs::remove_file(path);
        }
    }
}

pub(super) fn store_path(name: &str) -> TestDatabase {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ocentra-authenticated-delivery-{name}-{}-{}.sqlite",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos())
    ));
    TestDatabase(Arc::new(TestDatabasePath(path)))
}

pub(super) fn signed_grant(key: &SigningKey) -> AuthenticatedDeliveryGrant {
    let mut grant = AuthenticatedDeliveryGrant {
        schema_version: AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
        issuer_key_id: "parent-key-1".to_owned(),
        issuer_actor_id: "parent-1".to_owned(),
        household_id: "household-1".to_owned(),
        parent_device_id: "parent-device-1".to_owned(),
        child_profile_id: "child-1".to_owned(),
        target_device_id: "child-device-1".to_owned(),
        policy_decision_id: "decision-1".to_owned(),
        policy_version: "1".to_owned(),
        action_id: "action-1".to_owned(),
        capability_id: "process-control".to_owned(),
        evidence_digest: "evidence-1".to_owned(),
        payload_digest: DELIVERED_PAYLOAD_DIGEST.to_owned(),
        dry_run: false,
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-07-28T00:00:00Z".to_owned(),
        expires_at: "2026-07-28T00:05:00Z".to_owned(),
        revocation_version: "revocation-1".to_owned(),
        signature: vec![0; 64],
    };
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    grant
}

pub(super) fn trusted_issuer(key: &SigningKey) -> AuthenticatedDeliveryGrantTrustedIssuer {
    AuthenticatedDeliveryGrantTrustedIssuer {
        key_id: "parent-key-1".to_owned(),
        verifying_key: key.verifying_key(),
    }
}

pub(super) fn open(
    path: impl AsRef<std::path::Path>,
    issuer: AuthenticatedDeliveryGrantTrustedIssuer,
) -> TestResult<AuthenticatedDeliveryGrantConsumer> {
    must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        path,
        issuer,
        "2026-07-28T00:01:00.500Z",
    ))
}

pub(super) fn expected() -> AuthenticatedDeliveryGrantExpectation {
    AuthenticatedDeliveryGrantExpectation {
        issuer_actor_id: "parent-1".to_owned(),
        household_id: "household-1".to_owned(),
        parent_device_id: "parent-device-1".to_owned(),
        child_profile_id: "child-1".to_owned(),
        target_device_id: "child-device-1".to_owned(),
        policy_decision_id: "decision-1".to_owned(),
        policy_version: "1".to_owned(),
        action_id: "action-1".to_owned(),
        capability_id: "process-control".to_owned(),
        evidence_digest: "evidence-1".to_owned(),
        revocation_version: "revocation-1".to_owned(),
        observed_at: "2026-07-28T00:01:00Z".to_owned(),
    }
}

#[test]
fn consumer_persists_atomic_consume_and_rejects_restart_replay() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("restart-replay");
    let mut first = open(&path, trusted_issuer(&key))?;
    let grant = signed_grant(&key);
    let consumed = must(first.consume(&grant, &expected(), DELIVERED_PAYLOAD, "correlation-1"))?;
    let AuthenticatedDeliveryGrantConsumeOutcome::Consumed(audit) = consumed else {
        return Err(std::io::Error::other("first consume must apply").into());
    };
    assert_eq!(
        audit.outcome,
        AuthenticatedDeliveryGrantAuditOutcome::Consumed
    );
    assert_ne!(audit.nonce_digest, grant.nonce);
    drop(first);
    let mut reopened = open(&path, trusted_issuer(&key))?;
    let replay = must(reopened.consume(&grant, &expected(), DELIVERED_PAYLOAD, "correlation-2"))?;
    let AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(audit) = replay else {
        return Err(std::io::Error::other("restart replay must reject").into());
    };
    assert_eq!(
        audit.outcome,
        AuthenticatedDeliveryGrantAuditOutcome::ReplayRejected
    );
    Ok(())
}

#[test]
fn consumer_persists_redacted_bounded_audits_for_validation_rejections() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("validation-rejection-audits");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let mut tampered = grant.clone();
    tampered.target_device_id = "tampered-target-device".to_owned();
    let mut binding = grant.clone();
    binding.target_device_id = "other-target-device".to_owned();
    binding.signature = key.sign(&binding.signing_bytes()).to_bytes().to_vec();
    let mut dry_run = grant.clone();
    dry_run.dry_run = true;
    dry_run.signature = key.sign(&dry_run.signing_bytes()).to_bytes().to_vec();
    let mut expired = grant.clone();
    expired.expires_at = "2026-07-28T00:00:30Z".to_owned();
    expired.signature = key.sign(&expired.signing_bytes()).to_bytes().to_vec();
    let mut revoked = expected();
    revoked.revocation_version = "revocation-2".to_owned();
    let attempts = [
        (
            &tampered,
            expected(),
            AuthenticatedDeliveryGrantConsumeError::SignatureRejected,
        ),
        (
            &binding,
            expected(),
            AuthenticatedDeliveryGrantConsumeError::BindingRejected,
        ),
        (
            &dry_run,
            expected(),
            AuthenticatedDeliveryGrantConsumeError::DryRunRejected,
        ),
        (
            &expired,
            expected(),
            AuthenticatedDeliveryGrantConsumeError::Expired,
        ),
        (
            &grant,
            revoked,
            AuthenticatedDeliveryGrantConsumeError::Revoked,
        ),
    ];
    let expected_audits = [
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::SignatureRejected,
        ),
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::BindingRejected,
        ),
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::DryRunRejected,
        ),
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::Expired,
        ),
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            ocentra_parent_agent_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantValidationRejection::Revoked,
        ),
    ];
    for (index, (attempt, expected, error)) in attempts.into_iter().enumerate() {
        assert_eq!(
            consumer.consume(
                attempt,
                &expected,
                DELIVERED_PAYLOAD,
                format!("rejection-correlation-{index}"),
            ),
            Err(error)
        );
    }
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let audits = connection
        .prepare("SELECT audit_json FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2 ORDER BY rowid")?
        .query_map([grant.issuer_key_id.as_str(), grant.nonce.as_str()], |row| row.get::<_, String>(0))?
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(audits.len(), 5);
    for (index, audit_json) in audits.into_iter().enumerate() {
        let audit_keys = serde_json::from_str::<serde_json::Value>(&audit_json)?
            .as_object()
            .ok_or_else(|| std::io::Error::other("audit must be a JSON object"))?
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        assert_eq!(
            audit_keys,
            vec![
                "correlation_id".to_owned(),
                "grant_digest".to_owned(),
                "issuer_key_id_digest".to_owned(),
                "nonce_digest".to_owned(),
                "outcome".to_owned(),
            ]
        );
        let audit: AuthenticatedDeliveryGrantAudit = serde_json::from_str(&audit_json)?;
        assert_eq!(
            audit.correlation_id,
            format!("rejection-correlation-{index}")
        );
        assert_eq!(audit.issuer_key_id_digest.len(), 64);
        assert_eq!(audit.nonce_digest.len(), 64);
        assert_eq!(audit.grant_digest.len(), 64);
        assert_eq!(audit.outcome, expected_audits[index]);
    }
    Ok(())
}

#[test]
fn consumer_rejects_every_resigned_context_binding_and_wrong_issuer_key_pair() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let grant = signed_grant(&key);
    let path = store_path("every-binding");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    macro_rules! assert_binding_rejected {
        ($field:ident, $value:expr) => {{
            let mut wrong = grant.clone();
            wrong.$field = $value.to_owned();
            wrong.signature = key.sign(&wrong.signing_bytes()).to_bytes().to_vec();
            assert_eq!(
                consumer.consume(&wrong, &expected(), DELIVERED_PAYLOAD, "correlation-1"),
                Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
            );
        }};
    }
    assert_binding_rejected!(issuer_key_id, "other-key");
    assert_binding_rejected!(issuer_actor_id, "other-parent");
    assert_binding_rejected!(household_id, "other-household");
    assert_binding_rejected!(parent_device_id, "other-parent-device");
    assert_binding_rejected!(child_profile_id, "other-child");
    assert_binding_rejected!(target_device_id, "other-target-device");
    assert_binding_rejected!(policy_decision_id, "other-decision");
    assert_binding_rejected!(policy_version, "2");
    assert_binding_rejected!(action_id, "other-action");
    assert_binding_rejected!(capability_id, "other-capability");
    assert_binding_rejected!(evidence_digest, "other-evidence");
    let other_key = SigningKey::from_bytes(&[5; 32]);
    let wrong_key_path = store_path("wrong-key-pair");
    let mut other_key_consumer = open(
        &wrong_key_path,
        AuthenticatedDeliveryGrantTrustedIssuer {
            key_id: "parent-key-1".to_owned(),
            verifying_key: other_key.verifying_key(),
        },
    )?;
    assert_eq!(
        other_key_consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::SignatureRejected)
    );
    Ok(())
}

#[test]
fn consumer_rejects_substituted_delivered_payload_even_when_context_matches() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("substituted-payload");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let grant = signed_grant(&key);

    assert_eq!(
        consumer.consume(
            &grant,
            &expected(),
            b"substituted-delivered-action",
            "correlation-substituted-payload",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    Ok(())
}

#[test]
fn consumer_uses_trusted_instant_expiry_and_ignores_caller_observed_time() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let mut grant = signed_grant(&key);
    grant.issued_at = "2026-07-27T23:00:00Z".to_owned();
    grant.expires_at = "2026-07-28T00:30:00+01:00".to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    let path = store_path("instant-expiry");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    assert_eq!(
        consumer.consume_at_for_debug_test(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "correlation-1",
            "2026-07-28T00:01:00Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    let mut malformed_observed_at = expected();
    malformed_observed_at.observed_at = "not-a-timestamp".to_owned();
    assert!(matches!(
        must(consumer.consume_at_for_debug_test(
            &signed_grant(&key),
            &malformed_observed_at,
            DELIVERED_PAYLOAD,
            "correlation-1",
            "2026-07-28T00:01:00Z",
        ))?,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    Ok(())
}

#[test]
fn consumer_reads_trusted_time_at_each_consume_not_only_at_open() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("fresh-clock");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let grant = signed_grant(&key);
    assert!(matches!(
        must(consumer.consume_at_for_debug_test(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "correlation-before-expiry",
            "2026-07-28T00:04:59Z",
        ))?,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    let mut later_grant = signed_grant(&key);
    later_grant.nonce = "nonce-2".to_owned();
    later_grant.signature = key.sign(&later_grant.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.consume_at_for_debug_test(
            &later_grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "correlation-after-expiry",
            "2026-07-28T00:05:00Z",
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    Ok(())
}

#[test]
fn consumer_bounds_persisted_replay_audits_per_unexpired_grant() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("replay-audit-bound");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    must(consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, "initial-consume"))?;
    for index in 0..24 {
        assert!(matches!(
            must(consumer.consume(
                &grant,
                &expected(),
                DELIVERED_PAYLOAD,
                format!("replay-{index}")
            ))?,
            AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_)
        ));
    }
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    let audit_plan: String = connection.query_row(
        "EXPLAIN QUERY PLAN SELECT rowid FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(3),
    )?;
    assert_eq!(count, 16);
    assert_eq!(
        audit_plan,
        "SEARCH authenticated_delivery_grant_audits_v2 USING COVERING INDEX authenticated_delivery_grant_audits_v2_grant_idx (issuer_key_id=? AND nonce=?)"
    );
    Ok(())
}

#[test]
fn consumer_purges_expired_replay_rows_in_indexed_bounded_batches_with_matching_audits(
) -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("indexed-expiry-purge");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let connection = Connection::open(path.as_ref())?;
    for index in 0..129 {
        let issuer_key_id = format!("expired-issuer-{index}");
        let nonce = format!("expired-nonce-{index}");
        connection.execute(
            "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![issuer_key_id, nonce, "{}", "{}", 0_i64],
        )?;
        connection.execute(
            "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
            params![format!("expired-issuer-{index}"), format!("expired-nonce-{index}"), "{}"],
        )?;
    }
    let plan: String = connection.query_row(
        "EXPLAIN QUERY PLAN SELECT issuer_key_id, nonce FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos <= ?1 ORDER BY expires_at_nanos LIMIT ?2",
        params![1_i64, 128_i64],
        |row| row.get(3),
    )?;
    assert_eq!(
        plan,
        "SEARCH authenticated_delivery_grant_consumes_v2 USING COVERING INDEX authenticated_delivery_grant_consumes_v2_expiry_idx (expires_at_nanos<?)"
    );
    drop(connection);
    must(consumer.consume_at_for_debug_test(
        &signed_grant(&key),
        &expected(),
        DELIVERED_PAYLOAD,
        "trigger-indexed-expiry-purge",
        "2026-07-28T00:01:00Z",
    ))?;
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let remaining_expired: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos = 0",
        [],
        |row| row.get(0),
    )?;
    let remaining_expired_audits: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id LIKE 'expired-issuer-%'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(remaining_expired, 1);
    assert_eq!(remaining_expired_audits, 1);
    Ok(())
}

#[test]
fn consumer_preserves_nanosecond_expiry_precision_for_replay_retention() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("nanosecond-expiry-retention");
    let mut grant = signed_grant(&key);
    grant.expires_at = "2026-07-28T00:05:00.000000001Z".to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let consumed = must(consumer.consume_at_for_debug_test(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "nanosecond-expiry-consume",
        "2026-07-28T00:05:00Z",
    ))?;
    assert!(matches!(
        consumed,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    drop(consumer);
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
fn consumer_backfills_legacy_microsecond_rows_from_signed_grant_nanos() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("legacy-microsecond-backfill");
    let mut grant = signed_grant(&key);
    grant.expires_at = "2026-07-28T00:05:00.000000001Z".to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "CREATE TABLE authenticated_delivery_grant_consumes_v2 (issuer_key_id TEXT NOT NULL, nonce TEXT NOT NULL, grant_json TEXT NOT NULL, audit_json TEXT NOT NULL, expires_at_micros INTEGER, PRIMARY KEY (issuer_key_id, nonce))",
        [],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_json, audit_json, expires_at_micros) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![grant.issuer_key_id, grant.nonce, serde_json::to_string(&grant)?, "{}", 1_i64],
    )?;
    drop(connection);
    let consumer = open(&path, trusted_issuer(&key))?;
    drop(consumer);
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
fn consumer_open_purges_expired_replay_records_while_device_was_inactive() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("startup-expiry-purge");
    let grant = signed_grant(&key);
    let mut active = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    must(active.consume(
        &grant,
        &expected(),
        DELIVERED_PAYLOAD,
        "consume-before-inactive",
    ))?;
    drop(active);
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
        params![grant.issuer_key_id, grant.nonce, "{}"],
    )?;
    drop(connection);
    let inactive_restart = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:05:00Z",
    ))?;
    drop(inactive_restart);
    let connection = Connection::open(path.as_ref())?;
    let retained_grants: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    let retained_audits: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        [grant.issuer_key_id.as_str(), grant.nonce.as_str()],
        |row| row.get(0),
    )?;
    assert_eq!(retained_grants, 0);
    assert_eq!(retained_audits, 0);
    Ok(())
}

#[test]
fn consumer_open_drains_all_expired_replay_rows_in_bounded_batches() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("startup-expiry-drain");
    let initial = open(&path, trusted_issuer(&key))?;
    drop(initial);
    let connection = Connection::open(path.as_ref())?;
    for index in 0..257 {
        let issuer_key_id = format!("expired-startup-issuer-{index}");
        let nonce = format!("expired-startup-nonce-{index}");
        connection.execute(
            "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![issuer_key_id, nonce, "{}", "{}", 0_i64],
        )?;
        connection.execute(
            "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
            params![format!("expired-startup-issuer-{index}"), format!("expired-startup-nonce-{index}"), "{}"],
        )?;
    }
    drop(connection);
    let restarted = must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    ))?;
    drop(restarted);
    let connection = Connection::open(path.as_ref())?;
    let expired_grants: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE expires_at_nanos = 0",
        [],
        |row| row.get(0),
    )?;
    let expired_audits: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id LIKE 'expired-startup-issuer-%'",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(expired_grants, 0);
    assert_eq!(expired_audits, 0);
    Ok(())
}

#[test]
fn consumer_keeps_expired_grant_when_audit_delete_fails_atomically() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("atomic-expiry-delete");
    let initial = open(&path, trusted_issuer(&key))?;
    drop(initial);
    let connection = Connection::open(path.as_ref())?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_consumes_v2 (issuer_key_id, nonce, grant_fingerprint, audit_json, expires_at_nanos) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!["atomic-issuer", "atomic-nonce", "{}", "{}", 0_i64],
    )?;
    connection.execute(
        "INSERT INTO authenticated_delivery_grant_audits_v2 (issuer_key_id, nonce, audit_json) VALUES (?1, ?2, ?3)",
        params!["atomic-issuer", "atomic-nonce", "{}"],
    )?;
    connection.execute_batch(
        "CREATE TRIGGER reject_atomic_audit_delete BEFORE DELETE ON authenticated_delivery_grant_audits_v2 WHEN OLD.issuer_key_id = 'atomic-issuer' BEGIN SELECT RAISE(ABORT, 'audit-delete-blocked'); END;",
    )?;
    drop(connection);
    let error = AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        &path,
        trusted_issuer(&key),
        "2026-07-28T00:01:00Z",
    );
    let Err(error) = error else {
        return Err(
            std::io::Error::other("audit deletion trigger must reject startup purge").into(),
        );
    };
    assert_eq!(
        error,
        AuthenticatedDeliveryGrantConsumeError::StorageUnavailable
    );
    let connection = Connection::open(path.as_ref())?;
    let retained_grants: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        ["atomic-issuer", "atomic-nonce"],
        |row| row.get(0),
    )?;
    let retained_audits: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2 WHERE issuer_key_id = ?1 AND nonce = ?2",
        ["atomic-issuer", "atomic-nonce"],
        |row| row.get(0),
    )?;
    assert_eq!(retained_grants, 1);
    assert_eq!(retained_audits, 1);
    Ok(())
}

#[test]
fn consumers_allow_the_same_nonce_from_distinct_trusted_issuers() -> TestResult {
    let first_key = SigningKey::from_bytes(&[4; 32]);
    let second_key = SigningKey::from_bytes(&[5; 32]);
    let path = store_path("issuer-namespaced-nonce");
    let first_grant = signed_grant(&first_key);
    let mut second_grant = signed_grant(&second_key);
    second_grant.issuer_key_id = "parent-key-2".to_owned();
    second_grant.signature = second_key
        .sign(&second_grant.signing_bytes())
        .to_bytes()
        .to_vec();
    let mut first_consumer = open(&path, trusted_issuer(&first_key))?;
    assert!(matches!(
        must(first_consumer.consume(
            &first_grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "correlation-1"
        ))?,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    drop(first_consumer);
    let mut second_consumer = open(
        &path,
        AuthenticatedDeliveryGrantTrustedIssuer {
            key_id: "parent-key-2".to_owned(),
            verifying_key: second_key.verifying_key(),
        },
    )?;
    assert!(matches!(
        must(second_consumer.consume(
            &second_grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "correlation-2"
        ))?,
        AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
    ));
    Ok(())
}

#[test]
fn failed_commit_rolls_back_consume_so_retry_after_reopen_is_safe() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("commit-failure");
    let grant = signed_grant(&key);
    let mut consumer = open(&path, trusted_issuer(&key))?;
    consumer.inject_next_commit_failure_for_debug();
    assert_eq!(
        consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
    );
    drop(consumer);
    let mut reopened = open(&path, trusted_issuer(&key))?;
    let outcome = must(reopened.consume(&grant, &expected(), DELIVERED_PAYLOAD, "correlation-2"))?;
    let AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_) = outcome else {
        return Err(std::io::Error::other("uncommitted consume must retry safely").into());
    };
    Ok(())
}

#[test]
fn concurrent_consumers_allow_exactly_one_durable_consume() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("concurrent");
    let grant = signed_grant(&key);
    let expected = expected();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers = ["correlation-1", "correlation-2"].map(|correlation| {
        let path = path.clone();
        let grant = grant.clone();
        let expected = expected.clone();
        let barrier = std::sync::Arc::clone(&barrier);
        let verifying_key = key.verifying_key();
        std::thread::spawn(move || {
            barrier.wait();
            let mut consumer = open(
                path,
                AuthenticatedDeliveryGrantTrustedIssuer {
                    key_id: "parent-key-1".to_owned(),
                    verifying_key,
                },
            )?;
            must(consumer.consume(&grant, &expected, DELIVERED_PAYLOAD, correlation))
        })
    });
    let mut outcomes = Vec::new();
    for worker in workers {
        outcomes.push(
            worker
                .join()
                .map_err(|_error| std::io::Error::other("concurrent consumer panicked"))??,
        );
    }
    let consumed = outcomes
        .iter()
        .filter(|outcome| {
            matches!(
                outcome,
                AuthenticatedDeliveryGrantConsumeOutcome::Consumed(_)
            )
        })
        .count();
    let replayed = outcomes
        .iter()
        .filter(|outcome| {
            matches!(
                outcome,
                AuthenticatedDeliveryGrantConsumeOutcome::ReplayRejected(_)
            )
        })
        .count();
    assert_eq!(consumed, 1);
    assert_eq!(replayed, 1);
    Ok(())
}
