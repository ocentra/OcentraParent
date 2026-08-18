use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

use ocentra_parent_agent_protocol::constants;
use serde_json::{json, Value};
use tungstenite::{
    client::{client as websocket_client, IntoClientRequest},
    Message,
};

use super::{ExtractionError, Payload};

const CDP_FIELD_ID: &str = "id";
const CDP_FIELD_METHOD: &str = "method";
const CDP_FIELD_PARAMS: &str = "params";
const CDP_METHOD_RUNTIME_EVALUATE: &str = "Runtime.evaluate";
const CDP_PARAM_EXPRESSION: &str = "expression";
const CDP_PARAM_RETURN_BY_VALUE: &str = "returnByValue";
const CDP_PARAM_AWAIT_PROMISE: &str = "awaitPromise";

const STRUCTURED_EXTRACTION_EXPRESSION: &str = include_str!("structured_extraction.js");

pub(super) fn extract(
    endpoint: SocketAddr,
    websocket_url: &str,
) -> Result<Payload, ExtractionError> {
    let websocket_request = websocket_url
        .into_client_request()
        .map_err(|_error| ExtractionError::Transport)?;
    let stream = TcpStream::connect_timeout(
        &endpoint,
        Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS),
    )
    .map_err(|_error| ExtractionError::Transport)?;
    configure_stream(&stream)?;
    let (mut socket, _) =
        websocket_client(websocket_request, stream).map_err(|_error| ExtractionError::Transport)?;
    let body = serde_json::to_string(&json!({
        (CDP_FIELD_ID): 1,
        (CDP_FIELD_METHOD): CDP_METHOD_RUNTIME_EVALUATE,
        (CDP_FIELD_PARAMS): {
            (CDP_PARAM_EXPRESSION): STRUCTURED_EXTRACTION_EXPRESSION,
            (CDP_PARAM_RETURN_BY_VALUE): true,
            (CDP_PARAM_AWAIT_PROMISE): false,
        },
    }))
    .map_err(|_error| ExtractionError::InvalidResponse)?;
    socket
        .send(Message::Text(body))
        .map_err(|_error| ExtractionError::Transport)?;

    loop {
        let message = socket.read().map_err(|_error| ExtractionError::Transport)?;
        let Message::Text(text) = message else {
            continue;
        };
        if text.len() > constants::browser::DEVTOOLS_MAX_RESPONSE_BYTES {
            return Err(ExtractionError::ResponseTooLarge);
        }
        let value: Value =
            serde_json::from_str(&text).map_err(|_error| ExtractionError::InvalidResponse)?;
        if value.get(CDP_FIELD_ID).and_then(Value::as_u64) != Some(1) {
            continue;
        }
        return super::parser::parse_payload(&value);
    }
}

fn configure_stream(stream: &TcpStream) -> Result<(), ExtractionError> {
    let timeout = Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS);
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|_error| ExtractionError::Transport)?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|_error| ExtractionError::Transport)
}
