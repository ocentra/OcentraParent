use std::{
    ffi::OsString,
    os::unix::process::CommandExt,
    path::Path,
    process::{Child, Command, Stdio},
    time::Instant,
};

use super::{monitor, output::OutputDrain, termination, tool_paths, ChildOutcome, ChildResult};

pub(super) fn run_child(program: &Path, args: &[OsString], deadline: Instant) -> ChildResult {
    if Instant::now() >= deadline {
        return ChildResult {
            stdout: Vec::new(),
            outcome: ChildOutcome::TimedOut,
        };
    }
    if !tool_paths::trusted_executable_path(program) {
        return failed_spawn();
    }
    let mut child = match spawn_child(program, args) {
        Ok(child) => child,
        Err(_) => return failed_spawn(),
    };
    let Some(output) = OutputDrain::from_child(&mut child) else {
        termination::terminate_child_group(&mut child);
        return unavailable_result();
    };
    monitor::monitor_child(child, output, deadline)
}

fn spawn_child(program: &Path, args: &[OsString]) -> std::io::Result<Child> {
    let mut command = Command::new(program);
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .process_group(0);
    command.spawn()
}

fn failed_spawn() -> ChildResult {
    ChildResult {
        stdout: Vec::new(),
        outcome: ChildOutcome::SpawnFailed,
    }
}

fn unavailable_result() -> ChildResult {
    ChildResult {
        stdout: Vec::new(),
        outcome: ChildOutcome::OutputUnavailable,
    }
}
