use std::{
    process::{Child, ExitStatus},
    thread,
    time::{Duration, Instant},
};

const DOCKER_PROBE_TIMEOUT: Duration = Duration::from_secs(3);
const DOCKER_PROBE_POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn wait_bounded(child: &mut Child) -> Option<ExitStatus> {
    let deadline = Instant::now() + DOCKER_PROBE_TIMEOUT;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Some(status),
            Ok(None) if Instant::now() < deadline => thread::sleep(DOCKER_PROBE_POLL_INTERVAL),
            Ok(None) | Err(_) => {
                let _ = child.kill();
                let _ = child.wait();
                return None;
            }
        }
    }
}
