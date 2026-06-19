import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentActionReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './references';
import { ParentActorRole, ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';
import {
  AuditRequirementState,
  AuditRequirementStateSchema,
  DeviceTrustState,
  type DeviceTrustState as FamilyDeviceTrustState,
  HouseholdMembershipState,
  HouseholdMembershipStateSchema,
  HouseholdRole,
  HouseholdRoleSchema,
} from './household-authority';
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

export const RecoveryIdentityProofStateLiteral = {
  Verified: 'verified',
  Pending: 'pending',
  Failed: 'failed',
} as const;

export const RecoverySupportChannelLiteral = {
  SelfServe: 'self-serve',
  HouseholdOwnerAssisted: 'household-owner-assisted',
  SupportAssisted: 'support-assisted',
} as const;

export const RecoveryDecisionStateLiteral = {
  Authorized: 'authorized',
  Rejected: 'rejected',
} as const;

export const RecoveryChildEvidenceAccessStateLiteral = {
  Allowed: 'allowed',
  Blocked: 'blocked',
} as const;

export const RecoveryDataCustodyHandoffStateLiteral = {
  None: 'none',
  ExportDeleteHandoffRequired: 'export-delete-handoff-required',
  HouseholdTransferHandoffRequired: 'household-transfer-handoff-required',
} as const;

export const RecoveryBundleHandoffTargetLiteral = {
  None: 'none',
  SetupRestorePreview: 'setup-restore-preview',
  DeviceTrustRecoveryPersistence: 'device-trust-recovery-persistence',
  ParentLocalDeleteRuntime: 'parent-local-delete-runtime',
} as const;

export const RecoveryBundleStateLiteral = {
  None: 'none',
  PreviewOnly: 'preview-only',
  ApplyPending: 'apply-pending',
  Applied: 'applied',
  PartialRestore: 'partial-restore',
  Rejected: 'rejected',
  ManualRequired: 'manual-required',
} as const;

export const RecoveryBundleFailureReasonLiteral = {
  WrongHousehold: 'wrong-household',
  WrongKey: 'wrong-key',
  CorruptBundle: 'corrupt-bundle',
} as const;

export const RecoveryDeleteExportStateLiteral = {
  None: 'none',
  DeletePending: 'delete-pending',
  DeleteConfirmed: 'delete-confirmed',
} as const;

export const RecoveryFailureReasonLiteral = {
  RecoveryNotActive: 'recovery-not-active',
  WrongHousehold: 'wrong-household',
  IdentityProofRequired: 'identity-proof-required',
  RoleNotAuthorized: 'role-not-authorized',
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

export const SetupInviteReplayStateSchema = withParser(
  Schema.Literal(SetupInviteReplayStateLiteral.Fresh, SetupInviteReplayStateLiteral.ReplayDetected)
);

export const SetupRecoveryAbuseStateSchema = withParser(
  Schema.Literal(SetupRecoveryAbuseStateLiteral.WithinLimit, SetupRecoveryAbuseStateLiteral.Throttled)
);

export const SetupRecoveryResponseTimingStateSchema = withParser(
  Schema.Literal(
    SetupRecoveryResponseTimingStateLiteral.Uniform,
    SetupRecoveryResponseTimingStateLiteral.Variable
  )
);

export const SetupInviteTargetRoleSchema = withParser(
  Schema.Literal(
    'co-parent-guardian',
    'observer',
    'child-device-agent',
    'parent-owner'
  )
);

export const SetupInviteDecisionStateSchema = withParser(
  Schema.Literal(SetupInviteDecisionStateLiteral.Acceptable, SetupInviteDecisionStateLiteral.Rejected)
);

export const SetupInviteFailureReasonSchema = withParser(
  Schema.Literal(
    SetupInviteFailureReasonLiteral.InviteNotActive,
    SetupInviteFailureReasonLiteral.InviteReplayRejected,
    SetupInviteFailureReasonLiteral.InviteNotSingleUse,
    SetupInviteFailureReasonLiteral.WrongHousehold,
    SetupInviteFailureReasonLiteral.WrongTargetRole,
    SetupInviteFailureReasonLiteral.InviterNotAuthorized
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

export const RecoveryIdentityProofStateSchema = withParser(
  Schema.Literal(
    RecoveryIdentityProofStateLiteral.Verified,
    RecoveryIdentityProofStateLiteral.Pending,
    RecoveryIdentityProofStateLiteral.Failed
  )
);

export const RecoverySupportChannelSchema = withParser(
  Schema.Literal(
    RecoverySupportChannelLiteral.SelfServe,
    RecoverySupportChannelLiteral.HouseholdOwnerAssisted,
    RecoverySupportChannelLiteral.SupportAssisted
  )
);

export const RecoveryDecisionStateSchema = withParser(
  Schema.Literal(RecoveryDecisionStateLiteral.Authorized, RecoveryDecisionStateLiteral.Rejected)
);

export const RecoveryChildEvidenceAccessStateSchema = withParser(
  Schema.Literal(
    RecoveryChildEvidenceAccessStateLiteral.Allowed,
    RecoveryChildEvidenceAccessStateLiteral.Blocked
  )
);

export const RecoveryDataCustodyHandoffStateSchema = withParser(
  Schema.Literal(
    RecoveryDataCustodyHandoffStateLiteral.None,
    RecoveryDataCustodyHandoffStateLiteral.ExportDeleteHandoffRequired,
    RecoveryDataCustodyHandoffStateLiteral.HouseholdTransferHandoffRequired
  )
);

export const RecoveryBundleHandoffTargetSchema = withParser(
  Schema.Literal(
    RecoveryBundleHandoffTargetLiteral.None,
    RecoveryBundleHandoffTargetLiteral.SetupRestorePreview,
    RecoveryBundleHandoffTargetLiteral.DeviceTrustRecoveryPersistence,
    RecoveryBundleHandoffTargetLiteral.ParentLocalDeleteRuntime
  )
);

export const RecoveryBundleStateSchema = withParser(
  Schema.Literal(
    RecoveryBundleStateLiteral.None,
    RecoveryBundleStateLiteral.PreviewOnly,
    RecoveryBundleStateLiteral.ApplyPending,
    RecoveryBundleStateLiteral.Applied,
    RecoveryBundleStateLiteral.PartialRestore,
    RecoveryBundleStateLiteral.Rejected,
    RecoveryBundleStateLiteral.ManualRequired
  )
);

export const RecoveryBundleFailureReasonSchema = withParser(
  Schema.Literal(
    RecoveryBundleFailureReasonLiteral.WrongHousehold,
    RecoveryBundleFailureReasonLiteral.WrongKey,
    RecoveryBundleFailureReasonLiteral.CorruptBundle
  )
);

export const RecoveryDeleteExportStateSchema = withParser(
  Schema.Literal(
    RecoveryDeleteExportStateLiteral.None,
    RecoveryDeleteExportStateLiteral.DeletePending,
    RecoveryDeleteExportStateLiteral.DeleteConfirmed
  )
);

export const RecoveryFailureReasonSchema = withParser(
  Schema.Literal(
    RecoveryFailureReasonLiteral.RecoveryNotActive,
    RecoveryFailureReasonLiteral.WrongHousehold,
    RecoveryFailureReasonLiteral.IdentityProofRequired,
    RecoveryFailureReasonLiteral.RoleNotAuthorized
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

export const RecoveryOperationSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    recoveryOperationId: RecoveryOperationIdSchema,
    family: FamilyReferenceSchema,
    requestedBy: ParentActorReferenceSchema,
    requesterMembershipState: HouseholdMembershipStateSchema,
    relatedAccount: Schema.Union(ParentAccountReferenceSchema, Schema.Null),
    relatedDevice: Schema.Union(ParentDeviceReferenceSchema, Schema.Null),
    kind: RecoveryKindSchema,
    state: RecoveryStateSchema,
    ownerApprovalRequired: Schema.Boolean,
    identityProofState: RecoveryIdentityProofStateSchema,
    supportChannel: RecoverySupportChannelSchema,
    deleteExportHandoffRequired: Schema.Boolean,
    bundleHandoffTarget: Schema.optionalWith(RecoveryBundleHandoffTargetSchema, {
      default: () => RecoveryBundleHandoffTargetLiteral.None,
    }),
    bundleState: Schema.optionalWith(RecoveryBundleStateSchema, {
      default: () => RecoveryBundleStateLiteral.None,
    }),
    bundleFailureReason: Schema.optionalWith(Schema.Union(RecoveryBundleFailureReasonSchema, Schema.Null), {
      default: () => null,
    }),
    deleteExportState: Schema.optionalWith(RecoveryDeleteExportStateSchema, {
      default: () => RecoveryDeleteExportStateLiteral.None,
    }),
    openedAt: ParentTimestampSchema,
    closedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  })
);

export const RecoveryAuthorizationInputSchema = withParser(
  Schema.Struct({
    requesterRole: HouseholdRoleSchema,
    sameFamily: Schema.Boolean,
    kind: RecoveryKindSchema,
    state: RecoveryStateSchema,
    ownerApprovalRequired: Schema.Boolean,
    identityProofState: RecoveryIdentityProofStateSchema,
    supportChannel: RecoverySupportChannelSchema,
    deleteExportHandoffRequired: Schema.Boolean,
    abuseState: SetupRecoveryAbuseStateSchema,
    responseTimingState: SetupRecoveryResponseTimingStateSchema,
  })
);

export const RecoveryDecisionSchema = withParser(
  Schema.Struct({
    decisionState: RecoveryDecisionStateSchema,
    ownerApprovalRequired: Schema.Boolean,
    auditRequirementState: AuditRequirementStateSchema,
    childEvidenceAccessState: RecoveryChildEvidenceAccessStateSchema,
    dataCustodyHandoffState: RecoveryDataCustodyHandoffStateSchema,
    failureReason: Schema.Union(RecoveryFailureReasonSchema, Schema.Null),
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
export type RecoveryKind = Infer<typeof RecoveryKindSchema>;
export type RecoveryState = Infer<typeof RecoveryStateSchema>;
export type RecoveryIdentityProofState = Infer<typeof RecoveryIdentityProofStateSchema>;
export type RecoverySupportChannel = Infer<typeof RecoverySupportChannelSchema>;
export type RecoveryDecisionState = Infer<typeof RecoveryDecisionStateSchema>;
export type RecoveryChildEvidenceAccessState = Infer<typeof RecoveryChildEvidenceAccessStateSchema>;
export type RecoveryDataCustodyHandoffState = Infer<typeof RecoveryDataCustodyHandoffStateSchema>;
export type RecoveryBundleHandoffTarget = Infer<typeof RecoveryBundleHandoffTargetSchema>;
export type RecoveryBundleState = Infer<typeof RecoveryBundleStateSchema>;
export type RecoveryBundleFailureReason = Infer<typeof RecoveryBundleFailureReasonSchema>;
export type RecoveryDeleteExportState = Infer<typeof RecoveryDeleteExportStateSchema>;
export type RecoveryFailureReason = Infer<typeof RecoveryFailureReasonSchema>;
export type SetupAuditEventKind = Infer<typeof SetupAuditEventKindSchema>;
export type SetupInvite = Infer<typeof SetupInviteSchema>;
export type SetupInviteAuthorizationInput = Infer<typeof SetupInviteAuthorizationInputSchema>;
export type SetupInviteDecision = Infer<typeof SetupInviteDecisionSchema>;
export type RecoveryOperation = Infer<typeof RecoveryOperationSchema>;
export type RecoveryAuthorizationInput = Infer<typeof RecoveryAuthorizationInputSchema>;
export type RecoveryDecision = Infer<typeof RecoveryDecisionSchema>;
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
  InviterNotAuthorized: SetupInviteFailureReasonSchema.parse(
    SetupInviteFailureReasonLiteral.InviterNotAuthorized
  ),
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

export const RecoveryIdentityProofState = {
  Verified: RecoveryIdentityProofStateSchema.parse(RecoveryIdentityProofStateLiteral.Verified),
  Pending: RecoveryIdentityProofStateSchema.parse(RecoveryIdentityProofStateLiteral.Pending),
  Failed: RecoveryIdentityProofStateSchema.parse(RecoveryIdentityProofStateLiteral.Failed),
} as const;

export const RecoverySupportChannel = {
  SelfServe: RecoverySupportChannelSchema.parse(RecoverySupportChannelLiteral.SelfServe),
  HouseholdOwnerAssisted: RecoverySupportChannelSchema.parse(RecoverySupportChannelLiteral.HouseholdOwnerAssisted),
  SupportAssisted: RecoverySupportChannelSchema.parse(RecoverySupportChannelLiteral.SupportAssisted),
} as const;

export const RecoveryDecisionState = {
  Authorized: RecoveryDecisionStateSchema.parse(RecoveryDecisionStateLiteral.Authorized),
  Rejected: RecoveryDecisionStateSchema.parse(RecoveryDecisionStateLiteral.Rejected),
} as const;

export const RecoveryChildEvidenceAccessState = {
  Allowed: RecoveryChildEvidenceAccessStateSchema.parse(RecoveryChildEvidenceAccessStateLiteral.Allowed),
  Blocked: RecoveryChildEvidenceAccessStateSchema.parse(RecoveryChildEvidenceAccessStateLiteral.Blocked),
} as const;

export const RecoveryDataCustodyHandoffState = {
  None: RecoveryDataCustodyHandoffStateSchema.parse(RecoveryDataCustodyHandoffStateLiteral.None),
  ExportDeleteHandoffRequired: RecoveryDataCustodyHandoffStateSchema.parse(
    RecoveryDataCustodyHandoffStateLiteral.ExportDeleteHandoffRequired
  ),
  HouseholdTransferHandoffRequired: RecoveryDataCustodyHandoffStateSchema.parse(
    RecoveryDataCustodyHandoffStateLiteral.HouseholdTransferHandoffRequired
  ),
} as const;

export const RecoveryBundleHandoffTarget = {
  None: RecoveryBundleHandoffTargetSchema.parse(RecoveryBundleHandoffTargetLiteral.None),
  SetupRestorePreview: RecoveryBundleHandoffTargetSchema.parse(RecoveryBundleHandoffTargetLiteral.SetupRestorePreview),
  DeviceTrustRecoveryPersistence: RecoveryBundleHandoffTargetSchema.parse(
    RecoveryBundleHandoffTargetLiteral.DeviceTrustRecoveryPersistence
  ),
  ParentLocalDeleteRuntime: RecoveryBundleHandoffTargetSchema.parse(
    RecoveryBundleHandoffTargetLiteral.ParentLocalDeleteRuntime
  ),
} as const;

export const RecoveryBundleState = {
  None: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.None),
  PreviewOnly: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.PreviewOnly),
  ApplyPending: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.ApplyPending),
  Applied: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.Applied),
  PartialRestore: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.PartialRestore),
  Rejected: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.Rejected),
  ManualRequired: RecoveryBundleStateSchema.parse(RecoveryBundleStateLiteral.ManualRequired),
} as const;

