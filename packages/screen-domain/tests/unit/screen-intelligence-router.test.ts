import { describe, expect, it } from 'vitest';
import { planScreenIntelligenceRoute } from '../../src/screen-intelligence-router';
import {
  ScreenIntelligenceRouteDecisionSchema,
  ScreenManagedBrowserStructuredExtractionSchema,
} from '@ocentra-parent/schema-domain/screen-intelligence-router';
import {
  ScreenIntelligenceRouterSchemaVersion,
  ScreenManagedBrowserStructuredTextLimit,
} from '@ocentra-parent/schema-domain/screen-intelligence-router-values';

describe('screen intelligence router contracts', () => {
  it(
    'skips screenshots when managed browser structured evidence answers the policy question',
    skipsScreenshotsWhenStructuredEvidenceAnswersPolicyQuestion
  );
  it(
    'routes managed browser cases to structured extraction before any screenshot',
    routesManagedBrowserCasesStructuredFirst
  );
  it(
    'routes native game foreground cases to active-window capture only when parent scope allows it',
    routesNativeGameForegroundCasesToActiveWindowCapture
  );
  it(
    'returns manual-required when parent settings or supported scopes do not allow capture',
    returnsManualRequiredForDisabledOrUnsupportedCapture
  );
  it('returns unavailable for protected surfaces and credential prompt risk', returnsUnavailableForProtectedSurfaces);
  it(
    'rejects unsafe structured extraction and unsafe route decisions',
    rejectsUnsafeStructuredExtractionAndRouteDecisions
  );
});

function skipsScreenshotsWhenStructuredEvidenceAnswersPolicyQuestion() {
  const decision = planScreenIntelligenceRoute(
    routeRequest({
      structuredExtraction: enoughStructuredExtraction(),
    }),
    'screen-route-no-image-needed'
  );

  expect(decision.routeKind).toBe('noScreenNeeded');
  expect(decision.screenshotSkipped).toBe(true);
  expect(decision.captureScope).toBeNull();
  expect(decision.structuredExtractionId).toBe('managed-browser-structured-youtube-lesson');
  expect(decision.managedBrowserStructuredExtractionFirst).toBe(true);
  expect(decision.remoteAiAllowed).toBe(false);
  expect(decision.rawScreenshotRetained).toBe(false);
}

function routesManagedBrowserCasesStructuredFirst() {
  const decision = planScreenIntelligenceRoute(
    routeRequest({
      structuredExtraction: needsScreenshotStructuredExtraction(),
    }),
    'screen-route-structured-first'
  );

  expect(decision.routeKind).toBe('managedBrowserStructuredExtraction');
  expect(decision.screenshotSkipped).toBe(true);
  expect(decision.captureScope).toBeNull();
  expect(decision.managedBrowserStructuredExtractionFirst).toBe(true);
}

function routesNativeGameForegroundCasesToActiveWindowCapture() {
  const decision = planScreenIntelligenceRoute(
    routeRequest({
      sourceKind: 'nativeGame',
      structuredExtraction: null,
      existingEvidenceRefs: [evidenceRef('native-game-window-ref')],
      allowedCaptureScopes: ['activeWindow', 'selectedWindow'],
    }),
    'screen-route-native-game-active-window'
  );

  expect(decision.routeKind).toBe('screenCaptureActiveWindow');
  expect(decision.captureScope).toBe('activeWindow');
  expect(decision.screenshotSkipped).toBe(false);
  expect(decision.checkedExistingEvidenceFirst).toBe(true);
}

function returnsManualRequiredForDisabledOrUnsupportedCapture() {
  const disabled = planScreenIntelligenceRoute(
    routeRequest({
      sourceKind: 'launcher',
      structuredExtraction: null,
      parentAllowsScreenCapture: false,
    }),
    'screen-route-parent-disabled'
  );
  const unsupportedScope = planScreenIntelligenceRoute(
    routeRequest({
      sourceKind: 'unknownProcess',
      structuredExtraction: null,
      allowedCaptureScopes: ['managedBrowserWindow'],
    }),
    'screen-route-unsupported-scope'
  );

  expect(disabled.routeKind).toBe('manualRequired');
  expect(disabled.manualRequiredReason).toBe('parent setting requires manual review before screen capture');
  expect(unsupportedScope.routeKind).toBe('manualRequired');
  expect(unsupportedScope.manualRequiredReason).toBe(
    'no allowed active-window or selected-window capture scope is available'
  );
}

