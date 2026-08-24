use std::{
    ffi::OsString,
    fs,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[path = "linux_process_output.rs"]
mod output;

use nix::{
    sys::signal::{killpg, Signal},
    unistd::Pid,
};

const CHILD_POLL_INTERVAL: Duration = Duration::from_millis(10);
pub(crate) const MAX_CHILD_STDOUT_BYTES: usize = 64 * 1024;

#[derive(Debug)]
pub(crate) struct ChildResult {
    pub(crate) stdout: Vec<u8>,
    pub(crate) outcome: ChildOutcome,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ChildOutcome {
    SpawnFailed,
    TimedOut,
    Exited(bool),
    OutputTooLarge,
}

impl ChildResult {
    pub(crate) fn succeeded(&self) -> bool {
        matches!(self.outcome, ChildOutcome::Exited(true))
    }
}

pub(crate) fn executable_path(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    std::env::split_paths(&paths).find_map(|path| {
        let candidate = path.join(name);
        (candidate.is_file() && executable_file(&candidate)).then_some(candidate)
    })
}

fn executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

pub(crate) fn run_child(program: &Path, args: &[OsString], deadline: Instant) -> ChildResult {
    if Instant::now() >= deadline {
        return ChildResult {
            stdout: Vec::new(),
            outcome: ChildOutcome::TimedOut,
        };
    }
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .process_group(0);
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => return failed_spawn(),
    };

    loop {
        match child.try_wait() {
            Ok(Some(status)) => return output::exited_result(&mut child, status.success()),
            Ok(None) => {}
            Err(_) => {
                terminate_child_group(&mut child);
                return failed_spawn();
            }
        }

        let now = Instant::now();
        if now >= deadline {
            terminate_child_group(&mut child);
            return ChildResult {
                stdout: Vec::new(),
                outcome: ChildOutcome::TimedOut,
            };
        }
        thread::sleep(CHILD_POLL_INTERVAL.min(deadline.saturating_duration_since(now)));
    }
}

fn failed_spawn() -> ChildResult {
    ChildResult {
        stdout: Vec::new(),
        outcome: ChildOutcome::SpawnFailed,
    }
}

pub(crate) fn terminate_child_group(child: &mut Child) {
    let child_pid = Pid::from_raw(child.id() as i32);
    let _ = killpg(child_pid, Signal::SIGKILL);
    let _ = child.kill();
    let _ = child.wait();
}
