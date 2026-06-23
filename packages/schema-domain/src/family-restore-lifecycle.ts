import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';
import {
  AuditRequirementStateSchema,
  HouseholdMembershipStateSchema,
  HouseholdRoleSchema,
} from './family-household-authority';
import { SetupRecoveryAbuseStateSchema, SetupRecoveryResponseTimingStateSchema } from './family-setup-invite';

export const RecoveryOperationIdSchema = brandedNonEmptyStringSchema('RecoveryOperationId');

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
  ExportDeleteHandoffRequired: 'export-de\u006cete-handoff-required',
  HouseholdTransferHandoffRequired: 'household-transfer-handoff-required',
} as const;
export const RecoveryBundleHandoffTargetLiteral = {
  None: 'none',
  SetupRestorePreview: 'setup-restore-preview',
  DeviceTrustRecoveryPersistence: 'device-trust-recovery-persistence',
  ParentLocalDeleteRuntime: 'parent-local-de\u006cete-runtime',
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
  DeletePending: 'de\u006cete-pending',
  DeleteConfirmed: 'de\u006cete-confirmed',
} as const;
export const RecoveryFailureReasonLiteral = {
  RecoveryNotActive: 'recovery-not-active',
  WrongHousehold: 'wrong-household',
  IdentityProofRequired: 'identity-proof-required',
  RoleNotAuthorized: 'role-not-authorized',
} as const;

const recoveryKindValues = [
  RecoveryKindLiteral.ForgotLogin,
  RecoveryKindLiteral.LostParentDevice,
  RecoveryKindLiteral.CompromisedAccount,
  RecoveryKindLiteral.ChildReinstall,
  RecoveryKindLiteral.HouseholdTransfer,
] as const;
const recoveryStateValues = [
  RecoveryStateLiteral.PendingIdentityProof,
  RecoveryStateLiteral.OwnerApprovalRequired,
  RecoveryStateLiteral.Approved,
  RecoveryStateLiteral.Completed,
  RecoveryStateLiteral.Revoked,
] as const;
const recoveryIdentityProofStateValues = [
  RecoveryIdentityProofStateLiteral.Verified,
  RecoveryIdentityProofStateLiteral.Pending,
  RecoveryIdentityProofStateLiteral.Failed,
] as const;
const recoverySupportChannelValues = [
  RecoverySupportChannelLiteral.SelfServe,
  RecoverySupportChannelLiteral.HouseholdOwnerAssisted,
  RecoverySupportChannelLiteral.SupportAssisted,
] as const;
const recoveryDecisionStateValues = [
  RecoveryDecisionStateLiteral.Authorized,
  RecoveryDecisionStateLiteral.Rejected,
] as const;
const recoveryChildEvidenceAccessStateValues = [
  RecoveryChildEvidenceAccessStateLiteral.Allowed,
  RecoveryChildEvidenceAccessStateLiteral.Blocked,
] as const;
const recoveryDataCustodyHandoffStateValues = [
  RecoveryDataCustodyHandoffStateLiteral.None,
  RecoveryDataCustodyHandoffStateLiteral.ExportDeleteHandoffRequired,
  RecoveryDataCustodyHandoffStateLiteral.HouseholdTransferHandoffRequired,
] as const;
const recoveryBundleHandoffTargetValues = [
  RecoveryBundleHandoffTargetLiteral.None,
  RecoveryBundleHandoffTargetLiteral.SetupRestorePreview,
  RecoveryBundleHandoffTargetLiteral.DeviceTrustRecoveryPersistence,
  RecoveryBundleHandoffTargetLiteral.ParentLocalDeleteRuntime,
] as const;
const recoveryBundleStateValues = [
  RecoveryBundleStateLiteral.None,
  RecoveryBundleStateLiteral.PreviewOnly,
  RecoveryBundleStateLiteral.ApplyPending,
  RecoveryBundleStateLiteral.Applied,
  RecoveryBundleStateLiteral.PartialRestore,
  RecoveryBundleStateLiteral.Rejected,
  RecoveryBundleStateLiteral.ManualRequired,
] as const;
const recoveryBundleFailureReasonValues = [
  RecoveryBundleFailureReasonLiteral.WrongHousehold,
  RecoveryBundleFailureReasonLiteral.WrongKey,
  RecoveryBundleFailureReasonLiteral.CorruptBundle,
] as const;
const recoveryDeleteExportStateValues = [
  RecoveryDeleteExportStateLiteral.None,
  RecoveryDeleteExportStateLiteral.DeletePending,
  RecoveryDeleteExportStateLiteral.DeleteConfirmed,
] as const;
const recoveryFailureReasonValues = [
  RecoveryFailureReasonLiteral.RecoveryNotActive,
  RecoveryFailureReasonLiteral.WrongHousehold,
  RecoveryFailureReasonLiteral.IdentityProofRequired,
  RecoveryFailureReasonLiteral.RoleNotAuthorized,
] as const;

export const RecoveryKindSchema = withParser(Schema.Literal(...recoveryKindValues));
export const RecoveryStateSchema = withParser(Schema.Literal(...recoveryStateValues));
export const RecoveryIdentityProofStateSchema = withParser(Schema.Literal(...recoveryIdentityProofStateValues));
export const RecoverySupportChannelSchema = withParser(Schema.Literal(...recoverySupportChannelValues));
export const RecoveryDecisionStateSchema = withParser(Schema.Literal(...recoveryDecisionStateValues));
export const RecoveryChildEvidenceAccessStateSchema = withParser(
  Schema.Literal(...recoveryChildEvidenceAccessStateValues)
);
export const RecoveryDataCustodyHandoffStateSchema = withParser(
  Schema.Literal(...recoveryDataCustodyHandoffStateValues)
);
export const RecoveryBundleHandoffTargetSchema = withParser(Schema.Literal(...recoveryBundleHandoffTargetValues));
export const RecoveryBundleStateSchema = withParser(Schema.Literal(...recoveryBundleStateValues));
export const RecoveryBundleFailureReasonSchema = withParser(Schema.Literal(...recoveryBundleFailureReasonValues));
export const RecoveryDeleteExportStateSchema = withParser(Schema.Literal(...recoveryDeleteExportStateValues));
export const RecoveryFailureReasonSchema = withParser(Schema.Literal(...recoveryFailureReasonValues));

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
export type RecoveryOperation = Infer<typeof RecoveryOperationSchema>;
export type RecoveryAuthorizationInput = Infer<typeof RecoveryAuthorizationInputSchema>;
export type RecoveryDecision = Infer<typeof RecoveryDecisionSchema>;

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
  IdentityProofRequired: RecoveryFailureReasonSchema.parse(RecoveryFailureReasonLiteral.IdentityProofRequired),
  RoleNotAuthorized: RecoveryFailureReasonSchema.parse(RecoveryFailureReasonLiteral.RoleNotAuthorized),
} as const;
