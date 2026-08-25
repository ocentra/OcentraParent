use crate::constants::MAX_FIELD_BYTES;
use crate::types::ProtocolError;

pub(crate) fn validate_field(value: &[u8]) -> Result<(), ProtocolError> {
    if value.is_empty() {
        return Err(ProtocolError::EmptyField);
    }
    if value.len() > MAX_FIELD_BYTES {
        return Err(ProtocolError::FieldTooLarge);
    }
    Ok(())
}
