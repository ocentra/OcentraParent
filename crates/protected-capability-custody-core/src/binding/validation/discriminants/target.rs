use super::super::super::{BindingError, TargetKind};

pub(super) fn decode(value: u8) -> Result<TargetKind, BindingError> {
    match value {
        1 => Ok(TargetKind::Device),
        2 => Ok(TargetKind::Household),
        3 => Ok(TargetKind::Capability),
        _ => Err(BindingError::UnsupportedTarget),
    }
}
