import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from '@ocentra-parent/family-domain/references';
import {
  ParentContractSchemaVersionSchema,
  ParentPolicyVersionSchema,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import { literalSchema, parsedLiteralRecord } from './literal-contracts';

export const PolicyTimestampSchema = brandedNonEmptyStringSchema('PolicyTimestamp');

export const PolicyRuleIdSchema = brandedNonEmptyStringSchema('PolicyRuleId');
export const PolicyScheduleIdSchema = brandedNonEmptyStringSchema('PolicyScheduleId');
export const PolicyTargetIdSchema = brandedNonEmptyStringSchema('PolicyTargetId');
export const PermissionRequestIdSchema = brandedNonEmptyStringSchema('PermissionRequestId');
export const PolicyDecisionIdSchema = brandedNonEmptyStringSchema('PolicyDecisionId');
export const PolicyReasonCodeSchema = brandedNonEmptyStringSchema('PolicyReasonCode');
export const PolicyLocalTimeSchema = brandedNonEmptyStringSchema('PolicyLocalTime');
export const PolicyTimeZoneSchema = brandedNonEmptyStringSchema('PolicyTimeZone');
export const LocalAiResultReferenceIdSchema = brandedNonEmptyStringSchema('LocalAiResultReferenceId');
export const PolicyScheduleExceptionIdSchema = brandedNonEmptyStringSchema('PolicyScheduleExceptionId');

export const PolicyActionLiteral = {
  Allow: 'allow',
  Warn: 'warn',
  Block: 'block',
  TimeLimit: 'time-limit',
  AskParent: 'ask-parent',
  Unknown: 'unknown',
} as const;

export const PolicyTargetTypeLiteral = {
  App: 'app',
  Process: 'process',
  Window: 'window',
  Domain: 'domain',
  Site: 'site',
  Category: 'category',
  Video: 'video',
  Channel: 'channel',
  ActivityType: 'activity-type',
  Device: 'device',
} as const;

export const PolicyScheduleDayLiteral = {
  Monday: 'monday',
  Tuesday: 'tuesday',
  Wednesday: 'wednesday',
  Thursday: 'thursday',
  Friday: 'friday',
  Saturday: 'saturday',
  Sunday: 'sunday',
} as const;

export const PolicyDecisionHandoffStateLiteral = {
  NotRequested: 'not-requested',
  Disabled: 'disabled',
  Pending: 'pending',
  HandedOff: 'handed-off',
} as const;

export const PermissionRequestStateLiteral = {
  Open: 'open',
  Approved: 'approved',
  Denied: 'denied',
  Expired: 'expired',
  Cancelled: 'cancelled',
} as const;

export const PolicyScheduleBoundaryStateLiteral = {
  WithinWindow: 'within-window',
  OutsideWindow: 'outside-window',
  DstGap: 'dst-gap',
  DstOverlap: 'dst-overlap',
  ClockSkew: 'clock-skew',
  ExceptionActive: 'exception-active',
  Expired: 'expired',
} as const;

export const PolicyScheduleDstTransitionLiteral = {
  SpringForward: 'spring-forward',
  FallBack: 'fall-back',
} as const;

export const PolicyScheduleDstResolutionLiteral = {
  SkipForward: 'skip-forward',
  FirstOccurrence: 'first-occurrence',
  SecondOccurrence: 'second-occurrence',
  ManualRequired: 'manual-required',
} as const;

export const PolicyScheduleClockSourceLiteral = {
  ChildDevice: 'child-device',
  TrustedService: 'trusted-service',
  ManualRequired: 'manual-required',
} as const;

export const PolicyScheduleBudgetResetKindLiteral = {
  Daily: 'daily',
  Weekly: 'weekly',
  Monthly: 'monthly',
} as const;

export const PolicyScheduleBudgetCarryoverModeLiteral = {
  DiscardUnused: 'discard-unused',
  CarryForward: 'carry-forward',
  CapCarryover: 'cap-carryover',
} as const;

export const PolicyScheduleOfflineRecoveryLiteral = {
  ResumeRemaining: 'resume-remaining',
  RecomputeFromJournal: 'recompute-from-journal',
  ManualRequired: 'manual-required',
} as const;

export const PolicyScheduleOfflineRecoveryStateLiteral = {
  NotNeeded: 'not-needed',
  RecoveredFromDevice: 'recovered-from-device',
  RecomputedFromJournal: 'recomputed-from-journal',
  ManualRequired: 'manual-required',
} as const;

export const PolicyActionSchema = literalSchema(PolicyActionLiteral);

export const PolicyTargetTypeSchema = literalSchema(PolicyTargetTypeLiteral);

export const PolicyScheduleDaySchema = literalSchema(PolicyScheduleDayLiteral);

export const PolicyDecisionHandoffStateSchema = literalSchema(PolicyDecisionHandoffStateLiteral);

export const PermissionRequestStateSchema = literalSchema(PermissionRequestStateLiteral);

export const PolicyScheduleBoundaryStateSchema = literalSchema(PolicyScheduleBoundaryStateLiteral);

export const PolicyScheduleDstTransitionSchema = literalSchema(PolicyScheduleDstTransitionLiteral);

export const PolicyScheduleDstResolutionSchema = literalSchema(PolicyScheduleDstResolutionLiteral);

export const PolicyScheduleClockSourceSchema = literalSchema(PolicyScheduleClockSourceLiteral);

export const PolicyScheduleBudgetResetKindSchema = literalSchema(PolicyScheduleBudgetResetKindLiteral);

export const PolicyScheduleBudgetCarryoverModeSchema = literalSchema(PolicyScheduleBudgetCarryoverModeLiteral);

export const PolicyScheduleOfflineRecoverySchema = literalSchema(PolicyScheduleOfflineRecoveryLiteral);

export const PolicyScheduleOfflineRecoveryStateSchema = literalSchema(PolicyScheduleOfflineRecoveryStateLiteral);

export const PolicyTargetSchema = withParser(
  Schema.Struct({
    targetId: PolicyTargetIdSchema,
    targetType: PolicyTargetTypeSchema,
    targetValue: brandedNonEmptyStringSchema('PolicyTargetValue'),
  })
);

export const PolicyScheduleWindowSchema = withParser(
  Schema.Struct({
    days: Schema.Array(PolicyScheduleDaySchema),
    startLocalTime: PolicyLocalTimeSchema,
    endLocalTime: PolicyLocalTimeSchema,
  })
);

export const PolicyScheduleBudgetResetSchema = withParser(
  Schema.Struct({
    kind: PolicyScheduleBudgetResetKindSchema,
    localTime: PolicyLocalTimeSchema,
    day: Schema.Union(PolicyScheduleDaySchema, Schema.Null),
  })
);

export const PolicyScheduleBudgetCarryoverSchema = withParser(
  Schema.Struct({
    mode: PolicyScheduleBudgetCarryoverModeSchema,
    maxMinutes: Schema.Union(Schema.Number, Schema.Null),
  })
);

export const PolicyScheduleTimeBudgetSchema = withParser(
  Schema.Struct({
    budgetWindowMinutes: Schema.Number,
    reset: PolicyScheduleBudgetResetSchema,
    carryover: PolicyScheduleBudgetCarryoverSchema,
    gracePeriodMinutes: Schema.Number,
    effectiveFrom: PolicyTimestampSchema,
    effectiveUntil: Schema.Union(PolicyTimestampSchema, Schema.Null),
    clockSource: PolicyScheduleClockSourceSchema,
    offlineRecovery: PolicyScheduleOfflineRecoverySchema,
  })
);

export const PolicyScheduleSchema = withParser(
  Schema.Struct({
    scheduleId: PolicyScheduleIdSchema,
    timeZone: PolicyTimeZoneSchema,
    windows: Schema.Array(PolicyScheduleWindowSchema),
    timeBudget: PolicyScheduleTimeBudgetSchema,
  })
);

export const PolicyScheduleDstBoundarySchema = withParser(
  Schema.Struct({
    transition: PolicyScheduleDstTransitionSchema,
    localTime: PolicyLocalTimeSchema,
    offsetBeforeMinutes: Schema.Number,
    offsetAfterMinutes: Schema.Number,
    resolution: PolicyScheduleDstResolutionSchema,
  })
);

export const PolicyScheduleClockSkewSchema = withParser(
  Schema.Struct({
    observedAt: PolicyTimestampSchema,
    observedSkewMinutes: Schema.Number,
    allowedSkewMinutes: Schema.Number,
  })
);

export const PolicyScheduleExceptionSchema = withParser(
  Schema.Struct({
    exceptionId: PolicyScheduleExceptionIdSchema,
    action: PolicyActionSchema,
    reasonCode: PolicyReasonCodeSchema,
    startsAt: PolicyTimestampSchema,
    expiresAt: PolicyTimestampSchema,
    createdBy: ParentActorReferenceSchema,
  })
);

export const PolicyScheduleExpirySchema = withParser(
  Schema.Struct({
    expiresAt: PolicyTimestampSchema,
    expiredAt: PolicyTimestampSchema,
    reasonCode: PolicyReasonCodeSchema,
  })
);

export const PolicyScheduleOfflineRecoveryStatusSchema = withParser(
  Schema.Struct({
    state: PolicyScheduleOfflineRecoveryStateSchema,
    recoveredAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
    recoveredOfflineMinutes: Schema.Number,
  })
);

export const PolicyScheduleTimeBudgetStatusSchema = withParser(
  Schema.Struct({
    budgetWindowMinutes: Schema.Number,
    usedMinutes: Schema.Number,
    remainingMinutes: Schema.Number,
    carryoverMinutes: Schema.Number,
    gracePeriodMinutes: Schema.Number,
    resetAt: PolicyTimestampSchema,
    clockSource: PolicyScheduleClockSourceSchema,
    offlineRecovery: PolicyScheduleOfflineRecoveryStatusSchema,
    bonusTimeMinutes: Schema.Union(Schema.Number, Schema.Null),
    bonusTimeRemainingMinutes: Schema.Union(Schema.Number, Schema.Null),
    bonusTimeExpiresAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
  })
);

export const PolicyScheduleBoundarySchema = withParser(
  Schema.Struct({
    scheduleId: PolicyScheduleIdSchema,
    timeZone: PolicyTimeZoneSchema,
    evaluatedAt: PolicyTimestampSchema,
    localTime: PolicyLocalTimeSchema,
    state: PolicyScheduleBoundaryStateSchema,
    dstBoundary: Schema.Union(PolicyScheduleDstBoundarySchema, Schema.Null),
    clockSkew: Schema.Union(PolicyScheduleClockSkewSchema, Schema.Null),
    exception: Schema.Union(PolicyScheduleExceptionSchema, Schema.Null),
    expiry: Schema.Union(PolicyScheduleExpirySchema, Schema.Null),
    timeBudget: Schema.Union(PolicyScheduleTimeBudgetStatusSchema, Schema.Null),
  })
);

export const PolicyRuleSchema = withParser(
  Schema.Struct({
    ruleId: PolicyRuleIdSchema,
    target: PolicyTargetSchema,
    action: PolicyActionSchema,
    scheduleId: Schema.Union(PolicyScheduleIdSchema, Schema.Null),
    priority: Schema.Number,
    reasonCode: PolicyReasonCodeSchema,
    createdBy: ParentActorReferenceSchema,
    enabled: Schema.Boolean,
    effectiveFrom: Schema.Union(PolicyTimestampSchema, Schema.Null),
    effectiveUntil: Schema.Union(PolicyTimestampSchema, Schema.Null),
  })
);

export const FamilyPolicySetSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    family: FamilyReferenceSchema,
    childProfiles: Schema.Array(ChildProfileReferenceSchema),
    devices: Schema.Array(ParentDeviceReferenceSchema),
    policyVersion: ParentPolicyVersionSchema,
    rules: Schema.Array(PolicyRuleSchema),
    schedules: Schema.Array(PolicyScheduleSchema),
  })
);

