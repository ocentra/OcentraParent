use std::ffi::OsString;
use std::io;
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::Mutex;

use ocentra_lan_core::lan_mdns_advertiser::{encode_advertisement_packet, LanMdnsPacketSink};
use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason;
use ocentra_lan_core::network_inventory::LanPassiveRuntimeLocalNetworkIdentity;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::*;
use crate::app::lan_pairing_runtime_state::mdns_advertisement::LanMdnsAdvertisementSyncState;
use crate::app::lan_pairing_runtime_state::passive_discovery::{
    local_network_change_triggers, passive_discovery_udp_sources,
    LanPassiveDiscoveryRuntimeObservedState,
};
use crate::app::lan_pairing_runtime_state::provider_heartbeat::LanAiProviderHeartbeatState;
use crate::lan_pairing_test_commands::paired_runtime;
use crate::lan_runtime_test_support::{
    default_child_mdns_advertisement_fixture, LanChildMdnsAdvertisementFixture,
};
use crate::test_invariants::{require_ok, require_some};

static ENV_LOCK: Mutex<()> = Mutex::new(());
const CURRENT_HEARTBEAT_AT: &str = "2026-06-27T07:00:00.000Z";
const SECOND_HEARTBEAT_LOSS_AT: &str = "2026-06-27T07:05:00.000Z";

#[test]
fn from_env_defaults_to_local_json_registry_path() {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|_| unreachable!("lan runtime env lock remains available"));
    let previous_registry_path =
        std::env::var_os(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    let previous_child_device_id =
        std::env::var_os(constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV);
    std::env::remove_var(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    std::env::set_var(
        constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV,
        "Child Device 01",
    );

    let runtime = LanPairingRuntime::from_env();

    assert_eq!(
        runtime.persistence_mode(),
        constants::value::LAN_PERSISTENCE_LOCAL_JSON_REGISTRY
    );
    match &runtime.persistence {
        LanPairingRegistryPersistence::LocalJsonRegistry(path) => {
            assert_eq!(
                path,
                &std::env::temp_dir().join("ocentra-parent-lan-registry-child-device-01.json")
            );
        }
        LanPairingRegistryPersistence::InMemory => {
            unreachable!("from_env now defaults to local json persistence")
        }
    }

    restore_env_var(
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        previous_registry_path,
    );
    restore_env_var(
        constants::lan_pairing::LOCAL_CHILD_DEVICE_ID_ENV,
        previous_child_device_id,
    );
}

#[test]
fn from_env_respects_explicit_registry_path_override() {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|_| unreachable!("lan runtime env lock remains available"));
    let previous_registry_path =
        std::env::var_os(constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH);
    let explicit_path = std::env::temp_dir().join("ocentra-parent-lan-registry-override.json");
    std::env::set_var(
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        explicit_path.as_os_str(),
    );

    let runtime = LanPairingRuntime::from_env();

    match &runtime.persistence {
        LanPairingRegistryPersistence::LocalJsonRegistry(path) => {
            assert_eq!(path, &explicit_path);
        }
        LanPairingRegistryPersistence::InMemory => {
            unreachable!("explicit env path keeps local json persistence")
        }
    }

    restore_env_var(
        constants::env_var::AGENT_LAN_PAIRING_REGISTRY_PATH,
        previous_registry_path,
    );
}

