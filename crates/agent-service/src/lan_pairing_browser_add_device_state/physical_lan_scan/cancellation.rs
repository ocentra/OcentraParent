use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard,
    },
    thread,
    time::{Duration, Instant},
};

use chrono::{DateTime, Utc};
use ocentra_lan_core::network_inventory::LanDiscoveryRefreshMode;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::lan_pairing::LanPairingRuntime;

use super::super::scan_history::{
    load_scan_history_snapshot, scan_history_is_recent, LanScanHistorySnapshot,
};
use super::cache::{cached_scan_result_for_command, cached_status_snapshot_devices};
use super::{
    command_uses_physical_lan_scan, failed_scan_result, refresh_mode_for_command,
    LanNetworkDeviceScanResult,
};

pub(crate) fn refresh_network_device_scan_history_with_cancellation(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    cancellation: &AtomicBool,
) -> LanNetworkDeviceScanResult {
    let now = Utc::now();
    let previous_scan_snapshot = load_scan_history_snapshot(runtime);
    if cancellation.load(Ordering::Acquire) {
        return failed_scan_result(previous_scan_snapshot);
    }
    if let Some(scan_result) =
        cached_scan_result_for_command(command, previous_scan_snapshot.clone(), now)
    {
        return scan_result;
    }
    if !command_uses_physical_lan_scan(&command.command) {
        return LanNetworkDeviceScanResult::default();
    }
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
    execute_physical_lan_scan(
        runtime,
        previous_scan_snapshot,
        now,
        refresh_mode_for_command(&command.command),
        Some(cancellation),
    )
}

pub(super) fn execute_physical_lan_scan(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
    refresh_mode: LanDiscoveryRefreshMode,
    cancellation: Option<&AtomicBool>,
) -> LanNetworkDeviceScanResult {
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
        return failed_scan_result(previous_scan_snapshot);
    }
    // Never wait indefinitely behind another active scan. A cancelled
    // reconciliation must be able to return even when a foreground scan is
    // still finishing its bounded network work.
    const EXECUTION_LOCK_WAIT: Duration = Duration::from_secs(5);
    let Some(_execution_guard) = acquire_execution_guard(
        super::physical_lan_scan_execution_lock(),
        cancellation,
        EXECUTION_LOCK_WAIT,
    ) else {
        return failed_scan_result(previous_scan_snapshot);
    };
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
        return failed_scan_result(previous_scan_snapshot);
    }
    let Some(_cross_process_execution_lease) = super::execution_lease::acquire(runtime) else {
        return failed_scan_result(previous_scan_snapshot);
    };
    let previous_scan_snapshot = load_scan_history_snapshot(runtime).or(previous_scan_snapshot);
    let previous_scan_snapshot = previous_scan_snapshot.filter(|snapshot| {
        scan_history_is_recent(&LanPairingText(snapshot.updated_at.clone()), now)
    });
    super::execute_physical_lan_scan_locked(
        runtime,
        previous_scan_snapshot,
        now,
        refresh_mode,
        cancellation,
    )
}

fn acquire_execution_guard<'a>(
    lock: &'a Mutex<()>,
    cancellation: Option<&AtomicBool>,
    timeout: Duration,
) -> Option<MutexGuard<'a, ()>> {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if let Ok(guard) = lock.try_lock() {
            return Some(guard);
        }
        if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
            return None;
        }
        thread::sleep(Duration::from_millis(10));
    }
    None
}
