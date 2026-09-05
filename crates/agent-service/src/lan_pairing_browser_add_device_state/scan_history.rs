use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Duration, Utc};
use ocentra_lan_core::network_inventory::{LanDiscoveryScanPlan, LanNetworkInventoryDevice};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdRouteState,
};
use serde::{Deserialize, Serialize};

use crate::{
    lan_pairing::{LanPairingRegistryPersistence, LanPairingRuntime},
    time::timestamp_now,
};

#[path = "scan_history_previous_devices.rs"]
mod scan_history_previous_devices;
#[path = "scan_history/write_lock.rs"]
pub(crate) mod write_lock;

use write_lock::{scan_history_write_lock, ScanHistoryLockKind};

pub(crate) const LAN_SCAN_HISTORY_SCHEMA_VERSION: u16 = 2;
const LAN_SCAN_HISTORY_MIN_SUPPORTED_SCHEMA_VERSION: u16 = 1;
const LAN_SCAN_HISTORY_FILE_SUFFIX: &str = "-lan-scan-history.json";
const LAN_SCAN_HISTORY_TEMPORARY_EXTENSION_PREFIX: &str = "tmp";
const LAN_SCAN_HISTORY_TEMPORARY_EXTENSION_SEPARATOR: &str = "-";

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanScanHistoryRegistryPath(PathBuf);

