use std::net::SocketAddr;

use ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpEvidenceRefs;

use super::{authority::LaunchBinding, target::TargetSnapshot, ManagedBrowserCdpCaptureError};

#[path = "structured/accessors.rs"]
mod accessors;
#[path = "structured/binding.rs"]
mod binding;
#[path = "structured/parser.rs"]
mod parser;
#[path = "structured/transport.rs"]
mod transport;

/// Opaque evidence issued by the managed-browser/CDP owner after it has
/// rebound the live process, bridge, and target and evaluated only bounded,
/// redacted browser signals. The type deliberately has no public constructor,
/// serde surface, `Clone`, or `Debug` implementation: callers can hand this
/// evidence to a typed consumer, but cannot mint its authority or outcome.
pub struct ManagedBrowserCdpStructuredExtraction {
    source_id: &'static str,
    extraction_id: String,
    captured_at: String,
    managed_browser_session_ref: String,
    target_ref: String,
    evidence_refs: ManagedBrowserCdpEvidenceRefs,
    evidence_digest: String,
    visible_text_summary: Option<String>,
    visible_text_character_count: usize,
    dom_overflow_redacted: bool,
    private_content_redacted: bool,
    freshness: Freshness,
    outcome: Outcome,
    custody_state: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Freshness {
    Fresh,
    Stale,
}

pub(super) enum Outcome {
    PolicySufficient,
    NeedsScreenshot,
    ProtectedContentSkipped,
    Unavailable,
}

pub(super) struct Payload {
    pub(super) visible_text_summary: Option<String>,
    pub(super) visible_text_character_count: usize,
    pub(super) dom_overflow_redacted: bool,
    pub(super) private_content_redacted: bool,
    pub(super) signal_digest: String,
    pub(super) outcome: Outcome,
}

impl Payload {
    pub(super) fn unavailable() -> Self {
        Self {
            visible_text_summary: None,
            visible_text_character_count: 0,
            dom_overflow_redacted: false,
            private_content_redacted: false,
            signal_digest: String::new(),
            outcome: Outcome::Unavailable,
        }
    }
}

pub(super) enum ExtractionError {
    Transport,
    InvalidResponse,
    ResponseTooLarge,
}

pub(super) fn extract(
    endpoint: SocketAddr,
    websocket_url: &str,
) -> Result<Payload, ExtractionError> {
    transport::extract(endpoint, websocket_url)
}

pub(super) fn ensure_capture_safe(
    endpoint: SocketAddr,
    websocket_url: &str,
) -> Result<(), ManagedBrowserCdpCaptureError> {
    transport::ensure_capture_safe(endpoint, websocket_url)
}

pub(super) fn bind_extraction(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
    captured_at_epoch_ms: u64,
    payload: Payload,
) -> ManagedBrowserCdpStructuredExtraction {
    binding::bind_extraction(binding, target_id, snapshot, captured_at_epoch_ms, payload)
}
