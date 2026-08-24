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

const PASSIVE_RECONCILIATION_SCAN_BUDGET: Duration = Duration::from_millis(3_500);

#[path = "cancellation/watchdog.rs"]
mod watchdog;

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
        None,
    )
}

pub(super) fn execute_passive_reconciliation_scan(
    runtime: &LanPairingRuntime,
    external_cancellation: &AtomicBool,
) -> LanNetworkDeviceScanResult {
    let deadline = Instant::now() + PASSIVE_RECONCILIATION_SCAN_BUDGET;
    let scan_cancellation = AtomicBool::new(external_cancellation.load(Ordering::Acquire));
    let finished = AtomicBool::new(false);
    thread::scope(|scope| {
        let watcher = scope.spawn(|| {
            watchdog::wait(
                deadline,
                external_cancellation,
                &scan_cancellation,
                &finished,
            );
        });
        let result = execute_physical_lan_scan(
            runtime,
            None,
            Utc::now(),
            LanDiscoveryRefreshMode::Passive,
            Some(&scan_cancellation),
            Some(deadline),
        );
        finished.store(true, Ordering::Release);
        scan_cancellation.store(true, Ordering::Release);
        let _watcher_joined = watcher.join();
        result
    })
}

pub(super) fn execute_physical_lan_scan(
    runtime: &LanPairingRuntime,
    previous_scan_snapshot: Option<LanScanHistorySnapshot>,
    now: DateTime<Utc>,
    refresh_mode: LanDiscoveryRefreshMode,
    cancellation: Option<&AtomicBool>,
    deadline: Option<Instant>,
) -> LanNetworkDeviceScanResult {
    if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
        return failed_scan_result(previous_scan_snapshot);
    }
    // Never wait indefinitely behind another active scan. A cancelled
    // reconciliation must be able to return even when a foreground scan is
    // still finishing its bounded network work.
    const EXECUTION_LOCK_WAIT: Duration = Duration::from_secs(5);
    let lock_deadline = deadline.unwrap_or_else(|| Instant::now() + EXECUTION_LOCK_WAIT);
    let Some(_execution_guard) = acquire_execution_guard(
        super::physical_lan_scan_execution_lock(),
        cancellation,
        lock_deadline,
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
        deadline,
    )
}

fn acquire_execution_guard<'a>(
    lock: &'a Mutex<()>,
    cancellation: Option<&AtomicBool>,
    deadline: Instant,
) -> Option<MutexGuard<'a, ()>> {
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
