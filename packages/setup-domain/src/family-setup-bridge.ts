import {
  type HouseholdAuthorityInput,
  DeviceTrustState,
  HouseholdAuthorityInputSchema,
  HouseholdAuthorizationFailureReason,
  HouseholdAuthorizationState,
  ParentStepUpValidationFailureReason,
} from '@ocentra-parent/schema-domain/family-household-authority';
import {
  requiresParentStepUp,
  authorizeHouseholdAction,
  validateParentStepUpAssertion,
} from '@ocentra-parent/family-domain/household-authority';
import { ParentAccountReferenceSchema } from '@ocentra-parent/schema-domain/family-references';
import {
  deviceTrustStateForRecoveryOperation,
  isSetupInviteSinglePurpose,
  recoveryDataCustodyHandoffState,
  recoveryRequiresAuditedSupport,
} from '@ocentra-parent/family-domain/setup-lifecycle';
import {
  type RecoveryOperation as FamilyRecoveryOperation,
  RecoveryBundleState,
  RecoveryDataCustodyHandoffState,
  RecoveryState as FamilyRecoveryState,
} from '@ocentra-parent/schema-domain/family-restore-lifecycle';
import {
  type SetupInvite,
  SetupInviteSchema,
  SetupInviteState,
} from '@ocentra-parent/schema-domain/family-setup-invite';
import { type Infer } from '@ocentra-parent/schema-domain/effect';
import {
  SetupFamilyReadinessInputSchema,
  SetupFamilyRecoveryOperationInputSchema,
  SetupPairingProjectionSchema,
  SetupRecoveryProjectionSchema,
  type SetupFamilyReadinessInput,
  type SetupFamilyRecoveryOperationInput,
  type SetupPairingProjection,
  type SetupRecoveryProjection,
} from '@ocentra-parent/schema-domain/family-setup-bridge';
import {
  SetupAccountReadinessState,
  SetupChildInstallState,
  SetupChildServiceState,
  SetupDataCustodySyncState,
  SetupNetworkReachabilityState,
  SetupPermissionReadinessState,
  createSetupReadinessChecklist,
  deriveSetupChildInstallStateFromAppState,
  deriveSetupChildServiceStateFromAppState,
  type SetupReadinessChecklistItem,
  type SetupReadinessReport,
  SetupReadinessReportSchema,
  type SetupRecoveryOperation,
  SetupRecoveryOperationSchema,
  SetupRecoveryState,
} from '@ocentra-parent/schema-domain/setup-readiness';
import {
  deriveParentStepUpAssertionFromSetupPairingApproval,
  SetupPairingFailureReason,
  SetupPairingState,
} from '@ocentra-parent/schema-domain/setup-pairing-intent';

function resolvedChildInstallState(input: SetupFamilyReadinessInput) {
  return input.childInstallState ?? deriveSetupChildInstallStateFromAppState(input.childAppState);
}

function resolvedChildServiceState(input: SetupFamilyReadinessInput) {
  return input.childServiceState ?? deriveSetupChildServiceStateFromAppState(input.childAppState);
}

type SetupPairingAuthorityDecision = ReturnType<typeof authorizeHouseholdAction>;
type RequiredPairingStepUpStatus = ReturnType<typeof evaluateRequiredPairingStepUp>;
type SetupPairingContext = {
  readonly parsedInput: SetupFamilyReadinessInput;
  readonly authorityDecision: SetupPairingAuthorityDecision;
  readonly accountState: SetupAccountReadinessState;
  readonly familyRecoveryState: SetupRecoveryState;
  readonly childInstallState: ReturnType<typeof resolvedChildInstallState>;
  readonly childServiceState: ReturnType<typeof resolvedChildServiceState>;
  readonly inviteAccepted: boolean;
};

