use std::io;
use std::thread;
use std::time::Duration;

use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

use super::super::{PipeListenerType, PipeStream, TransportError};

const POLL_INTERVAL: Duration = Duration::from_millis(10);

pub(super) fn accept(
    listener: &PipeListenerType,
    parent: &ParentProcessObservation,
) -> Result<PipeStream, TransportError> {
    loop {
        parent.current().map_err(|_error| TransportError::Parent)?;
        match listener.accept() {
            Ok(stream) => match accepted_stream(parent, stream) {
                Ok(stream) => return Ok(stream),
                Err(TransportError::PeerRejected) => continue,
                Err(error) => return Err(error),
            },
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(POLL_INTERVAL);
            }
            Err(_) => return Err(TransportError::Io),
        }
    }
}

fn accepted_stream(
    parent: &ParentProcessObservation,
    stream: PipeStream,
) -> Result<PipeStream, TransportError> {
    match verify_client(parent, &stream) {
        Ok(()) => Ok(stream),
        Err(TransportError::PeerRejected) => Err(TransportError::PeerRejected),
        Err(error) => Err(error),
    }
}

pub(super) fn verify_client(
    parent: &ParentProcessObservation,
    stream: &PipeStream,
) -> Result<(), TransportError> {
    let client = parent
        .bind_named_pipe_client(stream)
        .map_err(|_error| TransportError::PeerRejected)?;
    client
        .verify_current()
        .map_err(|_error| TransportError::Parent)
}
