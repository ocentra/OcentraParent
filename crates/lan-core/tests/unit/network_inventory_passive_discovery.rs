use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

use chrono::DateTime;
use ocentra_lan_core::network_inventory::passive_discovery::*;

fn mdns_packet_with_child_service() -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&2_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    encode_dns_name(
        ocentra_parent_agent_protocol::constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE,
        &mut packet,
    );
    packet.extend_from_slice(&12_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&60_u32.to_be_bytes());
    let mut data = Vec::new();
    encode_dns_name(
        &format!(
            "Kitchen Tablet.{}",
            ocentra_parent_agent_protocol::constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
        ),
        &mut data,
    );
    packet.extend_from_slice(&(data.len() as u16).to_be_bytes());
    packet.extend_from_slice(&data);
    encode_dns_name(
        &format!(
            "Kitchen Tablet.{}",
            ocentra_parent_agent_protocol::constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
        ),
        &mut packet,
    );
    packet.extend_from_slice(&16_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&60_u32.to_be_bytes());
    let txt = encode_txt_data(&[
        "lan.mdns_advertisement_id=sha256:child-id",
        "opaque-device-id=opaque-child-id",
        "protocol-version=2.0.0",
        "family-hash=sha256:family-parent",
        "platform=windows",
        "agent-version=1.2.3",
        "pairing-state=unpaired",
        "lifecycle-state=update",
        "support-state=degraded",
    ]);
    packet.extend_from_slice(&(txt.len() as u16).to_be_bytes());
    packet.extend_from_slice(&txt);
    packet
}

fn encode_dns_name(name: &str, packet: &mut Vec<u8>) {
    for label in name.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
}

fn encode_txt_data(entries: &[&str]) -> Vec<u8> {
    let mut data = Vec::new();
    for entry in entries {
        data.push(entry.len() as u8);
        data.extend_from_slice(entry.as_bytes());
    }
    data
}

fn encode_ber_tlv(tag: u8, payload: &[u8]) -> Vec<u8> {
    let mut encoded = vec![tag];
    match payload.len() {
        0..=0x7f => encoded.push(payload.len() as u8),
        0x80..=0xff => {
            encoded.push(0x81);
            encoded.push(payload.len() as u8);
        }
        _ => {
            encoded.push(0x82);
            encoded.push(((payload.len() >> 8) & 0xff) as u8);
            encoded.push((payload.len() & 0xff) as u8);
        }
    }
    encoded.extend_from_slice(payload);
    encoded
}

fn encode_ber_integer(value: i64) -> Vec<u8> {
    let mut bytes = value.to_be_bytes().to_vec();
    while bytes.len() > 1
        && ((bytes[0] == 0x00 && bytes[1] & 0x80 == 0)
            || (bytes[0] == 0xff && bytes[1] & 0x80 == 0x80))
    {
        bytes.remove(0);
    }
    bytes
}

fn encode_ber_oid(oid: &[u32]) -> Vec<u8> {
    let Some((&first, rest)) = oid.split_first() else {
        return Vec::new();
    };
    let Some((&second, tail)) = rest.split_first() else {
        return vec![(first * 40) as u8];
    };
    let mut encoded = vec![(first * 40 + second) as u8];
    for component in tail {
        let mut stack = vec![(component & 0x7f) as u8];
        let mut value = *component >> 7;
        while value > 0 {
            stack.push(((value & 0x7f) as u8) | 0x80);
            value >>= 7;
        }
        stack.reverse();
        encoded.extend(stack);
    }
    encoded
}

fn allowed_snmp_response_payload(sys_descr: &str, sys_name: &str) -> Vec<u8> {
    const BER_TAG_INTEGER: u8 = 0x02;
    const BER_TAG_OCTET_STRING: u8 = 0x04;
    const BER_TAG_OBJECT_IDENTIFIER: u8 = 0x06;
    const BER_TAG_SEQUENCE: u8 = 0x30;
    const SNMP_GET_RESPONSE_TAG: u8 = 0xA2;
    const SNMP_VERSION_2C: i64 = 1;
    const SNMP_REQUEST_ID: i64 = 1;
    const SNMP_SYS_DESCR_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 1, 0];
    const SNMP_SYS_NAME_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 5, 0];

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
                    encode_ber_tlv(BER_TAG_OCTET_STRING, sys_descr.as_bytes()),
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
                    encode_ber_tlv(BER_TAG_OCTET_STRING, sys_name.as_bytes()),
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
            encode_ber_tlv(BER_TAG_OCTET_STRING, b"psblic"),
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

