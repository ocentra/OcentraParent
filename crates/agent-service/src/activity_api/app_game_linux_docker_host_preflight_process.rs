use std::{process::Stdio, thread, time::Instant};

use command_group::{AsyncCommandGroup, AsyncGroupChild};
use tokio::process::Command;

use super::{
    app_game_adapter_host_capabilities_paths::ResolvedExecutablePath,
    app_game_linux_docker_host_preflight_output::read_bounded_until,
    app_game_linux_docker_host_preflight_wait::{
        terminate_group_bounded, wait_bounded, DockerProcessGroup,
    },
};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub(super) struct DockerProbeOutput {
    pub(super) success: bool,
    pub(super) stdout: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DockerProbeArguments(pub(super) &'static [&'static str]);

impl DockerProbeOutput {
    fn unavailable() -> Self {
        Self {
            success: false,
            stdout: Vec::new(),
        }
    }
}

pub(super) fn run_docker_probe(
    executable: &ResolvedExecutablePath,
    arguments: DockerProbeArguments,
    deadline: Instant,
) -> DockerProbeOutput {
    let executable = executable.clone();
    // The runtime helper is joined; stdout is owned by its cancellable async
    // future, so this is not a detached reader thread or an unbounded wait.
    let Ok(worker) = thread::Builder::new()
        .spawn(move || run_docker_probe_on_runtime(executable, arguments, deadline))
    else {
        return DockerProbeOutput::unavailable();
    };
    worker
        .join()
        .unwrap_or_else(|_| DockerProbeOutput::unavailable())
}

fn run_docker_probe_on_runtime(
    executable: ResolvedExecutablePath,
    arguments: DockerProbeArguments,
    deadline: Instant,
) -> DockerProbeOutput {
    let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
    else {
        return DockerProbeOutput::unavailable();
    };
    runtime.block_on(run_docker_probe_async(executable, arguments, deadline))
}

async fn run_docker_probe_async(
    executable: ResolvedExecutablePath,
    arguments: DockerProbeArguments,
    deadline: Instant,
) -> DockerProbeOutput {
    if Instant::now() >= deadline {
        return DockerProbeOutput::unavailable();
    }

    let mut command = Command::new(executable.0);
    command
        .args(arguments.0)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    let mut child = match spawn_process_group(&mut command) {
        Ok(child) => child,
        Err(_) => return DockerProbeOutput::unavailable(),
    };
    let group = DockerProcessGroup::capture(&child);
    let stdout = match child.inner().stdout.take() {
        Some(stdout) => stdout,
        None => {
            terminate_group_bounded(&mut child, group, false).await;
            return DockerProbeOutput::unavailable();
        }
    };
    let captured = read_bounded_until(stdout, deadline).await;
    let status = if captured.timed_out || captured.overflow || captured.read_error {
        terminate_group_bounded(&mut child, group, false).await;
        None
    } else {
        wait_bounded(&mut child, group, deadline).await
    };

    DockerProbeOutput {
        success: status.is_some_and(|status| status.success())
            && !captured.overflow
            && !captured.read_error,
        stdout: captured.bytes,
    }
}

#[cfg(windows)]
fn spawn_process_group(command: &mut Command) -> std::io::Result<AsyncGroupChild> {
    command
        .group()
        .creation_flags(CREATE_NO_WINDOW)
        .kill_on_drop(true)
        .spawn()
}

#[cfg(not(windows))]
fn spawn_process_group(command: &mut Command) -> std::io::Result<AsyncGroupChild> {
    // Drop-kill is panic defense only. Probe success still requires the
    // explicit bounded kill/reap/group-absence proof in wait_bounded.
    command.group().kill_on_drop(true).spawn()
}
