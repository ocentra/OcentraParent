use tokio::time::Instant;

use ocentra_lan_core::network_inventory::passive_discovery::LanPassiveDiscoveryTriggerReason;

use super::super::PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL;
use super::PassiveDiscoveryReconciliationRuntime;

impl PassiveDiscoveryReconciliationRuntime {
    pub(super) async fn handle_latest_signal(&mut self, deliberate: bool) {
        let signal = if deliberate {
            self.refresh_signal.deliberate.borrow_and_update().clone()
        } else {
            self.refresh_signal.passive.borrow_and_update().clone()
        };
        let Some(signal) = signal else {
            return;
        };
        if !signal.is_coherent_after(self.observed_sequence) {
            return;
        }
        self.observed_sequence = signal.sequence;

        if signal.trigger_reason != LanPassiveDiscoveryTriggerReason::PassivePacketObserved {
            self.pending_passive_refresh = false;
            self.retry_pending = false;
            self.automatic_refresh_at = None;
            self.reconcile().await;
            return;
        }

        let next_allowed = self
            .last_attempt_at
            .map(|attempted_at| attempted_at + PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL);
        let Some(next_allowed) = next_allowed else {
            self.pending_passive_refresh = false;
            self.reconcile().await;
            return;
        };
        if Instant::now() >= next_allowed {
            self.pending_passive_refresh = false;
            self.reconcile().await;
            return;
        }
        self.pending_passive_refresh = true;
        self.schedule_automatic_refresh(next_allowed);
    }

    pub(super) async fn handle_automatic_refresh(&mut self) {
        self.automatic_refresh_at = None;
        if !self.pending_passive_refresh && !self.retry_pending {
            return;
        }
        self.pending_passive_refresh = false;
        self.retry_pending = false;
        self.reconcile().await;
    }

    pub(super) fn schedule_automatic_refresh(&mut self, refresh_at: Instant) {
        self.automatic_refresh_at = Some(
            self.automatic_refresh_at
                .map(|scheduled| scheduled.min(refresh_at))
                .unwrap_or(refresh_at),
        );
    }
}
