use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

use chrono::DateTime;
use ocentra_lan_core::network_inventory::passive_discovery::udp_multicast::ingest_passive_datagram;
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
        format!(
            "Kitchen Tablet.{}",
            ocentra_parent_agent_protocol::constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
        ),
        &mut data,
    );
    packet.extend_from_slice(&(data.len() as u16).to_be_bytes());
    packet.extend_from_slice(&data);
    encode_dns_name(
        format!(
            "Kitchen Tablet.{}",
            ocentra_parent_agent_protocol::constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
        ),
        &mut packet,
    );
    packet.extend_from_slice(&16_u16.to_be_bytes());
    packet.extend_from_slice(&1_u16.to_be_bytes());
    packet.extend_from_slice(&60_u32.to_be_bytes());
    let txt = encode_txt_data([
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

fn encode_dns_name(name: impl std::fmt::Display, packet: &mut Vec<u8>) {
    let name = name.to_string();
    for label in name.split('.') {
        packet.push(label.len() as u8);
        packet.extend_from_slice(label.as_bytes());
    }
    packet.push(0);
}

fn encode_txt_data(entries: impl IntoIterator<Item = impl std::fmt::Display>) -> Vec<u8> {
    let mut data = Vec::new();
    for entry in entries {
        let entry = entry.to_string();
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

fn allowed_snmp_response_payload(
    sys_descr: impl std::fmt::Display,
    sys_name: impl std::fmt::Display,
) -> Vec<u8> {
    const BER_TAG_INTEGER: u8 = 0x02;
    const BER_TAG_OCTET_STRING: u8 = 0x04;
    const BER_TAG_OBJECT_IDENTIFIER: u8 = 0x06;
    const BER_TAG_SEQUENCE: u8 = 0x30;
    const SNMP_GET_RESPONSE_TAG: u8 = 0xA2;
    const SNMP_VERSION_2C: i64 = 1;
    const SNMP_REQUEST_ID: i64 = 1;
    const SNMP_SYS_DESCR_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 1, 0];
    const SNMP_SYS_NAME_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 5, 0];
    let sys_descr = sys_descr.to_string();
    let sys_name = sys_name.to_string();

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
    encode_dns_name(encoded_netbios_name(name), &mut packet);
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

#[path = "passive_discovery_protocols.rs"]
mod passive_discovery_protocols;

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
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("device-3")
    );
    assert_eq!(
        snapshot.rows[0].scan_session_id.as_ref().map(AsRef::as_ref),
        Some("scan-3")
    );
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
    let payload = serde_json::to_vec(&packet).value_or_unreachable();

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
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("device-sdp-1")
    );
    assert_eq!(
        snapshot.rows[0].scan_session_id.as_ref().map(AsRef::as_ref),
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
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
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
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("roster-1")
    );
}
