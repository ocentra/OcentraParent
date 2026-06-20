import {
  type Infer,
  Schema,
  brandedNonEmptyStringSchema,
  withParser,
} from './effect';
import {
  ParentActorRole,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from './family-reference-primitives';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActionReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import {
  AuditRequirementStateSchema,
  HouseholdRoleSchema,
} from './family-household-authority';

export { ParentActorRole };

export const SetupInviteIdSchema = brandedNonEmptyStringSchema('SetupInviteId');
export const SetupAuditEventIdSchema = brandedNonEmptyStringSchema('SetupAuditEventId');

export const SetupInvitePurposeLiteral = {
  CoParentInvite: 'co-parent-invite',
  ObserverInvite: 'observer-invite',
  ChildDevicePairing: 'child-device-pairing',
  HouseholdTransfer: 'household-transfer',
} as const;
export const SetupInviteStateLiteral = {
  Pending: 'pending',
  Accepted: 'accepted',
  Expired: 'expired',
  Revoked: 'revoked',
} as const;
export const SetupInviteReplayStateLiteral = {
  Fresh: 'fresh',
  ReplayDetected: 'replay-detected',
} as const;
export const SetupRecoveryAbuseStateLiteral = {
  WithinLimit: 'within-limit',
  Throttled: 'throttled',
} as const;
export const SetupRecoveryResponseTimingStateLiteral = {
  Uniform: 'uniform',
  Variable: 'variable',
} as const;
export const SetupInviteDecisionStateLiteral = {
  Acceptable: 'acceptable',
  Rejected: 'rejected',
} as const;
export const SetupInviteFailureReasonLiteral = {
  InviteNotActive: 'invite-not-active',
  InviteReplayRejected: 'invite-replay-rejected',
  InviteNotSingleUse: 'invite-not-single-use',
  WrongHousehold: 'wrong-household',
  WrongTargetRole: 'wrong-target-role',
  InviterNotAuthorized: 'inviter-not-authorized',
} as const;
export const SetupAuditEventKindLiteral = {
  HouseholdCreated: 'household-created',
  ChildProfileAdded: 'child-profile-added',
  DevicePaired: 'device-paired',
  MemberInvited: 'member-invited',
  MemberRevoked: 'member-revoked',
  RecoveryApproved: 'recovery-approved',
  RecoveryCompleted: 'recovery-completed',
} as const;

const setupInvitePurposeValues = [
  SetupInvitePurposeLiteral.CoParentInvite,
  SetupInvitePurposeLiteral.ObserverInvite,
  SetupInvitePurposeLiteral.ChildDevicePairing,
  SetupInvitePurposeLiteral.HouseholdTransfer,
] as const;
const setupInviteStateValues = [
  SetupInviteStateLiteral.Pending,
  SetupInviteStateLiteral.Accepted,
  SetupInviteStateLiteral.Expired,
  SetupInviteStateLiteral.Revoked,
] as const;
const setupInviteReplayStateValues = [
  SetupInviteReplayStateLiteral.Fresh,
  SetupInviteReplayStateLiteral.ReplayDetected,
] as const;
const setupRecoveryAbuseStateValues = [
  SetupRecoveryAbuseStateLiteral.WithinLimit,
  SetupRecoveryAbuseStateLiteral.Throttled,
] as const;
const setupRecoveryResponseTimingStateValues = [
  SetupRecoveryResponseTimingStateLiteral.Uniform,
  SetupRecoveryResponseTimingStateLiteral.Variable,
] as const;
const setupInviteTargetRoleValues = ['co-parent-guardian', 'observer', 'child-device-agent', 'parent-owner'] as const;
const setupInviteDecisionStateValues = [
  SetupInviteDecisionStateLiteral.Acceptable,
  SetupInviteDecisionStateLiteral.Rejected,
] as const;
const setupInviteFailureReasonValues = [
  SetupInviteFailureReasonLiteral.InviteNotActive,
  SetupInviteFailureReasonLiteral.InviteReplayRejected,
  SetupInviteFailureReasonLiteral.InviteNotSingleUse,
  SetupInviteFailureReasonLiteral.WrongHousehold,
  SetupInviteFailureReasonLiteral.WrongTargetRole,
  SetupInviteFailureReasonLiteral.InviterNotAuthorized,
] as const;
const setupAuditEventKindValues = [
  SetupAuditEventKindLiteral.HouseholdCreated,
  SetupAuditEventKindLiteral.ChildProfileAdded,
  SetupAuditEventKindLiteral.DevicePaired,
  SetupAuditEventKindLiteral.MemberInvited,
  SetupAuditEventKindLiteral.MemberRevoked,
  SetupAuditEventKindLiteral.RecoveryApproved,
  SetupAuditEventKindLiteral.RecoveryCompleted,
] as const;

export const SetupInvitePurposeSchema = withParser(Schema.Literal(...setupInvitePurposeValues));
export const SetupInviteStateSchema = withParser(Schema.Literal(...setupInviteStateValues));
export const SetupInviteReplayStateSchema = withParser(Schema.Literal(...setupInviteReplayStateValues));
export const SetupRecoveryAbuseStateSchema = withParser(Schema.Literal(...setupRecoveryAbuseStateValues));
export const SetupRecoveryResponseTimingStateSchema = withParser(
  Schema.Literal(...setupRecoveryResponseTimingStateValues)
);
export const SetupInviteTargetRoleSchema = withParser(Schema.Literal(...setupInviteTargetRoleValues));
export const SetupInviteDecisionStateSchema = withParser(Schema.Literal(...setupInviteDecisionStateValues));
export const SetupInviteFailureReasonSchema = withParser(Schema.Literal(...setupInviteFailureReasonValues));
export const SetupAuditEventKindSchema = withParser(Schema.Literal(...setupAuditEventKindValues));

export const SetupInviteSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    inviteId: SetupInviteIdSchema,
    family: FamilyReferenceSchema,
    invitedBy: ParentActorReferenceSchema,
    targetAccount: Schema.Union(ParentAccountReferenceSchema, Schema.Null),
    targetChildProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    targetRole: HouseholdRoleSchema,
    purpose: SetupInvitePurposeSchema,
    state: SetupInviteStateSchema,
    expiresAt: ParentTimestampSchema,
    singleUse: Schema.Boolean,
  })
);
export const SetupInviteAuthorizationInputSchema = withParser(
  Schema.Struct({
    inviterRole: HouseholdRoleSchema,
    sameFamily: Schema.Boolean,
    purpose: SetupInvitePurposeSchema,
    targetRole: SetupInviteTargetRoleSchema,
    inviteState: SetupInviteStateSchema,
    singleUse: Schema.Boolean,
    replayState: SetupInviteReplayStateSchema,
    abuseState: SetupRecoveryAbuseStateSchema,
    responseTimingState: SetupRecoveryResponseTimingStateSchema,
  })
);
export const SetupInviteDecisionSchema = withParser(
  Schema.Struct({
    decisionState: SetupInviteDecisionStateSchema,
    auditRequirementState: AuditRequirementStateSchema,
    failureReason: Schema.Union(SetupInviteFailureReasonSchema, Schema.Null),
  })
);
export const SetupAuditEventSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    auditEventId: SetupAuditEventIdSchema,
    family: FamilyReferenceSchema,
    actor: ParentActorReferenceSchema,
    kind: SetupAuditEventKindSchema,
    childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    device: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    action: Schema.Union(ParentActionReferenceSchema, Schema.Null),
    observedAt: ParentTimestampSchema,
  })
);