export const RecoveryBundleFailureReason = {
  WrongHousehold: RecoveryBundleFailureReasonSchema.parse(RecoveryBundleFailureReasonLiteral.WrongHousehold),
  WrongKey: RecoveryBundleFailureReasonSchema.parse(RecoveryBundleFailureReasonLiteral.WrongKey),
  CorruptBundle: RecoveryBundleFailureReasonSchema.parse(RecoveryBundleFailureReasonLiteral.CorruptBundle),
} as const;

export const RecoveryDeleteExportState = {
  None: RecoveryDeleteExportStateSchema.parse(RecoveryDeleteExportStateLiteral.None),
  DeletePending: RecoveryDeleteExportStateSchema.parse(RecoveryDeleteExportStateLiteral.DeletePending),
  DeleteConfirmed: RecoveryDeleteExportStateSchema.parse(RecoveryDeleteExportStateLiteral.DeleteConfirmed),
} as const;

export const RecoveryFailureReason = {
  RecoveryNotActive: RecoveryFailureReasonSchema.parse(RecoveryFailureReasonLiteral.RecoveryNotActive),
  WrongHousehold: RecoveryFailureReasonSchema.parse(RecoveryFailureReasonLiteral.WrongHousehold),
  IdentityProofRequired: RecoveryFailureReasonSchema.parse(
    RecoveryFailureReasonLiteral.IdentityProofRequired
  ),
  RoleNotAuthorized: RecoveryFailureReasonSchema.parse(RecoveryFailureReasonLiteral.RoleNotAuthorized),
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

  return purposeMatchesTargetRole(invite.purpose, invite.targetRole);
}

