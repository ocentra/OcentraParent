use chrono::{DateTime, Utc};
use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingDeviceRef, LanPairingText};

use super::{scan_history_is_recent, LanScanHistorySnapshot};

pub(super) fn recent_previous_scan_agent_truth_devices(
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Vec<LanPairingDeviceRef> {
    let Some(previous_scan_snapshot) = previous_scan_snapshot else {
        return Vec::new();
    };
    if !scan_history_is_recent(
        &LanPairingText(previous_scan_snapshot.updated_at.clone()),
        now,
    ) {
        return Vec::new();
    }

    previous_scan_snapshot
        .devices
        .iter()
        .filter(|device| historical_agent_truth_should_suppress_probe(device))
        .map(previous_scan_truth_device)
        .collect()
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
