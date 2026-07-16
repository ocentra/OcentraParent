use crate::support::{OptionTestExt as _, ResultTestExt as _};
use std::collections::HashMap;
use std::fs;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use crate::network_inventory::{neighbor_record, neighbor_record_with_values, trusted_device};
use ocentra_lan_core::network_inventory::windows_neighbors::netbios::{
    direct_netbios_hostname, netbios_adapter_status_name, netbios_cache_entry,
};
use ocentra_lan_core::network_inventory::windows_neighbors::*;
use ocentra_lan_core::network_inventory::{
    LanIdentityHintInventory, LanNetworkInventoryDevice, LanPreviousNetworkInventory,
};

const TEST_NEIGHBOR_OBSERVED_AT: &str = "2026-06-28T12:00:00Z";

fn lan_plan_fixture_records(name: impl AsRef<std::path::Path>) -> Vec<serde_json::Value> {
    serde_json::from_str(
        &fs::read_to_string(format!(
            "{}/tests/fixtures/lan-plan/{}",
            env!("CARGO_MANIFEST_DIR"),
            name.as_ref().display()
        ))
        .value_or_unreachable(),
    )
    .value_or_unreachable()
}

#[test]
fn windows_neighbor_parser_keeps_cached_hostname_when_later_rows_are_ip_only() {
    ocentra_lan_core::network_inventory::neighbor_support::cache::clear_cached_neighbor_identities(
    );
    let named = network_device_from_windows_neighbor(
        &neighbor_record(Some(constants::lan_pairing::TEST_HOSTNAME.to_string())),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    );
    let named = named.value_or_unreachable();

    assert_eq!(
        named.hostname,
        Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
    );
    assert_eq!(named.label, constants::lan_pairing::TEST_HOSTNAME);

    let unnamed = network_device_from_windows_neighbor(
        &neighbor_record(None::<String>),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    );
    let unnamed = unnamed.value_or_unreachable();

    assert_eq!(
        unnamed.hostname,
        Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
    );
    assert_eq!(unnamed.label, constants::lan_pairing::TEST_HOSTNAME);
}

#[test]
fn windows_neighbor_parser_rejects_unsafe_and_oversized_hostnames() {
    let parsed = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.99".to_string(),
            "00-11-22-33-44-77".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Ethernet".to_string()),
            Some("bad host<script>".to_string()),
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    );
    let parsed = parsed.value_or_unreachable();

    assert!(parsed.hostname.is_none());
    assert_eq!(
        parsed.label,
        format!(
            "{}{}",
            constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
            "192.168.2.99"
        )
    );

    let oversized = "a".repeat(256);
    let parsed = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.100".to_string(),
            "00-11-22-33-44-78".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Ethernet".to_string()),
            Some(oversized),
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    );
    let parsed = parsed.value_or_unreachable();

    assert!(parsed.hostname.is_none());
    assert_eq!(
        parsed.label,
        format!(
            "{}{}",
            constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
            "192.168.2.100"
        )
    );
}

#[test]
fn windows_neighbor_parser_keeps_duplicate_hostname_rows_separate_by_mac() {
    let first = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.101".to_string(),
            "00-11-22-33-44-79".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Ethernet".to_string()),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    )
    .value_or_unreachable();
    let second = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.102".to_string(),
            "00-11-22-33-44-7A".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Ethernet".to_string()),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    )
    .value_or_unreachable();

    assert_ne!(first.device_id, second.device_id);
    assert_eq!(
        first.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(
        second.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
}

#[test]
fn windows_neighbor_parser_accepts_ipv6_rows_without_forcing_ipv4_only_logic() {
    let parsed = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "fe80::2b4d".to_string(),
            "00-11-22-33-44-66".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Ethernet".to_string()),
            None::<String>,
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    );
    let parsed = parsed.value_or_unreachable();

    assert_eq!(parsed.ip_address, "fe80::2b4d");
    assert_eq!(parsed.mac_address, "00-11-22-33-44-66");
    assert_eq!(parsed.platform, constants::lan_pairing::PLATFORM_UNKNOWN);
    assert_eq!(parsed.network_interface.as_deref(), Some("Ethernet"));
    assert_eq!(
        parsed.scan_sources,
        vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()]
    );
}

