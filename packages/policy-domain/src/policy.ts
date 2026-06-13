import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from '@ocentra-parent/schema-domain/effect';
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
} from '@ocentra-parent/family-domain/reference-primitives';

const PolicyTimestamp = brandedNonEmptyStringSchema('PolicyTimestamp');

export const PolicyRuleIdSchema = brandedNonEmptyStringSchema('PolicyRuleId');
export const PolicyScheduleIdSchema = brandedNonEmptyStringSchema('PolicyScheduleId');
export const PolicyTargetIdSchema = brandedNonEmptyStringSchema('PolicyTargetId');
export const PermissionRequestIdSchema = brandedNonEmptyStringSchema('PermissionRequestId');
export const PolicyDecisionIdSchema = brandedNonEmptyStringSchema('PolicyDecisionId');
export const PolicyReasonCodeSchema = brandedNonEmptyStringSchema('PolicyReasonCode');
export const PolicyLocalTimeSchema = brandedNonEmptyStringSchema('PolicyLocalTime');
export const PolicyTimeZoneSchema = brandedNonEmptyStringSchema('PolicyTimeZone');
export const LocalAiResultReferenceIdSchema = brandedNonEmptyStringSchema('LocalAiResultReferenceId');

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

export const PolicyActionSchema = withParser(
  Schema.Literal(
    PolicyActionLiteral.Allow,
    PolicyActionLiteral.Warn,
    PolicyActionLiteral.Block,
    PolicyActionLiteral.TimeLimit,
    PolicyActionLiteral.AskParent,
    PolicyActionLiteral.Unknown
  )
);

export const PolicyTargetTypeSchema = withParser(
  Schema.Literal(
    PolicyTargetTypeLiteral.App,
    PolicyTargetTypeLiteral.Process,
    PolicyTargetTypeLiteral.Window,
    PolicyTargetTypeLiteral.Domain,
    PolicyTargetTypeLiteral.Site,
    PolicyTargetTypeLiteral.Category,
    PolicyTargetTypeLiteral.Video,
    PolicyTargetTypeLiteral.Channel,
    PolicyTargetTypeLiteral.ActivityType,
    PolicyTargetTypeLiteral.Device
  )
);

export const PolicyScheduleDaySchema = withParser(
  Schema.Literal(
    PolicyScheduleDayLiteral.Monday,
    PolicyScheduleDayLiteral.Tuesday,
    PolicyScheduleDayLiteral.Wednesday,
    PolicyScheduleDayLiteral.Thursday,
    PolicyScheduleDayLiteral.Friday,
    PolicyScheduleDayLiteral.Saturday,
    PolicyScheduleDayLiteral.Sunday
  )
);

export const PolicyDecisionHandoffStateSchema = withParser(
  Schema.Literal(
    PolicyDecisionHandoffStateLiteral.NotRequested,
    PolicyDecisionHandoffStateLiteral.Disabled,
    PolicyDecisionHandoffStateLiteral.Pending,
    PolicyDecisionHandoffStateLiteral.HandedOff
  )
);

export const PermissionRequestStateSchema = withParser(
  Schema.Literal(
    PermissionRequestStateLiteral.Open,
    PermissionRequestStateLiteral.Approved,
    PermissionRequestStateLiteral.Denied,
    PermissionRequestStateLiteral.Expired,
    PermissionRequestStateLiteral.Cancelled
  )
);

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
  Allow: PolicyActionSchema.parse(PolicyActionLiteral.Allow),
  Warn: PolicyActionSchema.parse(PolicyActionLiteral.Warn),
  Block: PolicyActionSchema.parse(PolicyActionLiteral.Block),
  TimeLimit: PolicyActionSchema.parse(PolicyActionLiteral.TimeLimit),
  AskParent: PolicyActionSchema.parse(PolicyActionLiteral.AskParent),
  Unknown: PolicyActionSchema.parse(PolicyActionLiteral.Unknown),
} as const;

export const PolicyTargetType = {
  App: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.App),
  Process: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Process),
  Window: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Window),
  Domain: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Domain),
  Site: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Site),
  Category: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Category),
  Video: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Video),
  Channel: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Channel),
  ActivityType: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.ActivityType),
  Device: PolicyTargetTypeSchema.parse(PolicyTargetTypeLiteral.Device),
} as const;

export const PolicyScheduleDay = {
  Monday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Monday),
  Tuesday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Tuesday),
  Wednesday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Wednesday),
  Thursday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Thursday),
  Friday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Friday),
  Saturday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Saturday),
  Sunday: PolicyScheduleDaySchema.parse(PolicyScheduleDayLiteral.Sunday),
} as const;

export const PolicyDecisionHandoffState = {
  NotRequested: PolicyDecisionHandoffStateSchema.parse(PolicyDecisionHandoffStateLiteral.NotRequested),
  Disabled: PolicyDecisionHandoffStateSchema.parse(PolicyDecisionHandoffStateLiteral.Disabled),
  Pending: PolicyDecisionHandoffStateSchema.parse(PolicyDecisionHandoffStateLiteral.Pending),
  HandedOff: PolicyDecisionHandoffStateSchema.parse(PolicyDecisionHandoffStateLiteral.HandedOff),
} as const;

export const PermissionRequestState = {
  Open: PermissionRequestStateSchema.parse(PermissionRequestStateLiteral.Open),
  Approved: PermissionRequestStateSchema.parse(PermissionRequestStateLiteral.Approved),
  Denied: PermissionRequestStateSchema.parse(PermissionRequestStateLiteral.Denied),
  Expired: PermissionRequestStateSchema.parse(PermissionRequestStateLiteral.Expired),
  Cancelled: PermissionRequestStateSchema.parse(PermissionRequestStateLiteral.Cancelled),
} as const;

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

export function comparePolicyActionStrictness(left: PolicyAction, right: PolicyAction): number {
  return PolicyActionStrictnessRank[left] - PolicyActionStrictnessRank[right];
}

export function selectStricterPolicyAction(parentRuleAction: PolicyAction, localAiAction: PolicyAction): PolicyAction {
  return comparePolicyActionStrictness(parentRuleAction, localAiAction) >= 0 ? parentRuleAction : localAiAction;
}
