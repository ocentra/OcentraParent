use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::env;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, UdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};
use rcgen::generate_simple_self_signed;

use crate::network_inventory::trusted_device;
use ocentra_lan_core::network_inventory::service_identity::probe::{
    parse_probe_observation, probe_service_identity, probe_service_identity_on_target,
    read_probe_response,
};
use ocentra_lan_core::network_inventory::service_identity::snmp::{
    encode_ber_integer, encode_ber_oid, encode_ber_tlv, encode_snmp_identity_request,
    parse_snmp_probe_observation, probe_snmp_identity_query_at_endpoint,
};
use ocentra_lan_core::network_inventory::service_identity::targets::{
    service_identity_probe_family_decisions, service_identity_probe_targets,
};
use ocentra_lan_core::network_inventory::service_identity::wsd::{
    parse_wsd_probe_observation, probe_wsd_identity_query_at_endpoint, sanitize_wsd_device_id,
};
use ocentra_lan_core::network_inventory::service_identity::*;
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;

fn agent_addr_env_lock() -> std::sync::MutexGuard<'static, ()> {
    static AGENT_ADDR_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    AGENT_ADDR_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .value_or_unreachable()
}

fn service_identity_env_lock() -> std::sync::MutexGuard<'static, ()> {
    static SERVICE_IDENTITY_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    SERVICE_IDENTITY_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .value_or_unreachable()
}

#[test]
fn explicit_service_identity_scope_overrides_runtime_identity() {
    assert_eq!(
        ocentra_lan_core::network_inventory::api::service_identity_selected_interface_scope(Some(
            "  Explicit LAN  "
        ))
        .as_deref(),
        Some("Explicit LAN")
    );
}

#[test]
fn service_identity_probe_marks_weak_status_without_upgrading_identity() {
    let mut device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: format!(
            "{}{}",
            constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
            constants::lan_pairing::TEST_LAN_IP
        ),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: None,
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    apply_service_identity_probe(
        &mut device,
        LanServiceIdentityProbeObservation {
            status_code: Some(200),
            title: Some("Printer Admin".to_string()),
            server_header: Some("edge-proxy".to_string()),
            banner: Some("control-plane".to_string()),
            redirect_location: Some("/ui".to_string()),
            certificate_subject: Some("CN=printer-admin".to_string()),
            descriptor_links: vec!["</metadata>; rel=\"service-desc\"".to_string()],
            wsd_endpoint_address: None,
            wsd_types: None,
            snmp_sys_descr: None,
            snmp_sys_name: None,
        },
    );

    assert_eq!(
        device.agent_status.as_deref(),
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    );
    assert!(device.hostname.is_none());
    assert_eq!(
        device.label,
        format!(
            "{}{}",
            constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
            constants::lan_pairing::TEST_LAN_IP
        )
    );
    assert_eq!(device.platform, constants::lan_pairing::PLATFORM_UNKNOWN);
    assert_eq!(device.service_identity_probe_evidence.len(), 7);
    assert!(device
        .service_identity_probe_evidence
        .iter()
        .any(|evidence| {
            evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::HttpStatus
                && evidence.value == "200"
        }));
    assert!(device
        .service_identity_probe_evidence
        .iter()
        .any(|evidence| {
            evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::HtmlTitle
                && evidence.value == "Printer Admin"
        }));
    assert!(device
        .service_identity_probe_evidence
        .iter()
        .any(|evidence| {
            evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::CertificateSubject
                && evidence.value == "CN=printer-admin"
        }));
}

