use std::net::{Ipv4Addr, UdpSocket};
use std::string::String as TestString;

use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::*;
use crate::app::lan_pairing_runtime_state::passive_discovery::{
    local_network_change_triggers, passive_discovery_udp_sources,
    LanPassiveDiscoveryRuntimeObservedState,
};
use crate::app::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;
use crate::test_invariants::require_ok;

#[path = "../../../src/lan_pairing_runtime_state/passive_discovery/listener_runtime/cycle_cursor.rs"]
mod cycle_cursor_helper;

#[test]
fn passive_cycle_cursor_rotates_all_six_listeners_fairly() {
    let mut cursor = cycle_cursor_helper::PassiveDiscoveryCycleCursor::new(0, 6);
    for expected in 0..6 {
        assert_eq!(cursor.take_next(), Some(expected));
    }
    assert_eq!(cursor.take_next(), Some(0));
    let mut offset = cycle_cursor_helper::PassiveDiscoveryCycleCursor::new(4, 6);
    let sequence = (0..6).map(|_| offset.take_next()).collect::<Vec<_>>();
    assert_eq!(
        sequence,
        vec![Some(4), Some(5), Some(0), Some(1), Some(2), Some(3)]
    );
    let mut partial = cycle_cursor_helper::PassiveDiscoveryCycleCursor::new(2, 6);
    assert_eq!(partial.take_next(), Some(2));
    assert_eq!(partial.resume_index(), 3);
    let mut empty = cycle_cursor_helper::PassiveDiscoveryCycleCursor::new(0, 0);
    assert_eq!(empty.take_next(), None);
    assert_eq!(empty.resume_index(), 0);
}

#[test]
fn passive_cycle_cursor_honors_shared_budget_and_cancellation() {
    assert!(cycle_cursor_helper::PassiveDiscoveryCycleCursor::should_continue(
        true,
        0,
        6,
        std::time::Duration::from_millis(1),
    ));
    assert!(!cycle_cursor_helper::PassiveDiscoveryCycleCursor::should_continue(
        true,
        6,
        6,
        std::time::Duration::from_millis(1),
    ));
    assert!(!cycle_cursor_helper::PassiveDiscoveryCycleCursor::should_continue(
        true,
        0,
        6,
        std::time::Duration::ZERO,
    ));
    assert!(!cycle_cursor_helper::PassiveDiscoveryCycleCursor::should_continue(
        false,
        0,
        6,
        std::time::Duration::from_millis(1),
    ));
}

const CURRENT_HEARTBEAT_AT: &str = "2026-06-27T07:00:00.000Z";
const SECOND_HEARTBEAT_LOSS_AT: &str = "2026-06-27T07:05:00.000Z";

const BER_TAG_INTEGER: u8 = 0x02;
const BER_TAG_OCTET_STRING: u8 = 0x04;
const BER_TAG_OBJECT_IDENTIFIER: u8 = 0x06;
const BER_TAG_SEQUENCE: u8 = 0x30;
const SNMP_GET_RESPONSE_TAG: u8 = 0xA2;
const SNMP_VERSION_2C: i64 = 1;
const SNMP_REQUEST_ID: i64 = 1;
const SNMP_SYS_DESCR_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 1, 0];
const SNMP_SYS_NAME_OID: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 5, 0];

#[test]
fn passive_discovery_snapshot_starts_running_and_empty() {
    let runtime = LanPairingRuntime::empty();

    let snapshot = runtime.passive_discovery_history_snapshot();

    assert_eq!(snapshot.schema_version, 1);
    assert_eq!(snapshot.rows.len(), 0);
    assert_eq!(snapshot.max_rows, 128);
}

#[test]
fn passive_discovery_records_app_resumed_trigger() {
    let runtime = LanPairingRuntime::empty();

    runtime.record_passive_rescan_trigger(
        LanPassiveDiscoveryTriggerReason::AppResumed,
        "passive discovery runtime started",
    );

    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::AppResumed
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "passive discovery runtime started"
    );
}

