use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanDiscoveryEventKind, LanDiscoveryEventRow,
};

use super::event_row::{
    discovery_event_row, interface_changed_event_id, push_discovery_event_row,
    scan_finished_summary,
};
use super::{current_scan_snapshot, scan_session_id_for_result, scan_session_key};
use crate::lan_pairing_browser_add_device_state::physical_lan_scan::LanNetworkDeviceScanResult;

const LAN_DISCOVERY_SCAN_STARTED_SUMMARY: &str = "LAN scan started";
const LAN_DISCOVERY_INTERFACE_CHANGED_SUMMARY_PREFIX: &str = "LAN scan interface changed to ";

pub(super) fn push_scan_metadata_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) {
    let current_snapshot = current_scan_snapshot(scan_result);
    let scan_session_id = scan_session_id_for_result(scan_result);
    let scan_time = current_snapshot
        .map(|snapshot| LanPairingText(snapshot.updated_at.clone()))
        .unwrap_or_else(|| LanPairingText(read_model.generated_at.clone()));

    push_interface_changed_row(rows, scan_result, &scan_session_id, &scan_time);
    push_scan_lifecycle_rows(rows, scan_result, read_model, scan_session_id, scan_time);
}

fn push_interface_changed_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
    scan_session_id: &Option<LanPairingText>,
    scan_time: &LanPairingText,
) {
    let previous_metadata = scan_result
        .previous_scan_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.metadata.as_ref());
    let current_metadata =
        current_scan_snapshot(scan_result).and_then(|snapshot| snapshot.metadata.as_ref());
    let Some((previous_metadata, current_metadata)) = previous_metadata.zip(current_metadata)
    else {
        return;
    };

    if previous_metadata.scan_plan.selected_interface
        == current_metadata.scan_plan.selected_interface
        && previous_metadata.scan_plan.ipv4_cidr == current_metadata.scan_plan.ipv4_cidr
    {
        return;
    }

    let scan_key = scan_session_key(scan_session_id.as_ref(), scan_time.clone());
    let next_interface = current_metadata
        .scan_plan
        .selected_interface
        .clone()
        .unwrap_or_else(|| constants::value::EMPTY.to_string());
    push_discovery_event_row(
        rows,
        discovery_event_row(
            interface_changed_event_id(&scan_key),
            LanDiscoveryEventKind::InterfaceChanged,
            scan_time.clone(),
            scan_session_id.clone(),
            None,
            None,
            LanPairingText({
                let mut summary = String::from(LAN_DISCOVERY_INTERFACE_CHANGED_SUMMARY_PREFIX);
                summary.push_str(&next_interface);
                summary
            }),
        ),
    );
}

fn push_scan_lifecycle_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
    scan_session_id: Option<LanPairingText>,
    scan_time: LanPairingText,
) {
    let current_metadata =
        current_scan_snapshot(scan_result).and_then(|snapshot| snapshot.metadata.as_ref());
    if current_metadata.is_none() || scan_result.reused_recent_snapshot {
        return;
    }

    let scan_key = scan_session_key(scan_session_id.as_ref(), scan_time.clone());
    push_discovery_event_row(
        rows,
        discovery_event_row(
            LanPairingText(format!("lan-discovery-scan-started-{}", scan_key.0)),
            LanDiscoveryEventKind::ScanStarted,
            scan_time.clone(),
            scan_session_id.clone(),
            None,
            None,
            LanPairingText(LAN_DISCOVERY_SCAN_STARTED_SUMMARY.to_string()),
        ),
    );
    push_discovery_event_row(
        rows,
        discovery_event_row(
            LanPairingText(format!("lan-discovery-scan-finished-{}", scan_key.0)),
            LanDiscoveryEventKind::ScanFinished,
            scan_time,
            scan_session_id,
            None,
            None,
            scan_finished_summary(read_model.scan_summary.scanned_device_count),
        ),
    );
}
