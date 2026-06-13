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
  Disabled: 'disabled',
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
    DeviceTrustStateLiteral.Disabled
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

export type HouseholdRole = Infer<typeof HouseholdRoleSchema>;
export type HouseholdMembershipState = Infer<typeof HouseholdMembershipStateSchema>;
export type DeviceRole = Infer<typeof DeviceRoleSchema>;
export type DeviceTrustState = Infer<typeof DeviceTrustStateSchema>;
export type DeviceAuthorityAction = Infer<typeof DeviceAuthorityActionSchema>;
export type ParentControllerLeaseState = Infer<typeof ParentControllerLeaseStateSchema>;
export type ObserverPermissionScope = Infer<typeof ObserverPermissionScopeSchema>;
export type ObserverPermissionState = Infer<typeof ObserverPermissionStateSchema>;
export type HouseholdProfile = Infer<typeof HouseholdProfileSchema>;
export type ParentMember = Infer<typeof ParentMemberSchema>;
export type DeviceRegistration = Infer<typeof DeviceRegistrationSchema>;
export type ParentControllerLease = Infer<typeof ParentControllerLeaseSchema>;
export type ObserverPermission = Infer<typeof ObserverPermissionSchema>;

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
  Disabled: DeviceTrustStateSchema.parse(DeviceTrustStateLiteral.Disabled),
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
