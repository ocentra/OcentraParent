import { describe, expect, it } from 'vitest';
import { ActivityEvidenceKind } from '../src/kinds';
import {
  planScreenIntelligenceRoute,
  ScreenIntelligenceExistingEvidenceSchema,
  ScreenIntelligenceRouteDecisionSchema,
  ScreenIntelligenceRouterSchemaVersion,
} from '../src/screen-evidence';

const BrowserStructuredEvidence = {
  evidenceRef: {
    evidenceId: 'managed-browser-structured-evidence-1',
    kind: ActivityEvidenceKind.JournalEntry,
    digest: 'sha256:managed-browser-structured-evidence',
    uri: null,
  },
  evidenceKind: 'managedBrowserStructured',
  observedAt: '2026-06-05T03:20:00Z',
  category: 'school',
  confidence: 0.93,
  canAnswerPolicyQuestion: true,
  privacySafeForPolicy: true,
  rawScreenshotEvidence: false,
} as const;

const BaseRequest = {
  schemaVersion: ScreenIntelligenceRouterSchemaVersion,
  routeRequestId: 'screen-route-request-1',
  requestedAt: '2026-06-05T03:20:10Z',
  policyQuestion: 'categoryReview',
  surfaceKind: 'managedBrowser',
  capabilityStatus: 'ready',
  parentScreenAnalysisEnabled: true,
  captureAllowedByParent: true,
  allowedCaptureScope: 'managedBrowserWindow',
  managedBrowserStructuredExtractionAvailable: true,
  managedBrowserStructuredExtractionAttempted: false,
  sensitivityFlags: ['lowSensitivity'],
  availableEvidence: [],
  routeReason: 'A managed browser page changed and must be checked before screenshot capture.',
} as const;

describe('screen intelligence router contracts', () => {
  specifyExistingEvidenceRoute();
  specifyManagedBrowserStructuredRoute();
  specifyManagedBrowserScreenshotRejection();
  specifyNativeGameRoute();
  specifyDisabledAndSensitiveRoutes();
  specifyUnsafeInputRejections();
});

function specifyExistingEvidenceRoute() {
  it('selects no-screen-needed when existing structured evidence answers the policy question', () => {
    const decision = planScreenIntelligenceRoute({
      decisionId: 'screen-route-decision-no-screen-needed',
      decidedAt: '2026-06-05T03:20:20Z',
      request: {
        ...BaseRequest,
        routeRequestId: 'screen-route-request-no-screen-needed',
        managedBrowserStructuredExtractionAvailable: false,
        managedBrowserStructuredExtractionAttempted: true,
        availableEvidence: [BrowserStructuredEvidence],
      },
    });

    expect(decision.selectedRoute).toBe('noScreenNeeded');
    expect(decision.nextStep).toBe('deterministicSummary');
    expect(decision.screenshotQueued).toBe(false);
    expect(decision.sourceEvidenceRefs).toHaveLength(1);
    expect(decision.checkedEvidenceKinds).toContain('managedBrowserStructured');
  });
}

function specifyManagedBrowserStructuredRoute() {
  it('routes managed browser surfaces to structured extraction before screenshot capture', () => {
    const decision = planScreenIntelligenceRoute({
      decisionId: 'screen-route-decision-structured-first',
      decidedAt: '2026-06-05T03:21:20Z',
      request: {
        ...BaseRequest,
        routeRequestId: 'screen-route-request-structured-first',
      },
    });

    expect(decision.selectedRoute).toBe('managedBrowserStructuredExtraction');
    expect(decision.nextStep).toBe('structuredExtraction');
    expect(decision.structuredExtractionAttemptedBeforeScreenshot).toBe(true);
    expect(decision.screenshotQueued).toBe(false);
  });
}

function specifyManagedBrowserScreenshotRejection() {
  it('rejects managed browser screenshot routing before structured extraction is exhausted', () => {
    const unsafeDecision = ScreenIntelligenceRouteDecisionSchema.safeParse({
      schemaVersion: 1,
      decisionId: 'screen-route-decision-unsafe-browser-screenshot',
      requestId: 'screen-route-request-unsafe-browser-screenshot',
      decidedAt: '2026-06-05T03:22:20Z',
      selectedRoute: 'managedBrowserScreenshot',
      nextStep: 'encryptedImageQueue',
      reason: 'managedBrowserStructuredExhausted',
      policyQuestion: 'categoryReview',
      surfaceKind: 'managedBrowser',
      existingEvidenceChecked: true,
      checkedEvidenceKinds: ['managedBrowserStructured'],
      sourceEvidenceRefs: [],
      structuredExtractionAttemptedBeforeScreenshot: false,
      screenshotQueued: true,
      captureScope: 'managedBrowserWindow',
      rawScreenshotRetainedByDefault: false,
      remoteRawScreenshotUploadAllowed: false,
      parentVisibleSummary: 'Unsafe route queues screenshot before structured extraction.',
      sensitivityFlags: ['lowSensitivity'],
      degradedStates: [],
    });

    expect(unsafeDecision.success).toBe(false);
  });
}

