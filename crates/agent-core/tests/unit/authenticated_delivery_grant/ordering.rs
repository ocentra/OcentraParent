use super::*;

#[test]
fn consumer_persists_bounded_redacted_audits_for_invalid_correlation_before_grant_validation(
) -> TestResult {
    let key = SigningKey::from_bytes(&[4; 32]);
    let path = store_path("invalid-correlation-before-audit");
    let mut consumer = open(&path, trusted_issuer(&key))?;
    let mut grant = signed_grant(&key);
    grant.target_device_id = "tampered-target-device".to_owned();
    let oversized_correlation = "x".repeat(
        ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES
            + 1,
    );
    assert_eq!(
        consumer.consume(&signed_grant(&key), &expected(), DELIVERED_PAYLOAD, ""),
        Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
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
    let audits: Vec<(String, String)> = connection
        .prepare(
            "SELECT audit_scope, audit_json FROM authenticated_delivery_grant_audits_v2 ORDER BY rowid",
        )?
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<Result<_, _>>()?;
    assert_eq!(audits.len(), 2);
    for (scope, audit_json) in audits {
        assert_eq!(scope, "validation-rejection");
        assert!(audit_json.len() < 1_024);
        let mut audit_keys = serde_json::from_str::<serde_json::Value>(&audit_json)?
            .as_object()
            .ok_or_else(|| std::io::Error::other("audit must be a JSON object"))?
            .keys()
            .cloned()
            .collect::<Vec<_>>();
        audit_keys.sort();
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
        assert_eq!(audit.correlation_id.len(), 64);
        assert_ne!(audit.correlation_id, oversized_correlation);
        assert_eq!(audit.issuer_key_id_digest.len(), 64);
        assert_eq!(audit.nonce_digest.len(), 64);
        assert_eq!(audit.grant_digest.len(), 64);
        assert_ne!(audit.grant_digest, "tampered-target-device");
        assert_eq!(
            audit.outcome,
            AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
                AuthenticatedDeliveryGrantValidationRejection::BindingRejected
            )
        );
    }
    Ok(())
}

#[test]
fn consumer_audits_distinct_oversized_correlation_suffixes_with_distinct_bounded_digests(
) -> TestResult {
    let key = SigningKey::from_bytes(&[25; 32]);
    let path = store_path("oversized-correlation-tail-audit");
    let shared_prefix = "x".repeat(
        ocentra_schema::authenticated_delivery_grant::AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    );
    let first = format!("{shared_prefix}a");
    let second = format!("{shared_prefix}b");
    let mut consumer = open(&path, trusted_issuer(&key))?;

    for correlation in [&first, &second] {
        assert_eq!(
            consumer.consume(
                &signed_grant(&key),
                &expected(),
                DELIVERED_PAYLOAD,
                correlation
            ),
            Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
        );
    }
    drop(consumer);

    let connection = Connection::open(path.as_ref())?;
    let correlations: Vec<String> = connection
        .prepare(
            "SELECT audit_json FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' ORDER BY rowid",
        )?
        .query_map([], |row| row.get::<_, String>(0))?
        .map(|row| {
            let audit: AuthenticatedDeliveryGrantAudit = serde_json::from_str(&row?)?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(audit.correlation_id)
        })
        .collect::<Result<_, _>>()?;
    assert_eq!(correlations.len(), 2);
    assert!(correlations.iter().all(|value| value.len() == 64));
    assert_ne!(correlations[0], correlations[1]);
    assert_ne!(correlations[0], first);
    assert_ne!(correlations[1], second);
    Ok(())
}

#[test]
fn consumer_audits_sqlite_out_of_range_expiry_before_consumption_and_on_retry() -> TestResult {
    let key = SigningKey::from_bytes(&[26; 32]);
    let path = store_path("expiry-outside-sqlite-nanos");
    let mut grant = signed_grant(&key);
    grant.expires_at = "2500-01-01T00:00:00Z".to_owned();
    grant.signature = key.sign(&grant.signing_bytes()).to_bytes().to_vec();
    let mut consumer = open(&path, trusted_issuer(&key))?;

    for correlation in ["out-of-range-expiry-first", "out-of-range-expiry-retry"] {
        assert_eq!(
            consumer.consume(&grant, &expected(), DELIVERED_PAYLOAD, correlation),
            Err(AuthenticatedDeliveryGrantConsumeError::BindingRejected)
        );
    }
    drop(consumer);

    let connection = Connection::open(path.as_ref())?;
    let consumed_rows: i64 = connection.query_row(
        "SELECT COUNT(*) FROM authenticated_delivery_grant_consumes_v2",
        [],
        |row| row.get(0),
    )?;
    assert_eq!(consumed_rows, 0);
    let audits: Vec<AuthenticatedDeliveryGrantAudit> = connection
        .prepare(
            "SELECT audit_json FROM authenticated_delivery_grant_audits_v2 WHERE audit_scope = 'validation-rejection' ORDER BY rowid",
        )?
        .query_map([], |row| row.get::<_, String>(0))?
        .map(|row| Ok::<_, Box<dyn std::error::Error + Send + Sync>>(serde_json::from_str(&row?)?))
        .collect::<Result<_, _>>()?;
    assert_eq!(audits.len(), 2);
    assert_eq!(
        audits
            .iter()
            .map(|audit| audit.correlation_id.clone())
            .collect::<Vec<_>>(),
        vec![
            super::storage_keys::stored_key("out-of-range-expiry-first"),
            super::storage_keys::stored_key("out-of-range-expiry-retry"),
        ]
    );
    assert!(audits.iter().all(|audit| {
        audit.outcome
            == AuthenticatedDeliveryGrantAuditOutcome::ValidationRejected(
                AuthenticatedDeliveryGrantValidationRejection::BindingRejected,
            )
    }));
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
