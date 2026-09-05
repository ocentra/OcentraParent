use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGameLinuxDockerHostPreflight;

use super::app_game_linux_docker_host_preflight::unavailable_linux_docker_host_preflight;

#[derive(Clone)]
pub(crate) struct PlatformProbeCache {
    state: Arc<Mutex<PlatformProbeCacheState>>,
}

#[derive(Clone)]
struct PlatformProbeCacheState {
    snapshot: AppGameLinuxDockerHostPreflight,
    last_refresh: Option<Instant>,
    refresh_in_progress: bool,
}

impl PlatformProbeCache {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PlatformProbeCacheState {
                snapshot: unavailable_linux_docker_host_preflight(),
                last_refresh: None,
                refresh_in_progress: false,
            })),
        }
    }

    pub(super) fn snapshot(&self) -> AppGameLinuxDockerHostPreflight {
        let Ok(state) = self.state.lock() else {
            return unavailable_linux_docker_host_preflight();
        };
        state.snapshot.clone()
    }

    pub(super) fn begin_refresh(&self, minimum_interval: Duration) -> bool {
        let Ok(mut state) = self.state.lock() else {
            return false;
        };
        let within_rate_limit = state
            .last_refresh
            .is_some_and(|last| last.elapsed() < minimum_interval);
        if within_rate_limit || state.refresh_in_progress {
            return false;
        }
        state.refresh_in_progress = true;
        true
    }

    pub(super) fn finish_refresh(&self, snapshot: AppGameLinuxDockerHostPreflight) {
        let Ok(mut state) = self.state.lock() else {
            return;
        };
        state.snapshot = snapshot;
        state.last_refresh = Some(Instant::now());
        state.refresh_in_progress = false;
    }
}
