mod read;
mod write;

use std::io::{self, Read, Write};
use std::time::{Duration, Instant};

use crate::ClientError;

pub(super) const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn read_frame(
    reader: &mut impl Read,
    deadline: Instant,
) -> Result<Vec<u8>, ClientError> {
    read::read_frame(reader, deadline)
}

pub(super) fn write_frame(
    writer: &mut impl Write,
    frame: &[u8],
    deadline: Instant,
) -> Result<(), ClientError> {
    write::write_frame(writer, frame, deadline)
}

pub(super) fn connection_deadline() -> Result<Instant, ClientError> {
    Instant::now()
        .checked_add(Duration::from_millis(
            ocentra_protected_capability_custody_protocol::constants::BROKER_ACCEPT_DEADLINE_MILLIS,
        ))
        .ok_or(ClientError::Transport)
}

pub(super) fn unix_now_millis() -> Result<u64, ClientError> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| ClientError::PeerAuthentication)?;
    u64::try_from(duration.as_millis()).map_err(|_| ClientError::PeerAuthentication)
}

pub(super) fn map_transport_error(_error: io::Error) -> ClientError {
    ClientError::Transport
}
