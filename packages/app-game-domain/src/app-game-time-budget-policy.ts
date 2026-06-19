import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentActionReferenceSchema, ParentDeviceReferenceSchema, ParentEvidenceReferenceSchema } from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { PolicyRuleIdSchema, PolicyScheduleIdSchema } from '@ocentra-parent/policy-domain/policy';
import {
  AppGameTimeBudgetApprovalState,
  AppGameTimeBudgetBonusState,
  AppGameTimeBudgetDurationSource,
  AppGameTimeBudgetHandoffState,
  AppGameTimeBudgetPeriod,
  AppGameTimeBudgetRecommendedAction,
  AppGameTimeBudgetScheduleState,
  AppGameTimeBudgetSessionKind,
  AppGameTimeBudgetTargetKind,
  AppGameTimeBudgetTimerState,
  appGameTimeBudgetAllSessionsMatchPolicyDevice,
  appGameTimeBudgetBonusApprovalStateMatches,
  appGameTimeBudgetBonusGrantIsConsistent,
  appGameTimeBudgetDecisionBudgetMathIsConsistent,
  appGameTimeBudgetDecisionCountsAreConsistent,
  appGameTimeBudgetRecommendedActionMatchesDecision,
  appGameTimeBudgetTargetAllowsNullRef,
  appGameTimeBudgetTimerStateIsAuditable,
} from './app-game-time-budget-policy-rules';
const NonNegativeTimeBudgetNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value >= 0) || 'Expected a non-negative finite number')
);
const PositiveTimeBudgetNumber = Schema.Number.pipe(
  Schema.filter((value) => (Number.isFinite(value) && value > 0) || 'Expected a positive finite number')
);

export const AppGameTimeBudgetPolicyIdSchema = brandedNonEmptyStringSchema('AppGameTimeBudgetPolicyId');
export const AppGameTimeBudgetDecisionIdSchema = brandedNonEmptyStringSchema('AppGameTimeBudgetDecisionId');
export const AppGameTimeBudgetSessionRefIdSchema = brandedNonEmptyStringSchema('AppGameTimeBudgetSessionRefId');
export const AppGameTimeBudgetTargetRefSchema = brandedNonEmptyStringSchema('AppGameTimeBudgetTargetRef');
export const AppGameTimeBudgetAuditRefSchema = brandedNonEmptyStringSchema('AppGameTimeBudgetAuditRef');
export const AppGameTimeBudgetTimerRefSchema = brandedNonEmptyStringSchema('AppGameTimeBudgetTimerRef');

export const AppGameTimeBudgetTargetKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetTargetKind))
);
export const AppGameTimeBudgetSessionKindSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetSessionKind))
);
export const AppGameTimeBudgetPeriodSchema = withParser(Schema.Literal(...Object.values(AppGameTimeBudgetPeriod)));
export const AppGameTimeBudgetDurationSourceSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetDurationSource))
);
export const AppGameTimeBudgetScheduleStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetScheduleState))
);
export const AppGameTimeBudgetBonusStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetBonusState))
);
export const AppGameTimeBudgetApprovalStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetApprovalState))
);
export const AppGameTimeBudgetTimerStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetTimerState))
);
export const AppGameTimeBudgetRecommendedActionSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetRecommendedAction))
);
export const AppGameTimeBudgetHandoffStateSchema = withParser(
  Schema.Literal(...Object.values(AppGameTimeBudgetHandoffState))
);

export const AppGameTimeBudgetTargetSchema = withParser(
  Schema.Struct({
    targetKind: AppGameTimeBudgetTargetKindSchema,
    targetRef: Schema.Union(AppGameTimeBudgetTargetRefSchema, Schema.Null),
  }).pipe(
    Schema.filter(
      (target) =>
        (appGameTimeBudgetTargetAllowsNullRef(target) && target.targetRef === null) ||
        (!appGameTimeBudgetTargetAllowsNullRef(target) && target.targetRef !== null) ||
        'Expected aggregate targets to omit target refs and concrete targets to include target refs'
    )
  )
);

export const AppGameTimeBudgetSessionReferenceSchema = withParser(
  Schema.Struct({
    sessionRefId: AppGameTimeBudgetSessionRefIdSchema,
    device: ParentDeviceReferenceSchema,
    observedAt: ParentTimestampSchema,
  })
);

export const AppGameTimeBudgetSessionInputSchema = withParser(
  Schema.Struct({
    sessionRef: AppGameTimeBudgetSessionReferenceSchema,
    sessionKind: AppGameTimeBudgetSessionKindSchema,
    targetRef: Schema.Union(AppGameTimeBudgetTargetRefSchema, Schema.Null),
    categoryRef: Schema.Union(AppGameTimeBudgetTargetRefSchema, Schema.Null),
    riskSignalRef: Schema.Union(AppGameTimeBudgetTargetRefSchema, Schema.Null),
    parentAllowedCandidate: Schema.Boolean,
    runningDurationMs: NonNegativeTimeBudgetNumber,
    foregroundDurationMs: NonNegativeTimeBudgetNumber,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  }).pipe(
    Schema.filter(
      (session) =>
        session.evidenceReferences.length > 0 ||
        'Expected app/game time budget session inputs to cite stored session evidence'
    )
  )
);

