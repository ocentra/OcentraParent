import { type Infer, Schema } from '@ocentra-parent/schema-domain/effect';
import {
  ActorAccountState,
  ActorAccountStateSchema,
  AuditRequirementState,
  ChildProfileBindingState,
  type DeviceAuthorityAction,
  DeviceAuthorityActionLiteral,
  DeviceAuthorityActionSchema,
  type DeviceRegistration,
  DeviceRegistrationSchema,
  DeviceOwnershipScope,
  DeviceRole,
  DeviceTrustState,
  DeviceTrustStateSchema,
  ElevatedConfirmationState,
  type HouseholdAuthorityDecision,
  HouseholdAuthorityDecisionSchema,
  type HouseholdAuthorityInput,
  HouseholdAuthorityInputSchema,
  HouseholdAuthorizationFailureReason,
  HouseholdAuthorizationState,
  HouseholdMembershipState,
  HouseholdRole,
  HouseholdRoleSchema,
  ParentControllerLeaseState,
  type ParentMember,
  ParentMemberSchema,
  type ParentStepUpAssertion,
  ParentStepUpAssertionSchema,
  ParentStepUpValidationFailureReason,
  SessionFreshnessState,
} from '@ocentra-parent/schema-domain/family-household-authority';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  ChildProfileReferenceSchema,
  FamilyReferenceSchema,
  ParentAccountReferenceSchema,
  ParentDeviceReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';

export function isTrustedDeviceState(state: DeviceTrustState): boolean {
  return DeviceTrustStateSchema.parse(state) === DeviceTrustState.Trusted;
}

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
    case DeviceAuthorityActionLiteral.PairChildDevice:
    case DeviceAuthorityActionLiteral.RevokeChildDevice:
    case DeviceAuthorityActionLiteral.ChangePolicy:
      return isParentAuthorityRole(parsedRole);
    case DeviceAuthorityActionLiteral.ViewChildStatus:
    case DeviceAuthorityActionLiteral.StartRemoteView:
      return canObserveHouseholdRole(parsedRole);
    case DeviceAuthorityActionLiteral.StartRemoteControl:
      return isParentAuthorityRole(parsedRole);
    case DeviceAuthorityActionLiteral.ExportDeleteData:
    case DeviceAuthorityActionLiteral.ManageBilling:
      return parsedRole === HouseholdRole.ParentOwner;
    default:
      return false;
  }
}

export function requiresParentStepUp(action: DeviceAuthorityAction): boolean {
  const parsedAction = DeviceAuthorityActionSchema.parse(action);

  return (
    parsedAction === DeviceAuthorityActionLiteral.PairChildDevice ||
    parsedAction === DeviceAuthorityActionLiteral.RevokeChildDevice ||
    parsedAction === DeviceAuthorityActionLiteral.ChangePolicy ||
    parsedAction === DeviceAuthorityActionLiteral.StartRemoteControl ||
    parsedAction === DeviceAuthorityActionLiteral.ExportDeleteData ||
    parsedAction === DeviceAuthorityActionLiteral.ManageBilling
  );
}

export function validateParentStepUpAssertion(input: {
  assertion: ParentStepUpAssertion | null;
  family: Infer<typeof FamilyReferenceSchema>;
  parentAccount: Infer<typeof ParentAccountReferenceSchema>;
  actionDevice: Infer<typeof ParentDeviceReferenceSchema>;
  targetChildProfile: Infer<typeof ChildProfileReferenceSchema> | null;
  action: DeviceAuthorityAction;
  observedAt: Infer<typeof ParentTimestampSchema>;
  expectedNonce?: string | null;
}): {
  valid: boolean;
  failureReason: ParentStepUpValidationFailureReason | null;
} {
  if (input.assertion === null) {
    return rejectedStepUpValidation(ParentStepUpValidationFailureReason.Required);
  }

  const family = FamilyReferenceSchema.parse(input.family);
  const parentAccount = ParentAccountReferenceSchema.parse(input.parentAccount);
  const actionDevice = ParentDeviceReferenceSchema.parse(input.actionDevice);
  const targetChildProfile =
    input.targetChildProfile === null ? null : ChildProfileReferenceSchema.parse(input.targetChildProfile);
  const action = DeviceAuthorityActionSchema.parse(input.action);
  const assertion = ParentStepUpAssertionSchema.parse(input.assertion);
  const observedAt = Schema.decodeUnknownSync(ParentTimestampSchema)(input.observedAt);
  const expectedNonce = input.expectedNonce === undefined ? null : input.expectedNonce;
  const failureReason = stepUpValidationFailureReason({
    assertion,
    family,
    parentAccount,
    actionDevice,
    targetChildProfile,
    action,
    observedAt,
    expectedNonce,
  });

  return failureReason === null ? { valid: true, failureReason: null } : rejectedStepUpValidation(failureReason);
}

