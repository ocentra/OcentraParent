use super::capture::ScreenEvidenceCustodyState;
use serde::{Deserialize, Serialize};

#[path = "extraction/digest_validation.rs"]
mod digest_validation;
#[path = "extraction/owner.rs"]
pub mod owner;

pub(crate) const MANAGED_BROWSER_STRUCTURED_SOURCE_ID: &str = "managed-browser-cdp";
pub(crate) const MANAGED_BROWSER_SESSION_REF_PREFIX: &str = "managed-browser-session-";
pub(crate) const MANAGED_BROWSER_TARGET_REF_PREFIX: &str = "browser-target-";
pub(crate) const MANAGED_BROWSER_URL_REF_PREFIX: &str = "browser-url-";
pub(crate) const MANAGED_BROWSER_TITLE_REF_PREFIX: &str = "browser-title-";
pub(crate) const SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT: usize = 480;
const MANAGED_BROWSER_STRUCTURED_SIGNAL_PROTECTED: &str = "protected-content-redacted-v1";
const MANAGED_BROWSER_STRUCTURED_SIGNAL_UNAVAILABLE: &str =
    "managed-browser-structured-unavailable-v1";
const MANAGED_BROWSER_SENSITIVITY_STRUCTURAL_SAFE: &str =
    "managed-browser-sensitivity-structural-safe-v1";
const MANAGED_BROWSER_SENSITIVITY_UNKNOWN: &str = "managed-browser-sensitivity-unknown-v1";
const MANAGED_BROWSER_SENSITIVITY_PROTECTED: &str = "managed-browser-sensitivity-protected-v1";
const MANAGED_BROWSER_SENSITIVITY_UNAVAILABLE: &str = "managed-browser-sensitivity-unavailable-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvidenceRef {
    pub evidence_id: String,
    pub kind: String,
    pub digest: String,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenStructuredExtractionFallbackState {
    NotAttempted,
    NotRequired,
    AuthorityUnavailable,
    RedactedEvidenceInsufficient,
}

#[derive(PartialEq, Eq)]
enum ScreenStructuredExtractionRedactionState {
    None,
    PrivateTextRedacted,
    OverflowRedacted,
    ProtectedContentSkipped,
}

#[derive(PartialEq, Eq)]
enum ScreenStructuredExtractionFreshness {
    Fresh,
    Unavailable,
}

enum VerifiedStructuredExtractionOutcome {
    ReviewRequired,
    ProtectedContentSkipped,
    Unavailable,
}

struct VerifiedManagedBrowserStructuredExtractionAuthority {
    source_id: String,
    managed_browser_session_ref: String,
    target_ref: String,
}

struct VerifiedManagedBrowserStructuredExtractionReceipt {
    schema_version: u16,
    extraction_id: String,
    captured_at: String,
    authority: VerifiedManagedBrowserStructuredExtractionAuthority,
    evidence_refs: Vec<ActivityEvidenceRef>,
    structured_evidence_digest: String,
    structured_signal_digest: String,
    structured_body_digest: String,
    structured_sensitivity_digest: String,
    document_frame_id: Option<String>,
    document_loader_id: Option<String>,
    document_url_digest: Option<String>,
    authority_digest: String,
    freshness: ScreenStructuredExtractionFreshness,
    visible_text_summary: Option<String>,
    visible_text_character_count: usize,
    dom_overflow_redacted: bool,
    private_content_redacted: bool,
    raw_dom_included: bool,
    redaction_state: ScreenStructuredExtractionRedactionState,
    outcome: VerifiedStructuredExtractionOutcome,
    custody_state: ScreenEvidenceCustodyState,
}

/// This receipt has no public or crate-wide constructor, serializer, clone, or
/// debug surface. The public handoff accepts only a bounded neutral observation
/// and never turns caller data into browser, capture, or policy authority.
pub struct ScreenManagedBrowserStructuredExtraction {
    receipt: VerifiedManagedBrowserStructuredExtractionReceipt,
    owner_authority_is_validated: bool,
}

impl ScreenManagedBrowserStructuredExtraction {
    pub(crate) fn is_verified(&self) -> bool {
        receipt_is_bound_and_redacted(&self.receipt)
    }

    pub(crate) fn owner_authority_is_validated(&self) -> bool {
        self.owner_authority_is_validated
    }

    pub(crate) fn extraction_id(&self) -> &str {
        &self.receipt.extraction_id
    }

    pub(crate) fn captured_at(&self) -> &str {
        &self.receipt.captured_at
    }

    pub(crate) fn evidence_refs(&self) -> &[ActivityEvidenceRef] {
        &self.receipt.evidence_refs
    }

    pub(crate) fn custody_state(&self) -> ScreenEvidenceCustodyState {
        self.receipt.custody_state.clone()
    }

    pub(crate) fn protected_content_skipped(&self) -> bool {
        self.receipt.redaction_state
            == ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
    }

