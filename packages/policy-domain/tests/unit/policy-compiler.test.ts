import { describe, expect, it } from 'vitest';
import {
  PolicyCompiledArtifactSchema,
  PolicyCompilerCapabilityState,
  PolicyCompilerDomain,
  PolicyCompilerNoClaimLabel,
  PolicyCompilerRuleStatus,
  parsePolicyCompiledArtifact,
} from '@ocentra-parent/schema-domain/policy-compiler';

function sampleArtifact() {
  return {
    compiledArtifactId: 'policy-compiler:browser:policy-source-compiler:5',
    compilerSchemaVersion: 1,
    householdId: 'household-default',
    sourcePolicyVersion: 'policy-v5',
    consumerPolicyVersion: 'policy-v5',
    sourceDocumentId: 'policy-source-compiler',
    sourceStatus: 'confirmed',
    domain: PolicyCompilerDomain.Browser,
    deliveryTarget: {
      childProfileIds: ['child-primary'],
      deviceIds: ['device-laptop'],
      domain: PolicyCompilerDomain.Browser,
    },
    supportMatrix: {
      domain: PolicyCompilerDomain.Browser,
      rows: [
        { targetKind: 'child-profile', capabilityState: PolicyCompilerCapabilityState.ManualRequired },
        { targetKind: 'device', capabilityState: PolicyCompilerCapabilityState.Unsupported },
        { targetKind: 'app', capabilityState: PolicyCompilerCapabilityState.Supported },
        { targetKind: 'site', capabilityState: PolicyCompilerCapabilityState.ManualRequired },
        { targetKind: 'category', capabilityState: PolicyCompilerCapabilityState.Supported },
        { targetKind: 'resource', capabilityState: PolicyCompilerCapabilityState.Unsupported },
      ],
    },
    evidenceCustodyRequirements: {
      exportAllowed: true,
      deleteAllowed: true,
      syncAllowed: false,
    },
    noClaimLabels: [
      PolicyCompilerNoClaimLabel.CompiledArtifactNotSourceTruth,
      PolicyCompilerNoClaimLabel.RuntimeMutationNotClaimed,
      PolicyCompilerNoClaimLabel.EnforcementNotClaimed,
      PolicyCompilerNoClaimLabel.UiDeliveryNotClaimed,
      PolicyCompilerNoClaimLabel.PlatformSupportNotClaimed,
    ],
    auditReferenceIds: ['audit-policy-confirmed'],
    supersededByPolicyVersion: null,
    rollbackRef: null,
    schedules: [
      {
        scheduleId: 'schedule-school-night',
        timeZone: 'America/Toronto',
        startsAt: '21:00',
        endsAt: '07:00',
        timeBudget: {
          budgetWindowMinutes: 120,
          reset: {
            kind: 'daily',
            localTime: '00:00',
            day: null,
          },
          carryover: {
            mode: 'discard-unused',
            maxMinutes: null,
          },
          gracePeriodMinutes: 5,
          effectiveFrom: '2026-01-01T00:00:00.000Z',
          effectiveUntil: null,
          clockSource: 'trusted-service',
          offlineRecovery: 'recompute-from-journal',
        },
      },
    ],
    rules: [
      {
        ruleId: 'rule-site-block',
        target: {
          kind: 'site',
          referenceId: 'site-youtube',
        },
        action: 'block',
        scheduleId: 'schedule-school-night',
        capabilityState: PolicyCompilerCapabilityState.Supported,
        status: PolicyCompilerRuleStatus.Ready,
        reasonCode: null,
      },
      {
        ruleId: 'rule-device-review',
        target: {
          kind: 'device',
          referenceId: 'device-laptop',
        },
        action: 'warn',
        scheduleId: 'schedule-school-night',
        capabilityState: PolicyCompilerCapabilityState.ManualRequired,
        status: PolicyCompilerRuleStatus.ManualRequired,
        reasonCode: 'manual-required-target',
      },
      {
        ruleId: 'rule-resource-unsupported',
        target: {
          kind: 'resource',
          referenceId: 'geofence-school',
        },
        action: 'warn',
        scheduleId: 'schedule-school-night',
        capabilityState: PolicyCompilerCapabilityState.Unsupported,
        status: PolicyCompilerRuleStatus.Unsupported,
        reasonCode: 'unsupported-target',
      },
    ],
  } as const;
}

