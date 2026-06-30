use crate::support::{OptionTestExt as _, ResultTestExt as _};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_lan_core::lan_pairing::{
    evaluate_lan_discovery, evaluate_lan_mdns_advertisement_lifecycle,
    lan_discovery_decision_recorded_event, signed_child_agent_public_key_id,
    verify_lan_signed_child_agent_envelope, LanAggregateId, LanDiscoveryActionState,
    LanDiscoveryDecisionId, LanDiscoveryInput, LanInterfaceState,
    LanMdnsAdvertisementLifecycleAction, LanMdnsAdvertisementLifecycleInput,
    LanMdnsAdvertisementPlatformSupport, LanPairingActionState, LanPeerTrustState, LanRelayState,
    LanSignedChildAgentReplayGuard, LanSignedChildAgentVerificationContext,
    LanSignedChildAgentVerificationError,
};
use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequirement, ChildDomainPolicyEvaluationRequirement, ChildRuntimeDomain,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LanSignedChildAgentClaim, LanSignedChildAgentEnvelope, LanSignedChildAgentMessageKind,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceConfidence, LanCanonicalHouseholdDeviceSource,
    LanCanonicalHouseholdRouteState, LanDiscoveryEventHistoryState, LanDiscoveryEventKind,
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceSource,
    LanHouseholdDeviceActionKind, LanHouseholdDeviceDecision, LanPairingDiscoverySource,
    LanSelectedDeviceReadiness, LanServiceIdentityProbeEvidence,
    LanServiceIdentityProbeEvidenceKind,
};
use ocentra_parent_agent_protocol::LanTrustedDeviceRegistryEntry;

#[test]
fn lan_observation_records_presence_evidence_and_requests_policy() {
    let observed = ocentra_lan_core::lan_pairing::default_lan_observed_event();
    let evidence = ocentra_lan_core::lan_pairing::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_pairing::lan_ai_analysis_requested_event(&evidence);
    let policy = ocentra_lan_core::lan_pairing::lan_policy_evaluation_requested_event(&evidence)
        .expect_value("LAN policy request is expected");

    assert_eq!(
        observed.event_type,
        ChildRuntimeDomain::Lan.observed_event_type()
    );
    assert_eq!(
        evidence.event_type,
        ChildRuntimeDomain::Lan.evidence_recorded_event_type()
    );
    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert!(ai.is_none());
    assert_eq!(
        policy.event_type,
        ChildRuntimeDomain::Lan.policy_evaluation_requested_event_type()
    );
    assert_eq!(policy.evidence_refs, vec![evidence.evidence_ref]);
}

#[test]
fn lan_unknown_peer_requests_ai_before_policy() {
    let observed = ocentra_lan_core::lan_pairing::lan_observed_event(
        ocentra_lan_core::lan_pairing::LanObservationIntent::UnknownPeerRequiresAi,
    );
    let evidence = ocentra_lan_core::lan_pairing::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_pairing::lan_ai_analysis_requested_event(&evidence)
        .expect_value("unknown LAN peer requires AI boundary");
    let policy = ocentra_lan_core::lan_pairing::lan_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::Required
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::Required
    );
    assert_eq!(
        ai.event_type,
        ChildRuntimeDomain::Lan.ai_analysis_requested_event_type()
    );
    assert_eq!(ai.evidence_refs, vec![evidence.evidence_ref]);
    assert!(policy.is_none());
}

#[test]
fn lan_discovery_observation_only_records_no_ai_or_policy_work() {
    let observed = ocentra_lan_core::lan_pairing::lan_observed_event(
        ocentra_lan_core::lan_pairing::LanObservationIntent::DiscoveryObservationOnly,
    );
    let evidence = ocentra_lan_core::lan_pairing::lan_evidence_recorded_event(&observed);
    let ai = ocentra_lan_core::lan_pairing::lan_ai_analysis_requested_event(&evidence);
    let policy = ocentra_lan_core::lan_pairing::lan_policy_evaluation_requested_event(&evidence);

    assert_eq!(
        evidence.ai_analysis_requirement,
        ChildDomainAiAnalysisRequirement::NotRequired
    );
    assert_eq!(
        evidence.policy_evaluation_requirement,
        ChildDomainPolicyEvaluationRequirement::NotRequired
    );
    assert!(ai.is_none());
    assert!(policy.is_none());
}

