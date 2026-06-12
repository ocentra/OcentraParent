import { describe, expect, it } from 'vitest';
import {
  BrowserGameCloudGateDecisionSchema,
  BrowserGameCloudGateRequestSchema,
  type BrowserGameCloudGateRequest,
} from '../../src/browser-game-cloud-gaming-gate';

describe('browser-game cloud-gaming gate contracts', () => {
  it(
    'accepts candidate cloud-gaming gate requests for parent approval, block, time-limit, and allow-window outcomes',
    acceptsGateRequests
  );
  it('accepts manual-required and unavailable cloud-gaming fallback states', acceptsFallbackStates);
  it('accepts recorded candidate decisions without claiming runtime execution', acceptsDecisions);
  it('rejects cloud-frame, title, native, UI, runtime, account, and enforcement claims', rejectsRuntimeClaims);
  it('rejects inconsistent cloud-gaming actions, missing refs, and decision upgrades', rejectsInconsistentStates);
});

function acceptsGateRequests() {
  expect(BrowserGameCloudGateRequestSchema.safeParse(cloudRequest()).success).toBe(true);
  expect(
    BrowserGameCloudGateRequestSchema.safeParse(
      cloudRequest({
        gateRequestId: 'cloud-gate-request-block',
        gateSubject: 'mature-cloud-game',
        actionCandidate: 'block-candidate',
        policyCandidateRef: 'cloud-policy-candidate-block',
        reasonCodes: ['known-cloud-domain', 'mature-title-risk'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameCloudGateRequestSchema.safeParse(
      cloudRequest({
        gateRequestId: 'cloud-gate-request-time-limit',
        gateSubject: 'time-budget-cloud-gaming',
        actionCandidate: 'time-limit-candidate',
        policyCandidateRef: 'cloud-policy-candidate-time',
        scheduleContextRef: 'cloud-schedule-context',
        reasonCodes: ['known-cloud-domain', 'time-budget-candidate'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameCloudGateRequestSchema.safeParse(
      cloudRequest({
        gateRequestId: 'cloud-gate-request-allow',
        gateSubject: 'cloud-platform-session',
        actionCandidate: 'allow-window-candidate',
        parentApprovalRequestRef: null,
        policyCandidateRef: 'cloud-policy-candidate-allow',
        reasonCodes: ['known-cloud-domain', 'title-metadata-present'],
        signalKinds: ['known-cloud-domain', 'streaming-session-route', 'platform-title-metadata'],
      })
    ).success
  ).toBe(true);
}

function acceptsFallbackStates() {
  expect(
    BrowserGameCloudGateRequestSchema.safeParse(
      cloudRequest({
        gateRequestId: 'cloud-gate-request-manual',
        gateState: 'manual-required',
        actionCandidate: 'manual-review-candidate',
        parentApprovalRequestRef: null,
        reasonCodes: ['manual-required', 'content-frame-unavailable', 'cloud-title-unavailable'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameCloudGateRequestSchema.safeParse(
      cloudRequest({
        gateRequestId: 'cloud-gate-request-unavailable',
        gateState: 'unavailable',
        actionCandidate: 'unknown-fallback-candidate',
        managedRouteEvidenceRef: null,
        parentApprovalRequestRef: null,
        reasonCodes: ['missing-platform-proof', 'runtime-signal-unavailable'],
      })
    ).success
  ).toBe(true);
}

function acceptsDecisions() {
  expect(BrowserGameCloudGateDecisionSchema.safeParse(cloudDecision()).success).toBe(true);
  expect(
    BrowserGameCloudGateDecisionSchema.safeParse(
      cloudDecision({
        gateDecisionId: 'cloud-gate-decision-manual',
        decisionKind: 'manual-required',
        decisionState: 'manual-required',
        decidedByActorId: null,
        policyVersionRef: null,
        actionCandidateRef: null,
        reasonCodes: ['manual-required', 'content-frame-unavailable'],
      })
    ).success
  ).toBe(true);
}

function rejectsRuntimeClaims() {
  const invalidRequests = [
    { rawCloudTitleStored: true },
    { rawStreamFrameStored: true },
    { cloudStreamFrameAnalysisClaimed: true },
    { perGameCloudTitleClaimed: true },
    { nativeGameControlClaimed: true },
    { nativeLauncherControlClaimed: true },
    { gameChatContentClaimed: true },
    { accountOrPurchaseFlowClaimed: true },
    { notificationDeliveredClaimed: true },
    { uiRenderedClaimed: true },
    { childNotifiedClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRequests) {
    expect(BrowserGameCloudGateRequestSchema.safeParse(cloudRequest(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentStates() {
  const invalidRequests = [
    { managedRouteEvidenceRef: null },
    { expiresAt: null },
    { actionCandidate: 'parent-review-candidate', parentApprovalRequestRef: null },
    { actionCandidate: 'parent-review-candidate', reasonCodes: ['known-cloud-domain'] },
    { actionCandidate: 'block-candidate', policyCandidateRef: null, reasonCodes: ['mature-title-risk'] },
    { actionCandidate: 'time-limit-candidate', policyCandidateRef: 'policy', scheduleContextRef: null },
    { actionCandidate: 'allow-window-candidate', policyCandidateRef: null },
    { gateState: 'manual-required', actionCandidate: 'parent-review-candidate' },
    { gateState: 'unavailable', managedRouteEvidenceRef: 'cloud-route-ref' },
  ];

  for (const invalid of invalidRequests) {
    expect(BrowserGameCloudGateRequestSchema.safeParse(cloudRequest(invalid)).success).toBe(false);
  }

  const valid = cloudDecision();
  const invalidDecisions = [
    { ...valid, decisionKind: 'manual-required', decisionState: 'recorded-contract-only' },
    { ...valid, decisionState: 'manual-required' },
    { ...valid, decidedByActorId: null },
    { ...valid, policyVersionRef: null },
    { ...valid, actionCandidateRef: null },
    { ...valid, runtimeGateExecutedClaimed: true },
    { ...valid, cloudStreamFrameAnalysisClaimed: true },
    { ...valid, perGameCloudTitleClaimed: true },
    { ...valid, nativeLauncherControlClaimed: true },
    { ...valid, enforcementClaimed: true },
  ];

  for (const invalid of invalidDecisions) {
    expect(BrowserGameCloudGateDecisionSchema.safeParse(invalid).success).toBe(false);
  }
}

function cloudRequest(overrides = {}): BrowserGameCloudGateRequest {
  return {
    schemaVersion: 'v0.6',
    gateRequestId: 'cloud-gate-request-approval',
    familyId: 'family-cloud-gate',
    childProfileId: 'child-cloud-gate',
    requestedByDeviceId: 'device-cloud-gate',
    requestedAt: '2026-06-03T10:00:00.000Z',
    expiresAt: '2026-06-03T10:15:00.000Z',
    platform: 'geforce-now',
    gateSubject: 'unknown-cloud-game',
    gateState: 'candidate',
    actionCandidate: 'parent-review-candidate',
    confidence: 'medium',
    sourceEvidenceRefs: ['cloud-route-evidence', 'cloud-signal-evidence'],
    signalKinds: ['known-cloud-domain', 'streaming-session-route', 'gamepad-api', 'unknown-title-fallback'],
    managedRouteEvidenceRef: 'cloud-managed-route-evidence',
    platformTitleEvidenceRef: null,
    platformRatingEvidenceRef: null,
    policyCandidateRef: 'cloud-policy-candidate',
    parentApprovalRequestRef: 'cloud-parent-approval-request',
    scheduleContextRef: 'cloud-schedule-context',
    mobileCapabilityRef: 'cloud-mobile-capability',
    reasonCodes: ['known-cloud-domain', 'streaming-route', 'unknown-cloud-title', 'parent-approval-required'],
    deliveryState: 'contract-only',
    rawCloudTitleStored: false,
    rawStreamFrameStored: false,
    cloudStreamFrameAnalysisClaimed: false,
    perGameCloudTitleClaimed: false,
    nativeGameControlClaimed: false,
    nativeLauncherControlClaimed: false,
    gameChatContentClaimed: false,
    accountOrPurchaseFlowClaimed: false,
    notificationDeliveredClaimed: false,
    uiRenderedClaimed: false,
    childNotifiedClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function cloudDecision(overrides = {}) {
  return {
    schemaVersion: 'v0.6',
    gateDecisionId: 'cloud-gate-decision-approval',
    gateRequestId: 'cloud-gate-request-approval',
    familyId: 'family-cloud-gate',
    childProfileId: 'child-cloud-gate',
    decidedAt: '2026-06-03T10:02:00.000Z',
    decidedByActorId: 'parent-actor-cloud-gate',
    decisionKind: 'parent-review-candidate',
    decisionState: 'recorded-contract-only',
    sourceEvidenceRefs: ['cloud-route-evidence', 'cloud-parent-approval-evidence'],
    policyVersionRef: 'policy-version-cloud-gate',
    actionCandidateRef: 'cloud-action-candidate',
    reasonCodes: ['unknown-cloud-title', 'parent-approval-required'],
    deliveryState: 'contract-only',
    notificationDeliveredClaimed: false,
    uiRenderedClaimed: false,
    childNotifiedClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    cloudStreamFrameAnalysisClaimed: false,
    perGameCloudTitleClaimed: false,
    nativeGameControlClaimed: false,
    nativeLauncherControlClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
