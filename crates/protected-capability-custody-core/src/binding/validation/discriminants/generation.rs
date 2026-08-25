use super::super::super::{BindingError, GenerationSlotName};

pub(super) fn decode(value: u8) -> Result<GenerationSlotName, BindingError> {
    match value {
        1 => Ok(GenerationSlotName::Authority),
        2 => Ok(GenerationSlotName::Target),
        3 => Ok(GenerationSlotName::Key),
        4 => Ok(GenerationSlotName::Writer),
        _ => Err(BindingError::UnknownGenerationSlot),
    }
}
