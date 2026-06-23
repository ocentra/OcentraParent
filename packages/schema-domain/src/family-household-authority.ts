import { type Infer, Schema, brandedNonEmptyStringSchema, withParser } from './effect';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './family-reference-primitives';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './family-references';

export const HouseholdProfileIdSchema = brandedNonEmptyStringSchema('HouseholdProfileId');
export const HouseholdDisplayNameSchema = brandedNonEmptyStringSchema('HouseholdDisplayName');
export const HouseholdMembershipIdSchema = brandedNonEmptyStringSchema('HouseholdMembershipId');
export const ParentMemberDisplayNameSchema = brandedNonEmptyStringSchema('ParentMemberDisplayName');
export const DeviceRegistrationIdSchema = brandedNonEmptyStringSchema('DeviceRegistrationId');
export const ParentControllerLeaseIdSchema = brandedNonEmptyStringSchema('ParentControllerLeaseId');
export const ObserverPermissionIdSchema = brandedNonEmptyStringSchema('ObserverPermissionId');
export const ParentStepUpAssertionIdSchema = brandedNonEmptyStringSchema('ParentStepUpAssertionId');
export const ParentStepUpNonceSchema = brandedNonEmptyStringSchema('ParentStepUpNonce');

export const HouseholdRoleLiteral = {
  ParentOwner: 'parent-owner',
  CoParentGuardian: 'co-parent-guardian',
  Observer: 'observer',
  ChildProfile: 'child-profile',
  ChildDeviceAgent: 'child-device-agent',
  SupportAdmin: 'support-admin',
} as const;

export const HouseholdMembershipStateLiteral = {
  Invited: 'invited',
  Pending: 'pending',
  Active: 'active',
  Revoked: 'revoked',
  Disabled: 'disabled',
} as const;

export const DeviceRoleLiteral = {
  ParentController: 'parent-controller',
  ParentObserver: 'parent-observer',
  ChildAgent: 'child-agent',
} as const;

export const DeviceTrustStateLiteral = {
  Pending: 'pending',
  Trusted: 'trusted',
  Revoked: 'revoked',
  ResetRequired: 'reset-required',
  Disabled: 'disabled',
} as const;

export const ActorAccountStateLiteral = {
  Active: 'active',
  Suspended: 'suspended',
  Disabled: 'disabled',
} as const;

export const ChildProfileBindingStateLiteral = {
  Bound: 'bound',
  Missing: 'missing',
  Unassigned: 'unassigned',
} as const;

export const DeviceOwnershipScopeLiteral = {
  ChildProfileDevice: 'child-profile-device',
  ParentControllerDevice: 'parent-controller-device',
  OtherDevice: 'other-device',
} as const;

export const SessionFreshnessStateLiteral = {
  Fresh: 'fresh',
  Stale: 'stale',
  Expired: 'expired',
} as const;

export const HouseholdAuthorizationStateLiteral = {
  Authorized: 'authorized',
  Rejected: 'rejected',
} as const;

export const AuditRequirementStateLiteral = {
  Required: 'required',
  NotRequired: 'not-required',
} as const;

export const ElevatedConfirmationStateLiteral = {
  Required: 'required',
  NotRequired: 'not-required',
} as const;

export const ParentStepUpMethodLiteral = {
  Passkey: 'passkey',
  OsNative: 'os-native',
  PhoneQrApproval: 'phone-qr-approval',
} as const;

export const ParentStepUpValidationFailureReasonLiteral = {
  Required: 'required',
  Expired: 'expired',
  WrongHousehold: 'wrong-household',
  WrongAccount: 'wrong-account',
  WrongAction: 'wrong-action',
  WrongDevice: 'wrong-device',
  WrongTarget: 'wrong-target',
  ReplayRejected: 'replay-rejected',
} as const;

