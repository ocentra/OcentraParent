use std::io::{self, Read};

use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

use super::super::{endpoint, PipeStream, TransportError};
use crate::protocol::{FRAME_PREFIX_BYTES, MAXIMUM_FRAME_BYTES};

#[path = "io_read_prefix.rs"]
mod prefix;

pub(super) fn read_frame(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
) -> Result<Vec<u8>, TransportError> {
    let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
    prefix::read(parent, stream, &mut prefix)?;
    let declared = u32::from_be_bytes(prefix) as usize;
    if declared == 0 || declared > MAXIMUM_FRAME_BYTES.saturating_sub(FRAME_PREFIX_BYTES) {
        return Err(TransportError::InvalidFrame);
    }
    let deadline = super::deadline()?;
    let mut body = vec![0_u8; declared];
    read_exact(parent, stream, &mut body, deadline)?;
    endpoint::verify_client(parent, stream)?;
    Ok(body)
}

fn read_exact(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    buffer: &mut [u8],
    deadline: std::time::Instant,
) -> Result<(), TransportError> {
    let mut offset = 0_usize;
    while offset < buffer.len() {
        endpoint::verify_client(parent, stream)?;
        if std::time::Instant::now() >= deadline {
            return Err(TransportError::Timeout);
        }
        match stream.read(&mut buffer[offset..]) {
            Ok(0) => return Err(TransportError::Io),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(super::POLL_INTERVAL);
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return Err(TransportError::Io),
        }
    }
    Ok(())
}
