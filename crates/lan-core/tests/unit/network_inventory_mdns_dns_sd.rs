use crate::support::OptionTestExt as _;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind;

use ocentra_lan_core::network_inventory::mdns_dns_sd::accumulator::MdnsDnsSdDiscoveryAccumulator;
use ocentra_lan_core::network_inventory::mdns_dns_sd::merge::merge_mdns_dns_sd_discovery;
use ocentra_lan_core::network_inventory::mdns_dns_sd::packet::{
    mdns_query_names, parse_mdns_packet,
};
use ocentra_lan_core::network_inventory::mdns_dns_sd::parse_dns_name;
use ocentra_lan_core::network_inventory::mdns_dns_sd::text::sanitize_mdns_text;
use ocentra_lan_core::network_inventory::mdns_dns_sd::{
    MdnsDnsSdDiscovery, MdnsDnsSdServiceInstance, MdnsDnsSdTxtRecord, MDNS_CLASS_IN,
    MDNS_MAX_TEXT_BYTES, MDNS_SERVICE_ENUMERATION, MDNS_TYPE_A, MDNS_TYPE_AAAA, MDNS_TYPE_PTR,
    MDNS_TYPE_SRV, MDNS_TYPE_TXT,
};
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;

macro_rules! encode_name {
    ($name:expr, $packet:expr) => {{
        for label in $name.split('.') {
            $packet.push(label.len() as u8);
            $packet.extend_from_slice(label.as_bytes());
        }
        $packet.push(0);
    }};
}

macro_rules! encode_srv_data {
    ($port:expr, $target:expr) => {{
        let mut data = Vec::new();
        let port: u16 = $port;
        data.extend_from_slice(&0_u16.to_be_bytes());
        data.extend_from_slice(&0_u16.to_be_bytes());
        data.extend_from_slice(&port.to_be_bytes());
        encode_name!($target, &mut data);
        data
    }};
}

macro_rules! encode_txt_data {
    ($entries:expr) => {{
        let mut data = Vec::new();
        for entry in $entries {
            data.push(entry.len() as u8);
            data.extend_from_slice(entry.as_bytes());
        }
        data
    }};
}

fn packet_with_records(records: Vec<EncodedRecord>) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&(records.len() as u16).to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    encode_name!(MDNS_SERVICE_ENUMERATION, &mut packet);
    packet.extend_from_slice(&MDNS_TYPE_PTR.to_be_bytes());
    packet.extend_from_slice(&MDNS_CLASS_IN.to_be_bytes());
    for record in records {
        push_record(&mut packet, &record);
    }
    packet
}

#[derive(Clone)]
struct EncodedRecord {
    name: &'static str,
    record_type: u16,
    data: Vec<u8>,
}

fn push_record(packet: &mut Vec<u8>, record: &EncodedRecord) {
    encode_name!(record.name, packet);
    packet.extend_from_slice(&record.record_type.to_be_bytes());
    packet.extend_from_slice(&MDNS_CLASS_IN.to_be_bytes());
    packet.extend_from_slice(&60_u32.to_be_bytes());
    packet.extend_from_slice(&(record.data.len() as u16).to_be_bytes());
    packet.extend_from_slice(&record.data);
}

fn encode_raw_name(labels: &[&[u8]]) -> Vec<u8> {
    let mut data = Vec::new();
    for label in labels {
        data.push(label.len() as u8);
        data.extend_from_slice(label);
    }
    data.push(0);
    data
}

