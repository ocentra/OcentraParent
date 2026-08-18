//! Rust-owned contract for bounded screenshots from a managed browser target.
//!
//! This contract intentionally carries only capture intent and opaque evidence
//! references. Browser URLs, titles, debugger endpoints, OCR, and image bytes
//! remain inside their owning runtime boundaries.

use serde::{Deserialize, Serialize};

mod validation;

pub const MANAGED_BROWSER_CDP_CAPTURE_SCHEMA_VERSION: &str = "managed-browser-cdp-capture-v1";
pub const MANAGED_BROWSER_CDP_CAPTURE_REASON: &str = "managedBrowserCdpScreenshot";
pub const MANAGED_BROWSER_CDP_SOURCE_ID: &str = "managed-browser-cdp";
pub const MANAGED_BROWSER_CDP_ADAPTER_ID: &str = "screen-managed-browser-cdp-adapter";
pub const MANAGED_BROWSER_CDP_MAX_DIMENSION: u32 = 4096;
pub const MANAGED_BROWSER_CDP_MAX_PIXELS: u64 = 4_000_000;
pub const MANAGED_BROWSER_CDP_AUTHORITY_TTL_MS: u64 = 30_000;
pub const MANAGED_BROWSER_CDP_MAX_QUEUE_TTL_SECONDS: i64 = 300;
pub const MANAGED_BROWSER_CDP_IMAGE_FORMAT: &str = "png";
pub const MANAGED_BROWSER_CDP_ENCRYPTED_IMAGE_REF_PREFIX: &str = "screen-evidence:";
pub const MANAGED_BROWSER_CDP_METHOD_CAPTURE_SCREENSHOT: &str = "Page.captureScreenshot";
pub const MANAGED_BROWSER_CDP_TARGET_REF_PREFIX: &str = "browser-target";
pub const MANAGED_BROWSER_CDP_URL_REF_PREFIX: &str = "browser-url";
pub const MANAGED_BROWSER_CDP_TITLE_REF_PREFIX: &str = "browser-title";
pub const MANAGED_BROWSER_CDP_CAPTURE_REF_PREFIX: &str = "browser-capture-";
pub const MANAGED_BROWSER_CDP_STRUCTURED_BODY_DIGEST_PREFIX: &str =
    "managed-browser-body-sha256-v1-";
pub const MANAGED_BROWSER_CDP_FIELD_ID: &str = "id";
pub const MANAGED_BROWSER_CDP_FIELD_METHOD: &str = "method";
pub const MANAGED_BROWSER_CDP_FIELD_PARAMS: &str = "params";
pub const MANAGED_BROWSER_CDP_FIELD_RESULT: &str = "result";
pub const MANAGED_BROWSER_CDP_FIELD_DATA: &str = "data";
pub const MANAGED_BROWSER_CDP_PARAM_FORMAT: &str = "format";
pub const MANAGED_BROWSER_CDP_PARAM_FROM_SURFACE: &str = "fromSurface";
pub const MANAGED_BROWSER_CDP_PARAM_CAPTURE_BEYOND_VIEWPORT: &str = "captureBeyondViewport";
pub const MANAGED_BROWSER_CDP_PARAM_CLIP: &str = "clip";
pub const MANAGED_BROWSER_CDP_PARAM_X: &str = "x";
pub const MANAGED_BROWSER_CDP_PARAM_Y: &str = "y";
pub const MANAGED_BROWSER_CDP_PARAM_WIDTH: &str = "width";
pub const MANAGED_BROWSER_CDP_PARAM_HEIGHT: &str = "height";
pub const MANAGED_BROWSER_CDP_PARAM_SCALE: &str = "scale";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ManagedBrowserCdpCaptureMode {
    Page,
    Viewport,
    Crop,
}

impl ManagedBrowserCdpCaptureMode {
    pub fn as_protocol_str(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::Viewport => "viewport",
            Self::Crop => "crop",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedBrowserCdpCrop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedBrowserCdpCaptureRequest {
    pub schema_version: String,
    pub target_id: String,
    pub mode: ManagedBrowserCdpCaptureMode,
    pub viewport_width: Option<u32>,
    pub viewport_height: Option<u32>,
    pub crop: Option<ManagedBrowserCdpCrop>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManagedBrowserCdpCaptureRequestError {
    UnsupportedSchemaVersion,
    EmptyTargetId,
    TargetIdTooLong,
    DimensionsRequired,
    DimensionsOutOfBounds,
    CropRequired,
    CropNotAllowed,
    CropOutOfBounds,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedBrowserCdpEvidenceRefs {
    pub target_ref: String,
    pub url_ref: String,
    pub title_ref: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManagedBrowserCdpCaptureReceipt {
    pub schema_version: String,
    pub capture_ref: String,
    pub target_ref: String,
    pub evidence_refs: ManagedBrowserCdpEvidenceRefs,
    pub captured_at: String,
    pub structured_extraction_id: String,
    pub structured_evidence_digest: String,
    pub structured_signal_digest: String,
    pub structured_body_digest: String,
    pub document_frame_id: String,
    pub document_loader_id: String,
    pub document_url_digest: String,
    pub authority_digest: String,
    pub capture_context_digest: String,
    pub capture_mode: ManagedBrowserCdpCaptureMode,
    pub width: u32,
    pub height: u32,
    pub image_digest: String,
    pub image_byte_size: u64,
    pub image_format: String,
    pub custody_state: String,
    pub raw_image_retained: bool,
}