#[test]
fn windows_neighbor_table_basic_fixture_preserves_neighbor_rows() {
    let parsed = lan_plan_fixture_records("windows_neighbor_table_basic.json")
        .iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(
                record,
                TEST_NEIGHBOR_OBSERVED_AT,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].ip_address, "192.168.2.1");
    assert_eq!(parsed[0].mac_address, "00-11-22-33-44-55");
    assert_eq!(parsed[0].network_interface.as_deref(), Some("Ethernet"));
    assert_eq!(parsed[0].observed_at, TEST_NEIGHBOR_OBSERVED_AT);
    assert_eq!(parsed[1].hostname.as_deref(), Some("iphone.local"));
}

#[test]
fn windows_neighbor_table_empty_incomplete_and_malformed_fixtures_produce_no_rows() {
    let empty = lan_plan_fixture_records("windows_neighbor_table_empty.json")
        .iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(
                record,
                TEST_NEIGHBOR_OBSERVED_AT,
            )
        })
        .collect::<Vec<_>>();
    let incomplete = lan_plan_fixture_records("windows_neighbor_table_incomplete.json")
        .iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(
                record,
                TEST_NEIGHBOR_OBSERVED_AT,
            )
        })
        .collect::<Vec<_>>();
    let malformed = lan_plan_fixture_records("windows_neighbor_table_malformed.json")
        .iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(
                record,
                TEST_NEIGHBOR_OBSERVED_AT,
            )
        })
        .collect::<Vec<_>>();

    assert!(empty.is_empty());
    assert!(incomplete.is_empty());
    assert!(malformed.is_empty());
}

#[test]
fn windows_neighbor_table_duplicate_fixture_keeps_duplicate_candidates_until_merge() {
    let parsed = lan_plan_fixture_records("windows_neighbor_table_duplicate.json")
        .iter()
        .filter_map(|record| {
            windows_neighbor_observation_from_record_with_observed_at(
                record,
                TEST_NEIGHBOR_OBSERVED_AT,
            )
        })
        .collect::<Vec<_>>();

    assert_eq!(parsed.len(), 2);
    assert_eq!(parsed[0].mac_address, parsed[1].mac_address);
    assert_ne!(parsed[0].ip_address, parsed[1].ip_address);

    let merged = ocentra_lan_core::network_inventory::merge_neighbor_observations_by_mac(parsed);
    assert_eq!(merged.len(), 1);
    assert_eq!(merged[0].ip_address, "192.168.2.61");
    assert_eq!(merged[0].hostname.as_deref(), Some("iphone.local"));
}