    pub(crate) fn authority_is_managed_browser(&self) -> bool {
        self.receipt.authority.source_id == MANAGED_BROWSER_STRUCTURED_SOURCE_ID
            && self
                .receipt
                .authority
                .managed_browser_session_ref
                .starts_with(MANAGED_BROWSER_SESSION_REF_PREFIX)
            && self
                .receipt
                .authority
                .target_ref
                .starts_with(MANAGED_BROWSER_TARGET_REF_PREFIX)
    }

    pub(crate) fn has_structured_evidence(&self) -> bool {
        false
    }

    pub(crate) fn requires_review(&self) -> bool {
        matches!(
            &self.receipt.outcome,
            VerifiedStructuredExtractionOutcome::ReviewRequired
        )
    }

    pub(crate) fn is_fresh(&self) -> bool {
        self.receipt.freshness == ScreenStructuredExtractionFreshness::Fresh
    }

    pub(crate) fn is_unavailable(&self) -> bool {
        self.receipt.freshness == ScreenStructuredExtractionFreshness::Unavailable
            || matches!(
                &self.receipt.outcome,
                VerifiedStructuredExtractionOutcome::Unavailable
            )
    }
}

fn receipt_is_bound_and_redacted(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    receipt.schema_version > 0
        && !receipt.extraction_id.trim().is_empty()
        && !receipt.captured_at.trim().is_empty()
        && receipt.authority.source_id == MANAGED_BROWSER_STRUCTURED_SOURCE_ID
        && receipt
            .authority
            .managed_browser_session_ref
            .starts_with(MANAGED_BROWSER_SESSION_REF_PREFIX)
        && receipt
            .authority
            .target_ref
            .starts_with(MANAGED_BROWSER_TARGET_REF_PREFIX)
        && receipt.evidence_refs.len() >= 3
        && receipt.evidence_refs.iter().all(|reference| {
            !reference.evidence_id.trim().is_empty()
                && !reference.digest.trim().is_empty()
                && reference.uri.is_none()
        })
        && digest_validation::valid_digest(&receipt.structured_evidence_digest)
        && digest_validation::valid_signal_digest(receipt)
        && ((receipt.redaction_state
            == ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
            && receipt.structured_body_digest == "protected-content-redacted-v1")
            || (matches!(
                &receipt.outcome,
                VerifiedStructuredExtractionOutcome::Unavailable
            ) && receipt.structured_body_digest.is_empty())
            || digest_validation::valid_body_digest(&receipt.structured_body_digest))
        && digest_validation::valid_digest(&receipt.authority_digest)
        && digest_validation::valid_sensitivity_digest(&receipt.structured_sensitivity_digest)
        && document_identity_is_consistent(receipt)
        && receipt.evidence_refs.iter().any(|reference| {
            reference.evidence_id == receipt.authority.target_ref
                && reference
                    .evidence_id
                    .starts_with(MANAGED_BROWSER_TARGET_REF_PREFIX)
        })
        && receipt.evidence_refs.iter().any(|reference| {
            reference
                .evidence_id
                .starts_with(MANAGED_BROWSER_URL_REF_PREFIX)
        })
        && receipt.evidence_refs.iter().any(|reference| {
            reference
                .evidence_id
                .starts_with(MANAGED_BROWSER_TITLE_REF_PREFIX)
        })
        && receipt.visible_text_character_count <= SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT
        && receipt.visible_text_summary.as_ref().is_none_or(|summary| {
            let character_count = summary.chars().count();
            character_count <= SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT
                && character_count <= receipt.visible_text_character_count
        })
        && !receipt.raw_dom_included
        && receipt.custody_state != ScreenEvidenceCustodyState::OcentraHostedNonActivity
        && redaction_flags_are_consistent(receipt)
}

fn document_identity_is_consistent(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    if matches!(
        &receipt.outcome,
        VerifiedStructuredExtractionOutcome::ProtectedContentSkipped
            | VerifiedStructuredExtractionOutcome::Unavailable
    ) {
        return true;
    }
    receipt
        .document_frame_id
        .as_deref()
        .is_some_and(|value| !value.trim().is_empty())
        && receipt
            .document_loader_id
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
        && receipt
            .document_url_digest
            .as_deref()
            .is_some_and(digest_validation::valid_digest)
}

fn redaction_flags_are_consistent(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    match &receipt.redaction_state {
        ScreenStructuredExtractionRedactionState::None => {
            !receipt.private_content_redacted && !receipt.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::PrivateTextRedacted => {
            receipt.private_content_redacted
                && receipt.visible_text_summary.is_none()
                && receipt.visible_text_character_count == 0
                && !receipt.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::OverflowRedacted => {
            !receipt.private_content_redacted && receipt.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::ProtectedContentSkipped => {
            receipt.private_content_redacted
                && receipt.visible_text_summary.is_none()
                && receipt.visible_text_character_count == 0
                && !receipt.dom_overflow_redacted
        }
    }
}
