use super::super::{BindingError, BindingField, MAX_FIELD_BYTES};

pub(super) fn validate(value: &[u8], _field: BindingField) -> Result<(), BindingError> {
    if value.is_empty() {
        return Err(BindingError::EmptyField);
    }
    if value.len() > MAX_FIELD_BYTES {
        return Err(BindingError::FieldTooLarge);
    }
    Ok(())
}
