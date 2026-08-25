use std::{process::ExitStatus, thread, time::Duration};

use tokio::process::Child as TokioChild;

use super::app_game_linux_docker_host_preflight_group::{
    direct_child_kill_succeeded_or_gone, DockerProcessGroup,
};

pub(super) struct OwnedCleanupSupervisor {
    pub(super) child: TokioChild,
    pub(super) group: DockerProcessGroup,
    pub(super) status: Option<ExitStatus>,
    pub(super) group_signal_sent: bool,
}

impl OwnedCleanupSupervisor {
    pub(super) fn run(&mut self) {
        while !self.cleanup_iteration() {
            thread::sleep(Duration::from_millis(10));
        }
    }

    fn cleanup_iteration(&mut self) -> bool {
        if self.status.is_some() {
            return true;
        }
        if !self.group_signal_sent {
            // Keep the unreaped leader's PID/PGID reserved while the group
            // signal is sent. Never signal a numeric PGID after reaping it.
            if !direct_child_kill_succeeded_or_gone(&mut self.child) || !self.group.kill() {
                return false;
            }
            self.group_signal_sent = true;
        }
        match self.child.try_wait() {
            Ok(Some(status)) => {
                self.status = Some(status);
                true
            }
            Ok(None) | Err(_) => false,
        }
    }
}
