use super::super::capture::ScreenEvidenceCustodyState;
use super::ActivityEvidenceRef;

#[path = "owner_handoff.rs"]
pub mod handoff;

/// Neutral, non-authorizing observation from a managed-browser producer.
///
/// The handoff exposes only bounded evidence references, digests, and identity
/// projections. It carries no DOM, page metadata, accessibility text, image,
/// policy decision, or capture authority. The router validates it but never
/// treats it as policy, capture, or browser authority.
pub trait ManagedBrowserStructuredExtractionOwner: Send + Sync {
    fn observation(self: Box<Self>) -> ManagedBrowserStructuredExtractionObservation;
}

/// Bounded, redacted producer observation. This is intentionally untrusted:
/// the router validates it but never treats it as policy, capture, or browser
/// authority.
pub struct ManagedBrowserStructuredExtractionObservation {
    pub source_id: String,
    pub extraction_id: String,
    pub captured_at: String,
    pub managed_browser_session_ref: String,
    pub target_ref: String,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
    pub structured_evidence_digest: String,
    pub structured_signal_digest: String,
    pub structured_body_digest: String,
    pub structured_sensitivity_digest: String,
    pub document_frame_id: Option<String>,
    pub document_loader_id: Option<String>,
    pub document_url_digest: Option<String>,
    pub authority_digest: String,
    pub dom_overflow_redacted: bool,
    pub private_content_redacted: bool,
    pub protected_content_skipped: bool,
    pub fresh: bool,
    pub unavailable: bool,
    pub custody_state: ScreenEvidenceCustodyState,
}
