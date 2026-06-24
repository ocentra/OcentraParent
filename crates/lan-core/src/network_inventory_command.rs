use std::{
    process::{Command, Output, Stdio},
    thread::sleep,
    time::{Duration, Instant},
};

use ocentra_parent_agent_protocol::constants;

use crate::mac_identity::normalize_scan_mac_address;

pub(crate) fn command_json_records(program: &str, args: &[&str]) -> Vec<serde_json::Value> {
    command_stdout(program, args)
        .and_then(|output| {
            let value = serde_json::from_str::<serde_json::Value>(&output).ok()?;
            match value {
                serde_json::Value::Array(values) => Some(values),
                serde_json::Value::Object(_) => Some(vec![value]),
                _ => None,
            }
        })
        .unwrap_or_default()
}

pub(crate) fn command_json_single(program: &str, args: &[&str]) -> Option<serde_json::Value> {
    command_json_records(program, args).into_iter().next()
}

pub(crate) fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = command_output(program, args)?;
    if !output.status.success() {
        return None;
    }
    clean_string(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

fn command_output(program: &str, args: &[&str]) -> Option<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let started_at = Instant::now();
    let timeout =
        Duration::from_millis(constants::lan_pairing::LAN_NETWORK_INVENTORY_COMMAND_TIMEOUT_MS);

    loop {
        if child.try_wait().ok().flatten().is_some() {
            return child.wait_with_output().ok();
        }
        if started_at.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        sleep(Duration::from_millis(25));
    }
}

pub(crate) fn record_text(record: &serde_json::Value, field_name: &str) -> Option<String> {
    record.get(field_name).and_then(value_text)
}

pub(crate) fn value_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(value) => clean_string(Some(value.clone())),
        serde_json::Value::Number(value) => clean_string(Some(value.to_string())),
        _ => None,
    }
}

pub(crate) fn record_u64(record: &serde_json::Value, field_name: &str) -> Option<u64> {
    record.get(field_name).and_then(|value| match value {
        serde_json::Value::Number(value) => value.as_u64(),
        serde_json::Value::String(value) => value.parse().ok(),
        _ => None,
    })
}

fn clean_string(value: Option<String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

pub(crate) fn normalize_mac_address(value: &str) -> Option<String> {
    normalize_scan_mac_address(value)
}
