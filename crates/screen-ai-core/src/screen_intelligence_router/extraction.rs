use super::capture::ScreenEvidenceCustodyState;
use serde::{Deserialize, Serialize};

pub(crate) const MANAGED_BROWSER_STRUCTURED_SOURCE_ID: &str = "managed-browser-cdp";
pub(crate) const MANAGED_BROWSER_SESSION_REF_PREFIX: &str = "managed-browser-session-";
pub(crate) const MANAGED_BROWSER_TARGET_REF_PREFIX: &str = "browser-target-";
pub(crate) const MANAGED_BROWSER_URL_REF_PREFIX: &str = "browser-url-";
pub(crate) const MANAGED_BROWSER_TITLE_REF_PREFIX: &str = "browser-title-";
pub(crate) const SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT: usize = 480;

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
    ScreenshotRequired,
    AuthorityUnavailable,
    Stale,
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
    Stale,
    Unavailable,
}

enum VerifiedStructuredExtractionOutcome {
    PolicySufficient {
        category_candidate: String,
        risk_signals: Vec<String>,
        confidence_basis: String,
    },
    NeedsScreenshot {
        reason: String,
    },
    Unavailable {
        reason: String,
    },
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
/// debug surface. A managed-browser owner must issue it only after binding the
/// live target, session, source identity, and freshness at its authority
/// boundary. No such producer is wired in this crate yet.
pub struct ScreenManagedBrowserStructuredExtraction {
    receipt: VerifiedManagedBrowserStructuredExtractionReceipt,
}

impl ScreenManagedBrowserStructuredExtraction {
    pub(crate) fn is_verified(&self) -> bool {
        receipt_is_bound_and_redacted(&self.receipt)
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

    pub(crate) fn can_answer_policy(&self) -> bool {
        self.receipt.freshness == ScreenStructuredExtractionFreshness::Fresh
            && self.receipt.redaction_state
                != ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
            && matches!(
                &self.receipt.outcome,
                VerifiedStructuredExtractionOutcome::PolicySufficient { .. }
            )
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

    pub(crate) fn requires_screenshot(&self) -> bool {
        matches!(
            &self.receipt.outcome,
            VerifiedStructuredExtractionOutcome::NeedsScreenshot { .. }
        )
    }

    pub(crate) fn is_stale(&self) -> bool {
        self.receipt.freshness == ScreenStructuredExtractionFreshness::Stale
    }

    pub(crate) fn is_fresh(&self) -> bool {
        self.receipt.freshness == ScreenStructuredExtractionFreshness::Fresh
    }

    pub(crate) fn is_unavailable(&self) -> bool {
        self.receipt.freshness == ScreenStructuredExtractionFreshness::Unavailable
            || matches!(
                &self.receipt.outcome,
                VerifiedStructuredExtractionOutcome::Unavailable { .. }
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
        && receipt
            .visible_text_summary
            .as_ref()
            .is_none_or(|summary| summary.len() <= SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT)
        && !receipt.raw_dom_included
        && receipt.custody_state != ScreenEvidenceCustodyState::OcentraHostedNonActivity
        && redaction_flags_are_consistent(receipt)
}

fn redaction_flags_are_consistent(
    receipt: &VerifiedManagedBrowserStructuredExtractionReceipt,
) -> bool {
    match receipt.redaction_state {
        ScreenStructuredExtractionRedactionState::None => {
            !receipt.private_content_redacted && !receipt.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::PrivateTextRedacted => {
            receipt.private_content_redacted && !receipt.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::OverflowRedacted => {
            !receipt.private_content_redacted && receipt.dom_overflow_redacted
        }
        ScreenStructuredExtractionRedactionState::ProtectedContentSkipped => true,
    }
}
