use std::{
    process::ExitStatus,
    time::{Duration, Instant},
};
use tokio::time::sleep;

use super::app_game_linux_docker_host_preflight_cleanup::transfer_cleanup;
use super::app_game_linux_docker_host_preflight_supervisor::{
    DockerProcessSupervisor, DOCKER_PROBE_POLL_INTERVAL,
};

pub(super) const DOCKER_PREFLIGHT_TIMEOUT: Duration = Duration::from_secs(3);

pub(super) async fn wait_bounded(
    supervisor: &mut DockerProcessSupervisor,
    deadline: Instant,
) -> Option<ExitStatus> {
    while Instant::now() < deadline {
        let leader_exit_ready = match supervisor.leader_exit_ready() {
            Ok(ready) => ready,
            Err(_) => {
                // Without an identity-bound exit observation, do not reap the
                // leader or touch a numeric PGID from this request thread.
                transfer_cleanup(supervisor);
                return None;
            }
        };
        if !leader_exit_ready {
            sleep(next_poll_delay(deadline)).await;
            continue;
        }
        let Ok(progress) = supervisor.cleanup_after_leader_exit() else {
            transfer_cleanup(supervisor);
            return None;
        };
        if let Some(status) = progress {
            supervisor.mark_cleanup_complete();
            return Some(status);
        }
        sleep(next_poll_delay(deadline)).await;
    }
    let _ = terminate_group_bounded(supervisor).await;
    None
}

pub(super) async fn terminate_child_bounded(supervisor: &mut DockerProcessSupervisor) -> bool {
    terminate_group_bounded(supervisor).await
}

pub(super) async fn terminate_group_bounded(supervisor: &mut DockerProcessSupervisor) -> bool {
    let cleanup_deadline = supervisor.cleanup_deadline();
    while Instant::now() < cleanup_deadline {
        let Ok(progress) = supervisor.cleanup_step() else {
            transfer_cleanup(supervisor);
            return false;
        };
        if progress.is_some() {
            supervisor.mark_cleanup_complete();
            return true;
        }
        sleep(next_poll_delay(cleanup_deadline)).await;
    }
    // The request's absolute deadline has expired. The pre-reserved owner
    // takes custody now; Drop must not add another synchronous grace window.
    transfer_cleanup(supervisor);
    false
}

fn next_poll_delay(deadline: Instant) -> Duration {
    DOCKER_PROBE_POLL_INTERVAL.min(deadline.saturating_duration_since(Instant::now()))
}
