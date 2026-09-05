//! Bounded deadline framing for the fixed named pipe.

use std::time::Instant;

use ocentra_protected_capability_custody_protocol::constants::{
    FRAME_PREFIX_BYTES, MAX_FRAME_BYTES,
};

use crate::ClientError;

pub(crate) fn read_frame(
    reader: &mut impl std::io::Read,
    deadline: Instant,
) -> Result<Vec<u8>, ClientError> {
    let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
    super::io_read::exact(reader, &mut prefix, deadline)?;
    let declared = u32::from_be_bytes(prefix) as usize;
    if declared == 0 || declared > MAX_FRAME_BYTES.saturating_sub(FRAME_PREFIX_BYTES) {
        return Err(ClientError::Protocol(
            ocentra_protected_capability_custody_protocol::types::ProtocolError::InvalidFrameLength,
        ));
    }
    let frame_length = FRAME_PREFIX_BYTES
        .checked_add(declared)
        .ok_or(ClientError::Transport)?;
    let mut frame = Vec::with_capacity(frame_length);
    frame.extend_from_slice(&prefix);
    frame.resize(frame_length, 0);
    super::io_read::exact(reader, &mut frame[FRAME_PREFIX_BYTES..], deadline)?;
    Ok(frame)
}

pub(crate) fn write_frame(
    writer: &mut impl std::io::Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), ClientError> {
    if frame.len() < FRAME_PREFIX_BYTES || frame.len() > MAX_FRAME_BYTES {
        return Err(ClientError::Protocol(
            ocentra_protected_capability_custody_protocol::types::ProtocolError::InvalidFrameLength,
        ));
    }
    super::io_write::all(writer, frame, deadline)
}