fn sample_discovery() -> MdnsDnsSdDiscovery {
    let packet = packet_with_records(vec![
        EncodedRecord {
            name: MDNS_SERVICE_ENUMERATION,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("_airplay._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "_airplay._tcp.local",
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Living Room TV._airplay._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "Living Room TV._airplay._tcp.local",
            record_type: MDNS_TYPE_SRV,
            data: encode_srv_data!(7000, "apple-tv.local"),
        },
        EncodedRecord {
            name: "Living Room TV._airplay._tcp.local",
            record_type: MDNS_TYPE_TXT,
            data: encode_txt_data!(&["txtvers=1", "deviceid=AB:CD", "opaque=\u{0001}spoof"]),
        },
        EncodedRecord {
            name: "apple-tv.local",
            record_type: MDNS_TYPE_A,
            data: vec![192, 168, 2, 55],
        },
        EncodedRecord {
            name: "apple-tv.local",
            record_type: MDNS_TYPE_AAAA,
            data: vec![0xfd, 0x00, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        },
        EncodedRecord {
            name: "_ocentra-agent._tcp.local",
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Agent Name\u{0007}._ocentra-agent._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "Agent Name\u{0007}._ocentra-agent._tcp.local",
            record_type: MDNS_TYPE_SRV,
            data: encode_srv_data!(4477, "agent-host.local"),
        },
        EncodedRecord {
            name: "agent-host.local",
            record_type: MDNS_TYPE_A,
            data: vec![192, 168, 2, 77],
        },
    ]);
    parse_mdns_packet(&packet)
        .map(|packet| {
            let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
            accumulator.merge(packet);
            accumulator.finalize("2026-06-26T00:00:00Z".to_string())
        })
        .value_or_unreachable()
}

#[test]
fn parser_collects_service_enumeration_srv_txt_and_addresses() {
    let discovery = sample_discovery();

    assert!(discovery
        .service_types
        .iter()
        .any(|value| value == "_airplay._tcp.local"));
    let service = discovery
        .service_instances
        .iter()
        .find(|instance| instance.service_type == "_airplay._tcp.local")
        .value_or_unreachable();
    assert_eq!(service.display_name.as_deref(), Some("Living Room TV"));
    assert_eq!(service.target_hostname.as_deref(), Some("apple-tv.local"));
    assert_eq!(service.port, Some(7000));
    assert!(service
        .addresses
        .iter()
        .any(|address| address == "192.168.2.55"));
    assert!(service.addresses.iter().any(|address| address == "fd00::1"));
    assert!(service
        .txt_records
        .iter()
        .any(|record| record.key == "txtvers"));
}

#[test]
fn merge_sses_mdns_only_as_hint_and_sanitizes_display_name() {
    let mut devices = vec![LanNetworkInventoryDevice {
        device_id: "device-1".to_string(),
        label: "LAN 192.168.2.55".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.55".to_string(),
        mac_address: "aa:bb:cc:dd:ee:ff".to_string(),
        hostname: None,
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: Vec::new(),
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }];

    let discovery = MdnsDnsSdDiscovery {
        observed_at: "2026-06-26T00:00:00Z".to_string(),
        service_types: vec!["_ocentra-agent._tcp.local".to_string()],
        service_instances: vec![MdnsDnsSdServiceInstance {
            service_type: "_ocentra-agent._tcp.local".to_string(),
            instance_name: "Agent Name\u{0007}._ocentra-agent._tcp.local".to_string(),
            display_name: Some("Agent Name\u{0007}".to_string()),
            target_hostname: Some("agent-host.local".to_string()),
            port: Some(4477),
            addresses: vec!["192.168.2.55".to_string()],
            txt_records: vec![MdnsDnsSdTxtRecord {
                key: "confirmation-state".to_string(),
                value: Some("hint-only".to_string()),
            }],
            parent_advertisement: None,
            child_advertisement: None,
        }],
    };

    merge_mdns_dns_sd_discovery(&mut devices, &discovery);

    let device = &devices[0];
    assert_eq!(device.label, "Agent Name");
    assert_eq!(device.hostname.as_deref(), Some("agent-host.local"));
    assert_eq!(device.agent_status.as_deref(), None);
    assert!(device
        .scan_sources
        .iter()
        .any(|value| value == constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD));
}

#[test]
fn merge_keeps_existing_non_generic_labels_and_does_not_confirm_agents() {
    let mut devices = vec![LanNetworkInventoryDevice {
        device_id: "device-2".to_string(),
        label: "Kitchen Camera".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.77".to_string(),
        mac_address: "11:22:33:44:55:66".to_string(),
        hostname: Some("existing-host.local".to_string()),
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: Vec::new(),
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }];

    let discovery = MdnsDnsSdDiscovery {
        observed_at: "2026-06-26T00:00:00Z".to_string(),
        service_types: vec![constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE.to_string()],
        service_instances: vec![MdnsDnsSdServiceInstance {
            service_type: constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE.to_string(),
            instance_name: "Ocentra Child._ocentra-agent._tcp.local".to_string(),
            display_name: Some("Ocentra Child".to_string()),
            target_hostname: Some("child.local".to_string()),
            port: Some(4477),
            addresses: vec!["192.168.2.77".to_string()],
            txt_records: vec![],
            parent_advertisement: None,
            child_advertisement: None,
        }],
    };

    merge_mdns_dns_sd_discovery(&mut devices, &discovery);

    let device = &devices[0];
    assert_eq!(device.label, "Kitchen Camera");
    assert_eq!(device.hostname.as_deref(), Some("existing-host.local"));
    assert_eq!(device.agent_status.as_deref(), None);
}

#[test]
fn merge_adds_mdns_only_devices_as_agentless_hints() {
    let mut devices = Vec::new();
    let discovery = MdnsDnsSdDiscovery {
        observed_at: "2026-06-26T00:00:00Z".to_string(),
        service_types: vec!["_ipp._tcp.local".to_string()],
        service_instances: vec![MdnsDnsSdServiceInstance {
            service_type: "_ipp._tcp.local".to_string(),
            instance_name: "Office Printer._ipp._tcp.local".to_string(),
            display_name: Some("Office Printer".to_string()),
            target_hostname: Some("office-printer.local".to_string()),
            port: Some(631),
            addresses: vec!["192.168.2.88".to_string()],
            txt_records: Vec::new(),
            parent_advertisement: None,
            child_advertisement: None,
        }],
    };

    merge_mdns_dns_sd_discovery(&mut devices, &discovery);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert!(device.device_id.starts_with(&format!(
        "{}mdns-",
        constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX
    )));
    assert_eq!(device.label, "Office Printer");
    assert_eq!(device.hostname.as_deref(), Some("office-printer.local"));
    assert_eq!(device.ip_address, "192.168.2.88");
    assert!(device.mac_address.is_empty());
    assert_eq!(device.platform, constants::lan_pairing::PLATFORM_UNKNOWN);
    assert_eq!(device.agent_status, None);
    assert_eq!(device.reachability, LanPairingDeviceReachability::Online);
    assert_eq!(
        device.scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string()]
    );
}

#[test]
fn merge_normalizes_eqsivalent_ipv6_addresses_before_matching_devices() {
    let mut devices = vec![LanNetworkInventoryDevice {
        device_id: "device-3".to_string(),
        label: "LAN fd00::1".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "FD00::1".to_string(),
        mac_address: "22:33:44:55:66:77".to_string(),
        hostname: None,
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: Vec::new(),
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }];

    let discovery = MdnsDnsSdDiscovery {
        observed_at: "2026-06-26T00:00:00Z".to_string(),
        service_types: vec!["_airplay._tcp.local".to_string()],
        service_instances: vec![MdnsDnsSdServiceInstance {
            service_type: "_airplay._tcp.local".to_string(),
            instance_name: "Conference TV._airplay._tcp.local".to_string(),
            display_name: Some("Conference TV".to_string()),
            target_hostname: Some("conference-tv.local".to_string()),
            port: Some(7000),
            addresses: vec!["fd00:0:0:0:0:0:0:1".to_string()],
            txt_records: vec![],
            parent_advertisement: None,
            child_advertisement: None,
        }],
    };

    merge_mdns_dns_sd_discovery(&mut devices, &discovery);

    let device = &devices[0];
    assert_eq!(device.label, "Conference TV");
    assert_eq!(device.hostname.as_deref(), Some("conference-tv.local"));
    assert!(device
        .scan_sources
        .iter()
        .any(|value| value == constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD));
}

#[test]
fn merge_sses_sniqse_hostname_fallback_withost_creating_a_duplicate_device() {
    let mut devices = vec![LanNetworkInventoryDevice {
        device_id: "device-4".to_string(),
        label: "LAN 192.168.2.55".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.2.55".to_string(),
        mac_address: "44:55:66:77:88:99".to_string(),
        hostname: Some("agent-host.local".to_string()),
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: Vec::new(),
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }];

    let discovery = MdnsDnsSdDiscovery {
        observed_at: "2026-06-26T00:00:00Z".to_string(),
        service_types: vec!["_ocentra-agent._tcp.local".to_string()],
        service_instances: vec![MdnsDnsSdServiceInstance {
            service_type: "_ocentra-agent._tcp.local".to_string(),
            instance_name: "Agent Name._ocentra-agent._tcp.local".to_string(),
            display_name: Some("Agent Name".to_string()),
            target_hostname: Some("agent-host.local".to_string()),
            port: Some(4477),
            addresses: vec!["192.168.2.99".to_string()],
            txt_records: vec![MdnsDnsSdTxtRecord {
                key: "confirmation-state".to_string(),
                value: Some("hint-only".to_string()),
            }],
            parent_advertisement: None,
            child_advertisement: None,
        }],
    };

    merge_mdns_dns_sd_discovery(&mut devices, &discovery);

    assert_eq!(devices.len(), 1);
    let device = &devices[0];
    assert_eq!(device.ip_address, "192.168.2.55");
    assert_eq!(device.label, "Agent Name");
    assert_eq!(device.hostname.as_deref(), Some("agent-host.local"));
    assert!(device
        .scan_sources
        .iter()
        .any(|value| value == constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD));
    assert!(device.service_identity_probe_evidence.iter().any(|record| {
        record.evidence_kind == LanServiceIdentityProbeEvidenceKind::MdnsInstanceName
            && record.value == "Agent Name._ocentra-agent._tcp.local"
    }));
}

#[test]
fn merge_does_not_sse_hostname_fallback_when_hostname_is_ambigsoss() {
    let mut devices = vec![
        LanNetworkInventoryDevice {
            device_id: "device-5".to_string(),
            label: "LAN 192.168.2.10".to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "192.168.2.10".to_string(),
            mac_address: "00:11:22:33:44:55".to_string(),
            hostname: Some("shared-host.local".to_string()),
            network_interface: None,
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: Vec::new(),
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        },
        LanNetworkInventoryDevice {
            device_id: "device-6".to_string(),
            label: "LAN 192.168.2.11".to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "192.168.2.11".to_string(),
            mac_address: "66:77:88:99:aa:bb".to_string(),
            hostname: Some("shared-host.local".to_string()),
            network_interface: None,
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: Vec::new(),
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        },
    ];

    let discovery = MdnsDnsSdDiscovery {
        observed_at: "2026-06-26T00:00:00Z".to_string(),
        service_types: vec!["_ocentra-agent._tcp.local".to_string()],
        service_instances: vec![MdnsDnsSdServiceInstance {
            service_type: "_ocentra-agent._tcp.local".to_string(),
            instance_name: "Shared Host._ocentra-agent._tcp.local".to_string(),
            display_name: Some("Shared Host".to_string()),
            target_hostname: Some("shared-host.local".to_string()),
            port: Some(4477),
            addresses: vec!["192.168.2.99".to_string()],
            txt_records: vec![],
            parent_advertisement: None,
            child_advertisement: None,
        }],
    };

    merge_mdns_dns_sd_discovery(&mut devices, &discovery);

    assert_eq!(devices.len(), 3);
    assert_eq!(devices[0].label, "LAN 192.168.2.10");
    assert_eq!(devices[1].label, "LAN 192.168.2.11");
    let created = devices
        .iter()
        .find(|device| device.ip_address == "192.168.2.99")
        .value_or_unreachable();
    assert_eq!(created.label, "Shared Host");
    assert!(created
        .scan_sources
        .iter()
        .any(|value| value == constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD));
}

#[test]
fn parser_collects_selected_service_types_and_ignores_snselected() {
    let packet = packet_with_records(vec![
        EncodedRecord {
            name: MDNS_SERVICE_ENUMERATION,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("_airplay._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: MDNS_SERVICE_ENUMERATION,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("_Unsupported.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "_airplay._tcp.local",
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Office Speaker._airplay._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "_Unsupported.local",
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Not Selected._Unsupported.local", &mut data);
                data
            },
        },
    ]);
    let discovery = parse_mdns_packet(&packet)
        .map(|packet| {
            let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
            accumulator.merge(packet);
            accumulator.finalize("2026-06-26T00:00:00Z".to_string())
        })
        .value_or_unreachable();

    assert!(!discovery
        .service_types
        .iter()
        .any(|value| value == "_Unsupported.local"));
    assert!(!discovery
        .service_instances
        .iter()
        .any(|value| value.service_type == "_Unsupported.local"));
    assert!(discovery
        .service_instances
        .iter()
        .any(|value| value.service_type == "_airplay._tcp.local"));
}

#[test]
fn mdns_query_names_cover_service_enumeration_and_selected_service_types() {
    let query_names = mdns_query_names();

    assert_eq!(query_names.first().copied(), Some(MDNS_SERVICE_ENUMERATION));
    assert!(query_names
        .iter()
        .any(|value| value == &constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE));
    assert!(query_names
        .iter()
        .any(|value| value == &constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE));
    assert!(query_names
        .iter()
        .any(|value| value == &"_workstation._tcp.local"));
    assert!(query_names.iter().any(|value| value == &"_ipp._tcp.local"));
    assert!(query_names
        .iter()
        .any(|value| value == &"_printer._tcp.local"));
    assert!(query_names
        .iter()
        .any(|value| value == &"_airplay._tcp.local"));
    assert!(query_names.iter().any(|value| value == &"_raop._tcp.local"));
    assert!(query_names
        .iter()
        .any(|value| value == &"_googlecast._tcp.local"));
    assert!(query_names
        .iter()
        .any(|value| value == &"_companion-link._tcp.local"));
}

#[test]
fn parser_covers_selected_mdns_device_service_families() {
    let services = [
        (
            "_workstation._tcp.local",
            "Kid Laptop._workstation._tcp.local",
            "kid-laptop.local",
            [192, 168, 2, 40],
        ),
        (
            "_ipp._tcp.local",
            "Office Printer._ipp._tcp.local",
            "office-printer.local",
            [192, 168, 2, 41],
        ),
        (
            "_printer._tcp.local",
            "Legacy Printer._printer._tcp.local",
            "legacy-printer.local",
            [192, 168, 2, 42],
        ),
        (
            "_googlecast._tcp.local",
            "Living Chromecast._googlecast._tcp.local",
            "living-chromecast.local",
            [192, 168, 2, 43],
        ),
        (
            "_companion-link._tcp.local",
            "Companion Phone._companion-link._tcp.local",
            "companion-phone.local",
            [192, 168, 2, 44],
        ),
        (
            "_raop._tcp.local",
            "Living Room Asdio._raop._tcp.local",
            "living-room-asdio.local",
            [192, 168, 2, 45],
        ),
    ];
    let mut records = Vec::new();
    for (service_type, instance_name, target, address) in services {
        records.push(EncodedRecord {
            name: MDNS_SERVICE_ENUMERATION,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!(service_type, &mut data);
                data
            },
        });
        records.push(EncodedRecord {
            name: service_type,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!(instance_name, &mut data);
                data
            },
        });
        records.push(EncodedRecord {
            name: instance_name,
            record_type: MDNS_TYPE_SRV,
            data: encode_srv_data!(9, target),
        });
        records.push(EncodedRecord {
            name: target,
            record_type: MDNS_TYPE_A,
            data: address.to_vec(),
        });
    }
    let packet = packet_with_records(records);
    let discovery = parse_mdns_packet(&packet)
        .map(|packet| {
            let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
            accumulator.merge(packet);
            accumulator.finalize("2026-06-26T00:00:00Z".to_string())
        })
        .value_or_unreachable();

    for (service_type, instance_name, target, address) in services {
        let service = discovery
            .service_instances
            .iter()
            .find(|instance| instance.service_type == service_type)
            .value_or_unreachable();
        assert_eq!(service.instance_name, instance_name);
        assert_eq!(service.target_hostname.as_deref(), Some(target));
        assert!(service.addresses.iter().any(|candidate| {
            candidate
                == &format!(
                    "{}.{}.{}.{}",
                    address[0], address[1], address[2], address[3]
                )
        }));
    }
}

