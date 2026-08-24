use std::io::{self, Read};
use std::thread;
use std::time::Instant;

use ocentra_protected_capability_custody_protocol::constants::{
    FRAME_PREFIX_BYTES, MAX_FRAME_BYTES,
};

use super::POLL_INTERVAL;
use crate::ClientError;

pub(super) fn read_frame(
    reader: &mut impl Read,
    deadline: Instant,
) -> Result<Vec<u8>, ClientError> {
    let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
    read_exact(reader, &mut prefix, deadline)?;
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
    read_exact(reader, &mut frame[FRAME_PREFIX_BYTES..], deadline)?;
    Ok(frame)
}

fn read_exact(
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
            Err(error) => return Err(map_transport_error(error)),
        }
    }
    Ok(())
}

fn map_transport_error(_error: io::Error) -> ClientError {
    ClientError::Transport
}
