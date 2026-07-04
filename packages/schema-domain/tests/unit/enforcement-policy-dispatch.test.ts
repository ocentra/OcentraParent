import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

import {
  EnforcementPolicyDispatchApprovalStateSchema,
  EnforcementPolicyDispatchIntentSchema,
  EnforcementPolicyDispatchReadModelSchema,
} from '../../src/enforcement-policy-dispatch';

const InvalidDispatchIntent = {
  schemaVersion: 'v0.6',
  intentId: 'intent-1',
  actor: { actorId: 'parent-1', role: 'parent' },
  device: {
    deviceId: 'device-1',
    childProfileId: 'child-1',
    label: 'Child device',
    platform: 'windows',
  },
  policyDecisionId: 'policy-intent-1',
  policyDecisionRef: 'decision-intent-1',
  policyVersion: 'policy-version-1',
  target: {
    targetId: 'target-1',
    targetType: 'app',
    targetValue: 'game-launcher',
  },
  requestedPolicyAction: 'ask-parent',
  requestedParentAction: 'ask-parent',
  scheduleRef: 'schedule-intent-1',
  evidenceReferences: [],
  approvalRef: null,
  routeRef: 'route-1',
  sourceState: 'ready',
  dryRun: false,
  requestedAt: '2026-07-01T19:00:00.000Z',
} as const;

const InvalidDispatchReadModel = {
  schemaVersion: 'v0.6',
  readModelId: 'dispatch-read-model-1',
  generatedAt: '2026-07-01T19:00:00.000Z',
  entries: [
    {
      schemaVersion: 'v0.6',
      intent: {
        schemaVersion: 'v0.6',
        intentId: 'intent-2',
        actor: { actorId: 'parent-1', role: 'parent' },
        device: {
          deviceId: 'device-1',
          childProfileId: 'child-1',
          label: 'Child device',
          platform: 'windows',
        },
        policyDecisionId: 'policy-intent-2',
        policyDecisionRef: 'decision-intent-2',
        policyVersion: 'policy-version-2',
        target: {
          targetId: 'target-2',
          targetType: 'app',
          targetValue: 'game-launcher',
        },
        requestedPolicyAction: 'block',
        requestedParentAction: 'block-scoped-process',
        scheduleRef: 'schedule-intent-2',
        evidenceReferences: [
          {
            evidenceReferenceId: 'evidence-1',
            kind: 'activity-event',
            observedAt: '2026-07-01T19:00:00.000Z',
          },
        ],
        approvalRef: null,
        routeRef: 'route-2',
        sourceState: 'ready',
        dryRun: false,
        requestedAt: '2026-07-01T19:00:00.000Z',
      },
      matrixRow: {
        matrixId: 'matrix-1',
        surface: 'windows-app-time-limit-lifecycle',
        platform: 'windows',
        adapterKind: 'process-control',
        requestedAction: 'block-scoped-process',
        mode: 'terminate-process',
        capabilityState: 'supported',
        proofLevel: 'implemented',
        outcomeState: 'dispatch-ready',
        rejectionReason: 'none',
        sourceState: 'ready',
        childReasonCode: 'child-reason-time-limit-reached',
      },
      approvalState: 'not-required',
      timerState: 'not-required',
      auditRefs: [],
      timerRefs: [],
      childReasonCode: 'different-child-reason',
      reasonCodes: ['child-reason-time-limit-reached'],
      dispatchedAt: '2026-07-01T19:00:00.000Z',
      nextCheckAt: null,
    },
  ],
} as const;

describe('enforcement policy dispatch schema surface', () => {
  it('stays a schema-only edge contract without a hand-written dispatch fixture', () => {
    const source = readFileSync(new URL('../../src/enforcement-policy-dispatch.ts', import.meta.url), 'utf8');

    expect(source).not.toContain('export const EnforcementPolicyDispatchReadModel =');
    expect(source).not.toContain('dispatch-owned-process-time-limit');
    expect(source).not.toContain('route-localhost-agent-service');
  });

  it('keeps approval state decoding available to authority consumers', () => {
    expect(EnforcementPolicyDispatchApprovalStateSchema.parse('pending')).toBe('pending');
    expect(EnforcementPolicyDispatchApprovalStateSchema.parse('manual-required')).toBe('manual-required');
  });

  it('rejects read models and intents that claim dispatch without required evidence or dry-run gates', () => {
    expect(() => EnforcementPolicyDispatchIntentSchema.parse(InvalidDispatchIntent)).toThrow();
    expect(() => EnforcementPolicyDispatchReadModelSchema.parse(InvalidDispatchReadModel)).toThrow();
  });
});
