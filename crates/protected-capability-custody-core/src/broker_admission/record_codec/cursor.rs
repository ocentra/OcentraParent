use crate::platform::{PlatformError, SealedState};

pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub(super) fn take_exact(&mut self, length: usize) -> Result<&'a [u8], PlatformError> {
        let end = self
            .offset
            .checked_add(length)
            .ok_or(PlatformError::Tampered)?;
        let value = self
            .bytes
            .get(self.offset..end)
            .ok_or(PlatformError::Tampered)?;
        self.offset = end;
        Ok(value)
    }

    pub(super) fn take_u8(&mut self) -> Result<u8, PlatformError> {
        self.take_exact(1)?
            .first()
            .copied()
            .ok_or(PlatformError::Tampered)
    }

    pub(super) fn take_u16(&mut self) -> Result<u16, PlatformError> {
        self.take_exact(2)?
            .try_into()
            .map(u16::from_be_bytes)
            .map_err(map_slice_error)
    }

    pub(super) fn take_u32(&mut self) -> Result<u32, PlatformError> {
        self.take_exact(4)?
            .try_into()
            .map(u32::from_be_bytes)
            .map_err(map_slice_error)
    }

    pub(super) fn take_u64(&mut self) -> Result<u64, PlatformError> {
        self.take_exact(8)?
            .try_into()
            .map(u64::from_be_bytes)
            .map_err(map_slice_error)
    }

    pub(super) fn take_array<const LENGTH: usize>(
        &mut self,
    ) -> Result<[u8; LENGTH], PlatformError> {
        self.take_exact(LENGTH)?.try_into().map_err(map_slice_error)
    }

    pub(super) fn take_field(&mut self, maximum: usize) -> Result<Vec<u8>, PlatformError> {
        let length = self.take_u32()? as usize;
        if length == 0 || length > maximum {
            return Err(PlatformError::Tampered);
        }
        Ok(self.take_exact(length)?.to_vec())
    }

    pub(super) fn finish(self) -> Result<(), PlatformError> {
        if self.offset == self.bytes.len() {
            Ok(())
        } else {
            Err(PlatformError::Tampered)
        }
    }
}

fn map_slice_error(_error: std::array::TryFromSliceError) -> PlatformError {
    PlatformError::Tampered
}

pub(super) fn decode_state(value: u8) -> Result<SealedState, PlatformError> {
    match value {
        1 => Ok(SealedState::Prepared),
        2 => Ok(SealedState::CommitAmbiguous),
        3 => Ok(SealedState::AbortAmbiguous),
        4 => Ok(SealedState::Committed),
        5 => Ok(SealedState::Aborted),
        _ => Err(PlatformError::Tampered),
    }
}
