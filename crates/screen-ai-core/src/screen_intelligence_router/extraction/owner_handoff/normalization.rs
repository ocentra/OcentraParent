use super::super::super::{
    MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE, MANAGED_BROWSER_SESSION_REF_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_AUTHORITY_DIGEST_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_EVIDENCE_DIGEST_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_EVIDENCE_KIND_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_EXTRACTION_ID_UNAVAILABLE,
    MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE, MANAGED_BROWSER_TARGET_REF_UNAVAILABLE,
    MANAGED_BROWSER_TITLE_REF_UNAVAILABLE, MANAGED_BROWSER_URL_REF_UNAVAILABLE,
};
use super::super::{ActivityEvidenceRef, ManagedBrowserStructuredExtractionObservation};

pub(super) fn normalize(
    mut observation: ManagedBrowserStructuredExtractionObservation,
) -> ManagedBrowserStructuredExtractionObservation {
    if !observation.unavailable {
        return observation;
    }
    // Preserve the attempted observation time; it is not content or authority identity.
    observation.extraction_id = String::from(MANAGED_BROWSER_STRUCTURED_EXTRACTION_ID_UNAVAILABLE);
    observation.managed_browser_session_ref = String::from(MANAGED_BROWSER_SESSION_REF_UNAVAILABLE);
    observation.target_ref = String::from(MANAGED_BROWSER_TARGET_REF_UNAVAILABLE);
    observation.authority_digest =
        String::from(MANAGED_BROWSER_STRUCTURED_AUTHORITY_DIGEST_UNAVAILABLE);
    observation.structured_evidence_digest =
        String::from(MANAGED_BROWSER_STRUCTURED_EVIDENCE_DIGEST_UNAVAILABLE);
    observation.evidence_refs = vec![
        unavailable_evidence_ref(MANAGED_BROWSER_TARGET_REF_UNAVAILABLE),
        unavailable_evidence_ref(MANAGED_BROWSER_URL_REF_UNAVAILABLE),
        unavailable_evidence_ref(MANAGED_BROWSER_TITLE_REF_UNAVAILABLE),
    ];
    observation.structured_signal_digest =
        String::from(MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE);
    observation.structured_body_digest.clear();
    observation.structured_sensitivity_digest =
        String::from(MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE);
    observation.document_frame_id = None;
    observation.document_loader_id = None;
    observation.document_url_digest = None;
    observation.dom_overflow_redacted = false;
    observation.private_content_redacted = false;
    observation.protected_content_skipped = false;
    observation.fresh = false;
    observation
}

fn unavailable_evidence_ref(evidence_id: &str) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: String::from(evidence_id),
        kind: String::from(MANAGED_BROWSER_STRUCTURED_EVIDENCE_KIND_UNAVAILABLE),
        digest: String::from(MANAGED_BROWSER_STRUCTURED_EVIDENCE_DIGEST_UNAVAILABLE),
        uri: None,
    }
}
