use std::sync::{Mutex, OnceLock};

use chrono::{DateTime, Utc};
use ocentra_lan_core::network_inventory::{
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer,
    plan_lan_discovery_scan_with_active_refresh_suppression,
    targeted_arp_refresh_evidence_for_scan, LanDiscoveryRefreshMode, LanNetworkInventoryDevice,
};
use ocentra_parent_agent_protocol::lan_pairing::{LanPairingDeviceRef, LanPairingText};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentRoute,
};

use crate::lan_pairing::LanPairingRuntime;

use super::scan_history::{
    load_scan_history_snapshot, save_scan_history, scan_history_is_recent, LanScanHistoryMetadata,
    LanScanHistorySnapshot,
};

#[path = "physical_lan_scan/execution_lease.rs"]
mod execution_lease;
#[path = "physical_lan_scan/persisted_result.rs"]
mod persisted_result;
#[path = "physical_lan_scan/scan_truth.rs"]
mod scan_truth;
#[path = "physical_lan_scan/suppression_device.rs"]
mod suppression_device;

use self::persisted_result::persisted_scan_result_or_fail;
use self::suppression_device::scan_session_id;

static PHYSICAL_LAN_SCAN_EXECUTION_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct LanNetworkDeviceScanResult {
    pub(crate) devices: Vec<LanNetworkInventoryDevice>,
    pub(crate) previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    pub(crate) current_scan_snapshot: Option<LanScanHistorySnapshot>,
    pub(crate) reused_recent_snapshot: bool,
}