#[test]
fn service_identity_probe_merges_existing_weak_evidence() {
    let mut device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: format!(
            "{}{}",
            constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
            constants::lan_pairing::TEST_LAN_IP
        ),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: None,
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: vec![
            LanServiceIdentityProbeEvidence {
                evidence_kind: LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
                value: "_http._tcp.local".to_string(),
                selected_interface: Some(
                    constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string(),
                ),
            },
            LanServiceIdentityProbeEvidence {
                evidence_kind: LanServiceIdentityProbeEvidenceKind::ServerHeader,
                value: "edge-proxy".to_string(),
                selected_interface: Some(
                    constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string(),
                ),
            },
        ],
    };

    apply_service_identity_probe(
        &mut device,
        LanServiceIdentityProbeObservation {
            status_code: Some(200),
            title: Some("Printer Admin".to_string()),
            server_header: Some("edge-proxy".to_string()),
            banner: Some("control-plane".to_string()),
            redirect_location: Some("/ui".to_string()),
            certificate_subject: Some("CN=printer-admin".to_string()),
            descriptor_links: vec!["</metadata>; rel=\"service-desc\"".to_string()],
            wsd_endpoint_address: None,
            wsd_types: None,
            snmp_sys_descr: None,
            snmp_sys_name: None,
        },
    );

    assert_eq!(
        device.agent_status.as_deref(),
        Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    );
    assert!(device
        .service_identity_probe_evidence
        .iter()
        .any(|evidence| {
            evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::MdnsServiceType
                && evidence.value == "_http._tcp.local"
        }));
    assert_eq!(
        device
            .service_identity_probe_evidence
            .iter()
            .filter(|evidence| {
                evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::ServerHeader
                    && evidence.value == "edge-proxy"
            })
            .count(),
        1
    );
    assert_eq!(device.service_identity_probe_evidence.len(), 8);
}

#[test]
fn service_identity_probe_records_allowed_snmp_response_scan_source() {
    let mut device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: format!(
            "{}{}",
            constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
            constants::lan_pairing::TEST_LAN_IP
        ),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: None,
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    apply_service_identity_probe(
        &mut device,
        LanServiceIdentityProbeObservation {
            status_code: None,
            title: None,
            server_header: None,
            banner: None,
            redirect_location: None,
            certificate_subject: None,
            descriptor_links: Vec::new(),
            wsd_endpoint_address: None,
            wsd_types: None,
            snmp_sys_descr: Some("Printer".to_string()),
            snmp_sys_name: Some("printer-01".to_string()),
        },
    );

    assert!(device
        .scan_sources
        .contains(&constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()));
    assert!(device
        .scan_sources
        .contains(&constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string()));
    assert_eq!(
        device
            .scan_sources
            .iter()
            .filter(|source| {
                source.as_str() == constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE
            })
            .count(),
        1
    );
}

#[test]
fn service_identity_probe_requires_explicit_selected_interface_scope() {
    let _env_lock = agent_addr_env_lock();
    let listener = TcpListener::bind("127.0.0.1:0").value_or_unreachable();
    let port = listener.local_addr().value_or_unreachable().port();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().value_or_unreachable();
        let _ = read_request(&mut stream);
        let body = "<html><head><title>Scoped</title></head><body>ok</body></html>";
        let response = format!(
            "HTTP/1.1 200 OK\r\nServer: scoped\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).value_or_unreachable();
    });

    let previous_agent_addr = env::var(constants::env_var::AGENT_ADDR).ok();
    env::set_var(constants::env_var::AGENT_ADDR, format!("127.0.0.1:{port}"));

    let mut scoped_devices = vec![
        LanNetworkInventoryDevice {
            device_id: "lan-device-selected".to_string(),
            label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "127.0.0.1".to_string(),
            mac_address: "54-27-1e-97-c3-31".to_string(),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        },
        LanNetworkInventoryDevice {
            device_id: "lan-device-other-interface".to_string(),
            label: "other".to_string(),
            platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
            ip_address: "127.0.0.1".to_string(),
            mac_address: "54-27-1e-97-c3-32".to_string(),
            hostname: Some("other".to_string()),
            network_interface: Some("Ethernet 7".to_string()),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            observed_at: String::new(),
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        },
    ];

    enrich_service_identity_probes(
        &mut scoped_devices,
        &[],
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        None,
    );

    if let Some(previous_agent_addr) = previous_agent_addr {
        env::set_var(constants::env_var::AGENT_ADDR, previous_agent_addr);
    } else {
        env::remove_var(constants::env_var::AGENT_ADDR);
    }

    server.join().value_or_unreachable();

    assert!(is_service_identity_probe_status(
        scoped_devices[0].agent_status.as_deref()
    ));
    assert!(scoped_devices[0]
        .service_identity_probe_evidence
        .iter()
        .any(|evidence| evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::HtmlTitle));
    assert!(scoped_devices[1].agent_status.is_none());
    assert!(scoped_devices[1].service_identity_probe_evidence.is_empty());
}

