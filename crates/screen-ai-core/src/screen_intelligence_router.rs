use serde::{Deserialize, Serialize};

pub const SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION: u16 = 1;
pub const SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT: usize = 480;

const MANUAL_REQUIRED_PARENT_DISABLED: &str =
    "parent setting requires manual review before screen capture";
const MANUAL_REQUIRED_UNSUPPORTED_SCOPE: &str =
    "no allowed active-window or selected-window capture scope is available";
const UNAVAILABLE_PROTECTED_SURFACE: &str =
    "protected surface is not eligible for screen capture or model analysis";
const UNAVAILABLE_CREDENTIAL_PROMPT: &str =
    "credential prompt risk is not eligible for screen capture or model analysis";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenIntelligenceSourceKind {
    ManagedBrowser,
    NativeApp,
    NativeGame,
    Launcher,
    UnknownProcess,
    NetworkOrSessionSummary,
    ScreenAdjacentEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenIntelligenceRouteKind {
    NoScreenNeeded,
    ManagedBrowserStructuredExtraction,
    ScreenCaptureActiveWindow,
    ScreenCaptureSelectedWindow,
    ManualRequired,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenIntelligencePolicySensitivity {
    Ordinary,
    Private,
    CredentialRisk,
    ProtectedSurface,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenStructuredExtractionState {
    EnoughForPolicy,
    NeedsScreenshot,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenCaptureScope {
    FullScreen,
    PrimaryDisplay,
    ActiveDisplay,
    SelectedWindow,
    ActiveWindow,
    ManagedBrowserWindow,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScreenEvidenceCustodyState {
    LiveLocalChildAgent,
    LiveLanChildAgent,
    ChildDeviceTempQueue,
    ChildDeviceJournal,
    ChildDeviceQueryStore,
    ParentDeviceCache,
    ParentOwnedExport,
    OcentraHostedNonActivity,
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
    pub policy_question: String,
    pub policy_sensitivity: ScreenIntelligencePolicySensitivity,
    pub evidence_refs: Vec<ActivityEvidenceRef>,
    pub custody_state: ScreenEvidenceCustodyState,
    pub manual_required_reason: Option<String>,
    pub unavailable_reason: Option<String>,
    pub remote_ai_allowed: bool,
    pub raw_screenshot_retained: bool,
}

pub fn screen_managed_browser_structured_extraction_is_consistent(
    value: &ScreenManagedBrowserStructuredExtraction,
) -> bool {
    if value.raw_dom_included
        || value.visible_text_character_count > SCREEN_MANAGED_BROWSER_STRUCTURED_TEXT_LIMIT
        || value.custody_state == ScreenEvidenceCustodyState::OcentraHostedNonActivity
    {
        return false;
    }
    if value.extraction_state == ScreenStructuredExtractionState::EnoughForPolicy {
        return value.enough_for_policy
            && value.policy_question_answered
            && value.no_screen_needed
            && !value.screenshot_required
            && value.category_candidate.is_some();
    }
    !value.no_screen_needed
        && value.screenshot_required
            == (value.extraction_state == ScreenStructuredExtractionState::NeedsScreenshot)
}

pub fn screen_intelligence_route_request_is_consistent(
    value: &ScreenIntelligenceRouteRequest,
) -> bool {
    if value
        .allowed_capture_scopes
        .contains(&ScreenCaptureScope::FullScreen)
    {
        return false;
    }

    match &value.structured_extraction {
        Some(extraction) => {
            value.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
                && screen_managed_browser_structured_extraction_is_consistent(extraction)
        }
        None => true,
    }
}

pub fn screen_intelligence_route_decision_is_consistent(
    value: &ScreenIntelligenceRouteDecision,
) -> bool {
    if !value.checked_existing_evidence_first
        || value.remote_ai_allowed
        || value.raw_screenshot_retained
    {
        return false;
    }

    match value.route_kind {
        ScreenIntelligenceRouteKind::NoScreenNeeded => {
            value.screenshot_skipped
                && value.capture_scope.is_none()
                && value.structured_extraction_id.is_some()
        }
        ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction => {
            value.screenshot_skipped
                && value.managed_browser_structured_extraction_first
                && value.capture_scope.is_none()
        }
        ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow => {
            !value.screenshot_skipped
                && value.capture_scope == Some(ScreenCaptureScope::ActiveWindow)
        }
        ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow => {
            !value.screenshot_skipped
                && value.capture_scope == Some(ScreenCaptureScope::SelectedWindow)
        }
        ScreenIntelligenceRouteKind::ManualRequired => {
            value.screenshot_skipped
                && value.capture_scope.is_none()
                && value.manual_required_reason.is_some()
        }
        ScreenIntelligenceRouteKind::Unavailable => {
            value.screenshot_skipped
                && value.capture_scope.is_none()
                && value.unavailable_reason.is_some()
        }
    }
}

pub fn plan_screen_intelligence_route(
    request: &ScreenIntelligenceRouteRequest,
    route_id: impl Into<String>,
) -> ScreenIntelligenceRouteDecision {
    let route_kind = route_kind_for(request);
    let capture_scope = capture_scope_for_route(request, &route_kind);
    let structured_extraction_id = structured_extraction_for_route(request);

    ScreenIntelligenceRouteDecision {
        schema_version: SCREEN_INTELLIGENCE_ROUTER_SCHEMA_VERSION,
        route_id: route_id.into(),
        request_id: request.request_id.clone(),
        decided_at: request.requested_at.clone(),
        source_kind: request.source_kind.clone(),
        route_kind: route_kind.clone(),
        capture_scope: capture_scope.clone(),
        structured_extraction_id,
        screenshot_skipped: capture_scope.is_none(),
        checked_existing_evidence_first: true,
        managed_browser_structured_extraction_first: request.source_kind
            == ScreenIntelligenceSourceKind::ManagedBrowser
            && matches!(
                route_kind,
                ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction
                    | ScreenIntelligenceRouteKind::NoScreenNeeded
            ),
        policy_question: request.policy_question.clone(),
        policy_sensitivity: request.policy_sensitivity.clone(),
        evidence_refs: request
            .structured_extraction
            .as_ref()
            .map(|value| value.evidence_refs.clone())
            .unwrap_or_else(|| request.existing_evidence_refs.clone()),
        custody_state: request
            .structured_extraction
            .as_ref()
            .map(|value| value.custody_state.clone())
            .unwrap_or(ScreenEvidenceCustodyState::ChildDeviceQueryStore),
        manual_required_reason: if route_kind == ScreenIntelligenceRouteKind::ManualRequired {
            Some(manual_reason_for(request).to_string())
        } else {
            None
        },
        unavailable_reason: if route_kind == ScreenIntelligenceRouteKind::Unavailable {
            Some(unavailable_reason_for(request).to_string())
        } else {
            None
        },
        remote_ai_allowed: false,
        raw_screenshot_retained: false,
    }
}

fn screen_capture_is_unsafe(request: &ScreenIntelligenceRouteRequest) -> bool {
    request.protected_surface_suspected
        || request.credential_prompt_suspected
        || request.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface
        || request.policy_sensitivity == ScreenIntelligencePolicySensitivity::CredentialRisk
}

fn route_kind_for(request: &ScreenIntelligenceRouteRequest) -> ScreenIntelligenceRouteKind {
    if screen_capture_is_unsafe(request) {
        return ScreenIntelligenceRouteKind::Unavailable;
    }
    if request
        .structured_extraction
        .as_ref()
        .is_some_and(|value| value.no_screen_needed)
    {
        return ScreenIntelligenceRouteKind::NoScreenNeeded;
    }
    if request.source_kind == ScreenIntelligenceSourceKind::ManagedBrowser
        && request.parent_allows_managed_browser_structured_extraction
    {
        return ScreenIntelligenceRouteKind::ManagedBrowserStructuredExtraction;
    }
    if !request.parent_allows_screen_capture {
        return ScreenIntelligenceRouteKind::ManualRequired;
    }
    capture_route_kind_for(preferred_capture_scope(&request.allowed_capture_scopes))
}

fn capture_route_kind_for(
    capture_scope: Option<&ScreenCaptureScope>,
) -> ScreenIntelligenceRouteKind {
    match capture_scope {
        Some(ScreenCaptureScope::ActiveWindow) => {
            ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow
        }
        Some(ScreenCaptureScope::SelectedWindow) => {
            ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow
        }
        _ => ScreenIntelligenceRouteKind::ManualRequired,
    }
}

fn preferred_capture_scope(scopes: &[ScreenCaptureScope]) -> Option<&ScreenCaptureScope> {
    scopes
        .iter()
        .find(|scope| **scope == ScreenCaptureScope::ActiveWindow)
        .or_else(|| {
            scopes
                .iter()
                .find(|scope| **scope == ScreenCaptureScope::SelectedWindow)
        })
}

fn capture_scope_for_route(
    request: &ScreenIntelligenceRouteRequest,
    route_kind: &ScreenIntelligenceRouteKind,
) -> Option<ScreenCaptureScope> {
    match route_kind {
        ScreenIntelligenceRouteKind::ScreenCaptureActiveWindow
        | ScreenIntelligenceRouteKind::ScreenCaptureSelectedWindow => {
            preferred_capture_scope(&request.allowed_capture_scopes).cloned()
        }
        _ => None,
    }
}

fn structured_extraction_for_route(request: &ScreenIntelligenceRouteRequest) -> Option<String> {
    request
        .structured_extraction
        .as_ref()
        .map(|value| value.extraction_id.clone())
}

fn manual_reason_for(request: &ScreenIntelligenceRouteRequest) -> &'static str {
    if !request.parent_allows_screen_capture {
        MANUAL_REQUIRED_PARENT_DISABLED
    } else {
        MANUAL_REQUIRED_UNSUPPORTED_SCOPE
    }
}

fn unavailable_reason_for(request: &ScreenIntelligenceRouteRequest) -> &'static str {
    if request.protected_surface_suspected
        || request.policy_sensitivity == ScreenIntelligencePolicySensitivity::ProtectedSurface
    {
        UNAVAILABLE_PROTECTED_SURFACE
    } else {
        UNAVAILABLE_CREDENTIAL_PROMPT
    }
}

const SCREEN_INTELLIGENCE_ROUTER_GENERATED_TYPESCRIPT: &str = r#"/* generated from crates/screen-ai-core/src/screen_intelligence_router.rs */

import { type Infer } from '@ocentra-parent/schema-domain/effect';
import type { ScreenIntelligenceRouteRequest } from '@ocentra-parent/schema-domain/screen-intelligence-router';
import {
  ScreenIntelligenceRouterSchemaVersion,
  type ScreenIntelligenceRouteIdSchema,
  type ScreenIntelligenceRouteKindSchema,
  type ScreenStructuredExtractionIdSchema,
} from '@ocentra-parent/schema-domain/screen-intelligence-router-values';
import type { ScreenCaptureScopeSchema } from '@ocentra-parent/schema-domain/screen-evidence-states';

export function planScreenIntelligenceRouteGenerated(
  request: ScreenIntelligenceRouteRequest,
  routeId: Infer<typeof ScreenIntelligenceRouteIdSchema>
) {
  const routeKind = routeKindFor(request);
  return buildDecision(
    request,
    routeId,
    routeKind,
    captureScopeForRoute(request, routeKind),
    structuredExtractionForRoute(request)
  );
}

function screenCaptureIsUnsafe(request: ScreenIntelligenceRouteRequest): boolean {
  return (
    request.protectedSurfaceSuspected ||
    request.credentialPromptSuspected ||
    request.policySensitivity === 'protectedSurface' ||
    request.policySensitivity === 'credentialRisk'
  );
}

function routeKindFor(request: ScreenIntelligenceRouteRequest): Infer<typeof ScreenIntelligenceRouteKindSchema> {
  if (screenCaptureIsUnsafe(request)) {
    return 'unavailable';
  }
  if (request.structuredExtraction?.noScreenNeeded) {
    return 'noScreenNeeded';
  }
  if (request.sourceKind === 'managedBrowser' && request.parentAllowsManagedBrowserStructuredExtraction) {
    return 'managedBrowserStructuredExtraction';
  }
  if (!request.parentAllowsScreenCapture) {
    return 'manualRequired';
  }
  return captureRouteKindFor(preferredCaptureScope(request.allowedCaptureScopes));
}

function captureRouteKindFor(
  captureScope: Infer<typeof ScreenCaptureScopeSchema> | null
): Infer<typeof ScreenIntelligenceRouteKindSchema> {
  if (captureScope === 'activeWindow') {
    return 'screenCaptureActiveWindow';
  }
  if (captureScope === 'selectedWindow') {
    return 'screenCaptureSelectedWindow';
  }
  return 'manualRequired';
}

function preferredCaptureScope(scopes: readonly Infer<typeof ScreenCaptureScopeSchema>[]) {
  if (scopes.includes('activeWindow')) {
    return 'activeWindow' as const;
  }
  if (scopes.includes('selectedWindow')) {
    return 'selectedWindow' as const;
  }
  return null;
}

function captureScopeForRoute(
  request: ScreenIntelligenceRouteRequest,
  routeKind: Infer<typeof ScreenIntelligenceRouteKindSchema>
) {
  if (routeKind === 'screenCaptureActiveWindow' || routeKind === 'screenCaptureSelectedWindow') {
    return preferredCaptureScope(request.allowedCaptureScopes);
  }
  return null;
}

function structuredExtractionForRoute(request: ScreenIntelligenceRouteRequest) {
  return request.structuredExtraction?.extractionId ?? null;
}

function buildDecision(
  request: ScreenIntelligenceRouteRequest,
  routeId: Infer<typeof ScreenIntelligenceRouteIdSchema>,
  routeKind: Infer<typeof ScreenIntelligenceRouteKindSchema>,
  captureScope: Infer<typeof ScreenCaptureScopeSchema> | null,
  structuredExtractionId: Infer<typeof ScreenStructuredExtractionIdSchema> | null
) {
  return {
    schemaVersion: ScreenIntelligenceRouterSchemaVersion,
    routeId,
    requestId: request.requestId,
    decidedAt: request.requestedAt,
    sourceKind: request.sourceKind,
    routeKind,
    captureScope,
    structuredExtractionId,
    screenshotSkipped: captureScope === null,
    checkedExistingEvidenceFirst: true,
    managedBrowserStructuredExtractionFirst:
      request.sourceKind === 'managedBrowser' &&
      (routeKind === 'managedBrowserStructuredExtraction' || routeKind === 'noScreenNeeded'),
    policyQuestion: request.policyQuestion,
    policySensitivity: request.policySensitivity,
    evidenceRefs: request.structuredExtraction?.evidenceRefs ?? request.existingEvidenceRefs,
    custodyState: request.structuredExtraction?.custodyState ?? 'child-device-query-store',
    manualRequiredReason: routeKind === 'manualRequired' ? manualReasonFor(request) : null,
    unavailableReason: routeKind === 'unavailable' ? unavailableReasonFor(request) : null,
    remoteAiAllowed: false,
    rawScreenshotRetained: false,
  };
}

function manualReasonFor(request: ScreenIntelligenceRouteRequest) {
  if (!request.parentAllowsScreenCapture) {
    return 'parent setting requires manual review before screen capture';
  }
  return 'no allowed active-window or selected-window capture scope is available';
}

function unavailableReasonFor(request: ScreenIntelligenceRouteRequest) {
  if (request.protectedSurfaceSuspected || request.policySensitivity === 'protectedSurface') {
    return 'protected surface is not eligible for screen capture or model analysis';
  }
  return 'credential prompt risk is not eligible for screen capture or model analysis';
}
"#;

pub fn screen_intelligence_router_generated_typescript() -> String {
    SCREEN_INTELLIGENCE_ROUTER_GENERATED_TYPESCRIPT.to_string()
}
