//! Bounded big-endian TPM response cursors.

use super::{TPM_HEADER_BYTES, TPM_RC_SUCCESS};
use crate::{Error, Result, MAX_BUFFER_BYTES};

pub(super) struct ResponseCursor<'a> {
    bytes: &'a [u8],
    position: usize,
    response_code: u32,
}

impl<'a> ResponseCursor<'a> {
    pub(super) fn new(bytes: &'a [u8], expected_tag: u16) -> Result<Self> {
        if bytes.len() < TPM_HEADER_BYTES || bytes.len() > MAX_BUFFER_BYTES {
            return Err(Error::MalformedTpm);
        }
        let tag = u16::from_be_bytes([bytes[0], bytes[1]]);
        let size = u32::from_be_bytes([bytes[2], bytes[3], bytes[4], bytes[5]]) as usize;
        let response_code = u32::from_be_bytes([bytes[6], bytes[7], bytes[8], bytes[9]]);
        if size != bytes.len() {
            return Err(Error::MalformedTpm);
        }
        if response_code != TPM_RC_SUCCESS {
            return Err(Error::Tpm(response_code));
        }
        if tag != expected_tag {
            return Err(Error::MalformedTpm);
        }
        Ok(Self {
            bytes,
            position: TPM_HEADER_BYTES,
            response_code,
        })
    }

    pub(super) fn expect_response_code(&self) -> Result<()> {
        if self.response_code != TPM_RC_SUCCESS {
            return Err(Error::Tpm(self.response_code));
        }
        Ok(())
    }

    pub(super) fn take(&mut self, length: usize) -> Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(Error::MalformedTpm)?;
        if end > self.bytes.len() {
            return Err(Error::MalformedTpm);
        }
        let value = &self.bytes[self.position..end];
        self.position = end;
        Ok(value)
    }

    pub(super) fn take_tpm2b(&mut self) -> Result<&'a [u8]> {
        let length = self.take_u16()? as usize;
        self.take(length)
    }

    pub(super) fn take_u16(&mut self) -> Result<u16> {
        let bytes = self.take(2)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub(super) fn is_empty(&self) -> bool {
        self.position == self.bytes.len()
    }
}

pub(super) struct SliceCursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> SliceCursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub(super) fn take(&mut self, length: usize) -> Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(Error::MalformedTpm)?;
        if end > self.bytes.len() {
            return Err(Error::MalformedTpm);
        }
        let value = &self.bytes[self.position..end];
        self.position = end;
        Ok(value)
    }

    pub(super) fn take_u16(&mut self) -> Result<u16> {
        let bytes = self.take(2)?;
        Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
    }

    pub(super) fn take_u32(&mut self) -> Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    pub(super) fn take_tpm2b(&mut self) -> Result<&'a [u8]> {
        let length = self.take_u16()? as usize;
        self.take(length)
    }

    pub(super) fn is_empty(&self) -> bool {
        self.position == self.bytes.len()
    }
}
