import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentActorReferenceSchema,
  ParentDeviceReferenceSchema,
} from './references';
import { ParentContractSchemaVersionSchema, ParentTimestampSchema } from './reference-primitives';
import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';

function brandedNonEmptyStringSchema<const Brand extends string>(brand: Brand) {
  return Schema.String.pipe(Schema.minLength(1), Schema.brand(brand));
}

export const HouseholdProfileIdSchema = brandedNonEmptyStringSchema('HouseholdProfileId');
export const HouseholdDisplayNameSchema = brandedNonEmptyStringSchema('HouseholdDisplayName');
export const HouseholdMembershipIdSchema = brandedNonEmptyStringSchema('HouseholdMembershipId');
export const ParentMemberDisplayNameSchema = brandedNonEmptyStringSchema('ParentMemberDisplayName');
export const DeviceRegistrationIdSchema = brandedNonEmptyStringSchema('DeviceRegistrationId');
export const ParentControllerLeaseIdSchema = brandedNonEmptyStringSchema('ParentControllerLeaseId');
export const ObserverPermissionIdSchema = brandedNonEmptyStringSchema('ObserverPermissionId');

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

export const HouseholdRoleSchema = withParser(
  Schema.Literal(
    HouseholdRoleLiteral.ParentOwner,
    HouseholdRoleLiteral.CoParentGuardian,
    HouseholdRoleLiteral.Observer,
    HouseholdRoleLiteral.ChildProfile,
    HouseholdRoleLiteral.ChildDeviceAgent,
    HouseholdRoleLiteral.SupportAdmin
  )
);

export const HouseholdMembershipStateSchema = withParser(
  Schema.Literal(
    HouseholdMembershipStateLiteral.Invited,
    HouseholdMembershipStateLiteral.Pending,
    HouseholdMembershipStateLiteral.Active,
    HouseholdMembershipStateLiteral.Revoked,
    HouseholdMembershipStateLiteral.Disabled
  )
);

export const DeviceRoleSchema = withParser(
  Schema.Literal(DeviceRoleLiteral.ParentController, DeviceRoleLiteral.ParentObserver, DeviceRoleLiteral.ChildAgent)
);

export const DeviceTrustStateSchema = withParser(
  Schema.Literal(
    DeviceTrustStateLiteral.Pending,
    DeviceTrustStateLiteral.Trusted,
    DeviceTrustStateLiteral.Revoked,
    DeviceTrustStateLiteral.ResetRequired,
    DeviceTrustStateLiteral.Disabled
  )
);

export const ActorAccountStateSchema = withParser(
  Schema.Literal(ActorAccountStateLiteral.Active, ActorAccountStateLiteral.Suspended, ActorAccountStateLiteral.Disabled)
);

export const ChildProfileBindingStateSchema = withParser(
  Schema.Literal(
    ChildProfileBindingStateLiteral.Bound,
    ChildProfileBindingStateLiteral.Missing,
    ChildProfileBindingStateLiteral.Unassigned
  )
);

export const DeviceOwnershipScopeSchema = withParser(
  Schema.Literal(
    DeviceOwnershipScopeLiteral.ChildProfileDevice,
    DeviceOwnershipScopeLiteral.ParentControllerDevice,
    DeviceOwnershipScopeLiteral.OtherDevice
  )
);

export const SessionFreshnessStateSchema = withParser(
  Schema.Literal(
    SessionFreshnessStateLiteral.Fresh,
    SessionFreshnessStateLiteral.Stale,
    SessionFreshnessStateLiteral.Expired
  )
);

export const HouseholdAuthorizationStateSchema = withParser(
  Schema.Literal(HouseholdAuthorizationStateLiteral.Authorized, HouseholdAuthorizationStateLiteral.Rejected)
);

export const AuditRequirementStateSchema = withParser(
  Schema.Literal(AuditRequirementStateLiteral.Required, AuditRequirementStateLiteral.NotRequired)
);

export const ElevatedConfirmationStateSchema = withParser(
  Schema.Literal(ElevatedConfirmationStateLiteral.Required, ElevatedConfirmationStateLiteral.NotRequired)
);

