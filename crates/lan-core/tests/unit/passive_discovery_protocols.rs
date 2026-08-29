use super::*;
use ocentra_lan_core::network_inventory::passive_discovery::dns_like::{
    passive_llmnr_summary, passive_netbios_summary,
};
use ocentra_lan_core::network_inventory::passive_discovery::packet::parse_passive_discovery_packet;
use ocentra_lan_core::network_inventory::passive_discovery::raw_socket::raw_socket_protocol_support;
use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::{
    collect_allowed_snmp_response_packets, drain_udp_socket_packets,
    drain_udp_socket_packets_with_observed_at, ingest_passive_datagram,
    ingest_passive_datagram_with_observed_at, udp_multicast_support,
};

#[test]
fn native_ws_discovery_datagram_records_passive_observation_withost_json_envelope() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = br#"
        <s:Envelope xmlns:s="http://www.w3.org/2003/05/soap-envelope"
            xmlns:a="http://schemas.xmlsoap.org/ws/2004/08/addressing"
            xmlns:d="http://schemas.xmlsoap.org/ws/2005/04/discovery">
          <s:Header>
            <a:Action>http://schemas.xmlsoap.org/ws/2005/04/discovery/ProbeMatches</a:Action>
          </s:Header>
          <s:Body>
            <d:ProbeMatches>
              <d:ProbeMatch>
                <a:EndpointReference><a:Address>urn:uuid:camera-1</a:Address></a:EndpointReference>
                <d:Types>dn:NetworkVideoTransmitter</d:Types>
                <d:XAddrs>http://192.168.1.45/onvif/device_service</d:XAddrs>
              </d:ProbeMatch>
            </d:ProbeMatches>
          </s:Body>
        </s:Envelope>
    "#;

    assert_eq!(
        ingest_passive_datagram(&mut state, &LanPassiveDiscoverySource::WsDiscovery, payload),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::WsDiscovery)
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "WS-Discovery packet: action=http://schemas.xmlsoap.org/ws/2005/04/discovery/ProbeMatches; endpoint=urn:uuid:camera-1; types=dn:NetworkVideoTransmitter; xaddrs=http://192.168.1.45/onvif/device_service"
            .to_string()
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("camera-1")
    );
}

#[path = "../../src/network_inventory/passive_discovery/udp_multicast/deadline.rs"]
mod deadline_helper;
#[path = "../../src/network_inventory/passive_discovery/udp_multicast/timeout_guard.rs"]
mod timeout_guard_helper;

#[test]
fn passive_listener_deadline_is_absolute_and_expiry_is_empty() {
    let start = std::time::Instant::now();
    let deadline = start + std::time::Duration::from_secs(2);
    assert_eq!(deadline_helper::remaining_read_timeout_at(deadline, start + std::time::Duration::from_millis(900)), Some(std::time::Duration::from_millis(1100)));
    assert_eq!(deadline_helper::remaining_read_timeout_at(deadline, deadline), None);
}

#[test]
fn passive_listener_timeout_guard_restores_explicitly() -> std::io::Result<()> {
    let socket = std::net::UdpSocket::bind("127.0.0.1:0")?;
    socket.set_read_timeout(Some(std::time::Duration::from_millis(25)))?;
    let previous = socket.read_timeout()?;
    let mut guard = timeout_guard_helper::ReadTimeoutRestoreGuard::new(&socket, previous);
    socket.set_read_timeout(Some(std::time::Duration::from_millis(1)))?;
    guard.restore()?;
    assert_eq!(socket.read_timeout()?, previous);
    Ok(())
}

#[test]
fn passive_listener_timeout_guard_restores_on_unwind() -> std::io::Result<()> {
    let socket = std::net::UdpSocket::bind("127.0.0.1:0")?;
    socket.set_read_timeout(Some(std::time::Duration::from_millis(25)))?;
    let previous = socket.read_timeout()?;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = timeout_guard_helper::ReadTimeoutRestoreGuard::new(&socket, previous);
        assert!(socket.set_read_timeout(Some(std::time::Duration::from_millis(1))).is_ok());
        panic!("exercise timeout restoration on unwind");
    }));
    assert!(result.is_err());
    assert_eq!(socket.read_timeout()?, previous);
    Ok(())
}

