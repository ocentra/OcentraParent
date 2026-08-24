use std::{ffi::OsString, time::Instant};

use super::{
    linux_process::{executable_path, run_child, ChildOutcome},
    LinuxActiveWindowObservation, LinuxToolProbe,
};

pub(crate) fn probe_xprop(deadline: Instant) -> (LinuxToolProbe, LinuxActiveWindowObservation) {
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
        return (tool_probe_for(&result), not_observed());
    }
    let Some(observed) = parse_xprop_observation(&result.stdout) else {
        return (LinuxToolProbe::Failed, not_observed());
    };
    (LinuxToolProbe::Succeeded, observation(observed))
}

pub(crate) fn probe_xdotool(deadline: Instant) -> (LinuxToolProbe, LinuxActiveWindowObservation) {
    let Some(program) = executable_path("xdotool") else {
        return (LinuxToolProbe::Unavailable, not_observed());
    };
    let result = run_child(&program, &[OsString::from("getactivewindow")], deadline);
    if !result.succeeded() {
        return (tool_probe_for(&result), not_observed());
    }
    let Some(observed) = parse_decimal_observation(&result.stdout) else {
        return (LinuxToolProbe::Failed, not_observed());
    };
    (LinuxToolProbe::Succeeded, observation(observed))
}

fn unavailable_observation() -> (LinuxToolProbe, LinuxActiveWindowObservation) {
    (LinuxToolProbe::Unavailable, not_observed())
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
        ChildOutcome::OutputUnavailable => LinuxToolProbe::Unavailable,
        ChildOutcome::Exited(true) => LinuxToolProbe::Succeeded,
    }
}

fn parse_xprop_observation(stdout: &[u8]) -> Option<bool> {
    let text = std::str::from_utf8(stdout).ok()?;
    text.split_whitespace()
        .find_map(|token| token.strip_prefix("0x"))
        .filter(|token| !token.is_empty() && token.chars().all(|value| value.is_ascii_hexdigit()))
        .map(|token| token.chars().any(|value| value != '0'))
}

fn parse_decimal_observation(stdout: &[u8]) -> Option<bool> {
    let text = std::str::from_utf8(stdout).ok()?;
    text.split_whitespace()
        .find(|token| !token.is_empty() && token.chars().all(|value| value.is_ascii_digit()))
        .map(|token| token.chars().any(|value| value != '0'))
}

fn observation(observed: bool) -> LinuxActiveWindowObservation {
    if observed {
        LinuxActiveWindowObservation::Observed
    } else {
        not_observed()
    }
}
