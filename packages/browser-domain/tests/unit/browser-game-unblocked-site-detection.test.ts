import { describe, expect, it } from 'vitest';
import {
  BrowserGameUnblockedSiteDetectionSchema,
  type BrowserGameUnblockedSiteDetection,
  BrowserGameUnblockedSiteSignalSchema,
} from '@ocentra-parent/schema-domain/browser-game-unblocked-site-detection';

describe('browser-game unblocked site detection contracts', () => {
  it('accepts an evidence-backed managed unblocked game portal candidate', acceptsManagedUnblockedPortal);
  it('accepts unmanaged browser game use as bypass evidence only', acceptsUnmanagedBypassEvidenceOnly);
  it(
    'rejects raw URL/page/search, exact unmanaged URL, runtime, UI, native, cloud-frame, and enforcement claims',
    rejectsAuthorityClaims
  );
  it('rejects inconsistent action candidates and unknown candidate rows', rejectsInconsistentDetections);
});

function acceptsManagedUnblockedPortal() {
  const parsed = BrowserGameUnblockedSiteDetectionSchema.parse(detection());

  expect(parsed.schemaVersion).toBe('browser-game-unblocked-site-detection-contract');
  expect(parsed.classificationKind).toBe('unblocked-game-site');
  expect(parsed.actionCandidate).toBe('block-during-school-candidate');
  expect(parsed.deliveryState).toBe('contract-only');
  expect(parsed.runtimeGateExecutedClaimed).toBe(false);
  expect(parsed.enforcementClaimed).toBe(false);
}

function acceptsUnmanagedBypassEvidenceOnly() {
  const parsed = BrowserGameUnblockedSiteDetectionSchema.parse(
    detection({
      detectionId: 'browser-game-unblocked-detection-unmanaged',
      surfaceKind: 'unmanaged-browser-bypass',
      classificationKind: 'unmanaged-browser-game-bypass',
      actionCandidate: 'bypass-evidence-only-candidate',
      managedRouteEvidenceRef: null,
      portalIndexEvidenceRef: null,
      unmanagedProcessEvidenceRef: 'browser-game-unmanaged-process-evidence',
      parentPolicyRef: null,
      reasonCodes: ['unmanaged-browser-process-only'],
      signalRows: [
        signal({
          signalId: 'browser-game-unblocked-signal-unmanaged',
          signalKind: 'unmanaged-browser-process-only',
          surfaceKind: 'unmanaged-browser-bypass',
          evidenceRefs: ['browser-game-unmanaged-process-evidence'],
        }),
      ],
    })
  );

  expect(parsed.actionCandidate).toBe('bypass-evidence-only-candidate');
  expect(parsed.exactUnmanagedUrlClaimed).toBe(false);
}

function rejectsAuthorityClaims() {
  const invalidRows = [
    { rawUrlStored: true },
    { rawPageBodyStored: true },
    { rawSearchQueryStored: true },
    { iframeContentCaptured: true },
    { exactUnmanagedUrlClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { accountOrPurchaseFlowClaimed: true },
    { uiRenderedClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { enforcementClaimed: true },
  ];
  const invalidSignals = [
    { rawUrlStored: true },
    { rawPageBodyStored: true },
    { rawSearchQueryStored: true },
    { iframeContentCaptured: true },
    { exactUnmanagedUrlClaimed: true },
    { policyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameUnblockedSiteDetectionSchema.safeParse(detection(invalid)).success).toBe(false);
  }
  for (const invalid of invalidSignals) {
    expect(BrowserGameUnblockedSiteSignalSchema.safeParse(signal(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentDetections() {
  const invalidRows = [
    { signalRows: [] },
    { confidence: 'unknown' },
    { classificationKind: 'unknown' },
    { managedRouteEvidenceRef: null },
    { actionCandidate: 'block-during-school-candidate', parentPolicyRef: null },
    { actionCandidate: 'parent-review-candidate', portalIndexEvidenceRef: null },
    { actionCandidate: 'block-unknown-iframe-candidate', iframeEvidenceRef: null },
    { actionCandidate: 'bypass-evidence-only-candidate', unmanagedProcessEvidenceRef: null },
    { signalRows: [signal({ signalKind: 'unknown-signal' })] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameUnblockedSiteDetectionSchema.safeParse(detection(invalid)).success).toBe(false);
  }
}

function detection(overrides = {}): BrowserGameUnblockedSiteDetection {
  return {
    schemaVersion: 'browser-game-unblocked-site-detection-contract',
    detectionId: 'browser-game-unblocked-detection-school-portal',
    familyId: 'family-browser-game-unblocked',
    childProfileId: 'child-browser-game-unblocked',
    deviceId: 'device-browser-game-unblocked',
    detectedAt: '2026-06-03T10:45:00.000Z',
    surfaceKind: 'managed-browser-route',
    classificationKind: 'unblocked-game-site',
    detectionState: 'candidate',
    confidence: 'high',
    sourceEvidenceRefs: ['browser-game-managed-route-evidence', 'browser-game-portal-index-evidence'],
    signalRows: [
      signal(),
      signal({ signalId: 'browser-game-unblocked-signal-school', signalKind: 'school-bypass-language' }),
    ],
    actionCandidate: 'block-during-school-candidate',
    managedRouteEvidenceRef: 'browser-game-managed-route-evidence',
    portalIndexEvidenceRef: 'browser-game-portal-index-evidence',
    iframeEvidenceRef: null,
    searchIntentEvidenceRef: null,
    unmanagedProcessEvidenceRef: null,
    parentPolicyRef: 'browser-game-policy-school-night',
    reasonCodes: ['domain-keyword-match', 'portal-index-detected', 'school-bypass-portal'],
    deliveryState: 'contract-only',
    rawUrlStored: false,
    rawPageBodyStored: false,
    rawSearchQueryStored: false,
    iframeContentCaptured: false,
    exactUnmanagedUrlClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    accountOrPurchaseFlowClaimed: false,
    uiRenderedClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function signal(overrides = {}) {
  return {
    signalId: 'browser-game-unblocked-signal-domain-keyword',
    signalKind: 'unblocked-domain-keyword',
    surfaceKind: 'managed-browser-route',
    detectionState: 'candidate',
    confidence: 'high',
    evidenceRefs: ['browser-game-managed-route-evidence'],
    rawUrlStored: false,
    rawPageBodyStored: false,
    rawSearchQueryStored: false,
    iframeContentCaptured: false,
    exactUnmanagedUrlClaimed: false,
    policyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