#[test]
fn native_llmnr_datagram_records_passive_observation_withost_json_envelope() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::Llmnr,
            &llmnr_query_packet("kid-laptop.local"),
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Llmnr)
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("kid-laptop.local")
    );
    let summary = snapshot.rows[0]
        .summary
        .strip_prefix("LLMNR packet: ")
        .value_or_unreachable();
    let summary_parts = summary.split("; ").collect::<Vec<_>>();
    assert_eq!(summary_parts.len(), 9);
    assert_eq!(
        &summary_parts[..4],
        vec![
            "name=kid-laptop.local",
            "normalized=kid-laptop.local",
            "source=llmnr",
            "confidence=weak",
        ]
    );
    let first_seen_at = summary_parts[4].split('=').nth(1).value_or_unreachable();
    let last_seen_at = summary_parts[5].split('=').nth(1).value_or_unreachable();
    DateTime::parse_from_rfc3339(first_seen_at).value_or_unreachable();
    DateTime::parse_from_rfc3339(last_seen_at).value_or_unreachable();
    assert_eq!(first_seen_at, last_seen_at);
    assert_eq!(
        &summary_parts[6..],
        vec!["interface=n/a", "questions=1", "answers=0"]
    );
}

#[test]
fn native_netbios_datagram_records_passive_observation_withost_json_envelope() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::Netbios,
            &netbios_name_query_packet("KID-LAPTOP"),
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Netbios)
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("kid-laptop")
    );
    let summary = snapshot.rows[0]
        .summary
        .strip_prefix("NetBIOS name packet: ")
        .value_or_unreachable();
    let summary_parts = summary.split("; ").collect::<Vec<_>>();
    assert_eq!(summary_parts.len(), 9);
    assert_eq!(
        &summary_parts[..4],
        vec![
            "name=KID-LAPTOP",
            "normalized=kid-laptop",
            "source=netbios",
            "confidence=weak",
        ]
    );
    let first_seen_at = summary_parts[4].split('=').nth(1).value_or_unreachable();
    let last_seen_at = summary_parts[5].split('=').nth(1).value_or_unreachable();
    DateTime::parse_from_rfc3339(first_seen_at).value_or_unreachable();
    DateTime::parse_from_rfc3339(last_seen_at).value_or_unreachable();
    assert_eq!(first_seen_at, last_seen_at);
    assert_eq!(
        &summary_parts[6..],
        vec!["interface=n/a", "questions=1", "answers=0"]
    );
}

#[test]
fn native_name_datagram_ssmmaries_reject_snsafe_names() {
    assert!(passive_llmnr_summary(&llmnr_query_packet("bad host")).is_none());
    assert!(passive_netbios_summary(&netbios_name_query_packet("BAD HOST")).is_none());
}

#[test]
fn native_name_datagram_ssmmaries_reject_overlong_names() {
    assert!(passive_llmnr_summary(&llmnr_query_packet(&"a".repeat(64))).is_none());
    assert!(
        passive_netbios_summary(&malformed_netbios_name_query_packet(&"A".repeat(64))).is_none()
    );
}

#[test]
fn raw_arp_datagram_records_bytes_only_passive_observation() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = [0xde, 0xad, 0xbe, 0xef];

    assert_eq!(
        ingest_passive_datagram(&mut state, &LanPassiveDiscoverySource::Arp, &payload),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Arp)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(snapshot.rows[0].summary, "ARP packet: 4 byte(s)");
}

