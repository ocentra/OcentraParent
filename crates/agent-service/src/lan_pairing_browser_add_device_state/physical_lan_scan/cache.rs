use chrono::{DateTime, Utc};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentRoute,
};

use super::super::scan_history::{scan_history_is_recent, LanScanHistorySnapshot};
use super::LanNetworkDeviceScanResult;

pub(super) fn cached_scan_result_for_command(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<LanNetworkDeviceScanResult> {
    cached_localhost_status_scan_result(command, previous_scan_snapshot.clone(), now)
        .or_else(|| cached_runtime_event_stream_scan_result(command, previous_scan_snapshot, now))
}

fn cached_runtime_event_stream_scan_result(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<LanNetworkDeviceScanResult> {
    if command.command != AgentCommandName::AgentLanRuntimeEventChainStreamGet
        || command.target.route != AgentRoute::LocalNetwork
    {
        return None;
    }
    Some(cached_scan_result_from_snapshot(
        previous_scan_snapshot,
        now,
    ))
}

fn cached_localhost_status_scan_result(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<LanNetworkDeviceScanResult> {
    if command.command != AgentCommandName::AgentLanPairingStatusGet
        || command.target.route != AgentRoute::Localhost
    {
        return None;
    }
    Some(cached_scan_result_from_snapshot(
        previous_scan_snapshot,
        now,
    ))
}

fn cached_scan_result_from_snapshot(
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> LanNetworkDeviceScanResult {
    let Some(snapshot) = previous_scan_snapshot else {
        return LanNetworkDeviceScanResult::default();
    };
    if !scan_history_is_recent(&LanPairingText(snapshot.updated_at.clone()), now) {
        return LanNetworkDeviceScanResult {
            previous_scan_snapshot: Some(snapshot),
            ..LanNetworkDeviceScanResult::default()
        };
    }

    LanNetworkDeviceScanResult {
        devices: snapshot.devices.clone(),
        previous_scan_snapshot: Some(snapshot.clone()),
        current_scan_snapshot: Some(snapshot),
        reused_recent_snapshot: true,
    }
}

pub(super) fn cached_status_snapshot_devices(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<Vec<ocentra_lan_core::network_inventory::LanNetworkInventoryDevice>> {
    if command.command != AgentCommandName::AgentLanPairingStatusGet
        || command.target.route != AgentRoute::Localhost
    {
        return None;
    }
    let previous_scan_snapshot = previous_scan_snapshot?;
    scan_history_is_recent(
        &LanPairingText(previous_scan_snapshot.updated_at.clone()),
        now,
    )
    .then(|| previous_scan_snapshot.devices.clone())
}
