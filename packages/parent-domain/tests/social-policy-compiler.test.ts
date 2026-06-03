import { describe, expect, it } from 'vitest';
import {
  SocialParentPolicyCompilerInputSchema,
  SocialParentPolicyDecisionCandidateSchema,
  compileSocialParentPolicyCandidate,
} from '../src/social-policy-compiler';

describe('social parent policy compiler contracts', () => {
  it('compiles a contract-only social policy decision candidate from refs', compilesDecisionCandidate);
  it('accepts parent-review, manual-review, and unknown fallback candidates with required reasons', acceptsFallbacks);
  it('rejects raw payload, activity objects, UI, connector, native, and enforcement input claims', rejectsInputClaims);
  it('rejects final decision, runtime gate, UI, enforcement, and inconsistent candidate claims', rejectsDecisionClaims);
});

function compilesDecisionCandidate() {
  const input = SocialParentPolicyCompilerInputSchema.parse(policyInput());
  const decision = compileSocialParentPolicyCandidate({
    input,
    decisionCandidateId: 'social-policy-decision-candidate-video',
    decidedAt: '2026-06-03T06:50:30.000Z',
    expiresAt: '2026-06-03T07:50:30.000Z',
    actionCandidate: 'warn-candidate',
    reasonCodes: ['social-risk-high', 'video-safety-risk', 'parent-rule-match'],
    confidence: 'medium',
    fallbackUsed: false,
    parentApprovalRequired: false,
  });

  expect(decision.compileRequestId).toBe(input.compileRequestId);
  expect(decision.targetKind).toBe('social-video');
  expect(decision.finalPolicyDecisionClaimed).toBe(false);
  expect(decision.runtimeGateExecutedClaimed).toBe(false);
  expect(decision.enforcementClaimed).toBe(false);
}

function acceptsFallbacks() {
  const askParent = decisionCandidate({
    actionCandidate: 'parent-review-candidate',
    reasonCodes: ['parent-rule-match', 'secondary-account-risk'],
    parentApprovalRequired: true,
  });
  const manualReview = decisionCandidate({
    input: policyInput({ compilerMode: 'manual-required', targetKind: 'manual-required', signalSetRefs: [] }),
    actionCandidate: 'manual-review-candidate',
    reasonCodes: ['manual-required'],
    fallbackUsed: true,
  });
  const unknown = decisionCandidate({
    input: policyInput({ compilerMode: 'unavailable', targetKind: 'manual-required', signalSetRefs: [] }),
    actionCandidate: 'unknown-candidate',
    reasonCodes: ['missing-signal-proof', 'unknown-evidence'],
    confidence: 'unknown',
    fallbackUsed: true,
  });

  expect(SocialParentPolicyDecisionCandidateSchema.safeParse(askParent).success).toBe(true);
  expect(SocialParentPolicyDecisionCandidateSchema.safeParse(manualReview).success).toBe(true);
  expect(SocialParentPolicyDecisionCandidateSchema.safeParse(unknown).success).toBe(true);
}

function rejectsInputClaims() {
  const invalidRows = [
    { ...policyInput(), rawSignalPayloadIncluded: true },
    { ...policyInput(), rawModelTextIncluded: true },
    { ...policyInput(), activityDomainObjectIncluded: true },
    { ...policyInput(), finalDecisionClaimedByInput: true },
    { ...policyInput(), runtimeGateClaimedByInput: true },
    { ...policyInput(), uiClaimedByInput: true },
    { ...policyInput(), enforcementClaimedByInput: true },
    { ...policyInput(), nativeAppControlClaimed: true },
    { ...policyInput(), platformConnectorClaimed: true },
    { ...policyInput(), signalSetRefs: [] },
  ];

  for (const invalid of invalidRows) {
    expect(SocialParentPolicyCompilerInputSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsDecisionClaims() {
  const valid = decisionCandidate();
  const invalidRows = [
    { ...valid, finalPolicyDecisionClaimed: true },
    { ...valid, runtimeGateExecutedClaimed: true },
    { ...valid, uiRenderedClaimed: true },
    { ...valid, enforcementClaimed: true },
    { ...valid, nativeAppControlClaimed: true },
    { ...valid, platformConnectorClaimed: true },
    { ...valid, rawSignalPayloadStored: true },
    { ...valid, rawModelTextUsed: true },
    { ...valid, actionCandidate: 'unknown-candidate', fallbackUsed: false },
    { ...valid, actionCandidate: 'manual-review-candidate', fallbackUsed: false },
    { ...valid, actionCandidate: 'parent-review-candidate', parentApprovalRequired: false },
    { ...valid, actionCandidate: 'allow-candidate', reasonCodes: ['social-risk-high'] },
  ];

  for (const invalid of invalidRows) {
    expect(SocialParentPolicyDecisionCandidateSchema.safeParse(invalid).success).toBe(false);
  }
}

function decisionCandidate(overrides = {}) {
  const input = SocialParentPolicyCompilerInputSchema.parse(overridesInput(overrides));
  return compileSocialParentPolicyCandidate({
    input,
    decisionCandidateId: 'social-policy-decision-candidate-video',
    decidedAt: '2026-06-03T06:50:30.000Z',
    expiresAt: '2026-06-03T07:50:30.000Z',
    actionCandidate: 'warn-candidate',
    reasonCodes: ['social-risk-high', 'video-safety-risk'],
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
    compileRequestId: 'social-policy-compile-request-video',
    familyId: 'family-main',
    childProfileId: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    requestedAt: '2026-06-03T06:50:00.000Z',
    policyVersionRef: 'policy-version-2026-06-03',
    targetKind: 'social-video',
    sourceEvidenceRefs: ['parent-evidence-social-video-route', 'parent-evidence-social-signal-set'],
    signalSetRefs: ['social-riskbenefit-signal-set-video'],
    parentRuleRefs: ['parent-rule-school-night-video'],
    scheduleContextRefs: ['schedule-context-school-night'],
    compilerMode: 'contract-only',
    rawSignalPayloadIncluded: false,
    rawModelTextIncluded: false,
    activityDomainObjectIncluded: false,
    finalDecisionClaimedByInput: false,
    runtimeGateClaimedByInput: false,
    uiClaimedByInput: false,
    enforcementClaimedByInput: false,
    nativeAppControlClaimed: false,
    platformConnectorClaimed: false,
    ...overrides,
  };
}