export const PermissionRequestSchema = withParser(
  Schema.Struct({
    permissionRequestId: PermissionRequestIdSchema,
    childProfile: ChildProfileReferenceSchema,
    device: ParentDeviceReferenceSchema,
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    requestedAction: PolicyActionSchema,
    requestedTarget: PolicyTargetSchema,
    state: PermissionRequestStateSchema,
    parentAction: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    expiresAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
  })
);

export const PolicyDecisionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    decisionId: PolicyDecisionIdSchema,
    action: PolicyActionSchema,
    reasonCodes: Schema.Array(PolicyReasonCodeSchema),
    evidenceReferences: Schema.Array(ParentEvidenceReferenceSchema),
    ruleIds: Schema.Array(PolicyRuleIdSchema),
    localAiResultId: Schema.Union(LocalAiResultReferenceIdSchema, Schema.Null),
    dryRun: Schema.Boolean,
    enforcementHandoffState: PolicyDecisionHandoffStateSchema,
    expiresAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
  })
);

export const PolicyPreviewIdSchema = brandedNonEmptyStringSchema('PolicyPreviewId');

export const PolicyPreviewOriginLiteral = {
  ParentPreview: 'parent-preview',
  AssistantPreview: 'assistant-preview',
} as const;

export const PolicyPreviewConfirmationStateLiteral = {
  ConfirmationRequired: 'confirmation-required',
  Confirmed: 'confirmed',
} as const;

