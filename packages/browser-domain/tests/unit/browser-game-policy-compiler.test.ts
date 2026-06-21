import { describe, expect, it } from 'vitest';
import { PolicyCompilerCapabilityState } from '@ocentra-parent/schema-domain/policy-compiler';
import {
  BrowserGamePolicyCompilerInputSchema,
  BrowserGamePolicyDecisionCandidateSchema,
} from '@ocentra-parent/schema-domain/browser-game-policy-compiler';
import {
  compileBrowserGamePolicyCandidate,
} from '../../src/browser-game-policy-compiler';

describe('browser game parent policy compiler contracts', () => {
  it('compiles a contract-only browser-game policy decision candidate from refs', compilesDecisionCandidate);
  it('accepts parent-review, time-limit, manual-review, and unknown fallback candidates', acceptsFallbacks);
  it(
    'rejects raw payload, activity objects, UI, native-game, cloud-frame, and enforcement input claims',
    rejectsInputClaims
  );
  it('rejects final decision, runtime gate, UI, enforcement, and inconsistent candidate claims', rejectsDecisionClaims);
});

function compilesDecisionCandidate() {
  const input = BrowserGamePolicyCompilerInputSchema.parse(policyInput());
  const decision = compileBrowserGamePolicyCandidate({
    input,
    decisionCandidateId: 'browser-game-policy-decision-candidate-cloud',
    decidedAt: '2026-06-03T09:12:30.000Z',
    expiresAt: '2026-06-03T10:12:30.000Z',
    actionCandidate: 'warn-candidate',
    reasonCodes: ['cloud-gaming-risk', 'browser-game-risk-high', 'parent-rule-match'],
    confidence: 'medium',
    fallbackUsed: false,
    parentApprovalRequired: false,
  });

  expect(decision.compileRequestId).toBe(input.compileRequestId);
  expect(decision.targetKind).toBe('cloud-gaming-session');
  expect(decision.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.Supported);
  expect(decision.finalPolicyDecisionClaimed).toBe(false);
  expect(decision.runtimeGateExecutedClaimed).toBe(false);
  expect(decision.enforcementClaimed).toBe(false);
}

function acceptsFallbacks() {
  const askParent = decisionCandidate({
    actionCandidate: 'parent-review-candidate',
    reasonCodes: ['parent-rule-match', 'purchase-risk'],
    parentApprovalRequired: true,
  });
  const timeLimit = decisionCandidate({
    actionCandidate: 'time-limit-candidate',
    reasonCodes: ['parent-rule-match', 'schedule-context'],
  });
  const manualReview = decisionCandidate({
    input: policyInput({ compilerMode: 'manual-required', targetKind: 'manual-required', analysisRefs: [] }),
    actionCandidate: 'manual-review-candidate',
    reasonCodes: ['manual-required', 'mobile-capability-manual-required'],
    fallbackUsed: true,
  });
  const unknown = decisionCandidate({
    input: policyInput({ compilerMode: 'unavailable', targetKind: 'manual-required', analysisRefs: [] }),
    actionCandidate: 'unknown-candidate',
    reasonCodes: ['missing-game-evidence', 'unknown-evidence'],
    confidence: 'unknown',
    fallbackUsed: true,
  });

  expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(askParent).success).toBe(true);
  expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(timeLimit).success).toBe(true);
  expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(manualReview).success).toBe(true);
  expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(unknown).success).toBe(true);
  expect(askParent.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.Supported);
  expect(manualReview.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.ManualRequired);
  expect(unknown.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.Unsupported);
}

function rejectsInputClaims() {
  const invalidRows = [
    { ...policyInput(), rawGamePayloadIncluded: true },
    { ...policyInput(), rawModelTextIncluded: true },
    { ...policyInput(), activityDomainObjectIncluded: true },
    { ...policyInput(), finalDecisionClaimedByInput: true },
    { ...policyInput(), runtimeGateClaimedByInput: true },
    { ...policyInput(), uiClaimedByInput: true },
    { ...policyInput(), enforcementClaimedByInput: true },
    { ...policyInput(), nativeGameControlClaimed: true },
    { ...policyInput(), cloudFrameAnalysisClaimed: true },
    { ...policyInput(), analysisRefs: [] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGamePolicyCompilerInputSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsDecisionClaims() {
  const valid = decisionCandidate();
  const invalidRows = [
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, runtimeGateExecutedClaimed: true },
    { ...valid, uiRenderedClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeGameControlClaimed: true },
    { ...valid, cloudFrameAnalysisClaimed: true },
    { ...valid, rawGamePayloadStored: true },
    { ...valid, rawModelTextUsed: true },
    { ...valid, actionCandidate: 'unknown-candidate', fallbackUsed: false },
    { ...valid, actionCandidate: 'manual-review-candidate', fallbackUsed: false },
    { ...valid, actionCandidate: 'parent-review-candidate', parentApprovalRequired: false },
    { ...valid, actionCandidate: 'allow-candidate', reasonCodes: ['browser-game-risk-high'] },
    { ...valid, actionCandidate: 'time-limit-candidate', reasonCodes: ['parent-rule-match'] },
  ];

  for (const invalid of invalidRows) {
    expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(invalid).success).toBe(false);
  }
}

function decisionCandidate(overrides = {}) {
  const input = BrowserGamePolicyCompilerInputSchema.parse(overridesInput(overrides));
  return compileBrowserGamePolicyCandidate({
    input,
    decisionCandidateId: 'browser-game-policy-decision-candidate-cloud',
    decidedAt: '2026-06-03T09:12:30.000Z',
    expiresAt: '2026-06-03T10:12:30.000Z',
    actionCandidate: 'warn-candidate',
    reasonCodes: ['cloud-gaming-risk', 'browser-game-risk-high'],
    confidence: 'medium',
    fallbackUsed: false,
    parentApprovalRequired: false,
    ...overrides,
  });
}

function overridesInput(overrides = {}) {
  if ('input' in overrides) {
    return overrides.input;
  }
  return policyInput();
}

function policyInput(overrides = {}) {
  return {
    schemaVersion: 'v0.6',
    compileRequestId: 'browser-game-policy-compile-request-cloud',
    familyId: 'family-main',
    childProfileId: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    requestedAt: '2026-06-03T09:12:00.000Z',
    policyVersionRef: 'policy-version-2026-06-03',
    targetKind: 'cloud-gaming-session',
    sourceEvidenceRefs: ['parent-evidence-browser-game-route', 'parent-evidence-browser-game-signal-set'],
    analysisRefs: ['browser-game-riskbenefit-signal-set-cloud'],
    mobileCapabilityRefs: ['browser-game-mobile-capability-matrix'],
    parentRuleRefs: ['parent-rule-school-night-cloud-gaming'],
    scheduleContextRefs: ['schedule-context-school-night'],
    compilerMode: 'contract-only',
    rawGamePayloadIncluded: false,
    rawModelTextIncluded: false,
    activityDomainObjectIncluded: false,
    finalDecisionClaimedByInput: false,
    runtimeGateClaimedByInput: false,
    uiClaimedByInput: false,
    enforcementClaimedByInput: false,
    nativeGameControlClaimed: false,
    cloudFrameAnalysisClaimed: false,
    ...overrides,
  };
}
