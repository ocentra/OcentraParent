use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingText,
    LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanDiscoveryEventHistory, LanDiscoveryEventHistoryState,
    LanDiscoveryEventKind, LanDiscoveryEventRow,
};

#[path = "discovery_event_history/canonical_device_events.rs"]
mod canonical_device_events;
#[path = "discovery_event_history/event_row.rs"]
mod event_row;
#[path = "discovery_event_history/metadata_events.rs"]
mod metadata_events;
#[path = "discovery_event_history/network_device_events.rs"]
mod network_device_events;

use super::physical_lan_scan::LanNetworkDeviceScanResult;
use super::scan_history;

pub(crate) fn discovery_event_history(
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) -> LanDiscoveryEventHistory {
    let rows = ordered_discovery_event_rows(scan_result, read_model);
    let latest = rows.last();
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: read_model.generated_at.clone(),
        state: discovery_event_history_state(scan_result, &rows, read_model),
        latest_event_id: latest.map(|event| event.event_id.clone()),
        latest_observed_at: latest.map(|event| event.occurred_at.clone()),
        rows,
    }
}

pub(crate) fn discovery_event_history_state(
    scan_result: &LanNetworkDeviceScanResult,
    rows: &[LanDiscoveryEventRow],
    read_model: &LanBrowserAddDeviceReadModel,
) -> LanDiscoveryEventHistoryState {
    if read_model.add_device_state == LanPairingProductionDiscoveryState::Unavailable
        || read_model.physical_household_lan_state
            == LanPairingProductionDiscoveryState::Unavailable
    {
        return LanDiscoveryEventHistoryState::Unavailable;
    }
    if scan_history_snapshot_is_degraded(scan_result) {
        return LanDiscoveryEventHistoryState::Degraded;
    }
    if has_agent_offline_history_state(read_model) {
        return LanDiscoveryEventHistoryState::AgentOffline;
    }
    if has_material_discovery_event_rows(rows) {
        return LanDiscoveryEventHistoryState::Ready;
    }
    if scan_history_snapshot_is_available(scan_result) {
        return LanDiscoveryEventHistoryState::Empty;
    }
    if read_model.physical_household_lan_state == LanPairingProductionDiscoveryState::ManualRequired
    {
        return LanDiscoveryEventHistoryState::ManualRequired;
    }
    LanDiscoveryEventHistoryState::Empty
}

pub(crate) fn ordered_discovery_event_rows(
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) -> Vec<LanDiscoveryEventRow> {
    let mut rows = Vec::new();
    metadata_events::push_scan_metadata_event_rows(&mut rows, scan_result, read_model);
    network_device_events::push_scan_device_event_rows(&mut rows, scan_result);
    canonical_device_events::push_canonical_household_event_rows(
        &mut rows,
        scan_result,
        read_model,
    );
    rows.sort_by(|left, right| {
        left.occurred_at
            .cmp(&right.occurred_at)
            .then_with(|| left.event_id.cmp(&right.event_id))
    });
    for index in 1..rows.len() {
        let previous_event_id = rows[index - 1].event_id.clone();
        rows[index].previous_event_id = Some(previous_event_id);
    }
    rows
}

fn has_agent_offline_history_state(read_model: &LanBrowserAddDeviceReadModel) -> bool {
    let readiness = &read_model.selected_device_readiness;
    readiness.selected_child_device_id.is_some()
        && readiness.route_id.is_none()
        && readiness.trust_state == LanPairingTrustState::Paired
        && readiness.reachability == LanPairingDeviceReachability::Online
        && !readiness.ready_for_control
}

fn has_material_discovery_event_rows(rows: &[LanDiscoveryEventRow]) -> bool {
    rows.iter().any(|row| {
        !matches!(
            row.event_kind,
            LanDiscoveryEventKind::InterfaceChanged
                | LanDiscoveryEventKind::ScanStarted
                | LanDiscoveryEventKind::ScanFinished
        )
    })
}

fn scan_history_snapshot_is_available(scan_result: &LanNetworkDeviceScanResult) -> bool {
    current_scan_snapshot(scan_result)
        .and_then(|snapshot| snapshot.metadata.as_ref())
        .is_some()
}

fn scan_history_snapshot_is_degraded(scan_result: &LanNetworkDeviceScanResult) -> bool {
    [
        scan_result.current_scan_snapshot.as_ref(),
        scan_result.previous_scan_snapshot.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|snapshot| snapshot.metadata.is_none())
}

pub(super) fn current_scan_snapshot(
    scan_result: &LanNetworkDeviceScanResult,
) -> Option<&scan_history::LanScanHistorySnapshot> {
    scan_result
        .current_scan_snapshot
        .as_ref()
        .or(scan_result.previous_scan_snapshot.as_ref())
}

pub(super) fn scan_session_id_for_result(
    scan_result: &LanNetworkDeviceScanResult,
) -> Option<LanPairingText> {
    current_scan_snapshot(scan_result)
        .and_then(|snapshot| snapshot.metadata.as_ref())
        .map(|metadata| LanPairingText(metadata.scan_id.clone()))
}

pub(super) fn scan_session_key(
    scan_session_id: Option<&LanPairingText>,
    fallback: LanPairingText,
) -> LanPairingText {
    const SCAN_SESSION_KEY_REPLACEMENT: &str = "-";
    scan_session_id.cloned().unwrap_or_else(|| {
        let fallback = fallback.0;
        LanPairingText(fallback.replace([':', '.'], SCAN_SESSION_KEY_REPLACEMENT))
    })
}
