#[path = "io_read.rs"]
mod read;
#[path = "io_write.rs"]
mod write;

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

use super::{PipeStream, TransportError};

const POLL_INTERVAL: Duration = Duration::from_millis(10);
const IO_TIMEOUT: Duration = Duration::from_secs(15);

pub(super) fn read_frame(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
) -> Result<Vec<u8>, TransportError> {
    read::read_frame(parent, stream)
}

pub(super) fn write_frame(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    body: &[u8],
) -> Result<(), TransportError> {
    write::write_frame(parent, stream, body)
}

pub(super) fn deadline() -> Result<Instant, TransportError> {
    Instant::now()
        .checked_add(IO_TIMEOUT)
        .ok_or(TransportError::Timeout)
}

pub(super) fn flush(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    deadline: Instant,
) -> Result<(), TransportError> {
    loop {
        super::endpoint::verify_client(parent, stream)?;
        if Instant::now() >= deadline {
            return Err(TransportError::Timeout);
        }
        match stream.flush() {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(POLL_INTERVAL);
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return Err(TransportError::Io),
        }
    }
}
