use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;

use crate::lan_pairing::LanPairingRuntime;

use super::{
    load_scan_history_snapshot, save_scan_history, LanNetworkDeviceScanResult,
    LanScanHistoryMetadata, LanScanHistorySnapshot,
};

pub(super) fn persisted_scan_result_or_fail(
    runtime: &LanPairingRuntime,
    devices: Vec<LanNetworkInventoryDevice>,
    metadata: LanScanHistoryMetadata,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
) -> LanNetworkDeviceScanResult {
    if !save_scan_history(runtime, &devices, Some(metadata)) {
        return failed_persistence_scan_result(previous_scan_snapshot);
    }
    let current_scan_snapshot = load_scan_history_snapshot(runtime);
    LanNetworkDeviceScanResult {
        devices: current_scan_snapshot
            .as_ref()
            .map(|snapshot| snapshot.devices.clone())
            .unwrap_or(devices),
        previous_scan_snapshot,
        current_scan_snapshot,
        reused_recent_snapshot: false,
    }
}

fn failed_persistence_scan_result(
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
) -> LanNetworkDeviceScanResult {
    LanNetworkDeviceScanResult {
        devices: previous_scan_snapshot
            .as_ref()
            .map(|snapshot| snapshot.devices.clone())
            .unwrap_or_default(),
        current_scan_snapshot: None,
        previous_scan_snapshot,
        reused_recent_snapshot: true,
    }
}