export const PolicyPreviewBudgetBoundaryStateLiteral = {
  WithinBudget: 'within-budget',
  BonusTimeActive: 'bonus-time-active',
  BonusTimeExpiring: 'bonus-time-expiring',
  ManualRequired: 'manual-required',
  Expired: 'expired',
} as const;

export const PolicyPreviewOriginSchema = literalSchema(PolicyPreviewOriginLiteral);

export const PolicyPreviewConfirmationStateSchema = literalSchema(PolicyPreviewConfirmationStateLiteral);

export const PolicyPreviewBudgetBoundaryStateSchema = literalSchema(
  PolicyPreviewBudgetBoundaryStateLiteral
);

const PolicyPreviewFields = {
  previewId: PolicyPreviewIdSchema,
  origin: PolicyPreviewOriginSchema,
  confirmationState: PolicyPreviewConfirmationStateSchema,
  confirmedBy: Schema.Union(ParentActorReferenceSchema, Schema.Null),
  confirmedAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
  target: PolicyTargetSchema,
  requestedAction: PolicyActionSchema,
  scheduleBoundary: Schema.Union(PolicyScheduleBoundarySchema, Schema.Null),
  decision: PolicyDecisionSchema,
} as const;

export const PolicyPreviewSchema = withParser(Schema.Struct(PolicyPreviewFields));

