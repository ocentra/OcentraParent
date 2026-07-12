use std::time::Duration;

mod json;
mod probes;
mod process;
mod values;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TargetedArpProbeCommand {
    pub program: &'static str,
    pub args: Vec<String>,
}

pub(crate) fn command_json_records(program: &str, args: &[&str]) -> Vec<serde_json::Value> {
    json::command_json_records(program, args)
}

pub(crate) fn command_json_records_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Vec<serde_json::Value> {
    json::command_json_records_with_timeout(program, args, timeout)
}

pub(crate) fn command_json_single(program: &str, args: &[&str]) -> Option<serde_json::Value> {
    json::command_json_single(program, args)
}

pub(crate) fn command_stdout(program: &str, args: &[&str]) -> Option<String> {
    process::command_stdout(program, args)
}

pub(crate) fn command_stdout_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> Option<String> {
    process::command_stdout_with_timeout(program, args, timeout)
}

pub(crate) fn command_succeeded_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> bool {
    process::command_succeeded_with_timeout(program, args, timeout)
}

pub fn targeted_arp_probe_commands(
    ip_address: &str,
    selected_interface: Option<&str>,
) -> Vec<TargetedArpProbeCommand> {
    probes::targeted_arp_probe_commands(ip_address, selected_interface)
}

pub fn record_text(record: &serde_json::Value, field_name: &str) -> Option<String> {
    values::record_text(record, field_name)
}

pub fn value_text(value: &serde_json::Value) -> Option<String> {
    values::value_text(value)
}

pub fn record_u64(record: &serde_json::Value, field_name: &str) -> Option<u64> {
    values::record_u64(record, field_name)
}

pub fn normalize_mac_address(value: &str) -> Option<String> {
    values::normalize_mac_address(value)
}
