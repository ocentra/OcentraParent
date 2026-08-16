use ocentra_schema::authenticated_delivery_grant::{
    authenticated_delivery_grant_audit_fingerprint, AuthenticatedDeliveryGrant,
    AuthenticatedDeliveryGrantValidationError, AUTHENTICATED_DELIVERY_GRANT_MAX_ENCODED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES,
    AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn grant() -> AuthenticatedDeliveryGrant {
    AuthenticatedDeliveryGrant {
        schema_version: AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
        issuer_key_id: "parent-key-1".to_owned(),
        issuer_actor_id: "parent-actor-1".to_owned(),
        household_id: "household-1".to_owned(),
        parent_device_id: "parent-device-1".to_owned(),
        child_profile_id: "child-1".to_owned(),
        target_device_id: "child-device-1".to_owned(),
        policy_decision_id: "decision-1".to_owned(),
        policy_version: "1".to_owned(),
        action_id: "action-1".to_owned(),
        capability_id: "process-control".to_owned(),
        evidence_digest: "evidence-digest-1".to_owned(),
        payload_digest: "a".repeat(64),
        payload_length: 32,
        dry_run: false,
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-07-28T00:00:00Z".to_owned(),
        expires_at: "2026-07-28T00:05:00Z".to_owned(),
        revocation_version: "revocation-1".to_owned(),
        signature: vec![7; 64],
    }
}

#[test]
fn authenticated_delivery_grant_audit_fingerprint_is_domain_separated_and_signature_bound() {
    let grant = grant();
    let fingerprint = authenticated_delivery_grant_audit_fingerprint(&grant);
    assert_eq!(fingerprint.len(), 64);
    assert_ne!(fingerprint.as_bytes(), grant.signing_bytes().as_slice());

    let mut altered_signature = grant;
    altered_signature.signature[0] ^= 1;
    assert_ne!(
        fingerprint,
        authenticated_delivery_grant_audit_fingerprint(&altered_signature)
    );
}

#[test]
fn authenticated_delivery_grant_round_trips_and_binds_every_security_field() -> TestResult {
    let original = grant();
    let encoded = serde_json::to_string(&original)?;
    let decoded = AuthenticatedDeliveryGrant::decode_json_wire(&encoded)?;
    assert_eq!(decoded, original);
    assert_eq!(decoded.validate_shape(), Ok(()));
    let mut tampered = decoded.clone();
    tampered.target_device_id = "other-device".to_owned();
    assert_ne!(tampered.signing_bytes(), decoded.signing_bytes());
    Ok(())
}

#[test]
fn authenticated_delivery_grant_rejects_malformed_digest_and_time_window() {
    let mut malformed = grant();
    malformed.payload_digest = "not-a-digest".to_owned();
    assert_eq!(
        malformed.validate_shape(),
        Err(AuthenticatedDeliveryGrantValidationError::InvalidPayloadDigest)
    );
    let mut expired = grant();
    expired.issued_at = "2026-07-28T00:06:00Z".to_owned();
    assert_eq!(
        expired.validate_shape(),
        Err(AuthenticatedDeliveryGrantValidationError::InvalidTimeWindow)
    );
}

#[test]
fn authenticated_delivery_grant_requires_a_canonical_lowercase_sha256_payload_digest() {
    let mut lowercase_hex = grant();
    lowercase_hex.payload_digest = "0123456789abcdef".repeat(4);
    assert_eq!(lowercase_hex.validate_shape(), Ok(()));

    let mut uppercase = grant();
    uppercase.payload_digest = "A".repeat(64);
    assert_eq!(
        uppercase.validate_shape(),
        Err(AuthenticatedDeliveryGrantValidationError::InvalidPayloadDigest)
    );

    let mut non_hex_lowercase = grant();
    non_hex_lowercase.payload_digest = "z".repeat(64);
    assert_eq!(
        non_hex_lowercase.validate_shape(),
        Err(AuthenticatedDeliveryGrantValidationError::InvalidPayloadDigest)
    );
}

#[test]
fn authenticated_delivery_grant_compares_offset_timestamps_as_instants_and_rejects_malformed_ones()
{
    let mut offset_window = grant();
    offset_window.issued_at = "2026-07-28T01:00:00+01:00".to_owned();
    offset_window.expires_at = "2026-07-28T00:30:00Z".to_owned();
    assert_eq!(offset_window.validate_shape(), Ok(()));

    let mut malformed = grant();
    malformed.expires_at = "not-a-timestamp".to_owned();
    assert_eq!(
        malformed.validate_shape(),
        Err(AuthenticatedDeliveryGrantValidationError::InvalidTimestamp)
    );
}

#[test]
fn authenticated_delivery_grant_rejects_signed_or_non_decimal_fixed_width_timestamp_parts() {
    for malformed_timestamp in [
        "2026-+1-01T00:00:00Z",
        "2026-01-+1T00:00:00Z",
        "2026-01-01T+0:00:00Z",
        "2026-01-01T00:+0:00Z",
        "2026-01-01T00:00:+0Z",
        "2026-01-01T00:00:00++0:00",
        "2026-01-01T00:00:00+0+:00",
        "2026-01-01T00:00:00+00:+0",
        "202a-01-01T00:00:00Z",
    ] {
        let mut malformed = grant();
        malformed.expires_at = malformed_timestamp.to_owned();
        assert_eq!(
            malformed.validate_shape(),
            Err(AuthenticatedDeliveryGrantValidationError::InvalidTimestamp),
            "timestamp {malformed_timestamp} must be rejected"
        );
    }
}

#[test]
fn authenticated_delivery_grant_preserves_fractional_second_ordering() {
    let mut fraction = grant();
    fraction.issued_at = "2026-07-28T00:00:00.900Z".to_owned();
    fraction.expires_at = "2026-07-28T00:00:00.100Z".to_owned();
    assert_eq!(
        fraction.validate_shape(),
        Err(AuthenticatedDeliveryGrantValidationError::InvalidTimeWindow)
    );
}

#[test]
fn authenticated_delivery_grant_wire_decode_denies_unknown_and_oversized_fields() -> TestResult {
    let mut unknown = serde_json::to_value(grant())?;
    grant_object(&mut unknown)?.insert("unexpected".to_owned(), serde_json::Value::Bool(true));
    assert_eq!(
        decode_failure_reason(&unknown)?.split(", expected ").next(),
        Some("unknown field `unexpected`")
    );

    let mut individual_oversize = serde_json::to_value(grant())?;
    grant_object(&mut individual_oversize)?.insert(
        "issuerActorId".to_owned(),
        serde_json::Value::String("a".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES + 1)),
    );
    assert_eq!(
        decode_failure_reason(&individual_oversize)?,
        "authenticated delivery grant field exceeds its byte limit"
    );

    let mut aggregate_oversize = serde_json::to_value(grant())?;
    let fields = [
        "issuerKeyId",
        "issuerActorId",
        "householdId",
        "parentDeviceId",
        "childProfileId",
        "targetDeviceId",
        "policyDecisionId",
        "policyVersion",
    ];
    let object = grant_object(&mut aggregate_oversize)?;
    for field in fields {
        object.insert(
            field.to_owned(),
            serde_json::Value::String("a".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES)),
        );
    }
    assert_eq!(
        decode_failure_reason(&aggregate_oversize)?,
        "authenticated delivery grant signed wire fields exceed their byte limit"
    );
    Ok(())
}