#[test]
fn lan_discovery_allows_signed_pairing_for_trusted_local_peer() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Available,
        peer_trust_state: LanPeerTrustState::Trusted,
        relay_state: LanRelayState::LocalDirect,
    });

    assert_eq!(
        decision.discovery_action_state,
        LanDiscoveryActionState::AdvertiseAndListen
    );
    assert_eq!(
        decision.pairing_action_state,
        LanPairingActionState::AllowSignedPairing
    );
}

#[test]
fn lan_discovery_unknown_peer_requires_review_not_pairing_authority() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Available,
        peer_trust_state: LanPeerTrustState::Unknown,
        relay_state: LanRelayState::LocalDirect,
    });

    assert_eq!(
        decision.pairing_action_state,
        LanPairingActionState::RequireAiOrManualReview
    );
}

#[test]
fn lan_discovery_blocks_when_interface_is_unavailable() {
    let decision = evaluate_lan_discovery(LanDiscoveryInput {
        interface_state: LanInterfaceState::Unavailable,
        peer_trust_state: LanPeerTrustState::Trusted,
        relay_state: LanRelayState::RelayRequired,
    });

    assert_eq!(
        decision.discovery_action_state,
        LanDiscoveryActionState::ManualRequired
    );
    assert_eq!(decision.pairing_action_state, LanPairingActionState::Block);
}

#[test]
fn lan_discovery_decision_is_recorded_as_typed_event() {
    let event = lan_discovery_decision_recorded_event(
        LanAggregateId::parse("lan-child-default").expect_value("lan aggregate"),
        LanDiscoveryDecisionId::parse("lan-discovery-decision-default")
            .expect_value("lan discovery decision"),
        LanDiscoveryInput {
            interface_state: LanInterfaceState::Available,
            peer_trust_state: LanPeerTrustState::Trusted,
            relay_state: LanRelayState::LocalDirect,
        },
    );

    assert_eq!(
        event.decision.discovery_action_state,
        LanDiscoveryActionState::AdvertiseAndListen
    );
    assert_eq!(
        event
            .contract()
            .expect_value("lan contract")
            .event_type
            .as_str(),
        "lan.discovery.decision-recorded"
    );
}

#[test]
fn mdns_lifecycle_start_update_stop_and_degraded_are_explicit_hint_only_states() {
    let start = evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
        desired_present: true,
        running: false,
        platform_support: LanMdnsAdvertisementPlatformSupport::Supported,
    });
    let update = evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
        desired_present: true,
        running: true,
        platform_support: LanMdnsAdvertisementPlatformSupport::Supported,
    });
    let stop = evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
        desired_present: false,
        running: true,
        platform_support: LanMdnsAdvertisementPlatformSupport::Supported,
    });
    let degraded = evaluate_lan_mdns_advertisement_lifecycle(LanMdnsAdvertisementLifecycleInput {
        desired_present: true,
        running: true,
        platform_support: LanMdnsAdvertisementPlatformSupport::UnsupportedPlatform,
    });

    assert_eq!(
        start.lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Start
    );
    assert_eq!(
        update.lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Update
    );
    assert_eq!(
        stop.lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Stop
    );
    assert_eq!(
        degraded.lifecycle_action,
        LanMdnsAdvertisementLifecycleAction::Degraded
    );
    assert!(start.hint_only);
    assert!(update.hint_only);
    assert!(stop.hint_only);
    assert!(degraded.hint_only);
}

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
    .value_or_unreachable("signed hello verifies");
    let verified_heartbeat = verify_lan_signed_child_agent_envelope(
        &heartbeat,
        "2026-06-26T10:00:30Z",
        &signed_child_context(Some(constants::lan_pairing::CHILD_DEVICE_ID)),
        &mut replay_guard,
    )
    .value_or_unreachable("signed heartbeat verifies");

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
        &signed_child_context(None),
        &mut replay_guard,
    )
    .is_ok());
    assert_eq!(
        verify_lan_signed_child_agent_envelope(
            &envelope,
            "2026-06-26T10:00:31Z",
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
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
            &signed_child_context(None),
            &mut LanSignedChildAgentReplayGuard::new(),
        ),
        Err(LanSignedChildAgentVerificationError::MalformedTimestamp)
    );
}