export function isSetupInviteSinglePurpose(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  return invite.singleUse && doesSetupInviteMatchTargetRole(invite);
}

export function authorizeSetupInvite(input: SetupInviteAuthorizationInput): SetupInviteDecision {
  const parsedInput = SetupInviteAuthorizationInputSchema.parse(input);

  if (parsedInput.inviteState !== SetupInviteState.Pending) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotActive);
  }

  if (!parsedInput.singleUse) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotSingleUse);
  }

  if (parsedInput.replayState !== SetupInviteReplayState.Fresh) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteReplayRejected);
  }

  if (parsedInput.abuseState === SetupRecoveryAbuseState.Throttled) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotActive);
  }

  if (parsedInput.responseTimingState !== SetupRecoveryResponseTimingState.Uniform) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviteNotActive);
  }

  if (!parsedInput.sameFamily) {
    return rejectedSetupInvite(SetupInviteFailureReason.WrongHousehold);
  }

  if (!purposeMatchesTargetRole(parsedInput.purpose, parsedInput.targetRole)) {
    return rejectedSetupInvite(SetupInviteFailureReason.WrongTargetRole);
  }

  if (!inviterCanIssue(parsedInput.inviterRole, parsedInput.purpose)) {
    return rejectedSetupInvite(SetupInviteFailureReason.InviterNotAuthorized);
  }

  return SetupInviteDecisionSchema.parse({
    decisionState: SetupInviteDecisionState.Acceptable,
    auditRequirementState: AuditRequirementState.Required,
    failureReason: null,
  });
}

