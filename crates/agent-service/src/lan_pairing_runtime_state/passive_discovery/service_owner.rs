use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex, Weak,
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryListenerState;

use super::reconciliation::RECONCILIATION_SHUTDOWN_TIMEOUT;

pub(super) struct LanPassiveDiscoveryServiceOwner {
    listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
    listener_join: Mutex<Option<JoinHandle<()>>>,
    reconciliation_stop: Arc<AtomicBool>,
    reconciliation_join: Mutex<Option<JoinHandle<()>>>,
}

impl LanPassiveDiscoveryServiceOwner {
    pub(super) fn new(
        listener_state: Weak<Mutex<LanPassiveDiscoveryListenerState>>,
        listener_join: Option<JoinHandle<()>>,
        reconciliation_stop: Arc<AtomicBool>,
        reconciliation_join: JoinHandle<()>,
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
        let reconciliation_join = self
            .reconciliation_join
            .lock()
            .ok()
            .and_then(|mut join| join.take());
        let shutdown_deadline = Instant::now() + RECONCILIATION_SHUTDOWN_TIMEOUT;
        while Instant::now() < shutdown_deadline
            && (!is_finished(&listener_join) || !is_finished(&reconciliation_join))
        {
            thread::sleep(Duration::from_millis(5));
        }

        // Every listener read and reconciliation scan receives the same owner
        // cancellation and has an inner deadline shorter than this bound. If
        // either worker violates that invariant, abort the process rather than
        // detach it and drop storage/runtime ownership underneath live work.
        if !is_finished(&listener_join) || !is_finished(&reconciliation_join) {
            std::process::abort();
        }
        if let Some(join) = reconciliation_join {
            let _joined = join.join();
        }
        if let Some(join) = listener_join {
            let _joined = join.join();
        }
    }
}

fn is_finished(join: &Option<JoinHandle<()>>) -> bool {
    join.as_ref().map(JoinHandle::is_finished).unwrap_or(true)
}
