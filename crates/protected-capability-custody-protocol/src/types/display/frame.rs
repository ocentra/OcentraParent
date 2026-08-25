use std::fmt;

use crate::constants;

use super::super::ProtocolError;

pub(super) fn write(
    error: &ProtocolError,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    match error {
        ProtocolError::EmptyFrame => Some(formatter.write_str(constants::ERROR_EMPTY_FRAME)),
        ProtocolError::FrameTooLarge => Some(formatter.write_str(constants::ERROR_FRAME_TOO_LARGE)),
        ProtocolError::InvalidFrameLength => {
            Some(formatter.write_str(constants::ERROR_INVALID_FRAME_LENGTH))
        }
        ProtocolError::TrailingBytes => Some(formatter.write_str(constants::ERROR_TRAILING_BYTES)),
        _ => None,
    }
}
