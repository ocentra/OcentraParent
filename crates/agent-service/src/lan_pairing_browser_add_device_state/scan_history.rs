use std::fs;
use std::path::{Path, PathBuf};

use ocentra_lan_core::network_inventory::{LanDiscoveryScanPlan, LanNetworkInventoryDevice};
use serde::{Deserialize, Serialize};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

const LAN_SCAN_HISTORY_SCHEMA_VERSION: u16 = 2;
const LAN_SCAN_HISTORY_FILE_SUFFIX: &str = "-lan-scan-history.json";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LanScanHistoryMetadata {
    pub(super) scan_id: String,
    pub(super) paired_registry_truth_count: u32,
    pub(super) recent_previous_agent_truth_count: u32,
    #[serde(default)]
    pub(super) durable_household_truth_count: u32,
    pub(super) scan_plan: LanDiscoveryScanPlan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LanScanHistorySnapshot {
    pub(super) schema_version: u16,
    pub(super) updated_at: String,
    #[serde(default)]
    pub(super) metadata: Option<LanScanHistoryMetadata>,
    pub(super) devices: Vec<LanNetworkInventoryDevice>,
}

#[cfg(test)]
pub(super) fn load_scan_history(runtime: &LanPairingRuntime) -> Vec<LanNetworkInventoryDevice> {
    load_scan_history_snapshot(runtime)
        .map(|snapshot| snapshot.devices)
        .unwrap_or_default()
}

pub(super) fn load_scan_history_snapshot(
    runtime: &LanPairingRuntime,
) -> Option<LanScanHistorySnapshot> {
    scan_history_path(runtime).and_then(|path| read_scan_history(&path))
}

pub(super) fn save_scan_history(
    runtime: &LanPairingRuntime,
    devices: &[LanNetworkInventoryDevice],
    metadata: Option<LanScanHistoryMetadata>,
) {
    let Some(path) = scan_history_path(runtime) else {
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let snapshot = LanScanHistorySnapshot {
        schema_version: LAN_SCAN_HISTORY_SCHEMA_VERSION,
        updated_at: timestamp_now(),
        metadata,
        devices: devices.to_vec(),
    };
    if let Ok(json) = serde_json::to_vec_pretty(&snapshot) {
        let _ = fs::write(path, json);
    }
}

fn read_scan_history(path: &Path) -> Option<LanScanHistorySnapshot> {
    let json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

fn scan_history_path(runtime: &LanPairingRuntime) -> Option<PathBuf> {
    match &runtime.persistence {
        LanPairingRegistryPersistence::InMemory => None,
        LanPairingRegistryPersistence::LocalJsonRegistry(path) => {
            Some(scan_history_path_for_registry(path))
        }
    }
}

fn scan_history_path_for_registry(registry_path: &Path) -> PathBuf {
    let file_stem = registry_path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or("lan-pairing-registry");
    registry_path.with_file_name(format!("{file_stem}{LAN_SCAN_HISTORY_FILE_SUFFIX}"))
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use std::time::{SystemTime, UNIX_EPOCH};

    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

    use super::*;

    #[test]
    fn persistent_runtime_saves_and_loads_scan_history_sidecar() {
        let registry_path = temp_registry_path();
        cleanup_test_files(&registry_path);
        let runtime = LanPairingRuntime::persistent_json(&registry_path);
        let devices = vec![sample_network_device()];

        save_scan_history(&runtime, &devices, Some(sample_scan_metadata()));

        assert_eq!(load_scan_history(&runtime), devices);
        assert!(scan_history_path_for_registry(&registry_path).exists());

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

        let snapshot = load_scan_history_snapshot(&runtime)
            .unwrap_or_else(|| unreachable!("scan history snapshot persists"));
        assert_eq!(snapshot.schema_version, LAN_SCAN_HISTORY_SCHEMA_VERSION);
        assert_eq!(snapshot.devices, devices);
        assert!(!snapshot.updated_at.is_empty());
        assert_eq!(snapshot.metadata, Some(sample_scan_metadata()));

        cleanup_test_files(&registry_path);
    }

    #[test]
    fn legacy_snapshot_without_metadata_still_loads() {
        let registry_path = temp_registry_path();
        cleanup_test_files(&registry_path);
        let runtime = LanPairingRuntime::persistent_json(&registry_path);
        let path = scan_history_path_for_registry(&registry_path);
        let legacy_json = serde_json::json!({
            "schemaVersion": 1,
            "updatedAt": "2026-06-24T02:00:00.000Z",
            "devices": [sample_network_device()],
        });

        fs::write(
            &path,
            serde_json::to_vec_pretty(&legacy_json)
                .unwrap_or_else(|error| unreachable!("legacy snapshot serializes: {error:?}")),
        )
        .unwrap_or_else(|error| unreachable!("legacy snapshot writes: {error:?}"));

        let snapshot = load_scan_history_snapshot(&runtime)
            .unwrap_or_else(|| unreachable!("legacy snapshot loads"));
        assert_eq!(snapshot.schema_version, 1);
        assert!(snapshot.metadata.is_none());
        assert_eq!(snapshot.devices, vec![sample_network_device()]);

        cleanup_test_files(&registry_path);
    }

    fn temp_registry_path() -> PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        std::env::temp_dir().join(format!("ocentra-lan-registry-{unique_id}.json"))
    }

    fn cleanup_test_files(registry_path: &Path) {
        let _ = remove_file(registry_path);
        let _ = remove_file(scan_history_path_for_registry(registry_path));
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
            reachability: LanPairingDeviceReachability::Online,
            agent_status: None,
            scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR.to_string()],
            used_previous_scan_hint: true,
        }
    }

    fn sample_scan_metadata() -> LanScanHistoryMetadata {
        LanScanHistoryMetadata {
            scan_id: "lan-scan-1719196800000".to_string(),
            paired_registry_truth_count: 1,
            recent_previous_agent_truth_count: 1,
            durable_household_truth_count: 2,
            scan_plan: LanDiscoveryScanPlan {
                refresh_mode: ocentra_lan_core::network_inventory::LanDiscoveryRefreshMode::ActiveSubnetRefresh,
                selected_interface: Some("Wi-Fi".to_string()),
                local_ip_address: Some("192.168.0.42".to_string()),
                ipv4_cidr: Some("192.168.0.42/24".to_string()),
                default_gateway: Some("192.168.0.1".to_string()),
                dns_servers: vec![
                    "192.168.0.1".to_string(),
                    "1.1.1.1".to_string(),
                ],
                dhcp_server: Some("192.168.0.1".to_string()),
                broadcast_address: Some("192.168.0.255".to_string()),
                ipv6_prefixes: vec!["2001:db8::42/64".to_string()],
                trusted_truth_device_count: 2,
                previous_device_count: 3,
                active_ipv4_candidate_count: 253,
                active_ipv4_target_count: 251,
                prioritized_previous_target_count: 1,
                active_ipv4_target_timeout_ms: Some(200),
                suppressed_active_ipv4_targets: vec![
                    "192.168.0.1".to_string(),
                    "192.168.0.25".to_string(),
                ],
            },
        }
    }
}