#[test]
fn service_identity_probe_skips_when_selected_interface_scope_is_missing() {
    let _env_lock = service_identity_env_lock();
    let mut devices = vec![LanNetworkInventoryDevice {
        device_id: "lan-device-selected".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "127.0.0.1".to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }];

    enrich_service_identity_probes(&mut devices, &[], None, None);

    assert!(devices[0].agent_status.is_none());
    assert!(devices[0].service_identity_probe_evidence.is_empty());
    assert_eq!(
        devices[0].scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()]
    );
}

#[test]
fn wsd_identity_query_adds_weak_metadata_evidence_for_discovered_host() {
    let response = wsd_identity_query_response();

    let observation = parse_wsd_probe_observation(response.as_bytes()).value_or_unreachable();

    let evidence = observation.into_evidence();
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::HttpStatus && item.value == "200"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
            && item.value == "urn:uuid:camera-1"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::WsdTypes
            && item.value == "dn:NetworkVideoTransmitter"
    }));
}

#[test]
fn wsd_identity_query_executes_against_local_metadata_endpoint() {
    let listener = TcpListener::bind("127.0.0.1:0").value_or_unreachable();
    let endpoint = listener.local_addr().value_or_unreachable();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().value_or_unreachable();
        let request = read_request(&mut stream);
        assert!(request.starts_with("POST /camera-1 HTTP/1.1\r\n"));
        assert!(request.lines().any(|line| {
            line
                == "Content-Type: application/soap+xml; charset=utf-8; action=\"http://schemas.xmlsoap.org/ws/2004/09/transfer/Get\""
        }));
        stream
            .write_all(wsd_identity_query_response().as_bytes())
            .value_or_unreachable();
    });

    let observation =
        probe_wsd_identity_query_at_endpoint(endpoint, Some("camera-1")).value_or_unreachable();

    server.join().value_or_unreachable();

    let evidence = observation.into_evidence();
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::WsdEndpointAddress
            && item.value == "urn:uuid:camera-1"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::WsdTypes
            && item.value == "dn:NetworkVideoTransmitter"
    }));
}

#[test]
fn wsd_identity_query_rejects_unsafe_or_oversized_device_ids() {
    assert_eq!(sanitize_wsd_device_id(None), None);
    assert_eq!(sanitize_wsd_device_id(Some("   ")), None);
    assert_eq!(sanitize_wsd_device_id(Some("../camera-1")), None);
    assert_eq!(sanitize_wsd_device_id(Some("camera 1")), None);
    assert_eq!(
        sanitize_wsd_device_id(Some(&"a".repeat(SERVICE_IDENTITY_PROBE_MAX_TEXT_BYTES + 1))),
        None
    );
    assert_eq!(
        sanitize_wsd_device_id(Some("  camera-1  ")).as_deref(),
        Some("camera-1")
    );
}

fn wsd_identity_query_response() -> String {
    let body = concat!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>",
        "<s:Envelope xmlns:s=\"http://www.w3.org/2003/05/soap-envelope\" ",
        "xmlns:a=\"http://schemas.xmlsoap.org/ws/2004/08/addressing\" ",
        "xmlns:dpws=\"http://docs.oasis-open.org/ws-dd/ns/dpws/2009/01\">",
        "<s:Body>",
        "<a:EndpointReference>",
        "<a:Address>urn:uuid:camera-1</a:Address>",
        "</a:EndpointReference>",
        "<dpws:Relationship>",
        "<dpws:Host>",
        "<dpws:Types>dn:NetworkVideoTransmitter</dpws:Types>",
        "</dpws:Host>",
        "</dpws:Relationship>",
        "</s:Body>",
        "</s:Envelope>"
    );
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/soap+xml; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
        body.len(),
        body
    )
}

#[test]
fn snmp_identity_query_adds_weak_metadata_evidence_for_discovered_host() {
    let response = snmp_identity_query_response();

    let observation =
        parse_snmp_probe_observation(&response, SNMP_REQUEST_ID).value_or_unreachable();

    let evidence = observation.into_evidence();
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
            && item.value == "Linux camera controller"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysName
            && item.value == "cam-1"
    }));
    assert!(!evidence
        .iter()
        .any(|item| { item.evidence_kind == LanServiceIdentityProbeEvidenceKind::HttpStatus }));
}

