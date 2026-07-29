use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAuditOutcome, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumer,
    AuthenticatedDeliveryGrantExpectation, AuthenticatedDeliveryGrantTrustedIssuer,
};
use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};
use rusqlite::Connection;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn must<T, E: Debug>(result: Result<T, E>) -> TestResult<T> {
    result.map_err(|error| std::io::Error::other(format!("unexpected error: {error:?}")).into())
}

#[derive(Clone)]
struct TestDatabase(Arc<TestDatabasePath>);

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

fn store_path(name: &str) -> TestDatabase {
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

fn signed_grant(key: &SigningKey) -> AuthenticatedDeliveryGrant {
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
        payload_digest: "a".repeat(64),
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

fn signed_grant_for(key: &SigningKey, key_id: &str) -> AuthenticatedDeliveryGrant {
    let mut grant = signed_grant(key);
    grant.issuer_key_id = key_id.to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    grant
}

fn trusted_issuer(key: &SigningKey) -> AuthenticatedDeliveryGrantTrustedIssuer {
    AuthenticatedDeliveryGrantTrustedIssuer {
        key_id: "parent-key-1".to_owned(),
        verifying_key: key.verifying_key(),
    }
}

fn open(
    path: impl AsRef<std::path::Path>,
    issuer: AuthenticatedDeliveryGrantTrustedIssuer,
) -> TestResult<AuthenticatedDeliveryGrantConsumer> {
    must(AuthenticatedDeliveryGrantConsumer::open_at_for_debug_test(
        path,
        issuer,
        "2026-07-28T00:01:00.500Z",
    ))
}

fn expected() -> AuthenticatedDeliveryGrantExpectation {
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
        payload_digest: "a".repeat(64),
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
    let consumed = must(first.consume(&grant, &expected(), "correlation-1"))?;
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
    let replay = must(reopened.consume(&grant, &expected(), "correlation-2"))?;
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
fn consumer_rejects_tamper_wrong_target_expiry_and_revocation() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let mut consumer = open(store_path("negative"), trusted_issuer(&key))?;
    let grant = signed_grant(&key);
    let mut tampered = grant.clone();
    tampered.target_device_id = "other-device".to_owned();
    assert_eq!(
        consumer.consume(&tampered, &expected(), "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::SignatureRejected)
    );
    let mut wrong_target = grant.clone();
    wrong_target.target_device_id = "other-device".to_owned();
    wrong_target.signature = key.sign(&wrong_target.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.consume(&wrong_target, &expected(), "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    let mut expired = grant.clone();
    expired.expires_at = "2026-07-28T00:00:30Z".to_owned();
    expired.signature = key.sign(&expired.signing_bytes()).to_bytes().to_vec();
    assert_eq!(
        consumer.consume(&expired, &expected(), "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    let mut revoked = expected();
    revoked.revocation_version = "revocation-2".to_owned();
    assert_eq!(
        consumer.consume(&grant, &revoked, "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::Revoked)
    );
    Ok(())
}

#[test]
fn consumer_rejects_every_resigned_context_binding_and_wrong_issuer_key_pair() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let grant = signed_grant(&key);
    let mut consumer = open(store_path("every-binding"), trusted_issuer(&key))?;
    macro_rules! assert_binding_rejected {
        ($field:ident, $value:expr) => {{
            let mut wrong = grant.clone();
            wrong.$field = $value.to_owned();
            wrong.signature = key.sign(&wrong.signing_bytes()).to_bytes().to_vec();
            assert_eq!(
                consumer.consume(&wrong, &expected(), "correlation-1"),
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
    assert_binding_rejected!(payload_digest, &"b".repeat(64));

    let other_key = SigningKey::from_bytes(&[5; 32]);
    let mut other_key_consumer = open(
        store_path("wrong-key-pair"),
        AuthenticatedDeliveryGrantTrustedIssuer {
            key_id: "parent-key-1".to_owned(),
            verifying_key: other_key.verifying_key(),
        },
    )?;
    assert_eq!(
        other_key_consumer.consume(&grant, &expected(), "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::SignatureRejected)
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
    let mut consumer = open(store_path("fresh-clock"), trusted_issuer(&key))?;
    let grant = signed_grant(&key);
    assert!(matches!(
        must(consumer.consume_at_for_debug_test(
            &grant,
            &expected(),
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
    must(consumer.consume(&grant, &expected(), "initial-consume"))?;
    for index in 0..24 {
        assert!(matches!(
            must(consumer.consume(&grant, &expected(), format!("replay-{index}")))?,
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
    assert_eq!(count, 16);
    Ok(())
}

#[test]
fn consumers_allow_the_same_nonce_from_distinct_trusted_issuers() -> TestResult {
    let first_key = SigningKey::from_bytes(&[4; 32]);
    let second_key = SigningKey::from_bytes(&[5; 32]);
    let path = store_path("issuer-namespaced-nonce");
    let first_grant = signed_grant(&first_key);
    let second_grant = signed_grant_for(&second_key, "parent-key-2");
    let mut first_consumer = open(&path, trusted_issuer(&first_key))?;
    assert!(matches!(
        must(first_consumer.consume(&first_grant, &expected(), "correlation-1"))?,
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
        must(second_consumer.consume(&second_grant, &expected(), "correlation-2"))?,
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
        consumer.consume(&grant, &expected(), "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
    );
    drop(consumer);
    let mut reopened = open(&path, trusted_issuer(&key))?;
    let outcome = must(reopened.consume(&grant, &expected(), "correlation-2"))?;
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
            must(consumer.consume(&grant, &expected, correlation))
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
