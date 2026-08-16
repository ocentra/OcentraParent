use base64::{engine::general_purpose::STANDARD, Engine as _};
use chrono::{Duration, SecondsFormat, Utc};
use ed25519_dalek::{Signer, SigningKey};
use ocentra_lan_core::{
    lan_pairing::LanSignedChildAgentVerificationContext,
    network_inventory::passive_discovery::{
        LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
    },
};
use ocentra_parent_agent_protocol::{
    constants,
    lan_pairing::{
        LanPairingRejectionReason, LanSignedChildAgentClaim, LanSignedChildAgentEnvelope,
        LanSignedChildAgentMessageKind,
    },
    logging::{LogFieldValue, LogFields},
    transport::{AgentCommandName, AgentEventName},
};
use std::fmt::Display;

use crate::{
    app::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test},
    lan_pairing_test_assertions::assert_rejection,
    lan_pairing_test_commands::{command_for_target, local_network_target, serialize_command},
    test_invariants::require_ok,
    test_text::TestText,
};

#[test]
fn lan_pairing_runtime_accepts_signed_child_agent_hello_and_heartbeat_with_real_signatures() {
    let runtime = LanPairingRuntime::empty();
    let hello = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-hello-1",
        1,
        "2026-06-26T10:05:00Z",
    );
    let heartbeat = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-service-heartbeat-1",
        2,
        "2026-06-26T10:05:00Z",
    );

    let verified_hello = require_ok(
        runtime.verify_signed_child_agent_envelope(
            &hello,
            &TestText::from_display("2026-06-26T10:00:30Z"),
            &signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        ),
        "signed hello verifies through runtime replay guard",
    );
    let verified_heartbeat = require_ok(
        runtime.verify_signed_child_agent_envelope(
            &heartbeat,
            &TestText::from_display("2026-06-26T10:00:31Z"),
            &signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        ),
        "signed heartbeat verifies through runtime replay guard",
    );

    assert_eq!(
        verified_hello.message_kind,
        LanSignedChildAgentMessageKind::Hello
    );
    assert_eq!(verified_hello.install_id, "child-install-1");
    assert_eq!(verified_hello.family_hash, "sha256:family-1");
    assert_eq!(
        verified_hello.child_profile_hash.as_deref(),
        Some("sha256:child-profile-1")
    );
    assert_eq!(
        verified_hello.platform,
        constants::lan_pairing::PLATFORM_WINDOWS
    );
    assert_eq!(
        verified_hello.hostname,
        constants::lan_pairing::TEST_HOSTNAME
    );
    assert_eq!(verified_hello.agent_version, "1.2.3");
    assert_eq!(
        verified_hello.local_ips,
        vec![constants::lan_pairing::TEST_LAN_IP.to_string()]
    );
    assert_eq!(
        verified_hello.mac_addresses,
        vec![constants::lan_pairing::TEST_LAN_MAC.to_string()]
    );
    assert_eq!(
        verified_hello.capabilities,
        vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ]
    );
    assert_eq!(verified_hello.nonce, "nonce-service-hello-1");
    assert_eq!(
        verified_heartbeat.message_kind,
        LanSignedChildAgentMessageKind::Heartbeat
    );
    assert_eq!(verified_heartbeat.install_id, "child-install-1");
    assert_eq!(verified_heartbeat.nonce, "nonce-service-heartbeat-1");
    assert_eq!(
        verified_heartbeat.capabilities,
        vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ]
    );
    assert_eq!(runtime.signed_child_agent_replay_observation_count(), 2);
    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &heartbeat,
            &TestText::from_display("2026-06-26T10:00:32Z"),
            &signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID,)),
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::Replayed)
    );
}

