use std::io::{Read, Write};

use crate::constants::{FRAME_PREFIX_BYTES, MAX_FIELD_BYTES, MAX_FRAME_BYTES, PROTOCOL_DOMAIN};
use crate::types::{ProtocolError, ProtocolVersion};

pub(super) mod reader;

pub(super) fn append_header(payload: &mut Vec<u8>, message_kind: u8, version: ProtocolVersion) {
    payload.extend_from_slice(PROTOCOL_DOMAIN.as_bytes());
    payload.push(message_kind);
    payload.extend_from_slice(&version.value().to_be_bytes());
}

pub(super) fn encode_frame(payload: &[u8]) -> Result<Vec<u8>, ProtocolError> {
    let frame_length = FRAME_PREFIX_BYTES
        .checked_add(payload.len())
        .ok_or(ProtocolError::FrameTooLarge)?;
    if frame_length > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    let length = u32::try_from(payload.len()).map_err(map_frame_length)?;
    let mut frame = Vec::with_capacity(frame_length);
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(payload);
    Ok(frame)
}

pub(super) fn decode_frame(frame: &[u8]) -> Result<&[u8], ProtocolError> {
    if frame.is_empty() {
        return Err(ProtocolError::EmptyFrame);
    }
    if frame.len() < FRAME_PREFIX_BYTES {
        return Err(ProtocolError::InvalidFrameLength);
    }
    if frame.len() > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    let declared = u32::from_be_bytes(
        frame[..FRAME_PREFIX_BYTES]
            .try_into()
            .map_err(map_invalid_frame_length)?,
    ) as usize;
    let expected_frame_length = declared
        .checked_add(FRAME_PREFIX_BYTES)
        .ok_or(ProtocolError::InvalidFrameLength)?;
    if declared == 0 || expected_frame_length != frame.len() {
        return Err(ProtocolError::InvalidFrameLength);
    }
    Ok(&frame[FRAME_PREFIX_BYTES..])
}

pub(super) fn append_u64(payload: &mut Vec<u8>, value: u64) {
    payload.extend_from_slice(&value.to_be_bytes());
}

pub(super) fn append_field(payload: &mut Vec<u8>, value: &[u8]) -> Result<(), ProtocolError> {
    if !value.is_empty() && value.len() > MAX_FIELD_BYTES {
        return Err(ProtocolError::FieldTooLarge);
    }
    let length = u32::try_from(value.len()).map_err(map_field_length)?;
    payload.extend_from_slice(&length.to_be_bytes());
    payload.extend_from_slice(value);
    Ok(())
}

pub(crate) fn read_frame(reader: &mut impl Read) -> Result<Vec<u8>, ProtocolError> {
    let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
    reader
        .read_exact(&mut prefix)
        .map_err(map_transport_error)?;
    let declared = u32::from_be_bytes(prefix) as usize;
    if declared == 0 || declared > MAX_FRAME_BYTES.saturating_sub(FRAME_PREFIX_BYTES) {
        return Err(ProtocolError::InvalidFrameLength);
    }
    let frame_length = FRAME_PREFIX_BYTES
        .checked_add(declared)
        .ok_or(ProtocolError::FrameTooLarge)?;
    let mut frame = Vec::with_capacity(frame_length);
    frame.extend_from_slice(&prefix);
    frame.resize(frame_length, 0);
    reader
        .read_exact(&mut frame[FRAME_PREFIX_BYTES..])
        .map_err(map_transport_error)?;
    decode_frame(&frame)?;
    Ok(frame)
}

pub(crate) fn write_frame(writer: &mut impl Write, frame: &[u8]) -> Result<(), ProtocolError> {
    decode_frame(frame)?;
    writer.write_all(frame).map_err(map_transport_error)?;
    writer.flush().map_err(map_transport_error)
}

fn map_frame_length(_error: std::num::TryFromIntError) -> ProtocolError {
    ProtocolError::FrameTooLarge
}

fn map_invalid_frame_length(_error: std::array::TryFromSliceError) -> ProtocolError {
    ProtocolError::InvalidFrameLength
}

fn map_field_length(_error: std::num::TryFromIntError) -> ProtocolError {
    ProtocolError::FieldTooLarge
}

fn map_transport_error(_error: std::io::Error) -> ProtocolError {
    ProtocolError::Transport
}