#[test]
fn sanitize_mdns_text_strips_control_characters_and_rejects_empty_values() {
    assert_eq!(
        sanitize_mdns_text(" Agent\u{0007} Name "),
        Some("Agent Name".to_string())
    );
    assert_eq!(sanitize_mdns_text("\u{0000}\u{0007}"), None);
    assert_eq!(
        sanitize_mdns_text(" <script>alert(1)</script> "),
        Some("script alert(1) /script".to_string())
    );
    assert_eq!(
        sanitize_mdns_text(&format!("{}{}", "A".repeat(MDNS_MAX_TEXT_BYTES), "tail")),
        Some("A".repeat(MDNS_MAX_TEXT_BYTES))
    );
}

fn shared_contract_advertisement_packet() -> Vec<u8> {
    packet_with_records(vec![
        EncodedRecord {
            name: MDNS_SERVICE_ENUMERATION,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!(constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE, &mut data);
                data
            },
        },
        EncodedRecord {
            name: MDNS_SERVICE_ENUMERATION,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!(constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE, &mut data);
                data
            },
        },
        EncodedRecord {
            name: constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Parent Desk._ocentra-parent._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "Parent Desk._ocentra-parent._tcp.local",
            record_type: MDNS_TYPE_TXT,
            data: encode_txt_data!(&[
                "lan.mdns_advertisement_id=sha256:parent-id",
                "protocol-version=2.0.0",
                "family-hash=sha256:family-parent",
                "pairing-state=paired",
                "lifecycle-state=start",
                "support-state=supported",
            ]),
        },
        EncodedRecord {
            name: constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE,
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Child Tablet._ocentra-agent._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "Child Tablet._ocentra-agent._tcp.local",
            record_type: MDNS_TYPE_TXT,
            data: encode_txt_data!(&[
                "lan.mdns_advertisement_id=sha256:child-id",
                "opaque-device-id=opaque-child-id",
                "protocol-version=2.0.0",
                "family-hash=sha256:family-parent",
                "platform=windows",
                "agent-version=1.2.3",
                "pairing-state=unpaired",
                "lifecycle-state=update",
                "support-state=degraded",
            ]),
        },
    ])
}

