use std::{
    process::ExitStatus,
    thread,
    time::{Duration, Instant},
};

use tokio::process::Child as TokioChild;

use super::app_game_linux_docker_host_preflight_group::{
    direct_child_kill_succeeded_or_gone, DockerProcessGroup,
};

pub(super) struct OwnedCleanupSupervisor {
    pub(super) child: TokioChild,
    pub(super) group: DockerProcessGroup,
    pub(super) status: Option<ExitStatus>,
    pub(super) group_signal_sent: bool,
    pub(super) cleanup_deadline: Instant,
}

pub(super) const CLEANUP_OWNER_TIMEOUT: Duration = Duration::from_secs(3);

impl OwnedCleanupSupervisor {
    pub(super) fn run(&mut self) -> bool {
        while Instant::now() < self.cleanup_deadline {
            let Ok(done) = self.cleanup_iteration() else {
                return false;
            };
            if done {
                return true;
            }
            thread::sleep(Duration::from_millis(10));
        }
        // The owner cannot prove that the group is gone within its bounded
        // custody window. The Tokio child retains kill_on_drop custody, while
        // the registry records the platform as degraded rather than allowing
        // an unbounded retry or claiming descendant cleanup.
        false
    }

    fn cleanup_iteration(&mut self) -> Result<bool, ()> {
        if self.status.is_some() {
            return self.group_signal_sent.then_some(true).ok_or(());
        }
        if !self.group_signal_sent {
            // Keep the unreaped leader's PID/PGID reserved while the group
            // signal is sent. Never signal a numeric PGID after reaping it.
            if !direct_child_kill_succeeded_or_gone(&mut self.child) || !self.group.kill() {
                return Err(());
            }
            self.group_signal_sent = true;
        }
        match self.child.try_wait() {
            Ok(Some(status)) => {
                self.status = Some(status);
                Ok(true)
            }
            Ok(None) => Ok(false),
            // Do not turn a wait error into an endless retry or a false
            // cleanup success. The worker still owns the child and escalates
            // while that custody is retained.
            Err(_) => Err(()),
        }
    }
}
