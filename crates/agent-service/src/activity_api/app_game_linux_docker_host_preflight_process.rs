use std::{
    process::{Command, Stdio},
    thread,
};

use super::{
    app_game_adapter_host_capabilities_paths::ResolvedExecutablePath,
    app_game_linux_docker_host_preflight_output::read_bounded,
    app_game_linux_docker_host_preflight_wait::wait_bounded,
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
) -> DockerProbeOutput {
    let mut command = Command::new(&executable.0);
    command
        .args(arguments.0)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    configure_process(&mut command);

    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(_) => return DockerProbeOutput::unavailable(),
    };
    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => {
            let _ = child.kill();
            let _ = child.wait();
            return DockerProbeOutput::unavailable();
        }
    };
    let reader = thread::spawn(move || read_bounded(stdout));
    let status = wait_bounded(&mut child);
    let captured = match reader.join() {
        Ok(captured) => captured,
        Err(_) => return DockerProbeOutput::unavailable(),
    };

    DockerProbeOutput {
        success: status.is_some_and(|status| status.success())
            && !captured.overflow
            && !captured.read_error,
        stdout: captured.bytes,
    }
}

#[cfg(windows)]
fn configure_process(command: &mut Command) {
    use std::os::windows::process::CommandExt;

    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn configure_process(_command: &mut Command) {}
