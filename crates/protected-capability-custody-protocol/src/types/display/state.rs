use std::fmt;

use super::super::ProtocolError;

mod authentication;
mod session;

pub(super) fn write(
    error: &ProtocolError,
    formatter: &mut fmt::Formatter<'_>,
) -> Option<fmt::Result> {
    if let Some(result) = authentication::write(error, formatter) {
        return Some(result);
    }
    session::write(error, formatter)
}