function pairingProjectionForImmediateFailures(context: SetupPairingContext): SetupPairingProjection | null {
  const { parsedInput, accountState, childServiceState } = context;

  if (parsedInput.replayDetected) {
    return pairingProjection(
      SetupPairingState.Replayed,
      SetupPairingFailureReason.ReplayRejected,
      accountState,
      SetupRecoveryState.Required
    );
  }

  if (parsedInput.staleCode) {
    return pairingProjection(
      SetupPairingState.StaleSignedHello,
      SetupPairingFailureReason.StaleSignedHello,
      accountState,
      SetupRecoveryState.Required
    );
  }

  if (parsedInput.setupInvite.state === SetupInviteState.Expired) {
    return pairingProjection(
      SetupPairingState.Expired,
      SetupPairingFailureReason.StaleCode,
      accountState,
      SetupRecoveryState.Required
    );
  }

  if (targetAccountMismatch(parsedInput.setupInvite, parsedInput.parentAccount)) {
    return pairingProjection(
      SetupPairingState.Untrusted,
      SetupPairingFailureReason.WrongAccount,
      SetupAccountReadinessState.WrongAccount,
      SetupRecoveryState.Required
    );
  }

  if (!isSetupInviteSinglePurpose(parsedInput.setupInvite)) {
    return pairingProjection(
      SetupPairingState.WrongHousehold,
      SetupPairingFailureReason.WrongHousehold,
      SetupAccountReadinessState.RecoveryRequired,
      SetupRecoveryState.Required
    );
  }

  if (
    parsedInput.childDeviceRevoked ||
    parsedInput.setupInvite.state === SetupInviteState.Revoked ||
    childServiceState === SetupChildServiceState.Revoked
  ) {
    return pairingProjection(
      SetupPairingState.Revoked,
      SetupPairingFailureReason.RevokedDevice,
      accountState,
      SetupRecoveryState.Required
    );
  }

  return null;
}

function pairingProjectionForAuthorityAndReadiness(context: SetupPairingContext): SetupPairingProjection | null {
  const {
    parsedInput,
    authorityDecision,
    accountState,
    familyRecoveryState,
    inviteAccepted,
  } = context;

  if (rejectedAuthorityRequiresRecoveryProjection(context)) {
    return pairingProjection(
      pairingStateForRejectedAuthority(authorityDecision.failureReason),
      pairingFailureReasonForRejectedAuthority(authorityDecision.failureReason),
      accountStateForRejectedAuthority(parsedInput.householdAuthorityInput, authorityDecision.failureReason),
      SetupRecoveryState.Required
    );
  }

  if (inviteAcceptedWithPendingSetupDependencies(context)) {
    return pairingProjection(SetupPairingState.Accepted, null, accountState, familyRecoveryState);
  }

  if (parsedInput.permissionState !== SetupPermissionReadinessState.Granted) {
    return pairingProjection(
      SetupPairingState.Untrusted,
      SetupPairingFailureReason.PermissionLoss,
      accountState,
      SetupRecoveryState.Required
    );
  }

  if (pairingChildIsOffline(context)) {
    return pairingProjection(
      inviteAccepted ? SetupPairingState.Accepted : SetupPairingState.Untrusted,
      SetupPairingFailureReason.OfflineChild,
      accountState,
      inviteAccepted ? familyRecoveryState : SetupRecoveryState.Required
    );
  }

  return null;
}