export type SetupInvitePurpose = Infer<typeof SetupInvitePurposeSchema>;
export type SetupInviteState = Infer<typeof SetupInviteStateSchema>;
export type SetupInviteReplayState = Infer<typeof SetupInviteReplayStateSchema>;
export type SetupRecoveryAbuseState = Infer<typeof SetupRecoveryAbuseStateSchema>;
export type SetupRecoveryResponseTimingState = Infer<typeof SetupRecoveryResponseTimingStateSchema>;
export type SetupInviteTargetRole = Infer<typeof SetupInviteTargetRoleSchema>;
export type SetupInviteDecisionState = Infer<typeof SetupInviteDecisionStateSchema>;
export type SetupInviteFailureReason = Infer<typeof SetupInviteFailureReasonSchema>;
export type SetupAuditEventKind = Infer<typeof SetupAuditEventKindSchema>;
export type SetupInvite = Infer<typeof SetupInviteSchema>;
export type SetupInviteAuthorizationInput = Infer<typeof SetupInviteAuthorizationInputSchema>;
export type SetupInviteDecision = Infer<typeof SetupInviteDecisionSchema>;
export type SetupAuditEvent = Infer<typeof SetupAuditEventSchema>;

export const SetupInvitePurpose = {
  CoParentInvite: SetupInvitePurposeSchema.parse(SetupInvitePurposeLiteral.CoParentInvite),
  ObserverInvite: SetupInvitePurposeSchema.parse(SetupInvitePurposeLiteral.ObserverInvite),
  ChildDevicePairing: SetupInvitePurposeSchema.parse(SetupInvitePurposeLiteral.ChildDevicePairing),
  HouseholdTransfer: SetupInvitePurposeSchema.parse(SetupInvitePurposeLiteral.HouseholdTransfer),
} as const;
export const SetupInviteState = {
  Pending: SetupInviteStateSchema.parse(SetupInviteStateLiteral.Pending),
  Accepted: SetupInviteStateSchema.parse(SetupInviteStateLiteral.Accepted),
  Expired: SetupInviteStateSchema.parse(SetupInviteStateLiteral.Expired),
  Revoked: SetupInviteStateSchema.parse(SetupInviteStateLiteral.Revoked),
} as const;
export const SetupInviteReplayState = {
  Fresh: SetupInviteReplayStateSchema.parse(SetupInviteReplayStateLiteral.Fresh),
  ReplayDetected: SetupInviteReplayStateSchema.parse(SetupInviteReplayStateLiteral.ReplayDetected),
} as const;
export const SetupRecoveryAbuseState = {
  WithinLimit: SetupRecoveryAbuseStateSchema.parse(SetupRecoveryAbuseStateLiteral.WithinLimit),
  Throttled: SetupRecoveryAbuseStateSchema.parse(SetupRecoveryAbuseStateLiteral.Throttled),
} as const;
export const SetupRecoveryResponseTimingState = {
  Uniform: SetupRecoveryResponseTimingStateSchema.parse(SetupRecoveryResponseTimingStateLiteral.Uniform),
  Variable: SetupRecoveryResponseTimingStateSchema.parse(SetupRecoveryResponseTimingStateLiteral.Variable),
} as const;
export const SetupInviteDecisionState = {
  Acceptable: SetupInviteDecisionStateSchema.parse(SetupInviteDecisionStateLiteral.Acceptable),
  Rejected: SetupInviteDecisionStateSchema.parse(SetupInviteDecisionStateLiteral.Rejected),
} as const;
export const SetupInviteFailureReason = {
  InviteNotActive: SetupInviteFailureReasonSchema.parse(SetupInviteFailureReasonLiteral.InviteNotActive),
  InviteReplayRejected: SetupInviteFailureReasonSchema.parse(SetupInviteFailureReasonLiteral.InviteReplayRejected),
  InviteNotSingleUse: SetupInviteFailureReasonSchema.parse(SetupInviteFailureReasonLiteral.InviteNotSingleUse),
  WrongHousehold: SetupInviteFailureReasonSchema.parse(SetupInviteFailureReasonLiteral.WrongHousehold),
  WrongTargetRole: SetupInviteFailureReasonSchema.parse(SetupInviteFailureReasonLiteral.WrongTargetRole),
  InviterNotAuthorized: SetupInviteFailureReasonSchema.parse(SetupInviteFailureReasonLiteral.InviterNotAuthorized),
} as const;
export const SetupAuditEventKind = {
  HouseholdCreated: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.HouseholdCreated),
  ChildProfileAdded: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.ChildProfileAdded),
  DevicePaired: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.DevicePaired),
  MemberInvited: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.MemberInvited),
  MemberRevoked: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.MemberRevoked),
  RecoveryApproved: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.RecoveryApproved),
  RecoveryCompleted: SetupAuditEventKindSchema.parse(SetupAuditEventKindLiteral.RecoveryCompleted),
} as const;