#[test]
fn lan_pairing_runtime_rejects_invalid_signature_wrong_family_and_expired_signed_child_agent_envelopes(
) {
    let runtime = LanPairingRuntime::empty();
    let context = signed_child_agent_context(Some(constants::lan_pairing::CHILD_DEVICE_ID));
    let mut invalid_signature = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-invalid-signature-1",
        11,
        "2026-06-26T10:05:00Z",
    );
    invalid_signature.claim.nonce = "tampered-nonce".to_string();
    let mut wrong_family = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-service-wrong-family-1",
        12,
        "2026-06-26T10:05:00Z",
    );
    wrong_family = signed_child_agent_envelope_with_claim({
        let mut claim = wrong_family.claim;
        claim.family_hash = "sha256:family-2".to_string();
        claim
    });
    let mut expired = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-expired-1",
        13,
        "2026-06-26T09:55:00Z",
    );
    expired.claim.expires_at = "2026-06-26T09:54:59Z".to_string();

    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &invalid_signature,
            &TestText::from_display("2026-06-26T10:00:30Z"),
            &context,
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::SignatureRejected)
    );
    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &wrong_family,
            &TestText::from_display("2026-06-26T10:00:31Z"),
            &context,
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::WrongFamily)
    );
    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &expired,
            &TestText::from_display("2026-06-26T10:00:32Z"),
            &context,
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::Expired)
    );
}

#[test]
fn signed_child_agent_observation_records_passive_beacon_history_rows() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display(
            constants::lan_pairing::CHILD_DEVICE_ID,
        )),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let hello = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-passive-hello-1",
        71,
        "2026-06-26T10:05:00Z",
    );
    let heartbeat = signed_child_agent_envelope(
        LanSignedChildAgentMessageKind::Heartbeat,
        "nonce-passive-heartbeat-1",
        72,
        "2026-06-26T10:05:10Z",
    );

    require_ok(
        runtime.observe_signed_child_agent_envelope(
            &hello,
            &TestText::from_display("2026-06-26T10:00:30Z"),
        ),
        "hello observation",
    );
    require_ok(
        runtime.observe_signed_child_agent_envelope(
            &heartbeat,
            &TestText::from_display("2026-06-26T10:00:31Z"),
        ),
        "heartbeat observation",
    );

    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 2);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::OcentraBeacon)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(|id| id.as_str()),
        Some(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        snapshot.rows[1].source,
        Some(LanPassiveDiscoverySource::OcentraBeacon)
    );
    assert_eq!(
        snapshot.rows[1].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[1].device_id.as_ref().map(|id| id.as_str()),
        Some(constants::lan_pairing::CHILD_DEVICE_ID)
    );
    assert_eq!(
        snapshot.rows[0].summary,
        format!(
            "signed child hello observed: route={}; install-id=child-install-1",
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
        )
    );
    assert_eq!(
        snapshot.rows[1].summary,
        format!(
            "signed child heartbeat observed: route={}; install-id=child-install-1",
            constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK
        )
    );
}

#[tokio::test]
async fn lan_pairing_signed_child_agent_observe_command_verifies_and_reports() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display(
            constants::lan_pairing::CHILD_DEVICE_ID,
        )),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let issued_at =
        (Utc::now() - Duration::seconds(1)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at =
        (Utc::now() + Duration::minutes(5)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = signed_child_agent_envelope_with_window(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-command-1",
        31,
        &issued_at,
        &expires_at,
    );

    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;
    let replay = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime.clone(),
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentLanPairingSignedChildAgentReported
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_VERIFICATION),
        Some(&LogFieldValue::String(
            constants::value::LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED.to_string()
        ))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_MESSAGE_KIND),
        Some(&LogFieldValue::String("hello".to_string()))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::LAN_SIGNED_CHILD_AGENT_STATUS),
        Some(&LogFieldValue::String(
            constants::lan_pairing::PRODUCTION_PROOF_STATE_MANUAL_REQUIRED.to_string()
        ))
    );
    assert_rejection(&replay, constants::value::LAN_REASON_REPLAYED);
}

