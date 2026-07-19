use std::fs::remove_file;
use std::path::{Path, PathBuf as TestPathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::SecondsFormat;
use ocentra_lan_core::network_inventory::{
    LanTargetedArpRefreshEvidence, LanTargetedArpRefreshOutcome,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use super::*;
use crate::test_invariants::{require_ok, require_some};

#[test]
fn persistent_runtime_saves_and_loads_scan_history_sidecar() {
    let registry_path = temp_registry_path();
    cleanup_test_files(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let devices = vec![sample_network_device()];

    save_scan_history(&runtime, &devices, Some(sample_scan_metadata()));

    assert_eq!(load_scan_history(&runtime), devices);
    let scan_history_registry_path = LanScanHistoryRegistryPath::from(registry_path.as_path());
    assert!(scan_history_path_for_registry(&scan_history_registry_path)
        .as_ref()
        .exists());

    cleanup_test_files(&registry_path);
}

#[test]
fn in_memory_runtime_does_not_persist_scan_history() {
    let runtime = LanPairingRuntime::empty();
    let devices = vec![sample_network_device()];

    save_scan_history(&runtime, &devices, Some(sample_scan_metadata()));

    assert!(load_scan_history(&runtime).is_empty());
}

#[test]
fn persistent_runtime_loads_snapshot_metadata_for_probe_suppression() {
    let registry_path = temp_registry_path();
    cleanup_test_files(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let devices = vec![sample_network_device()];

    save_scan_history(&runtime, &devices, Some(sample_scan_metadata()));

    let snapshot = require_some(
        load_scan_history_snapshot(&runtime),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(snapshot.schema_version, LAN_SCAN_HISTORY_SCHEMA_VERSION);
    assert_eq!(snapshot.devices, devices);
    let parsed_updated_at = require_ok(
        chrono::DateTime::parse_from_rfc3339(&snapshot.updated_at),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(
        parsed_updated_at.to_rfc3339_opts(SecondsFormat::Millis, true),
        snapshot.updated_at
    );
    assert_eq!(snapshot.metadata, Some(sample_scan_metadata()));
    let metadata = require_some(
        snapshot.metadata.as_ref(),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(
        metadata.scan_plan.selected_interface.as_deref(),
        Some("Wi-Fi")
    );
    assert_eq!(
        metadata.scan_plan.targeted_arp_refresh_evidence[0]
            .selected_interface
            .as_deref(),
        Some("Wi-Fi")
    );

    cleanup_test_files(&registry_path);
}

#[test]
fn recent_previous_scan_agent_truth_ignores_agentless_history() {
    let now = Utc::now();
    let snapshot = LanScanHistorySnapshot {
        schema_version: 1,
        updated_at: now.to_rfc3339_opts(SecondsFormat::Millis, true),
        metadata: None,
        devices: vec![sample_agent_truth_device(), agentless_network_device()],
        replay_canonical_devices: None,
    };

    let devices = recent_previous_scan_agent_truth_devices(Some(&snapshot), now);

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].mac_address.as_deref(), Some("00-11-22-33-44-55"));
    assert!(devices
        .iter()
        .all(|device| device.mac_address.as_deref() != Some("00-66-77-88-99-AA")));
}

#[test]
fn legacy_snapshot_without_metadata_still_loads() {
    let registry_path = temp_registry_path();
    cleanup_test_files(&registry_path);
    let runtime = LanPairingRuntime::persistent_json(&registry_path);
    let scan_history_registry_path = LanScanHistoryRegistryPath::from(registry_path.as_path());
    let path = scan_history_path_for_registry(&scan_history_registry_path);
    let legacy_json = serde_json::json!({
        "schemaVersion": 1,
        "updatedAt": "2026-06-24T02:00:00.000Z",
        "devices": [sample_network_device()],
    });

    require_ok(
        fs::write(
            &path,
            require_ok(
                serde_json::to_vec_pretty(&legacy_json),
                constants::error::AGENT_EVENT_SERIALIZES,
            ),
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    let snapshot = require_some(
        load_scan_history_snapshot(&runtime),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert_eq!(snapshot.schema_version, 1);
    assert!(snapshot.metadata.is_none());
    assert_eq!(snapshot.devices, vec![sample_network_device()]);

    cleanup_test_files(&registry_path);
}

fn temp_registry_path() -> TestPathBuf {
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!("ocentra-lan-registry-{unique_id}.json"))
}

fn cleanup_test_files(registry_path: &Path) {
    let _ = remove_file(registry_path);
    let scan_history_registry_path = LanScanHistoryRegistryPath::from(registry_path);
    let _ = remove_file(scan_history_path_for_registry(&scan_history_registry_path).as_ref());
}

fn sample_network_device() -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: "lan-device-1".to_string(),
        label: "Family Tablet".to_string(),
        platform: "windows".to_string(),
        ip_address: "192.168.0.25".to_string(),
        mac_address: "00-11-22-33-44-55".to_string(),
        hostname: Some("family-tablet".to_string()),
        network_interface: Some("Wi-Fi".to_string()),
        observed_at: "2026-06-24T02:00:00.000Z".to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint: true,
        service_identity_probe_evidence: vec![LanServiceIdentityProbeEvidence {
            evidence_kind: LanServiceIdentityProbeEvidenceKind::HtmlTitle,
            value: "Family Tablet Admin".to_string(),
            selected_interface: Some("Wi-Fi".to_string()),
        }],
    }
}

fn sample_agent_truth_device() -> LanNetworkInventoryDevice {
    let mut device = sample_network_device();
    device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    device
}

fn agentless_network_device() -> LanNetworkInventoryDevice {
    LanNetworkInventoryDevice {
        device_id: "lan-device-2".to_string(),
        label: "Media Player".to_string(),
        platform: "unknown".to_string(),
        ip_address: "192.168.0.77".to_string(),
        mac_address: "00-66-77-88-99-AA".to_string(),
        hostname: Some("media-player".to_string()),
        network_interface: Some("Ethernet".to_string()),
        observed_at: "2026-06-24T02:00:01.000Z".to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    }
}

fn sample_scan_metadata() -> LanScanHistoryMetadata {
    LanScanHistoryMetadata {
        scan_id: "lan-scan-1719196800000".to_string(),
        paired_registry_truth_count: 1,
        recent_previous_agent_truth_count: 1,
        durable_household_truth_count: 2,
        scan_plan: LanDiscoveryScanPlan {
            refresh_mode:
                ocentra_lan_core::network_inventory::LanDiscoveryRefreshMode::ActiveSubnetRefresh,
            selected_interface: Some("Wi-Fi".to_string()),
            local_ip_address: Some("192.168.0.42".to_string()),
            ipv4_cidr: Some("192.168.0.42/24".to_string()),
            default_gateway: Some("192.168.0.1".to_string()),
            dns_servers: vec!["192.168.0.1".to_string(), "1.1.1.1".to_string()],
            dhcp_server: Some("192.168.0.1".to_string()),
            broadcast_address: Some("192.168.0.255".to_string()),
            ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
            trusted_truth_device_count: 2,
            previous_device_count: 3,
            active_ipv4_candidate_count: 253,
            active_ipv4_target_count: 251,
            prioritized_previous_target_count: 1,
            active_ipv4_target_timeout_ms: Some(200),
            allow_wsd_identity_query: true,
            allow_snmp_identity_query: false,
            allow_os_fingerprint: false,
            suppressed_active_ipv4_targets: vec![
                "192.168.0.1".to_string(),
                "192.168.0.25".to_string(),
            ],
            targeted_arp_refresh_evidence: vec![LanTargetedArpRefreshEvidence {
                target_ip_address: "192.168.0.25".to_string(),
                selected_interface: Some("Wi-Fi".to_string()),
                expected_mac_address: Some("00-11-22-33-44-55".to_string()),
                observed_mac_address: Some("00-11-22-33-44-55".to_string()),
                observed_at_unix_ms: 1_719_196_800_000,
                source: constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string(),
                outcome: Some(LanTargetedArpRefreshOutcome::Response),
                strong_identity_match: true,
                throttled: false,
            }],
        },
    }
}