#[test]
fn dhcp_datagram_extracts_bosnded_identity_fields() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let observed_at = "2026-06-25T00:00:05Z";
    let payload = dhcp_packet(
        [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff],
        "kitchen-tablet",
        "android-dhcp-14",
        &[1, 3, 6, 15, 119],
    );

    assert_eq!(
        ingest_passive_datagram_with_observed_at(
            &mut state,
            &LanPassiveDiscoverySource::Dhcp,
            &payload,
            observed_at,
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(snapshot.rows[0].observed_at, observed_at);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Dhcp)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("aa-bb-cc-dd-ee-ff")
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "DHCP packet: type=discover; client-mac=aa-bb-cc-dd-ee-ff; client-id=ethernet:aa-bb-cc-dd-ee-ff; hostname=kitchen-tablet; vendor-class=android-dhcp-14; params=1,3,6,15,119"
    );
}

#[test]
fn malformed_dhcp_options_fall_back_to_bytes_only_observation() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let mut payload = vec![0_u8; 240];
    payload[0] = 1;
    payload[1] = 1;
    payload[2] = 6;
    payload[236..240].copy_from_slice(&[99, 130, 83, 99]);
    payload[28..34].copy_from_slice(&[0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
    payload.push(12);
    payload.push(10);
    payload.extend_from_slice(b"short");

    assert_eq!(
        ingest_passive_datagram(&mut state, &LanPassiveDiscoverySource::Dhcp, &payload),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Dhcp)
    );
    assert_eq!(
        snapshot.rows[0].summary,
        format!("DHCP packet: {} byte(s)", payload.len())
    );
}

#[test]
fn allowed_snmp_response_datagram_records_parsed_identity_observation() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = allowed_snmp_response_payload("Linux camera controller", "cam-1");

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::AllowedSnmpResponse,
            &payload,
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::AllowedSnmpResponse)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "allowed SNMP response: sys-name=cam-1; sys-descr=Linux camera controller"
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("cam-1")
    );
}

#[test]
fn allowed_snmp_response_datagram_keeps_partial_identity_withost_inventing_device_id() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = allowed_snmp_response_payload("Linux camera controller", "");

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::AllowedSnmpResponse,
            &payload,
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::AllowedSnmpResponse)
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "allowed SNMP response: sys-descr=Linux camera controller"
    );
    assert_eq!(snapshot.rows[0].device_id, None);
}

#[test]
fn allowed_snmp_response_datagram_rejects_malformed_payload() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = [0x30, 0x03, 0x02, 0x01, 0x05, 0x00];

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::AllowedSnmpResponse,
            &payload,
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Rejected(
            LanPassiveDiscoveryPacketParseError::MalformedPayload,
        )
    );
    assert!(state.snapshot().rows.is_empty());
}

#[test]
fn explicit_allowed_snmp_socket_packets_feed_the_existing_passive_state_path() {
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable();

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    let receiver_addr = receiver.local_addr().value_or_unreachable();
    let payload = allowed_snmp_response_payload("Linux camera controller", "cam-1");

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable();

    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    assert_eq!(
        collect_allowed_snmp_response_packets(&receiver, &mut state, 1),
        1
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::AllowedSnmpResponse)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "allowed SNMP response: sys-name=cam-1; sys-descr=Linux camera controller"
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("cam-1")
    );
}

#[test]
fn explicit_allowed_snmp_socket_packets_keep_malformed_payloads_ost_of_history() {
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable();

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    let receiver_addr = receiver.local_addr().value_or_unreachable();
    let payload = [0x30, 0x03, 0x02, 0x01, 0x05, 0x00];

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable();

    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    assert_eq!(
        collect_allowed_snmp_response_packets(&receiver, &mut state, 1),
        1
    );
    assert!(state.snapshot().rows.is_empty());
}

#[test]
fn ocentra_beacon_datagram_records_bytes_only_passive_observation() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = [0x6f, 0x63, 0x65, 0x6e, 0x74, 0x72, 0x61];

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::OcentraBeacon,
            &payload,
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::OcentraBeacon)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(snapshot.rows[0].summary, "Ocentra beacon packet: 7 byte(s)");
}

