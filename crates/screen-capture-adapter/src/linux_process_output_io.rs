use std::{
    io::{ErrorKind, Read},
    process::ChildStdout,
};

use super::{output::DrainState, MAX_CHILD_STDOUT_BYTES};

const DRAIN_BUFFER_BYTES: usize = 8 * 1024;

pub(super) fn drain(
    pipe: &mut Option<ChildStdout>,
    stdout: &mut Vec<u8>,
    overflow: &mut bool,
    failed: &mut bool,
) -> DrainState {
    let Some(stdout_pipe) = pipe.as_mut() else {
        return DrainState::Closed;
    };
    let mut buffer = [0u8; DRAIN_BUFFER_BYTES];
    match stdout_pipe.read(&mut buffer) {
        Ok(0) => {
            pipe.take();
            DrainState::Closed
        }
        Ok(read) => {
            append_output(stdout, overflow, &buffer[..read]);
            DrainState::Pending
        }
        Err(error) if error.kind() == ErrorKind::WouldBlock => DrainState::Pending,
        Err(error) if error.kind() == ErrorKind::Interrupted => DrainState::Pending,
        Err(_) => {
            pipe.take();
            *failed = true;
            DrainState::Failed
        }
    }
}

fn append_output(stdout: &mut Vec<u8>, overflow: &mut bool, bytes: &[u8]) {
    if stdout.len().saturating_add(bytes.len()) > MAX_CHILD_STDOUT_BYTES {
        stdout.truncate(MAX_CHILD_STDOUT_BYTES);
        *overflow = true;
    } else if !*overflow {
        stdout.extend_from_slice(bytes);
    }
}
