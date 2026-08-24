use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex, OnceLock,
};
use std::time::Instant;

use chrono::{DateTime, Utc};
use ocentra_lan_core::network_inventory::{
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer_with_cancellation,
    targeted_arp_refresh_evidence_for_scan, LanDiscoveryRefreshMode, LanNetworkInventoryDevice,
};
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentRoute,
};

use crate::lan_pairing::LanPairingRuntime;

use super::scan_history::{
    load_scan_history_snapshot, save_scan_history, LanScanHistoryMetadata, LanScanHistorySnapshot,
};

#[path = "physical_lan_scan/cache.rs"]
mod cache;
#[path = "physical_lan_scan/cancellation.rs"]
pub(crate) mod cancellation;
#[path = "physical_lan_scan/execution_lease.rs"]
mod execution_lease;
#[path = "physical_lan_scan/persisted_result.rs"]
mod persisted_result;
#[path = "physical_lan_scan/planning.rs"]
mod planning;
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
        cache::cached_scan_result_for_command(command, previous_scan_snapshot.clone(), now)
    {
        return scan_result;
    }

    if command_uses_physical_lan_scan(&command.command) {
        if let Some(previous_devices) =
            cache::cached_status_snapshot_devices(command, previous_scan_snapshot.as_ref(), now)
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

pub(crate) fn refresh_network_device_scan_history_from_passive_runtime_with_cancellation(
    runtime: &LanPairingRuntime,
    cancellation: &AtomicBool,
) -> LanNetworkDeviceScanResult {
    cancellation::execute_passive_reconciliation_scan(runtime, cancellation)
}

fn execute_physical_lan_scan(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
    refresh_mode: LanDiscoveryRefreshMode,
) -> LanNetworkDeviceScanResult {
    cancellation::execute_physical_lan_scan(
        runtime,
        previous_scan_snapshot,
        now,
        refresh_mode,
        None,
        None,
    )
}

fn execute_physical_lan_scan_locked(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
    refresh_mode: LanDiscoveryRefreshMode,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) -> LanNetworkDeviceScanResult {
    if cancellation.is_some_and(|cancellation| cancellation.load(Ordering::Acquire)) {
        return failed_scan_result(previous_scan_snapshot);
    }
    let previous_devices = previous_scan_snapshot
        .as_ref()
        .map(|snapshot| snapshot.devices.as_slice())
        .unwrap_or_default();
    let scan_truth = scan_truth_context(runtime, previous_scan_snapshot.as_ref(), now);
    let Some(mut scan_plan) = planning::plan_for_scan(
        &scan_truth.identity_hint_devices,
        previous_devices,
        refresh_mode,
        &scan_truth.scan_suppression_devices,
        deadline,
        cancellation,
    ) else {
        return failed_scan_result(previous_scan_snapshot);
    };
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
    let devices = discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer_with_cancellation(
        &scan_truth.identity_hint_devices,
        previous_devices,
        inventory_refresh_mode,
        &scan_truth.scan_suppression_devices,
        &scan_truth.scan_suppression_devices,
        selected_interface_scope,
        Some(&allowed_snmp_response_observer),
        cancellation,
        deadline,
    );
    if cancellation.is_some_and(|cancellation| cancellation.load(Ordering::Acquire)) {
        return failed_scan_result(previous_scan_snapshot);
    }
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
        // A failed or cancelled scan has no current physical-LAN authority.
        // Preserve the prior snapshot for bounded continuity hints only.
        devices: Vec::new(),
        current_scan_snapshot: None,
        previous_scan_snapshot,
        reused_recent_snapshot: false,
    }
}

fn physical_lan_scan_execution_lock() -> &'static Mutex<()> {
    PHYSICAL_LAN_SCAN_EXECUTION_LOCK.get_or_init(|| Mutex::new(()))
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