#[test]
fn authenticated_delivery_grant_wire_decode_bounds_unknown_field_names_before_error_formatting(
) -> TestResult {
    let mut unknown = serde_json::to_value(grant())?;
    grant_object(&mut unknown)?.insert("x".repeat(129), serde_json::Value::Bool(true));
    assert_eq!(
        decode_failure_reason(&unknown)?,
        "authenticated delivery grant encoded field name exceeds its byte limit"
    );
    Ok(())
}

#[test]
fn authenticated_delivery_grant_canonical_wire_decode_rejects_oversized_outer_inputs() {
    for wire in [
        format!(
            "{{\"{}\":true}}",
            "x".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_ENCODED_WIRE_BYTES)
        ),
        format!(
            "{{\"issuerKeyId\":\"{}\"}}",
            "a".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_ENCODED_WIRE_BYTES)
        ),
    ] {
        assert_eq!(
            AuthenticatedDeliveryGrant::decode_json_wire(&wire),
            Err("authenticated delivery grant encoded wire exceeds its byte limit".to_owned())
        );
    }
}

#[test]
fn authenticated_delivery_grant_wire_decode_counts_outer_whitespace_against_its_limit() -> TestResult
{
    let encoded = serde_json::to_string(&grant())?;
    let padding = AUTHENTICATED_DELIVERY_GRANT_MAX_ENCODED_WIRE_BYTES + 1 - encoded.len();
    let wire = format!(
        "{}{}{}",
        " ".repeat(padding / 2),
        encoded,
        "\n".repeat(padding - (padding / 2)),
    );

    assert_eq!(
        AuthenticatedDeliveryGrant::decode_json_wire(&wire),
        Err("authenticated delivery grant encoded wire exceeds its byte limit".to_owned())
    );
    Ok(())
}