export const HouseholdAuthorizationFailureReasonSchema = withParser(
  Schema.Literal(
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
    HouseholdAuthorizationFailureReasonLiteral.RoleNotAuthorized
  )
);

export const DeviceAuthorityActionSchema = withParser(
  Schema.Literal(
    DeviceAuthorityActionLiteral.PairChildDevice,
    DeviceAuthorityActionLiteral.RevokeChildDevice,
    DeviceAuthorityActionLiteral.ViewChildStatus,
    DeviceAuthorityActionLiteral.ChangePolicy,
    DeviceAuthorityActionLiteral.StartRemoteView,
    DeviceAuthorityActionLiteral.StartRemoteControl,
    DeviceAuthorityActionLiteral.ExportDeleteData,
    DeviceAuthorityActionLiteral.ManageBilling
  )
);

export const ParentControllerLeaseStateSchema = withParser(
  Schema.Literal(
    ParentControllerLeaseStateLiteral.Active,
    ParentControllerLeaseStateLiteral.Expired,
    ParentControllerLeaseStateLiteral.Revoked
  )
);

export const ObserverPermissionScopeSchema = withParser(
  Schema.Literal(
    ObserverPermissionScopeLiteral.HouseholdSummary,
    ObserverPermissionScopeLiteral.ChildStatus,
    ObserverPermissionScopeLiteral.DeviceSourceState
  )
);

export const ObserverPermissionStateSchema = withParser(
  Schema.Literal(
    ObserverPermissionStateLiteral.Granted,
    ObserverPermissionStateLiteral.Revoked,
    ObserverPermissionStateLiteral.Disabled
  )
);

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
export type HouseholdAuthorizationFailureReason = Infer<typeof HouseholdAuthorizationFailureReasonSchema>;
export type ParentControllerLeaseState = Infer<typeof ParentControllerLeaseStateSchema>;
export type ObserverPermissionScope = Infer<typeof ObserverPermissionScopeSchema>;
export type ObserverPermissionState = Infer<typeof ObserverPermissionStateSchema>;
export type HouseholdProfile = Infer<typeof HouseholdProfileSchema>;
export type ParentMember = Infer<typeof ParentMemberSchema>;
export type DeviceRegistration = Infer<typeof DeviceRegistrationSchema>;
export type ParentControllerLease = Infer<typeof ParentControllerLeaseSchema>;
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

export function isTrustedDeviceState(state: DeviceTrustState): boolean {
  return DeviceTrustStateSchema.parse(state) === DeviceTrustState.Trusted;
}

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

export const DeviceAuthorityAction = {
  PairChildDevice: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.PairChildDevice),
  RevokeChildDevice: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.RevokeChildDevice),
  ViewChildStatus: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.ViewChildStatus),
  ChangePolicy: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.ChangePolicy),
  StartRemoteView: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.StartRemoteView),
  StartRemoteControl: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.StartRemoteControl),
  ExportDeleteData: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.ExportDeleteData),
  ManageBilling: DeviceAuthorityActionSchema.parse(DeviceAuthorityActionLiteral.ManageBilling),
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

export function isActiveParentMember(input: ParentMember): boolean {
  const member = ParentMemberSchema.parse(input);

  return (
    member.membershipState === HouseholdMembershipState.Active &&
    member.role !== HouseholdRole.ChildProfile &&
    member.role !== HouseholdRole.ChildDeviceAgent
  );
}

