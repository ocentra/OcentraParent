use std::process::Command;

use ocentra_parent_agent_protocol::constants;

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

pub(crate) fn command_json_single_owned(
    program: &str,
    args: Vec<String>,
) -> Option<serde_json::Value> {
    command_stdout_owned(program, args)
        .and_then(|output| serde_json::from_str::<serde_json::Value>(&output).ok())
}

pub(crate) fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    clean_string(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

pub(crate) fn command_stdout_owned(program: &str, args: Vec<String>) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }
    clean_string(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

pub(crate) fn record_text(record: &serde_json::Value, key: &str) -> Option<String> {
    record.get(key).and_then(value_text)
}

pub(crate) fn value_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(value) => clean_string(Some(value.clone())),
        serde_json::Value::Number(value) => clean_string(Some(value.to_string())),
        _ => None,
    }
}

pub(crate) fn record_u64(record: &serde_json::Value, key: &str) -> Option<u64> {
    record.get(key).and_then(|value| match value {
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

pub(crate) fn normalize_mac_address(value: String) -> Option<String> {
    let normalized = value
        .trim()
        .replace(':', constants::lan_pairing::MAC_DASH)
        .to_ascii_lowercase();
    let compact: String = normalized
        .chars()
        .filter(|character| *character != '-')
        .collect();
    if compact.len() != 12
        || compact == constants::lan_pairing::MAC_ZERO_COMPACT
        || compact == constants::lan_pairing::MAC_BROADCAST_COMPACT
        || compact.starts_with(constants::lan_pairing::MAC_IPV4_MULTICAST_PREFIX_COMPACT)
    {
        return None;
    }
    Some(normalized)
}
