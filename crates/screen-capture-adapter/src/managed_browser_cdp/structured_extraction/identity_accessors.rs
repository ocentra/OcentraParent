use super::super::ManagedBrowserStructuredExtraction;

impl ManagedBrowserStructuredExtraction {
    pub fn source_id(&self) -> &str {
        self.extraction.source_id()
    }

    pub fn extraction_id(&self) -> &str {
        self.extraction.extraction_id()
    }

    pub fn captured_at(&self) -> &str {
        self.extraction.captured_at()
    }

    pub fn managed_browser_session_ref(&self) -> &str {
        self.extraction.managed_browser_session_ref()
    }

    pub fn target_ref(&self) -> &str {
        self.extraction.target_ref()
    }

    pub fn url_ref(&self) -> &str {
        &self.extraction.evidence_refs().url_ref
    }

    pub fn title_ref(&self) -> &str {
        &self.extraction.evidence_refs().title_ref
    }

    pub fn evidence_digest(&self) -> &str {
        self.extraction.evidence_digest()
    }

    pub fn visible_text_summary(&self) -> Option<&str> {
        self.extraction.visible_text_summary()
    }
}
