use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
};

use super::app_game_linux_docker_host_preflight_cleanup_worker::cleanup_worker;

const CLEANUP_THREAD_NAME: &str = "ocentra-docker-probe-supervisor";

pub(super) struct CleanupMailbox {
    pub(super) owner:
        Option<super::app_game_linux_docker_host_preflight_cleanup_process::OwnedCleanupSupervisor>,
    pub(super) stop: bool,
}

pub(super) struct ReservedCleanupOwner {
    pub(super) mailbox: Arc<(Mutex<CleanupMailbox>, Condvar)>,
    _worker: thread::JoinHandle<()>,
}

impl ReservedCleanupOwner {
    pub(super) fn new() -> Option<Self> {
        let mailbox = Arc::new((
            Mutex::new(CleanupMailbox {
                owner: None,
                stop: false,
            }),
            Condvar::new(),
        ));
        let worker_mailbox = Arc::clone(&mailbox);
        let worker = thread::Builder::new()
            .name(CLEANUP_THREAD_NAME.to_string())
            .spawn(move || cleanup_worker(worker_mailbox))
            .ok()?;
        Some(Self {
            mailbox,
            _worker: worker,
        })
    }

    pub(super) fn handoff(
        &self,
        owner: super::app_game_linux_docker_host_preflight_cleanup_process::OwnedCleanupSupervisor,
    ) {
        let (lock, wake) = &*self.mailbox;
        let mut mailbox = recover_lock(lock);
        debug_assert!(mailbox.owner.is_none());
        mailbox.owner = Some(owner);
        wake.notify_one();
    }
}

impl Drop for ReservedCleanupOwner {
    fn drop(&mut self) {
        let (lock, wake) = &*self.mailbox;
        let mut mailbox = recover_lock(lock);
        if mailbox.owner.is_none() {
            mailbox.stop = true;
            wake.notify_one();
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