function pairingProjectionFromStepUpStatus(
  context: SetupPairingContext,
  requiredStepUpStatus: RequiredPairingStepUpStatus
): SetupPairingProjection {
  const { accountState, familyRecoveryState, inviteAccepted } = context;

  switch (requiredStepUpStatus.kind) {
    case 'pending':
      return pairingProjection(SetupPairingState.Accepted, null, accountState, familyRecoveryState);
    case 'rejected':
      return pairingProjection(
        requiredStepUpStatus.pairingState,
        requiredStepUpStatus.failureReason,
        accountStateForRejectedStepUp(accountState, requiredStepUpStatus.failureReason),
        SetupRecoveryState.Required
      );
    default:
      break;
  }

  if (acceptedInviteWithPendingTrustedDevice(context, requiredStepUpStatus)) {
    return pairingProjection(
      SetupPairingState.Accepted,
      null,
      accountStateForPendingTrustedInvite(familyRecoveryState),
      familyRecoveryState
    );
  }

  if (familyRecoveryState === SetupRecoveryState.Recovered && inviteAccepted) {
    return pairingProjection(
      SetupPairingState.Recovered,
      null,
      SetupAccountReadinessState.Ready,
      SetupRecoveryState.Recovered
    );
  }

  if (familyRecoveryState !== SetupRecoveryState.Normal) {
    return pairingProjection(
      inviteAccepted ? SetupPairingState.Accepted : SetupPairingState.Displayed,
      null,
      SetupAccountReadinessState.RecoveryRequired,
      familyRecoveryState
    );
  }

  if (inviteAccepted) {
    return pairingProjection(
      SetupPairingState.Trusted,
      null,
      SetupAccountReadinessState.Ready,
      SetupRecoveryState.Normal
    );
  }

  return pairingProjection(
    SetupPairingState.Displayed,
    null,
    SetupAccountReadinessState.Ready,
    SetupRecoveryState.Normal
  );
}

export function deriveSetupPairingProjectionFromFamilyContext(
  input: SetupFamilyReadinessInput
): SetupPairingProjection {
  const parsedInput = SetupFamilyReadinessInputSchema.parse(input);
  const authorityDecision = authorizeHouseholdAction(parsedInput.householdAuthorityInput);
  const pairingContext: SetupPairingContext = {
    parsedInput,
    authorityDecision,
    accountState: accountStateForPairing(parsedInput, authorityDecision),
    familyRecoveryState: setupRecoveryStateFromFamilyOperation(parsedInput.recoveryOperation),
    childInstallState: resolvedChildInstallState(parsedInput),
    childServiceState: resolvedChildServiceState(parsedInput),
    inviteAccepted: parsedInput.setupInvite.state === SetupInviteState.Accepted,
  };
  const immediateFailureProjection = pairingProjectionForImmediateFailures(pairingContext);

  if (immediateFailureProjection !== null) {
    return immediateFailureProjection;
  }

  const readinessProjection = pairingProjectionForAuthorityAndReadiness(pairingContext);

  if (readinessProjection !== null) {
    return readinessProjection;
  }

  return pairingProjectionFromStepUpStatus(pairingContext, evaluateRequiredPairingStepUp(parsedInput));
}

export function deriveSetupRecoveryProjectionFromFamilyContext(
  input: SetupFamilyReadinessInput,
  pairing: SetupPairingProjection
): SetupRecoveryProjection {
  const parsedInput = SetupFamilyReadinessInputSchema.parse(input);
  const parsedPairing = SetupPairingProjectionSchema.parse(pairing);
  const childServiceState = resolvedChildServiceState(parsedInput);
  const familyRecoveryState = setupRecoveryStateFromFamilyOperation(parsedInput.recoveryOperation);
  const recoveryState = mergedRecoveryState(parsedPairing.recoveryState, familyRecoveryState);
  const custodyBlockedByRecovery = recoveryBlocksCustodyHandoff(parsedInput.recoveryOperation, recoveryState);
  const custodySyncPending = custodySyncIsPending(parsedInput, childServiceState);
  const dataCustodySyncState = dataCustodySyncStateFromRecovery(
    custodyBlockedByRecovery,
    recoveryState,
    custodySyncPending
  );

  return SetupRecoveryProjectionSchema.parse({
    accountState: recoveryProjectionAccountState(parsedPairing.accountState, recoveryState),
    recoveryState,
    dataCustodySyncState,
  });
}

