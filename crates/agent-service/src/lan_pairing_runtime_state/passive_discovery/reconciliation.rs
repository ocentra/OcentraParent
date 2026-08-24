use tokio::sync::watch;

use crate::lan_pairing::LanPairingRuntime;
use crate::lan_pairing_browser_add_device_state::physical_lan_scan::refresh_network_device_scan_history_from_passive_runtime;

use super::LanPassiveDiscoveryRefreshSignal;

pub(super) fn spawn(
    runtime: LanPairingRuntime,
    mut refresh_signal: watch::Receiver<Option<LanPassiveDiscoveryRefreshSignal>>,
) {
    tokio::spawn(async move {
        let mut reconciled_sequence = 0_u64;
        while refresh_signal.changed().await.is_ok() {
            let signal = refresh_signal.borrow_and_update().clone();
            let Some(signal) = signal else {
                continue;
            };
            if !signal.is_coherent_after(reconciled_sequence) {
                continue;
            }
            reconciled_sequence = signal.sequence;
            let runtime = runtime.clone();
            let reconciliation = tokio::task::spawn_blocking(move || {
                refresh_network_device_scan_history_from_passive_runtime(&runtime)
            })
            .await;
            if reconciliation.is_err() {
                break;
            }
        }
    });
}
