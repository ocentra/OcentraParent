use super::ManagedBrowserCdpStructuredExtraction;

impl ManagedBrowserCdpStructuredExtraction {
    pub fn structured_signal_digest(&self) -> &str {
        &self.structured_signal_digest
    }

    pub fn structured_body_digest(&self) -> &str {
        &self.structured_body_digest
    }

    pub fn document_frame_id(&self) -> Option<&str> {
        self.document_frame_id.as_deref()
    }

    pub fn document_loader_id(&self) -> Option<&str> {
        self.document_loader_id.as_deref()
    }

    pub fn document_url_digest(&self) -> Option<&str> {
        self.document_url_digest.as_deref()
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }
}