#[test]
fn parser_bsilds_shared_contract_advertisement_dto_for_parent_and_child_types() {
    let packet = shared_contract_advertisement_packet();
    let discovery = parse_mdns_packet(&packet)
        .map(|packet| {
            let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
            accumulator.merge(packet);
            accumulator.finalize("2026-06-26T00:00:00Z".to_string())
        })
        .value_or_unreachable();

    let parent_instance = discovery
        .service_instances
        .iter()
        .find(|instance| instance.service_type == constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE)
        .value_or_unreachable();
    let child_instance = discovery
        .service_instances
        .iter()
        .find(|instance| instance.service_type == constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE)
        .value_or_unreachable();

    let parent_advertisement = parent_instance
        .parent_advertisement
        .as_ref()
        .value_or_unreachable();
    let child_advertisement = child_instance
        .child_advertisement
        .as_ref()
        .value_or_unreachable();

    assert_eq!(parent_advertisement.advertisement_id, "sha256:parent-id");
    assert_eq!(child_advertisement.opaque_device_id, "opaque-child-id");
    assert_eq!(
        parent_advertisement.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY.into()
    );
    assert_eq!(
        child_advertisement.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY.into()
    );
    assert_eq!(
        parent_advertisement.service_type,
        constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
    );
    assert_eq!(
        child_advertisement.service_type,
        constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
    );
    assert_eq!(
        parent_instance
            .parent_advertisement
            .as_ref()
            .map(|advertisement| advertisement.advertisement_id.as_str()),
        Some("sha256:parent-id")
    );
    assert!(parent_instance.child_advertisement.is_none());
    assert_eq!(
        child_instance
            .child_advertisement
            .as_ref()
            .map(|advertisement| advertisement.opaque_device_id.as_str()),
        Some("opaque-child-id")
    );
    assert!(child_instance.parent_advertisement.is_none());
    assert_eq!(
        parent_advertisement.pairing_state.as_str(),
        constants::value::LAN_PAIRING_PAIRED.into()
    );
    assert_eq!(
        child_advertisement.lifecycle_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_UPDATE.into()
    );
}

