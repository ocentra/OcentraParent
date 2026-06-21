import { describe, expect, it } from 'vitest';
import {
  BrowserGameApprovalDecisionSchema,
  BrowserGameApprovalRequestSchema,
  type BrowserGameApprovalRequest,
} from '@ocentra-parent/schema-domain/browser-game-account-purchase-gate';

describe('browser-game account and purchase gate contracts', () => {
  it(
    'accepts contract-only approval requests for account, purchase, download, cloud, and unknown game flows',
    acceptsRequests
  );
  it('accepts recorded candidate decisions without claiming runtime execution', acceptsDecisions);
  it(
    'rejects raw URL, credential, submission, payment, UI, runtime, native, cloud-frame, and enforcement claims',
    rejectsExecutionClaims
  );
  it('rejects inconsistent request states, refs, and reason codes', rejectsInconsistentRequests);
  it('rejects inconsistent decisions and executed action claims', rejectsInconsistentDecisions);
});

function acceptsRequests() {
  expect(BrowserGameApprovalRequestSchema.safeParse(approvalRequest()).success).toBe(true);
  expect(
    BrowserGameApprovalRequestSchema.safeParse(
      approvalRequest({
        approvalRequestId: 'browser-game-approval-request-subscription',
        requestKind: 'subscription-purchase',
        reasonCodes: ['subscription-route', 'parent-rule-requires-approval'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameApprovalRequestSchema.safeParse(
      approvalRequest({
        approvalRequestId: 'browser-game-approval-request-loot-box',
        requestKind: 'loot-box-purchase',
        requestState: 'blocked-candidate',
        reasonCodes: ['loot-box-route', 'parent-rule-blocks-flow'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameApprovalRequestSchema.safeParse(
      approvalRequest({
        approvalRequestId: 'browser-game-approval-request-download',
        requestKind: 'game-download',
        requestState: 'blocked-candidate',
        reasonCodes: ['download-or-install-route', 'parent-rule-blocks-flow'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameApprovalRequestSchema.safeParse(
      approvalRequest({
        approvalRequestId: 'browser-game-approval-request-cloud',
        requestKind: 'cloud-gaming-start',
        reasonCodes: ['cloud-gaming-route', 'parent-rule-requires-approval'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameApprovalRequestSchema.safeParse(
      approvalRequest({
        approvalRequestId: 'browser-game-approval-request-manual',
        requestKind: 'wallet-or-gambling-payment',
        requestState: 'manual-required',
        reasonCodes: ['wallet-payment-risk', 'gambling-like-payment-risk', 'manual-required'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameApprovalRequestSchema.safeParse(
      approvalRequest({
        approvalRequestId: 'browser-game-approval-request-unavailable',
        requestKind: 'unknown-game-start',
        requestState: 'unavailable',
        managedRouteEvidenceRef: null,
        reasonCodes: ['missing-route-proof', 'unknown-game-route'],
      })
    ).success
  ).toBe(true);
}

function acceptsDecisions() {
  expect(BrowserGameApprovalDecisionSchema.safeParse(approvalDecision()).success).toBe(true);
  expect(
    BrowserGameApprovalDecisionSchema.safeParse(
      approvalDecision({
        approvalDecisionId: 'browser-game-approval-decision-purchase',
        decisionKind: 'approve-purchase-candidate',
        reasonCodes: ['purchase-route', 'parent-rule-requires-approval'],
      })
    ).success
  ).toBe(true);
  expect(
    BrowserGameApprovalDecisionSchema.safeParse(
      approvalDecision({
        approvalDecisionId: 'browser-game-approval-decision-manual',
        decisionKind: 'manual-required',
        decisionState: 'manual-required',
        decidedByActorId: null,
        policyVersionRef: null,
        actionCandidateRef: null,
        reasonCodes: ['manual-required'],
      })
    ).success
  ).toBe(true);
}

function rejectsExecutionClaims() {
  const invalidRequests = [
    { rawUrlStored: true },
    { rawGameTitleStored: true },
    { rawAccountIdentifierCaptured: true },
    { credentialCaptured: true },
    { formSubmittedClaimed: true },
    { accountCreatedClaimed: true },
    { purchaseExecutedClaimed: true },
    { paymentInfoCaptured: true },
    { launcherDownloadClaimed: true },
    { notificationDeliveredClaimed: true },
    { uiRenderedClaimed: true },
    { childNotifiedClaimed: true },
    { policyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { nativeGameControlClaimed: true },
    { cloudFrameAnalysisClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRequests) {
    expect(BrowserGameApprovalRequestSchema.safeParse(approvalRequest(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentRequests() {
  const invalidRequests = [
    { managedRouteEvidenceRef: null },
    { parentRuleRef: null },
    { expiresAt: null },
    { reasonCodes: ['parent-rule-requires-approval'] },
    { requestState: 'blocked-candidate', reasonCodes: ['purchase-route', 'parent-rule-requires-approval'] },
    { requestState: 'unavailable', managedRouteEvidenceRef: 'managed-route-ref' },
    { requestState: 'unavailable', reasonCodes: ['unknown-game-route'] },
    { requestKind: 'game-login', reasonCodes: ['account-creation-route', 'parent-rule-requires-approval'] },
    { requestKind: 'virtual-currency-purchase', reasonCodes: ['purchase-route', 'parent-rule-requires-approval'] },
  ];

  for (const invalid of invalidRequests) {
    expect(BrowserGameApprovalRequestSchema.safeParse(approvalRequest(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentDecisions() {
  const valid = approvalDecision();
  const invalidDecisions = [
    { ...valid, decisionKind: 'manual-required', decisionState: 'recorded-contract-only' },
    { ...valid, decisionState: 'manual-required' },
    { ...valid, decidedByActorId: null },
    { ...valid, policyVersionRef: null },
    { ...valid, actionCandidateRef: null },
    { ...valid, notificationDeliveredClaimed: true },
    { ...valid, uiRenderedClaimed: true },
    { ...valid, childNotifiedClaimed: true },
    { ...valid, policyDecisionClaimed: true },
    { ...valid, runtimeGateExecutedClaimed: true },
    { ...valid, accountCreatedClaimed: true },
    { ...valid, purchaseExecutedClaimed: true },
    { ...valid, launcherDownloadClaimed: true },
    { ...valid, nativeGameControlClaimed: true },
    { ...valid, cloudFrameAnalysisClaimed: true },
    { ...valid, enforcementClaimed: true },
  ];

  for (const invalid of invalidDecisions) {
    expect(BrowserGameApprovalDecisionSchema.safeParse(invalid).success).toBe(false);
  }
}

function approvalRequest(overrides = {}): BrowserGameApprovalRequest {
  return {
    schemaVersion: 'v0.6',
    approvalRequestId: 'browser-game-approval-request-account',
    familyId: 'family-browser-game-approval',
    childProfileId: 'child-browser-game-approval',
    requestedByDeviceId: 'device-browser-game-approval',
    requestedAt: '2026-06-03T09:50:00.000Z',
    expiresAt: '2026-06-03T10:05:00.000Z',
    requestKind: 'game-account-creation',
    requestState: 'pending-contract-only',
    confidence: 'medium',
    sourceEvidenceRefs: ['browser-game-route-evidence-account'],
    managedRouteEvidenceRef: 'managed-route-evidence-account',
    gameTitleEvidenceRef: 'game-title-evidence-account',
    aiAnalysisRef: 'browser-game-ai-analysis-account',
    parentRuleRef: 'parent-rule-game-account-approval',
    reasonCodes: ['account-creation-route', 'parent-rule-requires-approval'],
    deliveryState: 'contract-only',
    rawUrlStored: false,
    rawGameTitleStored: false,
    rawAccountIdentifierCaptured: false,
    credentialCaptured: false,
    formSubmittedClaimed: false,
    accountCreatedClaimed: false,
    purchaseExecutedClaimed: false,
    paymentInfoCaptured: false,
    launcherDownloadClaimed: false,
    notificationDeliveredClaimed: false,
    uiRenderedClaimed: false,
    childNotifiedClaimed: false,
    policyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function approvalDecision(overrides = {}) {
  return {
    schemaVersion: 'v0.6',
    approvalDecisionId: 'browser-game-approval-decision-account',
    approvalRequestId: 'browser-game-approval-request-account',
    familyId: 'family-browser-game-approval',
    childProfileId: 'child-browser-game-approval',
    decidedAt: '2026-06-03T09:52:00.000Z',
    decidedByActorId: 'parent-actor-browser-game-approval',
    decisionKind: 'approve-account-candidate',
    decisionState: 'recorded-contract-only',
    sourceEvidenceRefs: ['browser-game-route-evidence-account', 'parent-approval-evidence-account'],
    policyVersionRef: 'policy-version-browser-game-approval',
    actionCandidateRef: 'action-candidate-browser-game-approval',
    reasonCodes: ['account-creation-route', 'parent-rule-requires-approval'],
    deliveryState: 'contract-only',
    notificationDeliveredClaimed: false,
    uiRenderedClaimed: false,
    childNotifiedClaimed: false,
    policyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    accountCreatedClaimed: false,
    purchaseExecutedClaimed: false,
    launcherDownloadClaimed: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
