use std::fmt;

use crate::constants;

use super::super::ProtocolError;

pub(super) fn write(
    error: &ProtocolError,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    match error {
        ProtocolError::UnsupportedRequest(value) => Some(write_request(formatter, *value)),
        ProtocolError::UnsupportedAction(value) => Some(write_action(formatter, *value)),
        ProtocolError::UnsupportedTarget(value) => Some(write_target(formatter, *value)),
        ProtocolError::UnsupportedStatus(value) => Some(write_status(formatter, *value)),
        _ => None,
    }
}

fn write_request(formatter: &mut fmt::Formatter<'_>, value: u8) -> fmt::Result {
    formatter.write_str(constants::ERROR_UNSUPPORTED_REQUEST)?;
    fmt::Display::fmt(&value, formatter)
}

fn write_action(formatter: &mut fmt::Formatter<'_>, value: u8) -> fmt::Result {
    formatter.write_str(constants::ERROR_UNSUPPORTED_ACTION)?;
    fmt::Display::fmt(&value, formatter)
}

fn write_target(formatter: &mut fmt::Formatter<'_>, value: u8) -> fmt::Result {
    formatter.write_str(constants::ERROR_UNSUPPORTED_TARGET)?;
    fmt::Display::fmt(&value, formatter)
}

fn write_status(formatter: &mut fmt::Formatter<'_>, value: u8) -> fmt::Result {
    formatter.write_str(constants::ERROR_UNSUPPORTED_STATUS)?;
    fmt::Display::fmt(&value, formatter)
}
