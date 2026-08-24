use std::{
    process::ExitStatus,
    time::{Duration, Instant},
};

use command_group::AsyncGroupChild;
use tokio::time::sleep;

#[cfg(unix)]
use nix::{
    errno::Errno,
    sys::{
        signal::killpg,
        wait::{waitpid, WaitPidFlag, WaitStatus},
    },
    unistd::Pid,
};

pub(super) const DOCKER_PREFLIGHT_TIMEOUT: Duration = Duration::from_secs(3);
const DOCKER_PROBE_POLL_INTERVAL: Duration = Duration::from_millis(10);
const DOCKER_GROUP_CLEANUP_GRACE: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, Debug)]
pub(super) struct DockerProcessGroup {
    #[cfg(unix)]
    pgid: Option<Pid>,
}

impl DockerProcessGroup {
    pub(super) fn capture(_child: &AsyncGroupChild) -> Self {
        Self {
            #[cfg(unix)]
            pgid: _child
                .id()
                .and_then(|id| i32::try_from(id).ok())
                .map(Pid::from_raw),
        }
    }
}

pub(super) async fn wait_bounded(
    child: &mut AsyncGroupChild,
    group: DockerProcessGroup,
    deadline: Instant,
) -> Option<ExitStatus> {
    while Instant::now() < deadline {
        if let Some(status) = child.inner().try_wait().ok().flatten() {
            return terminate_group_bounded(child, group, true)
                .await
                .then_some(status);
        }
        sleep(next_poll_delay(deadline)).await;
    }
    terminate_group_bounded(child, group, false).await;
    None
}

pub(super) async fn terminate_group_bounded(
    child: &mut AsyncGroupChild,
    group: DockerProcessGroup,
    mut direct_child_reaped: bool,
) -> bool {
    let group_kill_succeeded = child.start_kill().is_ok();
    if !group_kill_succeeded {
        let _ = child.inner().start_kill();
    }

    let cleanup_deadline = Instant::now() + DOCKER_GROUP_CLEANUP_GRACE;
    loop {
        if !direct_child_reaped {
            direct_child_reaped = child.inner().try_wait().ok().flatten().is_some();
        }
        direct_child_reaped |= reap_one_owned_group_child(group);
        if direct_child_reaped && group_cleanup_proven(group, group_kill_succeeded) {
            return true;
        }
        if Instant::now() >= cleanup_deadline {
            return false;
        }
        sleep(next_poll_delay(cleanup_deadline)).await;
    }
}

#[cfg(unix)]
fn reap_one_owned_group_child(group: DockerProcessGroup) -> bool {
    let Some(pgid) = group.pgid else {
        return false;
    };
    let group_wait_target = Pid::from_raw(-pgid.as_raw());
    matches!(
        waitpid(group_wait_target, Some(WaitPidFlag::WNOHANG)),
        Ok(WaitStatus::Exited(pid, _)) | Ok(WaitStatus::Signaled(pid, _, _)) if pid == pgid
    )
}

#[cfg(windows)]
fn reap_one_owned_group_child(_group: DockerProcessGroup) -> bool {
    false
}

#[cfg(unix)]
fn group_cleanup_proven(group: DockerProcessGroup, _group_kill_succeeded: bool) -> bool {
    group
        .pgid
        .is_some_and(|pgid| matches!(killpg(pgid, None), Err(Errno::ESRCH)))
}

#[cfg(windows)]
fn group_cleanup_proven(_group: DockerProcessGroup, group_kill_succeeded: bool) -> bool {
    // command-group owns a Windows Job Object; start_kill succeeds only when
    // TerminateJobObject accepted termination for every process in that job.
    group_kill_succeeded
}

fn next_poll_delay(deadline: Instant) -> Duration {
    DOCKER_PROBE_POLL_INTERVAL.min(deadline.saturating_duration_since(Instant::now()))
}
