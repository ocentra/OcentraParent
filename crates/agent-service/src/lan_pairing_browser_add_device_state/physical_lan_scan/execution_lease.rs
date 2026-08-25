use crate::lan_pairing::LanPairingRuntime;

use super::super::scan_history::{scan_history_execution_lock, write_lock::CrossProcessPathLock};

pub(super) struct ScanExecutionLease {
    _cross_process_lock: CrossProcessPathLock,
}

pub(super) fn acquire(runtime: &LanPairingRuntime) -> Option<ScanExecutionLease> {
    scan_history_execution_lock(runtime).map(|_cross_process_lock| ScanExecutionLease {
        _cross_process_lock,
    })
}
