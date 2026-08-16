use crate::support::{OptionTestExt as _, ResultTestExt as _};
use ocentra_lan_core::network_inventory::api::{
    discovered_devices_from_network_inventory,
    plan_lan_discovery_scan_with_manual_interface_selection,
};
use ocentra_lan_core::network_inventory::passive_discovery::collection::{
    local_neighbor_collection_support_for_platform,
    record_local_neighbor_passive_updates_from_observations,
};
use ocentra_lan_core::network_inventory::passive_discovery::{
    LanPassiveDiscoveryListenerState, LanPassiveDiscoveryLocalNeighborSource,
    LanPassiveDiscoverySource, LanPassiveDiscoveryTriggerReason,
};
use ocentra_lan_core::network_inventory::{
    LanDiscoveryRefreshMode, LanManualInterfaceSelection, LanNetworkInventoryDevice,
    LanPreviousNetworkInventory,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

pub(crate) fn neighbor_record(hostname: Option<impl Into<Value>>) -> Value {
    neighbor_record_with_values(
        constants::lan_pairing::TEST_LAN_IP,
        constants::lan_pairing::TEST_LAN_MAC,
        constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE,
        None::<Value>,
        hostname.map(Into::into),
    )
}

pub(crate) fn neighbor_record_with_values(
    ip_address: impl Into<Value>,
    mac_address: impl Into<Value>,
    state: impl Into<Value>,
    interface_alias: Option<impl Into<Value>>,
    hostname: Option<impl Into<Value>>,
) -> Value {
    let ip_address = ip_address.into();
    let mac_address = mac_address.into();
    let state = state.into();
    let mut record = Map::new();
    record.insert(
        constants::lan_pairing::JSON_KEY_IP_ADDRESS.to_string(),
        Value::String(ip_address.as_str().unwrap_or_default().to_owned()),
    );
    record.insert(
        constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS.to_string(),
        Value::String(mac_address.as_str().unwrap_or_default().to_owned()),
    );
    record.insert(
        constants::lan_pairing::JSON_KEY_STATE.to_string(),
        Value::String(state.as_str().unwrap_or_default().to_owned()),
    );
    if let Some(interface_alias) = interface_alias {
        record.insert(
            constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS.to_string(),
            Value::String(
                interface_alias
                    .into()
                    .as_str()
                    .unwrap_or_default()
                    .to_owned(),
            ),
        );
    }
    if let Some(hostname) = hostname {
        record.insert(
            constants::lan_pairing::JSON_KEY_HOSTNAME.to_string(),
            Value::String(hostname.into().as_str().unwrap_or_default().to_owned()),
        );
    }
    Value::Object(record)
}

pub(crate) fn trusted_device(
    mac_address: impl Into<Value>,
    ip_address: Option<impl Into<Value>>,
    hostname: Option<impl Into<Value>>,
    label: impl Into<Value>,
    platform: impl Into<Value>,
) -> LanPairingDeviceRef {
    let mac_address = mac_address.into().as_str().unwrap_or_default().to_owned();
    let label = label.into().as_str().unwrap_or_default().to_owned();
    let platform = platform.into().as_str().unwrap_or_default().to_owned();
    let mut device = LanPairingDeviceRef::new("trusted-child".to_string(), None, label, platform);
    if !mac_address.is_empty() {
        device.mac_address = Some(mac_address);
    }
    device.ip_address = ip_address
        .map(Into::into)
        .map(|value: Value| value.as_str().unwrap_or_default().to_owned());
    device.hostname = hostname
        .map(Into::into)
        .map(|value: Value| value.as_str().unwrap_or_default().to_owned());
    device
}

fn optional_probe_env_lock() -> std::sync::MutexGuard<'static, ()> {
    static OPTIONAL_PROBE_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    OPTIONAL_PROBE_ENV_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .value_or_unreachable()
}

