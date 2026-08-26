//! Deadline-bounded reads for the fixed named-pipe client.

use std::io::{self, Read};
use std::thread;
use std::time::{Duration, Instant};

use crate::ClientError;

const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(crate) fn exact(
    reader: &mut impl Read,
    buffer: &mut [u8],
    deadline: Instant,
) -> Result<(), ClientError> {
    let mut offset = 0;
    while offset < buffer.len() {
        if Instant::now() >= deadline {
            return Err(ClientError::Transport);
        }
        match reader.read(&mut buffer[offset..]) {
            Ok(0) => return Err(ClientError::Transport),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => thread::sleep(POLL_INTERVAL),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_error) => return Err(ClientError::Transport),
        }
    }
    Ok(())
}