#[test]
fn authenticated_delivery_grant_wire_decode_accepts_valid_fixture_with_outer_whitespace(
) -> TestResult {
    let expected = grant();
    let actual = AuthenticatedDeliveryGrant::decode_json_wire(&format!(
        " \n{}\t ",
        serde_json::to_string(&expected)?
    ))?;
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn authenticated_delivery_grant_wire_decode_matches_framed_signing_wire_limit() -> TestResult {
    let accepted = grant_with_signing_wire_len(AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES);
    let decoded = AuthenticatedDeliveryGrant::decode_json_wire(&serde_json::to_string(&accepted)?)?;
    assert_eq!(decoded, accepted);
    assert_eq!(decoded.validate_shape(), Ok(()));

    let oversized =
        grant_with_signing_wire_len(AUTHENTICATED_DELIVERY_GRANT_MAX_SIGNED_WIRE_BYTES + 1);
    assert_eq!(
        decode_failure_reason(&serde_json::to_value(oversized)?)?,
        "authenticated delivery grant signed wire fields exceed their byte limit"
    );
    Ok(())
}

#[test]
fn authenticated_delivery_grant_wire_decode_rejects_escaped_oversize_before_unescaping(
) -> TestResult {
    let encoded_oversize = r#"\u0061"#.repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES + 1);
    let wire = format!(
        r#"{{"schemaVersion":1,"issuerKeyId":"parent-key-1","issuerActorId":"{encoded_oversize}"}}"#,
    );

    let error = match AuthenticatedDeliveryGrant::decode_json_wire(&wire) {
        Ok(_) => {
            return Err(std::io::Error::other(
                "escaped field exceeding encoded bound must not deserialize",
            )
            .into())
        }
        Err(error) => error,
    };
    assert!(
        error.starts_with("authenticated delivery grant encoded field exceeds its byte limit"),
        "unexpected decode error: {error}"
    );
    Ok(())
}

fn grant_object(
    value: &mut serde_json::Value,
) -> TestResult<&mut serde_json::Map<String, serde_json::Value>> {
    value
        .as_object_mut()
        .ok_or_else(|| std::io::Error::other("test grant must encode as an object").into())
}

fn grant_with_signing_wire_len(target: usize) -> AuthenticatedDeliveryGrant {
    let mut bounded = grant();
    let mut remaining = target.saturating_sub(bounded.signing_bytes().len());
    for field in [
        &mut bounded.issuer_key_id,
        &mut bounded.issuer_actor_id,
        &mut bounded.household_id,
        &mut bounded.parent_device_id,
        &mut bounded.child_profile_id,
        &mut bounded.target_device_id,
        &mut bounded.policy_decision_id,
        &mut bounded.policy_version,
        &mut bounded.action_id,
        &mut bounded.capability_id,
        &mut bounded.evidence_digest,
        &mut bounded.nonce,
        &mut bounded.issued_at,
        &mut bounded.expires_at,
        &mut bounded.revocation_version,
    ] {
        let available = AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES - field.len();
        let added = remaining.min(available);
        field.push_str(&"a".repeat(added));
        remaining -= added;
    }
    assert_eq!(
        remaining, 0,
        "test fields must reach the target signing wire length"
    );
    assert_eq!(bounded.signing_bytes().len(), target);
    bounded
}

fn decode_failure_reason(value: &serde_json::Value) -> TestResult<String> {
    match AuthenticatedDeliveryGrant::decode_json_wire(&serde_json::to_string(value)?) {
        Ok(_) => Err(std::io::Error::other("malformed grant unexpectedly decoded").into()),
        Err(error) => Ok(error
            .split(" at line ")
            .next()
            .unwrap_or_default()
            .to_owned()),
    }
}