#[test]
fn manual_interface_selection_overrides_scan_plan_identity() {
    let plan = plan_lan_discovery_scan_with_manual_interface_selection(
        &[],
        &[],
        LanDiscoveryRefreshMode::ActiveSubnetRefresh,
        &[],
        Some(LanManualInterfaceSelection {
            selected_interface: " en7 ".to_string(),
            local_ip_address: " 192.168.50.10 ".to_string(),
            ipv4_cidr: " 192.168.50.10/24 ".to_string(),
            default_gateway: Some(" 192.168.50.1 ".to_string()),
            dns_servers: vec![" 192.168.50.1 ".to_string(), " ".to_string()],
            dhcp_server: Some(" 192.168.50.1 ".to_string()),
            broadcast_address: Some(" 192.168.50.255 ".to_string()),
            ipv6_prefixes: vec![" fd00::10/64 ".to_string()],
        }),
    );

    assert_eq!(plan.selected_interface.as_deref(), Some("en7"));
    assert_eq!(plan.local_ip_address.as_deref(), Some("192.168.50.10"));
    assert_eq!(plan.ipv4_cidr.as_deref(), Some("192.168.50.10/24"));
    assert_eq!(plan.default_gateway.as_deref(), Some("192.168.50.1"));
    assert_eq!(plan.dns_servers, vec!["192.168.50.1".to_string()]);
    assert_eq!(plan.dhcp_server.as_deref(), Some("192.168.50.1"));
    assert_eq!(plan.broadcast_address.as_deref(), Some("192.168.50.255"));
    assert_eq!(plan.ipv6_prefixes, vec!["fd00::10/64".to_string()]);
    assert_eq!(plan.active_ipv4_candidate_count, 253);
}

#[test]
fn manual_interface_selection_trims_into_identity_before_planning() {
    let identity = LanManualInterfaceSelection {
        selected_interface: " en7 ".to_string(),
        local_ip_address: " 192.168.50.10 ".to_string(),
        ipv4_cidr: " 192.168.50.10/24 ".to_string(),
        default_gateway: Some(" 192.168.50.1 ".to_string()),
        dns_servers: vec![" 192.168.50.1 ".to_string(), " ".to_string()],
        dhcp_server: Some(" 192.168.50.1 ".to_string()),
        broadcast_address: Some(" 192.168.50.255 ".to_string()),
        ipv6_prefixes: vec![" fd00::10/64 ".to_string()],
    }
    .into_identity()
    .value_or_unreachable();

    assert_eq!(identity.network_interface.as_deref(), Some("en7"));
    assert_eq!(identity.ip_address.as_deref(), Some("192.168.50.10"));
    assert_eq!(identity.ipv4_cidr.as_deref(), Some("192.168.50.10/24"));
    assert_eq!(identity.default_gateway.as_deref(), Some("192.168.50.1"));
    assert_eq!(identity.dns_servers, vec!["192.168.50.1".to_string()]);
    assert_eq!(identity.dhcp_server.as_deref(), Some("192.168.50.1"));
    assert_eq!(
        identity.broadcast_address.as_deref(),
        Some("192.168.50.255")
    );
    assert_eq!(identity.ipv6_prefixes, vec!["fd00::10/64".to_string()]);
}

#[test]
fn scan_plan_carries_runtime_optional_query_settings() {
    let _guard = optional_probe_env_lock();
    let previous_wsd = std::env::var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV).ok();
    let previous_snmp =
        std::env::var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV).ok();
    std::env::set_var(
        constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
        "true",
    );
    std::env::set_var(
        constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
        "yes",
    );

    let plan = plan_lan_discovery_scan_with_manual_interface_selection(
        &[],
        &[],
        LanDiscoveryRefreshMode::Passive,
        &[],
        Some(LanManualInterfaceSelection {
            selected_interface: "Wi-Fi".to_string(),
            local_ip_address: "192.168.50.10".to_string(),
            ipv4_cidr: "192.168.50.10/24".to_string(),
            default_gateway: Some("192.168.50.1".to_string()),
            dns_servers: vec!["192.168.50.1".to_string()],
            dhcp_server: Some("192.168.50.1".to_string()),
            broadcast_address: Some("192.168.50.255".to_string()),
            ipv6_prefixes: vec!["fd00::10/64".to_string()],
        }),
    );

    assert!(plan.allow_wsd_identity_query);
    assert!(plan.allow_snmp_identity_query);
    assert!(!plan.allow_os_fingerprint);

    match previous_wsd.as_deref() {
        Some(value) => std::env::set_var(
            constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV,
            value,
        ),
        None => std::env::remove_var(constants::lan_pairing::LAN_ALLOW_WSD_IDENTITY_QUERY_ENV),
    }
    match previous_snmp.as_deref() {
        Some(value) => std::env::set_var(
            constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV,
            value,
        ),
        None => std::env::remove_var(constants::lan_pairing::LAN_ALLOW_SNMP_IDENTITY_QUERY_ENV),
    }
}

