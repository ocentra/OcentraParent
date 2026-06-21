import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  ScreenIntelligenceRouteDecisionSchema,
  ScreenIntelligenceRouteRequestSchema,
  type ScreenIntelligenceRouteDecision,
  type ScreenIntelligenceRouteRequest,
} from '@ocentra-parent/schema-domain/screen-intelligence-router';
import {
  ScreenIntelligenceRouteIdSchema,
  ScreenIntelligenceRouteKindSchema,
  ScreenIntelligenceRouterSchemaVersion,
  ScreenStructuredExtractionIdSchema,
} from '@ocentra-parent/schema-domain/screen-intelligence-router-values';
import { ScreenCaptureScopeSchema } from '@ocentra-parent/schema-domain/screen-evidence-states';

export function planScreenIntelligenceRoute(
  request: ScreenIntelligenceRouteRequest,
  routeId: Infer<typeof ScreenIntelligenceRouteIdSchema>
): ScreenIntelligenceRouteDecision {
  const parsed = ScreenIntelligenceRouteRequestSchema.parse(request);
  const routeKind = routeKindFor(parsed);
  return buildDecision(
    parsed,
    routeId,
    routeKind,
    captureScopeForRoute(parsed, routeKind),
    structuredExtractionForRoute(parsed)
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
  return ScreenIntelligenceRouteDecisionSchema.parse({
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
  });
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
