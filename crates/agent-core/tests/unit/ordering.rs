use super::*;

#[test]
fn consumer_rejects_oversized_correlation_before_validation_audit_persistence() -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("oversized-correlation-before-audit");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let mut grant = signed_grant(&key);
    grant.target_device_id = "tampered-target-device".to_owned();
    assert_eq!(consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, "x".repeat(ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES + 1)), Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected));
    drop(consumer);
    let connection = Connection::open(path.as_ref())?;
    let count: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_audits_v2",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(count, 0);
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
