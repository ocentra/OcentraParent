pub(crate) const SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION: u16 = 1;

pub(crate) const MANUAL_REQUIRED_PARENT_DISABLED: &str =
    "parent setting requires manual review before screen capture";
pub(crate) const MANUAL_REQUIRED_UNSUPPORTED_SCOPE: &str =
    "no allowed active-window or selected-window capture scope is available";
pub(crate) const MANUAL_REQUIRED_EVIDENCE_ONLY: &str =
    "existing evidence requires an owner-backed answer before screen capture";
pub(crate) const UNAVAILABLE_PROTECTED_SURFACE: &str =
    "protected surface is not eligible for screen capture or model analysis";
pub(crate) const UNAVAILABLE_CREDENTIAL_PROMPT: &str =
    "credential prompt risk is not eligible for screen capture or model analysis";
pub(crate) const UNAVAILABLE_MANAGED_BROWSER_STRUCTURED_EXTRACTION: &str =
    "managed-browser structured extraction producer authority is unavailable";
pub(crate) const UNAVAILABLE_INCONSISTENT_REQUEST: &str =
    "screen intelligence route request is inconsistent or unsupported";

mod capture;
pub mod extraction;
mod policy;
mod route;

pub type ScreenCaptureScope = capture::ScreenCaptureScope;
pub type ScreenEvidenceCustodyState = capture::ScreenEvidenceCustodyState;
pub type ActivityEvidenceRef = extraction::ActivityEvidenceRef;
pub type ScreenManagedBrowserStructuredExtraction =
    extraction::ScreenManagedBrowserStructuredExtraction;
pub type ScreenStructuredExtractionFallbackState =
    extraction::ScreenStructuredExtractionFallbackState;
pub type ScreenIntelligencePolicySensitivity = policy::ScreenIntelligencePolicySensitivity;
pub type ScreenIntelligenceRouteKind = policy::ScreenIntelligenceRouteKind;
pub type ScreenIntelligenceSourceKind = policy::ScreenIntelligenceSourceKind;
pub type ScreenIntelligenceRouteDecision = route::ScreenIntelligenceRouteDecision;
pub type ScreenIntelligenceRouteRequest = route::ScreenIntelligenceRouteRequest;

pub fn screen_managed_browser_structured_extraction_is_consistent(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    crate::screen_intelligence_router_logic::screen_managed_browser_structured_extraction_is_consistent(value)
}

pub fn screen_intelligence_route_request_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    crate::screen_intelligence_router_logic::screen_intelligence_route_request_is_consistent(value)
}

pub fn screen_intelligence_route_decision_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    crate::screen_intelligence_router_logic::screen_intelligence_route_decision_is_consistent(value)
}

pub fn plan_screen_intelligence_route(
    request: &ScreenIntelligenceRouteRequest,
    route_id: impl Into<String>,
) -> ScreenIntelligenceRouteDecision {
    crate::screen_intelligence_router_logic::plan_screen_intelligence_route(request, route_id)
}

const SCREEN_INTELLIGENCE_ROUTER_GENERATED_TYPESCRIPT: &str = r#"/* generated from crates/screen-ai-core/src/screen_intelligence_router.rs */

export function planScreenIntelligenceRouteStub() {
  return 'planScreenIntelligenceRouteGenerated';
}
"#;

pub fn screen_intelligence_router_generated_typescript() -> String {
    SCREEN_INTELLIGENCE_ROUTER_GENERATED_TYPESCRIPT.to_string()
}
