use crate::support::{OptionTestExt as _, ResultTestExt as _};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_lan_core::lan_pairing::discovery::{LanAggregateId, LanDiscoveryDecisionId};
use ocentra_lan_core::lan_pairing::{
    evaluate_lan_discovery, evaluate_lan_mdns_advertisement_lifecycle,
    lan_discovery_decision_recorded_event, signed_child_agent_public_key_id,
    verify_lan_signed_child_agent_envelope, LanDiscoveryActionState, LanDiscoveryInput,
    LanInterfaceState, LanMdnsAdvertisementLifecycleAction, LanMdnsAdvertisementLifecycleInput,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LanText(String);

impl std::fmt::Display for LanText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

macro_rules! lt {
    ($value:expr) => {
        ($value).to_string()
    };
}

#[path = "lan_flow_read_model.rs"]
mod lan_flow_read_model;
#[path = "lan_flow_signed_child.rs"]
mod lan_flow_signed_child;

fn canonical_test_mac_device_id() -> LanText {
    LanText(format!(
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
    ))
}

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

fn signed_child_envelope(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: impl std::fmt::Display,
    sequence: u64,
    expires_at: impl std::fmt::Display,
) -> LanSignedChildAgentEnvelope {
    signed_child_envelope_with_family(
        message_kind,
        nonce,
        sequence,
        expires_at,
        lt!("sha256:family-1"),
    )
}

fn signed_child_envelope_with_family(
    message_kind: LanSignedChildAgentMessageKind,
    nonce: impl std::fmt::Display,
    sequence: u64,
    expires_at: impl std::fmt::Display,
    family_hash: impl std::fmt::Display,
) -> LanSignedChildAgentEnvelope {
    let nonce = nonce.to_string();
    let expires_at = expires_at.to_string();
    let family_hash = family_hash.to_string();
    let signing_key = SigningKey::from_bytes(&[7_u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let claim = LanSignedChildAgentClaim {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        message_kind,
        child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
        parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        install_id: "child-install-1".to_string(),
        family_hash,
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
        nonce,
        sequence,
        issued_at: "2026-06-26T10:00:00Z".to_string(),
        expires_at,
    };
    let payload = serde_json::to_vec(&claim).value_or_unreachable();
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
    let payload = serde_json::to_vec(&claim).value_or_unreachable();
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
    expected_child_device_id: Option<impl std::fmt::Display>,
) -> LanSignedChildAgentVerificationContext {
    LanSignedChildAgentVerificationContext {
        expected_parent_device_id: constants::lan_pairing::PARENT_DEVICE_ID.to_string(),
        expected_family_hash: "sha256:family-1".to_string(),
        expected_route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        expected_child_device_id: expected_child_device_id.map(|value| value.to_string()),
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

fn child_profile_device(
    device_id: impl std::fmt::Display,
    child_profile_id: impl std::fmt::Display,
    label: impl std::fmt::Display,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let device_id = device_id.to_string();
    let child_profile_id = child_profile_id.to_string();
    let label = label.to_string();
    let mut child_device = LanPairingDeviceRef::new(
        device_id.clone(),
        Some(child_profile_id),
        label,
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
        agent_peer_id: device_id,
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
    device_id: impl std::fmt::Display,
    child_profile_id: impl std::fmt::Display,
    label: impl std::fmt::Display,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let mut device = child_profile_device(device_id, child_profile_id, label);
    device.hint_sources = vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot];
    device
}

fn trusted_registry_entry_for_child_profile(
    device_id: impl std::fmt::Display,
    child_profile_id: impl std::fmt::Display,
    label: impl std::fmt::Display,
) -> LanTrustedDeviceRegistryEntry {
    let device_id = device_id.to_string();
    let child_profile_id = child_profile_id.to_string();
    let label = label.to_string();
    let mut child_device = LanPairingDeviceRef::new(
        device_id,
        Some(child_profile_id),
        label,
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
    device_id: impl std::fmt::Display,
    label: impl std::fmt::Display,
    ip_address: impl std::fmt::Display,
    mac_address: impl std::fmt::Display,
    hostname: impl std::fmt::Display,
    evidence_sources: Vec<LanDiscoveryEvidenceSource>,
) -> LanBrowserAddDeviceDiscoveryDevice {
    let device_id = device_id.to_string();
    let label = label.to_string();
    let ip_address = ip_address.to_string();
    let mac_address = mac_address.to_string();
    let hostname = hostname.to_string();
    let mut child_device = LanPairingDeviceRef::new(
        device_id.clone(),
        None,
        label,
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );
    child_device.ip_address = Some(ip_address);
    child_device.mac_address = Some(mac_address);
    child_device.hostname = Some(hostname);
    child_device.network_interface = Some("Ethernet".to_string());

    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: "2026-06-26T10:00:30Z".to_string(),
        child_device,
        agent_peer_id: device_id,
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