#[test]
fn previous_scan_hydrates_hostname_platform_and_label_for_same_mac() {
    let test_ip = "192.168.2.88";
    let test_mac = "00-50-56-aa-bb-cc";
    let previous_inventory = LanPreviousNetworkInventory::from_devices(&[
        LanNetworkInventoryDevice {
            device_id: "lan-device-previous".to_string(),
            label: constants::lan_pairing::TEST_HOSTNAME.to_string(),
            platform: constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
            ip_address: test_ip.to_string(),
            mac_address: test_mac.to_string(),
            hostname: Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            observed_at: String::new(),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        },
    ]);

    let hydrated = network_device_from_windows_neighbor(
        &serde_json::json!({
            constants::lan_pairing::JSON_KEY_IP_ADDRESS: test_ip,
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS: test_mac,
            constants::lan_pairing::JSON_KEY_STATE: constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE,
        }),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &previous_inventory,
        None,
    );
    let hydrated = hydrated.value_or_unreachable();

    assert_eq!(
        hydrated.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(hydrated.label, constants::lan_pairing::TEST_HOSTNAME);
    assert_eq!(hydrated.platform, constants::lan_pairing::PLATFORM_WINDOWS);
    assert!(hydrated.used_previous_scan_hint);
}

#[test]
fn trusted_registry_hydrates_identity_before_previous_scan_history() {
    let trusted_inventory = LanIdentityHintInventory::from_devices(&[trusted_device(
        constants::lan_pairing::TEST_LAN_MAC.to_string(),
        Some(constants::lan_pairing::TEST_LAN_IP.to_string()),
        Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        "Family Tablet".to_string(),
        constants::lan_pairing::PLATFORM_WINDOWS.to_string(),
    )]);
    let previous_inventory = LanPreviousNetworkInventory::from_devices(&[
        LanNetworkInventoryDevice {
            device_id: "lan-device-previous".to_string(),
            label: "history-label".to_string(),
            platform: constants::lan_pairing::PLATFORM_ROUTER.to_string(),
            ip_address: constants::lan_pairing::TEST_LAN_IP.to_string(),
            mac_address: constants::lan_pairing::TEST_LAN_MAC.to_string(),
            hostname: Some("history-hostname".to_string()),
            network_interface: Some(constants::lan_pairing::TEST_NETWORK_INTERFACE.to_string()),
            observed_at: String::new(),
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: false,
            service_identity_probe_evidence: Vec::new(),
        },
    ]);

    let hydrated = network_device_from_windows_neighbor(
        &neighbor_record(None::<String>),
        &HashMap::new(),
        &trusted_inventory,
        &previous_inventory,
        None,
    );
    let hydrated = hydrated.value_or_unreachable();

    assert_eq!(
        hydrated.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(hydrated.label, constants::lan_pairing::TEST_HOSTNAME);
    assert_eq!(hydrated.platform, constants::lan_pairing::PLATFORM_WINDOWS);
    assert!(!hydrated.used_previous_scan_hint);
}

#[test]
fn netbios_adapter_status_name_prefers_unique_device_service_rows() {
    assert_eq!(
        netbios_adapter_status_name(
            &[
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::NBTSTAT_SERVER_SERVICE_MARKER,
                constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
            ]
            .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
        ),
        Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
    );
    assert_eq!(
        netbios_adapter_status_name(
            &[
                constants::lan_pairing::TEST_HOSTNAME,
                constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
                constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
            ]
            .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
        ),
        Some(constants::lan_pairing::TEST_HOSTNAME.to_string())
    );
    assert!(netbios_adapter_status_name(
        &[
            constants::lan_pairing::TEST_HOSTNAME,
            constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
            constants::lan_pairing::NBTSTAT_GROUP_MARKER,
        ]
        .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
    )
    .is_none());
}

#[test]
fn netbios_adapter_status_name_rejects_unsafe_display_values() {
    assert!(netbios_adapter_status_name("bad<script> workstation unique").is_none());
    assert!(netbios_adapter_status_name(&format!(
        "{} {} {}",
        "a".repeat(256),
        constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
        constants::lan_pairing::NBTSTAT_UNIQUE_MARKER
    ))
    .is_none());
}

#[test]
fn netbios_cache_entry_rejects_unsafe_and_oversized_hostnames() {
    assert!(netbios_cache_entry(
        &[
            "bad<script>",
            constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
            constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
            "192.168.2.55",
        ]
        .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
    )
    .is_none());

    assert!(netbios_cache_entry(
        &[
            &"a".repeat(256),
            constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
            constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
            "192.168.2.56",
        ]
        .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
    )
    .is_none());
}

#[test]
fn netbios_cache_entry_normalizes_unique_hostnames_and_ip_text() {
    assert_eq!(
        netbios_cache_entry(
            &[
                " TEST-HOST ",
                constants::lan_pairing::NBTSTAT_WORKSTATION_SERVICE_MARKER,
                constants::lan_pairing::NBTSTAT_UNIQUE_MARKER,
                " 192.168.2.57 ",
            ]
            .join(constants::lan_pairing::TEST_NETBIOS_STATUS_ROW_SEPARATOR)
        ),
        Some(("192.168.2.57".to_string(), "TEST-HOST".to_string(),))
    );
}

#[test]
fn windows_neighbor_parser_uses_netbios_cache_hostname_and_marks_netbios_scan_source() {
    let mut netbios_names = HashMap::new();
    netbios_names.insert(
        "192.168.2.58".to_string(),
        constants::lan_pairing::TEST_HOSTNAME.to_string(),
    );

    let parsed = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.58".to_string(),
            "00-11-22-33-44-58".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Ethernet".to_string()),
            None::<String>,
        ),
        &netbios_names,
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        None,
    );
    let parsed = parsed.value_or_unreachable();

    assert_eq!(
        parsed.hostname.as_deref(),
        Some(constants::lan_pairing::TEST_HOSTNAME)
    );
    assert_eq!(parsed.label, constants::lan_pairing::TEST_HOSTNAME);
    assert_eq!(parsed.platform, constants::lan_pairing::PLATFORM_WINDOWS);
    assert_eq!(
        parsed.scan_sources,
        vec![
            constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
            constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS.to_string(),
        ]
    );
    assert!(!parsed.used_previous_scan_hint);
}

