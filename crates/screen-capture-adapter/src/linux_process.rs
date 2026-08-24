use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    time::Instant,
};

#[path = "linux_process_group_reap.rs"]
mod group_reap;
#[path = "linux_process_monitor.rs"]
mod monitor;
#[path = "linux_process_output.rs"]
mod output;
#[path = "linux_process_output_io.rs"]
mod output_io;
#[path = "linux_process_runner.rs"]
mod runner;
#[path = "linux_process_termination.rs"]
mod termination;
#[path = "linux_tool_paths.rs"]
mod tool_paths;
#[path = "linux_tool_security.rs"]
mod tool_security;
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
    OutputUnavailable,
}

impl ChildResult {
    pub(crate) fn succeeded(&self) -> bool {
        matches!(self.outcome, ChildOutcome::Exited(true))
    }
}

pub(crate) fn executable_path(name: &str) -> Option<PathBuf> {
    tool_paths::executable_path(name)
}

pub(crate) fn run_child(program: &Path, args: &[OsString], deadline: Instant) -> ChildResult {
    runner::run_child(program, args, deadline)
}