#[test]
fn snmp_identity_query_executes_against_local_udp_endpoint() {
    let socket = UdpSocket::bind("127.0.0.1:0").value_or_unreachable();
    let endpoint = socket.local_addr().value_or_unreachable();
    let server = thread::spawn(move || {
        let mut request = [0_u8; 1024];
        let (read, source) = socket.recv_from(&mut request).value_or_unreachable();
        assert_eq!(
            request[..read].to_vec(),
            encode_snmp_identity_request(SNMP_REQUEST_ID)
        );
        socket
            .send_to(&snmp_identity_query_response(), source)
            .value_or_unreachable();
    });

    let observation = probe_snmp_identity_query_at_endpoint(endpoint, None).value_or_unreachable();

    server.join().value_or_unreachable();

    let evidence = observation.into_evidence();
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
            && item.value == "Linux camera controller"
    }));
    assert!(evidence.iter().any(|item| {
        item.evidence_kind == LanServiceIdentityProbeEvidenceKind::SnmpSysName
            && item.value == "cam-1"
    }));
}

#[test]
fn snmp_identity_query_notifies_allowed_snmp_observer_with_received_payload() {
    let socket = UdpSocket::bind("127.0.0.1:0").value_or_unreachable();
    let endpoint = socket.local_addr().value_or_unreachable();
    let expected_response = snmp_identity_query_response();
    let server_response = expected_response.clone();
    let server = thread::spawn(move || {
        let mut request = [0_u8; 1024];
        let (_, source) = socket.recv_from(&mut request).value_or_unreachable();
        socket
            .send_to(&server_response, source)
            .value_or_unreachable();
    });
    let observed_payload = std::sync::Mutex::new(Vec::new());

    let observation = probe_snmp_identity_query_at_endpoint(
        endpoint,
        Some(&|payload| {
            observed_payload
                .lock()
                .unwrap_or_else(|_| unreachable!("observer lock available"))
                .push(payload.to_vec());
        }),
    )
    .value_or_unreachable();

    server.join().value_or_unreachable();

    assert!(observation.observed_allowed_snmp_response());
    let observed_payload = observed_payload
        .lock()
        .unwrap_or_else(|_| unreachable!("observer lock available"));
    assert_eq!(observed_payload.as_slice(), &[expected_response]);
}

fn snmp_identity_query_response() -> Vec<u8> {
    let varbind_list = encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[
            encode_ber_tlv(
                BER_TAG_SEQUENCE,
                &[
                    encode_ber_tlv(
                        BER_TAG_OBJECT_IDENTIFIER,
                        &encode_ber_oid(SNMP_SYS_DESCR_OID),
                    ),
                    encode_ber_tlv(BER_TAG_OCTET_STRING, b"Linux camera controller"),
                ]
                .concat(),
            ),
            encode_ber_tlv(
                BER_TAG_SEQUENCE,
                &[
                    encode_ber_tlv(
                        BER_TAG_OBJECT_IDENTIFIER,
                        &encode_ber_oid(SNMP_SYS_NAME_OID),
                    ),
                    encode_ber_tlv(BER_TAG_OCTET_STRING, b"cam-1"),
                ]
                .concat(),
            ),
        ]
        .concat(),
    );
    encode_ber_tlv(
        BER_TAG_SEQUENCE,
        &[
            encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(SNMP_VERSION_2C)),
            encode_ber_tlv(BER_TAG_OCTET_STRING, SNMP_PUBLIC_COMMUNITY.as_bytes()),
            encode_ber_tlv(
                SNMP_GET_RESPONSE_TAG,
                &[
                    encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(SNMP_REQUEST_ID)),
                    encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(0)),
                    encode_ber_tlv(BER_TAG_INTEGER, &encode_ber_integer(0)),
                    varbind_list,
                ]
                .concat(),
            ),
        ]
        .concat(),
    )
}

