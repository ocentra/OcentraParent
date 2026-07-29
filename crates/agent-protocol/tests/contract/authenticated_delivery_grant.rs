use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantValidationError,
    AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES, AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
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
        dry_run: false,
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-07-28T00:00:00Z".to_owned(),
        expires_at: "2026-07-28T00:05:00Z".to_owned(),
        revocation_version: "revocation-1".to_owned(),
        signature: vec![7; 64],
    }
}

#[test]
fn authenticated_delivery_grant_round_trips_and_binds_every_security_field() -> TestResult {
    let original = grant();
    let encoded = serde_json::to_string(&original)?;
    let decoded: AuthenticatedDeliveryGrant = serde_json::from_str(&encoded)?;
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
        decode_failure_reason(unknown)?.split(", expected ").next(),
        Some("unknown field `unexpected`")
    );

    let mut individual_oversize = serde_json::to_value(grant())?;
    grant_object(&mut individual_oversize)?.insert(
        "issuerActorId".to_owned(),
        serde_json::Value::String("a".repeat(AUTHENTICATED_DELIVERY_GRANT_MAX_FIELD_BYTES + 1)),
    );
    assert_eq!(
        decode_failure_reason(individual_oversize)?,
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
        decode_failure_reason(aggregate_oversize)?,
        "authenticated delivery grant signed wire fields exceed their byte limit"
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

fn decode_failure_reason(value: serde_json::Value) -> TestResult<String> {
    match serde_json::from_value::<AuthenticatedDeliveryGrant>(value) {
        Ok(_) => Err(std::io::Error::other("malformed grant unexpectedly decoded").into()),
        Err(error) => Ok(error.to_string()),
    }
}
