pub(crate) const SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION: u16 = 1;
pub(crate) const SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT: usize = 480;
pub(crate) use extraction::{
    MANAGED_BROWSER_STRUCTURED_SOURCE_ID, MANAGED_BROWSER_TARGET_REF_PREFIX,
    MANAGED_BROWSER_TITLE_REF_PREFIX, MANAGED_BROWSER_URL_REF_PREFIX,
};

pub(crate) const MANUAL_REQUIRED_PARENT_DISABLED: &str =
    "parent setting requires manual review before screen capture";
pub(crate) const MANUAL_REQUIRED_UNSUPPORTED_SCOPE: &str =
    "no allowed active-window or selected-window capture scope is available";
pub(crate) const UNAVAILABLE_PROTECTED_SURFACE: &str =
    "protected surface is not eligible for screen capture or model analysis";
pub(crate) const UNAVAILABLE_CREDENTIAL_PROMPT: &str =
    "credential prompt risk is not eligible for screen capture or model analysis";

mod capture;
mod extraction;
mod policy;
mod route;

pub type ScreenCaptureScope = capture::ScreenCaptureScope;
pub type ScreenEvidenceCustodyState = capture::ScreenEvidenceCustodyState;
pub type ActivityEvidenceRef = extraction::ActivityEvidenceRef;
pub type ScreenManagedBrowserStructuredExtraction =
    extraction::ScreenManagedBrowserStructuredExtraction;
pub type ScreenStructuredExtractionRedactionState =
    extraction::ScreenStructuredExtractionRedactionState;
pub type ScreenStructuredExtractionState = extraction::ScreenStructuredExtractionState;
pub type ScreenStructuredExtractionAuthority = extraction::ScreenStructuredExtractionAuthority;
pub type ScreenStructuredExtractionFreshness = extraction::ScreenStructuredExtractionFreshness;
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