#[tokio::test]
async fn lan_pairing_signed_child_agent_observe_rejects_when_parent_context_is_unconfigured() {
    let runtime = LanPairingRuntime::empty();
    let issued_at =
        (Utc::now() - Duration::seconds(1)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at =
        (Utc::now() + Duration::minutes(5)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = signed_child_agent_envelope_with_window(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-command-no-context-1",
        32,
        &issued_at,
        &expires_at,
    );

    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(
        &event,
        constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE,
    );
}

#[tokio::test]
async fn lan_pairing_signed_child_agent_observe_rejects_when_child_context_is_unpaired() {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        None,
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let issued_at =
        (Utc::now() - Duration::seconds(1)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let expires_at =
        (Utc::now() + Duration::minutes(5)).to_rfc3339_opts(SecondsFormat::Millis, true);
    let envelope = signed_child_agent_envelope_with_window(
        LanSignedChildAgentMessageKind::Hello,
        "nonce-service-command-missing-child-1",
        33,
        &issued_at,
        &expires_at,
    );

    let event = handle_command_text_for_test(
        serialize_command(command_for_target(
            AgentCommandName::AgentLanPairingSignedChildAgentObserve,
            local_network_target(constants::lan_pairing::CHILD_DEVICE_ID),
            signed_child_agent_payload(&envelope),
        )),
        runtime,
        Some(TestText::from_display(
            constants::lan_pairing::ALLOWED_ORIGIN,
        )),
    )
    .await;

    assert_rejection(
        &event,
        constants::value::LAN_REASON_SIGNED_CHILD_AGENT_CONTEXT_UNAVAILABLE,
    );
}

#[test]
fn lan_pairing_runtime_rejects_malformed_signed_child_agent_envelope() {
    let runtime = LanPairingRuntime::empty();
    let envelope = LanSignedChildAgentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        claim: LanSignedChildAgentClaim {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            message_kind: LanSignedChildAgentMessageKind::Hello,
            child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            install_id: "child-install-1".to_string(),
            family_hash: "sha256:family-1".to_string(),
            child_profile_hash: Some("sha256:child-profile-1".to_string()),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            hostname: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            agent_version: "1.2.3".to_string(),
            local_ips: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
            mac_addresses: vec![constants::lan_pairing::TEST_LAN_MAC.to_string()],
            capabilities: vec![
                constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string()
            ],
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            nonce: "nonce-service-wrapper-1".to_string(),
            sequence: 1,
            issued_at: "2026-06-26T10:00:00Z".to_string(),
            expires_at: "2026-06-26T10:05:00Z".to_string(),
        },
        public_key_base64: "not-base64".to_string(),
        public_key_id: "bad-key".to_string(),
        signature_base64: "not-base64".to_string(),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    };
    let context = LanSignedChildAgentVerificationContext {
        expected_parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        expected_family_hash: "sha256:family-1".to_string(),
        expected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        expected_child_device_id: Some(constants::lan_pairing::CHILD_DEVICE_ID.to_string()),
    };

    assert_eq!(
        runtime.verify_signed_child_agent_envelope(
            &envelope,
            &TestText::from_display("2026-06-26T10:00:30Z"),
            &context,
        ),
        Err(ocentra_lan_core::lan_pairing::LanSignedChildAgentVerificationError::InvalidPublicKey)
    );
}

#[test]
fn lan_pairing_runtime_rejects_signed_child_agent_wrong_parent_wrong_route_empty_nonce_and_schema_version(
) {
    let runtime = LanPairingRuntime::empty_with_signed_child_agent_context(
        Some(TestText::from_display(
            constants::lan_pairing::CHILD_DEVICE_ID,
        )),
        TestText::from_display(constants::lan_pairing::PARENT_DEVICE_ID),
        TestText::from_display("sha256:family-1"),
        TestText::from_display(constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK),
    );
    let observed_at = TestText::from_display("2026-06-26T10:00:30Z");

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-runtime-empty-1",
                    41,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.nonce = String::new();
                claim
            }),
            &observed_at,
        ),
        Err(LanPairingRejectionReason::Malformed)
    );

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-runtime-parent-1",
                    42,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.parent_device_id = "sha256:other-parent".to_string();
                claim
            }),
            &observed_at,
        ),
        Err(LanPairingRejectionReason::WrongDevice)
    );

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Hello,
                    "nonce-runtime-route-1",
                    43,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.route_id = constants::lan_pairing::ROUTE_ID_SECOND_LOCAL_NETWORK.to_string();
                claim
            }),
            &observed_at,
        ),
        Err(LanPairingRejectionReason::UnsupportedRoute)
    );

    assert_eq!(
        runtime.observe_signed_child_agent_envelope(
            &signed_child_agent_envelope_with_claim({
                let mut claim = signed_child_agent_envelope(
                    LanSignedChildAgentMessageKind::Heartbeat,
                    "nonce-runtime-schema-1",
                    44,
                    "2026-06-26T10:05:00Z",
                )
                .claim;
                claim.schema_version = constants::lan_pairing::SCHEMA_VERSION + 1;
                claim
            }),
            &observed_at,
        ),
        Err(LanPairingRejectionReason::Malformed)
    );
}

