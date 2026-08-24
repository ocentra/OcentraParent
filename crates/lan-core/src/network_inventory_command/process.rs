use std::{
    process::{Child, Command, Output, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread::sleep,
    time::{Duration, Instant},
};

use ocentra_parent_agent_protocol::constants;

use super::values::clean_string;

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
    let output = command_output_with_timeout(program, args, timeout)?;
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
    command_output_with_timeout(program, args, timeout)
        .is_some_and(|output| output.status.success())
}

fn command_output_with_timeout(program: &str, args: &[&str], timeout: Duration) -> Option<Output> {
    command_output_with_timeout_and_cancellation(program, args, timeout, None)
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
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let started_at = Instant::now();

    loop {
        if cancellation.is_some_and(|value| value.load(Ordering::Acquire)) {
            terminate_process_tree(&mut child);
            return None;
        }
        if child.try_wait().ok().flatten().is_some() {
            return child.wait_with_output().ok();
        }
        let elapsed = started_at.elapsed();
        if elapsed >= timeout {
            terminate_process_tree(&mut child);
            return None;
        }
        let remaining = timeout.saturating_sub(elapsed);
        sleep(remaining.min(Duration::from_millis(25)));
    }
}

fn terminate_process_tree(child: &mut Child) {
    #[cfg(windows)]
    terminate_windows_process_tree(child);

    // The platform-specific tree terminator is best effort. Always terminate
    // and reap the owned child as a final boundary so a timed-out command
    // cannot leave a child handle or pipe held by this worker.
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(windows)]
fn terminate_windows_process_tree(child: &mut Child) {
    let pid = child.id().to_string();
    let Ok(mut taskkill) = Command::new("taskkill")
        .args(["/PID", pid.as_str(), "/T", "/F"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    else {
        return;
    };
    let deadline = Instant::now() + Duration::from_secs(2);
    loop {
        match taskkill.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if Instant::now() < deadline => sleep(Duration::from_millis(10)),
            _ => {
                let _ = taskkill.kill();
                let _ = taskkill.wait();
                break;
            }
        }
    }
}
