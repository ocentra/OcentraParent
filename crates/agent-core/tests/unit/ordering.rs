use super::*;

#[test]
fn consumer_persists_bounded_audit_for_oversized_correlation_before_grant_validation() -> TestResult
{
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("oversized-correlation-before-audit");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let mut grant = signed_grant(&key);
    grant.target_device_id = "tampered-target-device".to_owned();
    let oversized_correlation = "x".repeat(
        ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
            + 1,
    );
    assert_eq!(
        consumer.consume(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            &oversized_correlation,
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let audit_json: String = connection.query_row(
        "SELECT audit_json FROM authenticated_delivery_grant_audits_v2 ORDER BY rowid DESC LIMIT 1",
        [],
        |row| row.get(0),
    )?;
    let audit: AuthenticatedDeliveryGrantAudit = serde_json::from_str(&audit_json)?;
    assert_eq!(audit.correlation_id.len(), 64);
    assert_ne!(audit.correlation_id, oversized_correlation);
    assert_eq!(audit.issuer_key_id_digest.len(), 64);
    assert_eq!(audit.nonce_digest.len(), 64);
    assert_eq!(audit.grant_digest.len(), 64);
    assert_eq!(
        audit.outcome,
        AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
            AuthenticatedDeliveryGrantValidationRejection::BindingRejected,
        )
    );
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
    Ok(())
}

#[test]
fn consumer_authenticates_grant_before_rejecting_oversized_delivered_payload() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("payload-authentication-order");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let mut invalid = signed_grant(&key);
    invalid.target_device_id = "tampered-target-device".to_owned();
    let payload = vec![0_u8; AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES + 1];
    assert_eq!(
        consumer.consume(
            &invalid,
            &expected(),
            &payload,
            "invalid-grant-oversized-payload"
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::SignatureRejected)
    );
    assert_eq!(
        consumer.consume(
            &signed_grant(&key),
            &expected(),
            &payload,
            "authenticated-grant-oversized-payload"
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
    );
    Ok(())
}

#[test]
fn consumer_rechecks_expiry_after_acquiring_consume_transaction() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("expiry-after-transaction");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let grant = signed_grant(&key);
    must(consumer.inject_trusted_now_after_transaction_for_debug("2026-07-28T00:05:00Z"))?;
    assert_eq!(
        consumer.consume_at_for_debug_test(
            &grant,
            &expected(),
            DELIVERED_PAYLOAD,
            "expiry-after-transaction",
            "2026-07-28T00:04:59Z"
        ),
        Err(AuthenticatedDeliveryGrantConsumeError::Expired)
    );
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(count, 0);
    Ok(())
}