#[test]
fn trusted_device_suppresses_service_identity_probe() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let mut trusted_device = LanPairingDeviceRef::new(
        "trusted-child".to_string(),
        None,
        constants::lan_pairing::TEST_HOSTNAME.to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    );
    trusted_device.mac_address = Some(constants::lan_pairing::TEST_LAN_MAC.to_string());

    assert!(!should_probe_service_identity(
        &device,
        &[trusted_device],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn trusted_device_without_mac_can_still_match_by_ip() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let trusted_device = trusted_device(
        "",
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_HOSTNAME),
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_WINDOWS,
    );

    assert!(!should_probe_service_identity(
        &device,
        &[trusted_device],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn trusted_device_with_mac_can_match_by_ip_when_protocol_source_has_no_mac() {
    let trusted_device = trusted_device(
        constants::lan_pairing::TEST_LAN_MAC,
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_HOSTNAME),
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_WINDOWS,
    );

    assert!(trusted_device_matches_network_identity(
        &trusted_device,
        "",
        constants::lan_pairing::TEST_LAN_IP,
    ));
}

#[test]
fn router_device_never_uses_service_identity_probe() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-router-1".to_string(),
        label: "Home Router".to_string(),
        platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
        ip_address: constants::lan_pairing::TEST_ROUTER_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_ROUTER_MAC.to_string(),
        hostname: Some("home-router".to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    assert!(!should_probe_service_identity(
        &device,
        &[],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn trusted_device_mac_mismatch_does_not_suppress_probe_on_reused_ip() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let trusted_device = trusted_device(
        "AA-BB-CC-DD-EE-FF",
        Some(constants::lan_pairing::TEST_LAN_IP),
        Some(constants::lan_pairing::TEST_HOSTNAME),
        constants::lan_pairing::TEST_HOSTNAME,
        constants::lan_pairing::PLATFORM_WINDOWS,
    );

    assert!(should_probe_service_identity(
        &device,
        &[trusted_device],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
}

#[test]
fn service_identity_probe_requires_selected_interface_match() {
    let device = LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
        mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
        hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };

    assert!(should_probe_service_identity(
        &device,
        &[],
        constants::lan_pairing::TEST_NETWORK_INTERFACE,
    ));
    assert!(!should_probe_service_identity(&device, &[], "Ethernet 7",));
}

#[test]
fn probe_response_parser_collects_sanitized_http_title_header_redirect_and_links() {
    let listener = TcpListener::bind("127.0.0.1:0").value_or_unreachable();
    let port = listener.local_addr().value_or_unreachable().port();
    let request_count = Arc::new(AtomicUsize::new(0));
    let request_path = Arc::new(Mutex::new(None::<String>));
    let request_path_clone = Arc::clone(&request_path);
    let request_count_clone = Arc::clone(&request_count);

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().value_or_unreachable();
        request_count_clone.fetch_add(1, Ordering::SeqCst);
        let request = read_request(&mut stream);
        *request_path_clone.lock().value_or_unreachable() = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .map(|value| value.to_string());

        let body = "<html><head><title> Demo\nPanel </title></head><body><a href=\"/child\">child</a></body></html>";
        let response = format!(
            "HTTP/1.1 302 Found\r\nServer: test-banner\r\nX-Powered-By: test-stack\r\nLocation: /admin/login\r\nLink: </metadata>; rel=\"service-desc\"\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
            body.len(),
            body
        );
        stream.write_all(response.as_bytes()).value_or_unreachable();
    });

    let observation = probe_service_identity_on_target(
        "127.0.0.1",
        ProbeTarget {
            port,
            transport: ProbeTransport::Http,
            request_paths: &["/"],
        },
    );

    server.join().value_or_unreachable();

    let Some(observation) = observation else {
        unreachable!("expected probe observation");
    };

    assert_eq!(observation.status_code, Some(302));
    assert_eq!(observation.title.as_deref(), Some("Demo Panel"));
    assert_eq!(observation.server_header.as_deref(), Some("test-banner"));
    assert_eq!(observation.banner.as_deref(), Some("test-stack"));
    assert_eq!(
        observation.redirect_location.as_deref(),
        Some("/admin/login")
    );
    assert_eq!(
        observation.descriptor_links,
        vec!["</metadata>; rel=\"service-desc\"".to_string()]
    );
    assert_eq!(
        request_count.load(Ordering::SeqCst),
        1,
        "probe must not crawl beyond the initial request"
    );
    assert_eq!(
        request_path.lock().value_or_unreachable().as_deref(),
        Some("/")
    );
}

#[test]
fn probe_response_parser_rejects_traversal_references_and_invalid_header_text() {
    let traversal = parse_probe_observation(
        b"HTTP/1.1 302 Found\r\nLocation: /../../secret?x=1\r\nLink: </../../metadata>; rel=\"service-desc\"\r\nContent-Length: 0\r\n\r\n",
        None,
    )
    .value_or_unreachable();

    assert_eq!(traversal.status_code, Some(302));
    assert!(traversal.redirect_location.is_none());
    assert!(traversal.descriptor_links.is_empty());

    assert!(parse_probe_observation(
        b"HTTP/1.1 200 OK\r\nServer: \xff\xfe\r\nContent-Length: 0\r\n\r\n",
        None,
    )
    .is_none());
}

#[test]
fn probe_response_parser_normalizes_backslash_references() {
    let observation = parse_probe_observation(
        b"HTTP/1.1 200 OK\r\nLink: <\\metadata\\service-desc>; rel=\"service-desc\"\r\nContent-Length: 0\r\n\r\n",
        None,
    )
    .value_or_unreachable();

    assert_eq!(
        observation.descriptor_links,
        vec!["</metadata/service-desc>; rel=\"service-desc\"".to_string()]
    );
}

#[test]
fn probe_response_reader_rejects_oversized_responses() {
    let mut response = b"HTTP/1.1 200 OK\r\nContent-Length: 40000\r\n\r\n".to_vec();
    response.extend(std::iter::repeat_n(
        b'a',
        SERVICE_IDENTITY_PROBE_MAX_RESPONSE_BYTES + 1,
    ));

    assert!(read_probe_response(&mut Cursor::new(response)).is_none());
}

#[test]
fn service_identity_probe_stops_when_scan_budget_is_exhausted() {
    let targets = vec![ProbeTarget {
        port: 80,
        transport: ProbeTransport::Http,
        request_paths: &["/"],
    }];

    assert!(probe_service_identity(
        "127.0.0.1",
        Some("camera-1"),
        &targets,
        ServiceIdentityProbeSettings::default(),
        Instant::now(),
        None,
    )
    .is_none());
}

#[test]
fn probe_response_parser_collects_tls_certificate_subject() {
    let cert = generate_simple_self_signed(vec!["service.local".into()]).value_or_unreachable();
    let cert_der = cert.cert.der().clone();
    let certificate_subject = parse_certificate_subject(&cert_der).value_or_unreachable();
    let observation = parse_probe_observation(
        b"HTTP/1.1 200 OK\r\nServer: tls-banner\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 70\r\n\r\n<html><head><title>Secure Control</title></head><body>ok</body></html>",
        Some(certificate_subject.clone()),
    )
    .value_or_unreachable();
    assert_eq!(observation.title.as_deref(), Some("Secure Control"));
    assert_eq!(observation.server_header.as_deref(), Some("tls-banner"));
    assert_eq!(certificate_subject, "CN=rcgen self signed cert");
    assert_eq!(
        observation.certificate_subject.as_deref(),
        Some(certificate_subject.as_str())
    );
}

#[test]
fn enrich_service_identity_probes_is_bounded_by_concurrency() {
    let _env_lock = agent_addr_env_lock();
    let listener = TcpListener::bind("127.0.0.1:0").value_or_unreachable();
    let port = listener.local_addr().value_or_unreachable().port();
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    let active_clone = Arc::clone(&active);
    let max_active_clone = Arc::clone(&max_active);

    let server = thread::spawn(move || {
        let mut handlers = Vec::new();
        for _ in 0..5 {
            let (mut stream, _) = listener.accept().value_or_unreachable();
            let active = Arc::clone(&active_clone);
            let max_active = Arc::clone(&max_active_clone);
            handlers.push(thread::spawn(move || {
                let current = active.fetch_add(1, Ordering::SeqCst) + 1;
                loop {
                    let observed = max_active.load(Ordering::SeqCst);
                    if current > observed {
                        if max_active
                            .compare_exchange(
                                observed,
                                current,
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                        {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                let _ = read_request(&mut stream);
                thread::sleep(Duration::from_millis(150));
                let body = "<html><head><title>Bounded</title></head><body>ok</body></html>";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nServer: bounded\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\n\r\n{}",
                    body.len(),
                    body
                );
                let _ = stream.write_all(response.as_bytes());
                active.fetch_sub(1, Ordering::SeqCst);
            }));
        }

        for handler in handlers {
            handler.join().value_or_unreachable();
        }
    });

    let previous_agent_addr = env::var(constants::env_var::AGENT_ADDR).ok();
    env::set_var(constants::env_var::AGENT_ADDR, format!("127.0.0.1:{port}"));

    let mut devices =
        (0..5)
            .map(|index| LanNetworkInventoryDevice {
                device_id: format!("lan-device-{index}"),
                label: format!("device-{index}"),
                platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
                ip_address: "127.0.0.1".to_string(),
                mac_address: format!("AA-BB-CC-DD-EE-{index:02X}"),
                hostname: None,
                network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
                reachability: LanPairingDeviceReachability::Online,
                agent_status: None,
                scan_sources: vec![
                    constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()
                ],
                observed_at: String::new(),
                used_previous_scan_hint: false,
                service_identity_probe_evidence: Vec::new(),
            })
            .collect::<Vec<_>>();

    enrich_service_identity_probes(
        &mut devices,
        &[],
        Some(constants::lan_pairing::TEST_NETWORK_INTERFACE),
        None,
    );

    if let Some(previous_agent_addr) = previous_agent_addr {
        env::set_var(constants::env_var::AGENT_ADDR, previous_agent_addr);
    } else {
        env::remove_var(constants::env_var::AGENT_ADDR);
    }

    server.join().value_or_unreachable();

    assert!(devices
        .iter()
        .all(|device| is_service_identity_probe_status(device.agent_status.as_deref())));
    assert!(devices.iter().all(|device| {
        device
            .service_identity_probe_evidence
            .iter()
            .any(|evidence| {
                evidence.evidence_kind == LanServiceIdentityProbeEvidenceKind::HtmlTitle
            })
    }));
    assert_eq!(max_active.load(Ordering::SeqCst), 4);
}

#[test]
fn service_identity_probe_family_policy_keeps_optional_queries_disabled_by_default() {
    let decisions =
        service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default());

    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::HttpTcp)
            .map(|decision| decision.decision),
        Some(ServiceIdentityProbeDecision::Execute)
    );
    assert!(decisions
        .iter()
        .all(|decision| decision.requires_discovered_host && decision.weak_evidence_only));
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            ServiceIdentityProbeDecision::OperatorSettingRequired,
            true,
            true
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            ServiceIdentityProbeDecision::OperatorSettingRequired,
            true,
            true
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::OsFingerprint)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((ServiceIdentityProbeDecision::ManualGateRequired, true, true))
    );
}

#[test]
fn optional_identity_queries_become_bounded_execute_only_when_enabled() {
    let decisions = service_identity_probe_family_decisions(ServiceIdentityProbeSettings {
        allow_wsd_identity_query: true,
        allow_snmp_identity_query: true,
        allow_os_fingerprint: true,
    });

    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
            .map(|decision| {
                (
                    &decision.decision,
                    decision.allowed_ports.as_slice(),
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            &ServiceIdentityProbeDecision::Execute,
            &[5357][..],
            true,
            true,
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
            .map(|decision| {
                (
                    &decision.decision,
                    decision.allowed_ports.as_slice(),
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            &ServiceIdentityProbeDecision::Execute,
            &[161][..],
            true,
            true,
        ))
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::OsFingerprint)
            .map(|decision| {
                (
                    decision.decision,
                    decision.requires_discovered_host,
                    decision.weak_evidence_only,
                )
            }),
        Some((
            ServiceIdentityProbeDecision::RuntimeNotImplemented,
            true,
            true
        ))
    );

    let executable = decisions
        .iter()
        .filter(|decision| decision.decision == ServiceIdentityProbeDecision::Execute)
        .collect::<Vec<_>>();
    assert_eq!(executable.len(), 3);
    assert!(executable
        .iter()
        .any(|decision| decision.family == ServiceIdentityProbeFamily::HttpTcp));
    assert!(executable
        .iter()
        .any(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery));
    assert!(executable
        .iter()
        .any(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery));
}

#[test]
fn runtime_visible_service_identity_policy_keeps_optional_families_weak_and_bounded() {
    let decisions =
        service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default());

    let wsd = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
        .value_or_unreachable();
    assert_eq!(
        wsd.decision,
        ServiceIdentityProbeDecision::OperatorSettingRequired
    );
    assert_eq!(wsd.allowed_ports, vec![5357]);
    assert!(wsd.requires_discovered_host);
    assert!(wsd.weak_evidence_only);

    let snmp = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
        .value_or_unreachable();
    assert_eq!(
        snmp.decision,
        ServiceIdentityProbeDecision::OperatorSettingRequired
    );
    assert_eq!(snmp.allowed_ports, vec![161]);
    assert!(snmp.requires_discovered_host);
    assert!(snmp.weak_evidence_only);

    let os = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::OsFingerprint)
        .value_or_unreachable();
    assert_eq!(
        os.decision,
        ServiceIdentityProbeDecision::ManualGateRequired
    );
    assert!(os.allowed_ports.is_empty());
    assert!(os.requires_discovered_host);
    assert!(os.weak_evidence_only);
}

#[test]
fn runtime_service_identity_settings_keep_optional_queries_disabled_by_default() {
    let _guard = service_identity_env_lock();
    let previous_wsd = env::var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV).ok();
    let previous_snmp = env::var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV).ok();
    env::remove_var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV);
    env::remove_var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV);

    let settings = runtime_service_identity_probe_settings();

    assert_eq!(
        settings,
        ServiceIdentityProbeSettings {
            allow_wsd_identity_query: false,
            allow_snmp_identity_query: false,
            allow_os_fingerprint: false,
        }
    );

    let decisions = service_identity_probe_family_decisions(settings);

    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::WsdIdentityQuery)
            .map(|decision| decision.decision),
        Some(ServiceIdentityProbeDecision::OperatorSettingRequired)
    );
    assert_eq!(
        decisions
            .iter()
            .find(|decision| decision.family == ServiceIdentityProbeFamily::SnmpIdentityQuery)
            .map(|decision| decision.decision),
        Some(ServiceIdentityProbeDecision::OperatorSettingRequired)
    );

    restore_optional_probe_env(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        previous_wsd.as_deref(),
    );
    restore_optional_probe_env(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        previous_snmp.as_deref(),
    );
}