#[test]
fn passive_discovery_records_heartbeat_loss_once_until_recovery() {
    let runtime = LanPairingRuntime::empty();
    let mut listener_state = require_ok(
        runtime.passive_discovery_listener_state.lock(),
        "passive discovery state lock remains available",
    );
    let mut observed_state = LanPassiveDiscoveryRuntimeObservedState::default();

    set_heartbeat_state(
        &runtime,
        constants::lan_pairing::EXPIRED_AT,
        LanPairingDeviceReachability::Stale,
    );
    runtime.record_heartbeat_loss_trigger_if_needed(
        &mut listener_state,
        &mut observed_state,
        constants::lan_pairing::EXPIRED_AT,
    );
    runtime.record_heartbeat_loss_trigger_if_needed(
        &mut listener_state,
        &mut observed_state,
        constants::lan_pairing::EXPIRED_AT,
    );

    set_heartbeat_state(
        &runtime,
        CURRENT_HEARTBEAT_AT,
        LanPairingDeviceReachability::Online,
    );
    runtime.record_heartbeat_loss_trigger_if_needed(
        &mut listener_state,
        &mut observed_state,
        CURRENT_HEARTBEAT_AT,
    );

    set_heartbeat_state(
        &runtime,
        SECOND_HEARTBEAT_LOSS_AT,
        LanPairingDeviceReachability::Offline,
    );
    runtime.record_heartbeat_loss_trigger_if_needed(
        &mut listener_state,
        &mut observed_state,
        SECOND_HEARTBEAT_LOSS_AT,
    );

    let snapshot = listener_state.snapshot();
    let heartbeat_loss_rows = snapshot
        .rows
        .iter()
        .filter(|row| row.trigger_reason == LanPassiveDiscoveryTriggerReason::HeartbeatLost)
        .count();
    assert_eq!(heartbeat_loss_rows, 2);
}

#[test]
fn first_local_network_identity_snapshot_does_not_emit_change_triggers() {
    assert!(local_network_change_triggers(
        None,
        &identity_snapshot("192.168.2.10", "Wi-Fi", "Home-Wifi", "192.168.2.1")
    )
    .is_empty());
}

#[test]
fn local_network_identity_change_emits_interface_ip_and_gateway_rescan_triggers() {
    let previous = identity_snapshot("192.168.2.10", "Wi-Fi", "Home-Wifi", "192.168.2.1");
    let current = identity_snapshot("192.168.2.20", "Ethernet", "", "192.168.2.254");

    let triggers = local_network_change_triggers(Some(&previous), &current);

    assert_eq!(triggers.len(), 4);
    assert_eq!(
        triggers[0].reason,
        LanPassiveDiscoveryTriggerReason::InterfaceDown
    );
    assert_eq!(triggers[0].summary, "network interface down: Wi-Fi");
    assert_eq!(
        triggers[1].reason,
        LanPassiveDiscoveryTriggerReason::InterfaceUp
    );
    assert_eq!(triggers[1].summary, "network interface up: Ethernet");
    assert_eq!(
        triggers[2].reason,
        LanPassiveDiscoveryTriggerReason::IpAddressChanged
    );
    assert_eq!(
        triggers[2].summary,
        "ip address changed: 192.168.2.10 -> 192.168.2.20"
    );
    assert_eq!(
        triggers[3].reason,
        LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged
    );
    assert_eq!(
        triggers[3].summary,
        "default gateway changed: 192.168.2.1 -> 192.168.2.254"
    );
}

#[test]
fn local_network_identity_change_emits_wifi_ssid_rescan_trigger_on_same_interface() {
    let previous = identity_snapshot("192.168.2.10", "Wi-Fi", "Home-Wifi", "192.168.2.1");
    let current = identity_snapshot("192.168.2.10", "Wi-Fi", "Guest-Wifi", "192.168.2.1");

    let triggers = local_network_change_triggers(Some(&previous), &current);

    assert_eq!(triggers.len(), 1);
    assert_eq!(
        triggers[0].reason,
        LanPassiveDiscoveryTriggerReason::WifiSsidChanged
    );
    assert_eq!(
        triggers[0].summary,
        "wifi ssid changed: Home-Wifi -> Guest-Wifi"
    );
}

