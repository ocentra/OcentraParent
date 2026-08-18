//! Neutral service-layer handoff for browser-owned structured evidence. The
//! browser owner retains capture safety and screenshot authority; this port
//! cannot mint either one.

use ocentra_parent_agent_core::browser_bridge_capture::{
    ManagedBrowserCdpCaptureError, ManagedBrowserCdpStructuredExtraction,
    ManagedBrowserCdpTargetAuthority,
};

#[path = "structured_extraction/accessors.rs"]
mod accessors;

/// The token is issued only by the browser-owned CDP authority and
/// intentionally has no serde, `Clone`, or `Debug` surface. Screen policy code
/// can consume this neutral port without depending on the browser owner crate
/// directly.
pub struct ManagedBrowserStructuredExtraction {
    extraction: ManagedBrowserCdpStructuredExtraction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManagedBrowserStructuredExtractionError {
    Browser(ManagedBrowserCdpCaptureError),
}

impl From<ManagedBrowserCdpCaptureError> for ManagedBrowserStructuredExtractionError {
    fn from(error: ManagedBrowserCdpCaptureError) -> Self {
        Self::Browser(error)
    }
}

pub fn extract_managed_browser_structured(
    authority: &ManagedBrowserCdpTargetAuthority,
) -> Result<ManagedBrowserStructuredExtraction, ManagedBrowserStructuredExtractionError> {
    Ok(ManagedBrowserStructuredExtraction {
        extraction: authority.extract_structured()?,
    })
}
