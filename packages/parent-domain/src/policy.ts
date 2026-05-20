import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
  ParentEvidenceReferenceSchema,
} from './references';
import { ParentContractSchemaVersionSchema, ParentPolicyVersionSchema } from './reference-primitives';

const NonEmptyPolicyText = Schema.String.pipe(Schema.minLength(1));
const PolicyTimestamp = NonEmptyPolicyText.pipe(Schema.brand('PolicyTimestamp'));

export const PolicyRuleIdSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyRuleId'));
export const PolicyScheduleIdSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyScheduleId'));
export const PolicyTargetIdSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyTargetId'));
export const PermissionRequestIdSchema = NonEmptyPolicyText.pipe(Schema.brand('PermissionRequestId'));
export const PolicyDecisionIdSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyDecisionId'));
export const PolicyReasonCodeSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyReasonCode'));
export const PolicyLocalTimeSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyLocalTime'));
export const PolicyTimeZoneSchema = NonEmptyPolicyText.pipe(Schema.brand('PolicyTimeZone'));
export const LocalAiResultReferenceIdSchema = NonEmptyPolicyText.pipe(Schema.brand('LocalAiResultReferenceId'));

export const PolicyActionSchema = withParser(
  Schema.Literal('allow', 'warn', 'block', 'time-limit', 'ask-parent', 'unknown')
);

export const PolicyTargetTypeSchema = withParser(
  Schema.Literal(
    'app',
    'process',
    'window',
    'domain',
    'site',
    'category',
    'video',
    'channel',
    'activity-type',
    'device'
  )
);

export const PolicyScheduleDaySchema = withParser(
  Schema.Literal('monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'sunday')
);

export const PolicyDecisionHandoffStateSchema = withParser(
  Schema.Literal('not-requested', 'disabled', 'pending', 'handed-off')
);

export const PermissionRequestStateSchema = withParser(
  Schema.Literal('open', 'approved', 'denied', 'expired', 'cancelled')
);

export const PolicyTargetSchema = withParser(
  Schema.Struct({
    targetId: PolicyTargetIdSchema,
    targetType: PolicyTargetTypeSchema,
    targetValue: NonEmptyPolicyText.pipe(Schema.brand('PolicyTargetValue')),
  })
);

export const PolicyScheduleWindowSchema = withParser(
  Schema.Struct({
    days: Schema.Array(PolicyScheduleDaySchema),
    startLocalTime: PolicyLocalTimeSchema,
    endLocalTime: PolicyLocalTimeSchema,
  })
);

export const PolicyScheduleSchema = withParser(
  Schema.Struct({
    scheduleId: PolicyScheduleIdSchema,
    timeZone: PolicyTimeZoneSchema,
    windows: Schema.Array(PolicyScheduleWindowSchema),
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
    effectiveFrom: Schema.Union(PolicyTimestamp, Schema.Null),
    effectiveUntil: Schema.Union(PolicyTimestamp, Schema.Null),
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
    expiresAt: Schema.Union(PolicyTimestamp, Schema.Null),
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
    expiresAt: Schema.Union(PolicyTimestamp, Schema.Null),
  })
);

export type PolicyRuleId = typeof PolicyRuleIdSchema.Type;
export type PolicyScheduleId = typeof PolicyScheduleIdSchema.Type;
export type PolicyTargetId = typeof PolicyTargetIdSchema.Type;
export type PermissionRequestId = typeof PermissionRequestIdSchema.Type;
export type PolicyDecisionId = typeof PolicyDecisionIdSchema.Type;
export type PolicyReasonCode = typeof PolicyReasonCodeSchema.Type;
export type LocalAiResultReferenceId = typeof LocalAiResultReferenceIdSchema.Type;
export type PolicyAction = Infer<typeof PolicyActionSchema>;
export type PolicyTargetType = Infer<typeof PolicyTargetTypeSchema>;
export type PolicyScheduleDay = Infer<typeof PolicyScheduleDaySchema>;
export type PolicyDecisionHandoffState = Infer<typeof PolicyDecisionHandoffStateSchema>;
export type PermissionRequestState = Infer<typeof PermissionRequestStateSchema>;
export type PolicyTarget = Infer<typeof PolicyTargetSchema>;
export type PolicyScheduleWindow = Infer<typeof PolicyScheduleWindowSchema>;
export type PolicySchedule = Infer<typeof PolicyScheduleSchema>;
export type PolicyRule = Infer<typeof PolicyRuleSchema>;
export type FamilyPolicySet = Infer<typeof FamilyPolicySetSchema>;
export type PermissionRequest = Infer<typeof PermissionRequestSchema>;
export type PolicyDecision = Infer<typeof PolicyDecisionSchema>;

export const PolicyAction = {
  Allow: PolicyActionSchema.parse('allow'),
  Warn: PolicyActionSchema.parse('warn'),
  Block: PolicyActionSchema.parse('block'),
  TimeLimit: PolicyActionSchema.parse('time-limit'),
  AskParent: PolicyActionSchema.parse('ask-parent'),
  Unknown: PolicyActionSchema.parse('unknown'),
} as const;

export const PolicyDecisionHandoffState = {
  NotRequested: PolicyDecisionHandoffStateSchema.parse('not-requested'),
  Disabled: PolicyDecisionHandoffStateSchema.parse('disabled'),
  Pending: PolicyDecisionHandoffStateSchema.parse('pending'),
  HandedOff: PolicyDecisionHandoffStateSchema.parse('handed-off'),
} as const;
