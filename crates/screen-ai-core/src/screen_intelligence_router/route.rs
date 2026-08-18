use super::capture::{ScreenCaptureScope, ScreenEvidenceCustodyState};
use super::extraction::{
    ActivityEvidenceRef, ScreenManagedBrowserStructuredExtraction,
    ScreenStructuredExtractionFallbackState,
};
use super::policy::{
    ScreenIntelligencePolicySensitivity, ScreenIntelligenceRouteKind, ScreenIntelligenceSourceKind,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenIntelligenceRouteRequest {
    pub schema_version: u16,
    pub request_id: String,
    pub requested_at: String,
    pub device_ref: String,
    pub source_kind: ScreenIntelligenceSourceKind,
    pub capture_reason: String,
    pub policy_question: String,
    pub policy_sensitivity: ScreenIntelligencePolicySensitivity,
    pub existing_evidence_refs: Vec<ActivityEvidenceRef>,
    pub structured_extraction: Option<ScreenManagedBrowserStructuredExtraction>,
    pub parent_allows_managed_browser_structured_extraction: bool,
    pub parent_allows_screen_capture: bool,
    pub allowed_capture_scopes: Vec<ScreenCaptureScope>,
    pub protected_surface_suspected: bool,
    pub credential_prompt_suspected: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenIntelligenceRouteDecision {
    pub schema_version: u16,
    pub route_id: String,
    pub request_id: String,
    pub decided_at: String,
    pub source_kind: ScreenIntelligenceSourceKind,
    pub route_kind: ScreenIntelligenceRouteKind,
    pub capture_scope: Option<ScreenCaptureScope>,
    pub structured_extraction_id: Option<String>,
    pub screenshot_skipped: bool,
    pub checked_existing_evidence_first: bool,
    pub managed_browser_structured_extraction_first: bool,
    pub structured_extraction_fallback_state: ScreenStructuredExtractionFallbackState,
    pub policy_question: String,
    pub policy_sensitivity: ScreenIntelligencePolicySensitivity,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
    pub custody_state: ScreenEvidenceCustodyState,
    pub manual_required_reason: Option<String>,
    pub unavailable_reason: Option<String>,
    pub remote_ai_allowed: bool,
    pub raw_screenshot_retained: bool,
}