export function createSetupReadinessReportFromFamilyContext(input: SetupFamilyReadinessInput): SetupReadinessReport {
  const parsedInput = SetupFamilyReadinessInputSchema.parse(input);
  const pairingProjection = deriveSetupPairingProjectionFromFamilyContext(parsedInput);
  const recoveryProjection = deriveSetupRecoveryProjectionFromFamilyContext(parsedInput, pairingProjection);
  const childInstallState = resolvedChildInstallState(parsedInput);
  const childServiceState = resolvedChildServiceState(parsedInput);

  const reportWithoutChecklist = SetupReadinessReportSchema.parse({
    schemaVersion: parsedInput.schemaVersion,
    readinessReportId: parsedInput.readinessReportId,
    family: parsedInput.family,
    parentAccount: parsedInput.parentAccount,
    parentDevice: parsedInput.parentDevice,
    childProfile: parsedInput.childProfile,
    pairingIntentId: parsedInput.pairingIntentId,
    accountState: recoveryProjection.accountState,
    parentAppState: parsedInput.parentAppState,
    childAppState: parsedInput.childAppState,
    childInstallState,
    childServiceState,
    permissionState: parsedInput.permissionState,
    pairingState: pairingProjection.pairingState,
    policyBaselineState: parsedInput.policyBaselineState,
    dataCustodySyncState: recoveryProjection.dataCustodySyncState,
    networkReachabilityState: parsedInput.networkReachabilityState,
    recoveryState: recoveryProjection.recoveryState,
    observedAt: parsedInput.observedAt,
    checklist: [],
  });

  return SetupReadinessReportSchema.parse({
    ...reportWithoutChecklist,
    checklist: createSetupReadinessChecklist(reportWithoutChecklist) as SetupReadinessChecklistItem[],
  });
}

export function createSetupRecoveryOperationFromFamilyRecovery(
  input: SetupFamilyRecoveryOperationInput
): SetupRecoveryOperation {
  const parsedInput = SetupFamilyRecoveryOperationInputSchema.parse(input);
  const recoveryState = setupRecoveryStateFromFamilyOperation(parsedInput.familyRecoveryOperation);

  return SetupRecoveryOperationSchema.parse({
    schemaVersion: parsedInput.familyRecoveryOperation.schemaVersion,
    recoveryOperationId: parsedInput.recoveryOperationId,
    family: parsedInput.familyRecoveryOperation.family,
    parentAccount: parsedInput.parentAccount,
    parentDevice: parsedInput.parentDevice,
    childProfile: parsedInput.childProfile,
    childDevice: parsedInput.childDevice,
    kind: parsedInput.setupRecoveryKind,
    state: recoveryState,
    sourcePairingState: parsedInput.sourcePairingState,
    openedAt: parsedInput.familyRecoveryOperation.openedAt,
    resolvedAt: recoveryState === SetupRecoveryState.Recovered ? parsedInput.familyRecoveryOperation.closedAt : null,
  });
}

function pairingProjection(
  pairingState: SetupPairingState,
  failureReason: SetupPairingFailureReason | null,
  accountState: SetupAccountReadinessState,
  recoveryState: SetupRecoveryState
): SetupPairingProjection {
  return SetupPairingProjectionSchema.parse({
    pairingState,
    failureReason,
    accountState,
    recoveryState,
  });
}

