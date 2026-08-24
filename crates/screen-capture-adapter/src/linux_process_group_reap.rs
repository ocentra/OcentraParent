use std::{
    process::Child,
    thread,
    time::{Duration, Instant},
};

use nix::{errno::Errno, sys::signal::killpg, unistd::Pid};

const CHILD_POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn reap_and_confirm(
    child: &mut Child,
    process_group: Pid,
    deadline: Instant,
    group_kill_ok: bool,
) -> bool {
    let mut child_reaped = false;
    loop {
        match child.try_wait() {
            Ok(Some(_)) => child_reaped = true,
            Ok(None) => {}
            Err(_) => return false,
        }
        if child_reaped && group_kill_ok && process_group_gone(process_group) {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(CHILD_POLL_INTERVAL);
    }
}

fn process_group_gone(process_group: Pid) -> bool {
    match killpg(process_group, None) {
        Err(Errno::ESRCH) => true,
        Ok(()) | Err(_) => false,
    }
}
