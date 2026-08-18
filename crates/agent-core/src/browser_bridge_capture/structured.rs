use std::{net::SocketAddr, time::Duration};

use ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpEvidenceRefs;

use super::{
    authority::LaunchBinding,
    target::{DocumentIdentity, TargetSnapshot},
    ManagedBrowserCdpCaptureError,
};

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
    Unavailable,
}

pub(super) enum Outcome {
    StructuredEvidenceAvailable,
    ReviewRequired,
    ProtectedContentSkipped,
    Unavailable,
}

pub(super) struct Payload {
    pub(super) visible_text_summary: Option<String>,
    pub(super) visible_text_character_count: usize,
    pub(super) dom_overflow_redacted: bool,
    pub(super) private_content_redacted: bool,
    pub(super) signal_digest: String,
    pub(super) sensitivity_digest: String,
    pub(super) capture_safe: bool,
    pub(super) document_url_digest: String,
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
            sensitivity_digest: String::from("managed-browser-sensitivity-unavailable-v1"),
            capture_safe: false,
            document_url_digest: String::new(),
            outcome: Outcome::Unavailable,
        }
    }

    pub(super) fn protected_content_skipped() -> Self {
        Self {
            visible_text_summary: None,
            visible_text_character_count: 0,
            dom_overflow_redacted: false,
            private_content_redacted: true,
            signal_digest: String::from("protected-content-redacted-v1"),
            sensitivity_digest: String::from("managed-browser-sensitivity-protected-v1"),
            capture_safe: false,
            document_url_digest: String::new(),
            outcome: Outcome::ProtectedContentSkipped,
        }
    }
}

pub(super) struct EvaluatedPayload {
    pub(super) payload: Payload,
    pub(super) document_identity: DocumentIdentity,
}

pub(super) enum ExtractionError {
    Transport,
    InvalidResponse,
    ResponseTooLarge,
    DocumentChanged,
}

pub(super) fn extract(
    endpoint: SocketAddr,
    websocket_url: &str,
) -> Result<EvaluatedPayload, ExtractionError> {
    transport::extract(endpoint, websocket_url)
}

pub(super) fn extract_on_session(
    session: &mut super::transport::CdpSession,
) -> Result<EvaluatedPayload, ExtractionError> {
    transport::extract_on_session(session)
}

pub(super) fn capture_error(error: ExtractionError) -> ManagedBrowserCdpCaptureError {
    match error {
        ExtractionError::Transport => ManagedBrowserCdpCaptureError::Transport,
        ExtractionError::ResponseTooLarge => ManagedBrowserCdpCaptureError::ResponseTooLarge,
        ExtractionError::InvalidResponse | ExtractionError::DocumentChanged => {
            ManagedBrowserCdpCaptureError::TargetAuthorityMismatch
        }
    }
}

pub(super) fn bind_extraction(
    binding: &LaunchBinding,
    target_id: &str,
    snapshot: &TargetSnapshot,
    captured_at_epoch_ms: u64,
    captured_at_monotonic: Duration,
    document_identity: Option<&DocumentIdentity>,
    payload: Payload,
) -> ManagedBrowserCdpStructuredExtraction {
    binding::bind_extraction(
        binding,
        target_id,
        snapshot,
        captured_at_epoch_ms,
        captured_at_monotonic,
        document_identity,
        payload,
    )
}
