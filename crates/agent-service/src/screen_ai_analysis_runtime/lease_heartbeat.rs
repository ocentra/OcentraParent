use std::{path::PathBuf, time::Duration};

use ocentra_parent_agent_core::{
    journal_crypto::JournalKey, screen_evidence_queue::ScreenEvidenceQueue,
};

use super::{config::ScreenAiAnalysisCycleClock, queue::analysis_lease_expires_at};

const SCREEN_ANALYSIS_LEASE_REFRESH_SECONDS: u64 = 30;

pub(super) struct ScreenAnalysisLeaseHeartbeatInput {
    pub(super) queue_dir: PathBuf,
    pub(super) key: JournalKey,
    pub(super) queue_job_id: String,
    pub(super) adapter_timeout_ms: u64,
}

pub(super) struct ScreenAnalysisLeaseHeartbeat {
    stop: Option<tokio::sync::oneshot::Sender<()>>,
}

pub(super) fn start_analysis_lease_heartbeat(
    input: ScreenAnalysisLeaseHeartbeatInput,
) -> ScreenAnalysisLeaseHeartbeat {
    let (stop, stopped) = tokio::sync::oneshot::channel();
    tokio::spawn(renew_until_stopped(input, stopped));
    ScreenAnalysisLeaseHeartbeat { stop: Some(stop) }
}

async fn renew_until_stopped(
    input: ScreenAnalysisLeaseHeartbeatInput,
    mut stopped: tokio::sync::oneshot::Receiver<()>,
) {
    let mut interval =
        tokio::time::interval(Duration::from_secs(SCREEN_ANALYSIS_LEASE_REFRESH_SECONDS));
    loop {
        tokio::select! {
            _ = &mut stopped => break,
            _ = interval.tick() => if !renew_analysis_lease(&input) {
                break;
            }
        }
    }
}

fn renew_analysis_lease(input: &ScreenAnalysisLeaseHeartbeatInput) -> bool {
    let clock = ScreenAiAnalysisCycleClock {
        epoch_seconds: 0,
        timestamp: crate::time::timestamp_now(),
    };
    let Ok(lease_expires_at) = analysis_lease_expires_at(&clock, input.adapter_timeout_ms) else {
        return false;
    };
    let Ok(queue) = ScreenEvidenceQueue::open(&input.queue_dir, input.key.clone()) else {
        return false;
    };
    queue
        .renew_claimed_entry(&input.queue_job_id, &lease_expires_at.0)
        .unwrap_or(false)
}

impl Drop for ScreenAnalysisLeaseHeartbeat {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _ = stop.send(());
        }
    }
}
