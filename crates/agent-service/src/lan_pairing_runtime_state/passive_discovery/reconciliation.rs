use std::{
    io,
    sync::{atomic::AtomicBool, Arc},
    thread::JoinHandle,
    time::Duration,
};

use tokio::time::Instant;

use crate::lan_pairing::LanPairingRuntime;

use super::capability_store::LanPassiveDiscoveryCapabilityStore;
use super::pipeline_health::LanPassiveDiscoveryPipelineHealth;
use super::LanPassiveDiscoveryRefreshSignalReceiver;

#[path = "reconciliation/execution.rs"]
mod execution;
#[path = "reconciliation/signal.rs"]
mod signal;
#[path = "reconciliation/wake.rs"]
mod wake;

use self::wake::ReconciliationWake;

const PASSIVE_DISCOVERY_RECONCILIATION_THREAD_NAME: &str = "lan-passive-discovery-reconciliation";

struct PassiveDiscoveryReconciliationRuntime {
    runtime: LanPairingRuntime,
    refresh_signal: LanPassiveDiscoveryRefreshSignalReceiver,
    pipeline_health: LanPassiveDiscoveryPipelineHealth,
    capability_store: LanPassiveDiscoveryCapabilityStore,
    observed_sequence: u64,
    last_attempt_at: Option<Instant>,
    pending_passive_refresh: bool,
    retry_pending: bool,
    automatic_refresh_at: Option<Instant>,
    stop: Arc<AtomicBool>,
}

pub(super) fn spawn(
    runtime: LanPairingRuntime,
    refresh_signal: LanPassiveDiscoveryRefreshSignalReceiver,
    pipeline_health: LanPassiveDiscoveryPipelineHealth,
    stop: Arc<AtomicBool>,
) -> io::Result<JoinHandle<()>> {
    let stop_for_runtime = stop.clone();
    let capability_store = LanPassiveDiscoveryCapabilityStore::for_runtime(&runtime);
    let async_runtime = tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()?;
    std::thread::Builder::new()
        .name(PASSIVE_DISCOVERY_RECONCILIATION_THREAD_NAME.to_string())
        .spawn(move || {
            async_runtime.block_on(
                PassiveDiscoveryReconciliationRuntime {
                    runtime,
                    refresh_signal,
                    pipeline_health,
                    capability_store,
                    observed_sequence: 0,
                    last_attempt_at: None,
                    pending_passive_refresh: false,
                    retry_pending: false,
                    automatic_refresh_at: None,
                    stop: stop_for_runtime,
                }
                .run(),
            );
        })
}

pub(super) const RECONCILIATION_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);

impl PassiveDiscoveryReconciliationRuntime {
    async fn run(mut self) {
        while !self.stop.load(std::sync::atomic::Ordering::Acquire) && self.run_once().await {}
    }

    async fn run_once(&mut self) -> bool {
        if self.stop.load(std::sync::atomic::Ordering::Acquire) {
            return false;
        }
        let wake = wake::next(&mut self.refresh_signal, self.automatic_refresh_at).await;

        match wake {
            ReconciliationWake::DeliberateSignal(true) => {
                self.handle_latest_signal(true).await;
            }
            ReconciliationWake::PassiveSignal(true) => {
                self.handle_latest_signal(false).await;
            }
            ReconciliationWake::DeliberateSignal(false)
            | ReconciliationWake::PassiveSignal(false) => {
                self.record_signal_channel_closed().await;
                return false;
            }
            ReconciliationWake::AutomaticRefresh => self.handle_automatic_refresh().await,
        }
        true
    }
}
