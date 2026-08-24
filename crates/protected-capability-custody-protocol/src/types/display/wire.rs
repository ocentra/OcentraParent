use std::fmt;

use crate::constants;

use super::super::ProtocolError;

pub(super) fn write(
    error: &ProtocolError,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    match error {
        ProtocolError::UnsupportedVersion(value) => Some(write_version(formatter, *value)),
        ProtocolError::InvalidDomain => Some(formatter.write_str(constants::ERROR_INVALID_DOMAIN)),
        ProtocolError::InvalidMessageKind(value) => Some(write_message_kind(formatter, *value)),
        ProtocolError::InvalidNonce => Some(formatter.write_str(constants::ERROR_INVALID_NONCE)),
        ProtocolError::InvalidCorrelationId => {
            Some(formatter.write_str(constants::ERROR_INVALID_CORRELATION_ID))
        }
        ProtocolError::EmptyField => Some(formatter.write_str(constants::ERROR_EMPTY_FIELD)),
        ProtocolError::FieldTooLarge => Some(formatter.write_str(constants::ERROR_FIELD_TOO_LARGE)),
        _ => None,
    }
}

fn write_version(formatter: &mut fmt::Formatter<'_>, value: u16) -> fmt::Result {
    formatter.write_str(constants::ERROR_UNSUPPORTED_VERSION)?;
    fmt::Display::fmt(&value, formatter)
}

fn write_message_kind(formatter: &mut fmt::Formatter<'_>, value: u8) -> fmt::Result {
    formatter.write_str(constants::ERROR_INVALID_MESSAGE_KIND)?;
    fmt::Display::fmt(&value, formatter)
}