fn llmnr_query_packet(name: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0x1234_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    encode_dns_name(name, &mut packet);
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet
}

fn netbios_name_query_packet(name: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0x1234_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    encode_dns_name(&encoded_netbios_name(name), &mut packet);
    packet.extend_from_slice(&0x20_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet
}

fn malformed_netbios_name_query_packet(name: &str) -> Vec<u8> {
    let mut packet = Vec::new();
    packet.extend_from_slice(&0x1234_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    packet.extend_from_slice(&0_u16.to_be_bytes());
    encode_dns_name(name, &mut packet);
    packet.extend_from_slice(&0x20_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet
}

fn dhcp_packet(
    client_mac: [u8; 6],
    hostname: &str,
    vendor_class: &str,
    parameter_request_list: &[u8],
) -> Vec<u8> {
    let mut packet = vec![0_u8; 240];
    packet[0] = 1;
    packet[1] = 1;
    packet[2] = 6;
    packet[236..240].copy_from_slice(&[99, 130, 83, 99]);
    packet[28..34].copy_from_slice(&client_mac);

    packet.push(53);
    packet.push(1);
    packet.push(1);

    packet.push(61);
    packet.push(7);
    packet.push(1);
    packet.extend_from_slice(&client_mac);

    packet.push(12);
    packet.push(hostname.len() as u8);
    packet.extend_from_slice(hostname.as_bytes());

    packet.push(60);
    packet.push(vendor_class.len() as u8);
    packet.extend_from_slice(vendor_class.as_bytes());

    packet.push(55);
    packet.push(parameter_request_list.len() as u8);
    packet.extend_from_slice(parameter_request_list);

    packet.push(255);
    packet
}

fn encoded_netbios_name(name: &str) -> String {
    let mut bytes = [b' '; 16];
    for (index, byte) in name.as_bytes().iter().copied().take(15).enumerate() {
        bytes[index] = byte.to_ascii_uppercase();
    }
    let mut encoded = String::new();
    for byte in bytes {
        encoded.push(char::from(b'A' + ((byte >> 4) & 0x0f)));
        encoded.push(char::from(b'A' + (byte & 0x0f)));
    }
    encoded
}

#[test]
fn passive_listener_dedupes_replayed_rows_by_stable_event_id() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());

    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::Mdns,
            LanPassiveDiscoveryTriggerReason::WifiSsidChanged,
            "2026-06-25T00:00:01Z",
            Some("device-1"),
            Some("scan-1"),
            "mdns update",
        ),
        LanPassiveDiscoveryRecordOutcome::Recorded
    );
    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::Mdns,
            LanPassiveDiscoveryTriggerReason::WifiSsidChanged,
            "2026-06-25T00:00:01Z",
            Some("device-1"),
            Some("scan-1"),
            "mdns update replay",
        ),
        LanPassiveDiscoveryRecordOutcome::Deduplicated
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.latest_event_id,
        Some(snapshot.rows[0].event_id.clone())
    );
    assert_eq!(snapshot.rows[0].previous_event_id, None);
}

#[test]
fn passive_listener_records_source_and_trigger_reason() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());

    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::OcentraBeacon,
            LanPassiveDiscoveryTriggerReason::WifiSsidChanged,
            "2026-06-25T00:00:01Z",
            Some("device-3"),
            Some("scan-3"),
            "beacon update",
        ),
        LanPassiveDiscoveryRecordOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].event_kind,
        LanPassiveDiscoveryEventKind::PassiveUpdate
    );
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::OcentraBeacon)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::WifiSsidChanged
    );
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("device-3"));
    assert_eq!(snapshot.rows[0].scan_session_id.as_deref(), Some("scan-3"));
}

#[test]
fn passive_listener_records_rescan_trigger_withost_source() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());

    assert_eq!(
        state.record_rescan_trigger(
            LanPassiveDiscoveryTriggerReason::HeartbeatLost,
            "2026-06-25T00:00:01Z",
            "heartbeat lost",
        ),
        LanPassiveDiscoveryRecordOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].event_kind,
        LanPassiveDiscoveryEventKind::RescanTrigger
    );
    assert_eq!(snapshot.rows[0].source, None);
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::HeartbeatLost
    );
    assert_eq!(snapshot.rows[0].device_id, None);
    assert_eq!(snapshot.rows[0].scan_session_id, None);
}

