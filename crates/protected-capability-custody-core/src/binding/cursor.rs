use super::{validation, BindingError, BindingField, MAX_FIELD_BYTES};

pub(super) struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub(super) fn remaining(&self) -> usize {
        self.bytes.len().saturating_sub(self.position)
    }

    pub(super) fn take_exact(&mut self, length: usize) -> Result<&'a [u8], BindingError> {
        let end = self
            .position
            .checked_add(length)
            .ok_or(BindingError::InvalidEncoding)?;
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or(BindingError::InvalidEncoding)?;
        self.position = end;
        Ok(value)
    }

    pub(super) fn take_u8(&mut self) -> Result<u8, BindingError> {
        self.take_exact(1)?
            .first()
            .copied()
            .ok_or(BindingError::InvalidEncoding)
    }

    pub(super) fn take_u16(&mut self) -> Result<u16, BindingError> {
        let bytes = self.take_exact(2)?;
        let array = <[u8; 2]>::try_from(bytes).map_err(|_| BindingError::InvalidEncoding)?;
        Ok(u16::from_be_bytes(array))
    }

    pub(super) fn take_u32(&mut self) -> Result<u32, BindingError> {
        let bytes = self.take_exact(4)?;
        let array = <[u8; 4]>::try_from(bytes).map_err(|_| BindingError::InvalidEncoding)?;
        Ok(u32::from_be_bytes(array))
    }

    pub(super) fn take_u64(&mut self) -> Result<u64, BindingError> {
        let bytes = self.take_exact(8)?;
        let array = <[u8; 8]>::try_from(bytes).map_err(|_| BindingError::InvalidEncoding)?;
        Ok(u64::from_be_bytes(array))
    }

    pub(super) fn take_frame(&mut self, field: BindingField) -> Result<Vec<u8>, BindingError> {
        let length = self.take_u32()? as usize;
        if length == 0 {
            return Err(BindingError::EmptyField);
        }
        if length > MAX_FIELD_BYTES {
            return Err(BindingError::FieldTooLarge);
        }
        let value = self.take_exact(length)?.to_vec();
        validation::validate_field(&value, field)?;
        Ok(value)
    }
}
