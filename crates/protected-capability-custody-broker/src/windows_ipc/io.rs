mod read;
mod write;

use std::io::{Read, Write};
use std::time::Instant;

use crate::BrokerError;

pub(super) fn read_frame(
    reader: &mut impl Read,
    deadline: Instant,
) -> Result<Vec<u8>, BrokerError> {
    read::read_frame(reader, deadline)
}

pub(super) fn write_frame(
    writer: &mut impl Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), BrokerError> {
    write::write_frame(writer, frame, deadline)
}
