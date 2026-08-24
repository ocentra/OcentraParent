use std::{ffi::OsString, time::Instant};

use super::{
    linux_process::{executable_path, run_child, ChildOutcome},
    LinuxActiveWindowObservation, LinuxToolProbe,
};

#[derive(Clone, Debug)]
pub(crate) struct LinuxWindowSelector(u64);

impl LinuxWindowSelector {
    pub(crate) fn xwd_argument(&self) -> OsString {
        OsString::from(format!("0x{:x}", self.0))
    }
}

pub(crate) fn probe_xprop(
    deadline: Instant,
) -> (
    LinuxToolProbe,
    LinuxActiveWindowObservation,
    Option<LinuxWindowSelector>,
) {
    let Some(program) = executable_path("xprop") else {
        return unavailable_observation();
    };
    let result = run_child(
        &program,
        &[
            OsString::from("-root"),
            OsString::from("_NET_ACTIVE_WINDOW"),
        ],
        deadline,
    );
    if !result.succeeded() {
        return (tool_probe_for(&result), not_observed(), None);
    }
    let Some(window_id) = parse_xprop_window_id(&result.stdout) else {
        return (LinuxToolProbe::Failed, not_observed(), None);
    };
    if window_id == 0 {
        return (LinuxToolProbe::Succeeded, not_observed(), None);
    }
    (
        LinuxToolProbe::Succeeded,
        LinuxActiveWindowObservation::Observed,
        Some(LinuxWindowSelector(window_id)),
    )
}

pub(crate) fn probe_xdotool(
    deadline: Instant,
) -> (
    LinuxToolProbe,
    LinuxActiveWindowObservation,
    Option<LinuxWindowSelector>,
) {
    let Some(program) = executable_path("xdotool") else {
        return (LinuxToolProbe::Unavailable, not_observed(), None);
    };
    let result = run_child(&program, &[OsString::from("getactivewindow")], deadline);
    if !result.succeeded() {
        return (tool_probe_for(&result), not_observed(), None);
    }
    let Some(window_id) = parse_decimal_window_id(&result.stdout) else {
        return (LinuxToolProbe::Failed, not_observed(), None);
    };
    if window_id == 0 {
        return (LinuxToolProbe::Succeeded, not_observed(), None);
    }
    (
        LinuxToolProbe::Succeeded,
        LinuxActiveWindowObservation::Observed,
        Some(LinuxWindowSelector(window_id)),
    )
}

fn unavailable_observation() -> (
    LinuxToolProbe,
    LinuxActiveWindowObservation,
    Option<LinuxWindowSelector>,
) {
    (LinuxToolProbe::Unavailable, not_observed(), None)
}

fn not_observed() -> LinuxActiveWindowObservation {
    LinuxActiveWindowObservation::NotObserved
}

fn tool_probe_for(result: &super::linux_process::ChildResult) -> LinuxToolProbe {
    match result.outcome {
        ChildOutcome::SpawnFailed => LinuxToolProbe::Unavailable,
        ChildOutcome::TimedOut | ChildOutcome::Exited(false) | ChildOutcome::OutputTooLarge => {
            LinuxToolProbe::Failed
        }
        ChildOutcome::Exited(true) => LinuxToolProbe::Succeeded,
    }
}

fn parse_xprop_window_id(stdout: &[u8]) -> Option<u64> {
    let text = std::str::from_utf8(stdout).ok()?;
    text.split_whitespace()
        .filter_map(|token| token.strip_prefix("0x"))
        .find_map(|token| u64::from_str_radix(token, 16).ok())
}

fn parse_decimal_window_id(stdout: &[u8]) -> Option<u64> {
    let text = std::str::from_utf8(stdout).ok()?;
    text.split_whitespace()
        .find_map(|token| token.parse::<u64>().ok())
}

pub(crate) fn capture_failure_status(
    result: &super::linux_process::ChildResult,
) -> ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus {
    if matches!(result.outcome, ChildOutcome::Exited(false)) {
        ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus::AccessDenied
    } else {
        ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus::AdapterError
    }
}
