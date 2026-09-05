#[path = "connection_loop.rs"]
mod loop_driver;
#[path = "connection_response.rs"]
mod response;

use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;
use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;

use super::{ConnectionDisposition, RuntimeError};
use crate::operations::{LeaseState, ProviderError};
use crate::protocol::ReadyFrame;
use crate::transport::{PipeStream, TransportError};

pub(super) fn serve_connection(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    ready: &ReadyFrame,
) -> Result<ConnectionDisposition, RuntimeError> {
    loop_driver::serve_connection(parent, stream, session, lease, ready)
}

pub(super) fn terminal_provider_error(
    send_result: Result<(), RuntimeError>,
    error: ProviderError,
) -> Result<ConnectionDisposition, RuntimeError> {
    match send_result {
        Ok(()) => Err(RuntimeError::Provider(error)),
        Err(send_error) => match send_error {
            other @ RuntimeError::Transport(TransportError::Parent) => Err(other),
            RuntimeError::Transport(_) => Err(RuntimeError::Provider(error)),
            other => Err(other),
        },
    }
}

pub(super) fn connection_error(error: RuntimeError) -> Result<ConnectionDisposition, RuntimeError> {
    if matches!(&error, RuntimeError::Transport(TransportError::Parent)) {
        Err(error)
    } else {
        match error {
            RuntimeError::Transport(_) => Ok(ConnectionDisposition::Close),
            other => Err(other),
        }
    }
}

pub(super) fn connection_transport_error(
    error: TransportError,
) -> Result<ConnectionDisposition, RuntimeError> {
    connection_error(RuntimeError::Transport(error))
}
