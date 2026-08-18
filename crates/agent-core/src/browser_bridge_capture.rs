//! Browser-owned authority and bounded CDP screenshot transport.
//!
//! The screen adapter receives an authority produced here. It cannot provide a
//! debugger endpoint or arbitrary target to the transport, and the authority
//! is created only after a fresh, custody-validated managed target inventory.

use std::net::SocketAddr;

use ocentra_parent_agent_protocol::constants;
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpCaptureMode, ManagedBrowserCdpCaptureReceipt, ManagedBrowserCdpCaptureRequest,
    ManagedBrowserCdpEvidenceRefs, MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION,
    MANAGED_BROWSER_CDP_TARGET_REF_PREFIX, MANAGED_BROWSER_CDP_TITLE_REF_PREFIX,
    MANAGED_BROWSER_CDP_URL_REF_PREFIX,
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::{
    browser_bridge_http::read_devtools_body,
    browser_bridge_poll::{
        validate_bridge_custody, BrowserBridgePollConfig, BrowserBridgePollError,
    },
};

const CDP_MAX_IMAGE_BYTES: usize = 32 * 1024 * 1024;

#[path = "browser_bridge_capture/transport.rs"]
mod transport;

#[derive(Clone, Debug)]
pub struct ManagedBrowserCdpTargetAuthority {
    endpoint: SocketAddr,
    target_id: String,
    websocket_url: String,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManagedBrowserCdpCaptureBytes {
    png_bytes: Vec<u8>,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManagedBrowserCdpCaptureError {
    Bridge(BrowserBridgePollError),
    TargetNotFound,
    TargetNotPage,
    TargetNotObservable,
    TargetAuthorityMismatch,
    InvalidWebSocketEndpoint,
    RequestRejected,
    Transport,
    ResponseTooLarge,
    InvalidResponse,
    InvalidImage,
}

impl From<BrowserBridgePollError> for ManagedBrowserCdpCaptureError {
    fn from(error: BrowserBridgePollError) -> Self {
        Self::Bridge(error)
    }
}

pub fn authorize_managed_browser_cdp_target(
    config: &BrowserBridgePollConfig,
    target_id: &str,
    observed_at: &str,
) -> Result<ManagedBrowserCdpTargetAuthority, ManagedBrowserCdpCaptureError> {
    if !config.endpoint.ip().is_loopback() {
        return Err(BrowserBridgePollError::NonLoopbackEndpoint.into());
    }
    validate_bridge_custody(config, observed_at)?;
    let body = read_devtools_body(&config.endpoint, constants::browser::HTTP_GET_JSON_LIST)?;
    let target = target_from_list(&body, target_id)?;
    let websocket_url = target
        .get(constants::browser::DEVTOOLS_FIELD_WEBSOCKET_DEBUGGER_URL)
        .and_then(Value::as_str)
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    validate_websocket_endpoint(websocket_url, config.endpoint)?;

    let evidence_refs = ManagedBrowserCdpEvidenceRefs {
        target_ref: opaque_ref(MANAGED_BROWSER_CDP_TARGET_REF_PREFIX, config, target_id),
        url_ref: opaque_ref(MANAGED_BROWSER_CDP_URL_REF_PREFIX, config, target_id),
        title_ref: opaque_ref(MANAGED_BROWSER_CDP_TITLE_REF_PREFIX, config, target_id),
    };

    Ok(ManagedBrowserCdpTargetAuthority {
        endpoint: config.endpoint,
        target_id: target_id.to_owned(),
        websocket_url: websocket_url.to_owned(),
        evidence_refs,
    })
}

impl ManagedBrowserCdpTargetAuthority {
    pub fn capture(
        &self,
        request: &ManagedBrowserCdpCaptureRequest,
    ) -> Result<ManagedBrowserCdpCaptureBytes, ManagedBrowserCdpCaptureError> {
        request
            .validate()
            .map_err(|_error| ManagedBrowserCdpCaptureError::RequestRejected)?;
        if request.target_id != self.target_id {
            return Err(ManagedBrowserCdpCaptureError::TargetAuthorityMismatch);
        }
        let png_bytes = transport::capture_screenshot(self, request)?;
        if png_bytes.is_empty() || png_bytes.len() > CDP_MAX_IMAGE_BYTES {
            return Err(ManagedBrowserCdpCaptureError::ResponseTooLarge);
        }
        if png_bytes.get(0..8) != Some(&[137, 80, 78, 71, 13, 10, 26, 10]) {
            return Err(ManagedBrowserCdpCaptureError::InvalidImage);
        }
        Ok(ManagedBrowserCdpCaptureBytes {
            png_bytes,
            evidence_refs: self.evidence_refs.clone(),
        })
    }

    pub fn evidence_refs(&self) -> &ManagedBrowserCdpEvidenceRefs {
        &self.evidence_refs
    }

    pub fn png_bytes(&self) -> &[u8] {
        &self.png_bytes
    }

    pub(crate) fn endpoint(&self) -> SocketAddr {
        self.endpoint
    }

    pub(crate) fn websocket_url(&self) -> &str {
        &self.websocket_url
    }
}

pub fn capture_receipt(
    capture_ref: String,
    capture: &ManagedBrowserCdpCaptureBytes,
    capture_mode: ManagedBrowserCdpCaptureMode,
    width: u32,
    height: u32,
    image_digest: String,
) -> ManagedBrowserCdpCaptureReceipt {
    ManagedBrowserCdpCaptureReceipt {
        schema_version: MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION.to_owned(),
        capture_ref,
        target_ref: capture.evidence_refs.target_ref.clone(),
        evidence_refs: capture.evidence_refs.clone(),
        capture_mode,
        width,
        height,
        image_digest,
        image_byte_size: capture.png_bytes.len() as u64,
        image_format: ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_IMAGE_FORMAT
            .to_owned(),
        custody_state: ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_TEMP_QUEUE
            .to_owned(),
        raw_image_retained: false,
    }
}

fn target_from_list(body: &str, target_id: &str) -> Result<Value, ManagedBrowserCdpCaptureError> {
    let value: Value = serde_json::from_str(body)
        .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidResponse)?;
    let targets = value
        .as_array()
        .ok_or(ManagedBrowserCdpCaptureError::InvalidResponse)?;
    let target = targets
        .iter()
        .find(|target| {
            target
                .get(constants::browser::DEVTOOLS_FIELD_ID)
                .and_then(Value::as_str)
                == Some(target_id)
        })
        .ok_or(ManagedBrowserCdpCaptureError::TargetNotFound)?
        .clone();
    if target
        .get(constants::browser::DEVTOOLS_FIELD_TYPE)
        .and_then(Value::as_str)
        != Some(constants::browser::DEVTOOLS_TARGET_TYPE_PAGE)
    {
        return Err(ManagedBrowserCdpCaptureError::TargetNotPage);
    }
    let url = target
        .get(constants::browser::DEVTOOLS_FIELD_URL)
        .and_then(Value::as_str)
        .ok_or(ManagedBrowserCdpCaptureError::TargetNotObservable)?;
    if url.is_empty()
        || url == constants::browser::CHROMIUM_DEFAULT_URL
        || url.starts_with(constants::browser::CHROMIUM_INTERNAL_CHROME_PREFIX)
        || url.starts_with(constants::browser::CHROMIUM_INTERNAL_DEVTOOLS_PREFIX)
        || url.starts_with(constants::browser::CHROMIUM_INTERNAL_EDGE_PREFIX)
    {
        return Err(ManagedBrowserCdpCaptureError::TargetNotObservable);
    }
    Ok(target)
}

fn validate_websocket_endpoint(
    url: &str,
    endpoint: SocketAddr,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    let remainder = url
        .strip_prefix("ws://")
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    let (authority, path) = remainder
        .split_once('/')
        .ok_or(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    if path.is_empty() {
        return Err(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint);
    }
    let websocket_endpoint: SocketAddr = authority
        .parse()
        .map_err(|_error| ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint)?;
    if websocket_endpoint != endpoint || !websocket_endpoint.ip().is_loopback() {
        return Err(ManagedBrowserCdpCaptureError::InvalidWebSocketEndpoint);
    }
    Ok(())
}

fn opaque_ref(prefix: &str, config: &BrowserBridgePollConfig, target_id: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(config.managed_browser_session_id.as_bytes());
    digest.update([0]);
    digest.update(config.profile_id.as_bytes());
    digest.update([0]);
    digest.update(target_id.as_bytes());
    let digest = digest.finalize();
    let mut reference = String::from(prefix);
    reference.push('-');
    for byte in digest {
        reference.push_str(&format!("{byte:02x}"));
    }
    reference
}