export type PolicyRuleId = typeof PolicyRuleIdSchema.Type;
export type PolicyScheduleId = typeof PolicyScheduleIdSchema.Type;
export type PolicyTargetId = typeof PolicyTargetIdSchema.Type;
export type PermissionRequestId = typeof PermissionRequestIdSchema.Type;
export type PolicyDecisionId = typeof PolicyDecisionIdSchema.Type;
export type PolicyReasonCode = typeof PolicyReasonCodeSchema.Type;
export type LocalAiResultReferenceId = typeof LocalAiResultReferenceIdSchema.Type;
export type PolicyScheduleExceptionId = typeof PolicyScheduleExceptionIdSchema.Type;
export type PolicyPreviewId = typeof PolicyPreviewIdSchema.Type;
export type PolicyAction = Infer<typeof PolicyActionSchema>;
export type PolicyTargetType = Infer<typeof PolicyTargetTypeSchema>;
export type PolicyScheduleDay = Infer<typeof PolicyScheduleDaySchema>;
export type PolicyDecisionHandoffState = Infer<typeof PolicyDecisionHandoffStateSchema>;
export type PermissionRequestState = Infer<typeof PermissionRequestStateSchema>;
export type PolicyScheduleBoundaryState = Infer<typeof PolicyScheduleBoundaryStateSchema>;
export type PolicyScheduleDstTransition = Infer<typeof PolicyScheduleDstTransitionSchema>;
export type PolicyScheduleDstResolution = Infer<typeof PolicyScheduleDstResolutionSchema>;
export type PolicyScheduleClockSource = Infer<typeof PolicyScheduleClockSourceSchema>;
export type PolicyScheduleBudgetResetKind = Infer<typeof PolicyScheduleBudgetResetKindSchema>;
export type PolicyScheduleBudgetCarryoverMode = Infer<typeof PolicyScheduleBudgetCarryoverModeSchema>;
export type PolicyScheduleOfflineRecovery = Infer<typeof PolicyScheduleOfflineRecoverySchema>;
export type PolicyScheduleOfflineRecoveryState = Infer<typeof PolicyScheduleOfflineRecoveryStateSchema>;
export type PolicyTarget = Infer<typeof PolicyTargetSchema>;
export type PolicyScheduleWindow = Infer<typeof PolicyScheduleWindowSchema>;
export type PolicyScheduleBudgetReset = Infer<typeof PolicyScheduleBudgetResetSchema>;
export type PolicyScheduleBudgetCarryover = Infer<typeof PolicyScheduleBudgetCarryoverSchema>;
export type PolicyScheduleTimeBudget = Infer<typeof PolicyScheduleTimeBudgetSchema>;
export type PolicySchedule = Infer<typeof PolicyScheduleSchema>;
export type PolicyScheduleDstBoundary = Infer<typeof PolicyScheduleDstBoundarySchema>;
export type PolicyScheduleClockSkew = Infer<typeof PolicyScheduleClockSkewSchema>;
export type PolicyScheduleException = Infer<typeof PolicyScheduleExceptionSchema>;
export type PolicyScheduleExpiry = Infer<typeof PolicyScheduleExpirySchema>;
export type PolicyScheduleOfflineRecoveryStatus = Infer<typeof PolicyScheduleOfflineRecoveryStatusSchema>;
export type PolicyScheduleTimeBudgetStatus = Infer<typeof PolicyScheduleTimeBudgetStatusSchema>;
export type PolicyScheduleBoundary = Infer<typeof PolicyScheduleBoundarySchema>;
export type PolicyRule = Infer<typeof PolicyRuleSchema>;
export type FamilyPolicySet = Infer<typeof FamilyPolicySetSchema>;
export type PermissionRequest = Infer<typeof PermissionRequestSchema>;
export type PolicyDecision = Infer<typeof PolicyDecisionSchema>;
export type PolicyPreviewOrigin = Infer<typeof PolicyPreviewOriginSchema>;
export type PolicyPreviewConfirmationState = Infer<typeof PolicyPreviewConfirmationStateSchema>;
export type PolicyPreviewBudgetBoundaryState = Infer<typeof PolicyPreviewBudgetBoundaryStateSchema>;
export type PolicyPreview = Infer<typeof PolicyPreviewSchema>;

export const PolicyAction = parsedLiteralRecord(PolicyActionLiteral, (value) =>
  PolicyActionSchema.parse(value)
);

export const PolicyTargetType = parsedLiteralRecord(PolicyTargetTypeLiteral, (value) =>
  PolicyTargetTypeSchema.parse(value)
);

export const PolicyScheduleDay = parsedLiteralRecord(PolicyScheduleDayLiteral, (value) =>
  PolicyScheduleDaySchema.parse(value)
);

export const PolicyDecisionHandoffState = parsedLiteralRecord(PolicyDecisionHandoffStateLiteral, (value) =>
  PolicyDecisionHandoffStateSchema.parse(value)
);

export const PermissionRequestState = parsedLiteralRecord(PermissionRequestStateLiteral, (value) =>
  PermissionRequestStateSchema.parse(value)
);

export const PolicyScheduleBoundaryState = parsedLiteralRecord(PolicyScheduleBoundaryStateLiteral, (value) =>
  PolicyScheduleBoundaryStateSchema.parse(value)
);

export const PolicyScheduleDstTransition = parsedLiteralRecord(PolicyScheduleDstTransitionLiteral, (value) =>
  PolicyScheduleDstTransitionSchema.parse(value)
);

export const PolicyScheduleDstResolution = parsedLiteralRecord(PolicyScheduleDstResolutionLiteral, (value) =>
  PolicyScheduleDstResolutionSchema.parse(value)
);

