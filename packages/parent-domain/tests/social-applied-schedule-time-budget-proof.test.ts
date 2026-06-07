import { describe, expect, it } from 'vitest';
import {
  SocialAppliedScheduleTimeBudgetProofReadModel,
  SocialAppliedScheduleTimeBudgetRowSchema,
  SocialAppliedScheduleTimeBudgetState,
  summarizeSocialAppliedScheduleTimeBudgetProof,
} from '../src/social-applied-schedule-time-budget-proof';

const ReadyRow = SocialAppliedScheduleTimeBudgetProofReadModel.rows[0];
const ManualRow = SocialAppliedScheduleTimeBudgetProofReadModel.rows[1];

describe('social applied schedule time-budget proof contracts', () => {
  it('accepts parent-owned schedule and budget application rows without runtime claims', () => {
    const parsed = SocialAppliedScheduleTimeBudgetRowSchema.parse(ReadyRow);
    const summary = summarizeSocialAppliedScheduleTimeBudgetProof(SocialAppliedScheduleTimeBudgetProofReadModel);

    expect(parsed.applicationState).toBe(SocialAppliedScheduleTimeBudgetState.ParentOwnedApplicationEvaluated);
    expect(parsed.parentOwnedScheduleWindowEvaluated).toBe(true);
    expect(parsed.parentOwnedTimeBudgetEvaluated).toBe(true);
    expect(parsed.runtimeScheduleAppliedClaimed).toBe(false);
    expect(parsed.runtimeTimeBudgetAppliedClaimed).toBe(false);
    expect(summary).toEqual({
      totalRows: 2,
      parentOwnedApplicationEvaluatedRows: 1,
      manualRequiredRows: 1,
      runtimeScheduleAppliedClaimed: false,
      runtimeTimeBudgetAppliedClaimed: false,
      browserRuntimeGateExecutedClaimed: false,
      enforcementClaimed: false,
    });
  });

  it('keeps manual-required rows out of parent-owned application and runtime handoff claims', () => {
    const parsed = SocialAppliedScheduleTimeBudgetRowSchema.parse(ManualRow);

    expect(parsed.applicationState).toBe(SocialAppliedScheduleTimeBudgetState.ManualRequired);
    expect(parsed.parentOwnedScheduleEvaluationRef).toBe(null);
    expect(parsed.parentOwnedBudgetEvaluationRef).toBe(null);
    expect(parsed.runtimeHandoffRef).toBe(null);
    expect(parsed.manualProofRequirements).toEqual([
      'manual-proof-runtime-social-schedule-budget-application-required',
    ]);
  });

  it('rejects runtime schedule budget final policy gate and enforcement claims', () => {
    for (const invalidRow of [
      {
        ...ReadyRow,
        appliedScheduleTimeBudgetRowId: 'invalid-runtime-schedule-applied',
        runtimeScheduleAppliedClaimed: true,
      },
      {
        ...ReadyRow,
        appliedScheduleTimeBudgetRowId: 'invalid-runtime-budget-applied',
        runtimeTimeBudgetAppliedClaimed: true,
      },
      {
        ...ReadyRow,
        appliedScheduleTimeBudgetRowId: 'invalid-runtime-gate',
        browserRuntimeGateExecutedClaimed: true,
      },
      {
        ...ReadyRow,
        appliedScheduleTimeBudgetRowId: 'invalid-final-policy',
        finalPolicyDecisionClaimed: true,
      },
      { ...ReadyRow, appliedScheduleTimeBudgetRowId: 'invalid-enforcement', enforcementClaimed: true },
    ]) {
      expect(SocialAppliedScheduleTimeBudgetRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });

  it('rejects ready rows that do not match compiler schedule and time-budget state', () => {
    const mismatchedCandidate = SocialAppliedScheduleTimeBudgetRowSchema.safeParse({
      ...ReadyRow,
      appliedScheduleTimeBudgetRowId: 'invalid-mismatched-schedule',
      evaluatedScheduleState: 'inside-allowed-window',
    });
    const missingBudgetEvaluation = SocialAppliedScheduleTimeBudgetRowSchema.safeParse({
      ...ReadyRow,
      appliedScheduleTimeBudgetRowId: 'invalid-missing-budget-evaluation',
      parentOwnedBudgetEvaluationRef: null,
    });

    expect(mismatchedCandidate.success).toBe(false);
    expect(missingBudgetEvaluation.success).toBe(false);
  });
});
