import { ScreenEvidenceSchemaVersion } from './screen-evidence-primitives';
import {
  ScreenIntelligenceRouteDecisionSchema,
  ScreenIntelligenceRoutePlanRequestSchema,
  type ScreenIntelligenceRouteDecision,
  type ScreenIntelligenceRoutePlanRequest,
  type ScreenIntelligenceRouteRequest,
} from './screen-intelligence-router-schemas';
import {
  type ScreenIntelligenceEvidenceKind,
  type ScreenIntelligenceRouteSelection,
} from './screen-intelligence-router-values';

export * from './screen-intelligence-router-schemas';

export function planScreenIntelligenceRoute(
  request: ScreenIntelligenceRoutePlanRequest
): ScreenIntelligenceRouteDecision {
  const parsed = ScreenIntelligenceRoutePlanRequestSchema.parse(request);
  const decision = buildRouteDecision(parsed.request);

  return ScreenIntelligenceRouteDecisionSchema.parse({
    schemaVersion: ScreenEvidenceSchemaVersion,
    decisionId: parsed.decisionId,
    requestId: parsed.request.routeRequestId,
    decidedAt: parsed.decidedAt,
    policyQuestion: parsed.request.policyQuestion,
    surfaceKind: parsed.request.surfaceKind,
    existingEvidenceChecked: true,
    checkedEvidenceKinds: checkedEvidenceKinds(parsed.request),
    sourceEvidenceRefs: evidenceRefsFor(parsed.request),
    rawScreenshotRetainedByDefault: false,
    remoteRawScreenshotUploadAllowed: false,
    parentVisibleSummary: summaryFor(decision.selectedRoute),
    sensitivityFlags: parsed.request.sensitivityFlags,
    ...decision,
  });
}

function buildRouteDecision(
  request: ScreenIntelligenceRouteRequest
): Pick<
  ScreenIntelligenceRouteDecision,
  | 'selectedRoute'
  | 'nextStep'
  | 'reason'
  | 'structuredExtractionAttemptedBeforeScreenshot'
  | 'screenshotQueued'
  | 'captureScope'
  | 'degradedStates'
> {
  const degradedDecision = preCaptureDegradedDecision(request);
  const existingEvidenceDecision = noScreenNeededDecision(request);
  const managedBrowserDecision = managedBrowserRouteDecision(request);
  const evidenceOnlyDecision = evidenceOnlyRouteDecision(request);

  if (degradedDecision !== null) {
    return degradedDecision;
  }
  if (existingEvidenceDecision !== null) {
    return existingEvidenceDecision;
  }
  if (managedBrowserDecision !== null) {
    return managedBrowserDecision;
  }
  if (evidenceOnlyDecision !== null) {
    return evidenceOnlyDecision;
  }

  if (request.allowedCaptureScope === 'selectedWindow') {
    return captureDecision('selectedWindowCapture', 'selectedWindowCaptureAllowed', 'selectedWindow');
  }

  return captureDecision('nativeActiveWindowCapture', 'nativeSurfaceCaptureAllowed', 'activeWindow');
}

function preCaptureDegradedDecision(request: ScreenIntelligenceRouteRequest) {
  if (!request.parentScreenAnalysisEnabled || !request.captureAllowedByParent) {
    return manualDecision('captureDisabledByParent', 'captureDisabled');
  }
  if (request.capabilityStatus !== 'ready') {
    return {
      ...manualDecision('capabilityUnavailable', 'capabilityUnavailable'),
      selectedRoute: 'unavailable' as const,
      nextStep: 'unavailable' as const,
    };
  }
  if (sensitivityRequiresManualReview(request)) {
    return manualDecision('protectedOrSensitiveSurface', 'protectedSurface');
  }

  return null;
}

function noScreenNeededDecision(request: ScreenIntelligenceRouteRequest) {
  if (!request.availableEvidence.some((evidence) => evidence.canAnswerPolicyQuestion)) {
    return null;
  }

  return {
    selectedRoute: 'noScreenNeeded' as const,
    nextStep: 'deterministicSummary' as const,
    reason: 'existingEvidenceAnswered' as const,
    structuredExtractionAttemptedBeforeScreenshot: request.managedBrowserStructuredExtractionAttempted,
    screenshotQueued: false,
    captureScope: null,
    degradedStates: [],
  };
}

