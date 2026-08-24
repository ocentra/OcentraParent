use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::{map_transport_error, BrokerError};

pub(super) fn write_all(
    writer: &mut impl Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), BrokerError> {
    let mut offset = 0;
    while offset < frame.len() {
        if Instant::now() >= deadline {
            return Err(BrokerError::Transport);
        }
        match writer.write(&frame[offset..]) {
            Ok(0) => return Err(BrokerError::Transport),
            Ok(written) => offset += written,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(map_transport_error(error)),
        }
    }
    writer.flush().map_err(map_transport_error)
}
