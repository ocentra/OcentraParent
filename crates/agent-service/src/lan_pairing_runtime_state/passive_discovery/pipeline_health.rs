use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::time::timestamp_now;

#[derive(Clone, Debug)]
pub(super) struct LanPassiveDiscoveryPipelineHealth {
    state: Arc<Mutex<LanPassiveDiscoveryPipelineHealthSnapshot>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct LanPassiveDiscoveryPipelineHealthSnapshot {
    pub(super) state: LanPassiveDiscoveryPipelineState,
    pub(super) consecutive_failures: u32,
    pub(super) retry_delay_millis: Option<u64>,
    pub(super) issue: Option<LanPassiveDiscoveryPipelineIssue>,
    pub(super) last_succeeded_at: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum LanPassiveDiscoveryPipelineState {
    Starting,
    Healthy,
    RetryScheduled,
    Stopped,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(super) enum LanPassiveDiscoveryPipelineIssue {
    ScanHistoryPersistenceFailed,
    ReconciliationJoinFailed,
    ListenerRuntimeExited,
    PipelineHealthStateUnavailable,
}

impl LanPassiveDiscoveryPipelineHealth {
    pub(super) fn starting() -> Self {
        Self {
            state: Arc::new(Mutex::new(
                LanPassiveDiscoveryPipelineHealthSnapshot::starting(),
            )),
        }
    }

    pub(super) fn snapshot(&self) -> LanPassiveDiscoveryPipelineHealthSnapshot {
        self.state
            .lock()
            .map(|state| state.clone())
            .unwrap_or_else(|_| LanPassiveDiscoveryPipelineHealthSnapshot::unavailable())
    }

    pub(super) fn record_success(&self) {
        if let Ok(mut state) = self.state.lock() {
            *state = LanPassiveDiscoveryPipelineHealthSnapshot {
                state: LanPassiveDiscoveryPipelineState::Healthy,
                consecutive_failures: 0,
                retry_delay_millis: None,
                issue: None,
                last_succeeded_at: Some(timestamp_now()),
            };
        }
    }

    pub(super) fn record_failure(
        &self,
        issue: LanPassiveDiscoveryPipelineIssue,
        retry_delay: Duration,
    ) {
        if let Ok(mut state) = self.state.lock() {
            let last_succeeded_at = state.last_succeeded_at.clone();
            *state = LanPassiveDiscoveryPipelineHealthSnapshot {
                state: LanPassiveDiscoveryPipelineState::RetryScheduled,
                consecutive_failures: state.consecutive_failures.saturating_add(1),
                retry_delay_millis: Some(duration_millis(retry_delay)),
                issue: Some(issue),
                last_succeeded_at,
            };
        }
    }

    pub(super) fn record_stopped(&self) {
        if let Ok(mut state) = self.state.lock() {
            state.state = LanPassiveDiscoveryPipelineState::Stopped;
            state.consecutive_failures = 0;
            state.retry_delay_millis = None;
            state.issue = None;
        }
    }
}

impl LanPassiveDiscoveryPipelineHealthSnapshot {
    pub(super) fn starting() -> Self {
        Self {
            state: LanPassiveDiscoveryPipelineState::Starting,
            consecutive_failures: 0,
            retry_delay_millis: None,
            issue: None,
            last_succeeded_at: None,
        }
    }

    pub(super) fn unavailable() -> Self {
        Self {
            state: LanPassiveDiscoveryPipelineState::RetryScheduled,
            consecutive_failures: 1,
            retry_delay_millis: Some(0),
            issue: Some(LanPassiveDiscoveryPipelineIssue::PipelineHealthStateUnavailable),
            last_succeeded_at: None,
        }
    }
}

fn duration_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}