function evaluateRequiredPairingStepUp(input: SetupFamilyReadinessInput):
  | { kind: 'not-required' }
  | { kind: 'satisfied' }
  | { kind: 'pending' }
  | {
      kind: 'rejected';
      pairingState: SetupPairingState;
      failureReason: SetupPairingFailureReason;
    } {
  if (
    input.setupInvite.state !== SetupInviteState.Accepted ||
    !requiresParentStepUp(input.householdAuthorityInput.action)
  ) {
    return { kind: 'not-required' };
  }

  if (input.parentStepUpAssertion !== null) {
    return stepUpStatusFromValidation(
      validateParentStepUpAssertion({
        assertion: input.parentStepUpAssertion,
        family: input.family,
        parentAccount: input.parentAccount,
        actionDevice: input.parentDevice,
        targetChildProfile: input.childProfile,
        action: input.householdAuthorityInput.action,
        observedAt: input.observedAt,
      })
    );
  }

  if (input.pairingApprovalResponse !== null && input.pairingApprovalChallenge === null) {
    return rejectedStepUpStatus(SetupPairingState.Replayed, SetupPairingFailureReason.ReplayRejected);
  }

  if (input.pairingApprovalChallenge === null || input.pairingApprovalResponse === null) {
    return { kind: 'pending' };
  }

  const approvalResolution = deriveParentStepUpAssertionFromSetupPairingApproval({
    challenge: input.pairingApprovalChallenge,
    response: input.pairingApprovalResponse,
    observedAt: input.observedAt,
  });

  if (approvalResolution.failureReason !== null) {
    return rejectedStepUpStatus(
      pairingStateForStepUpFailure(approvalResolution.failureReason),
      approvalResolution.failureReason
    );
  }

  return stepUpStatusFromValidation(
    validateParentStepUpAssertion({
      assertion: approvalResolution.assertion,
      family: input.family,
      parentAccount: input.parentAccount,
      actionDevice: input.parentDevice,
      targetChildProfile: input.childProfile,
      action: input.householdAuthorityInput.action,
      observedAt: input.observedAt,
      expectedNonce: input.pairingApprovalChallenge.challengeNonce,
    })
  );
}

function stepUpStatusFromValidation(validation: ReturnType<typeof validateParentStepUpAssertion>):
  | { kind: 'satisfied' }
  | { kind: 'pending' }
  | {
      kind: 'rejected';
      pairingState: SetupPairingState;
      failureReason: SetupPairingFailureReason;
    } {
  if (validation.valid) {
    return { kind: 'satisfied' };
  }

  switch (validation.failureReason) {
    case ParentStepUpValidationFailureReason.Required:
      return { kind: 'pending' };
    case ParentStepUpValidationFailureReason.Expired:
      return rejectedStepUpStatus(SetupPairingState.Expired, SetupPairingFailureReason.ApprovalExpired);
    case ParentStepUpValidationFailureReason.WrongHousehold:
      return rejectedStepUpStatus(SetupPairingState.WrongHousehold, SetupPairingFailureReason.WrongHousehold);
    case ParentStepUpValidationFailureReason.WrongAccount:
      return rejectedStepUpStatus(SetupPairingState.Untrusted, SetupPairingFailureReason.WrongAccount);
    case ParentStepUpValidationFailureReason.WrongDevice:
      return rejectedStepUpStatus(SetupPairingState.WrongDevice, SetupPairingFailureReason.WrongDevice);
    case ParentStepUpValidationFailureReason.WrongAction:
    case ParentStepUpValidationFailureReason.WrongTarget:
      return rejectedStepUpStatus(SetupPairingState.WrongTarget, SetupPairingFailureReason.WrongTarget);
    case ParentStepUpValidationFailureReason.ReplayRejected:
      return rejectedStepUpStatus(SetupPairingState.Replayed, SetupPairingFailureReason.ReplayRejected);
    default:
      return rejectedStepUpStatus(SetupPairingState.Untrusted, SetupPairingFailureReason.WrongTarget);
  }
}

function rejectedStepUpStatus(
  pairingState: SetupPairingState,
  failureReason: SetupPairingFailureReason
): {
  kind: 'rejected';
  pairingState: SetupPairingState;
  failureReason: SetupPairingFailureReason;
} {
  return {
    kind: 'rejected',
    pairingState,
    failureReason,
  };
}