#[test]
fn sdp_packet_ingest_dedupes_replayed_datagrams() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = serde_json::to_vec(&serde_json::json!({
        "schemaVersion": LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
        "source": "ocentra-beacon",
        "triggerReason": "app-resumed",
        "observedAt": "2026-06-25T00:00:01Z",
        "deviceId": "device-sdp-2",
        "scanSessionId": "scan-sdp-2",
        "summary": "beacon update"
    }))
    .value_or_unreachable();

    assert_eq!(
        state.ingest_udp_packet(&payload),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );
    assert_eq!(
        state.ingest_udp_packet(&payload),
        LanPassiveDiscoveryPacketIngestOutcome::Deduplicated
    );
    assert_eq!(state.snapshot().rows.len(), 1);
}

#[test]
fn sdp_packet_parser_rejects_malformed_and_oversized_payloads() {
    assert_eq!(
        parse_passive_discovery_packet(b"not-json"),
        Err(LanPassiveDiscoveryPacketParseError::MalformedPayload)
    );

    let oversized = vec![b'a'; LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES + 1];
    assert_eq!(
        parse_passive_discovery_packet(&oversized),
        Err(LanPassiveDiscoveryPacketParseError::OversizedPayload {
            payload_len: LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES + 1,
            max_payload_len: LAN_PASSIVE_DISCOVERY_MAX_PACKET_BYTES,
        })
    );
}

#[test]
fn sdp_packet_parser_rejects_empty_and_schema_mismatch_payloads() {
    assert_eq!(
        parse_passive_discovery_packet(&[]),
        Err(LanPassiveDiscoveryPacketParseError::EmptyPayload)
    );

    let payload = serde_json::to_vec(&serde_json::json!({
        "schemaVersion": 2_u16,
        "source": "mdns",
        "triggerReason": "wifi-ssid-changed",
        "observedAt": "2026-06-25T00:00:01Z",
        "summary": "bad schema"
    }))
    .value_or_unreachable();

    assert_eq!(
        parse_passive_discovery_packet(&payload),
        Err(
            LanPassiveDiscoveryPacketParseError::UnsupportedSchemaVersion {
                schema_version: 2,
                expected_schema_version: LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
            }
        )
    );
}

#[test]
fn raw_socket_protocols_report_csrrent_arp_collector_backing_and_dhcp_gap() {
    let expected_arp = match std::env::consts::OS {
        "windows" => LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "windows".to_string(),
            collector_labels: vec!["windows-neighbor-table".to_string()],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        },
        "linux" => LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "linux".to_string(),
            collector_labels: vec![
                "linux-proc-net-arp".to_string(),
                "linux-ip-neigh".to_string(),
            ],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        },
        "android" => LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "android".to_string(),
            collector_labels: vec![
                "linux-proc-net-arp".to_string(),
                "linux-ip-neigh".to_string(),
            ],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        },
        "macos" => LanPassiveDiscoveryRawSocketSupport::AvailableCollector {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: "macos".to_string(),
            collector_labels: vec!["macos-arp".to_string()],
            reason: "lan-core records passive ARP weak hints from OS neighbor tables instead of raw frames"
                .to_string(),
        },
        platform => LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Arp,
            platform: platform.to_string(),
            reason: "no passive ARP collector is implemented for this platform".to_string(),
        },
    };
    assert_eq!(
        raw_socket_protocol_support(LanPassiveDiscoveryRawSocketProtocol::Arp),
        expected_arp
    );
    assert_eq!(
        raw_socket_protocol_support(LanPassiveDiscoveryRawSocketProtocol::Dhcp),
        LanPassiveDiscoveryRawSocketSupport::UnsupportedPlatform {
            protocol: LanPassiveDiscoveryRawSocketProtocol::Dhcp,
            platform: std::env::consts::OS.to_string(),
            reason: "raw-socket passive capture is not implemented in lan-core".to_string(),
        }
    );
}

