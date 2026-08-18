use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use super::{EvaluatedPayload, ExtractionError};
use crate::browser_bridge_capture::{
    target::DocumentIdentity,
    transport::{CdpSession, CdpTransportError},
};

const CDP_METHOD_CREATE_ISOLATED_WORLD: &str = "Page.createIsolatedWorld";
const CDP_METHOD_GET_FRAME_TREE: &str = "Page.getFrameTree";
const CDP_METHOD_RUNTIME_EVALUATE: &str = "Runtime.evaluate";
const CDP_FIELD_EXECUTION_CONTEXT_ID: &str = "executionContextId";
const CDP_FIELD_FRAME: &str = "frame";
const CDP_FIELD_FRAME_ID: &str = "id";
const CDP_FIELD_FRAME_TREE: &str = "frameTree";
const CDP_FIELD_LOADER_ID: &str = "loaderId";
const CDP_FIELD_URL: &str = "url";
const CDP_PARAM_AWAIT_PROMISE: &str = "awaitPromise";
const CDP_PARAM_CONTEXT_ID: &str = "contextId";
const CDP_PARAM_EXPRESSION: &str = "expression";
const CDP_PARAM_FRAME_ID: &str = "frameId";
const CDP_PARAM_GRANT_UNIVERSAL_ACCESS: &str = "grantUniveralAccess";
const CDP_PARAM_RETURN_BY_VALUE: &str = "returnByValue";
const CDP_PARAM_WORLD_NAME: &str = "worldName";
const ISOLATED_WORLD_NAME: &str = "ocentra-managed-browser-safety-v1";

const STRUCTURED_EXTRACTION_EXPRESSION: &str = include_str!("structured_extraction.js");

pub(super) fn extract(
    endpoint: std::net::SocketAddr,
    websocket_url: &str,
) -> Result<EvaluatedPayload, ExtractionError> {
    let mut session = CdpSession::connect(endpoint, websocket_url).map_err(map_transport_error)?;
    extract_on_session(&mut session)
}

pub(super) fn extract_on_session(
    session: &mut CdpSession,
) -> Result<EvaluatedPayload, ExtractionError> {
    let before = frame_identity(session)?;
    let context_id = create_isolated_world(session, &before.frame_id)?;
    let result = session
        .call(
            CDP_METHOD_RUNTIME_EVALUATE,
            json!({
                (CDP_PARAM_EXPRESSION): STRUCTURED_EXTRACTION_EXPRESSION,
                (CDP_PARAM_RETURN_BY_VALUE): true,
                (CDP_PARAM_AWAIT_PROMISE): true,
                (CDP_PARAM_CONTEXT_ID): context_id,
            }),
        )
        .map_err(map_transport_error)?;
    let payload = super::parser::parse_payload(&result)?;
    if payload.document_url_digest != before.url_digest {
        return Err(ExtractionError::DocumentChanged);
    }
    let after = frame_identity(session)?;
    if before != after {
        return Err(ExtractionError::DocumentChanged);
    }
    Ok(EvaluatedPayload {
        payload,
        document_identity: before,
    })
}

fn create_isolated_world(session: &mut CdpSession, frame_id: &str) -> Result<u64, ExtractionError> {
    session
        .call(
            CDP_METHOD_CREATE_ISOLATED_WORLD,
            json!({
                (CDP_PARAM_FRAME_ID): frame_id,
                (CDP_PARAM_WORLD_NAME): ISOLATED_WORLD_NAME,
                (CDP_PARAM_GRANT_UNIVERSAL_ACCESS): false,
            }),
        )
        .map_err(map_transport_error)?
        .get(CDP_FIELD_EXECUTION_CONTEXT_ID)
        .and_then(Value::as_u64)
        .ok_or(ExtractionError::InvalidResponse)
}

fn frame_identity(session: &mut CdpSession) -> Result<DocumentIdentity, ExtractionError> {
    let result = session
        .call(CDP_METHOD_GET_FRAME_TREE, json!({}))
        .map_err(map_transport_error)?;
    let frame = result
        .get(CDP_FIELD_FRAME_TREE)
        .and_then(|tree| tree.get(CDP_FIELD_FRAME))
        .ok_or(ExtractionError::InvalidResponse)?;
    let frame_id = frame
        .get(CDP_FIELD_FRAME_ID)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or(ExtractionError::InvalidResponse)?;
    let loader_id = frame
        .get(CDP_FIELD_LOADER_ID)
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .ok_or(ExtractionError::InvalidResponse)?;
    let url = frame
        .get(CDP_FIELD_URL)
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
        .ok_or(ExtractionError::InvalidResponse)?;
    Ok(DocumentIdentity {
        frame_id: frame_id.to_owned(),
        loader_id: loader_id.to_owned(),
        url_digest: text_digest(url),
    })
}

fn map_transport_error(error: CdpTransportError) -> ExtractionError {
    match error {
        CdpTransportError::Transport => ExtractionError::Transport,
        CdpTransportError::ResponseTooLarge => ExtractionError::ResponseTooLarge,
        CdpTransportError::InvalidResponse | CdpTransportError::Protocol => {
            ExtractionError::InvalidResponse
        }
    }
}

fn text_digest(value: &str) -> String {
    let mut digest = String::new();
    for byte in Sha256::digest(value.as_bytes()) {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}
