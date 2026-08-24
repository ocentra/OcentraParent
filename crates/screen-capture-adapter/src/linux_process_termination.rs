use std::{
    process::Child,
    time::{Duration, Instant},
};

use nix::{
    errno::Errno,
    sys::signal::{killpg, Signal},
    unistd::Pid,
};

const CHILD_REAP_GRACE: Duration = Duration::from_millis(100);

use super::group_reap;

pub(super) fn terminate_child_group(child: &mut Child) -> bool {
    let child_pid = Pid::from_raw(child.id() as i32);
    let group_kill_ok = match killpg(child_pid, Signal::SIGKILL) {
        Ok(()) | Err(Errno::ESRCH) => true,
        Err(_) => false,
    };
    let _ = child.kill();
    group_reap::reap_and_confirm(
        child,
        child_pid,
        Instant::now() + CHILD_REAP_GRACE,
        group_kill_ok,
    )
}