fn restore_env_var(name: &str, value: Option<OsString>) {
    match value {
        Some(value) => std::env::set_var(name, value),
        None => std::env::remove_var(name),
    }
}

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
    let mut listener_state = runtime
        .passive_discovery_listener_state
        .lock()
        .unwrap_or_else(|_| unreachable!("passive discovery state lock remains available"));
    let mut observed_state = LanPassiveDiscoveryRuntimeObservedState::default();

    *runtime
        .lan_ai_provider_heartbeat
        .lock()
        .unwrap_or_else(|_| unreachable!("heartbeat state lock remains available")) =
        Some(LanAiProviderHeartbeatState {
            observed_at: constants::lan_pairing::EXPIRED_AT.to_string(),
            reachability: LanPairingDeviceReachability::Stale,
        });
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

    *runtime
        .lan_ai_provider_heartbeat
        .lock()
        .unwrap_or_else(|_| unreachable!("heartbeat state lock remains available")) =
        Some(LanAiProviderHeartbeatState {
            observed_at: CURRENT_HEARTBEAT_AT.to_string(),
            reachability: LanPairingDeviceReachability::Online,
        });
    runtime.record_heartbeat_loss_trigger_if_needed(
        &mut listener_state,
        &mut observed_state,
        CURRENT_HEARTBEAT_AT,
    );

    *runtime
        .lan_ai_provider_heartbeat
        .lock()
        .unwrap_or_else(|_| unreachable!("heartbeat state lock remains available")) =
        Some(LanAiProviderHeartbeatState {
            observed_at: SECOND_HEARTBEAT_LOSS_AT.to_string(),
            reachability: LanPairingDeviceReachability::Offline,
        });
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
    let mut listener_state = runtime
        .passive_discovery_listener_state
        .lock()
        .unwrap_or_else(|_| unreachable!("passive discovery state lock remains available"));
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

    let snapshot = listener_state.snapshot();
    let reasons = snapshot
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
    runtime
        .passive_discovery_listener_state
        .lock()
        .unwrap_or_else(|_| unreachable!("passive discovery state lock remains available"))
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

    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource::AllowedSnmpResponse)
    );
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("cam-1"));
    assert_eq!(
        snapshot.rows[0].summary,
        "allowed SNMP response: sys-name=cam-1; sys-descr=Linux camera controller"
    );
}

#[test]
fn stopped_passive_runtime_ignores_allowed_snmp_probe_responses() {
    let runtime = LanPairingRuntime::empty();
    runtime
        .passive_discovery_listener_state
        .lock()
        .unwrap_or_else(|_| unreachable!("passive discovery state lock remains available"))
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

    let snapshot = runtime.passive_discovery_history_snapshot();
    assert_eq!(snapshot.rows.len(), 1);
    assert_eq!(
        snapshot.rows[0].source,
        Some(ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoverySource::AllowedSnmpResponse)
    );
    assert_eq!(snapshot.rows[0].device_id.as_deref(), Some("cam-1"));
    assert_eq!(
        snapshot.rows[0].summary,
        "allowed SNMP response: sys-name=cam-1; sys-descr=Linux camera controller"
    );
}

