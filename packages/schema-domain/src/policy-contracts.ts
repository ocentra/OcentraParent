/* thin adapter over Rust-owned generated policy contracts */

import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import {
  GeneratedPolicyActionValues,
  GeneratedPermissionRequestStateValues,
  GeneratedPolicyDecisionHandoffStateValues,
  GeneratedPolicyPreviewBudgetBoundaryStateValues,
  GeneratedPolicyPreviewConfirmationStateValues,
  GeneratedPolicyScheduleBoundaryStateValues,
  GeneratedPolicyScheduleBudgetCarryoverModeValues,
  GeneratedPolicyScheduleBudgetResetKindValues,
  GeneratedPolicyScheduleClockSourceValues,
  GeneratedPolicyScheduleDstResolutionValues,
  GeneratedPolicyScheduleDstTransitionValues,
  GeneratedPolicyScheduleOfflineRecoveryStateValues,
  GeneratedPolicyScheduleOfflineRecoveryValues,
  GeneratedPolicyPreviewOriginValues,
  GeneratedPolicyTargetTypeValues,
  GeneratedPolicyScheduleDayValues,
} from './generated-policy-control-helpers-contracts';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './family-references';
import { ParentContractSchemaVersionSchema, ParentPolicyVersionSchema } from './family-reference-primitives';
import { literalRecordFromValues, literalSchema, parsedLiteralRecord } from './policy-literal-contracts';
import {
  validateGeneratedPolicyPreview,
  validateGeneratedPolicySchedule,
  validateGeneratedPolicyScheduleBoundary,
} from './generated-policy-control-helpers';

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

export const PolicyActionLiteral = literalRecordFromValues(GeneratedPolicyActionValues);
export const PolicyTargetTypeLiteral = literalRecordFromValues(GeneratedPolicyTargetTypeValues);
export const PolicyScheduleDayLiteral = literalRecordFromValues(GeneratedPolicyScheduleDayValues);
export const PolicyDecisionHandoffStateLiteral = literalRecordFromValues(GeneratedPolicyDecisionHandoffStateValues);
export const PermissionRequestStateLiteral = literalRecordFromValues(GeneratedPermissionRequestStateValues);
export const PolicyScheduleBoundaryStateLiteral = literalRecordFromValues(GeneratedPolicyScheduleBoundaryStateValues);
export const PolicyScheduleDstTransitionLiteral = literalRecordFromValues(GeneratedPolicyScheduleDstTransitionValues);
export const PolicyScheduleDstResolutionLiteral = literalRecordFromValues(GeneratedPolicyScheduleDstResolutionValues);
export const PolicyScheduleClockSourceLiteral = literalRecordFromValues(GeneratedPolicyScheduleClockSourceValues);
export const PolicyScheduleBudgetResetKindLiteral = literalRecordFromValues(
  GeneratedPolicyScheduleBudgetResetKindValues
);
export const PolicyScheduleBudgetCarryoverModeLiteral = literalRecordFromValues(
  GeneratedPolicyScheduleBudgetCarryoverModeValues
);
export const PolicyScheduleOfflineRecoveryLiteral = literalRecordFromValues(
  GeneratedPolicyScheduleOfflineRecoveryValues
);
export const PolicyScheduleOfflineRecoveryStateLiteral = literalRecordFromValues(
  GeneratedPolicyScheduleOfflineRecoveryStateValues
);
export const PolicyPreviewOriginLiteral = literalRecordFromValues(GeneratedPolicyPreviewOriginValues);
export const PolicyPreviewConfirmationStateLiteral = literalRecordFromValues(
  GeneratedPolicyPreviewConfirmationStateValues
);
export const PolicyPreviewBudgetBoundaryStateLiteral = literalRecordFromValues(
  GeneratedPolicyPreviewBudgetBoundaryStateValues
);

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
export const PolicyPreviewOriginSchema = literalSchema(PolicyPreviewOriginLiteral);
export const PolicyPreviewConfirmationStateSchema = literalSchema(PolicyPreviewConfirmationStateLiteral);
export const PolicyPreviewBudgetBoundaryStateSchema = literalSchema(PolicyPreviewBudgetBoundaryStateLiteral);

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

export const PolicyAction = parsedLiteralRecord(PolicyActionLiteral, (value) => PolicyActionSchema.parse(value));
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
export const PolicyPreviewConfirmationState = parsedLiteralRecord(PolicyPreviewConfirmationStateLiteral, (value) =>
  PolicyPreviewConfirmationStateSchema.parse(value)
);
export const PolicyPreviewBudgetBoundaryState = parsedLiteralRecord(PolicyPreviewBudgetBoundaryStateLiteral, (value) =>
  PolicyPreviewBudgetBoundaryStateSchema.parse(value)
);

export function parsePolicySchedule(input: unknown): PolicySchedule {
  const schedule = PolicyScheduleSchema.parse(input);
  validateGeneratedPolicySchedule(schedule);
  return schedule;
}

export function parseFamilyPolicySet(input: unknown): FamilyPolicySet {
  const policySet = FamilyPolicySetSchema.parse(input);
  for (const schedule of policySet.schedules) {
    validateGeneratedPolicySchedule(schedule);
  }
  return policySet;
}

export function parsePolicyScheduleBoundary(input: unknown): PolicyScheduleBoundary {
  const boundary = PolicyScheduleBoundarySchema.parse(input);
  validateGeneratedPolicyScheduleBoundary(boundary);
  return boundary;
}

export function parsePolicyPreview(input: unknown): PolicyPreview {
  const preview = PolicyPreviewSchema.parse(input);
  validateGeneratedPolicyPreview(preview);
  return preview;
}
