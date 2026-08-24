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
}

pub(super) fn spawn(
    runtime: LanPairingRuntime,
    refresh_signal: LanPassiveDiscoveryRefreshSignalReceiver,
    pipeline_health: LanPassiveDiscoveryPipelineHealth,
) {
    let capability_store = LanPassiveDiscoveryCapabilityStore::for_runtime(&runtime);
    tokio::spawn(async move {
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
        }
        .run()
        .await;
    });
}

impl PassiveDiscoveryReconciliationRuntime {
    async fn run(mut self) {
        while self.run_once().await {}
    }

    async fn run_once(&mut self) -> bool {
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
                self.record_signal_channel_closed();
                return false;
            }
            ReconciliationWake::AutomaticRefresh => self.handle_automatic_refresh().await,
        }
        true
    }
}