#[test]
fn foreground_netbios_lookup_is_disabled_for_scan_latency() {
    assert!(direct_netbios_hostname(
        "192.168.2.55",
        &LanPairingDeviceReachability::Online,
        constants::lan_pairing::PLATFORM_UNKNOWN,
    )
    .is_none());
}

#[test]
fn current_windows_neighbor_ipv4_observations_keep_first_normalized_mapping_for_duplicate_ips() {
    let records = vec![
        serde_json::json!({
            constants::lan_pairing::JSON_KEY_IP_ADDRESS: "192.168.2.90",
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS: "00-11-22-33-44-90",
        }),
        serde_json::json!({
            constants::lan_pairing::JSON_KEY_IP_ADDRESS: "192.168.2.90",
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS: "00-11-22-33-44-91",
        }),
    ];

    let observations =
        windows_neighbor_ipv4_observations_from_records(records, TEST_NEIGHBOR_OBSERVED_AT);
    let expected_mac = "00-11-22-33-44-90".to_string();

    assert_eq!(observations.get("192.168.2.90"), Some(&expected_mac));
    assert_eq!(observations.len(), 1);
}

#[test]
fn current_windows_neighbor_ipv4_observations_prefer_more_reachable_ip_for_same_mac() {
    let records = vec![
        serde_json::json!({
            constants::lan_pairing::JSON_KEY_IP_ADDRESS: "192.168.2.94",
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS: "00-11-22-33-44-92",
            constants::lan_pairing::JSON_KEY_STATE: constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE,
            constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS: "Ethernet",
            constants::lan_pairing::JSON_KEY_HOSTNAME: "stale-host",
        }),
        serde_json::json!({
            constants::lan_pairing::JSON_KEY_IP_ADDRESS: "192.168.2.95",
            constants::lan_pairing::JSON_KEY_LINK_LAYER_ADDRESS: "00-11-22-33-44-92",
            constants::lan_pairing::JSON_KEY_STATE: constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE,
            constants::lan_pairing::JSON_KEY_INTERFACE_ALIAS: "Ethernet",
            constants::lan_pairing::JSON_KEY_HOSTNAME: "live-host",
        }),
    ];

    let observations =
        windows_neighbor_ipv4_observations_from_records(records, TEST_NEIGHBOR_OBSERVED_AT);
    let expected_mac = "00-11-22-33-44-92".to_string();

    assert_eq!(observations.get("192.168.2.95"), Some(&expected_mac));
    assert!(!observations.contains_key("192.168.2.94"));
    assert_eq!(observations.len(), 1);
}

#[test]
fn windows_neighbor_parser_requires_selected_interface_match_when_scope_is_explicit() {
    assert!(network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.110".to_string(),
            "00-11-22-33-44-10".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some("Wi-Fi".to_string()),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        Some("Ethernet"),
    )
    .is_none());

    let selected = network_device_from_windows_neighbor(
        &neighbor_record_with_values(
            "192.168.2.111".to_string(),
            "00-11-22-33-44-11".to_string(),
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE.to_string(),
            Some(" ethernet ".to_string()),
            Some(constants::lan_pairing::TEST_HOSTNAME.to_string()),
        ),
        &HashMap::new(),
        &LanIdentityHintInventory::default(),
        &LanPreviousNetworkInventory::default(),
        Some("Ethernet"),
    )
    .value_or_unreachable();

    assert_eq!(selected.network_interface.as_deref(), Some("ethernet"));
}
