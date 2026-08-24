use std::io::{Read, Write};
use std::time::Instant;

use ocentra_protected_capability_custody_protocol::constants::{
    FRAME_PREFIX_BYTES, MAX_FRAME_BYTES,
};

use crate::BrokerError;

use super::deadline_io;

pub(super) fn read_frame(
    reader: &mut impl Read,
    deadline: Instant,
) -> Result<Vec<u8>, BrokerError> {
    let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
    deadline_io::read_exact(reader, &mut prefix, deadline)?;
    let declared = u32::from_be_bytes(prefix) as usize;
    if declared == 0 || declared > MAX_FRAME_BYTES.saturating_sub(FRAME_PREFIX_BYTES) {
        return Err(BrokerError::Protocol(
            ocentra_protected_capability_custody_protocol::types::ProtocolError::InvalidFrameLength,
        ));
    }
    let frame_length = FRAME_PREFIX_BYTES
        .checked_add(declared)
        .ok_or(BrokerError::Protocol(
            ocentra_protected_capability_custody_protocol::types::ProtocolError::FrameTooLarge,
        ))?;
    let mut frame = Vec::with_capacity(frame_length);
    frame.extend_from_slice(&prefix);
    frame.resize(frame_length, 0);
    deadline_io::read_exact(reader, &mut frame[FRAME_PREFIX_BYTES..], deadline)?;
    Ok(frame)
}

pub(super) fn write_frame(
    writer: &mut impl Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), BrokerError> {
    deadline_io::write_all(writer, frame, deadline)
}