#[test]
fn udp_multicast_support_reports_real_supported_sources_and_manual_blockers() {
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::Mdns),
        LanPassiveDiscoveryUdpMulticastSupport::Available {
            source: LanPassiveDiscoverySource::Mdns,
            multicast_group: Ipv4Addr::new(224, 0, 0, 251).to_string(),
            port: 5353,
        }
    );
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::Ssdp),
        LanPassiveDiscoveryUdpMulticastSupport::Available {
            source: LanPassiveDiscoverySource::Ssdp,
            multicast_group: Ipv4Addr::new(239, 255, 255, 250).to_string(),
            port: 1900,
        }
    );
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::WsDiscovery),
        LanPassiveDiscoveryUdpMulticastSupport::Available {
            source: LanPassiveDiscoverySource::WsDiscovery,
            multicast_group: Ipv4Addr::new(239, 255, 255, 250).to_string(),
            port: 3702,
        }
    );
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::Llmnr),
        LanPassiveDiscoveryUdpMulticastSupport::Available {
            source: LanPassiveDiscoverySource::Llmnr,
            multicast_group: Ipv4Addr::new(224, 0, 0, 252).to_string(),
            port: 5355,
        }
    );
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::Netbios),
        LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast {
            source: LanPassiveDiscoverySource::Netbios,
            port: 137,
        }
    );
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::Dhcp),
        LanPassiveDiscoveryUdpMulticastSupport::AvailableBroadcast {
            source: LanPassiveDiscoverySource::Dhcp,
            port: 67,
        }
    );
    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::Arp),
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
            source: LanPassiveDiscoverySource::Arp,
            reason: "raw-socket passive capture is not implemented in lan-core".to_string(),
        }
    );

    assert_eq!(
        udp_multicast_support(LanPassiveDiscoverySource::AllowedSnmpResponse),
        LanPassiveDiscoveryUdpMulticastSupport::Unsupported {
            source: LanPassiveDiscoverySource::AllowedSnmpResponse,
            reason: "passive SNMP response capture requires an explicit allowed probe socket"
                .to_string(),
        }
    );
}

#[test]
fn udp_socket_packets_feed_the_existing_passive_state_path() {
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable();

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    let receiver_addr = receiver.local_addr().value_or_unreachable();
    let payload = serde_json::to_vec(&serde_json::json!({
        "schemaVersion": LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
        "source": "mdns",
        "triggerReason": "app-resumed",
        "observedAt": "2026-06-25T00:00:01Z",
        "deviceId": "device-sdp-transport",
        "scanSessionId": "scan-sdp-transport",
        "summary": "mdns multicast update"
    }))
    .value_or_unreachable();

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable();

    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    assert_eq!(
        drain_udp_socket_packets(&receiver, &mut state, LanPassiveDiscoverySource::Mdns, 1),
        1
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Mdns)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::AppResumed
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("device-sdp-transport")
    );
    assert_eq!(
        snapshot.rows[0].scan_session_id.as_ref().map(AsRef::as_ref),
        Some("scan-sdp-transport")
    );
    assert_eq!(snapshot.rows[0].summary, "mdns multicast update");
}

#[test]
fn sdp_broadcast_dhcp_packets_feed_the_existing_passive_state_path() {
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable();

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable();
    let receiver_addr = receiver.local_addr().value_or_unreachable();
    let payload = dhcp_packet(
        [0xde, 0xad, 0xbe, 0xef, 0x00, 0x01],
        "living-room-tv",
        "android-dhcp-14",
        &[1, 3, 6],
    );

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable();

    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let mut observed_at = || "2026-06-25T00:00:09Z".to_string();
    assert_eq!(
        drain_udp_socket_packets_with_observed_at(
            &receiver,
            &mut state,
            LanPassiveDiscoverySource::Dhcp,
            1,
            &mut observed_at,
        ),
        1
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(snapshot.rows[0].observed_at, "2026-06-25T00:00:09Z");
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Dhcp)
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("de-ad-be-ef-00-01")
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "DHCP packet: type=discover; client-mac=de-ad-be-ef-00-01; client-id=ethernet:de-ad-be-ef-00-01; hostname=living-room-tv; vendor-class=android-dhcp-14; params=1,3,6"
    );
}
