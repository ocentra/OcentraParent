use std::{
    process::ExitStatus,
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::thread;

use command_group::AsyncGroupChild;
use tokio::time::sleep;

#[cfg(unix)]
use nix::{
    errno::Errno,
    sys::{
        signal::{Signal, killpg},
        wait::{WaitPidFlag, WaitStatus, waitpid},
    },
    unistd::Pid,
};

pub(super) const DOCKER_PREFLIGHT_TIMEOUT: Duration = Duration::from_secs(3);
const DOCKER_PROBE_POLL_INTERVAL: Duration = Duration::from_millis(10);
const DOCKER_GROUP_CLEANUP_GRACE: Duration = Duration::from_millis(500);

#[cfg(unix)]
pub(super) struct DockerProcessGroup {
    pgid: Pid,
    cleanup_complete: bool,
}

#[cfg(not(unix))]
pub(super) struct DockerProcessGroup;

impl DockerProcessGroup {
    pub(super) fn capture(child: &AsyncGroupChild) -> Option<Self> {
        #[cfg(unix)]
        {
            let pgid = child
                .id()
                .and_then(|id| i32::try_from(id).ok())
                .filter(|id| *id > 1)
                .map(Pid::from_raw)?;
            Some(Self {
                pgid,
                cleanup_complete: false,
            })
        }

        #[cfg(not(unix))]
        {
            child.id().map(|_| Self)
        }
    }

    #[cfg(unix)]
    fn group_empty(&self) -> bool {
        matches!(killpg(self.pgid, None), Err(Errno::ESRCH))
    }

    #[cfg(not(unix))]
    fn group_empty(&self) -> bool {
        true
    }

    #[cfg(unix)]
    fn request_kill(&self) -> bool {
        matches!(
            killpg(self.pgid, Signal::SIGKILL),
            Ok(()) | Err(Errno::ESRCH)
        )
    }

    #[cfg(not(unix))]
    fn request_kill(&self) -> bool {
        false
    }

    #[cfg(unix)]
    fn reap_nonblocking(&self) {
        while let Ok(status) = waitpid(
            Pid::from_raw(-self.pgid.as_raw()),
            Some(WaitPidFlag::WNOHANG),
        ) {
            if matches!(status, WaitStatus::StillAlive) {
                break;
            }
        }
    }

    #[cfg(not(unix))]
    fn reap_nonblocking(&self) {}

    fn mark_cleanup_complete(&mut self) {
        #[cfg(unix)]
        {
            self.cleanup_complete = true;
        }
    }
}

impl Drop for DockerProcessGroup {
    fn drop(&mut self) {
        #[cfg(unix)]
        if !self.cleanup_complete {
            self.request_kill();
            let deadline = Instant::now() + DOCKER_GROUP_CLEANUP_GRACE;
            while !self.group_empty() && Instant::now() < deadline {
                self.reap_nonblocking();
                thread::sleep(DOCKER_PROBE_POLL_INTERVAL);
            }
            self.cleanup_complete = self.group_empty();
        }
    }
}

pub(super) async fn wait_bounded(
    child: &mut AsyncGroupChild,
    group: &mut DockerProcessGroup,
    deadline: Instant,
) -> Option<ExitStatus> {
    while Instant::now() < deadline {
        let Some(status) = child.try_wait().ok().flatten() else {
            sleep(next_poll_delay(deadline)).await;
            continue;
        };
        if group.group_empty() {
            group.mark_cleanup_complete();
            return Some(status);
        }
        return terminate_group_bounded(child, group, true)
            .await
            .then_some(status);
    }
    terminate_group_bounded(child, group, false).await;
    None
}

pub(super) async fn terminate_child_bounded(child: &mut AsyncGroupChild) -> bool {
    let _ = child.start_kill();
    let cleanup_deadline = Instant::now() + DOCKER_GROUP_CLEANUP_GRACE;
    while Instant::now() < cleanup_deadline {
        if child.try_wait().ok().flatten().is_some() {
            return true;
        }
        sleep(next_poll_delay(cleanup_deadline)).await;
    }
    false
}

pub(super) async fn terminate_group_bounded(
    child: &mut AsyncGroupChild,
    group: &mut DockerProcessGroup,
    mut direct_child_reaped: bool,
) -> bool {
    let _ = group.request_kill();
    let _ = child.start_kill();

    let cleanup_deadline = Instant::now() + DOCKER_GROUP_CLEANUP_GRACE;
    loop {
        direct_child_reaped |= child.try_wait().ok().flatten().is_some();
        group.reap_nonblocking();
        if direct_child_reaped && group.group_empty() {
            group.mark_cleanup_complete();
            return true;
        }
        if Instant::now() >= cleanup_deadline {
            return false;
        }
        sleep(next_poll_delay(cleanup_deadline)).await;
    }
}

fn next_poll_delay(deadline: Instant) -> Duration {
    DOCKER_PROBE_POLL_INTERVAL.min(deadline.saturating_duration_since(Instant::now()))
}