describe('policy compiler contracts', () => {
  it('parsePolicyCompiledArtifact: parses deterministic compiled artifacts with explicit no-claim and status metadata', () => {
    const parsed = parsePolicyCompiledArtifact(sampleArtifact());
    const repeated = parsePolicyCompiledArtifact(sampleArtifact());

    expect(parsed).toEqual(repeated);
    expect(parsed.domain).toBe(PolicyCompilerDomain.Browser);
    expect(parsed.supportMatrix.rows[2]?.capabilityState).toBe(PolicyCompilerCapabilityState.Supported);
    expect(parsed.rules[1]?.status).toBe(PolicyCompilerRuleStatus.ManualRequired);
    expect(parsed.rules[2]?.status).toBe(PolicyCompilerRuleStatus.Unsupported);
  });

  it('PolicyCompiledArtifactSchema: rejects manual-required or unsupported rules without a reason code', () => {
    const result = PolicyCompiledArtifactSchema.safeParse({
      ...sampleArtifact(),
      rules: [
        {
          ...sampleArtifact().rules[0],
          status: PolicyCompilerRuleStatus.ManualRequired,
          reasonCode: null,
        },
      ],
    });

    expect(result.success).toBe(false);
  });

  it('PolicyCompiledArtifactSchema: rejects rules whose capabilityState and status disagree', () => {
    const result = PolicyCompiledArtifactSchema.safeParse({
      ...sampleArtifact(),
      rules: [
        {
          ...sampleArtifact().rules[0],
          capabilityState: PolicyCompilerCapabilityState.Supported,
          status: PolicyCompilerRuleStatus.ManualRequired,
          reasonCode: 'manual-required-target',
        },
      ],
    });

    expect(result.success).toBe(false);
  });

  it('PolicyCompiledArtifactSchema: rejects compiler artifacts missing the full no-claim set exactly once', () => {
    const result = PolicyCompiledArtifactSchema.safeParse({
      ...sampleArtifact(),
      noClaimLabels: [
        PolicyCompilerNoClaimLabel.CompiledArtifactNotSourceTruth,
        PolicyCompilerNoClaimLabel.CompiledArtifactNotSourceTruth,
      ],
    });

    expect(result.success).toBe(false);
  });

  it('PolicyCompiledArtifactSchema: rejects support matrices that do not classify each target kind exactly once', () => {
    const result = PolicyCompiledArtifactSchema.safeParse({
      ...sampleArtifact(),
      supportMatrix: {
        domain: PolicyCompilerDomain.Browser,
        rows: [
          { targetKind: 'child-profile', capabilityState: PolicyCompilerCapabilityState.ManualRequired },
          { targetKind: 'device', capabilityState: PolicyCompilerCapabilityState.Unsupported },
          { targetKind: 'app', capabilityState: PolicyCompilerCapabilityState.Supported },
          { targetKind: 'site', capabilityState: PolicyCompilerCapabilityState.ManualRequired },
          { targetKind: 'category', capabilityState: PolicyCompilerCapabilityState.Supported },
          { targetKind: 'category', capabilityState: PolicyCompilerCapabilityState.Supported },
        ],
      },
    });

    expect(result.success).toBe(false);
  });

  it('PolicyCompiledArtifactSchema: rejects artifacts that carry supersede and rollback refs together', () => {
    const result = PolicyCompiledArtifactSchema.safeParse({
      ...sampleArtifact(),
      supersededByPolicyVersion: 'policy-v6',
      rollbackRef: {
        householdId: 'household-default',
        rolledBackDocumentId: 'policy-source-compiler',
        rolledBackPolicyVersion: 'policy-v5',
        restoredDocumentId: 'policy-source-previous',
        restoredPolicyVersion: 'policy-v4',
      },
    });

    expect(result.success).toBe(false);
  });
});
