use std::io::{self, Read};
use std::thread;
use std::time::{Duration, Instant};

use crate::{map_transport_error, BrokerError};

pub(super) fn read_exact(
    reader: &mut impl Read,
    buffer: &mut [u8],
    deadline: Instant,
) -> Result<(), BrokerError> {
    let mut offset = 0;
    while offset < buffer.len() {
        if Instant::now() >= deadline {
            return Err(BrokerError::Transport);
        }
        match reader.read(&mut buffer[offset..]) {
            Ok(0) => return Err(BrokerError::Transport),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(map_transport_error(error)),
        }
    }
    Ok(())
}
