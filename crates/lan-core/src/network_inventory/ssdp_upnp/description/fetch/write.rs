use std::{
    io::{self, Write},
    net::TcpStream,
    sync::atomic::AtomicBool,
    time::Instant,
};

use super::super::super::http::io_error;
use super::super::super::SsdpDiscoveryError;
use super::deadline::{is_retryable_timeout, poll_timeout};

pub(super) fn write_all_until(
    stream: &mut TcpStream,
    bytes: &[u8],
    deadline: Instant,
    cancellation: Option<&AtomicBool>,
) -> Result<(), SsdpDiscoveryError> {
    let mut written = 0;
    while written < bytes.len() {
        stream
            .set_write_timeout(Some(poll_timeout(deadline, cancellation)?))
            .map_err(|error| io_error(&error))?;
        match stream.write(&bytes[written..]) {
            Ok(0) => return Err(io_error(&io::Error::from(io::ErrorKind::WriteZero))),
            Ok(count) => written = written.saturating_add(count),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            Err(error) if is_retryable_timeout(&error) => continue,
            Err(error) => return Err(io_error(&error)),
        }
    }
    Ok(())
}
