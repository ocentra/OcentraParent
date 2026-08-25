use std::{process::Stdio, thread, time::Instant};

#[cfg(target_os = "linux")]
use std::path::PathBuf;

use ocentra_parent_agent_protocol::constants::v08_supported_adapter_runtime_proof as proof;
use tokio::process::Command;

use super::{
    app_game_linux_docker_host_preflight_cleanup_owner::ReservedCleanupOwner,
    app_game_linux_docker_host_preflight_output::read_bounded_until,
    app_game_linux_docker_host_preflight_path_security::revalidate_trusted_docker_candidate,
    app_game_linux_docker_host_preflight_paths::TrustedDockerExecutable,
    app_game_linux_docker_host_preflight_supervisor::DockerProcessSupervisor,
    app_game_linux_docker_host_preflight_wait::{
        terminate_child_bounded, terminate_group_bounded, wait_bounded,
    },
};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const CLEANUP_SUPERVISOR_UNAVAILABLE: &str = "docker probe cleanup supervisor unavailable";

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
    executable: &TrustedDockerExecutable,
    arguments: DockerProbeArguments,
    deadline: Instant,
) -> DockerProbeOutput {
    let Some(executable) = executable.try_clone() else {
        return DockerProbeOutput::unavailable();
    };
    // The worker is joined. The owned process supervisor remains live through
    // every async timeout and unwind path and transfers cleanup ownership only
    // after bounded synchronous group-absence proof fails.
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
    executable: TrustedDockerExecutable,
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
    executable: TrustedDockerExecutable,
    arguments: DockerProbeArguments,
    deadline: Instant,
) -> DockerProbeOutput {
    if Instant::now() >= deadline {
        return DockerProbeOutput::unavailable();
    }

    if !revalidate_trusted_docker_candidate(&executable) {
        return DockerProbeOutput::unavailable();
    }

    let mut command = build_docker_command(&executable, arguments);
    let mut supervisor = match spawn_process_group(&mut command, deadline) {
        Ok(supervisor) => supervisor,
        Err(_) => return DockerProbeOutput::unavailable(),
    };
    let stdout = match supervisor.take_stdout() {
        Some(stdout) => stdout,
        None => {
            let _ = terminate_child_bounded(&mut supervisor).await;
            return DockerProbeOutput::unavailable();
        }
    };
    let captured = read_bounded_until(stdout, deadline).await;
    let status = if captured.timed_out || captured.overflow || captured.read_error {
        let _ = terminate_group_bounded(&mut supervisor).await;
        None
    } else {
        wait_bounded(&mut supervisor, deadline).await
    };

    DockerProbeOutput {
        success: status.is_some_and(|status| status.success())
            && !captured.overflow
            && !captured.read_error,
        stdout: captured.bytes,
    }
}

fn build_docker_command(
    executable: &TrustedDockerExecutable,
    arguments: DockerProbeArguments,
) -> Command {
    #[cfg(target_os = "linux")]
    let executable_path = {
        use std::os::fd::AsRawFd;
        PathBuf::from(format!(
            proof::DOCKER_DESCRIPTOR_PATH_FORMAT,
            executable.executable.as_raw_fd()
        ))
    };
    #[cfg(not(target_os = "linux"))]
    let executable_path = executable.path.clone();
    let mut command = Command::new(executable_path);
    command
        .args(arguments.0)
        .current_dir(&executable.cwd)
        .env_clear()
        .env(proof::ENV_PATH, &executable.cwd)
        .env(proof::ENV_DOCKER_HOST, proof::DOCKER_SERVICE_ENDPOINT)
        .env(proof::ENV_DOCKER_CONTEXT, proof::DOCKER_SERVICE_CONTEXT)
        .env(
            proof::ENV_DOCKER_CONFIG,
            proof::DOCKER_SERVICE_CONFIG_DIRECTORY,
        )
        .env(
            proof::ENV_DOCKER_TLS_VERIFY,
            proof::DOCKER_SERVICE_TLS_VERIFY,
        )
        .env(
            proof::ENV_DOCKER_CERT_PATH,
            ocentra_parent_agent_protocol::constants::value::EMPTY,
        )
        .env(proof::ENV_HOME, proof::DOCKER_SERVICE_CONFIG_DIRECTORY)
        .env(
            proof::ENV_USERPROFILE,
            proof::DOCKER_SERVICE_CONFIG_DIRECTORY,
        )
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null());
    command
}

fn spawn_process_group(
    command: &mut Command,
    cleanup_deadline: Instant,
) -> std::io::Result<DockerProcessSupervisor> {
    let cleanup_owner = ReservedCleanupOwner::new().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, CLEANUP_SUPERVISOR_UNAVAILABLE)
    })?;
    #[cfg(unix)]
    {
        // Tokio's flag is only a direct-child fallback; process-group
        // termination below remains the descendant-custody mechanism.
        command.process_group(0).kill_on_drop(true);
    }
    #[cfg(windows)]
    {
        command.creation_flags(CREATE_NO_WINDOW).kill_on_drop(true);
    }
    let child = command.spawn()?;
    DockerProcessSupervisor::from_spawned_child(child, cleanup_owner, cleanup_deadline)
}
