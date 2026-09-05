use std::{
    net::TcpStream,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_schema::managed_browser_cdp_capture::{
    MANAGED_BROWSER_CDP_FIELD_ID, MANAGED_BROWSER_CDP_FIELD_METHOD,
    MANAGED_BROWSER_CDP_FIELD_PARAMS, MANAGED_BROWSER_CDP_FIELD_RESULT,
};
use serde_json::{json, Value};
use tungstenite::{
    client::{client as websocket_client, IntoClientRequest},
    Message, WebSocket,
};

use super::{authority::LaunchBinding, ManagedBrowserCdpCaptureError};

#[path = "transport/screenshot.rs"]
mod screenshot;

const CDP_MAX_IMAGE_BYTES: usize = 32 * 1024 * 1024;
const CDP_MAX_RESPONSE_BYTES: usize = CDP_MAX_IMAGE_BYTES * 2;
const CDP_METHOD_SET_WEB_LIFECYCLE_STATE: &str = "Page.setWebLifecycleState";
const CDP_STATE_FROZEN: &str = "frozen";
const CDP_STATE_ACTIVE: &str = "active";

pub(super) struct CdpSession {
    socket: WebSocket<TcpStream>,
    next_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CdpTransportError {
    Transport,
    ResponseTooLarge,
    InvalidResponse,
    Protocol,
}

pub(super) struct FrozenPageGuard {
    armed: bool,
    retirement_binding: Option<LaunchBinding>,
    capability_revoked: Arc<AtomicBool>,
}

impl CdpSession {
    pub(super) fn connect(
        endpoint: std::net::SocketAddr,
        websocket_url: &str,
    ) -> Result<Self, CdpTransportError> {
        let websocket_request = websocket_url
            .into_client_request()
            .map_err(|_error| CdpTransportError::InvalidResponse)?;
        let stream = TcpStream::connect_timeout(
            &endpoint,
            Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS),
        )
        .map_err(|_error| CdpTransportError::Transport)?;
        configure_stream(&stream)?;
        let (socket, _) = websocket_client(websocket_request, stream)
            .map_err(|_error| CdpTransportError::Transport)?;
        Ok(Self { socket, next_id: 1 })
    }

    pub(super) fn call(
        &mut self,
        method: &str,
        params: &Value,
    ) -> Result<Value, CdpTransportError> {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        let body = serde_json::to_string(&json!({
            (MANAGED_BROWSER_CDP_FIELD_ID): id,
            (MANAGED_BROWSER_CDP_FIELD_METHOD): method,
            (MANAGED_BROWSER_CDP_FIELD_PARAMS): params,
        }))
        .map_err(|_error| CdpTransportError::InvalidResponse)?;
        self.socket
            .send(Message::Text(body))
            .map_err(|_error| CdpTransportError::Transport)?;

        loop {
            let message = self
                .socket
                .read()
                .map_err(|_error| CdpTransportError::Transport)?;
            let Message::Text(text) = message else {
                continue;
            };
            if text.len() > CDP_MAX_RESPONSE_BYTES {
                return Err(CdpTransportError::ResponseTooLarge);
            }
            let value: Value =
                serde_json::from_str(&text).map_err(|_error| CdpTransportError::InvalidResponse)?;
            if value
                .get(MANAGED_BROWSER_CDP_FIELD_ID)
                .and_then(Value::as_u64)
                != Some(id)
            {
                continue;
            }
            if value.get("error").is_some() {
                return Err(CdpTransportError::Protocol);
            }
            return value
                .get(MANAGED_BROWSER_CDP_FIELD_RESULT)
                .cloned()
                .ok_or(CdpTransportError::InvalidResponse);
        }
    }

    pub(super) fn freeze(
        &mut self,
        binding: &LaunchBinding,
        capability_revoked: Arc<AtomicBool>,
    ) -> Result<FrozenPageGuard, CdpTransportError> {
        self.call("Page.enable", &json!({}))?;
        self.call(
            CDP_METHOD_SET_WEB_LIFECYCLE_STATE,
            &json!({ "state": CDP_STATE_FROZEN }),
        )?;
        Ok(FrozenPageGuard {
            armed: true,
            retirement_binding: Some(binding.clone()),
            capability_revoked,
        })
    }
}

impl FrozenPageGuard {
    pub(super) fn release(&mut self, session: &mut CdpSession) -> Result<(), CdpTransportError> {
        if !self.armed {
            return Ok(());
        }
        session.call(
            CDP_METHOD_SET_WEB_LIFECYCLE_STATE,
            &json!({ "state": CDP_STATE_ACTIVE }),
        )?;
        self.armed = false;
        self.retirement_binding = None;
        Ok(())
    }
}

impl Drop for FrozenPageGuard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        if let Some(binding) = self.retirement_binding.take() {
            let _retired = super::process::retire(&binding);
        }
        self.capability_revoked.store(true, Ordering::Release);
        self.armed = false;
    }
}

pub(super) fn capture_screenshot(
    session: &mut CdpSession,
    request: &ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpCaptureRequest,
) -> Result<Vec<u8>, CdpTransportError> {
    screenshot::capture(session, request)
}

pub(super) fn capture_error(error: CdpTransportError) -> ManagedBrowserCdpCaptureError {
    screenshot::capture_error(error)
}

fn configure_stream(stream: &TcpStream) -> Result<(), CdpTransportError> {
    let timeout = Duration::from_millis(constants::browser::DEVTOOLS_TIMEOUT_MS);
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|_error| CdpTransportError::Transport)?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|_error| CdpTransportError::Transport)
}
