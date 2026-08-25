use std::{thread, time::Duration};

use super::{
    app_game_linux_docker_host_preflight_cleanup_process::OwnedCleanupSupervisor,
    app_game_linux_docker_host_preflight_supervisor::DockerProcessSupervisor,
};

pub(super) fn transfer_cleanup(supervisor: &mut DockerProcessSupervisor) {
    let Some(child) = supervisor.child.take() else {
        supervisor.cleanup_complete = true;
        return;
    };
    supervisor.cleanup_owner.handoff(OwnedCleanupSupervisor {
        child,
        group: supervisor.group.clone(),
        status: supervisor.status,
        group_signal_sent: supervisor.group_signal_sent,
    });
    supervisor.cleanup_complete = true;
}

impl DockerProcessSupervisor {
    pub(super) fn cleanup_sync_for(&mut self, deadline: std::time::Instant) -> bool {
        while std::time::Instant::now() < deadline {
            if self.cleanup_step() {
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
        if self.cleanup_complete {
            return;
        }
        if !self.cleanup_sync_for(self.cleanup_deadline()) {
            transfer_cleanup(self);
        }
    }
}
