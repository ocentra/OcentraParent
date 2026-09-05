use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;
use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;

use super::{response, ConnectionDisposition, RuntimeError};
use crate::operations::LeaseState;
use crate::protocol::ReadyFrame;
use crate::transport::{self, PipeStream};

pub(super) fn serve_connection(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    ready: &ReadyFrame,
) -> Result<ConnectionDisposition, RuntimeError> {
    if let Err(error) = response::send_json(parent, stream, ready) {
        return super::connection_error(error);
    }

    loop {
        let body = match transport::read_frame(parent, stream) {
            Ok(body) => body,
            Err(error) => return super::connection_transport_error(error),
        };
        let request = match response::parse_request(&body) {
            Ok(request) => request,
            Err(()) => return Ok(ConnectionDisposition::Close),
        };
        match response::process_request(parent, stream, session, lease, &request) {
            Ok(None) => {}
            Ok(Some(disposition)) => return Ok(disposition),
            Err(error) => return Err(error),
        }
    }
}
