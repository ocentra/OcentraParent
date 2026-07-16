use super::*;

#[test]
fn signed_child_agent_hello_and_heartbeat_verify_with_real_ed25519_signature() {
    let mut replay_guard = LanSignedChildAgentReplayGuard::new();
    let hello = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-hello-1",
        1,
        "2026-06-26T10:05:00Z",
    );
    let heartbeat = signed_child_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-heartbeat-1",
        2,
        "2026-06-26T10:05:00Z",
    );

    let verified_hello = verify_lan_signed_child_agent_envelope(
        &hello,
        "2026-06-26T10:00:30Z",
        &signed_child_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        &mut replay_guard,
    )
    .value_or_unreachable();
    let verified_heartbeat = verify_lan_signed_child_agent_envelope(
        &heartbeat,
        "2026-06-26T10:00:30Z",
        &signed_child_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        &mut replay_guard,
    )
    .value_or_unreachable();

    assert_eq!(
        verified_hello.message_kind,
        LanSignedChildAgentMessageKind::Hello
    );
    assert_eq!(
        verified_heartbeat.message_kind,
        LanSignedChildAgentMessageKind::Heartbeat
    );
    assert_eq!(
        verified_hello.child_device_id,
        constants::lan_pairing::CHILD_DEVICE_ID
    );
    assert_eq!(verified_hello.install_id, "child-install-1");
    assert_eq!(
        verified_hello.platform,
        constants::lan_pairing::PLATFORM_WINDOWS
    );
    assert_eq!(
        verified_hello.hostname,
        constants::lan_pairing::TEST_HOSTNAME
    );
    assert_eq!(
        verified_hello.capabilities,
        vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ]
    );
    assert_eq!(replay_guard.observed_count(), 2);
}

#[test]
fn signed_child_agent_verifier_rejects_replay_expiry_and_tampered_payloads() {
    let mut replay_guard = LanSignedChildAgentReplayGuard::new();
    let envelope = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-replay-1",
        3,
        "2026-06-26T10:05:00Z",
    );
    assert!(verify_lan_signed_child_agent_envelope(
        &envelope,
        "2026-06-26T10:00:30Z",
        &signed_child_context(None::<&str>),
        &mut replay_guard,
    )
    .is_ok());
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &envelope,
            "2026-06-26T10:00:31Z",
            &signed_child_context(None::<&str>),
            &mut replay_guard,
        ),
        Err(LanSignedChildAgentVerificationError::Replayed)
    );
    assert_eq!(replay_guard.observed_count(), 1);

    let expired = signed_child_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-expired-1",
        4,
        "2026-06-26T09:59:00Z",
    );
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &expired,
            "2026-06-26T10:00:30Z",
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::Expired)
    );

    let mut tampered = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-tampered-1",
        5,
        "2026-06-26T10:05:00Z",
    );
    tampered.claim.route_id = constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string();
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &tampered,
            "2026-06-26T10:00:30Z",
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::SignatureRejected)
    );

    let wrong_family = signed_child_envelope_with_family(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-wrong-family-1",
        6,
        "2026-06-26T10:05:00Z",
        "sha256:other-family",
    );
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &wrong_family,
            "2026-06-26T10:00:30Z",
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::WrongFamily)
    );

    let wrong_child = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-wrong-child-1",
        7,
        "2026-06-26T10:05:00Z",
    );
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &wrong_child,
            "2026-06-26T10:00:30Z",
            &signed_child_context(Some("other-child-device")),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::WrongChildDevice)
    );
}

#[test]
fn signed_child_agent_verifier_rejects_empty_required_fields_wrong_parent_wrong_route_and_schema_version(
) {
    let observed_at = "2026-06-26T10:00:30Z";

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-empty-child-device-1",
                    8,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.child_device_id = String::new();
                claim
            }),
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::EmptyRequiredField)
    );

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-empty-nonce-1",
                    9,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.nonce = String::new();
                claim
            }),
            observed_at,
            &signed_child_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::EmptyRequiredField)
    );

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-wrong-parent-1",
                    10,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.parent_device_id = "sha256:other-parent".to_string();
                claim
            }),
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::WrongParentDevice)
    );

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-wrong-route-1",
                    11,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.route_id = constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string();
                claim
            }),
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::WrongRoute)
    );

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-wrong-schema-1",
                    12,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.schema_version = constants::lan_pairing::SCHEMA_VERSION + 1;
                claim
            }),
            observed_at,
            &signed_child_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::UnsupportedSchemaVersion)
    );
}

#[test]
fn signed_child_agent_verifier_rejects_invalid_transport_and_key_shapes() {
    let observed_at = "2026-06-26T10:00:30Z";

    let mut unsupported_algorithm = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-unsupported-algorithm-1",
        13,
        "2026-06-26T10:05:00Z",
    );
    unsupported_algorithm.signature_algorithm = "rsa-pss".to_string();
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &unsupported_algorithm,
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::UnsupportedAlgorithm)
    );

    let mut invalid_public_key = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-invalid-public-key-1",
        14,
        "2026-06-26T10:05:00Z",
    );
    invalid_public_key.public_key_base64 = "@@@not-base64@@@".to_string();
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &invalid_public_key,
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::InvalidPublicKey)
    );

    let mut mismatched_public_key_id = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-mismatched-public-key-id-1",
        15,
        "2026-06-26T10:05:00Z",
    );
    mismatched_public_key_id.public_key_id = "deadbeefdeadbeefdeadbeefdeadbeef".to_string();
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &mismatched_public_key_id,
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::PublicKeyIdMismatch)
    );

    let mut invalid_signature = signed_child_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-invalid-signature-1",
        16,
        "2026-06-26T10:05:00Z",
    );
    invalid_signature.signature_base64 = "@@@invalid-signature@@@".to_string();
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &invalid_signature,
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::InvalidSignature)
    );
}

#[test]
fn signed_child_agent_verifier_rejects_invalid_metadata_and_bad_timestamps() {
    let observed_at = "2026-06-26T10:00:30Z";

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-invalid-metadata-1",
                    17,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.hostname = "study laptop".to_string();
                claim
            }),
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::InvalidMetadata)
    );

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-future-issued-at-1",
                    18,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.issued_at = "2026-06-26T10:01:00Z".to_string();
                claim
            }),
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::FutureIssuedAt)
    );

    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &signed_child_envelope_with_claim({
                let mut claim = signed_child_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-malformed-issued-at-1",
                    19,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.issued_at = "not-a-timestamp".to_string();
                claim
            }),
            observed_at,
            &signed_child_context(None::<&str>),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::MalformedTimestamp)
    );
}
