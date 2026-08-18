use super::{Freshness, ManagedBrowserCdpStructuredExtraction, Outcome};

impl ManagedBrowserCdpStructuredExtraction {
    pub fn source_id(&self) -> &str {
        self.source_id
    }

    pub fn extraction_id(&self) -> &str {
        &self.extraction_id
    }

    pub fn captured_at(&self) -> &str {
        &self.captured_at
    }

    pub fn managed_browser_session_ref(&self) -> &str {
        &self.managed_browser_session_ref
    }

    pub fn target_ref(&self) -> &str {
        &self.target_ref
    }

    pub fn evidence_refs(
        &self,
    ) -> &ocentra_schema::managed_browser_cdp_capture::ManagedBrowserCdpEvidenceRefs {
        &self.evidence_refs
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn visible_text_summary(&self) -> Option<&str> {
        self.visible_text_summary.as_deref()
    }

    pub fn visible_text_character_count(&self) -> usize {
        self.visible_text_character_count
    }

    pub fn dom_overflow_redacted(&self) -> bool {
        self.dom_overflow_redacted
    }

    pub fn private_content_redacted(&self) -> bool {
        self.private_content_redacted
    }

    pub fn is_fresh(&self) -> bool {
        self.freshness == Freshness::Fresh
    }

    pub fn is_stale(&self) -> bool {
        self.freshness == Freshness::Stale
    }

    pub fn is_policy_sufficient(&self) -> bool {
        matches!(&self.outcome, Outcome::PolicySufficient)
    }

    pub fn requires_screenshot(&self) -> bool {
        matches!(&self.outcome, Outcome::NeedsScreenshot)
    }

    pub fn protected_content_skipped(&self) -> bool {
        matches!(&self.outcome, Outcome::ProtectedContentSkipped)
    }

    pub fn is_unavailable(&self) -> bool {
        matches!(&self.outcome, Outcome::Unavailable)
    }

    pub fn custody_state(&self) -> &str {
        self.custody_state
    }
}
