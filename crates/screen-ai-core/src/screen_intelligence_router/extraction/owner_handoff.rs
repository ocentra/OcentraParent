use super::super::{
    receipt_is_bound_and_redacted, ScreenManagedBrowserStructuredExtraction,
    ScreenStructuredExtractionFreshness, ScreenStructuredExtractionRedactionState,
    VerifiedManagedBrowserStructuredExtractionAuthority,
    VerifiedManagedBrowserStructuredExtractionReceipt, VerifiedStructuredExtractionOutcome,
};
use super::{
    ManagedBrowserStructuredExtractionObservation, ManagedBrowserStructuredExtractionOwner,
};

#[path = "owner_handoff/normalization.rs"]
mod normalization;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenManagedBrowserStructuredExtractionHandoffError {
    InvalidOwnerHandoff,
}

impl ScreenManagedBrowserStructuredExtraction {
    pub fn from_untrusted_observation(
        owner: Box<dyn ManagedBrowserStructuredExtractionOwner>,
    ) -> Result<Self, ScreenManagedBrowserStructuredExtractionHandoffError> {
        let observation = normalization::normalize(owner.observation());
        let redaction_state = redaction_state_for(&observation);
        let outcome = outcome_for(&observation);
        let receipt = VerifiedManagedBrowserStructuredExtractionReceipt {
            schema_version: 1,
            extraction_id: observation.extraction_id,
            captured_at: observation.captured_at,
            authority: VerifiedManagedBrowserStructuredExtractionAuthority {
                source_id: observation.source_id,
                managed_browser_session_ref: observation.managed_browser_session_ref,
                target_ref: observation.target_ref,
            },
            evidence_refs: observation.evidence_refs,
            structured_evidence_digest: observation.structured_evidence_digest,
            structured_signal_digest: observation.structured_signal_digest,
            structured_body_digest: observation.structured_body_digest,
            structured_sensitivity_digest: observation.structured_sensitivity_digest,
            document_frame_id: observation.document_frame_id,
            document_loader_id: observation.document_loader_id,
            document_url_digest: observation.document_url_digest,
            authority_digest: observation.authority_digest,
            freshness: freshness_for(observation.fresh),
            visible_text_summary: None,
            visible_text_character_count: 0,
            dom_overflow_redacted: observation.dom_overflow_redacted,
            private_content_redacted: observation.private_content_redacted,
            raw_dom_included: false,
            redaction_state,
            outcome,
            custody_state: observation.custody_state,
        };
        if !receipt_is_bound_and_redacted(&receipt) {
            return Err(ScreenManagedBrowserStructuredExtractionHandoffError::InvalidOwnerHandoff);
        }
        Ok(Self {
            receipt,
            owner_authority_is_validated: false,
        })
    }
}

fn redaction_state_for(
    observation: &ManagedBrowserStructuredExtractionObservation,
) -> ScreenStructuredExtractionRedactionState {
    if observation.protected_content_skipped {
        ScreenStructuredExtractionRedactionState::ProtectedContentSkipped
    } else if observation.private_content_redacted {
        ScreenStructuredExtractionRedactionState::PrivateTextRedacted
    } else if observation.dom_overflow_redacted {
        ScreenStructuredExtractionRedactionState::OverflowRedacted
    } else {
        ScreenStructuredExtractionRedactionState::None
    }
}

fn outcome_for(
    observation: &ManagedBrowserStructuredExtractionObservation,
) -> VerifiedStructuredExtractionOutcome {
    match (
        observation.protected_content_skipped,
        observation.unavailable,
    ) {
        (true, _) => VerifiedStructuredExtractionOutcome::ProtectedContentSkipped,
        (false, true) => VerifiedStructuredExtractionOutcome::Unavailable,
        (false, false) => VerifiedStructuredExtractionOutcome::ReviewRequired,
    }
}

fn freshness_for(fresh: bool) -> ScreenStructuredExtractionFreshness {
    if fresh {
        ScreenStructuredExtractionFreshness::Fresh
    } else {
        ScreenStructuredExtractionFreshness::Unavailable
    }
}
