use ocentra_parent_screen_capture_adapter::managed_browser_cdp::structured_extraction::ManagedBrowserStructuredExtraction;

use super::{
    ActivityEvidenceRef, ScreenManagedBrowserStructuredExtraction,
    ScreenStructuredExtractionFreshness, ScreenStructuredExtractionRedactionState,
    VerifiedManagedBrowserStructuredExtractionAuthority,
    VerifiedManagedBrowserStructuredExtractionReceipt,
};

#[path = "owner_adapter/outcome.rs"]
mod outcome;

pub(super) fn from_owner_extraction(
    extraction: ManagedBrowserStructuredExtraction,
) -> ScreenManagedBrowserStructuredExtraction {
    let redaction_state = redaction_state_for(&extraction);
    let freshness = freshness_for(&extraction);
    let outcome = outcome::outcome_for(&extraction);
    let evidence = evidence_refs_for(&extraction);
    let custody_state = outcome::custody_state_for(&extraction);

    ScreenManagedBrowserStructuredExtraction {
        receipt: VerifiedManagedBrowserStructuredExtractionReceipt {
            schema_version: 1,
            extraction_id: extraction.extraction_id().to_owned(),
            captured_at: extraction.captured_at().to_owned(),
            authority: VerifiedManagedBrowserStructuredExtractionAuthority {
                source_id: extraction.source_id().to_owned(),
                managed_browser_session_ref: extraction.managed_browser_session_ref().to_owned(),
                target_ref: extraction.target_ref().to_owned(),
            },
            evidence_refs: evidence,
            freshness,
            visible_text_summary: extraction.visible_text_summary().map(str::to_owned),
            visible_text_character_count: extraction.visible_text_character_count(),
            dom_overflow_redacted: extraction.dom_overflow_redacted(),
            private_content_redacted: extraction.private_content_redacted(),
            raw_dom_included: false,
            redaction_state,
            outcome,
            custody_state,
        },
    }
}

fn redaction_state_for(
    extraction: &ManagedBrowserStructuredExtraction,
) -> ScreenStructuredExtractionRedactionState {
    if extraction.protected_content_skipped() {
        ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
    } else if extraction.private_content_redacted() {
        ScreenStructuredExtractionRedactionState::PrivateTextRedacted
    } else if extraction.dom_overflow_redacted() {
        ScreenStructuredExtractionRedactionState::OverflowRedacted
    } else {
        ScreenStructuredExtractionRedactionState::None
    }
}

fn freshness_for(
    extraction: &ManagedBrowserStructuredExtraction,
) -> ScreenStructuredExtractionFreshness {
    if extraction.is_stale() {
        ScreenStructuredExtractionFreshness::Stale
    } else if extraction.is_fresh() {
        ScreenStructuredExtractionFreshness::Fresh
    } else {
        ScreenStructuredExtractionFreshness::Unavailable
    }
}

fn evidence_refs_for(extraction: &ManagedBrowserStructuredExtraction) -> Vec<ActivityEvidenceRef> {
    let digest = extraction.evidence_digest().to_owned();
    vec![
        ActivityEvidenceRef {
            evidence_id: extraction.target_ref().to_owned(),
            kind: String::from("managed-browser-target"),
            digest: digest.clone(),
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: extraction.url_ref().to_owned(),
            kind: String::from("managed-browser-url"),
            digest: digest.clone(),
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: extraction.title_ref().to_owned(),
            kind: String::from("managed-browser-title"),
            digest,
            uri: None,
        },
    ]
}