function specifyNativeGameRoute() {
  it('routes native game surfaces to active-window capture only after existing game evidence is checked', () => {
    const decision = planScreenIntelligenceRoute({
      decisionId: 'screen-route-decision-native-game',
      decidedAt: '2026-06-05T03:23:20Z',
      request: {
        ...BaseRequest,
        routeRequestId: 'screen-route-request-native-game',
        surfaceKind: 'nativeGame',
        allowedCaptureScope: 'activeWindow',
        managedBrowserStructuredExtractionAvailable: false,
        managedBrowserStructuredExtractionAttempted: false,
      },
    });

    expect(decision.selectedRoute).toBe('nativeActiveWindowCapture');
    expect(decision.nextStep).toBe('encryptedImageQueue');
    expect(decision.checkedEvidenceKinds).toContain('gameForeground');
    expect(decision.screenshotQueued).toBe(true);
    expect(decision.captureScope).toBe('activeWindow');
  });
}

function specifyDisabledAndSensitiveRoutes() {
  it('degrades disabled or sensitive surfaces without queueing screenshots', () => {
    const disabledDecision = planScreenIntelligenceRoute({
      decisionId: 'screen-route-decision-disabled',
      decidedAt: '2026-06-05T03:24:20Z',
      request: {
        ...BaseRequest,
        routeRequestId: 'screen-route-request-disabled',
        parentScreenAnalysisEnabled: false,
        captureAllowedByParent: false,
      },
    });
    const protectedDecision = planScreenIntelligenceRoute({
      decisionId: 'screen-route-decision-protected',
      decidedAt: '2026-06-05T03:25:20Z',
      request: {
        ...BaseRequest,
        routeRequestId: 'screen-route-request-protected',
        sensitivityFlags: ['protectedSurfaceLikely'],
      },
    });

    expect(disabledDecision.selectedRoute).toBe('manualRequired');
    expect(disabledDecision.screenshotQueued).toBe(false);
    expect(disabledDecision.degradedStates).toContain('captureDisabled');
    expect(protectedDecision.selectedRoute).toBe('manualRequired');
    expect(protectedDecision.screenshotQueued).toBe(false);
    expect(protectedDecision.degradedStates).toContain('protectedSurface');
  });
}

function specifyUnsafeInputRejections() {
  it('rejects raw screenshot evidence and route decisions that skip existing evidence checks', () => {
    const rawScreenshotEvidence = ScreenIntelligenceExistingEvidenceSchema.safeParse({
      ...BrowserStructuredEvidence,
      rawScreenshotEvidence: true,
    });
    const noEvidenceCheckDecision = ScreenIntelligenceRouteDecisionSchema.safeParse({
      schemaVersion: 1,
      decisionId: 'screen-route-decision-no-check',
      requestId: 'screen-route-request-no-check',
      decidedAt: '2026-06-05T03:26:20Z',
      selectedRoute: 'nativeActiveWindowCapture',
      nextStep: 'encryptedImageQueue',
      reason: 'nativeSurfaceCaptureAllowed',
      policyQuestion: 'categoryReview',
      surfaceKind: 'nativeApp',
      existingEvidenceChecked: true,
      checkedEvidenceKinds: [],
      sourceEvidenceRefs: [],
      structuredExtractionAttemptedBeforeScreenshot: false,
      screenshotQueued: true,
      captureScope: 'activeWindow',
      rawScreenshotRetainedByDefault: false,
      remoteRawScreenshotUploadAllowed: false,
      parentVisibleSummary: 'Unsafe route did not record existing evidence checks.',
      sensitivityFlags: ['lowSensitivity'],
      degradedStates: [],
    });

    expect(rawScreenshotEvidence.success).toBe(false);
    expect(noEvidenceCheckDecision.success).toBe(false);
  });
}
