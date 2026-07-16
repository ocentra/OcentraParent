use std::time::Duration;

use super::process;

pub(super) fn command_json_records(program: &str, args: &[&str]) -> Vec<serde_json::Value> {
    process::command_stdout(program, args)
        .and_then(json_records_from_stdout)
        .unwrap_or_default()
}

pub(super) fn command_json_records_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Vec<serde_json::Value> {
    process::command_stdout_with_timeout(program, args, timeout)
        .and_then(json_records_from_stdout)
        .unwrap_or_default()
}

pub(super) fn command_json_single(program: &str, args: &[&str]) -> Option<serde_json::Value> {
    command_json_records(program, args).into_iter().next()
}

fn json_records_from_stdout(output: impl AsRef<str>) -> Option<Vec<serde_json::Value>> {
    let value = serde_json::from_str::<serde_json::Value>(output.as_ref()).ok()?;
    value
        .as_array()
        .cloned()
        .or_else(|| value.is_object().then_some(vec![value]))
}
