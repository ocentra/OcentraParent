use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, Utc};
use ocentra_lan_core::network_inventory::{LanDiscoveryScanPlan, LanNetworkInventoryDevice};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use serde::{Deserialize, Serialize};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

pub(crate) const LAN_SCAN_HISTORY_SCHEMA_VERSION: u16 = 2;
const LAN_SCAN_HISTORY_FILE_SUFFIX: &str = "-lan-scan-history.json";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanScanHistoryRegistryPath(PathBuf);

impl From<&Path> for LanScanHistoryRegistryPath {
    fn from(value: &Path) -> Self {
        Self(value.to_path_buf())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct LanScanHistoryDir(PathBuf);

impl AsRef<Path> for LanScanHistoryDir {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanScanHistoryPath(PathBuf);

impl AsRef<Path> for LanScanHistoryPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
    }
}

impl LanScanHistoryPath {
    fn parent_dir(&self) -> Option<LanScanHistoryDir> {
        self.0.parent().map(PathBuf::from).map(LanScanHistoryDir)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LanScanHistoryMetadata {
    pub(crate) scan_id: String,
    pub(crate) paired_registry_truth_count: u32,
    pub(crate) recent_previous_agent_truth_count: u32,
    #[serde(default)]
    pub(crate) durable_household_truth_count: u32,
    pub(crate) scan_plan: LanDiscoveryScanPlan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LanScanHistorySnapshot {
    pub(crate) schema_version: u16,
    pub(crate) updated_at: String,
    #[serde(default)]
    pub(crate) metadata: Option<LanScanHistoryMetadata>,
    pub(crate) devices: Vec<LanNetworkInventoryDevice>,
}

pub(crate) fn recent_previous_scan_agent_truth_devices(
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Vec<LanPairingDeviceRef> {
    let Some(previous_scan_snapshot) = previous_scan_snapshot else {
        return Vec::new();
    };
    if !scan_history_is_recent(&previous_scan_snapshot.updated_at.clone().into(), now) {
        return Vec::new();
    }

    previous_scan_snapshot
        .devices
        .iter()
        .filter(|device| historical_agent_truth_should_suppress_probe(device))
        .map(previous_scan_truth_device)
        .collect()
}

pub(crate) fn load_scan_history_snapshot(
    runtime: &LanPairingRuntime,
) -> Option<LanScanHistorySnapshot> {
    scan_history_path(runtime).and_then(|path| read_scan_history(&path))
}

pub(crate) fn save_scan_history(
    runtime: &LanPairingRuntime,
    devices: &[LanNetworkInventoryDevice],
    metadata: Option<LanScanHistoryMetadata>,
) {
    let Some(path) = scan_history_path(runtime) else {
        return;
    };
    if let Some(parent) = path.parent_dir() {
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

fn read_scan_history(path: &LanScanHistoryPath) -> Option<LanScanHistorySnapshot> {
    let json = fs::read_to_string(path).ok()?;
    serde_json::from_str(&json).ok()
}

fn scan_history_path(runtime: &LanPairingRuntime) -> Option<LanScanHistoryPath> {
    match &runtime.persistence {
        LanPairingRegistryPersistence::InMemory => None,
        LanPairingRegistryPersistence::LocalJsonRegistry(path) => Some(
            scan_history_path_for_registry(&LanScanHistoryRegistryPath(path.clone())),
        ),
    }
}

pub(crate) fn scan_history_path_for_registry(
    registry_path: &LanScanHistoryRegistryPath,
) -> LanScanHistoryPath {
    let file_stem = registry_path
        .0
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .unwrap_or(constants::lan_pairing::REGISTRY_FILE_STEM_FALLBACK);
    LanScanHistoryPath(
        registry_path
            .0
            .with_file_name(format!("{file_stem}{LAN_SCAN_HISTORY_FILE_SUFFIX}")),
    )
}

pub(crate) fn scan_history_is_recent(updated_at: &LanPairingText, now: DateTime<Utc>) -> bool {
    DateTime::parse_from_rfc3339(updated_at.0.as_str())
        .map(|parsed| {
            let parsed = parsed.with_timezone(&Utc);
            parsed <= now
                && now.signed_duration_since(parsed)
                    <= Duration::seconds(
                        constants::lan_pairing::LAN_PREVIOUS_SCAN_AGENT_TRUTH_REUSE_WINDOW_SECONDS,
                    )
        })
        .unwrap_or(false)
}

fn historical_agent_truth_should_suppress_probe(device: &LanNetworkInventoryDevice) -> bool {
    matches!(
        device.agent_status.as_deref(),
        Some(constants::lan_pairing::LOCAL_AGENT_STATUS)
            | Some(constants::lan_pairing::SERVICE_IDENTITY_PROBE_AGENT_STATUS)
    )
}

fn previous_scan_truth_device(device: &LanNetworkInventoryDevice) -> LanPairingDeviceRef {
    let mut truth_device = LanPairingDeviceRef::new(
        device.device_id.clone(),
        None,
        device.label.clone(),
        device.platform.clone(),
    );
    truth_device.ip_address = Some(device.ip_address.clone());
    truth_device.mac_address = Some(device.mac_address.clone());
    truth_device.hostname = device.hostname.clone();
    truth_device.network_interface = device.network_interface.clone();
    truth_device.agent_status = device.agent_status.clone();
    truth_device
}
