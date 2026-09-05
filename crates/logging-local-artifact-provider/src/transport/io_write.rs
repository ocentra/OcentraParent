use std::io::{self, Write};

use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

use super::super::{endpoint, PipeStream, TransportError};
use crate::protocol::{FRAME_PREFIX_BYTES, MAXIMUM_FRAME_BYTES};

pub(super) fn write_frame(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    body: &[u8],
) -> Result<(), TransportError> {
    if body.is_empty() || body.len() > MAXIMUM_FRAME_BYTES.saturating_sub(FRAME_PREFIX_BYTES) {
        return Err(TransportError::InvalidFrame);
    }
    let length = u32::try_from(body.len()).map_err(|_error| TransportError::InvalidFrame)?;
    let deadline = super::deadline()?;
    let prefix = length.to_be_bytes();
    write_all(parent, stream, &prefix, deadline)?;
    write_all(parent, stream, body, deadline)?;
    super::flush(parent, stream, deadline)?;
    endpoint::verify_client(parent, stream)
}

fn write_all(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    buffer: &[u8],
    deadline: std::time::Instant,
) -> Result<(), TransportError> {
    let mut offset = 0_usize;
    while offset < buffer.len() {
        endpoint::verify_client(parent, stream)?;
        if std::time::Instant::now() >= deadline {
            return Err(TransportError::Timeout);
        }
        match stream.write(&buffer[offset..]) {
            Ok(0) => return Err(TransportError::Io),
            Ok(written) => offset += written,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                std::thread::sleep(super::POLL_INTERVAL);
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(_) => return Err(TransportError::Io),
        }
    }
    Ok(())
}
