use crate::support::OptionTestExt as _;
use ocentra_lan_core::read_model_builder::canonical_household_device_spine::values::{
    child_agent_inventory_for, classification_for_discovery, network_identity_for, route_state_for,
    NetworkIdentityInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdDeviceConfidence,
    LanCanonicalHouseholdDeviceSource, LanCanonicalHouseholdRouteState,
    LanDiscoveryEvidenceConfidence, LanDiscoveryEvidenceKind, LanDiscoveryEvidenceSource,
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

#[test]
fn service_probe_and_mdns_evidence_classify_television_devices() {
    let mut device = discovery_device("scanner-only-device");
    device.hostname = Some("family-tv.local".to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            "Family TV",
        )],
    );

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Television
    );
    assert_eq!(
        child_agent_inventory_for(
            classification == LanCanonicalHouseholdDeviceClassification::ChildAgent,
            &device,
            LanPairingTrustState::Unpaired,
            LanCanonicalHouseholdRouteState::Unavailable,
        ),
        None
    );
}

#[test]
fn basic_label_and_hostname_hints_map_to_expected_device_classes() {
    for (device_id, hostname, expected) in [
        (
            "kid-phone",
            Some("kid-iphone.local"),
            LanCanonicalHouseholdDeviceClassification::Phone,
        ),
        (
            "family-ipad-tablet",
            None,
            LanCanonicalHouseholdDeviceClassification::Tablet,
        ),
        (
            "living-room-thinkpad",
            None,
            LanCanonicalHouseholdDeviceClassification::Laptop,
        ),
        (
            "office-desktop-workstation",
            None,
            LanCanonicalHouseholdDeviceClassification::Desktop,
        ),
        (
            "console-device",
            Some("family-xbox.local"),
            LanCanonicalHouseholdDeviceClassification::GameConsole,
        ),
        (
            "driveway-camera",
            Some("arlo-camera.local"),
            LanCanonicalHouseholdDeviceClassification::Camera,
        ),
        (
            "storage-device",
            Some("synology-nas.local"),
            LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage,
        ),
        (
            "hallway-thermostat-sensor",
            None,
            LanCanonicalHouseholdDeviceClassification::InternetOfThings,
        ),
    ] {
        let mut device = discovery_device(device_id);
        device.hostname = hostname.map(str::to_string);

        assert_eq!(classification_for_discovery(&device, &[], &[]), expected);
    }
}

#[test]
fn router_platform_stays_infrastructure_and_non_child() {
    let device =
        discovery_device_with_platform("router-device", constants::lan_pairing::PLATFORM_ROUTER);

    let classification =
        classification_for_discovery(&device, &[LanDiscoveryEvidenceSource::SsdpUpnpQuery], &[]);

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    );
    assert_eq!(
        child_agent_inventory_for(
            classification == LanCanonicalHouseholdDeviceClassification::ChildAgent,
            &device,
            LanPairingTrustState::Unpaired,
            LanCanonicalHouseholdRouteState::Unavailable,
        ),
        None
    );
}

#[test]
fn signed_child_agent_evidence_keeps_device_in_child_agent_classification() {
    let device = discovery_device("signed-child-agent");

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::ChildAgentHeartbeat],
        &[],
    );

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
}

#[test]
fn child_profile_devices_stay_child_agents_and_installable() {
    let mut device = discovery_device("child-profile-device");
    device.child_profile_id = Some("child-profile-42".to_string());

    let classification = classification_for_discovery(&device, &[], &[]);
    let route_state = route_state_for(true, &LanPairingDiscoveryRuntimeStatus::WebsocketDirect);

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::ChildAgent
    );
    assert_eq!(route_state, LanCanonicalHouseholdRouteState::LocalNetwork);
    assert!(
        child_agent_inventory_for(true, &device, LanPairingTrustState::Unpaired, route_state,)
            .is_some()
    );
}

#[test]
fn weak_name_and_service_probe_evidence_stay_explainable_and_weak() {
    let mut device = discovery_device("living-room-tv");
    device.hostname = Some("living-room-tv.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());
    device.network_interface = Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string());

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[
            LanDiscoveryEvidenceSource::DnsCache,
            LanDiscoveryEvidenceSource::ServiceIdentityProbe,
        ],
        hint_sources: &[],
        service_identity_probe_evidence: &[service_probe(
            LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            "Living Room TV",
        )],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let hostname_records = identity
        .evidence_records
        .iter()
        .filter(|record| record.evidence_kind == LanDiscoveryEvidenceKind::Hostname)
        .collect::<Vec<_>>();
    assert_eq!(hostname_records.len(), 1);
    assert_eq!(
        hostname_records[0].source,
        LanDiscoveryEvidenceSource::DnsCache
    );
    assert_eq!(
        hostname_records[0].confidence,
        LanDiscoveryEvidenceConfidence::Weak
    );

    let probe_record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint)
        .value_or_unreachable();
    assert_eq!(
        probe_record.source,
        LanDiscoveryEvidenceSource::ServiceIdentityProbe
    );
    assert_eq!(
        probe_record.confidence,
        LanDiscoveryEvidenceConfidence::Weak
    );
    assert_eq!(probe_record.value, "html-title:Living Room TV");
}

