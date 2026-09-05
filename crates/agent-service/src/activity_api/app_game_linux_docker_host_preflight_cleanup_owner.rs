use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex,
    },
    thread,
};

use super::app_game_linux_docker_host_preflight_cleanup_worker::{
    cleanup_worker, CleanupWorkerContext,
};

const CLEANUP_THREAD_NAME: &str = "ocentra-docker-probe-supervisor";

#[derive(Clone)]
pub(super) struct CleanupWorkerRegistry {
    inner: Arc<CleanupWorkerRegistryInner>,
}

struct CleanupWorkerRegistryInner {
    workers: Mutex<Vec<thread::JoinHandle<()>>>,
    degraded: Arc<AtomicBool>,
}

impl CleanupWorkerRegistry {
    pub(super) fn new() -> Self {
        let degraded = Arc::new(AtomicBool::new(false));
        Self {
            inner: Arc::new(CleanupWorkerRegistryInner {
                workers: Mutex::new(Vec::new()),
                degraded,
            }),
        }
    }

    fn retain(&self, worker: thread::JoinHandle<()>) {
        let mut workers = recover_lock(&self.inner.workers);
        let mut live_workers = Vec::with_capacity(workers.len() + 1);
        for worker in workers.drain(..) {
            if worker.is_finished() {
                self.join_finished(worker);
            } else {
                live_workers.push(worker);
            }
        }
        if worker.is_finished() {
            self.join_finished(worker);
        } else {
            live_workers.push(worker);
        }
        *workers = live_workers;
    }

    fn join_finished(&self, worker: thread::JoinHandle<()>) {
        if worker.join().is_err() {
            self.mark_degraded();
        }
    }

    pub(super) fn mark_degraded(&self) {
        self.inner.degraded.store(true, Ordering::Release);
    }

    fn degraded_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.inner.degraded)
    }

    pub(super) fn is_degraded(&self) -> bool {
        self.inner.degraded.load(Ordering::Acquire)
    }
}

impl Drop for CleanupWorkerRegistryInner {
    fn drop(&mut self) {
        let workers = std::mem::take(&mut *recover_lock(&self.workers));
        for worker in workers {
            if worker.join().is_err() {
                self.degraded.store(true, Ordering::Release);
            }
        }
    }
}

pub(super) struct CleanupMailbox {
    pub(super) owner:
        Option<super::app_game_linux_docker_host_preflight_cleanup_process::OwnedCleanupSupervisor>,
    pub(super) stop: bool,
    pub(super) handoff_active: bool,
}

pub(super) struct ReservedCleanupOwner {
    pub(super) mailbox: Arc<(Mutex<CleanupMailbox>, Condvar)>,
    worker: Option<thread::JoinHandle<()>>,
    registry: CleanupWorkerRegistry,
}

impl ReservedCleanupOwner {
    pub(super) fn new(registry: CleanupWorkerRegistry) -> Option<Self> {
        let mailbox = Arc::new((
            Mutex::new(CleanupMailbox {
                owner: None,
                stop: false,
                handoff_active: false,
            }),
            Condvar::new(),
        ));
        let worker_mailbox = Arc::clone(&mailbox);
        let worker_degraded = registry.degraded_flag();
        let worker = thread::Builder::new()
            .name(CLEANUP_THREAD_NAME.to_string())
            .spawn(move || {
                cleanup_worker(CleanupWorkerContext {
                    mailbox: worker_mailbox,
                    degraded: worker_degraded,
                })
            })
            .ok()?;
        Some(Self {
            mailbox,
            worker: Some(worker),
            registry,
        })
    }

    pub(super) fn handoff(
        &self,
        owner: super::app_game_linux_docker_host_preflight_cleanup_process::OwnedCleanupSupervisor,
    ) -> bool {
        let (lock, wake) = &*self.mailbox;
        let mut mailbox = recover_lock(lock);
        if mailbox.owner.is_some() || mailbox.stop || mailbox.handoff_active {
            drop(mailbox);
            // A second handoff or a handoff after shutdown would discard
            // process custody. Mark the server-owned probe path degraded and
            // let `owner` fall through its kill_on_drop direct-child fallback;
            // no descendant-custody claim is made from this failed handoff.
            self.registry.mark_degraded();
            return false;
        }
        mailbox.owner = Some(owner);
        mailbox.handoff_active = true;
        wake.notify_one();
        true
    }
}

impl Drop for ReservedCleanupOwner {
    fn drop(&mut self) {
        let handoff_active = self.signal_stop_if_unclaimed();
        self.finish_worker(handoff_active);
    }
}

impl ReservedCleanupOwner {
    fn signal_stop_if_unclaimed(&self) -> bool {
        let (lock, wake) = &*self.mailbox;
        let mut mailbox = recover_lock(lock);
        if mailbox.handoff_active {
            return true;
        }
        mailbox.stop = true;
        wake.notify_one();
        false
    }

    fn finish_worker(&mut self, handoff_active: bool) {
        let Some(worker) = self.worker.take() else {
            return;
        };
        if handoff_active {
            // The request thread has transferred child custody. Retain the
            // worker in the service-owned registry so this Drop path stays
            // inside the original request deadline without detaching a
            // child-owning thread.
            self.registry.retain(worker);
            return;
        }
        // No child was handed off; the reserved worker is joined on the
        // ordinary shutdown path rather than detached.
        if worker.join().is_err() {
            self.registry.mark_degraded();
        }
    }
}

pub(super) fn recover_lock<T>(lock: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    lock.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

pub(super) fn recover_wait<'a, T>(
    wake: &Condvar,
    guard: std::sync::MutexGuard<'a, T>,
) -> std::sync::MutexGuard<'a, T> {
    wake.wait(guard)
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
