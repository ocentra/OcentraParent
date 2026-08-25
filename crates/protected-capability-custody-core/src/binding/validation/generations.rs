use super::super::{BindingError, GenerationSlot, GenerationSlotName, GENERATION_SLOT_COUNT};

pub(super) fn validate(
    generations: &[GenerationSlot; GENERATION_SLOT_COUNT],
) -> Result<(), BindingError> {
    let expected = [
        GenerationSlotName::Authority,
        GenerationSlotName::Target,
        GenerationSlotName::Key,
        GenerationSlotName::Writer,
    ];
    for (actual, expected) in generations.iter().zip(expected) {
        if actual.name != expected {
            return Err(BindingError::DuplicateGeneration);
        }
        if actual.value == 0 {
            return Err(BindingError::ZeroGeneration);
        }
    }
    Ok(())
}
