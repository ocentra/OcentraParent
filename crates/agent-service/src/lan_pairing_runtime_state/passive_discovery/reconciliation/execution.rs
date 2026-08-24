use tokio::time::Instant;

use crate::lan_pairing::LanPairingRuntime;
use crate::lan_pairing_browser_add_device_state::physical_lan_scan::{
    refresh_network_device_scan_history_from_passive_runtime_with_cancellation,
    LanNetworkDeviceScanResult,
};

use super::super::pipeline_health::LanPassiveDiscoveryPipelineIssue;
use super::super::PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL;
use super::PassiveDiscoveryReconciliationRuntime;

impl PassiveDiscoveryReconciliationRuntime {
    pub(super) async fn reconcile(&mut self) {
        let attempted_at = Instant::now();
        self.last_attempt_at = Some(attempted_at);
        let runtime = self.runtime.clone();
        let stop = self.stop.clone();
        let reconciliation = tokio::task::spawn_blocking(move || {
            refresh_network_device_scan_history_from_passive_runtime_with_cancellation(
                &runtime, &stop,
            )
        })
        .await;

        let pipeline_health = self.pipeline_health.clone();
        let capability_store = self.capability_store.clone();
        let persisted_reconciliation = tokio::task::spawn_blocking(move || {
            let issue = reconciliation_issue(reconciliation);
            if let Some(issue) = issue {
                pipeline_health
                    .record_failure(issue, PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL);
                let persisted = capability_store.save_pipeline_health(&pipeline_health.snapshot());
                return (false, persisted);
            }
            pipeline_health.record_success();
            let persisted = capability_store.save_pipeline_health(&pipeline_health.snapshot());
            (true, persisted)
        })
        .await;

        let Ok((reconciled, _persisted)) = persisted_reconciliation else {
            self.retry_pending = true;
            self.schedule_automatic_refresh(
                attempted_at + PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL,
            );
            return;
        };
        if reconciled {
            self.retry_pending = false;
            self.automatic_refresh_at = None;
        } else {
            self.retry_pending = true;
            self.schedule_automatic_refresh(
                attempted_at + PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL,
            );
        }
    }

    pub(super) async fn record_signal_channel_closed(&self) {
        let runtime = self.runtime.clone();
        let pipeline_health = self.pipeline_health.clone();
        let capability_store = self.capability_store.clone();
        let _result = tokio::task::spawn_blocking(move || {
            if listener_is_running(&runtime) {
                pipeline_health.record_failure(
                    LanPassiveDiscoveryPipelineIssue::ListenerRuntimeExited,
                    PASSIVE_DISCOVERY_RECONCILIATION_MIN_INTERVAL,
                );
            } else {
                pipeline_health.record_stopped();
            }
            capability_store.save_pipeline_health(&pipeline_health.snapshot())
        })
        .await;
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
