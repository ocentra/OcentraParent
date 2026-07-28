use std::{fmt::Debug, path::PathBuf};

use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAuditOutcome, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumer,
    AuthenticatedDeliveryGrantExpectation,
};
use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn must<T, E: Debug>(result: Result<T, E>) -> TestResult<T> {
    result.map_err(|error| std::io::Error::other(format!("unexpected error: {error:?}")).into())
}

fn store_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push(format!(
        "ocentra-authenticated-delivery-{name}-{}.sqlite",
        std::process::id()
    ));
    path
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

fn expected() -> AuthenticatedDeliveryGrantExpectation {
    AuthenticatedDeliveryGrantExpectation {
        issuer_key_id: "parent-key-1".to_owned(),
        household_id: "household-1".to_owned(),
        child_profile_id: "child-1".to_owned(),
        target_device_id: "child-device-1".to_owned(),
        policy_decision_id: "decision-1".to_owned(),
        action_id: "action-1".to_owned(),
        payload_digest: "a".repeat(64),
        revocation_version: "revocation-1".to_owned(),
        observed_at: "2026-07-28T00:01:00Z".to_owned(),
    }
}

#[test]
fn consumer_persists_atomic_consume_and_rejects_restart_replay() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("restart-replay");
    let mut first = must(AuthenticatedDeliveryGrantConsumer::open(
        &path,
        key.verifying_key(),
    ))?;
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
    let mut reopened = must(AuthenticatedDeliveryGrantConsumer::open(
        &path,
        key.verifying_key(),
    ))?;
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
    let mut consumer = must(AuthenticatedDeliveryGrantConsumer::open(
        store_path("negative"),
        key.verifying_key(),
    ))?;
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
fn failed_commit_rolls_back_consume_so_retry_after_reopen_is_safe() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("commit-failure");
    let grant = signed_grant(&key);
    let mut consumer = must(AuthenticatedDeliveryGrantConsumer::open(
        &path,
        key.verifying_key(),
    ))?;
    consumer.inject_next_commit_failure_for_debug();
    assert_eq!(
        consumer.consume(&grant, &expected(), "correlation-1"),
        Err(AuthenticatedDeliveryGrantConsumeError::StorageUnavailable)
    );
    drop(consumer);
    let mut reopened = must(AuthenticatedDeliveryGrantConsumer::open(
        &path,
        key.verifying_key(),
    ))?;
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
        let barrier = barrier.clone();
        let verifying_key = key.verifying_key();
        std::thread::spawn(move || {
            barrier.wait();
            let mut consumer = must(AuthenticatedDeliveryGrantConsumer::open(
                path,
                verifying_key,
            ))?;
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
