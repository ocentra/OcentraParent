use super::super::super::{Action, BindingError};

pub(super) fn decode(value: u8) -> Result<Action, BindingError> {
    match value {
        1 => Ok(Action::Seal),
        2 => Ok(Action::Rotate),
        3 => Ok(Action::Revoke),
        4 => Ok(Action::Recover),
        _ => Err(BindingError::UnsupportedAction),
    }
}
