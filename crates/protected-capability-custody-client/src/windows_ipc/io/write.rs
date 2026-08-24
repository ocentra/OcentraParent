use std::io::{self, Write};
use std::thread;
use std::time::Instant;

use super::POLL_INTERVAL;
use crate::ClientError;

pub(super) fn write_frame(
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
            Err(error) => return Err(map_transport_error(error)),
        }
    }
    writer.flush().map_err(map_transport_error)
}

fn map_transport_error(_error: io::Error) -> ClientError {
    ClientError::Transport
}