export const HouseholdAuthorizationFailureReasonLiteral = {
  ExternalHousehold: 'external-household',
  MembershipNotActive: 'membership-not-active',
  AccountNotActive: 'account-not-active',
  DeviceNotTrusted: 'device-not-trusted',
  SessionNotFresh: 'session-not-fresh',
  ChildProfileNotBound: 'child-profile-not-bound',
  WrongDeviceScope: 'wrong-device-scope',
  MissingCapabilityGrant: 'missing-capability-grant',
  ControllerLeaseRequired: 'controller-lease-required',
  ControllerLeaseExpired: 'controller-lease-expired',
  ControllerLeaseRevoked: 'controller-lease-revoked',
  RoleNotAuthorized: 'role-not-authorized',
} as const;

export const DeviceAuthorityActionLiteral = {
  PairChildDevice: 'pair-child-device',
  RevokeChildDevice: 'revoke-child-device',
  ViewChildStatus: 'view-child-status',
  ChangePolicy: 'change-policy',
  StartRemoteView: 'start-remote-view',
  StartRemoteControl: 'start-remote-control',
  ExportDeleteData: 'export-delete-data',
  ManageBilling: 'manage-billing',
} as const;

export const ParentControllerLeaseStateLiteral = {
  Active: 'active',
  Expired: 'expired',
  Revoked: 'revoked',
} as const;

export const ObserverPermissionScopeLiteral = {
  HouseholdSummary: 'household-summary',
  ChildStatus: 'child-status',
  DeviceSourceState: 'device-source-state',
} as const;

export const ObserverPermissionStateLiteral = {
  Granted: 'granted',
  Revoked: 'revoked',
  Disabled: 'disabled',
} as const;

