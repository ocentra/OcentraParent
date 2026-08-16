const RequiredSocialAppliedScheduleNonClaims = [
  'no-runtime-applied-schedule',
  'no-runtime-applied-time-budget',
  'no-browser-runtime-gate',
  'no-final-policy-execution',
  'no-enforcement',
] as const;

export function socialAppliedScheduleTimeBudgetHasRequiredNonClaims(nonClaims: ReadonlyArray<string>): boolean {
  const values = new Set(nonClaims);
  return RequiredSocialAppliedScheduleNonClaims.every((requiredNonClaim) => values.has(requiredNonClaim));
}

type SocialAppliedScheduleTimeBudgetSummaryRow = {
  readonly applicationState: string;
  readonly runtimeScheduleAppliedClaimed: boolean;
  readonly runtimeTimeBudgetAppliedClaimed: boolean;
  readonly browserRuntimeGateExecutedClaimed: boolean;
  readonly enforcementClaimed: boolean;
};

export function summarizeSocialAppliedScheduleTimeBudgetRows(
  rows: ReadonlyArray<SocialAppliedScheduleTimeBudgetSummaryRow>,
  parentOwnedApplicationEvaluatedState: string,
  manualRequiredState: string
) {
  return {
    totalRows: rows.length,
    parentOwnedApplicationEvaluatedRows: rows.filter(
      (row) => row.applicationState === parentOwnedApplicationEvaluatedState
    ).length,
    manualRequiredRows: rows.filter((row) => row.applicationState === manualRequiredState).length,
    runtimeScheduleAppliedClaimed: rows.some((row) => row.runtimeScheduleAppliedClaimed),
    runtimeTimeBudgetAppliedClaimed: rows.some((row) => row.runtimeTimeBudgetAppliedClaimed),
    browserRuntimeGateExecutedClaimed: rows.some((row) => row.browserRuntimeGateExecutedClaimed),
    enforcementClaimed: rows.some((row) => row.enforcementClaimed),
  };
}
