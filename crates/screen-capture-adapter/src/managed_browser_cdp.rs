//! Screen-owned adapter for browser-target capture.
//!
//! This adapter is deliberately separate from desktop capture. It accepts only
//! the browser-owned target authority and emits bounded PNG bytes for the
//! encrypted queue handoff; it has no desktop, screencast, or remote mode.

use ocentra_parent_agent_core::browser_bridge_capture::{
    capture_receipt, ManagedBrowserCdpCaptureError, ManagedBrowserCdpTargetAuthority,
};
use ocentra_schema::managed_browser_cdp_capture::{
    ManagedBrowserCdpCaptureReceipt, ManagedBrowserCdpCaptureRequest,
    MANAGED_BROWSER_CDP_CAPTURE_REF_PREFIX,
};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManagedBrowserCdpScreenCapture {
    receipt: ManagedBrowserCdpCaptureReceipt,
    png_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManagedBrowserCdpScreenCaptureError {
    Browser(ManagedBrowserCdpCaptureError),
    InvalidPng,
    DimensionsOutOfBounds,
}

impl From<ManagedBrowserCdpCaptureError> for ManagedBrowserCdpScreenCaptureError {
    fn from(error: ManagedBrowserCdpCaptureError) -> Self {
        Self::Browser(error)
    }
}

pub fn capture_managed_browser_cdp(
    authority: &ManagedBrowserCdpTargetAuthority,
    request: &ManagedBrowserCdpCaptureRequest,
) -> Result<ManagedBrowserCdpScreenCapture, ManagedBrowserCdpScreenCaptureError> {
    let capture = authority.capture(request)?;
    let (width, height) = png_dimensions(capture.png_bytes())?;
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

fn png_dimensions(bytes: &[u8]) -> Result<(u32, u32), ManagedBrowserCdpScreenCaptureError> {
    if bytes.len() < 24 || bytes.get(0..8) != Some(&[137, 80, 78, 71, 13, 10, 26, 10]) {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    if &bytes[12..16] != b"IHDR" {
        return Err(ManagedBrowserCdpScreenCaptureError::InvalidPng);
    }
    let width = u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]);
    let height = u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]);
    if width == 0
        || height == 0
        || width > ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_MAX_DIMENSION
        || height > ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_MAX_DIMENSION
        || u64::from(width) * u64::from(height)
            > ocentra_schema::managed_browser_cdp_capture::MANAGED_BROWSER_CDP_MAX_PIXELS
    {
        return Err(ManagedBrowserCdpScreenCaptureError::DimensionsOutOfBounds);
    }
    Ok((width, height))
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
