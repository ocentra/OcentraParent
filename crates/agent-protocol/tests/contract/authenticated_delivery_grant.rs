use ocentra_parent_agent_protocol::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrant, AuthenticatedDeliveryGrantValidationError,
    AUTHENTICATED_DELIVERY_GRANT_SCHEMA_VERSION,
};

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
fn authenticated_delivery_grant_round_trips_and_binds_every_security_field() {
    let original = grant();
    let encoded = serde_json::to_string(&original).expect("serialize test grant");
    let decoded: AuthenticatedDeliveryGrant = serde_json::from_str(&encoded).expect("decode grant");
    assert_eq!(decoded, original);
    assert_eq!(decoded.validate_shape(), Ok(()));
    let mut tampered = decoded.clone();
    tampered.target_device_id = "other-device".to_owned();
    assert_ne!(tampered.signing_bytes(), decoded.signing_bytes());
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
