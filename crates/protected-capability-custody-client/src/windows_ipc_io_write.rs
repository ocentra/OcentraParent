//! Deadline-bounded writes for the fixed named-pipe client.

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::ClientError;

const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(crate) fn all(
    writer: &mut impl Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), ClientError> {
    let mut offset = 0;
    while offset < frame.len() {
        if Instant::now() >= deadline {
            return Err(ClientError::Transport);
        }
        match writer.write(&frame[offset..]) {
            Ok(0) => return Err(ClientError::Transport),
            Ok(written) => offset += written,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(POLL_INTERVAL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_error) => return Err(ClientError::Transport),
        }
    }
    writer.flush().map_err(|_error| ClientError::Transport)
}