pub(crate) fn network_device_scan_result_for_command(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> LanNetworkDeviceScanResult {
    let now = Utc::now();
    let previous_scan_snapshot = (!command_uses_physical_lan_scan(&command.command)
        || command.target.route == AgentRoute::Localhost)
        .then(|| load_scan_history_snapshot(runtime))
        .flatten();
    if let Some(scan_result) =
        cached_scan_result_for_command(command, previous_scan_snapshot.clone(), now)
    {
        return scan_result;
    }

    if command_uses_physical_lan_scan(&command.command) {
        if let Some(previous_devices) =
            cached_status_snapshot_devices(command, previous_scan_snapshot.as_ref(), now)
        {
            return LanNetworkDeviceScanResult {
                devices: previous_devices,
                current_scan_snapshot: previous_scan_snapshot.clone(),
                previous_scan_snapshot,
                reused_recent_snapshot: true,
            };
        }
        let refresh_mode = refresh_mode_for_command(&command.command);
        return execute_physical_lan_scan(runtime, previous_scan_snapshot, now, refresh_mode);
    }
    LanNetworkDeviceScanResult::default()
}

fn cached_scan_result_for_command(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<LanNetworkDeviceScanResult> {
    cached_localhost_status_scan_result(command, previous_scan_snapshot.clone(), now)
        .or_else(|| cached_runtime_event_stream_scan_result(command, previous_scan_snapshot, now))
}

pub(crate) fn cached_runtime_event_stream_scan_result(
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

pub(super) fn refresh_network_device_scan_history(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) -> LanNetworkDeviceScanResult {
    network_device_scan_result_for_command(runtime, command)
}

pub(crate) fn refresh_network_device_scan_history_from_passive_runtime(
    runtime: &LanPairingRuntime,
) -> LanNetworkDeviceScanResult {
    execute_physical_lan_scan(runtime, None, Utc::now(), LanDiscoveryRefreshMode::Passive)
}

fn execute_physical_lan_scan(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
    refresh_mode: LanDiscoveryRefreshMode,
) -> LanNetworkDeviceScanResult {
    let Ok(_execution_guard) = physical_lan_scan_execution_lock().lock() else {
        return failed_scan_result(previous_scan_snapshot);
    };
    let Some(_cross_process_execution_lease) = execution_lease::acquire(runtime) else {
        return failed_scan_result(previous_scan_snapshot);
    };
    let previous_scan_snapshot = load_scan_history_snapshot(runtime).or(previous_scan_snapshot);
    execute_physical_lan_scan_locked(runtime, previous_scan_snapshot, now, refresh_mode)
}

fn execute_physical_lan_scan_locked(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
    refresh_mode: LanDiscoveryRefreshMode,
) -> LanNetworkDeviceScanResult {
    let previous_devices = previous_scan_snapshot
        .as_ref()
        .map(|snapshot| snapshot.devices.as_slice())
        .unwrap_or_default();
    let scan_truth = scan_truth_context(runtime, previous_scan_snapshot.as_ref(), now);
    let mut scan_plan = plan_lan_discovery_scan_with_active_refresh_suppression(
        &scan_truth.identity_hint_devices,
        previous_devices,
        refresh_mode,
        &scan_truth.scan_suppression_devices,
    );
    scan_plan.targeted_arp_refresh_evidence = targeted_arp_refresh_evidence_for_scan(
        previous_devices,
        refresh_mode,
        &scan_truth.scan_suppression_devices,
    );
    let inventory_refresh_mode = inventory_refresh_mode_after_targeted_refresh(refresh_mode);
    let selected_interface_scope = scan_plan.selected_interface.as_deref();
    let allowed_snmp_response_observer = |payload: &[u8]| {
        let _ = runtime.record_allowed_snmp_probe_response_packet(payload);
    };
    let devices = discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer(
        &scan_truth.identity_hint_devices,
        previous_devices,
        inventory_refresh_mode,
        &scan_truth.scan_suppression_devices,
        &scan_truth.scan_suppression_devices,
        selected_interface_scope,
        Some(&allowed_snmp_response_observer),
    );
    persisted_scan_result_or_fail(
        runtime,
        devices,
        LanScanHistoryMetadata {
            scan_id: scan_session_id(now).0,
            paired_registry_truth_count: scan_truth.paired_registry_truth_count,
            recent_previous_agent_truth_count: scan_truth.recent_previous_agent_truth_count,
            durable_household_truth_count: scan_truth.durable_household_truth_count,
            scan_plan,
        },
        previous_scan_snapshot,
    )
}

fn failed_scan_result(
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

fn physical_lan_scan_execution_lock() -> &'static Mutex<()> {
    PHYSICAL_LAN_SCAN_EXECUTION_LOCK.get_or_init(|| Mutex::new(()))
}

pub(crate) fn cached_localhost_status_scan_result(
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

pub(crate) fn cached_status_snapshot_devices(
    command: &AgentCommandEnvelope,
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> Option<Vec<LanNetworkInventoryDevice>> {
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

pub(crate) struct LanScanTruthContext {
    pub(crate) identity_hint_devices: Vec<LanPairingDeviceRef>,
    pub(crate) scan_suppression_devices: Vec<LanPairingDeviceRef>,
    pub(crate) paired_registry_truth_count: u32,
    pub(crate) recent_previous_agent_truth_count: u32,
    pub(crate) durable_household_truth_count: u32,
}

pub(crate) fn scan_truth_context(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<&LanScanHistorySnapshot>,
    now: DateTime<Utc>,
) -> LanScanTruthContext {
    scan_truth::scan_truth_context(runtime, previous_scan_snapshot, now)
}

pub(crate) fn command_uses_physical_lan_scan(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentLanPairingStatusGet
            | AgentCommandName::AgentLanPairingBrowserDiscoveryScan
            | AgentCommandName::AgentLanPairingAddDeviceRequest
    )
}

pub(crate) fn refresh_mode_for_command(command: &AgentCommandName) -> LanDiscoveryRefreshMode {
    match command {
        AgentCommandName::AgentLanPairingBrowserDiscoveryScan
        | AgentCommandName::AgentLanPairingAddDeviceRequest => {
            LanDiscoveryRefreshMode::ActiveSubnetRefresh
        }
        AgentCommandName::AgentLanPairingStatusGet => LanDiscoveryRefreshMode::Passive,
        _ => LanDiscoveryRefreshMode::Passive,
    }
}

pub(crate) fn inventory_refresh_mode_after_targeted_refresh(
    refresh_mode: LanDiscoveryRefreshMode,
) -> LanDiscoveryRefreshMode {
    if refresh_mode == LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        LanDiscoveryRefreshMode::Passive
    } else {
        refresh_mode
    }
}
