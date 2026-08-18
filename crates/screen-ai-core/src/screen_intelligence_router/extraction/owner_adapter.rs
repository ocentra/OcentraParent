use ocentra_parent_agent_core::browser_bridge_capture::{
    ManagedBrowserCdpCaptureError, ManagedBrowserCdpStructuredExtraction,
    ManagedBrowserCdpTargetAuthority,
};

use super::{
    ActivityEvidenceRef, ScreenEvidenceCustodyState, ScreenManagedBrowserStructuredExtraction,
    ScreenStructuredExtractionFreshness, ScreenStructuredExtractionRedactionState,
    VerifiedManagedBrowserStructuredExtractionAuthority,
    VerifiedManagedBrowserStructuredExtractionReceipt, VerifiedStructuredExtractionOutcome,
};

pub(super) fn from_authority(
    authority: &ManagedBrowserCdpTargetAuthority,
) -> Result<ScreenManagedBrowserStructuredExtraction, ManagedBrowserCdpCaptureError> {
    let extraction = authority.extract_structured()?;
    Ok(from_owner_extraction(extraction))
}

fn from_owner_extraction(
    extraction: ManagedBrowserCdpStructuredExtraction,
) -> ScreenManagedBrowserStructuredExtraction {
    let redaction_state = redaction_state_for(&extraction);
    let freshness = freshness_for(&extraction);
    let outcome = outcome_for(&extraction);
    let evidence = evidence_refs_for(&extraction);
    let custody_state = custody_state_for(&extraction);

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
    extraction: &ManagedBrowserCdpStructuredExtraction,
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
    extraction: &ManagedBrowserCdpStructuredExtraction,
) -> ScreenStructuredExtractionFreshness {
    if extraction.is_stale() {
        ScreenStructuredExtractionFreshness::Stale
    } else if extraction.is_fresh() {
        ScreenStructuredExtractionFreshness::Fresh
    } else {
        ScreenStructuredExtractionFreshness::Unavailable
    }
}

fn outcome_for(
    extraction: &ManagedBrowserCdpStructuredExtraction,
) -> VerifiedStructuredExtractionOutcome {
    if extraction.protected_content_skipped() {
        VerifiedStructuredExtractionOutcome::ProtectedContentSkipped
    } else if extraction.is_unavailable() {
        VerifiedStructuredExtractionOutcome::Unavailable {
            reason: String::from("managed-browser structured extraction is unavailable"),
        }
    } else if extraction.is_policy_sufficient() {
        VerifiedStructuredExtractionOutcome::PolicySufficient {
            category_candidate: String::from("managed-browser-page"),
            risk_signals: Vec::new(),
            confidence_basis: String::from(
                "bounded managed-browser URL/title/meta/DOM/accessibility signals; not policy authority",
            ),
        }
    } else if extraction.requires_screenshot() {
        VerifiedStructuredExtractionOutcome::NeedsScreenshot {
            reason: String::from(
                "bounded managed-browser structured signals did not answer the policy question",
            ),
        }
    } else {
        VerifiedStructuredExtractionOutcome::Unavailable {
            reason: String::from("managed-browser structured extraction outcome is unavailable"),
        }
    }
}

fn evidence_refs_for(
    extraction: &ManagedBrowserCdpStructuredExtraction,
) -> Vec<ActivityEvidenceRef> {
    let references = extraction.evidence_refs();
    let digest = extraction.evidence_digest().to_owned();
    vec![
        ActivityEvidenceRef {
            evidence_id: references.target_ref.clone(),
            kind: String::from("managed-browser-target"),
            digest: digest.clone(),
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: references.url_ref.clone(),
            kind: String::from("managed-browser-url"),
            digest: digest.clone(),
            uri: None,
        },
        ActivityEvidenceRef {
            evidence_id: references.title_ref.clone(),
            kind: String::from("managed-browser-title"),
            digest,
            uri: None,
        },
    ]
}

fn custody_state_for(
    extraction: &ManagedBrowserCdpStructuredExtraction,
) -> ScreenEvidenceCustodyState {
    match extraction.custody_state() {
        "live-local-child-agent" => ScreenEvidenceCustodyState::LiveLocalChildAgent,
        _ => ScreenEvidenceCustodyState::Unavailable,
    }
}
