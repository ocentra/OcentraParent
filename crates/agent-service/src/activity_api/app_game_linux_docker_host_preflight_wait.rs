use std::{
    process::ExitStatus,
    time::{Duration, Instant},
};

use command_group::AsyncGroupChild;
use tokio::time::sleep;

pub(super) const DOCKER_PREFLIGHT_TIMEOUT: Duration = Duration::from_secs(3);
const DOCKER_PROBE_POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) async fn wait_bounded(
    child: &mut AsyncGroupChild,
    deadline: Instant,
) -> Option<ExitStatus> {
    while Instant::now() < deadline {
        if let Some(status) = child.try_wait().ok().flatten() {
            return Some(status);
        }
        sleep(next_poll_delay(deadline)).await;
    }
    terminate_group_bounded(child, deadline).await;
    None
}

pub(super) async fn terminate_group_bounded(child: &mut AsyncGroupChild, deadline: Instant) {
    // command-group owns the Windows Job Object and Unix process group. The
    // direct-child fallback is only for a group operation that reports an
    // error (for example, the group has already exited); it never waits.
    if child.start_kill().is_err() {
        let _ = child.inner().start_kill();
    }

    while Instant::now() < deadline && child.try_wait().ok().flatten().is_none() {
        sleep(next_poll_delay(deadline)).await;
    }
}

fn next_poll_delay(deadline: Instant) -> Duration {
    DOCKER_PROBE_POLL_INTERVAL.min(deadline.saturating_duration_since(Instant::now()))
}
