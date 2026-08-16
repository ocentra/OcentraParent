use super::capture::ScreenEvidenceCustodyState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenStructuredExtractionState {
    EnoughForPolicy,
    NeedsScreenshot,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenStructuredExtractionRedactionState {
    None,
    PrivateTextRedacted,
    OverflowRedacted,
    ProtectedContentSkipped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityEvidenceRef {
    pub evidence_id: String,
    pub kind: String,
    pub digest: String,
    pub uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenManagedBrowserStructuredExtraction {
    pub schema_version: u16,
    pub extraction_id: String,
    pub captured_at: String,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
    pub extraction_state: ScreenStructuredExtractionState,
    pub url_title_metadata_captured: bool,
    pub visible_text_summary: Option<String>,
    pub visible_text_character_count: usize,
    pub dom_overflow_redacted: bool,
    pub private_content_redacted: bool,
    pub raw_dom_included: bool,
    pub redaction_state: ScreenStructuredExtractionRedactionState,
    pub enough_for_policy: bool,
    pub policy_question_answered: bool,
    pub no_screen_needed: bool,
    pub screenshot_required: bool,
    pub category_candidate: Option<String>,
    pub risk_signals: Vec<String>,
    pub confidence: f64,
    pub custody_state: ScreenEvidenceCustodyState,
    pub reason: Option<String>,
}
