use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::env;
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, UdpSocket};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
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

#[path = "service_identity_http.rs"]
mod service_identity_http;
#[path = "service_identity_policy.rs"]
mod service_identity_policy;
#[path = "service_identity_request_support.rs"]
mod service_identity_request_support;
#[path = "service_identity_runtime.rs"]
mod service_identity_runtime;
#[path = "service_identity_snmp.rs"]
mod service_identity_snmp;
#[path = "service_identity_support.rs"]
mod service_identity_support;

use service_identity_request_support::read_request;
use service_identity_support::{agent_addr_env_lock, service_identity_env_lock};

macro_rules! wsd_identity_query_response {
    () => {{
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
    }};
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
    let response = wsd_identity_query_response!();

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
        assert!(request.0.starts_with("POST /camera-1 HTTP/1.1\r\n"));
        assert!(request.0.lines().any(|line| {
            line
                == "Content-Type: application/soap+xml; charset=utf-8; action=\"http://schemas.xmlsoap.org/ws/2004/09/transfer/Get\""
        }));
        stream
            .write_all(wsd_identity_query_response!().as_bytes())
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
        Some(&|payload| match observed_payload.lock() {
            Ok(mut guard) => guard.push(payload.to_vec()),
            Err(error) => {
                let mut guard = error.into_inner();
                guard.push(payload.to_vec());
            }
        }),
    )
    .value_or_unreachable();

    server.join().value_or_unreachable();

    assert!(observation.observed_allowed_snmp_response());
    let observed_payload = observed_payload.into_inner().ok();
    assert_eq!(
        observed_payload.as_deref(),
        Some(std::slice::from_ref(&expected_response))
    );
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