function pairingStateForStepUpFailure(
  failureReason: SetupPairingFailureReason
): SetupPairingState {
  switch (failureReason) {
    case SetupPairingFailureReason.WrongHousehold:
      return SetupPairingState.WrongHousehold;
    case SetupPairingFailureReason.WrongAccount:
      return SetupPairingState.Untrusted;
    case SetupPairingFailureReason.WrongDevice:
      return SetupPairingState.WrongDevice;
    case SetupPairingFailureReason.WrongTarget:
      return SetupPairingState.WrongTarget;
    case SetupPairingFailureReason.ApprovalExpired:
      return SetupPairingState.Expired;
    case SetupPairingFailureReason.ReplayRejected:
      return SetupPairingState.Replayed;
    default:
      return SetupPairingState.Untrusted;
  }
}

function accountStateForPairing(
  input: SetupFamilyReadinessInput,
  authorityDecision: ReturnType<typeof authorizeHouseholdAction>
): SetupAccountReadinessState {
  if (targetAccountMismatch(input.setupInvite, input.parentAccount)) {
    return SetupAccountReadinessState.WrongAccount;
  }

  if (authorityDecision.authorizationState === HouseholdAuthorizationState.Rejected) {
    return accountStateForRejectedAuthority(input.householdAuthorityInput, authorityDecision.failureReason);
  }

  return setupRecoveryStateFromFamilyOperation(input.recoveryOperation) === SetupRecoveryState.Normal
    ? SetupAccountReadinessState.Ready
    : SetupAccountReadinessState.RecoveryRequired;
}

function accountStateForRejectedAuthority(
  authorityInput: HouseholdAuthorityInput,
  failureReason: HouseholdAuthorizationFailureReason | null
): SetupAccountReadinessState {
  HouseholdAuthorityInputSchema.parse(authorityInput);

  switch (failureReason) {
    case HouseholdAuthorizationFailureReason.AccountNotActive:
      return SetupAccountReadinessState.WrongAccount;
    case HouseholdAuthorizationFailureReason.ExternalHousehold:
    case HouseholdAuthorizationFailureReason.MembershipNotActive:
      return SetupAccountReadinessState.RecoveryRequired;
    default:
      return SetupAccountReadinessState.Ready;
  }
}

function pairingStateForRejectedAuthority(
  failureReason: HouseholdAuthorizationFailureReason | null
): SetupPairingState {
  switch (failureReason) {
    case HouseholdAuthorizationFailureReason.ExternalHousehold:
      return SetupPairingState.WrongHousehold;
    case HouseholdAuthorizationFailureReason.DeviceNotTrusted:
      return SetupPairingState.Revoked;
    case HouseholdAuthorizationFailureReason.WrongDeviceScope:
      return SetupPairingState.WrongDevice;
    case HouseholdAuthorizationFailureReason.ChildProfileNotBound:
      return SetupPairingState.AnonymousDevice;
    case HouseholdAuthorizationFailureReason.RoleNotAuthorized:
      return SetupPairingState.ParentRoleRequired;
    default:
      return SetupPairingState.Untrusted;
  }
}

function pairingFailureReasonForRejectedAuthority(
  failureReason: HouseholdAuthorizationFailureReason | null
): SetupPairingFailureReason | null {
  switch (failureReason) {
    case HouseholdAuthorizationFailureReason.ExternalHousehold:
      return SetupPairingFailureReason.WrongHousehold;
    case HouseholdAuthorizationFailureReason.AccountNotActive:
      return SetupPairingFailureReason.WrongAccount;
    case HouseholdAuthorizationFailureReason.DeviceNotTrusted:
      return SetupPairingFailureReason.RevokedDevice;
    case HouseholdAuthorizationFailureReason.WrongDeviceScope:
      return SetupPairingFailureReason.WrongDevice;
    case HouseholdAuthorizationFailureReason.ChildProfileNotBound:
      return SetupPairingFailureReason.AnonymousDevice;
    case HouseholdAuthorizationFailureReason.RoleNotAuthorized:
      return SetupPairingFailureReason.ParentRoleRequired;
    case HouseholdAuthorizationFailureReason.MissingCapabilityGrant:
      return SetupPairingFailureReason.PermissionLoss;
    default:
      return null;
  }
}