export const PolicyScheduleClockSource = parsedLiteralRecord(PolicyScheduleClockSourceLiteral, (value) =>
  PolicyScheduleClockSourceSchema.parse(value)
);

export const PolicyScheduleBudgetResetKind = parsedLiteralRecord(PolicyScheduleBudgetResetKindLiteral, (value) =>
  PolicyScheduleBudgetResetKindSchema.parse(value)
);

export const PolicyScheduleBudgetCarryoverMode = parsedLiteralRecord(
  PolicyScheduleBudgetCarryoverModeLiteral,
  (value) => PolicyScheduleBudgetCarryoverModeSchema.parse(value)
);

export const PolicyScheduleOfflineRecovery = parsedLiteralRecord(PolicyScheduleOfflineRecoveryLiteral, (value) =>
  PolicyScheduleOfflineRecoverySchema.parse(value)
);

export const PolicyScheduleOfflineRecoveryState = parsedLiteralRecord(
  PolicyScheduleOfflineRecoveryStateLiteral,
  (value) => PolicyScheduleOfflineRecoveryStateSchema.parse(value)
);

export const PolicyPreviewOrigin = parsedLiteralRecord(PolicyPreviewOriginLiteral, (value) =>
  PolicyPreviewOriginSchema.parse(value)
);

export const PolicyPreviewConfirmationState = parsedLiteralRecord(
  PolicyPreviewConfirmationStateLiteral,
  (value) => PolicyPreviewConfirmationStateSchema.parse(value)
);

export const PolicyPreviewBudgetBoundaryState = parsedLiteralRecord(
  PolicyPreviewBudgetBoundaryStateLiteral,
  (value) => PolicyPreviewBudgetBoundaryStateSchema.parse(value)
);

export const PolicyActionStrictnessRank = Object.freeze(
  Object.fromEntries([
    [PolicyAction.Allow, 0],
    [PolicyAction.Warn, 10],
    [PolicyAction.Unknown, 20],
    [PolicyAction.AskParent, 30],
    [PolicyAction.TimeLimit, 40],
    [PolicyAction.Block, 50],
  ] as const)
) as Readonly<Record<PolicyAction, number>>;

function assertPolicyContract(condition: unknown, message: string): asserts condition {
  if (!condition) {
    throw new Error(message);
  }
}

function parsePolicyTimestampMillis(timestamp: string, fieldName: string): number {
  const millis = Date.parse(timestamp);
  assertPolicyContract(!Number.isNaN(millis), `${fieldName} must be an ISO-8601 timestamp`);
  return millis;
}

function parsePolicyLocalTimeMinutes(localTime: string, fieldName: string): number {
  const match = /^(?<hour>[01]\d|2[0-3]):(?<minute>[0-5]\d)$/.exec(localTime);
  assertPolicyContract(match !== null, `${fieldName} must use HH:MM 24-hour local time`);
  const hour = Number(match.groups?.['hour'] ?? '0');
  const minute = Number(match.groups?.['minute'] ?? '0');
  return hour * 60 + minute;
}

function assertPolicyNonNegativeNumber(value: number, fieldName: string): void {
  assertPolicyContract(Number.isFinite(value) && value >= 0, `${fieldName} must be a non-negative number`);
}

function assertPolicyPositiveNumber(value: number, fieldName: string): void {
  assertPolicyContract(Number.isFinite(value) && value > 0, `${fieldName} must be a positive number`);
}

function validatePolicyScheduleTimeBudget(timeBudget: PolicyScheduleTimeBudget): void {
  assertPolicyPositiveNumber(timeBudget.budgetWindowMinutes, 'timeBudget.budgetWindowMinutes');
  assertPolicyNonNegativeNumber(timeBudget.gracePeriodMinutes, 'timeBudget.gracePeriodMinutes');
  parsePolicyLocalTimeMinutes(timeBudget.reset.localTime, 'timeBudget.reset.localTime');

  const effectiveFrom = parsePolicyTimestampMillis(timeBudget.effectiveFrom, 'timeBudget.effectiveFrom');
  if (timeBudget.effectiveUntil !== null) {
    const effectiveUntil = parsePolicyTimestampMillis(timeBudget.effectiveUntil, 'timeBudget.effectiveUntil');
    assertPolicyContract(
      effectiveUntil > effectiveFrom,
      'timeBudget.effectiveUntil must be after timeBudget.effectiveFrom'
    );
  }

  if (timeBudget.reset.kind === PolicyScheduleBudgetResetKind.Weekly) {
    assertPolicyContract(timeBudget.reset.day !== null, 'weekly reset rules require timeBudget.reset.day');
  } else {
    assertPolicyContract(timeBudget.reset.day === null, 'non-weekly reset rules cannot set timeBudget.reset.day');
  }

  switch (timeBudget.carryover.mode) {
    case PolicyScheduleBudgetCarryoverMode.DiscardUnused:
      assertPolicyContract(
        timeBudget.carryover.maxMinutes === null,
        'discard-unused carryover cannot set timeBudget.carryover.maxMinutes'
      );
      break;
    case PolicyScheduleBudgetCarryoverMode.CarryForward:
      if (timeBudget.carryover.maxMinutes !== null) {
        assertPolicyPositiveNumber(timeBudget.carryover.maxMinutes, 'timeBudget.carryover.maxMinutes');
      }
      break;
    case PolicyScheduleBudgetCarryoverMode.CapCarryover:
      assertPolicyContract(
        timeBudget.carryover.maxMinutes !== null,
        'cap-carryover requires timeBudget.carryover.maxMinutes'
      );
      assertPolicyPositiveNumber(timeBudget.carryover.maxMinutes ?? 0, 'timeBudget.carryover.maxMinutes');
      break;
  }
}