function returnsUnavailableForProtectedSurfaces() {
  const protectedSurface = planScreenIntelligenceRoute(
    routeRequest({
      policySensitivity: 'protectedSurface',
      protectedSurfaceSuspected: true,
    }),
    'screen-route-protected-surface'
  );
  const credentialPrompt = planScreenIntelligenceRoute(
    routeRequest({
      policySensitivity: 'credentialRisk',
      credentialPromptSuspected: true,
    }),
    'screen-route-credential-risk'
  );

  expect(protectedSurface.routeKind).toBe('unavailable');
  expect(protectedSurface.unavailableReason).toBe(
    'protected surface is not eligible for screen capture or model analysis'
  );
  expect(credentialPrompt.routeKind).toBe('unavailable');
  expect(credentialPrompt.unavailableReason).toBe(
    'credential prompt risk is not eligible for screen capture or model analysis'
  );
}

function rejectsUnsafeStructuredExtractionAndRouteDecisions() {
  const enough = enoughStructuredExtraction();
  const decision = planScreenIntelligenceRoute(
    routeRequest({
      structuredExtraction: enough,
    }),
    'screen-route-safe'
  );

  expect(
    ScreenManagedBrowserStructuredExtractionSchema.safeParse({
      ...enough,
      rawDomIncluded: true,
    }).success
  ).toBe(false);
  expect(
    ScreenManagedBrowserStructuredExtractionSchema.safeParse({
      ...enough,
      visibleTextCharacterCount: ScreenManagedBrowserStructuredTextLimit + 1,
    }).success
  ).toBe(false);
  expect(
    ScreenIntelligenceRouteDecisionSchema.safeParse({
      ...decision,
      remoteAiAllowed: true,
      rawScreenshotRetained: true,
    }).success
  ).toBe(false);
}

function routeRequest(overrides = {}) {
  return {
    schemaVersion: ScreenIntelligenceRouterSchemaVersion,
    requestId: 'screen-route-request-youtube-lesson',
    requestedAt: '2026-06-05T04:45:00.000Z',
    deviceRef: 'windows-child-device',
    sourceKind: 'managedBrowser',
    captureReason: 'managedBrowserUrlChange',
    policyQuestion: 'Can typed browser evidence answer before taking a screenshot?',
    policySensitivity: 'ordinary',
    existingEvidenceRefs: [evidenceRef('managed-browser-url-ref')],
    structuredExtraction: null,
    parentAllowsManagedBrowserStructuredExtraction: true,
    parentAllowsScreenCapture: true,
    allowedCaptureScopes: ['managedBrowserWindow', 'activeWindow'],
    protectedSurfaceSuspected: false,
    credentialPromptSuspected: false,
    ...overrides,
  };
}

function enoughStructuredExtraction() {
  return {
    schemaVersion: ScreenIntelligenceRouterSchemaVersion,
    extractionId: 'managed-browser-structured-youtube-lesson',
    capturedAt: '2026-06-05T04:44:58.000Z',
    evidenceRefs: [evidenceRef('managed-browser-url-ref'), evidenceRef('managed-browser-title-ref')],
    extractionState: 'enoughForPolicy',
    urlTitleMetadataCaptured: true,
    visibleTextSummary: 'YouTube lesson page with math title and education metadata only.',
    visibleTextCharacterCount: 64,
    domOverflowRedacted: false,
    privateContentRedacted: false,
    rawDomIncluded: false,
    redactionState: 'none',
    enoughForPolicy: true,
    policyQuestionAnswered: true,
    noScreenNeeded: true,
    screenshotRequired: false,
    categoryCandidate: 'school',
    riskSignals: [],
    confidence: 0.91,
    custodyState: 'child-device-query-store',
    reason: null,
  };
}

function needsScreenshotStructuredExtraction() {
  return {
    ...enoughStructuredExtraction(),
    extractionId: 'managed-browser-structured-ambiguous-video',
    extractionState: 'needsScreenshot',
    visibleTextSummary: 'Video page title is ambiguous after URL and title parsing.',
    visibleTextCharacterCount: 57,
    domOverflowRedacted: true,
    redactionState: 'overflowRedacted',
    enoughForPolicy: false,
    policyQuestionAnswered: false,
    noScreenNeeded: false,
    screenshotRequired: true,
    categoryCandidate: null,
    confidence: 0.42,
    reason: 'structured browser evidence is not enough to classify visible activity',
  };
}

function evidenceRef(evidenceId: string) {
  return {
    evidenceId,
    kind: 'local-db-row',
    digest: `${evidenceId}-digest`,
    uri: null,
  };
}
