use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGameLinuxDockerHostPreflight;
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::app_game_adapter_host_capabilities::HostCapabilitySignals;
use super::app_game_linux_docker_host_preflight::{
    detect_linux_docker_host_preflight, unavailable_linux_docker_host_preflight,
};

const PLATFORM_PROBE_REFRESH_INTERVAL: Duration = Duration::from_secs(300);
// The Docker preflight uses one shared three-second absolute deadline; two
// seconds cover worker wake/join scheduling. A longer stop is an invariant
// violation rather than an ordinary service shutdown path.
const PLATFORM_PROBE_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(5);
const PLATFORM_PROBE_SHUTDOWN_POLL: Duration = Duration::from_millis(5);

#[derive(Clone)]
pub(crate) struct PlatformProbeCache {
    state: Arc<Mutex<PlatformProbeCacheState>>,
}

#[derive(Clone)]
struct PlatformProbeCacheState {
    snapshot: (HostCapabilitySignals, AppGameLinuxDockerHostPreflight),
    last_refresh: Option<Instant>,
    refresh_in_progress: bool,
}

impl PlatformProbeCache {
    pub(crate) fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PlatformProbeCacheState {
                snapshot: unavailable_platform_host_state(),
                last_refresh: None,
                refresh_in_progress: false,
            })),
        }
    }

    pub(super) fn snapshot(&self) -> (HostCapabilitySignals, AppGameLinuxDockerHostPreflight) {
        let Ok(state) = self.state.lock() else {
            return unavailable_platform_host_state();
        };
        state.snapshot.clone()
    }
}

pub(crate) struct PlatformProbeRuntimeOwner {
    cache: PlatformProbeCache,
    cancellation: Arc<AtomicBool>,
    join: Mutex<Option<thread::JoinHandle<()>>>,
}

impl PlatformProbeRuntimeOwner {
    pub(crate) fn start() -> Arc<Self> {
        let cache = PlatformProbeCache::new();
        let cancellation = Arc::new(AtomicBool::new(false));
        let worker_cache = cache.clone();
        let worker_cancellation = cancellation.clone();
        let join = thread::Builder::new()
            .name(proof::PLATFORM_PROBE_THREAD_NAME.to_string())
            .spawn(move || run_server_owned_refresh(worker_cache, worker_cancellation))
            .ok();
        Arc::new(Self {
            cache,
            cancellation,
            join: Mutex::new(join),
        })
    }

    pub(crate) fn cache(&self) -> PlatformProbeCache {
        self.cache.clone()
    }
}

impl Drop for PlatformProbeRuntimeOwner {
    fn drop(&mut self) {
        self.cancellation.store(true, Ordering::Release);
        let join = self.join.lock().ok().and_then(|mut join| join.take());
        let Some(join) = join else {
            return;
        };
        join.thread().unpark();
        let shutdown_deadline = Instant::now() + PLATFORM_PROBE_SHUTDOWN_TIMEOUT;
        while !join.is_finished() && Instant::now() < shutdown_deadline {
            thread::sleep(PLATFORM_PROBE_SHUTDOWN_POLL);
        }
        if !join.is_finished() {
            std::process::abort();
        }
        let _joined = join.join();
    }
}

fn run_server_owned_refresh(cache: PlatformProbeCache, cancellation: Arc<AtomicBool>) {
    while !cancellation.load(Ordering::Acquire) {
        refresh_server_owned_cache(cache.clone());
        if cancellation.load(Ordering::Acquire) {
            return;
        }
        thread::park_timeout(PLATFORM_PROBE_REFRESH_INTERVAL);
    }
}

fn refresh_server_owned_cache(cache: PlatformProbeCache) {
    let Ok(mut state) = cache.state.lock() else {
        return;
    };
    let within_rate_limit = state
        .last_refresh
        .is_some_and(|last| last.elapsed() < PLATFORM_PROBE_REFRESH_INTERVAL);
    if within_rate_limit || state.refresh_in_progress {
        return;
    }
    state.refresh_in_progress = true;
    drop(state);

    let snapshot = detect_platform_host_state();
    let Ok(mut state) = cache.state.lock() else {
        return;
    };
    state.snapshot = snapshot;
    state.last_refresh = Some(Instant::now());
    state.refresh_in_progress = false;
}

fn detect_platform_host_state() -> (HostCapabilitySignals, AppGameLinuxDockerHostPreflight) {
    (
        HostCapabilitySignals::detect(),
        detect_linux_docker_host_preflight(),
    )
}

pub(super) fn unavailable_platform_host_state(
) -> (HostCapabilitySignals, AppGameLinuxDockerHostPreflight) {
    (
        HostCapabilitySignals::unavailable(),
        unavailable_linux_docker_host_preflight(),
    )
}
