use std::{
    process::Child,
    thread,
    time::{Duration, Instant},
};

use super::{
    output::{DrainState, OutputDrain},
    termination, ChildOutcome, ChildResult,
};

const CHILD_POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn monitor_child(
    mut child: Child,
    mut output: OutputDrain,
    deadline: Instant,
) -> ChildResult {
    loop {
        if output.drain() == DrainState::Failed {
            termination::terminate_child_group(&mut child);
            return output.failed_result();
        }
        match child.try_wait() {
            Ok(Some(status)) => {
                return child_exited(&mut child, output, status.success(), deadline);
            }
            Ok(None) => {}
            Err(_) => {
                termination::terminate_child_group(&mut child);
                return output.failed_result();
            }
        }
        let now = Instant::now();
        if now >= deadline {
            return timed_out(&mut child, output);
        }
        thread::sleep(CHILD_POLL_INTERVAL.min(deadline.saturating_duration_since(now)));
    }
}

fn child_exited(
    child: &mut Child,
    mut output: OutputDrain,
    success: bool,
    deadline: Instant,
) -> ChildResult {
    if !termination::terminate_child_group(child) {
        output.close();
        return unavailable_result();
    }
    output.finish_after_exit(success, deadline)
}

fn timed_out(child: &mut Child, mut output: OutputDrain) -> ChildResult {
    let reaped = termination::terminate_child_group(child);
    output.close();
    ChildResult {
        stdout: Vec::new(),
        outcome: if reaped {
            ChildOutcome::TimedOut
        } else {
            ChildOutcome::OutputUnavailable
        },
    }
}

fn unavailable_result() -> ChildResult {
    ChildResult {
        stdout: Vec::new(),
        outcome: ChildOutcome::OutputUnavailable,
    }
}