function validatePolicyScheduleTimeBudgetStatus(timeBudget: PolicyScheduleTimeBudgetStatus, evaluatedAt: number): void {
  assertPolicyPositiveNumber(timeBudget.budgetWindowMinutes, 'timeBudget.budgetWindowMinutes');
  assertPolicyNonNegativeNumber(timeBudget.usedMinutes, 'timeBudget.usedMinutes');
  assertPolicyNonNegativeNumber(timeBudget.remainingMinutes, 'timeBudget.remainingMinutes');
  assertPolicyNonNegativeNumber(timeBudget.carryoverMinutes, 'timeBudget.carryoverMinutes');
  assertPolicyNonNegativeNumber(timeBudget.gracePeriodMinutes, 'timeBudget.gracePeriodMinutes');

  const resetAt = parsePolicyTimestampMillis(timeBudget.resetAt, 'timeBudget.resetAt');
  assertPolicyContract(resetAt > evaluatedAt, 'timeBudget.resetAt must be after evaluatedAt');

  switch (timeBudget.offlineRecovery.state) {
    case PolicyScheduleOfflineRecoveryState.NotNeeded:
      assertPolicyContract(
        timeBudget.offlineRecovery.recoveredAt === null,
        'offline recovery state not-needed cannot include recoveredAt'
      );
      assertPolicyContract(
        timeBudget.offlineRecovery.recoveredOfflineMinutes === 0,
        'offline recovery state not-needed cannot recover offline minutes'
      );
      break;
    case PolicyScheduleOfflineRecoveryState.RecoveredFromDevice:
    case PolicyScheduleOfflineRecoveryState.RecomputedFromJournal:
      assertPolicyContract(
        timeBudget.offlineRecovery.recoveredAt !== null,
        'recovered offline timer states require recoveredAt'
      );
      parsePolicyTimestampMillis(timeBudget.offlineRecovery.recoveredAt ?? '', 'offlineRecovery.recoveredAt');
      assertPolicyNonNegativeNumber(
        timeBudget.offlineRecovery.recoveredOfflineMinutes,
        'offlineRecovery.recoveredOfflineMinutes'
      );
      break;
    case PolicyScheduleOfflineRecoveryState.ManualRequired:
      if (timeBudget.offlineRecovery.recoveredAt !== null) {
        parsePolicyTimestampMillis(timeBudget.offlineRecovery.recoveredAt, 'offlineRecovery.recoveredAt');
      }
      assertPolicyNonNegativeNumber(
        timeBudget.offlineRecovery.recoveredOfflineMinutes,
        'offlineRecovery.recoveredOfflineMinutes'
      );
      break;
  }

  if (timeBudget.bonusTimeMinutes === null) {
    assertPolicyContract(
      timeBudget.bonusTimeRemainingMinutes === null,
      'timeBudget.bonusTimeRemainingMinutes requires bonusTimeMinutes'
    );
    assertPolicyContract(
      timeBudget.bonusTimeExpiresAt === null,
      'timeBudget.bonusTimeExpiresAt requires bonusTimeMinutes'
    );
    return;
  }

  assertPolicyPositiveNumber(timeBudget.bonusTimeMinutes, 'timeBudget.bonusTimeMinutes');
  assertPolicyContract(
    timeBudget.bonusTimeRemainingMinutes !== null,
    'timeBudget.bonusTimeRemainingMinutes is required when bonusTimeMinutes are active'
  );
  assertPolicyNonNegativeNumber(
    timeBudget.bonusTimeRemainingMinutes ?? 0,
    'timeBudget.bonusTimeRemainingMinutes'
  );
  assertPolicyContract(
    (timeBudget.bonusTimeRemainingMinutes ?? 0) <= timeBudget.bonusTimeMinutes,
    'timeBudget.bonusTimeRemainingMinutes cannot exceed timeBudget.bonusTimeMinutes'
  );
  assertPolicyContract(
    timeBudget.bonusTimeExpiresAt !== null,
    'timeBudget.bonusTimeExpiresAt is required when bonusTimeMinutes are active'
  );
  const bonusTimeExpiresAt = parsePolicyTimestampMillis(
    timeBudget.bonusTimeExpiresAt ?? '',
    'timeBudget.bonusTimeExpiresAt'
  );
  assertPolicyContract(
    bonusTimeExpiresAt > evaluatedAt,
    'timeBudget.bonusTimeExpiresAt must be after evaluatedAt while bonus time is active'
  );
}