#[test]
fn lan_read_model_projects_snapshot_events_for_visible_unknown_devices() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        unknown_network_device_with_sources(vec![
            LanDiscoveryEvidenceSource::WindowsNeighborTable,
            LanDiscoveryEvidenceSource::DnsCache,
        ]),
    ]));

    assert_eq!(
        model.discovery_event_history.state,
        LanDiscoveryEventHistoryState::Ready
    );
    assert!(model.discovery_event_history.rows.len() >= 4);
    assert_eq!(
        model.discovery_event_history.latest_event_id,
        model
            .discovery_event_history
            .rows
            .last()
            .map(|row| row.event_id.clone())
    );
    assert!(model.discovery_event_history.rows.iter().all(|row| {
        row.scan_session_id
            .as_deref()
            .map(|value| value.starts_with("lan-scan-"))
            .unwrap_or(false)
    }));
    assert_eq!(
        model.discovery_event_history.rows[0].event_kind,
        LanDiscoveryEventKind::ScanStarted
    );
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .any(|row| row.event_kind == LanDiscoveryEventKind::UnknownDetected));
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .any(|row| row.event_kind == LanDiscoveryEventKind::DeviceOnline));
    assert_eq!(
        model
            .discovery_event_history
            .rows
            .last()
            .map(|row| &row.event_kind),
        Some(&LanDiscoveryEventKind::ScanFinished)
    );
    assert!(model
        .discovery_event_history
        .rows
        .iter()
        .skip(1)
        .all(|row| row.previous_event_id.is_some()));
    assert!(model.discovery_event_history.rows.iter().any(|row| {
        row.event_kind == LanDiscoveryEventKind::EvidenceFound
            && row.evidence_id.is_some()
            && row.affected_device_id.is_some()
    }));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("unknown LAN device is projected");
    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
}

#[test]
fn locally_administered_mac_does_not_merge_distinct_neighbor_devices() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        unknown_network_device(
            "lan-randomized-one",
            "Printer One",
            "192.168.1.31",
            "02-aa-bb-cc-dd-ee",
            "printer-one.local",
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
        unknown_network_device(
            "lan-randomized-two",
            "Printer Two",
            "192.168.1.32",
            "02-aa-bb-cc-dd-ee",
            "printer-two.local",
            vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(model.canonical_household_devices.iter().all(|device| {
        !device.enrollable
            && device.network_identity.confidence
                == LanCanonicalHouseholdDeviceConfidence::ManualRequired
    }));
    assert!(model.canonical_household_devices.iter().all(|device| {
        device
            .network_identity
            .evidence_records
            .iter()
            .any(|record| {
                record.evidence_kind == LanDiscoveryEvidenceKind::Vendor
                    && record.confidence == LanDiscoveryEvidenceConfidence::ManualRequired
            })
    }));
}

#[test]
fn service_probe_hints_make_device_visible_without_control_authority() {
    let model =
        build_lan_add_device_read_model(lan_read_model_input(vec![service_probe_only_device()]));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("service-probed LAN device is projected");

    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert_eq!(
        canonical.route_state,
        LanCanonicalHouseholdRouteState::Unavailable
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::ServiceIdentityProbe
                && record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
}

#[test]
fn service_probe_hints_stay_weak_when_previous_scan_hints_exist() {
    let mut device = service_probe_only_device();
    device.hint_sources = vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot];

    let model = build_lan_add_device_read_model(lan_read_model_input(vec![device]));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("service-probed LAN device is projected");

    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::ServiceIdentityProbe
                && record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::PreviousScanSnapshot
                && record.evidence_kind == LanDiscoveryEvidenceKind::HistoricalIdentityHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
}

#[test]
fn dns_cache_and_netbios_hostname_evidence_stays_weak() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        unknown_network_device_with_sources(vec![
            LanDiscoveryEvidenceSource::WindowsNeighborTable,
            LanDiscoveryEvidenceSource::DnsCache,
            LanDiscoveryEvidenceSource::Netbios,
            LanDiscoveryEvidenceSource::Llmnr,
        ]),
    ]));
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("canonical LAN device");
    let hostname_records = canonical
        .network_identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::Hostname)
        .collect::<Vec<_>>();

    assert!(hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::DnsCache
            && record.confidence == LanDiscoveryEvidenceConfidence::Weak
    }));
    assert!(hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::Netbios
            && record.confidence == LanDiscoveryEvidenceConfidence::Weak
    }));
    assert!(hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::Llmnr
            && record.confidence == LanDiscoveryEvidenceConfidence::Weak
    }));
    assert!(!hostname_records.iter().any(|record| {
        record.source == LanDiscoveryEvidenceSource::WindowsNeighborTable
            && record.confidence == LanDiscoveryEvidenceConfidence::Strong
    }));
}