function setupRecoveryStateFromFamilyOperation(
  recoveryOperation: FamilyRecoveryOperation | null
): SetupRecoveryState {
  if (recoveryOperation === null) {
    return SetupRecoveryState.Normal;
  }

  if (recoveryOperation.state === FamilyRecoveryState.Revoked) {
    return SetupRecoveryState.Required;
  }

  if (recoveryOperation.bundleFailureReason !== null) {
    return SetupRecoveryState.Required;
  }

  const bundleRecoveryState = recoveryStateFromBundleState(recoveryOperation);

  if (bundleRecoveryState !== null) {
    return bundleRecoveryState;
  }

  return recoveryStateFromFamilyState(recoveryOperation.state);
}

function rejectedAuthorityRequiresRecoveryProjection(context: SetupPairingContext): boolean {
  return (
    context.authorityDecision.authorizationState === HouseholdAuthorizationState.Rejected &&
    !(context.inviteAccepted && context.parsedInput.householdAuthorityInput.deviceTrustState === DeviceTrustState.Pending)
  );
}

function inviteAcceptedWithPendingSetupDependencies(context: SetupPairingContext): boolean {
  return (
    context.inviteAccepted &&
    (context.childInstallState !== SetupChildInstallState.Installed ||
      context.childServiceState === SetupChildServiceState.NotStarted ||
      context.parsedInput.permissionState !== SetupPermissionReadinessState.Granted)
  );
}

function pairingChildIsOffline(context: SetupPairingContext): boolean {
  return (
    context.childServiceState === SetupChildServiceState.Offline ||
    context.parsedInput.networkReachabilityState === SetupNetworkReachabilityState.OfflineChild
  );
}

function accountStateForRejectedStepUp(
  accountState: SetupAccountReadinessState,
  failureReason: SetupPairingFailureReason
): SetupAccountReadinessState {
  return failureReason === SetupPairingFailureReason.WrongAccount
    ? SetupAccountReadinessState.WrongAccount
    : accountState;
}

function acceptedInviteWithPendingTrustedDevice(
  context: SetupPairingContext,
  requiredStepUpStatus: RequiredPairingStepUpStatus
): boolean {
  return (
    context.inviteAccepted &&
    context.parsedInput.householdAuthorityInput.deviceTrustState === DeviceTrustState.Pending &&
    requiredStepUpStatus.kind !== 'satisfied'
  );
}

function accountStateForPendingTrustedInvite(
  familyRecoveryState: SetupRecoveryState
): SetupAccountReadinessState {
  return familyRecoveryState === SetupRecoveryState.Normal || familyRecoveryState === SetupRecoveryState.Recovered
    ? SetupAccountReadinessState.Ready
    : SetupAccountReadinessState.RecoveryRequired;
}

function mergedRecoveryState(
  pairingRecoveryState: SetupRecoveryState,
  familyRecoveryState: SetupRecoveryState
): SetupRecoveryState {
  if (
    pairingRecoveryState === SetupRecoveryState.Recovered ||
    familyRecoveryState === SetupRecoveryState.Recovered
  ) {
    return SetupRecoveryState.Recovered;
  }

  if (
    pairingRecoveryState === SetupRecoveryState.InProgress ||
    familyRecoveryState === SetupRecoveryState.InProgress
  ) {
    return SetupRecoveryState.InProgress;
  }

  if (
    pairingRecoveryState === SetupRecoveryState.Required ||
    familyRecoveryState === SetupRecoveryState.Required
  ) {
    return SetupRecoveryState.Required;
  }

  return SetupRecoveryState.Normal;
}

function recoveryBlocksCustodyHandoff(
  recoveryOperation: FamilyRecoveryOperation | null,
  recoveryState: SetupRecoveryState
): boolean {
  return (
    recoveryOperation !== null &&
    (recoveryDataCustodyHandoffState(recoveryOperation) !== RecoveryDataCustodyHandoffState.None ||
      (recoveryRequiresAuditedSupport(recoveryOperation) && recoveryState !== SetupRecoveryState.Recovered))
  );
}

