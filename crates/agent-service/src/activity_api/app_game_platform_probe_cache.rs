use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::{Duration, Instant};

use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGameLinuxDockerHostPreflight;
use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::app_game_linux_docker_host_preflight::{
    detect_linux_docker_host_preflight, unavailable_linux_docker_host_preflight,
};
use super::app_game_linux_docker_host_preflight_cleanup_owner::CleanupWorkerRegistry;

const PLATFORM_PROBE_REFRESH_INTERVAL: Duration = Duration::from_secs(300);

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
}

pub(crate) struct PlatformProbeRuntimeOwner {
    cache: PlatformProbeCache,
    cancellation: Arc<AtomicBool>,
    join: Mutex<Option<thread::JoinHandle<()>>>,
    _cleanup_workers: CleanupWorkerRegistry,
}

impl PlatformProbeRuntimeOwner {
    pub(crate) fn start() -> Arc<Self> {
        let cache = PlatformProbeCache::new();
        let cancellation = Arc::new(AtomicBool::new(false));
        let cleanup_workers = CleanupWorkerRegistry::new();
        let worker_cache = cache.clone();
        let worker_cancellation = cancellation.clone();
        let worker_cleanup_workers = cleanup_workers.clone();
        let join = thread::Builder::new()
            .name(proof::PLATFORM_PROBE_THREAD_NAME.to_string())
            .spawn(move || {
                run_server_owned_refresh(worker_cache, worker_cancellation, worker_cleanup_workers)
            })
            .ok();
        Arc::new(Self {
            cache,
            cancellation,
            join: Mutex::new(join),
            _cleanup_workers: cleanup_workers,
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
        // The worker owns one Docker refresh at a time. Each refresh shares
        // the absolute three-second preflight deadline, and the parked
        // cadence is explicitly unparked above. Joining therefore retains
        // the worker until its bounded operation exits; dropping a live
        // JoinHandle here would detach a server-owned probe thread.
        let _joined = join.join();
    }
}

fn run_server_owned_refresh(
    cache: PlatformProbeCache,
    cancellation: Arc<AtomicBool>,
    cleanup_workers: CleanupWorkerRegistry,
) {
    while !cancellation.load(Ordering::Acquire) {
        refresh_server_owned_cache(cache.clone(), cleanup_workers.clone());
        if cancellation.load(Ordering::Acquire) {
            return;
        }
        thread::park_timeout(PLATFORM_PROBE_REFRESH_INTERVAL);
    }
}

fn refresh_server_owned_cache(cache: PlatformProbeCache, cleanup_workers: CleanupWorkerRegistry) {
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

    let snapshot = detect_linux_docker_host_preflight(cleanup_workers);
    let Ok(mut state) = cache.state.lock() else {
        return;
    };
    state.snapshot = snapshot;
    state.last_refresh = Some(Instant::now());
    state.refresh_in_progress = false;
}