#[test]
fn parser_ignores_malformed_dns_names_and_parses_following_records() {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&3_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&[0x40]);
    packet.extend_from_slice(&MDNS_TYPE_PTR.to_be_bytes());
    packet.extend_from_slice(&MDNS_CLASS_IN.to_be_bytes());
    for record in [
        EncodedRecord {
            name: "_airplay._tcp.local",
            record_type: MDNS_TYPE_PTR,
            data: {
                let mut data = Vec::new();
                encode_name!("Kitchen Speaker._airplay._tcp.local", &mut data);
                data
            },
        },
        EncodedRecord {
            name: "Kitchen Speaker._airplay._tcp.local",
            record_type: MDNS_TYPE_SRV,
            data: encode_srv_data!(7000, "kitchen.local"),
        },
        EncodedRecord {
            name: "kitchen.local",
            record_type: MDNS_TYPE_A,
            data: vec![192, 168, 2, 90],
        },
    ] {
        push_record(&mut packet, &record);
    }

    let discovery = parse_mdns_packet(&packet)
        .map(|packet| {
            let mut accumulator = MdnsDnsSdDiscoveryAccumulator::default();
            accumulator.merge(packet);
            accumulator.finalize("2026-06-26T00:00:00Z".to_string())
        })
        .value_or_unreachable();

    assert_eq!(discovery.service_instances.len(), 1);
    assert_eq!(
        discovery.service_instances[0].service_type,
        "_airplay._tcp.local"
    );
    assert_eq!(discovery.service_instances[0].addresses[0], "192.168.2.90");

    let malformed_name_payload = vec![0xFF, 0x00];
    assert!(parse_dns_name(&malformed_name_payload, 0).is_none());

    let oversized_label_payload = vec![64, b'a', b'b', b'c', b'd', 0];
    assert!(parse_dns_name(&oversized_label_payload, 0).is_none());

    let invalid_utf8_payload =
        encode_raw_name(&[&[0xF0, 0x80, 0x80], b"_airplay", b"_tcp", b"local"]);
    let expected_name = [
        String::from_utf8_lossy(&[0xF0, 0x80, 0x80]).to_string(),
        "_airplay".to_string(),
        "_tcp".to_string(),
        "local".to_string(),
    ]
    .join(".");
    assert_eq!(
        parse_dns_name(&invalid_utf8_payload, 0),
        Some((expected_name, invalid_utf8_payload.len()))
    );
}