export const AppGameTimeBudgetBonusGrantSchema = withParser(
  Schema.Struct({
    bonusState: AppGameTimeBudgetBonusStateSchema,
    bonusDurationMs: NonNegativeTimeBudgetNumber,
    approvalRef: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    auditRefs: Schema.Array(AppGameTimeBudgetAuditRefSchema),
  }).pipe(
    Schema.filter(
      (bonusGrant) =>
        appGameTimeBudgetBonusGrantIsConsistent(bonusGrant) ||
        'Expected bonus time to require matching approval and audit proof'
    )
  )
);

export const AppGameTimeBudgetPolicySchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    budgetPolicyId: AppGameTimeBudgetPolicyIdSchema,
    policyVersion: ParentPolicyVersionSchema,
    ruleId: PolicyRuleIdSchema,
    device: ParentDeviceReferenceSchema,
    target: AppGameTimeBudgetTargetSchema,
    period: AppGameTimeBudgetPeriodSchema,
    baseBudgetLimitMs: PositiveTimeBudgetNumber,
    durationSource: AppGameTimeBudgetDurationSourceSchema,
    scheduleRef: PolicyScheduleIdSchema,
    previewEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
  }).pipe(
    Schema.filter(
      (policy) =>
        policy.previewEvidenceReferences.length > 0 || 'Expected app/game time budget policies to cite preview evidence'
    )
  )
);

export const AppGameTimeBudgetDryRunDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    decisionId: AppGameTimeBudgetDecisionIdSchema,
    policy: AppGameTimeBudgetPolicySchema,
    sessions: Schema.Array(AppGameTimeBudgetSessionInputSchema),
    countedSessionRefs: Schema.Array(AppGameTimeBudgetSessionRefIdSchema),
    excludedSessionRefs: Schema.Array(AppGameTimeBudgetSessionRefIdSchema),
    countedDurationMs: NonNegativeTimeBudgetNumber,
    effectiveBudgetLimitMs: PositiveTimeBudgetNumber,
    budgetExceeded: Schema.Boolean,
    scheduleState: AppGameTimeBudgetScheduleStateSchema,
    scheduleEvidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    bonusGrant: AppGameTimeBudgetBonusGrantSchema,
    approvalState: AppGameTimeBudgetApprovalStateSchema,
    recommendedAction: AppGameTimeBudgetRecommendedActionSchema,
    dryRun: Schema.Literal(true),
    enforcementHandoffState: AppGameTimeBudgetHandoffStateSchema,
    timerState: AppGameTimeBudgetTimerStateSchema,
    timerRefs: Schema.Array(AppGameTimeBudgetTimerRefSchema),
    auditRefs: Schema.Array(AppGameTimeBudgetAuditRefSchema),
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    evaluatedAt: ParentTimestampSchema,
  })
    .pipe(
      Schema.filter((decision) => decision.sessions.length > 0 || 'Expected time budget decisions to include sessions')
    )
    .pipe(
      Schema.filter(
        (decision) =>
          decision.scheduleEvidenceReferences.length > 0 ||
          'Expected app/game time budget decisions to cite schedule proof'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          decision.evidenceReferences.length > 0 ||
          'Expected app/game time budget decisions to cite stored session evidence'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGameTimeBudgetAllSessionsMatchPolicyDevice(decision) ||
          'Expected app/game time budget decisions to stay on one policy device'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGameTimeBudgetDecisionCountsAreConsistent(decision) ||
          'Expected counted and excluded app/game sessions to match the policy target and duration source'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGameTimeBudgetBonusApprovalStateMatches(decision.bonusGrant, decision.approvalState) ||
          'Expected bonus time state to match approval state'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGameTimeBudgetDecisionBudgetMathIsConsistent(decision) ||
          'Expected app/game time budget limit and exceeded math to match schedule and bonus time'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGameTimeBudgetRecommendedActionMatchesDecision(decision) ||
          'Expected app/game time budget action to match dry-run, ask-parent, or manual-required state'
      )
    )
    .pipe(
      Schema.filter(
        (decision) =>
          appGameTimeBudgetTimerStateIsAuditable(decision) ||
          'Expected restart-recovered time budget timers to carry timer and audit refs'
      )
    )
);

export type AppGameTimeBudgetTarget = Infer<typeof AppGameTimeBudgetTargetSchema>;
export type AppGameTimeBudgetSessionInput = Infer<typeof AppGameTimeBudgetSessionInputSchema>;
export type AppGameTimeBudgetBonusGrant = Infer<typeof AppGameTimeBudgetBonusGrantSchema>;
export type AppGameTimeBudgetPolicy = Infer<typeof AppGameTimeBudgetPolicySchema>;
export type AppGameTimeBudgetDryRunDecision = Infer<typeof AppGameTimeBudgetDryRunDecisionSchema>;