const householdRoleValues = [
  HouseholdRoleLiteral.ParentOwner,
  HouseholdRoleLiteral.CoParentGuardian,
  HouseholdRoleLiteral.Observer,
  HouseholdRoleLiteral.ChildProfile,
  HouseholdRoleLiteral.ChildDeviceAgent,
  HouseholdRoleLiteral.SupportAdmin,
] as const;
const householdMembershipStateValues = [
  HouseholdMembershipStateLiteral.Invited,
  HouseholdMembershipStateLiteral.Pending,
  HouseholdMembershipStateLiteral.Active,
  HouseholdMembershipStateLiteral.Revoked,
  HouseholdMembershipStateLiteral.Disabled,
] as const;
const deviceRoleValues = [
  DeviceRoleLiteral.ParentController,
  DeviceRoleLiteral.ParentObserver,
  DeviceRoleLiteral.ChildAgent,
] as const;
const deviceTrustStateValues = [
  DeviceTrustStateLiteral.Pending,
  DeviceTrustStateLiteral.Trusted,
  DeviceTrustStateLiteral.Revoked,
  DeviceTrustStateLiteral.ResetRequired,
  DeviceTrustStateLiteral.Disabled,
] as const;
const actorAccountStateValues = [
  ActorAccountStateLiteral.Active,
  ActorAccountStateLiteral.Suspended,
  ActorAccountStateLiteral.Disabled,
] as const;
const childProfileBindingStateValues = [
  ChildProfileBindingStateLiteral.Bound,
  ChildProfileBindingStateLiteral.Missing,
  ChildProfileBindingStateLiteral.Unassigned,
] as const;
const deviceOwnershipScopeValues = [
  DeviceOwnershipScopeLiteral.ChildProfileDevice,
  DeviceOwnershipScopeLiteral.ParentControllerDevice,
  DeviceOwnershipScopeLiteral.OtherDevice,
] as const;
const sessionFreshnessStateValues = [
  SessionFreshnessStateLiteral.Fresh,
  SessionFreshnessStateLiteral.Stale,
  SessionFreshnessStateLiteral.Expired,
] as const;
const householdAuthorizationStateValues = [
  HouseholdAuthorizationStateLiteral.Authorized,
  HouseholdAuthorizationStateLiteral.Rejected,
] as const;
const auditRequirementStateValues = [
  AuditRequirementStateLiteral.Required,
  AuditRequirementStateLiteral.NotRequired,
] as const;
const elevatedConfirmationStateValues = [
  ElevatedConfirmationStateLiteral.Required,
  ElevatedConfirmationStateLiteral.NotRequired,
] as const;
const parentStepUpMethodValues = [
  ParentStepUpMethodLiteral.Passkey,
  ParentStepUpMethodLiteral.OsNative,
  ParentStepUpMethodLiteral.PhoneQrApproval,
] as const;
const parentStepUpValidationFailureReasonValues = [
  ParentStepUpValidationFailureReasonLiteral.Required,
  ParentStepUpValidationFailureReasonLiteral.Expired,
  ParentStepUpValidationFailureReasonLiteral.WrongHousehold,
  ParentStepUpValidationFailureReasonLiteral.WrongAccount,
  ParentStepUpValidationFailureReasonLiteral.WrongAction,
  ParentStepUpValidationFailureReasonLiteral.WrongDevice,
  ParentStepUpValidationFailureReasonLiteral.WrongTarget,
  ParentStepUpValidationFailureReasonLiteral.ReplayRejected,
] as const;
const householdAuthorizationFailureReasonValues = [
  HouseholdAuthorizationFailureReasonLiteral.ExternalHousehold,
  HouseholdAuthorizationFailureReasonLiteral.MembershipNotActive,
  HouseholdAuthorizationFailureReasonLiteral.AccountNotActive,
  HouseholdAuthorizationFailureReasonLiteral.DeviceNotTrusted,
  HouseholdAuthorizationFailureReasonLiteral.SessionNotFresh,
  HouseholdAuthorizationFailureReasonLiteral.ChildProfileNotBound,
  HouseholdAuthorizationFailureReasonLiteral.WrongDeviceScope,
  HouseholdAuthorizationFailureReasonLiteral.MissingCapabilityGrant,
  HouseholdAuthorizationFailureReasonLiteral.ControllerLeaseRequired,
  HouseholdAuthorizationFailureReasonLiteral.ControllerLeaseExpired,
  HouseholdAuthorizationFailureReasonLiteral.ControllerLeaseRevoked,
  HouseholdAuthorizationFailureReasonLiteral.RoleNotAuthorized,
] as const;
const deviceAuthorityActionValues = [
  DeviceAuthorityActionLiteral.PairChildDevice,
  DeviceAuthorityActionLiteral.RevokeChildDevice,
  DeviceAuthorityActionLiteral.ViewChildStatus,
  DeviceAuthorityActionLiteral.ChangePolicy,
  DeviceAuthorityActionLiteral.StartRemoteView,
  DeviceAuthorityActionLiteral.StartRemoteControl,
  DeviceAuthorityActionLiteral.ExportDeleteData,
  DeviceAuthorityActionLiteral.ManageBilling,
] as const;
const parentControllerLeaseStateValues = [
  ParentControllerLeaseStateLiteral.Active,
  ParentControllerLeaseStateLiteral.Expired,
  ParentControllerLeaseStateLiteral.Revoked,
] as const;
const observerPermissionScopeValues = [
  ObserverPermissionScopeLiteral.HouseholdSummary,
  ObserverPermissionScopeLiteral.ChildStatus,
  ObserverPermissionScopeLiteral.DeviceSourceState,
] as const;
const observerPermissionStateValues = [
  ObserverPermissionStateLiteral.Granted,
  ObserverPermissionStateLiteral.Revoked,
  ObserverPermissionStateLiteral.Disabled,
] as const;