#[test]
fn runtime_service_identity_settings_enable_optional_queries_from_env() {
    let _guard = service_identity_env_lock();
    let previous_wsd = env::var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV).ok();
    let previous_snmp = env::var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV).ok();
    env::set_var(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        "true",
    );
    env::set_var(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        "1",
    );

    let settings = runtime_service_identity_probe_settings();

    assert_eq!(
        settings,
        ServiceIdentityProbeSettings {
            allow_wsd_identity_query: true,
            allow_snmp_identity_query: true,
            allow_os_fingerprint: false,
        }
    );

    restore_optional_probe_env(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        previous_wsd.as_deref(),
    );
    restore_optional_probe_env(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        previous_snmp.as_deref(),
    );
}

#[test]
fn executable_service_identity_target_catalog_remains_curated_tcp_only() {
    let targets = service_identity_probe_targets();
    let decisions =
        service_identity_probe_family_decisions(ServiceIdentityProbeSettings::default());
    let http_tcp_ports = decisions
        .iter()
        .find(|decision| decision.family == ServiceIdentityProbeFamily::HttpTcp)
        .map(|decision| decision.allowed_ports.clone())
        .value_or_unreachable();

    assert_eq!(targets.len(), http_tcp_ports.len());
    assert!(targets
        .iter()
        .all(|target| http_tcp_ports.contains(&target.port)));
    assert!(targets.iter().all(|target| {
        matches!(
            target.transport,
            ProbeTransport::Http | ProbeTransport::Https
        )
    }));
    assert!(!targets.iter().any(|target| target.port == 161));
    assert!(!targets.iter().any(|target| target.port == 3702));
}