function validatePolicyPreview(preview: PolicyPreview): void {
  const hasConfirmedBy = preview.confirmedBy !== null;
  const hasConfirmedAt = preview.confirmedAt !== null;

  assertPolicyContract(
    hasConfirmedBy === hasConfirmedAt,
    'preview confirmation requires both confirmedBy and confirmedAt together'
  );
  assertPolicyContract(preview.decision.dryRun, 'preview decisions must remain dry-run');
  assertPolicyContract(
    preview.decision.enforcementHandoffState === PolicyDecisionHandoffState.Disabled,
    'preview decisions must keep enforcement handoff disabled'
  );

  switch (preview.confirmationState) {
    case PolicyPreviewConfirmationState.ConfirmationRequired:
      assertPolicyContract(
        preview.confirmedBy === null && preview.confirmedAt === null,
        'confirmation-required previews cannot include confirmedBy or confirmedAt'
      );
      break;
    case PolicyPreviewConfirmationState.Confirmed:
      assertPolicyContract(preview.confirmedBy !== null, 'confirmed previews require confirmedBy');
      assertPolicyContract(preview.confirmedAt !== null, 'confirmed previews require confirmedAt');
      break;
  }
}

function validatePolicySchedule(schedule: PolicySchedule): void {
  assertPolicyContract(schedule.windows.length > 0, 'schedules must define at least one local window');
  for (const [index, window] of schedule.windows.entries()) {
    assertPolicyContract(window.days.length > 0, `schedules must define at least one day for windows[${index}]`);
    parsePolicyLocalTimeMinutes(window.startLocalTime, `windows[${index}].startLocalTime`);
    parsePolicyLocalTimeMinutes(window.endLocalTime, `windows[${index}].endLocalTime`);
  }
  validatePolicyScheduleTimeBudget(schedule.timeBudget);
}

function validatePolicyScheduleBoundary(boundary: PolicyScheduleBoundary): void {
  const evaluatedAt = parsePolicyTimestampMillis(boundary.evaluatedAt, 'evaluatedAt');
  parsePolicyLocalTimeMinutes(boundary.localTime, 'localTime');

  if (boundary.dstBoundary !== null) {
    parsePolicyLocalTimeMinutes(boundary.dstBoundary.localTime, 'dstBoundary.localTime');
    assertPolicyContract(
      Number.isInteger(boundary.dstBoundary.offsetBeforeMinutes),
      'dstBoundary.offsetBeforeMinutes must be an integer minute offset'
    );
    assertPolicyContract(
      Number.isInteger(boundary.dstBoundary.offsetAfterMinutes),
      'dstBoundary.offsetAfterMinutes must be an integer minute offset'
    );
  }

  if (boundary.clockSkew !== null) {
    parsePolicyTimestampMillis(boundary.clockSkew.observedAt, 'clockSkew.observedAt');
    assertPolicyContract(
      Number.isFinite(boundary.clockSkew.allowedSkewMinutes) && boundary.clockSkew.allowedSkewMinutes >= 0,
      'clockSkew.allowedSkewMinutes must be a non-negative number'
    );
    assertPolicyContract(
      Number.isFinite(boundary.clockSkew.observedSkewMinutes),
      'clockSkew.observedSkewMinutes must be a finite number'
    );
  }

  if (boundary.exception !== null) {
    const exceptionStartsAt = parsePolicyTimestampMillis(boundary.exception.startsAt, 'exception.startsAt');
    const exceptionExpiresAt = parsePolicyTimestampMillis(boundary.exception.expiresAt, 'exception.expiresAt');
    assertPolicyContract(exceptionExpiresAt > exceptionStartsAt, 'schedule exceptions must expire after they start');
  }

  if (boundary.expiry !== null) {
    const expiresAt = parsePolicyTimestampMillis(boundary.expiry.expiresAt, 'expiry.expiresAt');
    const expiredAt = parsePolicyTimestampMillis(boundary.expiry.expiredAt, 'expiry.expiredAt');
    assertPolicyContract(expiredAt >= expiresAt, 'expiry.expiredAt must be on or after expiry.expiresAt');
    if (boundary.state !== PolicyScheduleBoundaryState.Expired) {
      assertPolicyContract(evaluatedAt < expiresAt, 'non-expired schedule boundaries cannot be evaluated after expiry');
    }
  }

  if (boundary.timeBudget !== null) {
    validatePolicyScheduleTimeBudgetStatus(boundary.timeBudget, evaluatedAt);
  }

  switch (boundary.state) {
    case PolicyScheduleBoundaryState.DstGap: {
      assertPolicyContract(boundary.dstBoundary !== null, 'dst-gap boundaries require dstBoundary details');
      assertPolicyContract(
        boundary.dstBoundary.transition === PolicyScheduleDstTransition.SpringForward,
        'dst-gap boundaries must use the spring-forward transition'
      );
      assertPolicyContract(
        boundary.dstBoundary.resolution !== PolicyScheduleDstResolution.FirstOccurrence &&
          boundary.dstBoundary.resolution !== PolicyScheduleDstResolution.SecondOccurrence,
        'dst-gap boundaries cannot use overlap-only resolutions'
      );
      break;
    }
    case PolicyScheduleBoundaryState.DstOverlap: {
      assertPolicyContract(boundary.dstBoundary !== null, 'dst-overlap boundaries require dstBoundary details');
      assertPolicyContract(
        boundary.dstBoundary.transition === PolicyScheduleDstTransition.FallBack,
        'dst-overlap boundaries must use the fall-back transition'
      );
      assertPolicyContract(
        boundary.dstBoundary.resolution !== PolicyScheduleDstResolution.SkipForward,
        'dst-overlap boundaries cannot skip the repeated hour'
      );
      break;
    }
    case PolicyScheduleBoundaryState.ClockSkew: {
      assertPolicyContract(boundary.clockSkew !== null, 'clock-skew boundaries require clockSkew details');
      assertPolicyContract(
        Math.abs(boundary.clockSkew.observedSkewMinutes) > boundary.clockSkew.allowedSkewMinutes,
        'clock-skew boundaries require skew beyond the allowed tolerance'
      );
      break;
    }
    case PolicyScheduleBoundaryState.ExceptionActive: {
      assertPolicyContract(boundary.exception !== null, 'exception-active boundaries require exception details');
      const exceptionStartsAt = parsePolicyTimestampMillis(boundary.exception.startsAt, 'exception.startsAt');
      const exceptionExpiresAt = parsePolicyTimestampMillis(boundary.exception.expiresAt, 'exception.expiresAt');
      assertPolicyContract(
        evaluatedAt >= exceptionStartsAt && evaluatedAt < exceptionExpiresAt,
        'exception-active boundaries must be evaluated inside the exception window'
      );
      break;
    }
    case PolicyScheduleBoundaryState.Expired: {
      assertPolicyContract(boundary.expiry !== null, 'expired schedule boundaries require expiry details');
      const expiresAt = parsePolicyTimestampMillis(boundary.expiry.expiresAt, 'expiry.expiresAt');
      assertPolicyContract(
        evaluatedAt >= expiresAt,
        'expired schedule boundaries must be evaluated on or after expiry'
      );
      break;
    }
    default:
      break;
  }
}