export function canHouseholdRoleAuthorizeAction(role: HouseholdRole, action: DeviceAuthorityAction): boolean {
  const parsedRole = HouseholdRoleSchema.parse(role);
  const parsedAction = DeviceAuthorityActionSchema.parse(action);

  switch (parsedAction) {
    case DeviceAuthorityAction.PairChildDevice:
    case DeviceAuthorityAction.RevokeChildDevice:
    case DeviceAuthorityAction.ChangePolicy:
      return parsedRole === HouseholdRole.ParentOwner || parsedRole === HouseholdRole.CoParentGuardian;
    case DeviceAuthorityAction.ViewChildStatus:
      return (
        parsedRole === HouseholdRole.ParentOwner ||
        parsedRole === HouseholdRole.CoParentGuardian ||
        parsedRole === HouseholdRole.Observer
      );
    case DeviceAuthorityAction.StartRemoteView:
      return (
        parsedRole === HouseholdRole.ParentOwner ||
        parsedRole === HouseholdRole.CoParentGuardian ||
        parsedRole === HouseholdRole.Observer
      );
    case DeviceAuthorityAction.StartRemoteControl:
      return parsedRole === HouseholdRole.ParentOwner || parsedRole === HouseholdRole.CoParentGuardian;
    case DeviceAuthorityAction.ExportDeleteData:
    case DeviceAuthorityAction.ManageBilling:
      return parsedRole === HouseholdRole.ParentOwner;
    default:
      return false;
  }
}

export function canParentMemberAuthorizeDeviceAction(
  member: ParentMember,
  targetFamily: Infer<typeof FamilyReferenceSchema>,
  action: DeviceAuthorityAction,
  actorAccountState: ActorAccountState
): boolean {
  const parsedMember = ParentMemberSchema.parse(member);
  const parsedFamily = FamilyReferenceSchema.parse(targetFamily);
  const parsedActorAccountState = ActorAccountStateSchema.parse(actorAccountState);

  if (parsedActorAccountState !== ActorAccountState.Active) {
    return false;
  }

  if (parsedMember.family.familyId !== parsedFamily.familyId) {
    return false;
  }

  if (parsedMember.membershipState !== HouseholdMembershipState.Active) {
    return false;
  }

  if (parsedMember.role === HouseholdRole.SupportAdmin) {
    return false;
  }

  return canHouseholdRoleAuthorizeAction(parsedMember.role, action);
}

export function authorizeHouseholdAction(input: HouseholdAuthorityInput): HouseholdAuthorityDecision {
  const parsedInput = HouseholdAuthorityInputSchema.parse(input);

  if (!parsedInput.sameFamily) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.ExternalHousehold, parsedInput.action);
  }

  if (parsedInput.membershipState !== HouseholdMembershipState.Active) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.MembershipNotActive, parsedInput.action);
  }

  if (parsedInput.actorAccountState !== ActorAccountState.Active) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.AccountNotActive, parsedInput.action);
  }

  if (!isTrustedDeviceState(parsedInput.deviceTrustState)) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.DeviceNotTrusted, parsedInput.action);
  }

  if (requiresFreshSession(parsedInput.action) && parsedInput.sessionFreshnessState !== SessionFreshnessState.Fresh) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.SessionNotFresh, parsedInput.action);
  }

  if (
    requiresBoundChildScope(parsedInput.action) &&
    parsedInput.childProfileBindingState !== ChildProfileBindingState.Bound
  ) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.ChildProfileNotBound, parsedInput.action);
  }

  if (
    requiresChildProfileDeviceScope(parsedInput.action) &&
    parsedInput.deviceOwnershipScope !== DeviceOwnershipScope.ChildProfileDevice
  ) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.WrongDeviceScope, parsedInput.action);
  }

  if (requiresCapabilityGrant(parsedInput.action) && !parsedInput.capabilityGranted) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.MissingCapabilityGrant, parsedInput.action);
  }

  if (!canHouseholdRoleAuthorizeAction(parsedInput.actorRole, parsedInput.action)) {
    return rejectedHouseholdAction(HouseholdAuthorizationFailureReason.RoleNotAuthorized, parsedInput.action);
  }

  const controllerLeaseFailureReason = controllerLeaseFailureReasonForAction(parsedInput);
  if (controllerLeaseFailureReason !== null) {
    return rejectedHouseholdAction(controllerLeaseFailureReason, parsedInput.action);
  }

  return HouseholdAuthorityDecisionSchema.parse({
    authorizationState: HouseholdAuthorizationState.Authorized,
    auditRequirementState: auditRequirementState(parsedInput.action),
    elevatedConfirmationState: elevatedConfirmationState(parsedInput.action),
    failureReason: null,
  });
}

