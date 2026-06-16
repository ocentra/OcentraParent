import { describe, expect, it } from 'vitest';
import {
  PolicyCompilerCapabilityState,
  PolicyCompilerDomain,
  PolicyCompilerNoClaimLabel,
  PolicyCompilerRuleStatus,
  parsePolicyCompiledArtifact,
} from '@ocentra-parent/policy-domain/policy-compiler';
import {
  TrackingPolicyCompilerRuntimeProofRequestSchema,
  TrackingPolicyCompilerRuntimeProofResultSchema,
  compileTrackingPolicyRuntimeProofDecision,
} from '@ocentra-parent/parent-domain/tracking-policy-compiler-runtime-proof';
import { TrackingPolicySchemaVersion } from '@ocentra-parent/tracking-domain/tracking-location-policy';

describe('parent-domain tracking policy compiler runtime proof wrapper', () => {
  it('forwards the tracking compiler runtime proof request and result schemas', () => {
    const request = TrackingPolicyCompilerRuntimeProofRequestSchema.parse(requestShapeFor('observe'));
    const result = compileTrackingPolicyRuntimeProofDecision(request);

    expect(result.decision.action).toBe('observe');
    expect(result.parentPolicyFinalAuthority).toBe(true);
    expect(result.runtimeEnforcementClaimed).toBe(false);
    expect(TrackingPolicyCompilerRuntimeProofResultSchema.safeParse(result).success).toBe(true);
  });
});

function requestShapeFor(ruleAction: 'observe') {
  const rule = {
    ...RuleBase,
    action: ruleAction,
  };

  return {
    schemaVersion: RuleBase.schemaVersion,
    requestId: `tracking-policy-compile-${ruleAction}`,
    compiledArtifact: compiledArtifactFor(rule),
    rule,
    requestedAt: '2026-06-05T17:00:00.000Z',
    decidedAt: '2026-06-05T17:01:00.000Z',
    followUpExpiresAt: '2026-06-05T17:11:00.000Z',
    decisionId: `tracking-policy-decision-${ruleAction}`,
    requestedAction: ruleAction,
    compilerMode: 'dry-run',
    evidenceReferences: [EvidenceTrace],
    aiAnalysis: null,
    alertId: null,
    alertSeverity: null,
    checkInId: null,
    escalationId: null,
    liveTrackingGrantId: null,
    liveTrackingDurationSeconds: null,
    parentConfirmationReceived: true,
    freshEvidenceAvailable: true,
    platformManualRequired: false,
    reasonCodes: ['tracking-compiler-requested'],
    auditRefs: ['tracking-compiler-audit'],
  } as const;
}

function compiledArtifactFor(rule: typeof RuleBase) {
  return parsePolicyCompiledArtifact({
    compiledArtifactId: `policy-compiler:tracking:${rule.ruleId}`,
    compilerSchemaVersion: 1,
    householdId: 'tracking-household-1',
    sourcePolicyVersion: rule.policyVersion,
    consumerPolicyVersion: rule.policyVersion,
    sourceDocumentId: 'tracking-policy-source-1',
    sourceStatus: 'confirmed',
    domain: PolicyCompilerDomain.Tracking,
    deliveryTarget: {
      childProfileIds: [rule.childProfileId],
      deviceIds: [rule.deviceId],
      domain: PolicyCompilerDomain.Tracking,
    },
    supportMatrix: {
      domain: PolicyCompilerDomain.Tracking,
      rows: [
        { targetKind: 'child-profile', capabilityState: PolicyCompilerCapabilityState.Supported },
        { targetKind: 'device', capabilityState: PolicyCompilerCapabilityState.Supported },
        { targetKind: 'app', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'site', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'category', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'resource', capabilityState: PolicyCompilerCapabilityState.Supported },
      ],
    },
    evidenceCustodyRequirements: {
      exportAllowed: true,
      deleteAllowed: true,
      syncAllowed: true,
    },
    noClaimLabels: [
      PolicyCompilerNoClaimLabel.CompiledArtifactNotSourceTruth,
      PolicyCompilerNoClaimLabel.RuntimeMutationNotClaimed,
      PolicyCompilerNoClaimLabel.EnforcementNotClaimed,
      PolicyCompilerNoClaimLabel.UiDeliveryNotClaimed,
      PolicyCompilerNoClaimLabel.PlatformSupportNotClaimed,
    ],
    auditReferenceIds: ['tracking-policy-compiler-audit'],
    supersededByPolicyVersion: null,
    rollbackRef: null,
    schedules: [],
    rules: [
      {
        ruleId: rule.ruleId,
        target: {
          kind: 'resource',
          referenceId: `tracking:${rule.targetKind}:${rule.deviceId}`,
        },
        action: 'warn',
        scheduleId: null,
        capabilityState: PolicyCompilerCapabilityState.Supported,
        status: PolicyCompilerRuleStatus.Ready,
        reasonCode: null,
      },
    ],
  });
}

const EvidenceTrace = {
  evidenceReferenceId: 'tracking-policy-evidence-1',
  kind: 'journal-event',
  observedAt: '2026-06-05T17:00:00.000Z',
} as const;

const RuleBase = {
  schemaVersion: TrackingPolicySchemaVersion,
  ruleId: 'tracking-policy-rule-1',
  familyId: 'family-1',
  childProfileId: 'child-1',
  deviceId: 'parent-device-1',
  policyVersion: 'tracking-policy-v3',
  targetKind: 'geofence-transition',
  action: 'observe',
  enabled: true,
  requiresFreshEvidence: true,
  requiresParentConfirmation: false,
  reasonCodes: ['tracking-rule-matched'],
  auditRefs: ['tracking-rule-audit'],
} as const;
