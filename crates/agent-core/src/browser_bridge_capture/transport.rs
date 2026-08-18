use std::{net::TcpStream, time::Duration};

use base64::Engine;
use ocentra_parent_agent_protocol::constants;
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpCaptureMode, ManagedBrowserCdpCaptureRequest, MANAGED_BROWSER_CDP_FIELD_DATA,
    MANAGED_BROWSER_CDP_FIELD_ID, MANAGED_BROWSER_CDP_FIELD_METHOD,
    MANAGED_BROWSER_CDP_FIELD_PARAMS, MANAGED_BROWSER_CDP_FIELD_RESULT,
    MANAGED_BROWSER_CDP_IMAGE_FORMAT, MANAGED_BROWSER_CDP_METHOD_CAPTURE_SCREENSHOT,
    MANAGED_BROWSER_CDP_PARAM_CAPTURE_BEYOND_VIEWPORT, MANAGED_BROWSER_CDP_PARAM_CLIP,
    MANAGED_BROWSER_CDP_PARAM_FORMAT, MANAGED_BROWSER_CDP_PARAM_FROM_SURFACE,
    MANAGED_BROWSER_CDP_PARAM_HEIGHT, MANAGED_BROWSER_CDP_PARAM_SCALE,
    MANAGED_BROWSER_CDP_PARAM_WIDTH, MANAGED_BROWSER_CDP_PARAM_X, MANAGED_BROWSER_CDP_PARAM_Y,
};
use serde_json::{json, Value};
use tungstenite::{
    client::{client as websocket_client, IntoClientRequest},
    Message,
};

use super::{ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority, CDP_MAX_IMAGE_BYTES};

pub(super) fn capture_screenshot(
    authority: &ManagedBrowserCdpTargetAuthority,
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<Vec<u8>, ManagedBrowserCdpCaptureError> {
    let mut websocket_request = authority
        .websocket_url()
        .into_client_request()
        .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)?;
    let stream = TcpStream::connect_timeout(
        &authority.endpoint(),
        Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS),
    )
    .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)?;
    configure_stream(&stream)?;
    let (mut socket, _) = websocket_client(websocket_request, stream)
        .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)?;
    let body = serde_json::to_string(&json!({
        (MANAGED_BROWSER_CDP_FIELD_ID): 1,
        (MANAGED_BROWSER_CDP_FIELD_METHOD): MANAGED_BROWSER_CDP_METHOD_CAPTURE_SCREENSHOT,
        (MANAGED_BROWSER_CDP_FIELD_PARAMS): screenshot_params(request),
    }))
    .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidResponse)?;
    socket
        .send(Message::Text(body))
        .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)?;

    loop {
        let message = socket
            .read()
            .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)?;
        let Message::Text(text) = message else {
            continue;
        };
        if text.len() > CDP_MAX_IMAGE_BYTES * 2 {
            return Err(ManagedBrowserCdpCaptureError::ResponseTooLarge);
        }
        let value: Value = serde_json::from_str(&text)
            .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidResponse)?;
        if value
            .get(MANAGED_BROWSER_CDP_FIELD_ID)
            .and_then(Value::as_u64)
            != Some(1)
        {
            continue;
        }
        let data = value
            .get(MANAGED_BROWSER_CDP_FIELD_RESULT)
            .and_then(|result| result.get(MANAGED_BROWSER_CDP_FIELD_DATA))
            .and_then(Value::as_str)
            .ok_or(ManagedBrowserCdpCaptureError::InvalidResponse)?;
        return base64::engine::general_purpose::STANDARD
            .decode(data)
            .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidImage);
    }
}

fn configure_stream(stream: &TcpStream) -> Result<(), ManagedBrowserCdpCaptureError> {
    let timeout = Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS);
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|_error| ManagedBrowserCdpCaptureError::Transport)
}

fn screenshot_params(request: &ManagedBrowserCdpCaptureRequest) -> Value {
    let mut params = json!({
        (MANAGED_BROWSER_CDP_PARAM_FORMAT): MANAGED_BROWSER_CDP_IMAGE_FORMAT,
        (MANAGED_BROWSER_CDP_PARAM_FROM_SURFACE): true,
        (MANAGED_BROWSER_CDP_PARAM_CAPTURE_BEYOND_VIEWPORT): matches!(request.mode, ManagedBrowserCdpCaptureMode::Page),
    });
    if let Some(crop) = request.crop.as_ref() {
        params[MANAGED_BROWSER_CDP_PARAM_CLIP] = json!({
                (MANAGED_BROWSER_CDP_PARAM_X): crop.x,
                (MANAGED_BROWSER_CDP_PARAM_Y): crop.y,
                (MANAGED_BROWSER_CDP_PARAM_WIDTH): crop.width,
                (MANAGED_BROWSER_CDP_PARAM_HEIGHT): crop.height,
                (MANAGED_BROWSER_CDP_PARAM_SCALE): 1,
        });
    }
    params
}
