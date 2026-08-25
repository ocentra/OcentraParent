use std::{thread, time::Duration};

use super::{
    app_game_linux_docker_host_preflight_cleanup_process::{
        OwnedCleanupSupervisor, CLEANUP_OWNER_TIMEOUT,
    },
    app_game_linux_docker_host_preflight_supervisor::DockerProcessSupervisor,
};

pub(super) fn transfer_cleanup(supervisor: &mut DockerProcessSupervisor) {
    let Some(child) = supervisor.child.take() else {
        supervisor.cleanup_failed = true;
        return;
    };
    let handed_off = supervisor.cleanup_owner.handoff(OwnedCleanupSupervisor {
        child,
        group: supervisor.group.clone(),
        status: supervisor.status,
        group_signal_sent: supervisor.group_signal_sent,
        cleanup_deadline: std::time::Instant::now() + CLEANUP_OWNER_TIMEOUT,
    });
    if handed_off {
        supervisor.cleanup_complete = true;
    } else {
        // The rejected handoff has already dropped the owned child through
        // Tokio's kill_on_drop fallback. Keep the outcome explicitly failed;
        // do not let Drop retry against a handle whose custody was rejected.
        supervisor.cleanup_failed = true;
    }
}

impl DockerProcessSupervisor {
    pub(super) fn cleanup_sync_for(&mut self, deadline: std::time::Instant) -> bool {
        while std::time::Instant::now() < deadline {
            let Ok(progress) = self.cleanup_step() else {
                return false;
            };
            if progress.is_some() {
                self.cleanup_complete = true;
                return true;
            }
            thread::sleep(Duration::from_millis(10));
        }
        false
    }
}

impl Drop for DockerProcessSupervisor {
    fn drop(&mut self) {
        if self.cleanup_complete || self.cleanup_failed {
            return;
        }
        if !self.cleanup_sync_for(self.cleanup_deadline()) {
            transfer_cleanup(self);
        }
    }
}
