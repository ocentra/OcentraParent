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

use super::{ExtractionError, Outcome, Payload};
use crate::browser_bridge_capture::ManagedBrowserCdpCaptureError;

const CDP_FIELD_ID: &str = "id";
const CDP_FIELD_METHOD: &str = "method";
const CDP_FIELD_PARAMS: &str = "params";
const CDP_METHOD_RUNTIME_EVALUATE: &str = "Runtime.evaluate";
const CDP_PARAM_EXPRESSION: &str = "expression";
const CDP_PARAM_RETURN_BY_VALUE: &str = "returnByValue";
const CDP_PARAM_AWAIT_PROMISE: &str = "awaitPromise";

const STRUCTURED_EXTRACTION_EXPRESSION: &str = r#"(() => {
  const limit = 480;
  const bodyText = typeof document.body?.innerText === 'string'
    ? document.body.innerText
    : '';
  const probe = bodyText.slice(0, 4096);
  const credentialSelector = [
    'input[type="password"]',
    'input[autocomplete="current-password"]',
    'input[autocomplete="new-password"]',
    'input[autocomplete="one-time-code"]',
    'input[autocomplete="cc-number"]',
    'input[autocomplete="cc-csc"]'
  ].join(',');
  const protectedSelector = [
    '[data-ocentra-protected="true"]',
    '[data-sensitive-content="true"]',
    '[aria-label*="password" i]',
    '[aria-label*="security code" i]',
    '[aria-label*="credit card" i]'
  ].join(',');
  const credentialRisk = Boolean(document.querySelector(credentialSelector))
    || /password|passcode|verification code|security code|credit card|cvv|ssn/i.test(probe);
  const protectedSurface = Boolean(document.querySelector(protectedSelector));
  const protectedContentSkipped = credentialRisk || protectedSurface;
  const metaValues = Array.from(document.querySelectorAll(
    'meta[name="description"], meta[property="og:title"], meta[property="og:description"]'
  )).slice(0, 6).map((node) => node.getAttribute('content') || '')
    .filter((value) => value.length > 0).join(' ').slice(0, limit);
  const accessibilityValues = Array.from(document.querySelectorAll(
    '[aria-label], [role]'
  )).slice(0, 32).map((node) => `${node.getAttribute('role') || ''}:${node.getAttribute('aria-label') || ''}`)
    .filter((value) => value !== ':').join(' ').slice(0, limit);
  const privateContentRedacted = protectedContentSkipped;
  const safeText = privateContentRedacted ? '' : bodyText;
  const visibleText = safeText.slice(0, limit);
  const visibleTextCharacterCount = safeText.length;
  const domOverflowRedacted = visibleTextCharacterCount > limit;
  return {
    visibleText,
    visibleTextCharacterCount,
    domOverflowRedacted,
    privateContentRedacted,
    protectedContentSkipped,
    metaValues,
    accessibilityValues,
  };
})()"#;

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

pub(super) fn ensure_capture_safe(
    endpoint: SocketAddr,
    websocket_url: &str,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    let payload = extract(endpoint, websocket_url)
        .map_err(|_error| ManagedBrowserCdpCaptureError::StructuredExtractionUnavailable)?;
    match payload.outcome {
        Outcome::ProtectedContentSkipped => {
            Err(ManagedBrowserCdpCaptureError::ProtectedSurfaceRejected)
        }
        Outcome::Unavailable => Err(ManagedBrowserCdpCaptureError::StructuredExtractionUnavailable),
        Outcome::PolicySufficient | Outcome::NeedsScreenshot => Ok(()),
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
