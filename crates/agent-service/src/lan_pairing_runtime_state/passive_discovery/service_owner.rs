use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak,
    },
    thread::JoinHandle,
};

use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;

pub(super) struct LanPassiveDiscoveryServiceOwner {
    listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
    listener_join: Mutex<Option<JoinHandle<()>>>,
    reconciliation_stop: Arc<AtomicBool>,
    reconciliation_join: Mutex<Option<tokio::task::JoinHandle<()>>>,
}

impl LanPassiveDiscoveryServiceOwner {
    pub(super) fn new(
        listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
        listener_join: Option<JoinHandle<()>>,
        reconciliation_stop: Arc<AtomicBool>,
        reconciliation_join: tokio::task::JoinHandle<()>,
    ) -> Self {
        Self {
            listener_state,
            listener_join: Mutex::new(listener_join),
            reconciliation_stop,
            reconciliation_join: Mutex::new(Some(reconciliation_join)),
        }
    }
}

impl Drop for LanPassiveDiscoveryServiceOwner {
    fn drop(&mut self) {
        self.reconciliation_stop.store(true, Ordering::Release);
        let reconciliation_join = self
            .reconciliation_join
            .lock()
            .ok()
            .and_then(|mut join| join.take());
        if let Some(reconciliation_join) = reconciliation_join {
            // The async task owns the reconciliation loop. Abort its wake
            // future immediately; any blocking scan it spawned observes the
            // same cancellation flag and has bounded network/lock waits.
            reconciliation_join.abort();
        }
        if let Some(listener_state) = self.listener_state.upgrade() {
            if let Ok(mut listener_state) = listener_state.lock() {
                listener_state.stop();
            }
        }
        let listener_join = self
            .listener_join
            .lock()
            .ok()
            .and_then(|mut join| join.take());
        if let Some(listener_join) = listener_join {
            // The listener uses bounded socket reads and observes the stopped
            // state between every receive cycle. Take the handle before
            // joining so no mutex is held across worker shutdown.
            let _ = listener_join.join();
        }
    }
}
