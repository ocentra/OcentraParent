use chrono::DateTime;
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
#[path = "discovery_event_history/network_device_events.rs"]
mod network_device_events;

use super::physical_lan_scan::LanNetworkDeviceScanResult;
use super::scan_history;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Rfc3339Timestamp {
    text: LanPairingText,
    instant: Option<DateTime<chrono::FixedOffset>>,
}

impl Rfc3339Timestamp {
    fn parse(text: LanPairingText) -> Self {
        let instant = DateTime::parse_from_rfc3339(&text.0).ok();
        Self { text, instant }
    }

    fn compare_instant(&self, other: &Self) -> Ordering {
        self.instant.cmp(&other.instant)
    }
}

#[derive(Eq, PartialEq)]
struct DiscoveryEventSortKey {
    timestamp: Rfc3339Timestamp,
    event_id: LanPairingText,
}

impl Ord for DiscoveryEventSortKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp
            .compare_instant(&other.timestamp)
            .then_with(|| self.event_id.0.cmp(&other.event_id.0))
            .then_with(|| self.timestamp.text.0.cmp(&other.timestamp.text.0))
    }
}

impl PartialOrd for DiscoveryEventSortKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn replay_discovery_event_history(
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
    generated_at: &LanPairingText,
    has_persisted_projection: bool,
) -> LanDiscoveryEventHistory {
    let mut replay_read_model = read_model.clone();
    replay_read_model.generated_at = generated_at.0.clone();
    let mut rows = Vec::new();
    network_device_events::push_scan_device_event_rows(&mut rows, scan_result);
    canonical_device_events::push_canonical_household_event_rows(
        &mut rows,
        scan_result,
        &replay_read_model,
    );
    let rows = finalize_discovery_event_rows(rows);
    let latest = rows.last();
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.0.clone(),
        state: if has_persisted_projection {
            discovery_event_history_state(scan_result, &rows, &replay_read_model)
        } else {
            LanDiscoveryEventHistoryState::Degraded
        },
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

fn finalize_discovery_event_rows(mut rows: Vec<LanDiscoveryEventRow>) -> Vec<LanDiscoveryEventRow> {
    rows.sort_by_cached_key(|row| DiscoveryEventSortKey {
        timestamp: Rfc3339Timestamp::parse(LanPairingText(row.occurred_at.clone())),
        event_id: LanPairingText(row.event_id.clone()),
    });
    for index in 1..rows.len() {
        let previous_event_id = rows[index - 1].event_id.clone();
        rows[index].previous_event_id = Some(previous_event_id);
    }
    rows
}

pub(super) fn latest_rfc3339_timestamp(
    timestamps: impl Iterator<Item = LanPairingText>,
) -> Option<LanPairingText> {
    timestamps
        .map(Rfc3339Timestamp::parse)
        .max_by(|left, right| {
            left.compare_instant(right)
                .then_with(|| left.text.0.cmp(&right.text.0))
        })
        .map(|timestamp| timestamp.text)
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
