use std::{io::Read, process::Child};

use super::{ChildOutcome, ChildResult, MAX_CHILD_STDOUT_BYTES};

pub(super) fn exited_result(child: &mut Child, success: bool) -> ChildResult {
    let mut stdout = Vec::new();
    let outcome = match child.stdout.take() {
        Some(pipe) => match pipe
            .take((MAX_CHILD_STDOUT_BYTES + 1) as u64)
            .read_to_end(&mut stdout)
        {
            Ok(_) if stdout.len() <= MAX_CHILD_STDOUT_BYTES => ChildOutcome::Exited(success),
            Ok(_) => {
                stdout.truncate(MAX_CHILD_STDOUT_BYTES);
                ChildOutcome::OutputTooLarge
            }
            Err(_) => {
                stdout.clear();
                ChildOutcome::OutputTooLarge
            }
        },
        None => ChildOutcome::Exited(success),
    };
    ChildResult { stdout, outcome }
}