#[test]
fn passive_listener_bosnded_history_drops_old_rows() {
    let mut state =
        LanPassiveDiscoveryListenerState::with_capacity("2026-06-25T00:00:00Z".to_string(), 2);

    assert_eq!(
        state.record_rescan_trigger(
            LanPassiveDiscoveryTriggerReason::AppResumed,
            "2026-06-25T00:00:01Z",
            "app resumed",
        ),
        LanPassiveDiscoveryRecordOutcome::Recorded
    );
    assert_eq!(
        state.record_rescan_trigger(
            LanPassiveDiscoveryTriggerReason::HeartbeatLost,
            "2026-06-25T00:00:02Z",
            "child heartbeat lost",
        ),
        LanPassiveDiscoveryRecordOutcome::Recorded
    );
    assert_eq!(
        state.record_passive_update(
            LanPassiveDiscoverySource::Arp,
            LanPassiveDiscoveryTriggerReason::IpAddressChanged,
            "2026-06-25T00:00:03Z",
            Some("device-2"),
            None,
            "arp update",
        ),
        LanPassiveDiscoveryRecordOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.max_rows, 2);
    assert_eq!(snapshot.dropped_row_count, 1);
    assert_eq!(snapshot.rows.len(), 2);
    assert_eq!(
        snapshot.rows[0].event_kind,
        LanPassiveDiscoveryEventKind::RescanTrigger
    );
    assert_eq!(
        snapshot.rows[1].event_kind,
        LanPassiveDiscoveryEventKind::PassiveUpdate
    );
    assert_eq!(
        snapshot.rows[1].previous_event_id,
        Some(snapshot.rows[0].event_id.clone())
    );
}

#[test]
fn stopped_listener_rejects_new_events() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    state.stop();

    assert_eq!(
        state.record_rescan_trigger(
            LanPassiveDiscoveryTriggerReason::InterfaceDown,
            "2026-06-25T00:00:01Z",
            "interface down",
        ),
        LanPassiveDiscoveryRecordOutcome::Stopped
    );
    assert_eq!(state.snapshot().rows.len(), 0);
    assert_eq!(
        state.lifecycle_state(),
        LanPassiveDiscoveryListenerLifecycleState::Stopped
    );
}

#[test]
fn sdp_packet_ingest_records_source_and_trigger_reason() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let packet = serde_json::json!({
        "schemaVersion": LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
        "source": "mdns",
        "triggerReason": "wifi-ssid-changed",
        "observedAt": "2026-06-25T00:00:01Z",
        "deviceId": "device-sdp-1",
        "scanSessionId": "scan-sdp-1",
        "summary": "mdns update"
    });
    let payload = serde_json::to_vec(&packet).value_or_unreachable("serialize packet");

    assert_eq!(
        state.ingest_udp_packet(&payload),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Mdns)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::WifiSsidChanged
    );
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("device-sdp-1"));
    assert_eq!(
        snapshot.rows[0].scan_session_id.as_deref(),
        Some("scan-sdp-1")
    );
    assert_eq!(snapshot.rows[0].summary, "mdns update");
}

#[test]
fn native_mdns_datagram_records_passive_observation_withost_json_envelope() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());

    assert_eq!(
        ingest_passive_datagram(
            &mut state,
            &LanPassiveDiscoverySource::Mdns,
            &mdns_packet_with_child_service(),
        ),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Mdns)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].summary,
        format!(
            "mDNS DNS-SD packet: 0 service type(s), 1 instance(s); first service={}; display=Kitchen Tablet",
            ocentra_parent_agent_protocol::constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
        )
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_deref(),
        Some("opaque-child-id")
    );
}

