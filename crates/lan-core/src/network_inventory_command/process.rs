use std::{
    process::{Output, Stdio},
    sync::atomic::AtomicBool,
    thread,
    time::{Duration, Instant},
};

use command_group::{AsyncCommandGroup, AsyncGroupChild};
use ocentra_parent_agent_protocol::constants;
use tokio::process::Command;

use super::values::clean_string;

mod executable;
mod output;
mod wait;

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;

pub(super) fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let timeout =
        Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS);
    command_stdout_with_timeout(program, args, timeout)
}

pub(super) fn command_stdout_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Option<String> {
    let output = command_output_with_timeout_and_cancellation(program, args, timeout, None)?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).to_string())
        .and_then(|value| clean_string(&value))
}

pub(super) fn command_succeeded_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> bool {
    command_output_with_timeout_and_cancellation(program, args, timeout, None)
        .is_some_and(|output| output.status.success())
}

pub(super) fn command_stdout_with_timeout_and_cancellation(
    program: &str,
    args: &[&str],
    timeout: Duration,
    cancellation: &AtomicBool,
) -> Option<String> {
    let output =
        command_output_with_timeout_and_cancellation(program, args, timeout, Some(cancellation))?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).to_string())
        .and_then(|value| clean_string(&value))
}

fn command_output_with_timeout_and_cancellation(
    program: &str,
    args: &[&str],
    timeout: Duration,
    cancellation: Option<&AtomicBool>,
) -> Option<Output> {
    if timeout.is_zero() {
        return None;
    }
    let deadline = Instant::now().checked_add(timeout)?;
    thread::scope(|scope| {
        let worker = thread::Builder::new()
            .name("lan-network-inventory-command".to_string())
            .spawn_scoped(scope, || {
                run_on_current_thread_runtime(program, args, deadline, cancellation)
            })
            .ok()?;
        worker.join().ok().flatten()
    })
}

fn run_on_current_thread_runtime(
    program: &str,
    args: &[&str],
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<Output> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .ok()?;
    runtime.block_on(command_output_async(program, args, deadline, cancellation))
}

async fn command_output_async(
    program: &str,
    args: &[&str],
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Option<Output> {
    let executable = executable::resolve_trusted_executable(program)?;
    let mut command = Command::new(executable);
    command
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = spawn_process_group(&mut command).ok()?;
    let group = wait::ProcessGroup::capture(&child);
    let Some(stdout) = child.inner().stdout.take() else {
        wait::terminate_group_bounded(&mut child, group, false, deadline).await;
        return None;
    };
    let Some(stderr) = child.inner().stderr.take() else {
        wait::terminate_group_bounded(&mut child, group, false, deadline).await;
        return None;
    };
    let terminate = AtomicBool::new(false);

    let (stdout, stderr, status) = tokio::join!(
        output::read_bounded_until(stdout, deadline, cancellation, &terminate),
        output::read_bounded_until(stderr, deadline, cancellation, &terminate),
        wait::wait_bounded(&mut child, group, deadline, cancellation, &terminate),
    );
    let status = status?;
    if !stdout.complete() || !stderr.complete() {
        return None;
    }
    Some(Output {
        status,
        stdout: stdout.bytes,
        stderr: stderr.bytes,
    })
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
    command.group().kill_on_drop(true).spawn()
}
