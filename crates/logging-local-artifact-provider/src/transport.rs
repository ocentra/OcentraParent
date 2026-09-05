//! Parent-bound named-pipe transport facade.

use interprocess::os::windows::named_pipe::{pipe_mode, DuplexPipeStream, PipeListener};
use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;

#[path = "transport/endpoint.rs"]
pub(crate) mod endpoint;
#[path = "transport/io.rs"]
mod io;

pub(crate) type PipeStream = DuplexPipeStream<pipe_mode::Bytes>;
pub(crate) type PipeListenerType = PipeListener<pipe_mode::Bytes, pipe_mode::Bytes>;

#[derive(Debug)]
pub(crate) enum TransportError {
    Parent,
    Io,
    PeerRejected,
    Timeout,
    InvalidFrame,
}
pub(crate) fn bind(name: &endpoint::PipeName) -> Result<PipeListenerType, TransportError> {
    endpoint::bind(name)
}

pub(crate) fn accept(
    listener: &PipeListenerType,
    parent: &ParentProcessObservation,
) -> Result<PipeStream, TransportError> {
    endpoint::accept(listener, parent)
}

pub(crate) fn read_frame(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
) -> Result<Vec<u8>, TransportError> {
    io::read_frame(parent, stream)
}

pub(crate) fn write_frame(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    body: &[u8],
) -> Result<(), TransportError> {
    io::write_frame(parent, stream, body)
}