export function recoveryRequiresOwnerApproval(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);

  return requiresOwnerApproval(recovery.kind, recovery.ownerApprovalRequired);
}

export function recoveryDataCustodyHandoffState(input: RecoveryOperation): RecoveryDataCustodyHandoffState {
  const recovery = RecoveryOperationSchema.parse(input);

  return dataCustodyHandoffState(
    recovery.kind,
    recovery.deleteExportHandoffRequired,
    recovery.deleteExportState
  );
}

export function recoveryRequiresAuditedSupport(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);

  return recovery.supportChannel === RecoverySupportChannel.SupportAssisted;
}

export function recoveryCanAccessChildEvidence(input: RecoveryOperation): boolean {
  const recovery = RecoveryOperationSchema.parse(input);
  const hasHouseholdAuthority =
    recovery.requesterMembershipState === HouseholdMembershipState.Active &&
    (recovery.requestedBy.role === ParentActorRole.Parent || recovery.requestedBy.role === ParentActorRole.Guardian);

  return (
    hasHouseholdAuthority &&
    recovery.identityProofState === RecoveryIdentityProofState.Verified &&
    recovery.supportChannel !== RecoverySupportChannel.SupportAssisted
  );
}

export function deviceTrustStateForRecoveryState(state: RecoveryState): FamilyDeviceTrustState {
  const parsedState = RecoveryStateSchema.parse(state);

  if (
    parsedState === RecoveryState.PendingIdentityProof ||
    parsedState === RecoveryState.OwnerApprovalRequired ||
    parsedState === RecoveryState.Approved
  ) {
    return DeviceTrustState.ResetRequired;
  }

  if (parsedState === RecoveryState.Completed) {
    return DeviceTrustState.Pending;
  }

  return DeviceTrustState.Revoked;
}