export const HouseholdRoleSchema = withParser(Schema.Literal(...householdRoleValues));
export const HouseholdMembershipStateSchema = withParser(Schema.Literal(...householdMembershipStateValues));
export const DeviceRoleSchema = withParser(Schema.Literal(...deviceRoleValues));
export const DeviceTrustStateSchema = withParser(Schema.Literal(...deviceTrustStateValues));
export const ActorAccountStateSchema = withParser(Schema.Literal(...actorAccountStateValues));
export const ChildProfileBindingStateSchema = withParser(Schema.Literal(...childProfileBindingStateValues));
export const DeviceOwnershipScopeSchema = withParser(Schema.Literal(...deviceOwnershipScopeValues));
export const SessionFreshnessStateSchema = withParser(Schema.Literal(...sessionFreshnessStateValues));
export const HouseholdAuthorizationStateSchema = withParser(Schema.Literal(...householdAuthorizationStateValues));
export const AuditRequirementStateSchema = withParser(Schema.Literal(...auditRequirementStateValues));
export const ElevatedConfirmationStateSchema = withParser(Schema.Literal(...elevatedConfirmationStateValues));
export const ParentStepUpMethodSchema = withParser(Schema.Literal(...parentStepUpMethodValues));
export const ParentStepUpValidationFailureReasonSchema = withParser(
  Schema.Literal(...parentStepUpValidationFailureReasonValues)
);
export const HouseholdAuthorizationFailureReasonSchema = withParser(
  Schema.Literal(...householdAuthorizationFailureReasonValues)
);
export const DeviceAuthorityActionSchema = withParser(Schema.Literal(...deviceAuthorityActionValues));
export const ParentControllerLeaseStateSchema = withParser(Schema.Literal(...parentControllerLeaseStateValues));
export const ObserverPermissionScopeSchema = withParser(Schema.Literal(...observerPermissionScopeValues));
export const ObserverPermissionStateSchema = withParser(Schema.Literal(...observerPermissionStateValues));

export const HouseholdProfileSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    householdProfileId: HouseholdProfileIdSchema,
    family: FamilyReferenceSchema,
    displayName: HouseholdDisplayNameSchema,
    ownerAccount: ParentAccountReferenceSchema,
    createdAt: ParentTimestampSchema,
  })
);
export const ParentMemberSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    membershipId: HouseholdMembershipIdSchema,
    family: FamilyReferenceSchema,
    account: ParentAccountReferenceSchema,
    role: HouseholdRoleSchema,
    membershipState: HouseholdMembershipStateSchema,
    displayName: ParentMemberDisplayNameSchema,
    invitedBy: Schema.Union(ParentActorReferenceSchema, Schema.Null),
  })
);
export const DeviceRegistrationSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    registrationId: DeviceRegistrationIdSchema,
    family: FamilyReferenceSchema,
    device: ParentDeviceReferenceSchema,
    deviceRole: DeviceRoleSchema,
    trustState: DeviceTrustStateSchema,
    registeredBy: ParentActorReferenceSchema,
    registeredAt: ParentTimestampSchema,
  })
);
export const ParentControllerLeaseSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    leaseId: ParentControllerLeaseIdSchema,
    family: FamilyReferenceSchema,
    controller: ParentActorReferenceSchema,
    device: ParentDeviceReferenceSchema,
    action: DeviceAuthorityActionSchema,
    state: ParentControllerLeaseStateSchema,
    issuedAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
  })
);
export const ParentStepUpAssertionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    stepUpAssertionId: ParentStepUpAssertionIdSchema,
    family: FamilyReferenceSchema,
    parentAccount: ParentAccountReferenceSchema,
    actionDevice: ParentDeviceReferenceSchema,
    approverDevice: ParentDeviceReferenceSchema,
    targetChildProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    action: DeviceAuthorityActionSchema,
    method: ParentStepUpMethodSchema,
    nonce: ParentStepUpNonceSchema,
    issuedAt: ParentTimestampSchema,
    expiresAt: ParentTimestampSchema,
  })
);
export const ObserverPermissionSchema = withParser(
  Schema.Struct({
    schemaVersion: ParentContractSchemaVersionSchema,
    permissionId: ObserverPermissionIdSchema,
    family: FamilyReferenceSchema,
    observerAccount: ParentAccountReferenceSchema,
    scope: ObserverPermissionScopeSchema,
    childProfile: Schema.Union(ChildProfileReferenceSchema, Schema.Null),
    state: ObserverPermissionStateSchema,
    grantedAt: ParentTimestampSchema,
    revokedAt: Schema.Union(ParentTimestampSchema, Schema.Null),
  })
);
export const HouseholdAuthorityInputSchema = withParser(
  Schema.Struct({
    actorRole: HouseholdRoleSchema,
    actorAccountState: ActorAccountStateSchema,
    sameFamily: Schema.Boolean,
    membershipState: HouseholdMembershipStateSchema,
    childProfileBindingState: ChildProfileBindingStateSchema,
    deviceOwnershipScope: DeviceOwnershipScopeSchema,
    deviceTrustState: DeviceTrustStateSchema,
    sessionFreshnessState: SessionFreshnessStateSchema,
    capabilityGranted: Schema.Boolean,
    controllerLeaseState: Schema.optional(Schema.Union(ParentControllerLeaseStateSchema, Schema.Null)),
    action: DeviceAuthorityActionSchema,
  })
);
export const HouseholdAuthorityDecisionSchema = withParser(
  Schema.Struct({
    authorizationState: HouseholdAuthorizationStateSchema,
    auditRequirementState: AuditRequirementStateSchema,
    elevatedConfirmationState: ElevatedConfirmationStateSchema,
    failureReason: Schema.Union(HouseholdAuthorizationFailureReasonSchema, Schema.Null),
  })
);

