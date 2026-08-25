#[cfg(unix)]
use nix::{
    errno::Errno,
    sys::signal::{killpg, Signal},
    unistd::Pid,
};
use std::io;
use tokio::process::Child as TokioChild;

#[derive(Clone)]
pub(super) struct DockerProcessGroup {
    #[cfg(unix)]
    pgid: Pid,
}

impl DockerProcessGroup {
    pub(super) fn from_pid(id: u32) -> Option<Self> {
        #[cfg(unix)]
        {
            i32::try_from(id)
                .ok()
                .filter(|id| *id > 1)
                .map(Pid::from_raw)
                .map(|pgid| Self { pgid })
        }
        #[cfg(not(unix))]
        {
            let _ = id;
            Some(Self {})
        }
    }

    pub(super) fn kill(&self) -> bool {
        #[cfg(unix)]
        {
            matches!(
                killpg(self.pgid, Some(Signal::SIGKILL)),
                Ok(()) | Err(Errno::ESRCH)
            )
        }
        #[cfg(not(unix))]
        {
            // Windows resolution is fail-closed before this owner is used;
            // there is no Unix-style numeric process-group signal here.
            true
        }
    }
}

pub(super) fn direct_child_kill_succeeded_or_gone(child: &mut TokioChild) -> bool {
    child
        .start_kill()
        .map(|()| true)
        .unwrap_or_else(|error| process_is_gone(&error))
}

#[cfg(unix)]
fn process_is_gone(error: &io::Error) -> bool {
    error.kind() == io::ErrorKind::NotFound
        || error.raw_os_error() == Some(nix::errno::Errno::ESRCH as i32)
}

#[cfg(not(unix))]
fn process_is_gone(error: &io::Error) -> bool {
    error.kind() == io::ErrorKind::NotFound
}
