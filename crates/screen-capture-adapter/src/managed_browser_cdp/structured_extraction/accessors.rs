use super::ManagedBrowserStructuredExtraction;

#[path = "identity_accessors.rs"]
mod identity_accessors;

impl ManagedBrowserStructuredExtraction {
    pub fn visible_text_character_count(&self) -> usize {
        self.extraction.visible_text_character_count()
    }

    pub fn dom_overflow_redacted(&self) -> bool {
        self.extraction.dom_overflow_redacted()
    }

    pub fn private_content_redacted(&self) -> bool {
        self.extraction.private_content_redacted()
    }

    pub fn has_structured_evidence(&self) -> bool {
        self.extraction.has_structured_evidence()
    }

    pub fn requires_review(&self) -> bool {
        self.extraction.requires_review()
    }

    pub fn protected_content_skipped(&self) -> bool {
        self.extraction.protected_content_skipped()
    }

    pub fn is_unavailable(&self) -> bool {
        self.extraction.is_unavailable()
    }

    pub fn is_fresh(&self) -> bool {
        self.extraction.is_fresh()
    }

    pub fn custody_state(&self) -> &str {
        self.extraction.custody_state()
    }
}
