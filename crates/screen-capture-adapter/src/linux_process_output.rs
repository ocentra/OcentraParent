use std::{
    process::{Child, ChildStdout},
    thread,
    time::{Duration, Instant},
};

use nix::fcntl::{fcntl, FcntlArg, OFlag};

use super::{output_io, ChildOutcome, ChildResult};

const DRAIN_POLL_INTERVAL: Duration = Duration::from_millis(1);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DrainState {
    Pending,
    Closed,
    Failed,
}

pub(super) struct OutputDrain {
    pipe: Option<ChildStdout>,
    stdout: Vec<u8>,
    overflow: bool,
    failed: bool,
}

impl OutputDrain {
    pub(super) fn from_child(child: &mut Child) -> Option<Self> {
        let pipe = child.stdout.take()?;
        let flags = fcntl(&pipe, FcntlArg::F_GETFL).ok()?;
        let flags = OFlag::from_bits_truncate(flags) | OFlag::O_NONBLOCK;
        fcntl(&pipe, FcntlArg::F_SETFL(flags)).ok()?;
        Some(Self {
            pipe: Some(pipe),
            stdout: Vec::new(),
            overflow: false,
            failed: false,
        })
    }

    pub(super) fn drain(&mut self) -> DrainState {
        output_io::drain(
            &mut self.pipe,
            &mut self.stdout,
            &mut self.overflow,
            &mut self.failed,
        )
    }

    pub(super) fn finish_after_exit(mut self, success: bool, deadline: Instant) -> ChildResult {
        loop {
            let state = self.drain();
            if state == DrainState::Closed {
                return self.result(success);
            }
            if state == DrainState::Failed {
                return self.result(false);
            }
            if Instant::now() >= deadline {
                self.close();
                self.failed = true;
                return self.result(false);
            }
            thread::sleep(
                DRAIN_POLL_INTERVAL.min(deadline.saturating_duration_since(Instant::now())),
            );
        }
    }

    pub(super) fn failed_result(mut self) -> ChildResult {
        self.close();
        self.failed = true;
        self.result(false)
    }

    pub(super) fn close(&mut self) {
        self.pipe.take();
    }

    fn result(self, success: bool) -> ChildResult {
        let outcome = if self.failed {
            ChildOutcome::OutputUnavailable
        } else if self.overflow {
            ChildOutcome::OutputTooLarge
        } else {
            ChildOutcome::Exited(success)
        };
        ChildResult {
            stdout: self.stdout,
            outcome,
        }
    }
}
