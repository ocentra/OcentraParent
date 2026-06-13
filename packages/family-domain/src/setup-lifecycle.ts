import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';
import { HouseholdRole, HouseholdRoleSchema } from './household-authority';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

function brandedNonEmptyStringSchema<const Brand extends string>(brand: Brand) {
  return Schema.String.pipe(Schema.minLength(1), Schema.brand(brand));
}

export const SetupInviteIdSchema = brandedNonEmptyStringSchema('SetupInviteId');
export const RecoveryOperationIdSchema = brandedNonEmptyStringSchema('RecoveryOperationId');
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

export const RecoveryKindLiteral = {
  ForgotLogin: 'forgot-login',
  LostParentDevice: 'lost-parent-device',
  CompromisedAccount: 'compromised-account',
  ChildReinstall: 'child-reinstall',
  HouseholdTransfer: 'household-transfer',
} as const;

export const RecoveryStateLiteral = {
  PendingIdentityProof: 'pending-identity-proof',
  OwnerApprovalRequired: 'owner-approval-required',
  Approved: 'approved',
  Completed: 'completed',
  Revoked: 'revoked',
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

export const SetupInvitePurposeSchema = withParser(
  Schema.Literal(
    SetupInvitePurposeLiteral.CoParentInvite,
    SetupInvitePurposeLiteral.ObserverInvite,
    SetupInvitePurposeLiteral.ChildDevicePairing,
    SetupInvitePurposeLiteral.HouseholdTransfer
  )
);

export const SetupInviteStateSchema = withParser(
  Schema.Literal(
    SetupInviteStateLiteral.Pending,
    SetupInviteStateLiteral.Accepted,
    SetupInviteStateLiteral.Expired,
    SetupInviteStateLiteral.Revoked
  )
);

export const RecoveryKindSchema = withParser(
  Schema.Literal(
    RecoveryKindLiteral.ForgotLogin,
    RecoveryKindLiteral.LostParentDevice,
    RecoveryKindLiteral.CompromisedAccount,
    RecoveryKindLiteral.ChildReinstall,
    RecoveryKindLiteral.HouseholdTransfer
  )
);

export const RecoveryStateSchema = withParser(
  Schema.Literal(
    RecoveryStateLiteral.PendingIdentityProof,
    RecoveryStateLiteral.OwnerApprovalRequired,
    RecoveryStateLiteral.Approved,
    RecoveryStateLiteral.Completed,
    RecoveryStateLiteral.Revoked
  )
);

export const SetupAuditEventKindSchema = withParser(
  Schema.Literal(
    SetupAuditEventKindLiteral.HouseholdCreated,
    SetupAuditEventKindLiteral.ChildProfileAdded,
    SetupAuditEventKindLiteral.DevicePaired,
    SetupAuditEventKindLiteral.MemberInvited,
    SetupAuditEventKindLiteral.MemberRevoked,
    SetupAuditEventKindLiteral.RecoveryApproved,
    SetupAuditEventKindLiteral.RecoveryCompleted
  )
);

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

export const RecoveryOperationSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    recoveryOperationId: RecoveryOperationIdSchema,
    family: FamilyReferenceSchema,
    requestedBy: ParentActorReferenceSchema,
    relatedAccount: Schema.Union(ParentAccountReferenceSchema, Schema.Null),
    relatedDevice: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    kind: RecoveryKindSchema,
    state: RecoveryStateSchema,
    ownerApprovalRequired: Schema.Boolean,
    openedAt: ParentTimestampSchema,
    closedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
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
export type RecoveryKind = Infer<typeof RecoveryKindSchema>;
export type RecoveryState = Infer<typeof RecoveryStateSchema>;
export type SetupAuditEventKind = Infer<typeof SetupAuditEventKindSchema>;
export type SetupInvite = Infer<typeof SetupInviteSchema>;
export type RecoveryOperation = Infer<typeof RecoveryOperationSchema>;
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

export const RecoveryKind = {
  ForgotLogin: RecoveryKindSchema.parse(RecoveryKindLiteral.ForgotLogin),
  LostParentDevice: RecoveryKindSchema.parse(RecoveryKindLiteral.LostParentDevice),
  CompromisedAccount: RecoveryKindSchema.parse(RecoveryKindLiteral.CompromisedAccount),
  ChildReinstall: RecoveryKindSchema.parse(RecoveryKindLiteral.ChildReinstall),
  HouseholdTransfer: RecoveryKindSchema.parse(RecoveryKindLiteral.HouseholdTransfer),
} as const;

export const RecoveryState = {
  PendingIdentityProof: RecoveryStateSchema.parse(RecoveryStateLiteral.PendingIdentityProof),
  OwnerApprovalRequired: RecoveryStateSchema.parse(RecoveryStateLiteral.OwnerApprovalRequired),
  Approved: RecoveryStateSchema.parse(RecoveryStateLiteral.Approved),
  Completed: RecoveryStateSchema.parse(RecoveryStateLiteral.Completed),
  Revoked: RecoveryStateSchema.parse(RecoveryStateLiteral.Revoked),
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

export function isSetupInviteActive(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  return invite.state === SetupInviteState.Pending;
}

export function doesSetupInviteMatchTargetRole(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  switch (invite.purpose) {
    case SetupInvitePurpose.CoParentInvite:
      return invite.targetRole === HouseholdRole.CoParentGuardian;
    case SetupInvitePurpose.ObserverInvite:
      return invite.targetRole === HouseholdRole.Observer;
    case SetupInvitePurpose.ChildDevicePairing:
      return invite.targetRole === HouseholdRole.ChildDeviceAgent;
    case SetupInvitePurpose.HouseholdTransfer:
      return invite.targetRole === HouseholdRole.ParentOwner;
    default:
      return false;
  }
}

export function recoveryRequiresOwnerApproval(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);

  return (
    recovery.ownerApprovalRequired ||
    recovery.kind === RecoveryKind.LostParentDevice ||
    recovery.kind === RecoveryKind.CompromisedAccount ||
    recovery.kind === RecoveryKind.HouseholdTransfer
  );
}
