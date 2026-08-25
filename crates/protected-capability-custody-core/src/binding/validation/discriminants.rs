mod action;
mod generation;
mod target;

use super::super::{Action, BindingError, GenerationSlotName, TargetKind};

pub(super) fn action(value: u8) -> Result<Action, BindingError> {
    action::decode(value)
}

pub(super) fn target(value: u8) -> Result<TargetKind, BindingError> {
    target::decode(value)
}

pub(super) fn generation(value: u8) -> Result<GenerationSlotName, BindingError> {
    generation::decode(value)
}
