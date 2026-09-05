use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;

use super::app_game_linux_docker_host_preflight::detect_linux_docker_host_preflight;
use super::app_game_linux_docker_host_preflight_cleanup_owner::CleanupWorkerRegistry;
use super::app_game_platform_probe_cache::PlatformProbeCache;

const PLATFORM_PROBE_REFRESH_INTERVAL: Duration = Duration::from_secs(300);

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
        let worker_cancellation = Arc::clone(&cancellation);
        let worker_cleanup_workers = cleanup_workers.clone();
        let join = thread::Builder::new()
            .name(proof::PLATFORM_PROBE_THREAD_NAME.to_string())
            .spawn(move || {
                run_server_owned_refresh(
                    &worker_cache,
                    &worker_cancellation,
                    &worker_cleanup_workers,
                )
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
    cache: &PlatformProbeCache,
    cancellation: &Arc<AtomicBool>,
    cleanup_workers: &CleanupWorkerRegistry,
) {
    while !cancellation.load(Ordering::Acquire) {
        refresh_server_owned_cache(cache, cleanup_workers);
        if cancellation.load(Ordering::Acquire) {
            return;
        }
        thread::park_timeout(PLATFORM_PROBE_REFRESH_INTERVAL);
    }
}

fn refresh_server_owned_cache(cache: &PlatformProbeCache, cleanup_workers: &CleanupWorkerRegistry) {
    if !cache.begin_refresh(PLATFORM_PROBE_REFRESH_INTERVAL) {
        return;
    }
    let snapshot = detect_linux_docker_host_preflight(cleanup_workers.clone());
    cache.finish_refresh(snapshot);
}
