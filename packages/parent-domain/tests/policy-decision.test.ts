import { describe, expect, it } from 'vitest';
import { PermissionRequestSchema, PolicyDecisionHandoffState, PolicyDecisionSchema } from '../src/policy';

const evidenceReference = {
  evidenceReferenceId: 'evidence-1',
  kind: 'activity-event',
  observedAt: '2026-05-20T20:45:00.000Z',
};

describe('parent policy decision contracts', () => {
  it('PermissionRequestSchema and PolicyDecisionSchema: keep approval state and enforcement handoff explicit', () => {
    const permission = PermissionRequestSchema.parse({
      permissionRequestId: 'request-1',
      childProfile: { childProfileId: 'child-1', displayName: 'Sam' },
      device: { deviceId: 'device-1', childProfileId: 'child-1', label: 'Sam Windows PC', platform: 'windows' },
      evidenceReferences: [evidenceReference],
      requestedAction: 'allow',
      requestedTarget: { targetId: 'target-1', targetType: 'domain', targetValue: 'school.example' },
      state: 'open',
      parentAction: null,
      expiresAt: '2026-05-20T21:00:00.000Z',
    });
    const decision = PolicyDecisionSchema.parse({
      schemaVersion: 'v0.6',
      decisionId: 'decision-1',
      action: 'ask-parent',
      reasonCodes: ['permission-required'],
      evidenceReferences: permission.evidenceReferences,
      ruleIds: ['rule-1'],
      localAiResultId: 'ai-result-1',
      dryRun: true,
      enforcementHandoffState: PolicyDecisionHandoffState.Disabled,
      expiresAt: null,
    });

    expect(permission.state).toBe('open');
    expect(decision).toEqual({
      schemaVersion: 'v0.6',
      decisionId: 'decision-1',
      action: 'ask-parent',
      reasonCodes: ['permission-required'],
      evidenceReferences: [evidenceReference],
      ruleIds: ['rule-1'],
      localAiResultId: 'ai-result-1',
      dryRun: true,
      enforcementHandoffState: 'disabled',
      expiresAt: null,
    });
  });
});
