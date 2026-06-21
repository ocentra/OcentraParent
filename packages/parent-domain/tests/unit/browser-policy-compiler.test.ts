import { describe, expect, it } from 'vitest';
import { PolicyCompilerCapabilityState } from '@ocentra-parent/schema-domain/policy-compiler';
import {
  BrowserGamePolicyCompilerModeSchema,
  BrowserGamePolicyReasonCodeSchema,
  BrowserGamePolicyTargetKindSchema,
} from '@ocentra-parent/schema-domain/browser-game-policy-compiler-values';
import {
  compileBrowserGamePolicyCandidate,
} from '@ocentra-parent/browser-domain/browser-game-policy-compiler';
import {
  BrowserGamePolicyCompilerInputSchema,
  BrowserGamePolicyDecisionCandidateSchema,
} from '@ocentra-parent/schema-domain/browser-game-policy-compiler';
import {
  SocialParentPolicyCompilerModeSchema,
  SocialParentPolicyReasonCodeSchema,
  SocialParentPolicyTargetKindSchema,
} from '@ocentra-parent/schema-domain/social-policy-compiler-values';
import {
  compileSocialParentPolicyCandidate,
} from '@ocentra-parent/browser-domain/social-policy-compiler';
import {
  SocialParentPolicyCompilerInputSchema,
  SocialParentPolicyDecisionCandidateSchema,
} from '@ocentra-parent/schema-domain/social-policy-compiler';

describe('parent-domain browser policy compiler consumers', () => {
  it('uses the browser-game compiler candidate with shared capability state from browser-domain', () => {
    const input = BrowserGamePolicyCompilerInputSchema.parse(browserGamePolicyInput());
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

    expect(decision.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.Supported);
    expect(BrowserGamePolicyDecisionCandidateSchema.safeParse(decision).success).toBe(true);
  });

  it('uses the social compiler candidate with shared capability state from browser-domain', () => {
    const input = SocialParentPolicyCompilerInputSchema.parse(socialPolicyInput());
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

    expect(decision.compilerCapabilityState).toBe(PolicyCompilerCapabilityState.Supported);
    expect(SocialParentPolicyDecisionCandidateSchema.safeParse(decision).success).toBe(true);
  });

  it('uses the browser-game and social compiler value schemas from canonical owners', () => {
    expect(BrowserGamePolicyCompilerModeSchema.parse('contract-only')).toBe('contract-only');
    expect(BrowserGamePolicyTargetKindSchema.parse('cloud-gaming-session')).toBe('cloud-gaming-session');
    expect(BrowserGamePolicyReasonCodeSchema.parse('parent-rule-match')).toBe('parent-rule-match');
    expect(SocialParentPolicyCompilerModeSchema.parse('contract-only')).toBe('contract-only');
    expect(SocialParentPolicyTargetKindSchema.parse('social-video')).toBe('social-video');
    expect(SocialParentPolicyReasonCodeSchema.parse('parent-rule-match')).toBe('parent-rule-match');
  });
});

function browserGamePolicyInput() {
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
  };
}

function socialPolicyInput() {
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
  };
}