#[test]
fn mdns_airplay_hint_classifies_television_and_records_strong_reason() {
    let mut device = discovery_device("airplay-speaker");
    device.hostname = Some("family-speaker.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_airplay._tcp.local",
        )],
    );
    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Television
    );

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        hint_sources: &[],
        service_identity_probe_evidence: &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_airplay._tcp.local",
        )],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint)
        .value_or_unreachable();
    assert_eq!(record.confidence, LanDiscoveryEvidenceConfidence::Strong);
    assert_eq!(record.value, "mdns-service-type:_airplay._tcp.local");
}

#[test]
fn mdns_google_cast_hint_classifies_television_and_records_strong_reason() {
    let mut device = discovery_device("google-cast-display");
    device.hostname = Some("living-room-cast.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_googlecast._tcp.local",
        )],
    );
    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Television
    );

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        hint_sources: &[],
        service_identity_probe_evidence: &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_googlecast._tcp.local",
        )],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint)
        .value_or_unreachable();
    assert_eq!(record.confidence, LanDiscoveryEvidenceConfidence::Strong);
    assert_eq!(record.value, "mdns-service-type:_googlecast._tcp.local");
}

#[test]
fn mdns_ipp_printer_hint_classifies_printer_and_records_strong_reason() {
    let mut device = discovery_device("ipp-printer");
    device.hostname = Some("office-printer.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_ipp._tcp.local",
        )],
    );
    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        hint_sources: &[],
        service_identity_probe_evidence: &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_ipp._tcp.local",
        )],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint)
        .value_or_unreachable();
    assert_eq!(record.confidence, LanDiscoveryEvidenceConfidence::Strong);
    assert_eq!(record.value, "mdns-service-type:_ipp._tcp.local");
}

#[test]
fn strong_airplay_service_hint_outweighs_weak_speaker_iot_label() {
    let mut device = discovery_device("homepod-airplay");
    device.label = "kitchen-homepod-speaker".to_string();
    device.hostname = Some("kitchen-speaker.local".to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_airplay._tcp.local",
        )],
    );

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Television
    );
    assert_eq!(
        child_agent_inventory_for(
            classification == LanCanonicalHouseholdDeviceClassification::ChildAgent,
            &device,
            LanPairingTrustState::Unpaired,
            LanCanonicalHouseholdRouteState::Unavailable,
        ),
        None
    );
}

#[test]
fn strong_printer_service_hint_outweighs_weak_tv_hostname() {
    let mut device = discovery_device("printer-tv-conflict");
    device.label = "living-room-device".to_string();
    device.hostname = Some("family-tv.local".to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
            "_ipp._tcp.local",
        )],
    );

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );
}

#[test]
fn conflicting_strong_service_hints_stay_unknown_and_non_child() {
    let mut device = discovery_device("ambiguous-strong-services");
    device.hostname = Some("family-device.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::MdnsDnsSdQuery],
        &[
            service_probe(
                LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
                "_airplay._tcp.local",
            ),
            service_probe(
                LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
                "_ipp._tcp.local",
            ),
        ],
    );

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
    );
    assert_eq!(
        child_agent_inventory_for(
            classification == LanCanonicalHouseholdDeviceClassification::ChildAgent,
            &device,
            LanPairingTrustState::Unpaired,
            LanCanonicalHouseholdRouteState::Unavailable,
        ),
        None
    );
}

#[test]
fn conflicting_weak_hints_without_strong_signal_stay_unknown() {
    let mut device = discovery_device("ambiguous-device");
    device.label = "printer-camera".to_string();
    device.hostname = Some("printer-camera.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification =
        classification_for_discovery(&device, &[LanDiscoveryEvidenceSource::DnsCache], &[]);

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::UnknownLanDevice
    );
    assert_eq!(
        child_agent_inventory_for(
            classification == LanCanonicalHouseholdDeviceClassification::ChildAgent,
            &device,
            LanPairingTrustState::Unpaired,
            LanCanonicalHouseholdRouteState::Unavailable,
        ),
        None
    );
}

