use std::{path::PathBuf, time::Duration};

use ocentra_parent_agent_core::{
    journal_crypto::JournalKey, screen_evidence_queue::ScreenEvidenceQueue,
};

use super::{config::ScreenAiAnalysisCycleClock, queue::analysis_lease_expires_at};

const SCREEN_ANALYSIS_LEASE_REFRESH_INTERVAL: Duration = Duration::from_secs(30);

pub(crate) struct ScreenAnalysisLeaseHeartbeatInput {
    pub(crate) queue_dir: PathBuf,
    pub(crate) key: JournalKey,
    pub(crate) queue_job_id: String,
    pub(crate) adapter_timeout_ms: u64,
}

pub(crate) struct ScreenAnalysisLeaseHeartbeat {
    stop: Option<tokio::sync::oneshot::Sender<()>>,
}

enum ScreenAnalysisLeaseRenewal {
    Renewed,
    LeaseMissing,
    RetryableFailure,
    InvalidExpiry,
}

pub(crate) fn start_analysis_lease_heartbeat(
    input: ScreenAnalysisLeaseHeartbeatInput,
) -> ScreenAnalysisLeaseHeartbeat {
    start_analysis_lease_heartbeat_with_interval(input, SCREEN_ANALYSIS_LEASE_REFRESH_INTERVAL)
}

pub(crate) fn start_analysis_lease_heartbeat_with_interval(
    input: ScreenAnalysisLeaseHeartbeatInput,
    refresh_interval: Duration,
) -> ScreenAnalysisLeaseHeartbeat {
    let (stop, stopped) = tokio::sync::oneshot::channel();
    tokio::spawn(renew_until_stopped(input, refresh_interval, stopped));
    ScreenAnalysisLeaseHeartbeat { stop: Some(stop) }
}

async fn renew_until_stopped(
    input: ScreenAnalysisLeaseHeartbeatInput,
    refresh_interval: Duration,
    mut stopped: tokio::sync::oneshot::Receiver<()>,
) {
    let mut interval = tokio::time::interval(refresh_interval);
    while wait_for_renewal_or_stop(&input, &mut interval, &mut stopped).await {}
}

async fn wait_for_renewal_or_stop(
    input: &ScreenAnalysisLeaseHeartbeatInput,
    interval: &mut tokio::time::Interval,
    stopped: &mut tokio::sync::oneshot::Receiver<()>,
) -> bool {
    tokio::select! {
        _ = stopped => false,
        _ = interval.tick() => renewal_should_continue(&renew_analysis_lease(input)),
    }
}

fn renewal_should_continue(renewal: &ScreenAnalysisLeaseRenewal) -> bool {
    matches!(
        renewal,
        ScreenAnalysisLeaseRenewal::Renewed | ScreenAnalysisLeaseRenewal::RetryableFailure
    )
}

fn renew_analysis_lease(input: &ScreenAnalysisLeaseHeartbeatInput) -> ScreenAnalysisLeaseRenewal {
    let clock = ScreenAiAnalysisCycleClock {
        epoch_seconds: 0,
        timestamp: crate::time::timestamp_now(),
    };
    let Ok(lease_expires_at) = analysis_lease_expires_at(&clock, input.adapter_timeout_ms) else {
        return ScreenAnalysisLeaseRenewal::InvalidExpiry;
    };
    let Ok(queue) = ScreenEvidenceQueue::open(&input.queue_dir, input.key.clone()) else {
        return ScreenAnalysisLeaseRenewal::RetryableFailure;
    };
    match queue.renew_claimed_entry(&input.queue_job_id, &lease_expires_at.0) {
        Ok(true) => ScreenAnalysisLeaseRenewal::Renewed,
        Ok(false) => ScreenAnalysisLeaseRenewal::LeaseMissing,
        Err(_) => ScreenAnalysisLeaseRenewal::RetryableFailure,
    }
}

impl Drop for ScreenAnalysisLeaseHeartbeat {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
    }
}