export function deviceTrustStateForRecoveryOperation(input: RecoveryOperation): FamilyDeviceTrustState {
  const recovery = RecoveryOperationSchema.parse(input);

  if (recovery.state === RecoveryState.Revoked) {
    return DeviceTrustState.Revoked;
  }

  if (recovery.bundleFailureReason !== null) {
    return DeviceTrustState.ResetRequired;
  }

  switch (recovery.bundleState) {
    case RecoveryBundleState.PreviewOnly:
    case RecoveryBundleState.ApplyPending:
    case RecoveryBundleState.PartialRestore:
    case RecoveryBundleState.Rejected:
    case RecoveryBundleState.ManualRequired:
      return DeviceTrustState.ResetRequired;
    case RecoveryBundleState.Applied:
      return dataCustodyHandoffState(
        recovery.kind,
        recovery.deleteExportHandoffRequired,
        recovery.deleteExportState
      ) === RecoveryDataCustodyHandoffState.None && recovery.state === RecoveryState.Completed
        ? DeviceTrustState.Pending
        : DeviceTrustState.ResetRequired;
    default:
      return deviceTrustStateForRecoveryState(recovery.state);
  }
}

export function evaluateRecoveryOperation(input: RecoveryAuthorizationInput): RecoveryDecision {
  const parsedInput = RecoveryAuthorizationInputSchema.parse(input);
  const ownerApprovalRequired = requiresOwnerApproval(parsedInput.kind, parsedInput.ownerApprovalRequired);
  const childEvidenceAccessState = recoveryChildEvidenceAccessState(
    parsedInput.requesterRole,
    parsedInput.sameFamily,
    parsedInput.supportChannel
  );
  const custodyHandoffState = dataCustodyHandoffState(
    parsedInput.kind,
    parsedInput.deleteExportHandoffRequired,
    RecoveryDeleteExportState.None
  );

  if (parsedInput.state === RecoveryState.Revoked) {
    return rejectedRecovery(
      RecoveryFailureReason.RecoveryNotActive,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.requesterRole !== HouseholdRole.SupportAdmin && !parsedInput.sameFamily) {
    return rejectedRecovery(
      RecoveryFailureReason.WrongHousehold,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.identityProofState !== RecoveryIdentityProofState.Verified) {
    return rejectedRecovery(
      RecoveryFailureReason.IdentityProofRequired,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.abuseState === SetupRecoveryAbuseState.Throttled) {
    return rejectedRecovery(
      RecoveryFailureReason.IdentityProofRequired,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (parsedInput.responseTimingState !== SetupRecoveryResponseTimingState.Uniform) {
    return rejectedRecovery(
      RecoveryFailureReason.IdentityProofRequired,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  if (!requesterCanRecover(parsedInput.requesterRole, parsedInput.kind, parsedInput.supportChannel)) {
    return rejectedRecovery(
      RecoveryFailureReason.RoleNotAuthorized,
      ownerApprovalRequired,
      childEvidenceAccessState,
      custodyHandoffState
    );
  }

  return RecoveryDecisionSchema.parse({
    decisionState: RecoveryDecisionState.Authorized,
    ownerApprovalRequired,
    auditRequirementState: AuditRequirementState.Required,
    childEvidenceAccessState,
    dataCustodyHandoffState: custodyHandoffState,
    failureReason: null,
  });
}

function rejectedSetupInvite(failureReason: SetupInviteFailureReason): SetupInviteDecision {
  return SetupInviteDecisionSchema.parse({
    decisionState: SetupInviteDecisionState.Rejected,
    auditRequirementState: AuditRequirementState.Required,
    failureReason,
  });
}

function rejectedRecovery(
  failureReason: RecoveryFailureReason,
  ownerApprovalRequired: boolean,
  childEvidenceAccessState: RecoveryChildEvidenceAccessState,
  dataCustodyHandoffState: RecoveryDataCustodyHandoffState
): RecoveryDecision {
  return RecoveryDecisionSchema.parse({
    decisionState: RecoveryDecisionState.Rejected,
    ownerApprovalRequired,
    auditRequirementState: AuditRequirementState.Required,
    childEvidenceAccessState,
    dataCustodyHandoffState,
    failureReason,
  });
}

function purposeMatchesTargetRole(
  purpose: SetupInvitePurpose,
  targetRole: SetupInviteTargetRole | HouseholdRole
): boolean {
  return (
    (purpose === SetupInvitePurpose.CoParentInvite && targetRole === HouseholdRole.CoParentGuardian) ||
    (purpose === SetupInvitePurpose.ObserverInvite && targetRole === HouseholdRole.Observer) ||
    (purpose === SetupInvitePurpose.ChildDevicePairing && targetRole === HouseholdRole.ChildDeviceAgent) ||
    (purpose === SetupInvitePurpose.HouseholdTransfer && targetRole === HouseholdRole.ParentOwner)
  );
}

function inviterCanIssue(role: HouseholdRole, purpose: SetupInvitePurpose): boolean {
  switch (purpose) {
    case SetupInvitePurpose.CoParentInvite:
    case SetupInvitePurpose.ObserverInvite:
    case SetupInvitePurpose.ChildDevicePairing:
      return role === HouseholdRole.ParentOwner || role === HouseholdRole.CoParentGuardian;
    case SetupInvitePurpose.HouseholdTransfer:
      return role === HouseholdRole.ParentOwner;
  }

  return false;
}

function requesterCanRecover(
  role: HouseholdRole,
  kind: RecoveryKind,
  supportChannel: RecoverySupportChannel
): boolean {
  if (role === HouseholdRole.SupportAdmin) {
    return supportChannel === RecoverySupportChannel.SupportAssisted;
  }

  if (kind === RecoveryKind.HouseholdTransfer) {
    return role === HouseholdRole.ParentOwner;
  }

  return role === HouseholdRole.ParentOwner || role === HouseholdRole.CoParentGuardian;
}

function requiresOwnerApproval(kind: RecoveryKind, ownerApprovalRequired: boolean): boolean {
  return (
    ownerApprovalRequired ||
    kind === RecoveryKind.LostParentDevice ||
    kind === RecoveryKind.CompromisedAccount ||
    kind === RecoveryKind.HouseholdTransfer
  );
}

function dataCustodyHandoffState(
  kind: RecoveryKind,
  deleteExportHandoffRequired: boolean,
  deleteExportState: RecoveryDeleteExportState
): RecoveryDataCustodyHandoffState {
  if (kind === RecoveryKind.HouseholdTransfer) {
    return RecoveryDataCustodyHandoffState.HouseholdTransferHandoffRequired;
  }

  if (deleteExportHandoffRequired && deleteExportState !== RecoveryDeleteExportState.DeleteConfirmed) {
    return RecoveryDataCustodyHandoffState.ExportDeleteHandoffRequired;
  }

  return RecoveryDataCustodyHandoffState.None;
}

function recoveryChildEvidenceAccessState(
  requesterRole: HouseholdRole,
  sameFamily: boolean,
  supportChannel: RecoverySupportChannel
): RecoveryChildEvidenceAccessState {
  const hasHouseholdAuthority =
    sameFamily &&
    (requesterRole === HouseholdRole.ParentOwner || requesterRole === HouseholdRole.CoParentGuardian);

  if (hasHouseholdAuthority && supportChannel !== RecoverySupportChannel.SupportAssisted) {
    return RecoveryChildEvidenceAccessState.Allowed;
  }

  return RecoveryChildEvidenceAccessState.Blocked;
}
