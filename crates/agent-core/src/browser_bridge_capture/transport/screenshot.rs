use base64::Engine;
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpCaptureMode, ManagedBrowserCdpCaptureRequest, MANAGED_BROWSER_CDP_FIELD_DATA,
    MANAGED_BROWSER_CDP_IMAGE_FORMAT, MANAGED_BROWSER_CDP_METHOD_CAPTURE_SCREENSHOT,
    MANAGED_BROWSER_CDP_PARAM_CAPTURE_BEYOND_VIEWPORT, MANAGED_BROWSER_CDP_PARAM_CLIP,
    MANAGED_BROWSER_CDP_PARAM_FORMAT, MANAGED_BROWSER_CDP_PARAM_FROM_SURFACE,
    MANAGED_BROWSER_CDP_PARAM_HEIGHT, MANAGED_BROWSER_CDP_PARAM_SCALE,
    MANAGED_BROWSER_CDP_PARAM_WIDTH, MANAGED_BROWSER_CDP_PARAM_X, MANAGED_BROWSER_CDP_PARAM_Y,
};
use serde_json::{json, Value};

use super::{CdpSession, CdpTransportError};
use crate::browser_bridge_capture::ManagedBrowserCdpCaptureError;

const CDP_MAX_IMAGE_BYTES: usize = 32 * 1024 * 1024;

pub(super) fn capture(
    session: &mut CdpSession,
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<Vec<u8>, CdpTransportError> {
    let value = session.call(
        MANAGED_BROWSER_CDP_METHOD_CAPTURE_SCREENSHOT,
        screenshot_params(request),
    )?;
    let data = value
        .get(MANAGED_BROWSER_CDP_FIELD_DATA)
        .and_then(Value::as_str)
        .ok_or(CdpTransportError::InvalidResponse)?;
    if data.len() > ((CDP_MAX_IMAGE_BYTES + 2) / 3) * 4 {
        return Err(CdpTransportError::ResponseTooLarge);
    }
    base64::engine::general_purpose::STANDARD
        .decode(data)
        .map_err(|_error| CdpTransportError::InvalidResponse)
}

pub(super) fn capture_error(error: CdpTransportError) -> ManagedBrowserCdpCaptureError {
    match error {
        CdpTransportError::Transport => ManagedBrowserCdpCaptureError::Transport,
        CdpTransportError::ResponseTooLarge => ManagedBrowserCdpCaptureError::ResponseTooLarge,
        CdpTransportError::InvalidResponse | CdpTransportError::Protocol => {
            ManagedBrowserCdpCaptureError::InvalidResponse
        }
    }
}

fn screenshot_params(request: &ManagedBrowserCdpCaptureRequest) -> Value {
    let mut params = json!({
        (MANAGED_BROWSER_CDP_PARAM_FORMAT): MANAGED_BROWSER_CDP_IMAGE_FORMAT,
        (MANAGED_BROWSER_CDP_PARAM_FROM_SURFACE): true,
        (MANAGED_BROWSER_CDP_PARAM_CAPTURE_BEYOND_VIEWPORT): matches!(request.mode, ManagedBrowserCdpCaptureMode::Page),
    });
    let clip = match request.mode {
        ManagedBrowserCdpCaptureMode::Page => None,
        ManagedBrowserCdpCaptureMode::Viewport => Some((
            0,
            0,
            request.viewport_width.unwrap_or_default(),
            request.viewport_height.unwrap_or_default(),
        )),
        ManagedBrowserCdpCaptureMode::Crop => request
            .crop
            .as_ref()
            .map(|crop| (crop.x, crop.y, crop.width, crop.height)),
    };
    if let Some((x, y, width, height)) = clip {
        params[MANAGED_BROWSER_CDP_PARAM_CLIP] = json!({
            (MANAGED_BROWSER_CDP_PARAM_X): x,
            (MANAGED_BROWSER_CDP_PARAM_Y): y,
            (MANAGED_BROWSER_CDP_PARAM_WIDTH): width,
            (MANAGED_BROWSER_CDP_PARAM_HEIGHT): height,
            (MANAGED_BROWSER_CDP_PARAM_SCALE): 1,
        });
    }
    params
}