export type HouseholdRole = Infer<typeof HouseholdRoleSchema>;
export type HouseholdMembershipState = Infer<typeof HouseholdMembershipStateSchema>;
export type DeviceRole = Infer<typeof DeviceRoleSchema>;
export type DeviceTrustState = Infer<typeof DeviceTrustStateSchema>;
export type ActorAccountState = Infer<typeof ActorAccountStateSchema>;
export type ChildProfileBindingState = Infer<typeof ChildProfileBindingStateSchema>;
export type DeviceOwnershipScope = Infer<typeof DeviceOwnershipScopeSchema>;
export type DeviceAuthorityAction = Infer<typeof DeviceAuthorityActionSchema>;
export type SessionFreshnessState = Infer<typeof SessionFreshnessStateSchema>;
export type HouseholdAuthorizationState = Infer<typeof HouseholdAuthorizationStateSchema>;
export type AuditRequirementState = Infer<typeof AuditRequirementStateSchema>;
export type ElevatedConfirmationState = Infer<typeof ElevatedConfirmationStateSchema>;
export type ParentStepUpMethod = Infer<typeof ParentStepUpMethodSchema>;
export type ParentStepUpValidationFailureReason = Infer<typeof ParentStepUpValidationFailureReasonSchema>;
export type HouseholdAuthorizationFailureReason = Infer<typeof HouseholdAuthorizationFailureReasonSchema>;
export type ParentControllerLeaseState = Infer<typeof ParentControllerLeaseStateSchema>;
export type ObserverPermissionScope = Infer<typeof ObserverPermissionScopeSchema>;
export type ObserverPermissionState = Infer<typeof ObserverPermissionStateSchema>;
export type HouseholdProfile = Infer<typeof HouseholdProfileSchema>;
export type ParentMember = Infer<typeof ParentMemberSchema>;
export type DeviceRegistration = Infer<typeof DeviceRegistrationSchema>;
export type ParentControllerLease = Infer<typeof ParentControllerLeaseSchema>;
export type ParentStepUpAssertion = Infer<typeof ParentStepUpAssertionSchema>;
export type ObserverPermission = Infer<typeof ObserverPermissionSchema>;
export type HouseholdAuthorityInput = Infer<typeof HouseholdAuthorityInputSchema>;
export type HouseholdAuthorityDecision = Infer<typeof HouseholdAuthorityDecisionSchema>;

