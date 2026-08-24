use std::io::{Read, Write};
use std::time::Instant;

use super::{deadline_read, deadline_write};
use crate::BrokerError;

pub(super) fn read_exact(
    reader: &mut impl Read,
    buffer: &mut [u8],
    deadline: Instant,
) -> Result<(), BrokerError> {
    deadline_read::read_exact(reader, buffer, deadline)
}

pub(super) fn write_all(
    writer: &mut impl Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), BrokerError> {
    deadline_write::write_all(writer, frame, deadline)
}
