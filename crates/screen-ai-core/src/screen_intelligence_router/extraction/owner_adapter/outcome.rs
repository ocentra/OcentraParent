use ocentra_parent_screen_capture_adapter::managed_browser_cdp::structured_extraction::ManagedBrowserStructuredExtraction;

use super::super::{ScreenEvidenceCustodyState, VerifiedStructuredExtractionOutcome};

pub(super) fn outcome_for(
    extraction: &ManagedBrowserStructuredExtraction,
) -> VerifiedStructuredExtractionOutcome {
    if extraction.protected_content_skipped() {
        VerifiedStructuredExtractionOutcome::ProtectedContentSkipped
    } else if extraction.is_unavailable() {
        VerifiedStructuredExtractionOutcome::Unavailable {
            reason: String::from("managed-browser structured extraction is unavailable"),
        }
    } else if extraction.has_structured_evidence() {
        VerifiedStructuredExtractionOutcome::StructuredEvidenceAvailable {
            reason: String::from(
                "bounded managed-browser signals are available for policy-owner review; no policy sufficiency asserted",
            ),
        }
    } else if extraction.requires_review() {
        VerifiedStructuredExtractionOutcome::ReviewRequired {
            reason: String::from(
                "managed-browser structured evidence is bounded but insufficient for an automated policy decision",
            ),
        }
    } else {
        VerifiedStructuredExtractionOutcome::Unavailable {
            reason: String::from("managed-browser structured extraction outcome is unavailable"),
        }
    }
}

pub(super) fn custody_state_for(
    _extraction: &ManagedBrowserStructuredExtraction,
) -> ScreenEvidenceCustodyState {
    ScreenEvidenceCustodyState::Unavailable
}
