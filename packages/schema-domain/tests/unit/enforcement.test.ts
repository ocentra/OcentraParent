import { describe, expect, it } from 'vitest';
import {
  EnforcementActionSchema,
  EnforcementAdapterKind,
  EnforcementAdapterResultCode,
  EnforcementCapabilityState,
  EnforcementCapabilityStatusSchema,
  EnforcementMode,
  EnforcementResultSchema,
  EnforcementResultStatus,
  EnforcementRollbackState,
  EnforcementTimerEventKind,
  EnforcementTimerEventSchema,
} from '../../src/enforcement';
import { ParentContractSchemaVersion, ParentPlatform } from '../../src/family-reference-primitives';
import { PolicyAction } from '../../src/policy';

describe('enforcement schema surface', () => {
  it('parses supported capability status and keeps the adapter claim explicit', () => {
    const parsed = EnforcementCapabilityStatusSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      platform: ParentPlatform.Windows,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      capabilityState: EnforcementCapabilityState.Supported,
      permissionState: 'allowed',
      dependencyState: 'installed',
      supportedActions: [EnforcementMode.TerminateProcess],
      degradedReason: null,
      lastCheckedAt: '2026-07-01T19:00:00.000Z',
    });

    expect(parsed.adapterKind).toBe(EnforcementAdapterKind.ProcessControl);
    expect(parsed.capabilityState).toBe(EnforcementCapabilityState.Supported);
  });

  it('rejects degraded capability status without a typed degraded reason', () => {
    expect(() =>
      EnforcementCapabilityStatusSchema.parse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        platform: ParentPlatform.Windows,
        adapterKind: EnforcementAdapterKind.ProcessControl,
        capabilityState: EnforcementCapabilityState.Degraded,
        permissionState: 'allowed',
        dependencyState: 'installed',
        supportedActions: [EnforcementMode.TerminateProcess],
        degradedReason: null,
        lastCheckedAt: '2026-07-01T19:00:00.000Z',
      })
    ).toThrow('Expected unavailable and degraded enforcement capabilities to include typed degraded reason');
  });

  it('rejects unavailable results that omit the typed unavailable status', () => {
    expect(() =>
      EnforcementResultSchema.parse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        resultId: 'result-001',
        actionId: 'action-001',
        status: EnforcementResultStatus.Unavailable,
        adapterResultCode: EnforcementAdapterResultCode.AdapterUnavailable,
        startedAt: '2026-07-01T19:00:00.000Z',
        completedAt: '2026-07-01T19:01:00.000Z',
        rollbackToken: null,
        rollbackState: EnforcementRollbackState.Unavailable,
        unavailableReason: 'adapter-unavailable',
        unavailableStatus: null,
        failedReason: null,
        nextCheckAt: null,
        capability: {
          schemaVersion: ParentContractSchemaVersion.V0_6,
          platform: ParentPlatform.Windows,
          adapterKind: EnforcementAdapterKind.ProcessControl,
          capabilityState: EnforcementCapabilityState.Supported,
          permissionState: 'allowed',
          dependencyState: 'installed',
          supportedActions: [EnforcementMode.TerminateProcess],
          degradedReason: null,
          lastCheckedAt: '2026-07-01T19:00:00.000Z',
        },
      })
    ).toThrow('Expected unavailable enforcement results to include typed unavailable status');
  });
});

describe('enforcement timer and action schema surface', () => {
  it('rejects unavailable timer events that omit the typed unavailable reason', () => {
    expect(() =>
      EnforcementTimerEventSchema.parse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        timerEventId: 'timer-event-001',
        timerEventKind: EnforcementTimerEventKind.Unavailable,
        actionId: 'action-001',
        policyDecisionId: 'decision-001',
        evidenceReferences: [],
        scheduledAt: '2026-07-01T19:00:00.000Z',
        effectiveAt: null,
        rollbackToken: null,
        recoveredAfterRestart: false,
        unavailableReason: null,
      })
    ).toThrow('Expected unavailable and recovery-needed enforcement timer events to include typed unavailable reason');
  });

  it('keeps the enforcement action schema available for policy dispatch callers', () => {
    const parsed = EnforcementActionSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      actionId: 'action-001',
      intentId: 'intent-001',
      policyDecisionId: 'decision-001',
      policyAction: PolicyAction.Warn,
      adapterKind: EnforcementAdapterKind.ProcessControl,
      platform: ParentPlatform.Windows,
      target: {
        targetId: 'target-001',
        targetType: 'app',
        targetValue: 'game-launcher',
      },
      mode: EnforcementMode.ObserveOnly,
      capability: {
        schemaVersion: ParentContractSchemaVersion.V0_6,
        platform: ParentPlatform.Windows,
        adapterKind: EnforcementAdapterKind.ProcessControl,
        capabilityState: EnforcementCapabilityState.Supported,
        permissionState: 'allowed',
        dependencyState: 'installed',
        supportedActions: [EnforcementMode.TerminateProcess],
        degradedReason: null,
        lastCheckedAt: '2026-07-01T19:00:00.000Z',
      },
      reasonCodes: ['policy-dispatch'],
      evidenceReferences: [],
      localAiResultId: null,
      parentApproval: null,
      dryRun: true,
      requestedAt: '2026-07-01T19:00:00.000Z',
      expiresAt: null,
      rollbackToken: null,
    });

    expect(parsed.mode).toBe(EnforcementMode.ObserveOnly);
    expect(parsed.capability.capabilityState).toBe(EnforcementCapabilityState.Supported);
  });
});