#[test]
fn local_neighbor_collection_support_reports_explicit_platform_blockers() {
    let blocked = local_neighbor_collection_support_for_platform(
        LanPassiveDiscoveryLocalNeighborSource::WindowsNeighborTable,
        "linux",
    )
    .error_or_unreachable();
    assert_eq!(
        blocked,
        "windows-neighbor-table passive collection is only available on windows; current platform is linux"
    );

    let allowed = local_neighbor_collection_support_for_platform(
        LanPassiveDiscoveryLocalNeighborSource::LinuxIpNeigh,
        "linux",
    )
    .value_or_unreachable();
    assert_eq!(
        allowed,
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH
    );
}

#[test]
fn local_neighbor_observations_record_weak_hints_with_source_labels() {
    let mut state = LanPassiveDiscoveryListenerState::running("2026-06-26T00:00:00Z".to_string());
    let observations = HashMap::from([
        ("192.168.50.10".to_string(), "11:22:33:44:55:66".to_string()),
        (
            "192.168.50.200".to_string(),
            "aa:bb:cc:dd:ee:ff".to_string(),
        ),
    ]);

    let (observed_count, recorded_count) = record_local_neighbor_passive_updates_from_observations(
        &mut state,
        constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH,
        observations,
    );

    assert_eq!(observed_count, 2);
    assert_eq!(recorded_count, 2);

    let snapshot = state.snapshot();
    assert_eq!(snapshot.rows.len(), 2);
    assert_eq!(
        snapshot.rows[0].source,
        Some(LanPassiveDiscoverySource::Arp)
    );
    assert_eq!(
        snapshot.rows[0].trigger_reason,
        LanPassiveDiscoveryTriggerReason::PassivePacketObserved
    );
    assert_eq!(
        snapshot.rows[0].device_id.as_ref().map(AsRef::as_ref),
        Some("11:22:33:44:55:66")
    );
    assert_eq!(
        snapshot.rows[0].summary,
        "linux-ip-neigh weak hint: ip=192.168.50.10; mac=11:22:33:44:55:66"
    );
    assert_eq!(
        snapshot.rows[1].device_id.as_ref().map(AsRef::as_ref),
        Some("aa:bb:cc:dd:ee:ff")
    );
    assert_eq!(
        snapshot.rows[1].summary,
        "linux-ip-neigh weak hint: ip=192.168.50.200; mac=aa:bb:cc:dd:ee:ff"
    );
    assert_eq!(
        snapshot.rows[1].previous_event_id,
        Some(snapshot.rows[0].event_id.clone())
    );
}

#[test]
fn previous_inventory_does_not_match_unrelated_devices_by_empty_mac() {
    let previous = LanNetworkInventoryDevice {
        device_id: "mdns-only-printer".to_string(),
        label: "Kitchen Printer".to_string(),
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: "192.168.50.40".to_string(),
        mac_address: String::new(),
        hostname: Some("kitchen-printer.local".to_string()),
        network_interface: None,
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string()],
        observed_at: String::new(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    let inventory = LanPreviousNetworkInventory::from_devices(&[previous]);

    assert!(inventory.find("", "192.168.50.41").is_none());
    assert_eq!(
        inventory
            .find("", "192.168.50.40")
            .map(|device| device.device_id.as_str()),
        Some("mdns-only-printer")
    );
}

#[test]
fn discovered_devices_preserve_selected_interface_on_service_identity_evidence() {
    let devices = vec![LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: "Family Tablet".to_string(),
        platform: "windows".to_string(),
        ip_address: "192.168.0.25".to_string(),
        mac_address: "00-11-22-33-44-55".to_string(),
        hostname: Some("family-tablet".to_string()),
        network_interface: Some("Wi-Fi".to_string()),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS.to_string()),
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        observed_at: "2026-06-28T00:00:15Z".to_string(),
        used_previous_scan_hint: false,
        service_identity_probe_evidence: vec![LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            value: "Family Tablet Admin".to_string(),
            selected_interface: Some("Wi-Fi".to_string()),
        }],
    }];

    let discovered = discovered_devices_from_network_inventory(&devices, "2026-06-28T00:00:00Z");

    assert_eq!(discovered.len(), 1);
    assert_eq!(
        discovered[0].service_identity_probe_evidence[0]
            .selected_interface
            .as_deref(),
        Some("Wi-Fi")
    );
    assert_eq!(discovered[0].discovered_at, "2026-06-28T00:00:15Z");
}