#[test]
fn native_ssdp_datagram_records_passive_observation_withost_json_envelope() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-25T00:00:00Z".to_string());
    let payload = b"NOTIFY * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nNT: urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\nNTS: ssdp:alive\r\nUSN: uuid:roster-1::urn:schemas-upnp-org:device:InternetGatewayDevice:1\r\nLOCATION: http://192.168.1.1/root.xml\r\n\r\n";

    assert_eq!(
        ingest_passive_datagram(&mut state, &LanPassiveDiscoverySource::Ssdp, payload),
        LanPassiveDiscoveryPacketIngestOutcome::Recorded
    );

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Ssdp)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "SSDP notify: nt=urn:schemas-upnp-org:device:InternetGatewayDevice:1; nts=ssdp:alive; usn=uuid:roster-1::urn:schemas-upnp-org:device:InternetGatewayDevice:1; location=http://192.168.1.1/root.xml"
            .to_string()
    );
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("roster-1"));
}

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
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("camera-1"));
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
        snapshot.rows[0].device_id.as_deref(),
        Some("kid-laptop.local")
    );
    let summary = snapshot.rows[0]
        .summary
        .strip_prefix("LLMNR packet: ")
        .value_or_unreachable("llmnr summary prefix");
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
    let first_seen_at = summary_parts[4]
        .split('=')
        .nth(1)
        .value_or_unreachable("first seen timestamp");
    let last_seen_at = summary_parts[5]
        .split('=')
        .nth(1)
        .value_or_unreachable("last seen timestamp");
    DateTime::parse_from_rfc3339(first_seen_at)
        .value_or_unreachable("first seen timestamp is RFC3339");
    DateTime::parse_from_rfc3339(last_seen_at)
        .value_or_unreachable("last seen timestamp is RFC3339");
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
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("kid-laptop"));
    let summary = snapshot.rows[0]
        .summary
        .strip_prefix("NetBIOS name packet: ")
        .value_or_unreachable("netbios summary prefix");
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
    let first_seen_at = summary_parts[4]
        .split('=')
        .nth(1)
        .value_or_unreachable("first seen timestamp");
    let last_seen_at = summary_parts[5]
        .split('=')
        .nth(1)
        .value_or_unreachable("last seen timestamp");
    DateTime::parse_from_rfc3339(first_seen_at)
        .value_or_unreachable("first seen timestamp is RFC3339");
    DateTime::parse_from_rfc3339(last_seen_at)
        .value_or_unreachable("last seen timestamp is RFC3339");
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
        snapshot.rows[0].device_id.as_deref(),
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
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("cam-1"));
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
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind receiver");
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable("set receiver timeout");

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind sender");
    let receiver_addr = receiver
        .local_addr()
        .value_or_unreachable("receiver address");
    let payload = allowed_snmp_response_payload("Linux camera controller", "cam-1");

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable("send allowed snmp packet");

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
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("cam-1"));
}

#[test]
fn explicit_allowed_snmp_socket_packets_keep_malformed_payloads_ost_of_history() {
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind receiver");
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable("set receiver timeout");

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind sender");
    let receiver_addr = receiver
        .local_addr()
        .value_or_unreachable("receiver address");
    let payload = [0x30, 0x03, 0x02, 0x01, 0x05, 0x00];

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable("send malformed snmp packet");

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
    .value_or_unreachable("serialize packet");

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
    .value_or_unreachable("serialize packet");

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
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind receiver");
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable("set receiver timeout");

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind sender");
    let receiver_addr = receiver
        .local_addr()
        .value_or_unreachable("receiver address");
    let payload = serde_json::to_vec(&serde_json::json!({
        "schemaVersion": LanPassiveDiscoveryListenerState::SCHEMA_VERSION,
        "source": "mdns",
        "triggerReason": "app-resumed",
        "observedAt": "2026-06-25T00:00:01Z",
        "deviceId": "device-sdp-transport",
        "scanSessionId": "scan-sdp-transport",
        "summary": "mdns multicast update"
    }))
    .value_or_unreachable("serialize packet");

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable("send sdp packet");

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
        snapshot.rows[0].device_id.as_deref(),
        Some("device-sdp-transport")
    );
    assert_eq!(
        snapshot.rows[0].scan_session_id.as_deref(),
        Some("scan-sdp-transport")
    );
    assert_eq!(snapshot.rows[0].summary, "mdns multicast update");
}

#[test]
fn sdp_broadcast_dhcp_packets_feed_the_existing_passive_state_path() {
    let receiver = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind receiver");
    receiver
        .set_read_timeout(Some(Duration::from_millis(250)))
        .value_or_unreachable("set receiver timeout");

    let sender = UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)).value_or_unreachable("bind sender");
    let receiver_addr = receiver
        .local_addr()
        .value_or_unreachable("receiver address");
    let payload = dhcp_packet(
        [0xde, 0xad, 0xbe, 0xef, 0x00, 0x01],
        "living-room-tv",
        "android-dhcp-14",
        &[1, 3, 6],
    );

    sender
        .send_to(&payload, receiver_addr)
        .value_or_unreachable("send dhcp packet");

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
        snapshot.rows[0].device_id.as_deref(),
        Some("de-ad-be-ef-00-01")
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "DHCP packet: type=discover; client-mac=de-ad-be-ef-00-01; client-id=ethernet:de-ad-be-ef-00-01; hostname=living-room-tv; vendor-class=android-dhcp-14; params=1,3,6"
    );
}
