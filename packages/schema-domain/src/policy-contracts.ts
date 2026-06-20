import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import { ParentContractSchemaVersionSchema, ParentPolicyVersionSchema } from './family-reference-primitives';

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
export const PolicyPreviewIdSchema = brandedNonEmptyStringSchema('PolicyPreviewId');

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

export const PolicyActionSchema = withParser(Schema.Literal('allow', 'warn', 'block', 'time-limit', 'ask-parent', 'unknown'));
export const PolicyTargetTypeSchema = withParser(Schema.Literal('app', 'process', 'window', 'domain', 'site', 'category', 'video', 'channel', 'activity-type', 'device'));
export const PolicyScheduleDaySchema = withParser(Schema.Literal('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday'));
export const PolicyDecisionHandoffStateSchema = withParser(Schema.Literal('not-requested', 'disabled', 'pending', 'handed-off'));
export const PermissionRequestStateSchema = withParser(Schema.Literal('open', 'approved', 'denied', 'expired', 'cancelled'));
export const PolicyScheduleBoundaryStateSchema = withParser(Schema.Literal('within-window', 'outside-window', 'dst-gap', 'dst-overlap', 'clock-skew', 'exception-active', 'expired'));
export const PolicyScheduleDstTransitionSchema = withParser(Schema.Literal('spring-forward', 'fall-back'));
export const PolicyScheduleDstResolutionSchema = withParser(Schema.Literal('skip-forward', 'first-occurrence', 'second-occurrence', 'manual-required'));
export const PolicyScheduleClockSourceSchema = withParser(Schema.Literal('child-device', 'trusted-service', 'manual-required'));
export const PolicyScheduleBudgetResetKindSchema = withParser(Schema.Literal('daily', 'weekly', 'monthly'));
export const PolicyScheduleBudgetCarryoverModeSchema = withParser(Schema.Literal('discard-unused', 'carry-forward', 'cap-carryover'));
export const PolicyScheduleOfflineRecoverySchema = withParser(Schema.Literal('resume-remaining', 'recompute-from-journal', 'manual-required'));
export const PolicyScheduleOfflineRecoveryStateSchema = withParser(Schema.Literal('not-needed', 'recovered-from-device', 'recomputed-from-journal', 'manual-required'));
export const PolicyPreviewOriginSchema = withParser(Schema.Literal('parent-preview', 'assistant-preview'));
export const PolicyPreviewConfirmationStateSchema = withParser(Schema.Literal('confirmation-required', 'confirmed'));
export const PolicyPreviewBudgetBoundaryStateSchema = withParser(Schema.Literal('within-budget', 'bonus-time-active', 'bonus-time-expiring', 'manual-required', 'expired'));

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
export const PolicyPreviewSchema = withParser(
  Schema.Struct({
    previewId: PolicyPreviewIdSchema,
    origin: PolicyPreviewOriginSchema,
    confirmationState: PolicyPreviewConfirmationStateSchema,
    confirmedBy: Schema.Union(ParentActorReferenceSchema, Schema.Null),
    confirmedAt: Schema.Union(PolicyTimestampSchema, Schema.Null),
    target: PolicyTargetSchema,
    requestedAction: PolicyActionSchema,
    scheduleBoundary: Schema.Union(PolicyScheduleBoundarySchema, Schema.Null),
    decision: PolicyDecisionSchema,
  })
);

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

export const PolicyAction = {
  Allow: PolicyActionSchema.parse('allow'),
  Warn: PolicyActionSchema.parse('warn'),
  Block: PolicyActionSchema.parse('block'),
  TimeLimit: PolicyActionSchema.parse('time-limit'),
  AskParent: PolicyActionSchema.parse('ask-parent'),
  Unknown: PolicyActionSchema.parse('unknown'),
} as const;
export const PolicyTargetType = {
  App: PolicyTargetTypeSchema.parse('app'),
  Process: PolicyTargetTypeSchema.parse('process'),
  Window: PolicyTargetTypeSchema.parse('window'),
  Domain: PolicyTargetTypeSchema.parse('domain'),
  Site: PolicyTargetTypeSchema.parse('site'),
  Category: PolicyTargetTypeSchema.parse('category'),
  Video: PolicyTargetTypeSchema.parse('video'),
  Channel: PolicyTargetTypeSchema.parse('channel'),
  ActivityType: PolicyTargetTypeSchema.parse('activity-type'),
  Device: PolicyTargetTypeSchema.parse('device'),
} as const;
