use tokio::time::Instant;

use crate::lan_pairing::LanPairingRuntime;
use crate::lan_pairing_browser_add_device_state::physical_lan_scan::{
    refresh_network_device_scan_history_from_passive_runtime, LanNetworkDeviceScanResult,
};

use super::super::pipeline_health::LanPassiveDiscoveryPipelineIssue;
use super::super::PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL;
use super::PassiveDiscoveryReconciliationRuntime;

impl PassiveDiscoveryReconciliationRuntime {
    pub(super) async fn reconcile(&mut self) {
        let attempted_at = Instant::now();
        self.last_attempt_at = Some(attempted_at);
        let runtime = self.runtime.clone();
        let reconciliation = tokio::task::spawn_blocking(move || {
            refresh_network_device_scan_history_from_passive_runtime(&runtime)
        })
        .await;

        if let Some(issue) = reconciliation_issue(reconciliation) {
            self.pipeline_health
                .record_failure(issue, PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL);
            self.retry_pending = true;
            self.schedule_automatic_refresh(
                attempted_at + PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL,
            );
        } else {
            self.pipeline_health.record_success();
            self.retry_pending = false;
            self.automatic_refresh_at = None;
        }
        let _persisted = self
            .capability_store
            .save_pipeline_health(&self.pipeline_health.snapshot());
    }

    pub(super) fn record_signal_channel_closed(&self) {
        if listener_is_running(&self.runtime) {
            self.pipeline_health.record_failure(
                LanPassiveDiscoveryPipelineIssue::ListenerRuntimeExited,
                PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL,
            );
        } else {
            self.pipeline_health.record_stopped();
        }
        let _persisted = self
            .capability_store
            .save_pipeline_health(&self.pipeline_health.snapshot());
    }
}

fn reconciliation_issue(
    reconciliation: Result<LanNetworkDeviceScanResult, tokio::task::JoinError>,
) -> Option<LanPassiveDiscoveryPipelineIssue> {
    match reconciliation {
        Ok(result) if result.current_scan_snapshot.is_some() => None,
        Ok(_) => Some(LanPassiveDiscoveryPipelineIssue::ScanHistoryPersistenceFailed),
        Err(_) => Some(LanPassiveDiscoveryPipelineIssue::ReconciliationJoinFailed),
    }
}

fn listener_is_running(runtime: &LanPairingRuntime) -> bool {
    runtime
        .passive_discovery_listener_state
        .lock()
        .map(|state| state.is_running())
        .unwrap_or(false)
}
