use std::{
    process::ExitStatus,
    sync::atomic::{AtomicBool, Ordering},
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

const PROCESS_POLL_INTERVAL: Duration = Duration::from_millis(10);
const PROCESS_TERMINATION_GRACE: Duration = Duration::from_millis(500);

#[derive(Clone, Copy, Debug)]
pub(super) struct ProcessGroup {
    #[cfg(unix)]
    pgid: Option<Pid>,
}

impl ProcessGroup {
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
    group: ProcessGroup,
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
    terminate: &AtomicBool,
) -> Option<ExitStatus> {
    let execution_deadline = deadline
        .checked_sub(PROCESS_TERMINATION_GRACE)
        .unwrap_or_else(Instant::now);
    while Instant::now() < execution_deadline {
        if externally_cancelled(cancellation) || terminate.load(Ordering::Acquire) {
            terminate_group_bounded(child, group, false, deadline).await;
            return None;
        }
        match child.inner().try_wait() {
            Ok(Some(status)) => {
                return terminate_group_bounded(child, group, true, deadline)
                    .await
                    .then_some(status);
            }
            Ok(None) => sleep(next_poll_delay(execution_deadline)).await,
            Err(_) => {
                terminate_group_bounded(child, group, false, deadline).await;
                return None;
            }
        }
    }
    terminate_group_bounded(child, group, false, deadline).await;
    None
}

pub(super) async fn terminate_group_bounded(
    child: &mut AsyncGroupChild,
    group: ProcessGroup,
    mut direct_child_reaped: bool,
    cleanup_deadline: Instant,
) -> bool {
    // The caller's command deadline may be later than an enclosing scan's
    // cancellation deadline. Never spend that whole residual budget after a
    // cancellation or pipe failure: group cleanup itself has a hard bound.
    let cleanup_deadline = cleanup_deadline.min(Instant::now() + PROCESS_TERMINATION_GRACE);
    let group_kill_succeeded = child.start_kill().is_ok();
    if !group_kill_succeeded {
        let _ = child.inner().start_kill();
    }

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
fn reap_one_owned_group_child(group: ProcessGroup) -> bool {
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
fn reap_one_owned_group_child(_group: ProcessGroup) -> bool {
    false
}

#[cfg(unix)]
fn group_cleanup_proven(group: ProcessGroup, _group_kill_succeeded: bool) -> bool {
    group
        .pgid
        .is_some_and(|pgid| matches!(killpg(pgid, None), Err(Errno::ESRCH)))
}

#[cfg(windows)]
fn group_cleanup_proven(_group: ProcessGroup, group_kill_succeeded: bool) -> bool {
    group_kill_succeeded
}

fn externally_cancelled(cancellation: Option<&AtomicBool>) -> bool {
    cancellation.is_some_and(|value| value.load(Ordering::Acquire))
}

fn next_poll_delay(deadline: Instant) -> Duration {
    PROCESS_POLL_INTERVAL.min(deadline.saturating_duration_since(Instant::now()))
}
