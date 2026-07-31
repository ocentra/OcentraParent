use super::storage_keys::stored_key;
use super::{
    expected, must, open, signed_grant, store_path, trusted_issuer, TestResult, DELIVERED_PAYLOAD,
};
use ed25519_dalek::{Signer, SigningKey};
use ocentra_parent_agent_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantAudit, AuthenticatedDeliveryGrantConsumeError,
    AuthenticatedDeliveryGrantConsumeOutcome, AuthenticatedDeliveryGrantConsumer,
};
use rusqlite::Connection;

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