#[test]
fn household_revoke_decision_records_audit_evidence_and_blocks_control() {
    let mut input = lan_read_model_input(vec![unknown_network_device_with_sources(vec![
        LanDiscoveryEvidenceSource::WindowsNeighborTable,
    ])]);
    input.household_device_decisions = vec![LanHouseholdDeviceDecision {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        action_id: "household-action-revoke-1".to_string(),
        action_kind: LanHouseholdDeviceActionKind::Revoke,
        canonical_device_id: canonical_test_mac_device_id(),
        child_profile_id: Some("child-profile-1".to_string()),
        display_name: Some("Kitchen Printer".to_string()),
        device_kind: Some(constants::lan_pairing::HOUSEHOLD_DEVICE_KIND_UNKNOWN.to_string()),
        parent_actor_id: "parent-1".to_string(),
        decided_at: "2026-06-26T10:01:00Z".to_string(),
        revoked_at: None,
    }];

    let model = build_lan_add_device_read_model(input);
    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("canonical LAN device");

    assert_eq!(canonical.display_name, "Kitchen Printer");
    assert_eq!(
        canonical.discovery_state,
        LanPairingProductionDiscoveryState::Revoked
    );
    assert_eq!(canonical.trust_state, LanPairingTrustState::Revoked);
    assert_eq!(canonical.route_id, None);
    assert_eq!(
        canonical.route_state,
        LanCanonicalHouseholdRouteState::Unavailable
    );
    assert!(!canonical.enrollable);
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::ParentAssignment
                && record.evidence_kind == LanDiscoveryEvidenceKind::ParentDecision
                && record.value == constants::lan_pairing::HOUSEHOLD_ACTION_REVOKE
                && record.confidence == LanDiscoveryEvidenceConfidence::Rejected
        }));
}

#[test]
fn conflicting_child_profile_assignments_do_not_merge_even_with_matching_mac() {
    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        child_profile_device(
            "lan-device-child-profile-one",
            "child-profile-1",
            "Alpha Tablet",
        ),
        child_profile_device(
            "lan-device-child-profile-two",
            "child-profile-2",
            "Alpha Tablet",
        ),
    ]));

    assert_eq!(model.canonical_household_devices.len(), 2);
    assert!(model.canonical_household_devices.iter().all(
        |device| device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
    ));
    assert!(model.canonical_household_devices.iter().any(|device| {
        device.canonical_device_id.starts_with("lan-child-profile-")
            && device
                .source_labels
                .contains(&LanCanonicalHouseholdDeviceSource::NetworkNeighbor)
    }));
}

#[test]
fn paired_registry_truth_outweighs_previous_scan_hint() {
    let discovered = hinted_child_profile_device(
        "lan-device-child-profile-hint",
        "child-profile-3",
        "Kitchen Tablet",
    );
    let registry_entry = trusted_registry_entry_for_child_profile(
        "lan-device-child-profile-hint",
        "child-profile-3",
        "Kitchen Tablet",
    );

    let model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:00:30Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: vec![discovered],
        pairing_requests: Vec::new(),
        trusted_device_registry: vec![registry_entry],
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });

    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("canonical LAN device");
    assert_eq!(model.canonical_household_devices.len(), 1);
    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(canonical.trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        canonical.network_identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::AgentConfirmed
    );
    let inventory = canonical
        .child_agent_inventory
        .as_ref()
        .value_or_unreachable("trusted child inventory");
    assert_eq!(inventory.device_name, "tablet.local");
    assert_eq!(inventory.pairing_trust_state, LanPairingTrustState::Paired);
    assert_eq!(
        inventory.route_state,
        LanCanonicalHouseholdRouteState::LocalNetwork
    );
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::PreviousScanSnapshot
                && record.evidence_kind == LanDiscoveryEvidenceKind::HistoricalIdentityHint
                && record.confidence == LanDiscoveryEvidenceConfidence::Weak
        }));
    assert!(canonical
        .network_identity
        .evidence_records
        .iter()
        .any(|record| {
            record.source == LanDiscoveryEvidenceSource::TrustedRegistry
                && record.evidence_kind == LanDiscoveryEvidenceKind::TrustedRegistry
                && record.confidence == LanDiscoveryEvidenceConfidence::ManualRequired
        }));
}

