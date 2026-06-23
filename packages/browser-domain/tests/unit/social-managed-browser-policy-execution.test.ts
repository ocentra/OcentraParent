import { describe, expect, it } from 'vitest';
import {
  SocialManagedBrowserPolicyExecutionSchema,
  buildSocialManagedBrowserPolicyExecution,
  summarizeSocialManagedBrowserPolicyExecution,
} from '../../src/social-managed-browser-policy-execution';
import {
  type SocialParentPolicyDecisionCandidate,
  SocialParentPolicyCompilerInputSchema,
} from '@ocentra-parent/schema-domain/social-policy-compiler';
import { compileSocialParentPolicyCandidate } from '../../src/social-policy-candidate-compiler';

describe('social managed browser policy execution', () => {
  it('accepts a managed social video block only when real intervention evidence is present', acceptsManagedExecution);
  it('rejects missing managed intervention evidence for final execution claims', rejectsMissingEvidence);
  it('rejects unmanaged, broad enforcement, provider, native, apple, and raw custody claims', rejectsScopeExpansion);
  it('rejects allow/manual/unknown candidates as managed intervention execution', rejectsNonExecutableCandidates);
});

function acceptsManagedExecution() {
  const execution = managedExecution();
  const summary = summarizeSocialManagedBrowserPolicyExecution(execution);

  expect(execution.sourceDecisionCandidate.actionCandidate).toBe('block-candidate');
  expect(summary.finalPolicyExecutionClaimed).toBe(true);
  expect(summary.browserMutationObserved).toBe(true);
  expect(summary.childInterventionExecuted).toBe(true);
  expect(summary.managedInterventionEnforced).toBe(true);
  expect(summary.broadOsEnforcementClaimed).toBe(false);
  expect(summary.unmanagedBrowserClaimed).toBe(false);
  expect(summary.providerDeliveryAttempted).toBe(false);
  expect(summary.nativeAppControlClaimed).toBe(false);
  expect(summary.applePlatformClaimed).toBe(false);
}

function rejectsMissingEvidence() {
  const execution = managedExecution();
  const invalidRows = [
    { ...execution, managedBrowserInterventionEvidenceRef: null },
    { ...execution, childInterventionEndpointRef: null },
    { ...execution, targetUrlEvidenceRef: null },
    { ...execution, screenshotEvidenceRefs: [] },
    { ...execution, managedSessionObserved: false },
    { ...execution, exactManagedUrlObserved: false },
    { ...execution, liveSurfaceCapturedBeforeMutation: false },
    { ...execution, browserMutationObserved: false },
    { ...execution, childInterventionExecuted: false },
    { ...execution, managedInterventionEnforced: false },
    { ...execution, finalPolicyExecutionClaimed: false },
  ];

  for (const invalid of invalidRows) {
    expect(SocialManagedBrowserPolicyExecutionSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsScopeExpansion() {
  const execution = managedExecution();
  const invalidRows = [
    { ...execution, unmanagedBrowserClaimed: true },
    { ...execution, broadOsEnforcementClaimed: true },
    { ...execution, providerDeliveryAttempted: true },
    { ...execution, nativeAppControlClaimed: true },
    { ...execution, applePlatformClaimed: true },
    { ...execution, rawUrlPersisted: true },
    { ...execution, rawPageContentPersisted: true },
  ];

  for (const invalid of invalidRows) {
    expect(SocialManagedBrowserPolicyExecutionSchema.safeParse(invalid).success).toBe(false);
  }
}

function rejectsNonExecutableCandidates() {
  for (const actionCandidate of ['allow-candidate', 'manual-review-candidate', 'unknown-candidate'] as const) {
    const invalid = {
      ...managedExecution(),
      sourceDecisionCandidate: decisionCandidate(actionCandidate),
    };
    expect(SocialManagedBrowserPolicyExecutionSchema.safeParse(invalid).success).toBe(false);
  }
}

function managedExecution() {
  return buildSocialManagedBrowserPolicyExecution({
    executionId: 'social-managed-browser-policy-execution',
    sourceDecisionCandidate: decisionCandidate('block-candidate'),
    executionEvidenceRefs: [
      'parent-evidence-managed-browser-composited-block',
      'parent-evidence-social-video-policy-candidate',
    ],
    managedBrowserInterventionEvidenceRef: 'parent-evidence-managed-browser-composited-block',
    childInterventionEndpointRef: 'parent-evidence-child-agent-intervention-endpoint',
    targetUrlEvidenceRef: 'parent-evidence-managed-social-video-url-ref',
    screenshotEvidenceRefs: ['parent-evidence-managed-browser-composited-block-screenshot'],
    createdAt: '2026-06-08T22:20:00.000Z',
  });
}

function decisionCandidate(actionCandidate: SocialParentPolicyDecisionCandidate['actionCandidate']) {
  const input = SocialParentPolicyCompilerInputSchema.parse({
    schemaVersion: 'v0.6',
    compileRequestId: 'social-policy-compile-request-managed-video',
    familyId: 'family-main',
    childProfileId: 'child-profile-middle-school',
    deviceId: 'child-device-laptop',
    requestedAt: '2026-06-08T22:18:00.000Z',
    policyVersionRef: 'policy-version-social-video-managed-block',
    targetKind: 'social-video',
    sourceEvidenceRefs: ['parent-evidence-social-video-route'],
    signalSetRefs: ['social-riskbenefit-signal-set-video'],
    parentRuleRefs: ['parent-rule-school-night-video'],
    scheduleContextRefs: ['schedule-context-school-night'],
    timeBudgetContextRefs: ['time-budget-context-social-video-daily'],
    scheduleState: 'outside-allowed-window',
    timeBudgetState: 'budget-low',
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
  });
  const reasonCodes =
    actionCandidate === 'allow-candidate'
      ? ['social-benefit-present']
      : actionCandidate === 'manual-review-candidate'
        ? ['manual-required']
        : actionCandidate === 'unknown-candidate'
          ? ['missing-signal-proof', 'unknown-evidence']
          : ['social-risk-high', 'video-safety-risk', 'parent-rule-match'];

  return compileSocialParentPolicyCandidate({
    input,
    decisionCandidateId: `social-policy-decision-candidate-${actionCandidate}`,
    decidedAt: '2026-06-08T22:18:30.000Z',
    expiresAt: '2026-06-08T23:18:30.000Z',
    actionCandidate,
    reasonCodes,
    confidence: actionCandidate === 'unknown-candidate' ? 'unknown' : 'medium',
    fallbackUsed: actionCandidate === 'manual-review-candidate' || actionCandidate === 'unknown-candidate',
    parentApprovalRequired: actionCandidate === 'parent-review-candidate',
  });
}
