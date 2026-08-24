use crate::constants::{FRAME_PREFIX_BYTES, MAX_FIELD_BYTES, MAX_FRAME_BYTES, PROTOCOL_DOMAIN};
use crate::types::{ProtocolError, ProtocolVersion};

pub(super) fn append_header(payload: &mut Vec<u8>, message_kind: u8, version: ProtocolVersion) {
    payload.extend_from_slice(PROTOCOL_DOMAIN);
    payload.push(message_kind);
    payload.extend_from_slice(&version.value().to_be_bytes());
}

pub(super) fn encode_frame(payload: Vec<u8>) -> Result<Vec<u8>, ProtocolError> {
    let frame_length = FRAME_PREFIX_BYTES
        .checked_add(payload.len())
        .ok_or(ProtocolError::FrameTooLarge)?;
    if frame_length > MAX_FRAME_BYTES {
        return Err(ProtocolError::FrameTooLarge);
    }
    let length = u32::try_from(payload.len()).map_err(|_| ProtocolError::FrameTooLarge)?;
    let mut frame = Vec::with_capacity(frame_length);
    frame.extend_from_slice(&length.to_be_bytes());
    frame.extend_from_slice(&payload);
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
            .map_err(|_| ProtocolError::InvalidFrameLength)?,
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
    let length = u32::try_from(value.len()).map_err(|_| ProtocolError::FieldTooLarge)?;
    payload.extend_from_slice(&length.to_be_bytes());
    payload.extend_from_slice(value);
    Ok(())
}

pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(super) fn take_exact(&mut self, length: usize) -> Result<&'a [u8], ProtocolError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(ProtocolError::Truncated)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(ProtocolError::Truncated)?;
        self.offset = end;
        Ok(value)
    }

    pub(super) fn take_header(
        &mut self,
        expected_message_kind: u8,
    ) -> Result<ProtocolVersion, ProtocolError> {
        if self.take_exact(PROTOCOL_DOMAIN.len())? != PROTOCOL_DOMAIN {
            return Err(ProtocolError::InvalidDomain);
        }
        let message_kind = self.take_u8()?;
        if message_kind != expected_message_kind {
            return Err(ProtocolError::InvalidMessageKind(message_kind));
        }
        ProtocolVersion::decode(self.take_u16()?)
    }

    pub(super) fn take_u8(&mut self) -> Result<u8, ProtocolError> {
        Ok(*self
            .take_exact(1)?
            .first()
            .ok_or(ProtocolError::Truncated)?)
    }

    pub(super) fn take_u16(&mut self) -> Result<u16, ProtocolError> {
        Ok(u16::from_be_bytes(
            self.take_exact(2)?
                .try_into()
                .map_err(|_| ProtocolError::Truncated)?,
        ))
    }

    pub(super) fn take_u64(&mut self) -> Result<u64, ProtocolError> {
        Ok(u64::from_be_bytes(
            self.take_exact(8)?
                .try_into()
                .map_err(|_| ProtocolError::Truncated)?,
        ))
    }

    pub(super) fn take_field(&mut self) -> Result<Vec<u8>, ProtocolError> {
        let length = u32::from_be_bytes(
            self.take_exact(4)?
                .try_into()
                .map_err(|_| ProtocolError::Truncated)?,
        ) as usize;
        if length > MAX_FIELD_BYTES {
            return Err(ProtocolError::FieldTooLarge);
        }
        Ok(self.take_exact(length)?.to_vec())
    }

    pub(super) fn finish(self) -> Result<(), ProtocolError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(ProtocolError::TrailingBytes)
        }
    }
}
