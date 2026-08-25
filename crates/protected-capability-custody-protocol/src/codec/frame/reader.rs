use crate::constants::{MAX_FIELD_BYTES, PROTOCOL_DOMAIN};
use crate::types::{ProtocolError, ProtocolVersion};

pub(in crate::codec) struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(in crate::codec) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(in crate::codec) fn take_exact(
        &mut self,
        length: usize,
    ) -> Result<&'a [u8], ProtocolError> {
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

    pub(in crate::codec) fn take_header(
        &mut self,
        expected_message_kind: u8,
    ) -> Result<ProtocolVersion, ProtocolError> {
        if self.take_exact(PROTOCOL_DOMAIN.len())? != PROTOCOL_DOMAIN.as_bytes() {
            return Err(ProtocolError::InvalidDomain);
        }
        let message_kind = self.take_u8()?;
        if message_kind != expected_message_kind {
            return Err(ProtocolError::InvalidMessageKind(message_kind));
        }
        ProtocolVersion::decode(self.take_u16()?)
    }

    pub(in crate::codec) fn take_u8(&mut self) -> Result<u8, ProtocolError> {
        self.take_exact(1)?
            .first()
            .copied()
            .ok_or(ProtocolError::Truncated)
    }

    pub(in crate::codec) fn take_u16(&mut self) -> Result<u16, ProtocolError> {
        Ok(u16::from_be_bytes(
            self.take_exact(2)?.try_into().map_err(map_truncated)?,
        ))
    }

    pub(in crate::codec) fn take_u32(&mut self) -> Result<u32, ProtocolError> {
        Ok(u32::from_be_bytes(
            self.take_exact(4)?.try_into().map_err(map_truncated)?,
        ))
    }

    pub(in crate::codec) fn take_u64(&mut self) -> Result<u64, ProtocolError> {
        Ok(u64::from_be_bytes(
            self.take_exact(8)?.try_into().map_err(map_truncated)?,
        ))
    }

    pub(in crate::codec) fn take_field(&mut self) -> Result<Vec<u8>, ProtocolError> {
        let length =
            u32::from_be_bytes(self.take_exact(4)?.try_into().map_err(map_truncated)?) as usize;
        if length > MAX_FIELD_BYTES {
            return Err(ProtocolError::FieldTooLarge);
        }
        Ok(self.take_exact(length)?.to_vec())
    }

    pub(in crate::codec) fn finish(self) -> Result<(), ProtocolError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(ProtocolError::TrailingBytes)
        }
    }
}

fn map_truncated(_error: std::array::TryFromSliceError) -> ProtocolError {
    ProtocolError::Truncated
}