fn signed_child_agent_context<T>(
    expected_child_device_id: Option<T>,
) -> LanSignedChildAgentVerificationContext
where
    T: Display,
{
    LanSignedChildAgentVerificationContext {
        expected_parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        expected_family_hash: "sha256:family-1".to_string(),
        expected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        expected_child_device_id: expected_child_device_id
            .map(TestText::from_display)
            .map(|value| value.to_string()),
    }
}

fn signed_child_agent_envelope<TNonce, TExpires>(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: TNonce,
    sequence: u64,
    expires_at: TExpires,
) -> LanSignedChildAgentEnvelope
where
    TNonce: Display,
    TExpires: Display,
{
    signed_child_agent_envelope_with_window(
        message_kind,
        nonce,
        sequence,
        "2026-06-26T10:00:00Z",
        expires_at,
    )
}

fn signed_child_agent_envelope_with_window<TNonce, TIssuedAt, TExpires>(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: TNonce,
    sequence: u64,
    issued_at: TIssuedAt,
    expires_at: TExpires,
) -> LanSignedChildAgentEnvelope
where
    TNonce: Display,
    TIssuedAt: Display,
    TExpires: Display,
{
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let nonce = TestText::from_display(nonce);
    let issued_at = TestText::from_display(issued_at);
    let expires_at = TestText::from_display(expires_at);
    let claim = LanSignedChildAgentClaim {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        message_kind,
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        install_id: "child-install-1".to_string(),
        family_hash: "sha256:family-1".to_string(),
        child_profile_hash: Some("sha256:child-profile-1".to_string()),
        platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
        hostname: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        agent_version: "1.2.3".to_string(),
        local_ips: vec![constants::lan_pairing::TEST_LAN_IP.to_string()],
        mac_addresses: vec![constants::lan_pairing::TEST_LAN_MAC.to_string()],
        capabilities: vec![
            constants::lan_pairing::CHILD_AGENT_CAPABILITY_PAIRING_ROUTE.to_string(),
            "future-safe-local-capability".to_string(),
        ],
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        nonce: nonce.to_string(),
        sequence,
        issued_at: issued_at.to_string(),
        expires_at: expires_at.to_string(),
    };
    let payload = require_ok(serde_json::to_vec(&claim), "signed child claim serializes");
    let signature = signing_key.sign(&payload);

    LanSignedChildAgentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        claim,
        public_key_base64: STANDARD.encode(verifying_key.as_bytes()),
        public_key_id: ocentra_lan_core::lan_pairing::signed_child_agent_public_key_id(
            &verifying_key,
        ),
        signature_base64: STANDARD.encode(signature.to_bytes()),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

fn signed_child_agent_envelope_with_claim(
    claim: LanSignedChildAgentClaim,
) -> LanSignedChildAgentEnvelope {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let payload = require_ok(serde_json::to_vec(&claim), "signed child claim serializes");
    let signature = signing_key.sign(&payload);

    LanSignedChildAgentEnvelope {
        schema_version: claim.schema_version,
        claim,
        public_key_base64: STANDARD.encode(verifying_key.as_bytes()),
        public_key_id: ocentra_lan_core::lan_pairing::signed_child_agent_public_key_id(
            &verifying_key,
        ),
        signature_base64: STANDARD.encode(signature.to_bytes()),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

fn signed_child_agent_payload(envelope: &LanSignedChildAgentEnvelope) -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::LAN_SIGNED_CHILD_AGENT_ENVELOPE_JSON.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(envelope),
            "signed child envelope serializes",
        )),
    );
    fields
}