#[test]
fn router_classification_is_visible_but_not_controllable() {
    let mut child_device = LanPairingDeviceRef::new(
        "lan-device-router".to_string(),
        None,
        "Gateway".to_string(),
        constants::lan_pairing::PLATFORM_ROUTER.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_ROUTER_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_ROUTER_MAC.to_string());
    child_device.hostname = Some("gateway.local".to_string());
    child_device.network_interface =
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let model = build_lan_add_device_read_model(lan_read_model_input(vec![
        LanBrowserAddDeviceDiscoveryDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            discovered_at: "2026-06-26T10:00:30Z".to_string(),
            child_device,
            agent_peer_id: "lan-device-router".to_string(),
            pairing_id: None,
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
            discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            evidence_sources: vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
            hint_sources: Vec::new(),
            service_identity_probe_evidence: Vec::new(),
        },
    ]));

    let canonical = model
        .canonical_household_devices
        .first()
        .value_or_unreachable("canonical router device");
    assert_eq!(
        canonical.classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    );
    assert!(!canonical.enrollable);
    assert!(canonical.child_agent_inventory.is_none());
    assert_eq!(
        canonical.route_state,
        LanCanonicalHouseholdRouteState::Unavailable
    );
}

fn signed_child_envelope(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: &str,
    sequence: u64,
    expires_at: &str,
) -> LanSignedChildAgentEnvelope {
    signed_child_envelope_with_family(message_kind, nonce, sequence, expires_at, "sha256:family-1")
}