function custodySyncIsPending(
  input: SetupFamilyReadinessInput,
  childServiceState: ReturnType<typeof resolvedChildServiceState>
): boolean {
  return (
    input.custodySyncPending ||
    childServiceState === SetupChildServiceState.Offline ||
    input.networkReachabilityState === SetupNetworkReachabilityState.OfflineChild
  );
}

function dataCustodySyncStateFromRecovery(
  custodyBlockedByRecovery: boolean,
  recoveryState: SetupRecoveryState,
  custodySyncPending: boolean
): SetupDataCustodySyncState {
  if (custodyBlockedByRecovery) {
    return SetupDataCustodySyncState.Blocked;
  }

  if (recoveryState !== SetupRecoveryState.Normal && recoveryState !== SetupRecoveryState.Recovered) {
    return custodySyncPending ? SetupDataCustodySyncState.SyncPending : SetupDataCustodySyncState.Blocked;
  }

  return custodySyncPending ? SetupDataCustodySyncState.SyncPending : SetupDataCustodySyncState.Synced;
}

function recoveryProjectionAccountState(
  accountState: SetupAccountReadinessState,
  recoveryState: SetupRecoveryState
): SetupAccountReadinessState {
  if (accountState === SetupAccountReadinessState.WrongAccount) {
    return SetupAccountReadinessState.WrongAccount;
  }

  return recoveryState === SetupRecoveryState.Normal || recoveryState === SetupRecoveryState.Recovered
    ? accountState
    : SetupAccountReadinessState.RecoveryRequired;
}

function recoveryStateFromBundleState(
  recoveryOperation: FamilyRecoveryOperation
): SetupRecoveryState | null {
  switch (recoveryOperation.bundleState) {
    case RecoveryBundleState.PreviewOnly:
    case RecoveryBundleState.ApplyPending:
      return SetupRecoveryState.InProgress;
    case RecoveryBundleState.PartialRestore:
    case RecoveryBundleState.Rejected:
    case RecoveryBundleState.ManualRequired:
      return SetupRecoveryState.Required;
    case RecoveryBundleState.Applied:
      return appliedRecoveryStateFromBundle(recoveryOperation);
    default:
      return null;
  }
}

function appliedRecoveryStateFromBundle(recoveryOperation: FamilyRecoveryOperation): SetupRecoveryState {
  const pendingDeviceTrust = deviceTrustStateForRecoveryOperation(recoveryOperation) === DeviceTrustState.Pending;
  const noCustodyHandoff =
    recoveryDataCustodyHandoffState(recoveryOperation) === RecoveryDataCustodyHandoffState.None;

  return pendingDeviceTrust && noCustodyHandoff ? SetupRecoveryState.Recovered : SetupRecoveryState.InProgress;
}

function recoveryStateFromFamilyState(recoveryState: FamilyRecoveryState): SetupRecoveryState {
  switch (recoveryState) {
    case FamilyRecoveryState.PendingIdentityProof:
    case FamilyRecoveryState.OwnerApprovalRequired:
      return SetupRecoveryState.Required;
    case FamilyRecoveryState.Approved:
      return SetupRecoveryState.InProgress;
    case FamilyRecoveryState.Completed:
      return SetupRecoveryState.Recovered;
    default:
      return SetupRecoveryState.Required;
  }
}

function targetAccountMismatch(
  setupInvite: SetupInvite,
  parentAccount: Infer<typeof ParentAccountReferenceSchema>
): boolean {
  SetupInviteSchema.parse(setupInvite);
  ParentAccountReferenceSchema.parse(parentAccount);

  return (
    setupInvite.targetAccount !== null && setupInvite.targetAccount.parentAccountId !== parentAccount.parentAccountId
  );
}
