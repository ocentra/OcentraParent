use super::super::{BindingError, BindingField, MAX_FIELD_BYTES, MAX_LOCATOR_BYTES};

pub(super) fn validate(value: &[u8], field: BindingField) -> Result<(), BindingError> {
    if value.is_empty() {
        return Err(BindingError::EmptyField);
    }
    if value.len() > max_bytes(field) {
        return Err(BindingError::FieldTooLarge);
    }
    Ok(())
}

pub(super) fn max_bytes(field: BindingField) -> usize {
    match field {
        BindingField::Locator => MAX_LOCATOR_BYTES,
        BindingField::Operation
        | BindingField::Household
        | BindingField::Device
        | BindingField::Target => MAX_FIELD_BYTES,
    }
}
