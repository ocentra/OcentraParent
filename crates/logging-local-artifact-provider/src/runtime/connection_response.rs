use ocentra_logging_local_artifact_windows_ffi::transport::ParentProcessObservation;
use ocentra_parent_logging_core::local_artifact_mutation::LocalArtifactMutationSession;
use serde::{Deserialize, Serialize};

use super::{ConnectionDisposition, RuntimeError};
use crate::operations::{self, FailureDisposition, LeaseState};
use crate::protocol::text::ErrorText;
use crate::protocol::{self, ErrorBody, Request, Response};
use crate::transport::{self, PipeStream};

pub(super) fn parse_request(body: &[u8]) -> Result<Request, ()> {
    let mut deserializer = serde_json::Deserializer::from_slice(body);
    let request = Request::deserialize(&mut deserializer).map_err(|_error| ())?;
    deserializer.end().map_err(|_error| ())?;
    Ok(request)
}

pub(super) fn failed_response(
    request: &Request,
    operation: protocol::OperationName,
    error: ErrorText,
) -> Response {
    // CLONE-JUSTIFICATION: failed responses must echo the raw boundary DTO
    // even when conversion into validated request types failed.
    Response {
        protocol_version: protocol::PROTOCOL_VERSION,
        request_id: request.request_id.clone(),
        operation: operation.into(),
        nonce: request.nonce.clone(),
        ok: false,
        result: None,
        error: Some(ErrorBody {
            code: error.code(),
            message: error.message(),
        }),
    }
}

pub(super) fn process_request(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &mut LeaseState,
    request: &Request,
) -> Result<Option<ConnectionDisposition>, RuntimeError> {
    let operation_name = request.operation.name();
    let validated = match request.validate_frame() {
        Ok(request) => request,
        Err(error) => {
            if !request.can_echo() {
                return Ok(Some(ConnectionDisposition::Close));
            }
            let response = failed_response(request, operation_name, error.response_failure());
            if let Err(send_error) = send_json(parent, stream, &response) {
                return super::connection_error(send_error).map(Some);
            }
            return Ok(None);
        }
    };

    let execution = match operations::execute(session, lease, &validated) {
        Ok(execution) => execution,
        Err(error) => {
            let disposition = error.disposition();
            let response = failed_response(request, operation_name, error.text());
            let send_result = send_json(parent, stream, &response);
            if disposition == FailureDisposition::Terminate {
                return super::terminal_provider_error(send_result, error).map(Some);
            }
            if let Err(send_error) = send_result {
                return super::connection_error(send_error).map(Some);
            }
            return Ok(None);
        }
    };

    let disposition = execution.disposition();
    let response = Response {
        protocol_version: protocol::PROTOCOL_VERSION,
        request_id: validated.request_id().text(),
        operation: operation_name.into(),
        nonce: validated.nonce().text(),
        ok: true,
        result: Some(execution.into_result()),
        error: None,
    };
    if disposition == operations::ExecutionDisposition::Shutdown {
        let body = encode_json(&response)?;
        return Ok(Some(ConnectionDisposition::Shutdown(body)));
    }
    if let Err(error) = send_json(parent, stream, &response) {
        return super::connection_error(error).map(Some);
    }
    Ok(None)
}

fn encode_json<T: Serialize>(value: &T) -> Result<Vec<u8>, RuntimeError> {
    serde_json::to_vec(value).map_err(|_error| RuntimeError::Protocol)
}

pub(super) fn send_json<T: Serialize>(
    parent: &ParentProcessObservation,
    stream: &mut PipeStream,
    value: &T,
) -> Result<(), RuntimeError> {
    let body = encode_json(value)?;
    transport::write_frame(parent, stream, &body).map_err(RuntimeError::Transport)
}