function managedBrowserRouteDecision(request: ScreenIntelligenceRouteRequest) {
  if (request.surfaceKind !== 'managedBrowser') {
    return null;
  }
  if (request.managedBrowserStructuredExtractionAvailable) {
    return {
      selectedRoute: 'managedBrowserStructuredExtraction' as const,
      nextStep: 'structuredExtraction' as const,
      reason: 'managedBrowserStructuredFirst' as const,
      structuredExtractionAttemptedBeforeScreenshot: true,
      screenshotQueued: false,
      captureScope: null,
      degradedStates: ['structuredEvidenceMissing' as const],
    };
  }
  if (request.managedBrowserStructuredExtractionAttempted) {
    return {
      selectedRoute: 'managedBrowserScreenshot' as const,
      nextStep: 'encryptedImageQueue' as const,
      reason: 'managedBrowserStructuredExhausted' as const,
      structuredExtractionAttemptedBeforeScreenshot: true,
      screenshotQueued: true,
      captureScope: 'managedBrowserWindow' as const,
      degradedStates: [],
    };
  }

  return null;
}

function evidenceOnlyRouteDecision(request: ScreenIntelligenceRouteRequest) {
  if (request.surfaceKind !== 'networkOnly' && request.surfaceKind !== 'sessionOnly') {
    return null;
  }

  return {
    selectedRoute: 'manualRequired' as const,
    nextStep: 'manualReviewRequired' as const,
    reason: 'capabilityUnavailable' as const,
    structuredExtractionAttemptedBeforeScreenshot: false,
    screenshotQueued: false,
    captureScope: null,
    degradedStates: ['structuredEvidenceMissing' as const, 'manualReviewRequired' as const],
  };
}

function captureDecision(
  selectedRoute: Extract<ScreenIntelligenceRouteSelection, 'nativeActiveWindowCapture' | 'selectedWindowCapture'>,
  reason: 'nativeSurfaceCaptureAllowed' | 'selectedWindowCaptureAllowed',
  captureScope: 'activeWindow' | 'selectedWindow'
) {
  return {
    selectedRoute,
    nextStep: 'encryptedImageQueue' as const,
    reason,
    structuredExtractionAttemptedBeforeScreenshot: false,
    screenshotQueued: true,
    captureScope,
    degradedStates: [],
  };
}

function manualDecision(
  reason: 'captureDisabledByParent' | 'protectedOrSensitiveSurface' | 'capabilityUnavailable',
  degradedState: 'captureDisabled' | 'protectedSurface' | 'capabilityUnavailable'
) {
  return {
    selectedRoute: 'manualRequired' as const,
    nextStep: 'manualReviewRequired' as const,
    reason,
    structuredExtractionAttemptedBeforeScreenshot: false,
    screenshotQueued: false,
    captureScope: null,
    degradedStates: [degradedState, 'manualReviewRequired' as const],
  };
}

function checkedEvidenceKinds(request: ScreenIntelligenceRouteRequest): ScreenIntelligenceEvidenceKind[] {
  const kinds = new Set<ScreenIntelligenceEvidenceKind>(
    request.availableEvidence.map((evidence) => evidence.evidenceKind)
  );

  if (request.surfaceKind === 'managedBrowser') {
    kinds.add('managedBrowserStructured');
  }
  if (
    request.surfaceKind === 'nativeApp' ||
    request.surfaceKind === 'launcher' ||
    request.surfaceKind === 'unknownProcess'
  ) {
    kinds.add('appForeground');
  }
  if (request.surfaceKind === 'nativeGame') {
    kinds.add('gameForeground');
  }
  if (request.surfaceKind === 'networkOnly') {
    kinds.add('networkDigest');
  }
  if (request.surfaceKind === 'sessionOnly') {
    kinds.add('sessionState');
  }

  return Array.from(kinds);
}

function evidenceRefsFor(request: ScreenIntelligenceRouteRequest) {
  return request.availableEvidence
    .filter((evidence) => evidence.privacySafeForPolicy)
    .map((evidence) => evidence.evidenceRef);
}

function sensitivityRequiresManualReview(request: ScreenIntelligenceRouteRequest): boolean {
  return (
    request.sensitivityFlags.includes('protectedSurfaceLikely') ||
    request.sensitivityFlags.includes('credentialLikeText') ||
    request.sensitivityFlags.includes('privateMessageLikely')
  );
}

function summaryFor(selectedRoute: ScreenIntelligenceRouteSelection): string {
  if (selectedRoute === 'noScreenNeeded') {
    return 'Existing structured evidence answered the screen policy question without a new screenshot.';
  }
  if (selectedRoute === 'managedBrowserStructuredExtraction') {
    return 'Managed browser structured extraction must run before any managed browser screenshot.';
  }
  if (selectedRoute === 'managedBrowserScreenshot') {
    return 'Managed browser structured extraction was exhausted before queuing a scoped browser screenshot.';
  }
  if (selectedRoute === 'unavailable') {
    return 'Screen intelligence routing is unavailable for this surface.';
  }
  if (selectedRoute === 'manualRequired') {
    return 'Screen intelligence routing requires manual review and does not queue a screenshot.';
  }

  return 'Native surface capture may use the encrypted image queue after existing evidence is checked.';
}
