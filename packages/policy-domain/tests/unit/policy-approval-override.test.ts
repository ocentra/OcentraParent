import { describe, expect, it } from 'vitest';
import {
  PolicyApprovalKind,
  PolicyApprovalOrigin,
  PolicyApprovalState,
  PolicyOverrideState,
  PolicyOverrideType,
  resolvePolicyApprovalLifecycle,
} from '../../src/authority';
import {
  PolicyAction,
  PolicyScheduleBoundaryState,
  PolicyScheduleClockSource,
  PolicyScheduleOfflineRecoveryState,
  PolicyTargetType,
} from '../../src/policy';

function createApprovalRequest() {
  return {
    approvalId: 'approval-1',
    permissionRequestId: 'request-1',
    origin: PolicyApprovalOrigin.ChildRequest,
    kind: PolicyApprovalKind.AskParent,
    childProfile: { childProfileId: 'child-1', displayName: 'Sam' },
    device: {
      deviceId: 'device-1',
      childProfileId: 'child-1',
      label: 'Sam Windows PC',
      platform: 'windows',
    },
    requestedTarget: {
      targetId: 'target-1',
      targetType: PolicyTargetType.App,
      targetValue: 'game.exe',
    },
    requestedAction: PolicyAction.Allow,
    requestedAt: '2026-06-13T20:00:00.000Z',
    expiresAt: '2026-06-13T20:15:00.000Z',
    requestedBonusTimeMinutes: null,
    scheduleBoundary: null,
  } as const;
}

function createScheduleBoundary() {
  return {
    scheduleId: 'school-night',
    timeZone: 'America/Toronto',
    evaluatedAt: '2026-06-13T20:05:00.000Z',
    localTime: '16:05',
    state: PolicyScheduleBoundaryState.WithinWindow,
    dstBoundary: null,
    clockSkew: null,
    exception: null,
    expiry: null,
    timeBudget: {
      budgetWindowMinutes: 120,
      usedMinutes: 60,
      remainingMinutes: 60,
      carryoverMinutes: 0,
      gracePeriodMinutes: 10,
      resetAt: '2026-06-14T00:00:00.000Z',
      clockSource: PolicyScheduleClockSource.ChildDevice,
      offlineRecovery: {
        state: PolicyScheduleOfflineRecoveryState.NotNeeded,
        recoveredAt: null,
        recoveredOfflineMinutes: 0,
      },
      bonusTimeMinutes: null,
      bonusTimeRemainingMinutes: null,
      bonusTimeExpiresAt: null,
    },
  } as const;
}