fn signed_child_envelope_with_family(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: &str,
    sequence: u64,
    expires_at: &str,
    family_hash: &str,
) -> LanSignedChildAgentEnvelope {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let claim = LanSignedChildAgentClaim {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        message_kind,
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        install_id: "child-install-1".to_string(),
        family_hash: family_hash.to_string(),
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
        issued_at: "2026-06-26T10:00:00Z".to_string(),
        expires_at: expires_at.to_string(),
    };
    let payload = serde_json::to_vec(&claim).value_or_unreachable("claim serializes");
    let signature = signing_key.sign(&payload);
    LanSignedChildAgentEnvelope {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        claim,
        public_key_base64: STANDARD.encode(verifying_key.to_bytes()),
        public_key_id: signed_child_agent_public_key_id(&verifying_key),
        signature_base64: STANDARD.encode(signature.to_bytes()),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

fn signed_child_envelope_with_claim(
    claim: LanSignedChildAgentClaim,
) -> LanSignedChildAgentEnvelope {
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let payload = serde_json::to_vec(&claim).value_or_unreachable("claim serializes");
    let signature = signing_key.sign(&payload);

    LanSignedChildAgentEnvelope {
        schema_version: claim.schema_version,
        claim,
        public_key_base64: STANDARD.encode(verifying_key.to_bytes()),
        public_key_id: signed_child_agent_public_key_id(&verifying_key),
        signature_base64: STANDARD.encode(signature.to_bytes()),
        signature_algorithm: constants::lan_pairing::SIGNED_CHILD_AGENT_SIGNATURE_ALGORITHM_ED25519
            .to_string(),
    }
}

fn signed_child_context(
    expected_child_device_id: Option<&str>,
) -> LanSignedChildAgentVerificationContext {
    LanSignedChildAgentVerificationContext {
        expected_parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        expected_family_hash: "sha256:family-1".to_string(),
        expected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        expected_child_device_id: expected_child_device_id.map(ToString::to_string),
    }
}

fn lan_read_model_input(
    discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
) -> LanAddDeviceReadModelInput {
    LanAddDeviceReadModelInput {
        generated_at: "2026-06-26T10:00:30Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    }
}

fn canonical_test_mac_device_id() -> String {
    format!(
        "{}{}-{}",
        constants::lan_pairing::CANONICAL_DEVICE_MAC_PREFIX,
        constants::lan_pairing::TEST_LAN_MAC
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>(),
        "lan-device-54271e97c331"
            .chars()
            .filter(|character| character.is_ascii_alphanumeric())
            .flat_map(char::to_lowercase)
            .collect::<String>()
    )
}

fn child_profile_device(
    device_id: &str,
    child_profile_id: &str,
    label: &str,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let mut child_device = LanPairingDeviceRef::new(
        device_id.to_string(),
        Some(child_profile_id.to_string()),
        label.to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    child_device.hostname = Some("tablet.local".to_string());
    child_device.network_interface = Some("Ethernet".to_string());

    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: "2026-06-26T10:00:30Z".to_string(),
        child_device,
        agent_peer_id: device_id.to_string(),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}

fn hinted_child_profile_device(
    device_id: &str,
    child_profile_id: &str,
    label: &str,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let mut device = child_profile_device(device_id, child_profile_id, label);
    device.hint_sources = vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot];
    device
}

fn trusted_registry_entry_for_child_profile(
    device_id: &str,
    child_profile_id: &str,
    label: &str,
) -> LanTrustedDeviceRegistryEntry {
    let mut child_device = LanPairingDeviceRef::new(
        device_id.to_string(),
        Some(child_profile_id.to_string()),
        label.to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    child_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    child_device.hostname = Some("tablet.local".to_string());
    child_device.network_interface = Some("Ethernet".to_string());

    LanTrustedDeviceRegistryEntry {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        pairing_id: "pairing-child-profile-3".to_string(),
        child_device,
        parent_device: LanPairingDeviceRef::new(
            constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
            None,
            "Parent".to_string(),
            constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        origin: "test-trusted-registry".to_string(),
        proof_digest: "sha256:test-proof".to_string(),
        trust_state: LanPairingTrustState::Paired,
        trusted_at: "2026-06-26T09:59:00Z".to_string(),
        expires_at: "2026-06-27T09:59:00Z".to_string(),
        revoked_at: None,
    }
}

fn unknown_network_device_with_sources(
    evidence_sources: Vec<LanDiscoveryEvidenceSource>,
) -> LanBrowserAddDeviceDiscoveryDevice {
    unknown_network_device(
        "lan-device-54271e97c331",
        "mystery-device.local",
        constants::lan_pairing::TEST_LAN_IP,
        constants::lan_pairing::TEST_LAN_MAC,
        "mystery-device.local",
        evidence_sources,
    )
}

fn unknown_network_device(
    device_id: &str,
    label: &str,
    ip_address: &str,
    mac_address: &str,
    hostname: &str,
    evidence_sources: Vec<LanDiscoveryEvidenceSource>,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let mut child_device = LanPairingDeviceRef::new(
        device_id.to_string(),
        None,
        label.to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.ip_address = Some(ip_address.to_string());
    child_device.mac_address = Some(mac_address.to_string());
    child_device.hostname = Some(hostname.to_string());
    child_device.network_interface = Some("Ethernet".to_string());

    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: "2026-06-26T10:00:30Z".to_string(),
        child_device,
        agent_peer_id: device_id.to_string(),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources,
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}

fn service_probe_only_device() -> LanBrowserAddDeviceDiscoveryDevice {
    let child_device = LanPairingDeviceRef::new(
        "lan-service-probe-only".to_string(),
        None,
        "HTTP Printer".to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );

    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: "2026-06-26T10:00:30Z".to_string(),
        child_device,
        agent_peer_id: "lan-service-probe-only".to_string(),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: vec![LanDiscoveryEvidenceSource::ServiceIdentityProbe],
        hint_sources: Vec::new(),
        service_identity_probe_evidence: vec![LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::ServerHeader,
            value: "IPP/2.0".to_string(),
            selected_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        }],
    }
}
