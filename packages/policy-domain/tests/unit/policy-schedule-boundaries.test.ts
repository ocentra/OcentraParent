import { describe, expect, it } from 'vitest';
import {
  PolicyAction,
  PolicyScheduleBoundaryState,
  PolicyScheduleClockSource,
  PolicyScheduleDstResolution,
  PolicyScheduleDstTransition,
  PolicyScheduleOfflineRecoveryState,
  PolicyPreviewBudgetBoundaryState,
  parsePolicyScheduleBoundary,
  resolvePolicyPreviewBudgetBoundaryState,
} from '../../src/policy';

function createBoundary() {
  return {
    scheduleId: 'school-night',
    timeZone: 'America/Toronto',
    evaluatedAt: '2026-11-01T06:30:00.000Z',
    localTime: '01:30',
    state: PolicyScheduleBoundaryState.WithinWindow,
    dstBoundary: null,
    clockSkew: null,
    exception: null,
    expiry: null,
    timeBudget: null,
  } as const;
}

describe('policy schedule boundary contracts', () => {
  it('parsePolicyScheduleBoundary: accepts fall-back overlap boundaries with an explicit occurrence choice', () => {
    const parsed = parsePolicyScheduleBoundary({
      ...createBoundary(),
      state: PolicyScheduleBoundaryState.DstOverlap,
      dstBoundary: {
        transition: PolicyScheduleDstTransition.FallBack,
        localTime: '01:30',
        offsetBeforeMinutes: -240,
        offsetAfterMinutes: -300,
        resolution: PolicyScheduleDstResolution.FirstOccurrence,
      },
    });

    expect(parsed.state).toBe(PolicyScheduleBoundaryState.DstOverlap);
    expect(parsed.dstBoundary?.resolution).toBe(PolicyScheduleDstResolution.FirstOccurrence);
  });

  it('parsePolicyScheduleBoundary: rejects spring-forward gaps that try to use overlap-only resolution', () => {
    expect(() =>
      parsePolicyScheduleBoundary({
        ...createBoundary(),
        state: PolicyScheduleBoundaryState.DstGap,
        dstBoundary: {
          transition: PolicyScheduleDstTransition.SpringForward,
          localTime: '02:15',
          offsetBeforeMinutes: -300,
          offsetAfterMinutes: -240,
          resolution: PolicyScheduleDstResolution.FirstOccurrence,
        },
      })
    ).toThrow('dst-gap boundaries cannot use overlap-only resolutions');
  });

  it('parsePolicyScheduleBoundary: rejects clock-skew boundaries that stay within the allowed tolerance', () => {
    expect(() =>
      parsePolicyScheduleBoundary({
        ...createBoundary(),
        state: PolicyScheduleBoundaryState.ClockSkew,
        clockSkew: {
          observedAt: '2026-06-13T20:05:00.000Z',
          observedSkewMinutes: 4,
          allowedSkewMinutes: 5,
        },
      })
    ).toThrow('clock-skew boundaries require skew beyond the allowed tolerance');
  });

  it('parsePolicyScheduleBoundary: accepts active exception windows only while they are still live', () => {
    const parsed = parsePolicyScheduleBoundary({
      ...createBoundary(),
      evaluatedAt: '2026-06-13T20:10:00.000Z',
      state: PolicyScheduleBoundaryState.ExceptionActive,
      exception: {
        exceptionId: 'exception-1',
        action: PolicyAction.Allow,
        reasonCode: 'homework-extension',
        startsAt: '2026-06-13T20:00:00.000Z',
        expiresAt: '2026-06-13T20:20:00.000Z',
        createdBy: { actorId: 'parent-1', role: 'parent' },
      },
    });

    expect(parsed.state).toBe(PolicyScheduleBoundaryState.ExceptionActive);
    expect(parsed.exception?.action).toBe(PolicyAction.Allow);
  });

  it('parsePolicyScheduleBoundary: rejects non-expired boundaries evaluated after schedule expiry', () => {
    expect(() =>
      parsePolicyScheduleBoundary({
        ...createBoundary(),
        evaluatedAt: '2026-06-13T22:00:00.000Z',
        expiry: {
          expiresAt: '2026-06-13T21:00:00.000Z',
          expiredAt: '2026-06-13T21:00:00.000Z',
          reasonCode: 'daily-cutoff',
        },
      })
    ).toThrow('non-expired schedule boundaries cannot be evaluated after expiry');
  });

  it('parsePolicyScheduleBoundary: accepts budget status with recovered offline timer state and expiring bonus time', () => {
    const parsed = parsePolicyScheduleBoundary({
      ...createBoundary(),
      evaluatedAt: '2026-06-13T20:10:00.000Z',
      timeBudget: {
        budgetWindowMinutes: 120,
        usedMinutes: 65,
        remainingMinutes: 55,
        carryoverMinutes: 10,
        gracePeriodMinutes: 5,
        resetAt: '2026-06-14T00:00:00.000Z',
        clockSource: PolicyScheduleClockSource.ChildDevice,
        offlineRecovery: {
          state: PolicyScheduleOfflineRecoveryState.RecomputedFromJournal,
          recoveredAt: '2026-06-13T20:09:00.000Z',
          recoveredOfflineMinutes: 12,
        },
        bonusTimeMinutes: 15,
        bonusTimeRemainingMinutes: 8,
        bonusTimeExpiresAt: '2026-06-13T20:25:00.000Z',
      },
    });

    expect(parsed.timeBudget?.offlineRecovery.state).toBe(PolicyScheduleOfflineRecoveryState.RecomputedFromJournal);
    expect(parsed.timeBudget?.bonusTimeMinutes).toBe(15);
    expect(parsed.timeBudget?.bonusTimeRemainingMinutes).toBe(8);
  });

  it('parsePolicyScheduleBoundary: rejects active bonus time without an explicit expiry timestamp', () => {
    expect(() =>
      parsePolicyScheduleBoundary({
        ...createBoundary(),
        evaluatedAt: '2026-06-13T20:10:00.000Z',
        timeBudget: {
          budgetWindowMinutes: 120,
          usedMinutes: 65,
          remainingMinutes: 55,
          carryoverMinutes: 10,
          gracePeriodMinutes: 5,
          resetAt: '2026-06-14T00:00:00.000Z',
          clockSource: PolicyScheduleClockSource.ChildDevice,
          offlineRecovery: {
            state: PolicyScheduleOfflineRecoveryState.NotNeeded,
            recoveredAt: null,
            recoveredOfflineMinutes: 0,
          },
          bonusTimeMinutes: 15,
          bonusTimeRemainingMinutes: 8,
          bonusTimeExpiresAt: null,
        },
      })
    ).toThrow('timeBudget.bonusTimeExpiresAt is required when bonusTimeMinutes are active');
  });

  it('parsePolicyScheduleBoundary: rejects active bonus time that omits remaining preview minutes', () => {
    expect(() =>
      parsePolicyScheduleBoundary({
        ...createBoundary(),
        evaluatedAt: '2026-06-13T20:10:00.000Z',
        timeBudget: {
          budgetWindowMinutes: 120,
          usedMinutes: 65,
          remainingMinutes: 55,
          carryoverMinutes: 10,
          gracePeriodMinutes: 5,
          resetAt: '2026-06-14T00:00:00.000Z',
          clockSource: PolicyScheduleClockSource.ChildDevice,
          offlineRecovery: {
            state: PolicyScheduleOfflineRecoveryState.NotNeeded,
            recoveredAt: null,
            recoveredOfflineMinutes: 0,
          },
          bonusTimeMinutes: 15,
          bonusTimeRemainingMinutes: null,
          bonusTimeExpiresAt: '2026-06-13T20:25:00.000Z',
        },
      })
    ).toThrow('timeBudget.bonusTimeRemainingMinutes is required when bonusTimeMinutes are active');
  });

  it('parsePolicyScheduleBoundary: rejects not-needed offline recovery states that still claim recovered timer state', () => {
    expect(() =>
      parsePolicyScheduleBoundary({
        ...createBoundary(),
        evaluatedAt: '2026-06-13T20:10:00.000Z',
        timeBudget: {
          budgetWindowMinutes: 120,
          usedMinutes: 65,
          remainingMinutes: 55,
          carryoverMinutes: 10,
          gracePeriodMinutes: 5,
          resetAt: '2026-06-14T00:00:00.000Z',
          clockSource: PolicyScheduleClockSource.TrustedService,
          offlineRecovery: {
            state: PolicyScheduleOfflineRecoveryState.NotNeeded,
            recoveredAt: '2026-06-13T20:09:00.000Z',
            recoveredOfflineMinutes: 4,
          },
          bonusTimeMinutes: null,
          bonusTimeRemainingMinutes: null,
          bonusTimeExpiresAt: null,
        },
      })
    ).toThrow('offline recovery state not-needed cannot include recoveredAt');
  });

  it('resolvePolicyPreviewBudgetBoundaryState: marks active bonus time with shrinking minutes as expiring', () => {
    const boundary = parsePolicyScheduleBoundary({
      ...createBoundary(),
      evaluatedAt: '2026-06-13T20:10:00.000Z',
      timeBudget: {
        budgetWindowMinutes: 120,
        usedMinutes: 65,
        remainingMinutes: 55,
        carryoverMinutes: 10,
        gracePeriodMinutes: 5,
        resetAt: '2026-06-14T00:00:00.000Z',
        clockSource: PolicyScheduleClockSource.ChildDevice,
        offlineRecovery: {
          state: PolicyScheduleOfflineRecoveryState.NotNeeded,
          recoveredAt: null,
          recoveredOfflineMinutes: 0,
        },
        bonusTimeMinutes: 15,
        bonusTimeRemainingMinutes: 8,
        bonusTimeExpiresAt: '2026-06-13T20:25:00.000Z',
      },
    });

    expect(resolvePolicyPreviewBudgetBoundaryState(boundary)).toBe(
      PolicyPreviewBudgetBoundaryState.BonusTimeExpiring
    );
  });

  it('resolvePolicyPreviewBudgetBoundaryState: marks manual clock-source preview boundaries as manual-required', () => {
    const boundary = parsePolicyScheduleBoundary({
      ...createBoundary(),
      evaluatedAt: '2026-06-13T20:10:00.000Z',
      timeBudget: {
        budgetWindowMinutes: 120,
        usedMinutes: 65,
        remainingMinutes: 55,
        carryoverMinutes: 10,
        gracePeriodMinutes: 5,
        resetAt: '2026-06-14T00:00:00.000Z',
        clockSource: PolicyScheduleClockSource.ManualRequired,
        offlineRecovery: {
          state: PolicyScheduleOfflineRecoveryState.NotNeeded,
          recoveredAt: null,
          recoveredOfflineMinutes: 0,
        },
        bonusTimeMinutes: null,
        bonusTimeRemainingMinutes: null,
        bonusTimeExpiresAt: null,
      },
    });

    expect(resolvePolicyPreviewBudgetBoundaryState(boundary)).toBe(PolicyPreviewBudgetBoundaryState.ManualRequired);
  });
});