#[test]
fn ssdp_media_renderer_hint_classifies_television_and_records_strong_reason() {
    let mut device = discovery_device("media-renderer");
    device.hostname = Some("media-renderer.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::SsdpUpnpQuery],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
            "urn:schemas-upnp-org:device:MediaRenderer:1",
        )],
    );
    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Television
    );

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::SsdpUpnpQuery],
        hint_sources: &[],
        service_identity_probe_evidence: &[service_probe(
            LanServiceIdentityProbeEvidenceKind::SsdpDeviceType,
            "urn:schemas-upnp-org:device:MediaRenderer:1",
        )],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint)
        .value_or_unreachable();
    assert_eq!(record.confidence, LanDiscoveryEvidenceConfidence::Strong);
    assert_eq!(
        record.value,
        "ssdp-device-type:urn:schemas-upnp-org:device:MediaRenderer:1"
    );
}

#[test]
fn jetdirect_banner_hint_classifies_printer_and_records_weak_reason() {
    let mut device = discovery_device("jetdirect-printer");
    device.hostname = Some("lab-printer.local".to_string());
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let classification = classification_for_discovery(
        &device,
        &[LanDiscoveryEvidenceSource::ServiceIdentityProbe],
        &[service_probe(
            LanServiceIdentityProbeEvidenceKind::Banner,
            "HP JetDirect",
        )],
    );
    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Printer
    );

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::ServiceIdentityProbe],
        hint_sources: &[],
        service_identity_probe_evidence: &[service_probe(
            LanServiceIdentityProbeEvidenceKind::Banner,
            "HP JetDirect",
        )],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::ServiceProbeHint)
        .value_or_unreachable();
    assert_eq!(record.confidence, LanDiscoveryEvidenceConfidence::Weak);
    assert_eq!(record.value, "banner:HP JetDirect");
}

#[test]
fn router_evidence_is_classified_and_explained_as_infrastructure() {
    let mut device =
        discovery_device_with_platform("router-evidence", constants::lan_pairing::PLATFORM_ROUTER);
    device.ip_address = Some(constants::lan_pairing::TEST_LAN_IP.to_string());

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::SsdpUpnpQuery],
        hint_sources: &[],
        service_identity_probe_evidence: &[],
        observed_at: "2026-06-26T12:00:00Z",
    });

    let router_record = identity
        .evidence_records
        .iter()
        .find(|record| record.evidence_kind == LanDiscoveryEvidenceKind::RouterClassification)
        .value_or_unreachable();
    assert_eq!(
        router_record.source,
        LanDiscoveryEvidenceSource::SsdpUpnpQuery
    );
    assert_eq!(
        router_record.confidence,
        LanDiscoveryEvidenceConfidence::Strong
    );

    let classification =
        classification_for_discovery(&device, &[LanDiscoveryEvidenceSource::SsdpUpnpQuery], &[]);
    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
    );
}

#[test]
fn locally_administered_mac_downgrades_neighbor_confidence_to_manual_required() {
    let mut device = discovery_device("locally-administered-mac");
    device.mac_address = Some("02:00:00:00:00:01".to_string());

    let identity = network_identity_for(NetworkIdentityInput {
        device: &device,
        pairing_id: None,
        reachability: LanPairingDeviceReachability::Online,
        confidence: LanCanonicalHouseholdDeviceConfidence::NetworkNeighbor,
        source: &LanCanonicalHouseholdDeviceSource::NetworkNeighbor,
        evidence_sources: &[LanDiscoveryEvidenceSource::WindowsNeighborTable],
        hint_sources: &[],
        service_identity_probe_evidence: &[],
        observed_at: "2026-06-26T12:00:00Z",
    });

    assert_eq!(
        identity.confidence,
        LanCanonicalHouseholdDeviceConfidence::ManualRequired
    );
    assert_eq!(identity.mac_address, None);
}

#[test]
fn device_without_platform_or_lan_identity_stays_unsupported() {
    let device = LanPairingDeviceRef::new(
        "unsupported-device".to_string(),
        None,
        "unsupported-device".to_string(),
        constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
    );

    let classification = classification_for_discovery(&device, &[], &[]);

    assert_eq!(
        classification,
        LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice
    );
}

fn discovery_device(device_id: impl std::fmt::Display) -> LanPairingDeviceRef {
    discovery_device_with_platform(device_id, constants::lan_pairing::PLATFORM_UNKNOWN)
}

fn discovery_device_with_platform(
    device_id: impl std::fmt::Display,
    platform: impl std::fmt::Display,
) -> LanPairingDeviceRef {
    let device_id = device_id.to_string();
    let platform = platform.to_string();
    let mut device = LanPairingDeviceRef::new(device_id.clone(), None, device_id, platform);
    device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());
    device
}

fn service_probe(
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: impl std::fmt::Display,
) -> LanServiceIdentityProbeEvidence {
    LanServiceIdentityProbeEvidence {
        evidence_kind,
        value: value.to_string(),
        selected_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
    }
}