export function parsePolicySchedule(input: unknown): PolicySchedule {
  const schedule = PolicyScheduleSchema.parse(input);
  validatePolicySchedule(schedule);
  return schedule;
}

export function parseFamilyPolicySet(input: unknown): FamilyPolicySet {
  const policySet = FamilyPolicySetSchema.parse(input);
  for (const schedule of policySet.schedules) {
    validatePolicySchedule(schedule);
  }
  return policySet;
}

export function parsePolicyScheduleBoundary(input: unknown): PolicyScheduleBoundary {
  const boundary = PolicyScheduleBoundarySchema.parse(input);
  validatePolicyScheduleBoundary(boundary);
  return boundary;
}

export function parsePolicyPreview(input: unknown): PolicyPreview {
  const preview = PolicyPreviewSchema.parse(input);
  validatePolicyPreview(preview);
  return preview;
}

export function resolvePolicyPreviewBudgetBoundaryState(
  boundary: PolicyScheduleBoundary | null
): PolicyPreviewBudgetBoundaryState {
  if (boundary === null) {
    return PolicyPreviewBudgetBoundaryState.WithinBudget;
  }

  if (
    boundary.state === PolicyScheduleBoundaryState.ClockSkew ||
    (boundary.state === PolicyScheduleBoundaryState.DstGap &&
      boundary.dstBoundary?.resolution === PolicyScheduleDstResolution.ManualRequired) ||
    (boundary.state === PolicyScheduleBoundaryState.DstOverlap &&
      boundary.dstBoundary?.resolution === PolicyScheduleDstResolution.ManualRequired) ||
    boundary.timeBudget?.clockSource === PolicyScheduleClockSource.ManualRequired ||
    boundary.timeBudget?.offlineRecovery.state === PolicyScheduleOfflineRecoveryState.ManualRequired
  ) {
    return PolicyPreviewBudgetBoundaryState.ManualRequired;
  }

  if (boundary.state === PolicyScheduleBoundaryState.Expired) {
    return PolicyPreviewBudgetBoundaryState.Expired;
  }

  if (boundary.timeBudget?.bonusTimeMinutes !== null && boundary.timeBudget?.bonusTimeMinutes !== undefined) {
    const bonusTimeRemainingMinutes =
      boundary.timeBudget.bonusTimeRemainingMinutes ?? boundary.timeBudget.bonusTimeMinutes;
    return bonusTimeRemainingMinutes < boundary.timeBudget.bonusTimeMinutes
      ? PolicyPreviewBudgetBoundaryState.BonusTimeExpiring
      : PolicyPreviewBudgetBoundaryState.BonusTimeActive;
  }

  return PolicyPreviewBudgetBoundaryState.WithinBudget;
}

export function comparePolicyActionStrictness(left: PolicyAction, right: PolicyAction): number {
  return PolicyActionStrictnessRank[left] - PolicyActionStrictnessRank[right];
}

export function selectStricterPolicyAction(parentRuleAction: PolicyAction, localAiAction: PolicyAction): PolicyAction {
  return comparePolicyActionStrictness(parentRuleAction, localAiAction) >= 0 ? parentRuleAction : localAiAction;
}