impl From<&Path> for LanScanHistoryRegistryPath {
    fn from(value: &Path) -> Self {
        Self(value.to_path_buf())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LanScanHistoryPath(PathBuf);

impl AsRef<Path> for LanScanHistoryPath {
    fn as_ref(&self) -> &Path {
        self.0.as_path()
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
    #[serde(default)]
    pub(crate) replay_canonical_projection: Option<LanReplayCanonicalProjection>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LanReplayCanonicalProjection {
    pub(crate) schema_version: u16,
    pub(crate) generated_at: String,
    pub(crate) canonical_devices: Vec<LanCanonicalHouseholdDevice>,
}

const LAN_REPLAY_CANONICAL_PROJECTION_SCHEMA_VERSION: u16 = 1;

pub(crate) fn recent_previous_scan_agent_truth_devices(
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Vec<LanPairingDeviceRef> {
    scan_history_previous_devices::recent_previous_scan_agent_truth_devices(
        previous_scan_snapshot,
        now,
    )
}

pub(crate) fn load_scan_history_snapshot(
    runtime: &LanPairingRuntime,
) -> Option<LanScanHistorySnapshot> {
    scan_history_path(runtime).and_then(|path| read_scan_history(&path))
}

pub(crate) fn scan_history_execution_lock(
    runtime: &LanPairingRuntime,
) -> Option<write_lock::CrossProcessPathLock> {
    let path = scan_history_path(runtime)?;
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).ok()?;
    }
    write_lock::cross_process_path_lock(&path, &ScanHistoryLockKind::Execution)
}

pub(crate) fn save_scan_history(
    runtime: &LanPairingRuntime,
    devices: &[LanNetworkInventoryDevice],
    metadata: Option<LanScanHistoryMetadata>,
) -> bool {
    let Some(path) = scan_history_path(runtime) else {
        return false;
    };
    if let Some(parent) = path.as_ref().parent() {
        let _ = fs::create_dir_all(parent);
    }
    let Some(_lock) = scan_history_write_lock(&path) else {
        return false;
    };
    let snapshot = LanScanHistorySnapshot {
        schema_version: LAN_SCAN_HISTORY_SCHEMA_VERSION,
        updated_at: timestamp_now(),
        metadata,
        devices: devices.to_vec(),
        replay_canonical_projection: None,
    };
    write_scan_history(&path, &snapshot)
}

pub(crate) fn save_replay_canonical_devices(
    runtime: &LanPairingRuntime,
    expected_snapshot: &LanScanHistorySnapshot,
    replay_canonical_devices: &[LanCanonicalHouseholdDevice],
    generated_at: &LanPairingText,
) -> Option<LanReplayCanonicalProjection> {
    if !canonical_devices_are_valid(replay_canonical_devices) {
        return None;
    }
    let path = scan_history_path(runtime)?;
    let _lock = scan_history_write_lock(&path)?;
    let mut snapshot = read_scan_history(&path)?;
    if !same_scan_generation(&snapshot, expected_snapshot) {
        return None;
    }
    if let Some(projection) = valid_replay_projection(snapshot.replay_canonical_projection.as_ref())
    {
        return Some(projection.clone());
    }
    let projection = LanReplayCanonicalProjection {
        schema_version: LAN_REPLAY_CANONICAL_PROJECTION_SCHEMA_VERSION,
        generated_at: strict_rfc3339_timestamp(generated_at)?
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        canonical_devices: replay_canonical_devices.to_vec(),
    };
    snapshot.replay_canonical_projection = Some(projection.clone());
    write_scan_history(&path, &snapshot).then_some(projection)
}

fn same_scan_generation(
    current: &LanScanHistorySnapshot,
    expected: &LanScanHistorySnapshot,
) -> bool {
    current.schema_version == expected.schema_version
        && current.updated_at == expected.updated_at
        && current.metadata == expected.metadata
        && current.devices == expected.devices
}

pub(crate) fn valid_replay_projection(
    projection: Option<&LanReplayCanonicalProjection>,
) -> Option<&LanReplayCanonicalProjection> {
    projection.filter(|projection| {
        projection.schema_version == LAN_REPLAY_CANONICAL_PROJECTION_SCHEMA_VERSION
            && strict_rfc3339_timestamp(&LanPairingText(projection.generated_at.clone())).is_some()
            && canonical_devices_are_valid(&projection.canonical_devices)
    })
}

fn canonical_devices_are_valid(devices: &[LanCanonicalHouseholdDevice]) -> bool {
    let mut canonical_device_ids = HashSet::with_capacity(devices.len());
    devices.iter().all(|device| {
        let canonical_device_id = device.canonical_device_id.trim();
        let route_id_is_valid = device
            .route_id
            .as_deref()
            .map(|route_id| !route_id.trim().is_empty())
            .unwrap_or(true);
        let non_child_device_is_unroutable = matches!(
            &device.classification,
            LanCanonicalHouseholdDeviceClassification::ChildAgent
        ) || (device.route_id.is_none()
            && device.child_agent_inventory.is_none()
            && matches!(
                &device.route_state,
                LanCanonicalHouseholdRouteState::Unavailable
            ));

        device.schema_version == constants::lan_pairing::SCHEMA_VERSION
            && !canonical_device_id.is_empty()
            && canonical_device_id == device.canonical_device_id
            && !device.display_name.trim().is_empty()
            && canonical_device_ids.insert(canonical_device_id)
            && (!device.enrollable
                || matches!(
                    &device.classification,
                    LanCanonicalHouseholdDeviceClassification::ChildAgent
                ))
            && route_id_is_valid
            && non_child_device_is_unroutable
    })
}

fn strict_rfc3339_timestamp(value: &LanPairingText) -> Option<DateTime<Utc>> {
    let parsed = DateTime::parse_from_rfc3339(value.0.as_str())
        .ok()?
        .with_timezone(&Utc);
    (parsed.to_rfc3339_opts(chrono::SecondsFormat::Millis, true) == value.0).then_some(parsed)
}

fn write_scan_history(path: &LanScanHistoryPath, snapshot: &LanScanHistorySnapshot) -> bool {
    let Ok(json) = serde_json::to_vec_pretty(snapshot) else {
        return false;
    };
    let process_id = std::process::id().to_string();
    let temporary_extension = [
        LAN_SCAN_HISTORY_TEMPORARY_EXTENSION_PREFIX,
        LAN_SCAN_HISTORY_TEMPORARY_EXTENSION_SEPARATOR,
        process_id.as_str(),
    ]
    .concat();
    let temporary_path = path.0.with_extension(temporary_extension);
    fs::write(&temporary_path, json).is_ok() && fs::rename(temporary_path, path.0.as_path()).is_ok()
}

fn read_scan_history(path: &LanScanHistoryPath) -> Option<LanScanHistorySnapshot> {
    let json = fs::read_to_string(path).ok()?;
    let snapshot = serde_json::from_str(&json).ok()?;
    valid_scan_history_snapshot(&snapshot).then_some(snapshot)
}

fn valid_scan_history_snapshot(snapshot: &LanScanHistorySnapshot) -> bool {
    (LAN_SCAN_HISTORY_MIN_SUPPORTED_SCHEMA_VERSION..=LAN_SCAN_HISTORY_SCHEMA_VERSION)
        .contains(&snapshot.schema_version)
        && strict_rfc3339_timestamp(&LanPairingText(snapshot.updated_at.clone())).is_some()
        && snapshot
            .metadata
            .as_ref()
            .map(|metadata| !metadata.scan_id.trim().is_empty())
            .unwrap_or(true)
        && snapshot
            .replay_canonical_projection
            .as_ref()
            .map(|projection| valid_replay_projection(Some(projection)).is_some())
            .unwrap_or(true)
}

fn scan_history_path(runtime: &LanPairingRuntime) -> Option<LanScanHistoryPath> {
    match &runtime.persistence {
        LanPairingRegistryPersistence::InMemory
        | LanPairingRegistryPersistence::UnavailableLocalJsonRegistry => None,
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
