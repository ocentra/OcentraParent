import { HouseholdRole, SessionFreshnessState } from './family-household-authority';
import { type RecoveryOperation, RecoveryState } from './family-restore-lifecycle';
import {
  RegistrationEntryFailureState,
  RegistrationEntryRejectionReason,
  RegistrationEntryRoute,
  RegistrationEntryRouteContracts,
  RegistrationEntryRouteSchema,
  RegistrationIdentityProviderState,
  RegistrationSetupState,
  RegistrationStateMatrix,
  RegistrationSetupStateSchema,
} from './setup-registration-entry-contracts';
import { type HouseholdAuthorityInput } from './family-household-authority';
import { type SetupInvite, SetupInvitePurpose, SetupInviteSchema, SetupInviteState } from './family-setup-invite';
import type { RegistrationIdentityHandoff } from './setup-registration-entry-contracts';
import type { RegistrationEntryRouteContract, RegistrationStateMatrixRow } from './setup-registration-entry-contracts';

export function registrationEntryRouteContract(routeId: RegistrationEntryRoute): RegistrationEntryRouteContract {
  const parsedRouteId = RegistrationEntryRouteSchema.parse(routeId);
  const route = RegistrationEntryRouteContracts.find((candidate) => candidate.routeId === parsedRouteId);

  if (route === undefined) {
    throw new Error(`registration.unknown-route:${parsedRouteId}`);
  }

  return route;
}

export function registrationStateMatrixRow(setupState: RegistrationSetupState): RegistrationStateMatrixRow {
  const parsedSetupState = RegistrationSetupStateSchema.parse(setupState);
  const row = RegistrationStateMatrix.find((candidate) => candidate.setupState === parsedSetupState);

  if (row === undefined) {
    throw new Error(`registration.unknown-state:${parsedSetupState}`);
  }

  return row;
}

export function assertRegistrationRouteRequirements(input: RegistrationIdentityHandoff): void {
  const routeContract = registrationEntryRouteContract(input.routeId);
  const routeRequirementViolations = [
    routeContract.requiresAuthenticatedParent && input.parentAccount === null
      ? `registration.route-requires-account:${input.routeId}`
      : null,
    routeContract.requiresSetupInvite && input.setupInvite === null ? `registration.route-requires-invite:${input.routeId}` : null,
    routeContract.requiresRecoveryMethod && input.recoveryMethod === null && input.recoveryOperation === null
      ? `registration.route-requires-recovery:${input.routeId}`
      : null,
  ].find((candidate) => candidate !== null);

  if (routeRequirementViolations !== undefined && routeRequirementViolations !== null) {
    throw new Error(routeRequirementViolations);
  }
}

export function assertRegistrationEntryBoundary(input: RegistrationIdentityHandoff): void {
  const boundaryViolation = [
    input.family === null && (input.childProfile !== null || input.childDevice !== null || input.pairingIntentId !== null)
      ? 'registration.no-child-data-before-household'
      : null,
    input.childProfile === null && (input.childDevice !== null || input.pairingIntentId !== null)
      ? 'registration.child-device-before-child-profile'
      : null,
    input.childDevice !== null && input.pairingIntentId === null ? 'registration.child-device-requires-pairing-intent' : null,
  ].find((candidate) => candidate !== null);

  if (boundaryViolation !== undefined && boundaryViolation !== null) {
    throw new Error(boundaryViolation);
  }
}

export function registrationEntryRejectedReason(
  input: RegistrationIdentityHandoff & {
    setupInvite: SetupInvite | null;
    recoveryOperation: RecoveryOperation | null;
    householdAuthorityInput: HouseholdAuthorityInput | null;
  }
): RegistrationEntryRejectionReason | null {
  const rejectedReason = [
    input.setupInvite?.state === SetupInviteState.Expired ? RegistrationEntryRejectionReason.ExpiredInvite : null,
    input.setupInvite?.state === SetupInviteState.Revoked ? RegistrationEntryRejectionReason.RevokedInvite : null,
    hasCrossFamilyMismatch(input) ? RegistrationEntryRejectionReason.CrossFamily : null,
    input.setupInvite !== null && !doesSetupInviteMatchTargetRole(input.setupInvite)
      ? RegistrationEntryRejectionReason.WrongRole
      : null,
  ].find((candidate) => candidate !== null);

  return rejectedReason ?? null;
}

export function registrationEntryFailureState(
  input: RegistrationIdentityHandoff & {
    setupInvite: SetupInvite | null;
    recoveryOperation: RecoveryOperation | null;
    householdAuthorityInput: HouseholdAuthorityInput | null;
  }
): RegistrationEntryFailureState | null {
  const failureState = [
    input.providerState === RegistrationIdentityProviderState.ProviderUnavailable
      ? RegistrationEntryFailureState.ProviderUnavailable
      : null,
    input.sessionFreshnessState === SessionFreshnessState.Expired ? RegistrationEntryFailureState.SessionExpired : null,
  ].find((candidate) => candidate !== null);

  return failureState ?? null;
}

function hasCrossFamilyMismatch(
  input: RegistrationIdentityHandoff & {
    householdAuthorityInput: HouseholdAuthorityInput | null;
  }
): boolean {
  const mismatch = [
    input.householdAuthorityInput?.sameFamily === false,
    input.family !== null && input.setupInvite !== null && input.setupInvite.family.familyId !== input.family.familyId,
    input.family !== null && input.recoveryOperation !== null && input.recoveryOperation.family.familyId !== input.family.familyId,
  ].some(Boolean);

  return mismatch;
}

function doesSetupInviteMatchTargetRole(input: SetupInvite): boolean {
  const invite = SetupInviteSchema.parse(input);

  return (
    (invite.purpose === SetupInvitePurpose.CoParentInvite && invite.targetRole === HouseholdRole.CoParentGuardian) ||
    (invite.purpose === SetupInvitePurpose.ObserverInvite && invite.targetRole === HouseholdRole.Observer) ||
    (invite.purpose === SetupInvitePurpose.ChildDevicePairing &&
      invite.targetRole === HouseholdRole.ChildDeviceAgent) ||
    (invite.purpose === SetupInvitePurpose.HouseholdTransfer && invite.targetRole === HouseholdRole.ParentOwner)
  );
}