export function isTrustedChildAgentRegistrationForProfile(
  registration: DeviceRegistration,
  childProfile: Infer<typeof ChildProfileReferenceSchema>
): boolean {
  const parsedRegistration = DeviceRegistrationSchema.parse(registration);
  const parsedChildProfile = ChildProfileReferenceSchema.parse(childProfile);

  return (
    parsedRegistration.deviceRole === DeviceRole.ChildAgent &&
    isTrustedDeviceState(parsedRegistration.trustState) &&
    parsedRegistration.device.childProfileId === parsedChildProfile.childProfileId
  );
}

function rejectedHouseholdAction(
  failureReason: HouseholdAuthorizationFailureReason,
  action: DeviceAuthorityAction
): HouseholdAuthorityDecision {
  return HouseholdAuthorityDecisionSchema.parse({
    authorizationState: HouseholdAuthorizationState.Rejected,
    auditRequirementState: AuditRequirementState.Required,
    elevatedConfirmationState: elevatedConfirmationState(action),
    failureReason,
  });
}

function requiresCapabilityGrant(action: DeviceAuthorityAction): boolean {
  return action === DeviceAuthorityAction.StartRemoteView || action === DeviceAuthorityAction.StartRemoteControl;
}

function controllerLeaseFailureReasonForAction(
  input: HouseholdAuthorityInput
): HouseholdAuthorizationFailureReason | null {
  if (!requiresControllerLease(input.action)) {
    return null;
  }

  if (input.controllerLeaseState === undefined || input.controllerLeaseState === null) {
    return HouseholdAuthorizationFailureReason.ControllerLeaseRequired;
  }

  if (input.controllerLeaseState === ParentControllerLeaseState.Expired) {
    return HouseholdAuthorizationFailureReason.ControllerLeaseExpired;
  }

  if (input.controllerLeaseState === ParentControllerLeaseState.Revoked) {
    return HouseholdAuthorizationFailureReason.ControllerLeaseRevoked;
  }

  return null;
}

function requiresFreshSession(action: DeviceAuthorityAction): boolean {
  return (
    action === DeviceAuthorityAction.ChangePolicy ||
    action === DeviceAuthorityAction.StartRemoteView ||
    action === DeviceAuthorityAction.StartRemoteControl ||
    action === DeviceAuthorityAction.ExportDeleteData ||
    action === DeviceAuthorityAction.ManageBilling
  );
}

function requiresBoundChildScope(action: DeviceAuthorityAction): boolean {
  return (
    action === DeviceAuthorityAction.PairChildDevice ||
    action === DeviceAuthorityAction.RevokeChildDevice ||
    action === DeviceAuthorityAction.ViewChildStatus ||
    action === DeviceAuthorityAction.ChangePolicy ||
    action === DeviceAuthorityAction.StartRemoteView ||
    action === DeviceAuthorityAction.StartRemoteControl
  );
}

function requiresChildProfileDeviceScope(action: DeviceAuthorityAction): boolean {
  return requiresBoundChildScope(action);
}

function requiresControllerLease(action: DeviceAuthorityAction): boolean {
  return action === DeviceAuthorityAction.StartRemoteView || action === DeviceAuthorityAction.StartRemoteControl;
}

function auditRequirementState(action: DeviceAuthorityAction): AuditRequirementState {
  if (action === DeviceAuthorityAction.ViewChildStatus) {
    return AuditRequirementState.NotRequired;
  }

  return AuditRequirementState.Required;
}

function elevatedConfirmationState(action: DeviceAuthorityAction): ElevatedConfirmationState {
  if (
    action === DeviceAuthorityAction.RevokeChildDevice ||
    action === DeviceAuthorityAction.StartRemoteControl ||
    action === DeviceAuthorityAction.ExportDeleteData ||
    action === DeviceAuthorityAction.ManageBilling
  ) {
    return ElevatedConfirmationState.Required;
  }

  return ElevatedConfirmationState.NotRequired;
}
