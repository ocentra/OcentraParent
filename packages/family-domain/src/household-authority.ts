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
      return parsedRole === HouseholdRole.ParentOwner || parsedRole === HouseholdRole.CoParentGuardian;
    case DeviceAuthorityActionLiteral.ViewChildStatus:
    case DeviceAuthorityActionLiteral.StartRemoteView:
      return (
        parsedRole === HouseholdRole.ParentOwner ||
        parsedRole === HouseholdRole.CoParentGuardian ||
        parsedRole === HouseholdRole.Observer
      );
    case DeviceAuthorityActionLiteral.StartRemoteControl:
      return parsedRole === HouseholdRole.ParentOwner || parsedRole === HouseholdRole.CoParentGuardian;
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
  const family = FamilyReferenceSchema.parse(input.family);
  const parentAccount = ParentAccountReferenceSchema.parse(input.parentAccount);
  const actionDevice = ParentDeviceReferenceSchema.parse(input.actionDevice);
  const targetChildProfile =
    input.targetChildProfile === null ? null : ChildProfileReferenceSchema.parse(input.targetChildProfile);
  const action = DeviceAuthorityActionSchema.parse(input.action);

  if (input.assertion === null) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.Required,
    };
  }

  const assertion = ParentStepUpAssertionSchema.parse(input.assertion);
  const observedAt = Schema.decodeUnknownSync(ParentTimestampSchema)(input.observedAt);
  const expectedNonce = input.expectedNonce === undefined ? null : input.expectedNonce;

  if (assertion.expiresAt < observedAt) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.Expired,
    };
  }

  if (assertion.family.familyId !== family.familyId) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.WrongHousehold,
    };
  }

  if (assertion.parentAccount.parentAccountId !== parentAccount.parentAccountId) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.WrongAccount,
    };
  }

  if (assertion.action !== action) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.WrongAction,
    };
  }

  if (
    assertion.actionDevice.deviceId !== actionDevice.deviceId ||
    assertion.actionDevice.childProfileId !== actionDevice.childProfileId
  ) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.WrongDevice,
    };
  }

  if (
    (assertion.targetChildProfile === null) !== (targetChildProfile === null) ||
    (assertion.targetChildProfile !== null &&
      targetChildProfile !== null &&
      assertion.targetChildProfile.childProfileId !== targetChildProfile.childProfileId)
  ) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.WrongTarget,
    };
  }

  if (expectedNonce !== null && assertion.nonce !== expectedNonce) {
    return {
      valid: false,
      failureReason: ParentStepUpValidationFailureReason.ReplayRejected,
    };
  }

  return {
    valid: true,
    failureReason: null,
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
    requiresBoundChildScope(parsedInput.action) &&
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