describe('policy approval and override contracts', () => {
  it('resolvePolicyApprovalLifecycle: keeps assistant-drafted actions preview-only until a parent confirms', () => {
    const resolution = resolvePolicyApprovalLifecycle({
      approval: {
        ...createApprovalRequest(),
        origin: PolicyApprovalOrigin.AssistantDraft,
        kind: PolicyApprovalKind.TemporaryOverride,
      },
      state: PolicyApprovalState.PreviewOnly,
      evaluatedAt: '2026-06-13T20:05:00.000Z',
      reviewedBy: null,
      reviewedAt: null,
      auditReferenceId: null,
      override: null,
      replayOfApprovalId: null,
    });

    expect(resolution.state).toBe(PolicyApprovalState.PreviewOnly);
    expect(resolution.approval.origin).toBe(PolicyApprovalOrigin.AssistantDraft);
  });

  it('resolvePolicyApprovalLifecycle: rejects child requests that self-approve the override', () => {
    expect(() =>
      resolvePolicyApprovalLifecycle({
        approval: createApprovalRequest(),
        state: PolicyApprovalState.Approved,
        evaluatedAt: '2026-06-13T20:05:00.000Z',
        reviewedBy: { actorId: 'child-1', role: 'parent' },
        reviewedAt: '2026-06-13T20:04:00.000Z',
        auditReferenceId: 'audit-1',
        override: {
          overrideId: 'override-1',
          overrideType: PolicyOverrideType.TemporaryAllow,
          state: PolicyOverrideState.Active,
          action: PolicyAction.Allow,
          effectiveFrom: '2026-06-13T20:05:00.000Z',
          effectiveUntil: '2026-06-13T20:10:00.000Z',
          bonusTimeMinutes: null,
        },
        replayOfApprovalId: null,
      })
    ).toThrow('child requests cannot self-approve or self-modify');
  });

  it('resolvePolicyApprovalLifecycle: keeps replayed requests from creating extra overrides', () => {
    const resolution = resolvePolicyApprovalLifecycle({
      approval: createApprovalRequest(),
      state: PolicyApprovalState.ReplayRejected,
      evaluatedAt: '2026-06-13T20:06:00.000Z',
      reviewedBy: null,
      reviewedAt: null,
      auditReferenceId: null,
      override: null,
      replayOfApprovalId: 'approval-0',
    });

    expect(resolution.state).toBe(PolicyApprovalState.ReplayRejected);
    expect(resolution.replayOfApprovalId).toBe('approval-0');
  });

  it('resolvePolicyApprovalLifecycle: accepts positive bonus-time grants that stay inside approval and schedule context', () => {
    const resolution = resolvePolicyApprovalLifecycle({
      approval: {
        ...createApprovalRequest(),
        kind: PolicyApprovalKind.BonusTime,
        requestedBonusTimeMinutes: 15,
        scheduleBoundary: createScheduleBoundary(),
      },
      state: PolicyApprovalState.Approved,
      evaluatedAt: '2026-06-13T20:06:00.000Z',
      reviewedBy: { actorId: 'parent-1', role: 'parent' },
      reviewedAt: '2026-06-13T20:05:30.000Z',
      auditReferenceId: 'audit-2',
      override: {
        overrideId: 'override-2',
        overrideType: PolicyOverrideType.BonusTime,
        state: PolicyOverrideState.Active,
        action: PolicyAction.Allow,
        effectiveFrom: '2026-06-13T20:06:00.000Z',
        effectiveUntil: '2026-06-13T20:21:00.000Z',
        bonusTimeMinutes: 15,
      },
      replayOfApprovalId: null,
    });

    expect(resolution.state).toBe(PolicyApprovalState.Approved);
    expect(resolution.override?.bonusTimeMinutes).toBe(15);
  });

  it('resolvePolicyApprovalLifecycle: rejects bonus-time approvals without a positive request amount', () => {
    expect(() =>
      resolvePolicyApprovalLifecycle({
        approval: {
          ...createApprovalRequest(),
          kind: PolicyApprovalKind.BonusTime,
          requestedBonusTimeMinutes: 0,
        },
        state: PolicyApprovalState.Pending,
        evaluatedAt: '2026-06-13T20:05:00.000Z',
        reviewedBy: null,
        reviewedAt: null,
        auditReferenceId: null,
        override: null,
        replayOfApprovalId: null,
      })
    ).toThrow('bonus-time requests must include a positive requestedBonusTimeMinutes value');
  });

  it('resolvePolicyApprovalLifecycle: rejects bonus-time approvals without schedule budget context', () => {
    expect(() =>
      resolvePolicyApprovalLifecycle({
        approval: {
          ...createApprovalRequest(),
          kind: PolicyApprovalKind.BonusTime,
          requestedBonusTimeMinutes: 15,
        },
        state: PolicyApprovalState.Pending,
        evaluatedAt: '2026-06-13T20:05:00.000Z',
        reviewedBy: null,
        reviewedAt: null,
        auditReferenceId: null,
        override: null,
        replayOfApprovalId: null,
      })
    ).toThrow('bonus-time requests must include scheduleBoundary details');
  });

  it('resolvePolicyApprovalLifecycle: marks unreviewed requests as expired once their request window passes', () => {
    const resolution = resolvePolicyApprovalLifecycle({
      approval: createApprovalRequest(),
      state: PolicyApprovalState.ExpiredRequest,
      evaluatedAt: '2026-06-13T20:16:00.000Z',
      reviewedBy: null,
      reviewedAt: null,
      auditReferenceId: null,
      override: null,
      replayOfApprovalId: null,
    });

    expect(resolution.state).toBe(PolicyApprovalState.ExpiredRequest);
  });

  it('resolvePolicyApprovalLifecycle: rejects active overrides that are already past effectiveUntil', () => {
    expect(() =>
      resolvePolicyApprovalLifecycle({
        approval: {
          ...createApprovalRequest(),
          kind: PolicyApprovalKind.TemporaryOverride,
        },
        state: PolicyApprovalState.Approved,
        evaluatedAt: '2026-06-13T20:25:00.000Z',
        reviewedBy: { actorId: 'parent-1', role: 'parent' },
        reviewedAt: '2026-06-13T20:05:00.000Z',
        auditReferenceId: 'audit-3',
        override: {
          overrideId: 'override-3',
          overrideType: PolicyOverrideType.TemporaryAllow,
          state: PolicyOverrideState.Active,
          action: PolicyAction.Allow,
          effectiveFrom: '2026-06-13T20:05:00.000Z',
          effectiveUntil: '2026-06-13T20:20:00.000Z',
          bonusTimeMinutes: null,
        },
        replayOfApprovalId: null,
      })
    ).toThrow('active overrides cannot already be past effectiveUntil');
  });
});