type ParsedStepUpValidationInput = {
  readonly assertion: ParentStepUpAssertion;
  readonly family: Infer<typeof FamilyReferenceSchema>;
  readonly parentAccount: Infer<typeof ParentAccountReferenceSchema>;
  readonly actionDevice: Infer<typeof ParentDeviceReferenceSchema>;
  readonly targetChildProfile: Infer<typeof ChildProfileReferenceSchema> | null;
  readonly action: DeviceAuthorityAction;
  readonly observedAt: Infer<typeof ParentTimestampSchema>;
  readonly expectedNonce: string | null;
};

function stepUpValidationFailureReason(input: ParsedStepUpValidationInput): ParentStepUpValidationFailureReason | null {
  if (input.assertion.expiresAt < input.observedAt) {
    return ParentStepUpValidationFailureReason.Expired;
  }

  if (input.assertion.family.familyId !== input.family.familyId) {
    return ParentStepUpValidationFailureReason.WrongHousehold;
  }

  if (input.assertion.parentAccount.parentAccountId !== input.parentAccount.parentAccountId) {
    return ParentStepUpValidationFailureReason.WrongAccount;
  }

  if (input.assertion.action !== input.action) {
    return ParentStepUpValidationFailureReason.WrongAction;
  }

  if (
    input.assertion.actionDevice.deviceId !== input.actionDevice.deviceId ||
    input.assertion.actionDevice.childProfileId !== input.actionDevice.childProfileId
  ) {
    return ParentStepUpValidationFailureReason.WrongDevice;
  }

  if (!matchesTargetChildProfile(input.assertion.targetChildProfile, input.targetChildProfile)) {
    return ParentStepUpValidationFailureReason.WrongTarget;
  }

  if (input.expectedNonce !== null && input.assertion.nonce !== input.expectedNonce) {
    return ParentStepUpValidationFailureReason.ReplayRejected;
  }

  return null;
}

function matchesTargetChildProfile(
  assertedTarget: Infer<typeof ChildProfileReferenceSchema> | null,
  expectedTarget: Infer<typeof ChildProfileReferenceSchema> | null
): boolean {
  if ((assertedTarget === null) !== (expectedTarget === null)) {
    return false;
  }

  return (
    assertedTarget === null ||
    expectedTarget === null ||
    assertedTarget.childProfileId === expectedTarget.childProfileId
  );
}

function rejectedStepUpValidation(failureReason: ParentStepUpValidationFailureReason): {
  valid: boolean;
  failureReason: ParentStepUpValidationFailureReason;
} {
  return {
    valid: false,
    failureReason,
  };
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
  const baseFailureReason = householdAuthorizationFailureReason(parsedInput);

  if (baseFailureReason !== null) {
    return rejectedHouseholdAction(baseFailureReason, parsedInput.action);
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

function isParentAuthorityRole(role: HouseholdRole): boolean {
  return role === HouseholdRole.ParentOwner || role === HouseholdRole.CoParentGuardian;
}

function canObserveHouseholdRole(role: HouseholdRole): boolean {
  return isParentAuthorityRole(role) || role === HouseholdRole.Observer;
}

function sameFamilyFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  return input.sameFamily ? null : HouseholdAuthorizationFailureReason.ExternalHousehold;
}

function membershipFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  return input.membershipState === HouseholdMembershipState.Active
    ? null
    : HouseholdAuthorizationFailureReason.MembershipNotActive;
}

function actorAccountFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  return input.actorAccountState === ActorAccountState.Active
    ? null
    : HouseholdAuthorizationFailureReason.AccountNotActive;
}

function deviceTrustFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  return isTrustedDeviceState(input.deviceTrustState) ? null : HouseholdAuthorizationFailureReason.DeviceNotTrusted;
}

function sessionFreshnessFailureReasonForAction(
  input: HouseholdAuthorityInput
): HouseholdAuthorizationFailureReason | null {
  return requiresFreshSession(input.action) && input.sessionFreshnessState !== SessionFreshnessState.Fresh
    ? HouseholdAuthorizationFailureReason.SessionNotFresh
    : null;
}

function childScopeFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  if (!requiresBoundChildScope(input.action)) {
    return null;
  }

  if (input.childProfileBindingState !== ChildProfileBindingState.Bound) {
    return HouseholdAuthorizationFailureReason.ChildProfileNotBound;
  }

  return input.deviceOwnershipScope === DeviceOwnershipScope.ChildProfileDevice
    ? null
    : HouseholdAuthorizationFailureReason.WrongDeviceScope;
}

function capabilityGrantFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  return requiresCapabilityGrant(input.action) && !input.capabilityGranted
    ? HouseholdAuthorizationFailureReason.MissingCapabilityGrant
    : null;
}

function roleAuthorizationFailureReason(input: HouseholdAuthorityInput): HouseholdAuthorizationFailureReason | null {
  return canHouseholdRoleAuthorizeAction(input.actorRole, input.action)
    ? null
    : HouseholdAuthorizationFailureReason.RoleNotAuthorized;
}

function householdAuthorizationFailureReason(
  input: HouseholdAuthorityInput
): HouseholdAuthorizationFailureReason | null {
  const failureChecks = [
    sameFamilyFailureReason,
    membershipFailureReason,
    actorAccountFailureReason,
    deviceTrustFailureReason,
    sessionFreshnessFailureReasonForAction,
    childScopeFailureReason,
    capabilityGrantFailureReason,
    roleAuthorizationFailureReason,
  ] as const;

  for (const failureCheck of failureChecks) {
    const failureReason = failureCheck(input);
    if (failureReason !== null) {
      return failureReason;
    }
  }

  return null;
}

function requiresCapabilityGrant(action: DeviceAuthorityAction): boolean {
  return (
    action === DeviceAuthorityActionLiteral.StartRemoteView ||
    action === DeviceAuthorityActionLiteral.StartRemoteControl
  );
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
    action === DeviceAuthorityActionLiteral.ChangePolicy ||
    action === DeviceAuthorityActionLiteral.StartRemoteView ||
    action === DeviceAuthorityActionLiteral.StartRemoteControl ||
    action === DeviceAuthorityActionLiteral.ExportDeleteData ||
    action === DeviceAuthorityActionLiteral.ManageBilling
  );
}

function requiresBoundChildScope(action: DeviceAuthorityAction): boolean {
  return (
    action === DeviceAuthorityActionLiteral.PairChildDevice ||
    action === DeviceAuthorityActionLiteral.RevokeChildDevice ||
    action === DeviceAuthorityActionLiteral.ViewChildStatus ||
    action === DeviceAuthorityActionLiteral.ChangePolicy ||
    action === DeviceAuthorityActionLiteral.StartRemoteView ||
    action === DeviceAuthorityActionLiteral.StartRemoteControl
  );
}

function requiresControllerLease(action: DeviceAuthorityAction): boolean {
  return (
    action === DeviceAuthorityActionLiteral.StartRemoteView ||
    action === DeviceAuthorityActionLiteral.StartRemoteControl
  );
}

function auditRequirementState(action: DeviceAuthorityAction): AuditRequirementState {
  if (action === DeviceAuthorityActionLiteral.ViewChildStatus) {
    return AuditRequirementState.NotRequired;
  }

  return AuditRequirementState.Required;
}

function elevatedConfirmationState(action: DeviceAuthorityAction): ElevatedConfirmationState {
  if (requiresParentStepUp(action)) {
    return ElevatedConfirmationState.Required;
  }

  return ElevatedConfirmationState.NotRequired;
}