fn identity_snapshot(
    ip_address: &str,
    network_interface: &str,
    wifi_ssid: &str,
    default_gateway: &str,
) -> LanPassiveRuntimeLocalNetworkIdentity {
    LanPassiveRuntimeLocalNetworkIdentity {
        ip_address: Some(ip_address.to_string()),
        network_interface: Some(network_interface.to_string()),
        wifi_ssid: if wifi_ssid.is_empty() {
            None
        } else {
            Some(wifi_ssid.to_string())
        },
        default_gateway: if default_gateway.is_empty() {
            None
        } else {
            Some(default_gateway.to_string())
        },
    }
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

    fn encode_octet_string(value: &str) -> Vec<u8> {
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

    fn encode_varbind(oid: &[u32], value: &str) -> Vec<u8> {
        encode_sequence(
            BER_TAG_SEQUENCE,
            vec![encode_oid(oid), encode_octet_string(value)],
        )
    }

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

#[tokio::test]
async fn lan_pairing_runtime_builds_hint_only_parent_and_child_mdns_advertisements() {
    let runtime = paired_runtime().await;
    let parent = require_ok(
        runtime.parent_mdns_advertisement(
            "sha256:parent-family-1",
            constants::lan_pairing::SCHEMA_VERSION_TEXT,
            "sha256:family-1",
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Start,
            ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Supported,
        ),
        "parent advertisement",
    );
    let child = require_ok(
        runtime.child_mdns_advertisement(LanChildMdnsAdvertisementFixture {
            opaque_device_id: "opaque-child-id",
            ..default_child_mdns_advertisement_fixture(
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementLifecycleState::Update,
                ocentra_parent_agent_protocol::lan_pairing::LanMdnsAdvertisementSupportState::Degraded,
            )
        }),
        "child advertisement",
    );

    assert_eq!(
        parent.service_type,
        constants::lan_pairing::MDNS_PARENT_SERVICE_TYPE
    );
    assert_eq!(
        child.service_type,
        constants::lan_pairing::MDNS_CHILD_SERVICE_TYPE
    );
    assert_eq!(
        parent.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        child.confirmation_state.as_str(),
        constants::lan_pairing::MDNS_TXT_VALUE_HINT_ONLY
    );
    assert_eq!(
        parent.protocol_version,
        constants::lan_pairing::SCHEMA_VERSION_TEXT
    );
    assert_eq!(
        child.protocol_version,
        constants::lan_pairing::SCHEMA_VERSION_TEXT
    );
    assert_eq!(parent.family_hash, "sha256:family-1");
    assert_eq!(child.family_hash, "sha256:family-1");
    assert_eq!(child.opaque_device_id, "opaque-child-id");
    assert_eq!(child.platform, constants::lan_pairing::PLATFORM_WINDOWS);
    assert_eq!(child.agent_version, "1.2.3");
    assert_eq!(parent.txt_records.len(), 7);
    assert_eq!(child.txt_records.len(), 10);
    assert!(parent
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
    assert!(child
        .txt_records
        .iter()
        .all(|record| !record.value.contains(' ') && !record.value.contains('@')));
}

#[tokio::test]
async fn lan_pairing_runtime_syncs_mdns_advertisements_and_retracts_on_degraded_platform() {
    let mut runtime = paired_runtime().await;
    runtime.local_child_device_id = Some("opaque-child-id".to_string());
    runtime.signed_child_agent_parent_device_id =
        Some(constants::lan_pairing::PARENT_DEVICE_ID.to_string());
    runtime.signed_child_agent_family_hash = Some("sha256:family-1".to_string());
    runtime.signed_child_agent_route_id =
        constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string();
    let sink = RecordingMdnsSink::default();
    let mut sync_state = LanMdnsAdvertisementSyncState::default();

    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "initial sync succeeds",
    );
    let first_parent = require_some(sync_state.parent.clone(), "parent instance");
    let first_child = require_some(sync_state.child.clone(), "child instance");
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Supported,
            &sink,
        ),
        "update sync succeeds",
    );
    let second_parent = require_some(sync_state.parent.clone(), "parent instance");
    let second_child = require_some(sync_state.child.clone(), "child instance");
    require_ok(
        runtime.sync_mdns_advertisements_with_sink(
            &mut sync_state,
            ocentra_lan_core::lan_pairing::LanMdnsAdvertisementPlatformSupport::Degraded,
            &sink,
        ),
        "degraded sync succeeds",
    );

    assert_eq!(
        sink.packets(),
        vec![
            encode_advertisement_packet(std::slice::from_ref(&first_parent), 120),
            encode_advertisement_packet(std::slice::from_ref(&first_child), 120),
            encode_advertisement_packet(std::slice::from_ref(&second_parent), 120),
            encode_advertisement_packet(std::slice::from_ref(&second_child), 120),
            encode_advertisement_packet(std::slice::from_ref(&second_parent), 0),
            encode_advertisement_packet(std::slice::from_ref(&second_child), 0),
        ]
    );
    assert!(sync_state.parent.is_none());
    assert!(sync_state.child.is_none());
}

#[derive(Default)]
struct RecordingMdnsSink {
    packets: Mutex<Vec<Vec<u8>>>,
}

impl RecordingMdnsSink {
    fn packets(&self) -> Vec<Vec<u8>> {
        require_ok(self.packets.lock(), "packets").clone()
    }
}

impl LanMdnsPacketSink for RecordingMdnsSink {
    fn send(&self, packet: &[u8]) -> io::Result<()> {
        require_ok(self.packets.lock(), "packets").push(packet.to_vec());
        Ok(())
    }
}
