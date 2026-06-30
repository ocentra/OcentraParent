use std::{
    process::{Command, Output, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use ocentra_parent_agent_protocol::constants;

use crate::mac_identity::normalize_scan_mac_address;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetedArpProbeCommand {
    pub program: &'static str,
    pub args: Vec<String>,
}

pub(crate) fn command_json_records(program: &str, args: &[&str]) -> Vec<serde_json::Value> {
    command_stdout(program, args)
        .and_then(json_records_from_stdout)
        .unwrap_or_default()
}

pub(crate) fn command_json_records_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Vec<serde_json::Value> {
    command_stdout_with_timeout(program, args, timeout)
        .and_then(json_records_from_stdout)
        .unwrap_or_default()
}

fn json_records_from_stdout(output: impl AsRef<str>) -> Option<Vec<serde_json::Value>> {
    let value = serde_json::from_str::<serde_json::Value>(output.as_ref()).ok()?;
    match value {
        serde_json::Value::Array(values) => Some(values),
        serde_json::Value::Object(_) => Some(vec![value]),
        _ => None,
    }
}

pub(crate) fn command_json_single(program: &str, args: &[&str]) -> Option<serde_json::Value> {
    command_json_records(program, args).into_iter().next()
}

pub(crate) fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let timeout =
        Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS);
    command_stdout_with_timeout(program, args, timeout)
}

pub(crate) fn command_stdout_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Option<String> {
    let output = command_output_with_timeout(program, args, timeout)?;
    if !output.status.success() {
        return None;
    }
    clean_string(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

pub(crate) fn command_succeeded_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> bool {
    command_output_with_timeout(program, args, timeout)
        .map(|output| output.status.success())
        .unwrap_or(false)
}

pub fn targeted_arp_probe_commands(
    ip_address: &str,
    selected_interface: Option<&str>,
) -> Vec<TargetedArpProbeCommand> {
    let mut commands = Vec::new();

    if cfg!(target_os = "windows") {
        commands.push(TargetedArpProbeCommand {
            program: constants::lan_pairing::PING_EXE,
            args: vec![
                constants::lan_pairing::PING_WINDOWS_COUNT_ARG.to_string(),
                "1".to_string(),
                constants::lan_pairing::PING_WINDOWS_TIMEOUT_ARG.to_string(),
                "200".to_string(),
                ip_address.to_string(),
            ],
        });
        return commands;
    }

    if cfg!(target_os = "linux") {
        if let Some(selected_interface) = selected_interface
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            commands.push(TargetedArpProbeCommand {
                program: "arping",
                args: vec![
                    "-I".to_string(),
                    selected_interface.to_string(),
                    "-c".to_string(),
                    "1".to_string(),
                    "-w".to_string(),
                    "1".to_string(),
                    ip_address.to_string(),
                ],
            });
        }

        commands.push(TargetedArpProbeCommand {
            program: constants::lan_pairing::PING_EXE,
            args: vec![
                constants::lan_pairing::PING_LINUX_COUNT_ARG.to_string(),
                "1".to_string(),
                constants::lan_pairing::PING_LINUX_TIMEOUT_ARG.to_string(),
                "1".to_string(),
                ip_address.to_string(),
            ],
        });
    }

    commands
}

fn command_output_with_timeout(program: &str, args: &[&str], timeout: Duration) -> Option<Output> {
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
        if child.try_wait().ok().flatten().is_some() {
            return child.wait_with_output().ok();
        }
        let elapsed = started_at.elapsed();
        if elapsed >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        let remaining = timeout.saturating_sub(elapsed);
        sleep(remaining.min(Duration::from_millis(25)));
    }
}

pub fn record_text(record: &serde_json::Value, field_name: &str) -> Option<String> {
    record.get(field_name).and_then(value_text)
}

pub fn value_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(value) => clean_string(Some(value.clone())),
        serde_json::Value::Number(value) => clean_string(Some(value.to_string())),
        _ => None,
    }
}

pub fn record_u64(record: &serde_json::Value, field_name: &str) -> Option<u64> {
    record.get(field_name).and_then(|value| match value {
        serde_json::Value::Number(value) => value.as_u64(),
        serde_json::Value::String(value) => {
            clean_string(Some(value.clone())).and_then(|value| value.parse().ok())
        }
        _ => None,
    })
}

fn clean_string(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub fn normalize_mac_address(value: &str) -> Option<String> {
    normalize_scan_mac_address(value)
}