#[test]
fn passive_runtime_records_local_network_change_triggers_once_snapshot_exists() {
    let runtime = LanPairingRuntime::empty();
    let mut listener_state = require_ok(
        runtime.passive_discovery_listener_state.lock(),
        "passive discovery state lock remains available",
    );
    let mut observed_state = LanPassiveDiscoveryRuntimeObservedState::default();

    runtime.record_local_network_change_triggers_if_needed(
        &mut listener_state,
        &mut observed_state,
        "2026-06-27T07:10:00.000Z",
        &identity_snapshot("192.168.2.10", "Wi-Fi", "Home-Wifi", "192.168.2.1"),
    );
    runtime.record_local_network_change_triggers_if_needed(
        &mut listener_state,
        &mut observed_state,
        "2026-06-27T07:11:00.000Z",
        &identity_snapshot("192.168.2.20", "Ethernet", "", "192.168.2.254"),
    );

    let reasons = listener_state
        .snapshot()
        .rows
        .iter()
        .map(|row| row.trigger_reason.clone())
        .collect::<Vec<_>>();

    assert_eq!(
        reasons,
        vec![
            LanPassiveDiscoveryTriggerReason::InterfaceDown,
            LanPassiveDiscoveryTriggerReason::InterfaceUp,
            LanPassiveDiscoveryTriggerReason::IpAddressChanged,
            LanPassiveDiscoveryTriggerReason::DefaultGatewayChanged,
        ]
    );
}

#[test]
fn stopped_passive_listener_halts_runtime_collection_without_recording_rows() {
    let runtime = LanPairingRuntime::empty();
    require_ok(
        runtime.passive_discovery_listener_state.lock(),
        "passive discovery state lock remains available",
    )
    .stop();
    let mut observed_state = LanPassiveDiscoveryRuntimeObservedState::default();

    assert!(!runtime.collect_passive_discovery_runtime_slice(&mut observed_state));

    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 0);
    assert_eq!(
        snapshot.lifecycle_state,
        ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerLifecycleState::Stopped
    );
}

#[test]
fn passive_runtime_udp_sources_include_dhcp_before_multicast_hint_sources() {
    assert_eq!(
        passive_discovery_udp_sources(),
        &[
            LanPassiveDiscoverySource::Dhcp,
            LanPassiveDiscoverySource::Mdns,
            LanPassiveDiscoverySource::Ssdp,
            LanPassiveDiscoverySource::WsDiscovery,
            LanPassiveDiscoverySource::Llmnr,
            LanPassiveDiscoverySource::Netbios,
        ]
    );
}

#[test]
fn passive_runtime_records_allowed_snmp_probe_responses_into_history() {
    let runtime = LanPairingRuntime::empty();
    let receiver = require_ok(UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)), "bind receiver");
    require_ok(
        receiver.set_read_timeout(Some(std::time::Duration::from_millis(250))),
        "set receiver timeout",
    );
    let sender = require_ok(UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)), "bind sender");
    let receiver_addr = require_ok(receiver.local_addr(), "receiver address");

    require_ok(
        sender.send_to(
            &allowed_snmp_response_payload("Linux camera controller", "cam-1"),
            receiver_addr,
        ),
        "send snmp response",
    );

    assert_eq!(runtime.record_allowed_snmp_probe_responses(&receiver, 1), 1);

    assert_allowed_snmp_row(&runtime);
}

#[test]
fn stopped_passive_runtime_ignores_allowed_snmp_probe_responses() {
    let runtime = LanPairingRuntime::empty();
    require_ok(
        runtime.passive_discovery_listener_state.lock(),
        "passive discovery state lock remains available",
    )
    .stop();
    let receiver = require_ok(UdpSocket::bind((Ipv4Addr::LOCALHOST, 0)), "bind receiver");
    require_ok(
        receiver.set_read_timeout(Some(std::time::Duration::from_millis(250))),
        "set receiver timeout",
    );

    assert_eq!(runtime.record_allowed_snmp_probe_responses(&receiver, 1), 0);
    assert!(runtime.passive_discovery_history_snapshot().rows.is_empty());
}

#[test]
fn passive_runtime_records_allowed_snmp_probe_response_packet_into_history() {
    let runtime = LanPairingRuntime::empty();

    assert!(
        runtime.record_allowed_snmp_probe_response_packet(&allowed_snmp_response_payload(
            "Linux camera controller",
            "cam-1",
        ))
    );

    assert_allowed_snmp_row(&runtime);
}

fn set_heartbeat_state(
    runtime: &LanPairingRuntime,
    observed_at: impl Into<TestString>,
    reachability: LanPairingDeviceReachability,
) {
    let observed_at = observed_at.into();
    *require_ok(
        runtime.lan_ai_provider_heartbeat.lock(),
        "heartbeat state lock remains available",
    ) = Some(LanAiProviderHeartbeatState {
        observed_at,
        reachability,
    });
}