fn read_request(stream: &mut impl Read) -> String {
    let mut request = Vec::new();
    let mut chunk = [0_u8; 512];
    let mut expected_total_len = None;

    loop {
        let read = stream.read(&mut chunk).value_or_unreachable();
        if read == 0 {
            break;
        }
        request.extend_from_slice(&chunk[..read]);
        if let Some(expected_total_len) = expected_total_len {
            if request.len() >= expected_total_len {
                break;
            }
            continue;
        }
        if let Some(header_end) = request.windows(4).position(|window| window == b"\r\n\r\n") {
            let header_len = header_end + 4;
            let content_length = String::from_utf8_lossy(&request[..header_end])
                .lines()
                .find_map(|line| {
                    let (name, value) = line.split_once(':')?;
                    name.trim()
                        .eq_ignore_ascii_case("content-length")
                        .then(|| value.trim().parse::<usize>().ok())
                        .flatten()
                })
                .unwrap_or(0);
            let total_len = header_len + content_length;
            if request.len() >= total_len {
                break;
            }
            expected_total_len = Some(total_len);
        }
    }

    String::from_utf8_lossy(&request).into_owned()
}

fn restore_optional_probe_env(name: &str, value: Option<&str>) {
    match value {
        Some(value) => env::set_var(name, value),
        None => env::remove_var(name),
    }
}
