//! Screen-owned adapter for browser-target capture.
//!
//! This adapter is deliberately separate from desktop capture. It accepts only
//! the browser-owned target authority. Structured evidence is handed off
//! through an opaque producer token; screenshot bytes arrive only after the
//! browser owner completes its frozen-page guard and post-capture checks. It
//! has no desktop, screencast, or remote mode.

use std::fmt;

use ocentra_parent_agent_core::browser_bridge_capture::{
    capture_receipt, ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority,
};
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpCaptureMode, ManagedBrowserCdpCaptureReceipt, ManagedBrowserCdpCaptureRequest,
    MANAGED_BROWSER_CDP_CAPTURE_REF_PREFIX, MANAGED_BROWSER_CDP_MAX_DIMENSION,
    MANAGED_BROWSER_CDP_MAX_PIXELS,
};
use sha2::{Digest, Sha256};

const CDP_MAX_IMAGE_BYTES: usize = 32 * 1024 * 1024;

#[path = "managed_browser_cdp/decoder.rs"]
mod decoder;
pub mod structured_extraction;

#[derive(Clone, PartialEq, Eq)]
pub struct ManagedBrowserCdpScreenCapture {
    receipt: ManagedBrowserCdpCaptureReceipt,
    png_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManagedBrowserCdpScreenCaptureError {
    Browser(ManagedBrowserCdpCaptureError),
    InvalidPng,
    DimensionsOutOfBounds,
    RequestedDimensionsNotApplied,
}

impl From<ManagedBrowserCdpCaptureError> for ManagedBrowserCdpScreenCaptureError {
    fn from(error: ManagedBrowserCdpCaptureError) -> Self {
        Self::Browser(error)
    }
}

impl fmt::Debug for ManagedBrowserCdpScreenCapture {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ManagedBrowserCdpScreenCapture")
            .field("receipt", &self.receipt)
            .field("png_byte_size", &self.png_bytes.len())
            .finish()
    }
}

pub fn capture_managed_browser_cdp(
    authority: &ManagedBrowserCdpTargetAuthority,
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<ManagedBrowserCdpScreenCapture, ManagedBrowserCdpScreenCaptureError> {
    let capture = authority.capture(request)?;
    let (width, height) = decoder::decode_png(capture.png_bytes())?;
    validate_requested_dimensions(request, width, height)?;
    let image_digest = image_digest(capture.png_bytes());
    let capture_ref = capture_ref(&capture.evidence_refs().target_ref, &image_digest);
    let receipt = capture_receipt(
        capture_ref,
        &capture,
        request.mode,
        width,
        height,
        image_digest,
    );
    Ok(ManagedBrowserCdpScreenCapture {
        receipt,
        png_bytes: capture.png_bytes().to_vec(),
    })
}

impl ManagedBrowserCdpScreenCapture {
    pub fn receipt(&self) -> &ManagedBrowserCdpCaptureReceipt {
        &self.receipt
    }

    pub fn png_bytes(&self) -> &[u8] {
        &self.png_bytes
    }
}

fn validate_requested_dimensions(
    request: &ManagedBrowserCdpCaptureRequest,
    width: u32,
    height: u32,
) -> Result<(), ManagedBrowserCdpScreenCaptureError> {
    let expected = match request.mode {
        ManagedBrowserCdpCaptureMode::Page => None,
        ManagedBrowserCdpCaptureMode::Viewport => Some((
            request
                .viewport_width
                .ok_or(ManagedBrowserCdpScreenCaptureError::RequestedDimensionsNotApplied)?,
            request
                .viewport_height
                .ok_or(ManagedBrowserCdpScreenCaptureError::RequestedDimensionsNotApplied)?,
        )),
        ManagedBrowserCdpCaptureMode::Crop => Some((
            request
                .crop
                .as_ref()
                .ok_or(ManagedBrowserCdpScreenCaptureError::RequestedDimensionsNotApplied)?
                .width,
            request
                .crop
                .as_ref()
                .ok_or(ManagedBrowserCdpScreenCaptureError::RequestedDimensionsNotApplied)?
                .height,
        )),
    };
    if expected.is_some_and(|(expected_width, expected_height)| {
        width != expected_width || height != expected_height
    }) {
        return Err(ManagedBrowserCdpScreenCaptureError::RequestedDimensionsNotApplied);
    }
    Ok(())
}

fn image_digest(bytes: &[u8]) -> String {
    let mut digest = String::new();
    for byte in Sha256::digest(bytes) {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}

fn capture_ref(target_ref: &str, image_digest: &str) -> String {
    let mut digest = Sha256::new();
    digest.update(target_ref.as_bytes());
    digest.update([0]);
    digest.update(image_digest.as_bytes());
    let mut value = String::from(MANAGED_BROWSER_CDP_CAPTURE_REF_PREFIX);
    for byte in digest.finalize() {
        value.push_str(&format!("{byte:02x}"));
    }
    value
}

pub(super) fn validate_dimensions(
    width: u32,
    height: u32,
) -> Result<(), ManagedBrowserCdpScreenCaptureError> {
    if width == 0
        || height == 0
        || width > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || height > MANAGED_BROWSER_CDP_MAX_DIMENSION
        || u64::from(width) * u64::from(height) > MANAGED_BROWSER_CDP_MAX_PIXELS
    {
        return Err(ManagedBrowserCdpScreenCaptureError::DimensionsOutOfBounds);
    }
    Ok(())
}

pub(super) fn max_decoded_bytes() -> u64 {
    u64::from(MANAGED_BROWSER_CDP_MAX_PIXELS) * 8
}

pub(super) fn max_png_bytes() -> usize {
    CDP_MAX_IMAGE_BYTES
}
