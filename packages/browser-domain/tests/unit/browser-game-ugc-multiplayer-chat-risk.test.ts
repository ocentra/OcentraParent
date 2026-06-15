import { describe, expect, it } from 'vitest';
import {
  BrowserGameUgcRiskAssessmentSchema,
  type BrowserGameUgcRiskAssessment,
  BrowserGameUgcRiskRowSchema,
} from '../../src/browser-game-ugc-multiplayer-chat-risk';

describe('browser-game UGC multiplayer chat risk contracts', () => {
  it('accepts UGC multiplayer risk candidates without claiming enforcement', acceptsUgcRiskCandidate);
  it('accepts chat control candidates only with capability proof refs', acceptsChatControlCandidate);
  it('rejects raw chat/profile/account/runtime/native/policy authority claims', rejectsAuthorityClaims);
  it('rejects inconsistent UGC, chat, purchase, and degraded states', rejectsInconsistentStates);
});

function acceptsUgcRiskCandidate() {
  const parsed = BrowserGameUgcRiskAssessmentSchema.parse(assessment());

  expect(parsed.schemaVersion).toBe('browser-game-ugc-multiplayer-chat-risk-contract');
  expect(parsed.recommendedControl).toBe('block-unknown-ugc-candidate');
  expect(parsed.finalPolicyDecisionClaimed).toBe(false);
  expect(parsed.runtimeGateExecutedClaimed).toBe(false);
  expect(parsed.enforcementClaimed).toBe(false);
}

function acceptsChatControlCandidate() {
  const parsed = BrowserGameUgcRiskAssessmentSchema.parse(
    assessment({
      assessmentId: 'browser-game-ugc-risk-chat',
      platformSurfaceKind: 'profile-friends-messages',
      riskRows: [riskRow({ riskKind: 'chat-contact', evidenceKind: 'chat-control-capability' })],
      recommendedControl: 'block-chat-candidate',
      chatControlCapabilityRef: 'browser-game-chat-control-capability-ref',
    })
  );

  expect(parsed.recommendedControl).toBe('block-chat-candidate');
  expect(parsed.rawChatContentRead).toBe(false);
}

function rejectsAuthorityClaims() {
  const invalidRows = [
    { rawChatContentRead: true },
    { rawProfileContentStored: true },
    { rawExperienceIdentifierStored: true },
    { rawAccountIdentifierStored: true },
    { rawGamePayloadUsed: true },
    { webToAppLaunchExecuted: true },
    { purchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { enforcementClaimed: true },
  ];
  const invalidAssessments = [
    { rawChatContentRead: true },
    { rawProfileContentStored: true },
    { rawExperienceIdentifierStored: true },
    { rawAccountIdentifierStored: true },
    { rawGamePayloadUsed: true },
    { webToAppLaunchExecuted: true },
    { purchaseExecutionClaimed: true },
    { nativeGameControlClaimed: true },
    { finalPolicyDecisionClaimed: true },
    { runtimeGateExecutedClaimed: true },
    { uiRenderedClaimed: true },
    { enforcementClaimed: true },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameUgcRiskRowSchema.safeParse(riskRow(invalid)).success).toBe(false);
  }
  for (const invalid of invalidAssessments) {
    expect(BrowserGameUgcRiskAssessmentSchema.safeParse(assessment(invalid)).success).toBe(false);
  }
}

function rejectsInconsistentStates() {
  const invalidRows = [
    { riskRows: [] },
    { recommendedControl: 'block-chat-candidate' },
    {
      recommendedControl: 'purchase-approval-candidate',
      riskRows: [riskRow({ riskKind: 'in-game-purchase', evidenceKind: 'purchase-control-capability' })],
    },
    { recommendedControl: 'approved-experience-only-candidate', approvedExperienceRef: null, parentRuleRef: null },
    { riskRows: [riskRow({ riskKind: 'unknown-risk' })] },
    {
      degradedState: 'degraded',
      confidence: 'high',
      recommendedControl: 'manual-review-candidate',
      uncertaintyReasons: ['low-confidence'],
    },
    {
      degradedState: 'manual-required',
      platformSurfaceKind: 'manual-required',
      confidence: 'low',
      recommendedControl: 'block-unknown-ugc-candidate',
      uncertaintyReasons: ['manual-required'],
    },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGameUgcRiskAssessmentSchema.safeParse(assessment(invalid)).success).toBe(false);
  }
}

function assessment(overrides = {}): BrowserGameUgcRiskAssessment {
  return {
    schemaVersion: 'browser-game-ugc-multiplayer-chat-risk-contract',
    assessmentId: 'browser-game-ugc-risk-unknown-experience',
    familyId: 'family-browser-game-ugc',
    childProfileId: 'child-browser-game-ugc',
    deviceId: 'device-browser-game-ugc',
    assessedAt: '2026-06-03T10:28:00.000Z',
    platformSurfaceKind: 'experience-page',
    sourceEvidenceRefs: ['browser-game-ugc-route-evidence', 'browser-game-ugc-risk-context-evidence'],
    riskRows: [
      riskRow({ riskKind: 'ugc-world', evidenceKind: 'managed-route' }),
      riskRow({
        riskRowId: 'browser-game-ugc-risk-unknown-player',
        riskKind: 'unknown-player-contact',
        evidenceKind: 'public-risk-context',
      }),
    ],
    recommendedControl: 'block-unknown-ugc-candidate',
    confidence: 'medium',
    degradedState: 'none',
    uncertaintyReasons: [],
    parentRuleRef: 'browser-game-ugc-parent-rule-ref',
    approvedExperienceRef: null,
    chatControlCapabilityRef: null,
    purchaseApprovalCapabilityRef: null,
    mobileCapabilityRef: 'browser-game-mobile-capability-ref',
    rawChatContentRead: false,
    rawProfileContentStored: false,
    rawExperienceIdentifierStored: false,
    rawAccountIdentifierStored: false,
    rawGamePayloadUsed: false,
    webToAppLaunchExecuted: false,
    purchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    uiRenderedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}

function riskRow(overrides = {}) {
  return {
    riskRowId: 'browser-game-ugc-risk-row',
    evidenceKind: 'managed-route',
    riskKind: 'ugc-world',
    state: 'candidate',
    severity: 'high',
    confidence: 'medium',
    evidenceRefs: ['browser-game-ugc-risk-evidence-ref'],
    rawChatContentRead: false,
    rawProfileContentStored: false,
    rawExperienceIdentifierStored: false,
    rawAccountIdentifierStored: false,
    rawGamePayloadUsed: false,
    webToAppLaunchExecuted: false,
    purchaseExecutionClaimed: false,
    nativeGameControlClaimed: false,
    finalPolicyDecisionClaimed: false,
    runtimeGateExecutedClaimed: false,
    enforcementClaimed: false,
    ...overrides,
  };
}
