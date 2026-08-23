mod discriminants;
mod fields;
mod generations;

use super::{BindingError, BindingField, GenerationSlot, GENERATION_SLOT_COUNT};

pub(super) fn validate_field(value: &[u8], field: BindingField) -> Result<(), BindingError> {
    fields::validate(value, field)
}

pub(super) fn validate_generation_order(
    generations: &[GenerationSlot; GENERATION_SLOT_COUNT],
) -> Result<(), BindingError> {
    generations::validate(generations)
}

pub(super) fn decode_action(value: u8) -> Result<super::Action, BindingError> {
    discriminants::action(value)
}

pub(super) fn decode_target(value: u8) -> Result<super::TargetKind, BindingError> {
    discriminants::target(value)
}

pub(super) fn decode_generation(value: u8) -> Result<super::GenerationSlotName, BindingError> {
    discriminants::generation(value)
}