export const HouseholdRole = {
  ParentOwner: HouseholdRoleSchema.parse(HouseholdRoleLiteral.ParentOwner),
  CoParentGuardian: HouseholdRoleSchema.parse(HouseholdRoleLiteral.CoParentGuardian),
  Observer: HouseholdRoleSchema.parse(HouseholdRoleLiteral.Observer),
  ChildProfile: HouseholdRoleSchema.parse(HouseholdRoleLiteral.ChildProfile),
  ChildDeviceAgent: HouseholdRoleSchema.parse(HouseholdRoleLiteral.ChildDeviceAgent),
  SupportAdmin: HouseholdRoleSchema.parse(HouseholdRoleLiteral.SupportAdmin),
} as const;
export const HouseholdMembershipState = {
  Invited: HouseholdMembershipStateSchema.parse(HouseholdMembershipStateLiteral.Invited),
  Pending: HouseholdMembershipStateSchema.parse(HouseholdMembershipStateLiteral.Pending),
  Active: HouseholdMembershipStateSchema.parse(HouseholdMembershipStateLiteral.Active),
  Revoked: HouseholdMembershipStateSchema.parse(HouseholdMembershipStateLiteral.Revoked),
  Disabled: HouseholdMembershipStateSchema.parse(HouseholdMembershipStateLiteral.Disabled),
} as const;
export const DeviceRole = {
  ParentController: DeviceRoleSchema.parse(DeviceRoleLiteral.ParentController),
  ParentObserver: DeviceRoleSchema.parse(DeviceRoleLiteral.ParentObserver),
  ChildAgent: DeviceRoleSchema.parse(DeviceRoleLiteral.ChildAgent),
} as const;
export const DeviceTrustState = {
  Pending: DeviceTrustStateSchema.parse(DeviceTrustStateLiteral.Pending),
  Trusted: DeviceTrustStateSchema.parse(DeviceTrustStateLiteral.Trusted),
  Revoked: DeviceTrustStateSchema.parse(DeviceTrustStateLiteral.Revoked),
  ResetRequired: DeviceTrustStateSchema.parse(DeviceTrustStateLiteral.ResetRequired),
  Disabled: DeviceTrustStateSchema.parse(DeviceTrustStateLiteral.Disabled),
} as const;
export const ActorAccountState = {
  Active: ActorAccountStateSchema.parse(ActorAccountStateLiteral.Active),
  Suspended: ActorAccountStateSchema.parse(ActorAccountStateLiteral.Suspended),
  Disabled: ActorAccountStateSchema.parse(ActorAccountStateLiteral.Disabled),
} as const;
export const ChildProfileBindingState = {
  Bound: ChildProfileBindingStateSchema.parse(ChildProfileBindingStateLiteral.Bound),
  Missing: ChildProfileBindingStateSchema.parse(ChildProfileBindingStateLiteral.Missing),
  Unassigned: ChildProfileBindingStateSchema.parse(ChildProfileBindingStateLiteral.Unassigned),
} as const;
export const DeviceOwnershipScope = {
  ChildProfileDevice: DeviceOwnershipScopeSchema.parse(DeviceOwnershipScopeLiteral.ChildProfileDevice),
  ParentControllerDevice: DeviceOwnershipScopeSchema.parse(DeviceOwnershipScopeLiteral.ParentControllerDevice),
  OtherDevice: DeviceOwnershipScopeSchema.parse(DeviceOwnershipScopeLiteral.OtherDevice),
} as const;
export const SessionFreshnessState = {
  Fresh: SessionFreshnessStateSchema.parse(SessionFreshnessStateLiteral.Fresh),
  Stale: SessionFreshnessStateSchema.parse(SessionFreshnessStateLiteral.Stale),
  Expired: SessionFreshnessStateSchema.parse(SessionFreshnessStateLiteral.Expired),
} as const;
export const HouseholdAuthorizationState = {
  Authorized: HouseholdAuthorizationStateSchema.parse(HouseholdAuthorizationStateLiteral.Authorized),
  Rejected: HouseholdAuthorizationStateSchema.parse(HouseholdAuthorizationStateLiteral.Rejected),
} as const;
export const AuditRequirementState = {
  Required: AuditRequirementStateSchema.parse(AuditRequirementStateLiteral.Required),
  NotRequired: AuditRequirementStateSchema.parse(AuditRequirementStateLiteral.NotRequired),
} as const;
export const ElevatedConfirmationState = {
  Required: ElevatedConfirmationStateSchema.parse(ElevatedConfirmationStateLiteral.Required),
  NotRequired: ElevatedConfirmationStateSchema.parse(ElevatedConfirmationStateLiteral.NotRequired),
} as const;
export const ParentStepUpMethod = {
  Passkey: ParentStepUpMethodSchema.parse(ParentStepUpMethodLiteral.Passkey),
  OsNative: ParentStepUpMethodSchema.parse(ParentStepUpMethodLiteral.OsNative),
  PhoneQrApproval: ParentStepUpMethodSchema.parse(ParentStepUpMethodLiteral.PhoneQrApproval),
} as const;
export const ParentStepUpValidationFailureReason = {
  Required: ParentStepUpValidationFailureReasonSchema.parse(ParentStepUpValidationFailureReasonLiteral.Required),
  Expired: ParentStepUpValidationFailureReasonSchema.parse(ParentStepUpValidationFailureReasonLiteral.Expired),
  WrongHousehold: ParentStepUpValidationFailureReasonSchema.parse(
    ParentStepUpValidationFailureReasonLiteral.WrongHousehold
  ),
  WrongAccount: ParentStepUpValidationFailureReasonSchema.parse(
    ParentStepUpValidationFailureReasonLiteral.WrongAccount
  ),
  WrongAction: ParentStepUpValidationFailureReasonSchema.parse(ParentStepUpValidationFailureReasonLiteral.WrongAction),
  WrongDevice: ParentStepUpValidationFailureReasonSchema.parse(ParentStepUpValidationFailureReasonLiteral.WrongDevice),
  WrongTarget: ParentStepUpValidationFailureReasonSchema.parse(ParentStepUpValidationFailureReasonLiteral.WrongTarget),
  ReplayRejected: ParentStepUpValidationFailureReasonSchema.parse(
    ParentStepUpValidationFailureReasonLiteral.ReplayRejected
  ),
} as const;
export const HouseholdAuthorizationFailureReason = {
  ExternalHousehold: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.ExternalHousehold
  ),
  MembershipNotActive: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.MembershipNotActive
  ),
  AccountNotActive: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.AccountNotActive
  ),
  DeviceNotTrusted: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.DeviceNotTrusted
  ),
  SessionNotFresh: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.SessionNotFresh
  ),
  ChildProfileNotBound: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.ChildProfileNotBound
  ),
  WrongDeviceScope: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.WrongDeviceScope
  ),
  MissingCapabilityGrant: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.MissingCapabilityGrant
  ),
  ControllerLeaseRequired: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.ControllerLeaseRequired
  ),
  ControllerLeaseExpired: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.ControllerLeaseExpired
  ),
  ControllerLeaseRevoked: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.ControllerLeaseRevoked
  ),
  RoleNotAuthorized: HouseholdAuthorizationFailureReasonSchema.parse(
    HouseholdAuthorizationFailureReasonLiteral.RoleNotAuthorized
  ),
} as const;
export const ParentControllerLeaseState = {
  Active: ParentControllerLeaseStateSchema.parse(ParentControllerLeaseStateLiteral.Active),
  Expired: ParentControllerLeaseStateSchema.parse(ParentControllerLeaseStateLiteral.Expired),
  Revoked: ParentControllerLeaseStateSchema.parse(ParentControllerLeaseStateLiteral.Revoked),
} as const;
export const ObserverPermissionScope = {
  HouseholdSummary: ObserverPermissionScopeSchema.parse(ObserverPermissionScopeLiteral.HouseholdSummary),
  ChildStatus: ObserverPermissionScopeSchema.parse(ObserverPermissionScopeLiteral.ChildStatus),
  DeviceSourceState: ObserverPermissionScopeSchema.parse(ObserverPermissionScopeLiteral.DeviceSourceState),
} as const;
export const ObserverPermissionState = {
  Granted: ObserverPermissionStateSchema.parse(ObserverPermissionStateLiteral.Granted),
  Revoked: ObserverPermissionStateSchema.parse(ObserverPermissionStateLiteral.Revoked),
  Disabled: ObserverPermissionStateSchema.parse(ObserverPermissionStateLiteral.Disabled),
} as const;