fn assert_allowed_snmp_row(runtime: &LanPairingRuntime) {
    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource::AllowedSnmpResponse)
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(|id| id.as_str()),
        Some("cam-1")
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "allowed SNMP response: sys-name=cam-1; sys-descr=Linux camera controller"
    );
}

fn identity_snapshot(
    ip_address: impl Into<TestString>,
    network_interface: impl Into<TestString>,
    wifi_ssid: impl Into<TestString>,
    default_gateway: impl Into<TestString>,
) -> LanPassiveRuntimeLocalNetworkIdentity {
    let ip_address = ip_address.into();
    let network_interface = network_interface.into();
    let wifi_ssid = wifi_ssid.into();
    let default_gateway = default_gateway.into();
    LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some(ip_address),
        network_interface: Some(network_interface),
        wifi_ssid: if wifi_ssid.is_empty() {
            None
        } else {
            Some(wifi_ssid)
        },
        default_gateway: if default_gateway.is_empty() {
            None
        } else {
            Some(default_gateway)
        },
    }
}

fn allowed_snmp_response_payload(
    sys_descr: impl Into<TestString>,
    sys_name: impl Into<TestString>,
) -> Vec<u8> {
    let sys_descr = sys_descr;
    let sys_name = sys_name;
    let varbind_list = encode_sequence(
        BER_TAG_SEQUENCE,
        vec![
            encode_varbind(SNMP_SYS_DESCR_OID, sys_descr),
            encode_varbind(SNMP_SYS_NAME_OID, sys_name),
        ],
    );
    let response = encode_sequence(
        SNMP_GET_RESPONSE_TAG,
        vec![
            encode_integer(SNMP_REQUEST_ID),
            encode_integer(0),
            encode_integer(0),
            varbind_list,
        ],
    );
    encode_sequence(
        BER_TAG_SEQUENCE,
        vec![
            encode_integer(SNMP_VERSION_2C),
            encode_octet_string("public"),
            response,
        ],
    )
}

fn encode_length(length: usize) -> Vec<u8> {
    if length < 0x80 {
        return vec![length as u8];
    }

    let mut encoded = Vec::new();
    let mut remaining = length;
    while remaining > 0 {
        encoded.push((remaining & 0xff) as u8);
        remaining >>= 8;
    }
    encoded.reverse();

    let mut result = vec![0x80 | encoded.len() as u8];
    result.extend(encoded);
    result
}

fn encode_integer(value: i64) -> Vec<u8> {
    let mut bytes = value.to_be_bytes().to_vec();
    while bytes.len() > 1
        && ((bytes[0] == 0x00 && bytes[1] & 0x80 == 0)
            || (bytes[0] == 0xff && bytes[1] & 0x80 == 0x80))
    {
        bytes.remove(0);
    }

    let mut encoded = vec![BER_TAG_INTEGER];
    encoded.extend(encode_length(bytes.len()));
    encoded.extend(bytes);
    encoded
}

fn encode_octet_string(value: impl Into<TestString>) -> Vec<u8> {
    let value = value.into();
    let bytes = value.as_bytes();
    let mut encoded = vec![BER_TAG_OCTET_STRING];
    encoded.extend(encode_length(bytes.len()));
    encoded.extend(bytes);
    encoded
}

fn encode_base128(value: u32) -> Vec<u8> {
    let mut encoded = vec![(value & 0x7f) as u8];
    let mut remaining = value >> 7;
    while remaining > 0 {
        encoded.push(((remaining & 0x7f) as u8) | 0x80);
        remaining >>= 7;
    }
    encoded.reverse();
    encoded
}

fn encode_oid(oid: &[u32]) -> Vec<u8> {
    let mut body = vec![(oid[0] * 40 + oid[1]) as u8];
    for component in &oid[2..] {
        body.extend(encode_base128(*component));
    }
    let mut encoded = vec![BER_TAG_OBJECT_IDENTIFIER];
    encoded.extend(encode_length(body.len()));
    encoded.extend(body);
    encoded
}

fn encode_sequence(tag: u8, children: Vec<Vec<u8>>) -> Vec<u8> {
    let body = children.into_iter().flatten().collect::<Vec<_>>();
    let mut encoded = vec![tag];
    encoded.extend(encode_length(body.len()));
    encoded.extend(body);
    encoded
}

fn encode_varbind(oid: &[u32], value: impl Into<TestString>) -> Vec<u8> {
    encode_sequence(
        BER_TAG_SEQUENCE,
        vec![encode_oid(oid), encode_octet_string(value)],
    )
}
