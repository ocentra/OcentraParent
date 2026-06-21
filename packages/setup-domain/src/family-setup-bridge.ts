import {
  type HouseholdAuthorityInput,
  DeviceTrustState,
  HouseholdAuthorityInputSchema,
  HouseholdAuthorizationFailureReasonSchema,
  HouseholdAuthorizationFailureReason,
  HouseholdAuthorizationState,
  ParentStepUpValidationFailureReason,
} from '@ocentra-parent/schema-domain/family-household-authority';
import {
  requiresParentStepUp,
  authorizeHouseholdAction,
  validateParentStepUpAssertion,
} from '@ocentra-parent/family-domain/household-authority';
import {
  ParentAccountReferenceSchema,
} from '@ocentra-parent/schema-domain/family-references';
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
  SetupAccountReadinessStateSchema,
  SetupChildInstallState,
  SetupChildServiceState,
  SetupDataCustodySyncState,
  SetupNetworkReachabilityState,
  SetupPermissionReadinessState,
  SetupReadinessChecklistItem,
  createSetupReadinessChecklist,
  deriveSetupChildInstallStateFromAppState,
  deriveSetupChildServiceStateFromAppState,
  SetupReadinessReport,
  SetupReadinessReportSchema,
  SetupRecoveryOperation,
  SetupRecoveryOperationSchema,
  SetupRecoveryState,
  SetupRecoveryStateSchema,
} from '@ocentra-parent/schema-domain/setup-readiness';
import {
  deriveParentStepUpAssertionFromSetupPairingApproval,
  SetupPairingFailureReason,
  SetupPairingFailureReasonSchema,
  SetupPairingState,
  SetupPairingStateSchema,
} from '@ocentra-parent/schema-domain/setup-pairing-intent';

function resolvedChildInstallState(input: SetupFamilyReadinessInput) {
  return input.childInstallState ?? deriveSetupChildInstallStateFromAppState(input.childAppState);
}

function resolvedChildServiceState(input: SetupFamilyReadinessInput) {
  return input.childServiceState ?? deriveSetupChildServiceStateFromAppState(input.childAppState);
}

export function deriveSetupPairingProjectionFromFamilyContext(
  input: SetupFamilyReadinessInput
): SetupPairingProjection {
  const parsedInput = SetupFamilyReadinessInputSchema.parse(input);
  const authorityDecision = authorizeHouseholdAction(parsedInput.householdAuthorityInput);
  const familyRecoveryState = setupRecoveryStateFromFamilyOperation(parsedInput.recoveryOperation);
  const childInstallState = resolvedChildInstallState(parsedInput);
  const childServiceState = resolvedChildServiceState(parsedInput);
  const inviteAccepted = parsedInput.setupInvite.state === SetupInviteState.Accepted;

  if (parsedInput.replayDetected) {
    return pairingProjection(
      SetupPairingState.Replayed,
      SetupPairingFailureReason.ReplayRejected,
      accountStateForPairing(parsedInput, authorityDecision),
      SetupRecoveryState.Required
    );
  }

  if (parsedInput.staleCode) {
    return pairingProjection(
      SetupPairingState.StaleSignedHello,
      SetupPairingFailureReason.StaleSignedHello,
      accountStateForPairing(parsedInput, authorityDecision),
      SetupRecoveryState.Required
    );
  }

  if (parsedInput.setupInvite.state === SetupInviteState.Expired) {
    return pairingProjection(
      SetupPairingState.Expired,
      SetupPairingFailureReason.StaleCode,
      accountStateForPairing(parsedInput, authorityDecision),
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

  if (parsedInput.childDeviceRevoked || parsedInput.setupInvite.state === SetupInviteState.Revoked) {
    return pairingProjection(
      SetupPairingState.Revoked,
      SetupPairingFailureReason.RevokedDevice,
      accountStateForPairing(parsedInput, authorityDecision),
      SetupRecoveryState.Required
    );
  }

  if (childServiceState === SetupChildServiceState.Revoked) {
    return pairingProjection(
      SetupPairingState.Revoked,
      SetupPairingFailureReason.RevokedDevice,
      accountStateForPairing(parsedInput, authorityDecision),
      SetupRecoveryState.Required
    );
  }

  if (
    authorityDecision.authorizationState === HouseholdAuthorizationState.Rejected &&
    !(
      parsedInput.setupInvite.state === SetupInviteState.Accepted &&
      parsedInput.householdAuthorityInput.deviceTrustState === DeviceTrustState.Pending
    )
  ) {
    return pairingProjection(
      pairingStateForRejectedAuthority(authorityDecision.failureReason),
      pairingFailureReasonForRejectedAuthority(authorityDecision.failureReason),
      accountStateForRejectedAuthority(parsedInput.householdAuthorityInput, authorityDecision.failureReason),
      SetupRecoveryState.Required
    );
  }

  if (
    inviteAccepted &&
    (childInstallState !== SetupChildInstallState.Installed ||
      childServiceState === SetupChildServiceState.NotStarted ||
      parsedInput.permissionState !== SetupPermissionReadinessState.Granted)
  ) {
    return pairingProjection(
      SetupPairingState.Accepted,
      null,
      accountStateForPairing(parsedInput, authorityDecision),
      familyRecoveryState
    );
  }

  if (parsedInput.permissionState !== SetupPermissionReadinessState.Granted) {
    return pairingProjection(
      SetupPairingState.Untrusted,
      SetupPairingFailureReason.PermissionLoss,
      accountStateForPairing(parsedInput, authorityDecision),
      SetupRecoveryState.Required
    );
  }

  if (
    childServiceState === SetupChildServiceState.Offline ||
    parsedInput.networkReachabilityState === SetupNetworkReachabilityState.OfflineChild
  ) {
    return pairingProjection(
      inviteAccepted ? SetupPairingState.Accepted : SetupPairingState.Untrusted,
      SetupPairingFailureReason.OfflineChild,
      accountStateForPairing(parsedInput, authorityDecision),
      inviteAccepted ? familyRecoveryState : SetupRecoveryState.Required
    );
  }

  const requiredStepUpStatus = evaluateRequiredPairingStepUp(parsedInput);

  if (requiredStepUpStatus.kind === 'pending') {
    return pairingProjection(
      SetupPairingState.Accepted,
      null,
      accountStateForPairing(parsedInput, authorityDecision),
      familyRecoveryState
    );
  }

  if (requiredStepUpStatus.kind === 'rejected') {
    return pairingProjection(
      requiredStepUpStatus.pairingState,
      requiredStepUpStatus.failureReason,
      requiredStepUpStatus.failureReason === SetupPairingFailureReason.WrongAccount
        ? SetupAccountReadinessState.WrongAccount
        : accountStateForPairing(parsedInput, authorityDecision),
      SetupRecoveryState.Required
    );
  }

  if (
    parsedInput.setupInvite.state === SetupInviteState.Accepted &&
    parsedInput.householdAuthorityInput.deviceTrustState === DeviceTrustState.Pending &&
    requiredStepUpStatus.kind !== 'satisfied'
  ) {
    return pairingProjection(
      SetupPairingState.Accepted,
      null,
      familyRecoveryState === SetupRecoveryState.Normal || familyRecoveryState === SetupRecoveryState.Recovered
        ? SetupAccountReadinessState.Ready
        : SetupAccountReadinessState.RecoveryRequired,
      familyRecoveryState
    );
  }

  if (
    familyRecoveryState === SetupRecoveryState.Recovered &&
    parsedInput.setupInvite.state === SetupInviteState.Accepted
  ) {
    return pairingProjection(
      SetupPairingState.Recovered,
      null,
      SetupAccountReadinessState.Ready,
      SetupRecoveryState.Recovered
    );
  }

  if (familyRecoveryState !== SetupRecoveryState.Normal) {
    return pairingProjection(
      parsedInput.setupInvite.state === SetupInviteState.Accepted
        ? SetupPairingState.Accepted
        : SetupPairingState.Displayed,
      null,
      SetupAccountReadinessState.RecoveryRequired,
      familyRecoveryState
    );
  }

  if (parsedInput.setupInvite.state === SetupInviteState.Accepted) {
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

export function deriveSetupRecoveryProjectionFromFamilyContext(
  input: SetupFamilyReadinessInput,
  pairing: SetupPairingProjection
): SetupRecoveryProjection {
  const parsedInput = SetupFamilyReadinessInputSchema.parse(input);
  const parsedPairing = SetupPairingProjectionSchema.parse(pairing);
  const childServiceState = resolvedChildServiceState(parsedInput);

  const familyRecoveryState = setupRecoveryStateFromFamilyOperation(parsedInput.recoveryOperation);
  const recoveryState =
    parsedPairing.recoveryState === SetupRecoveryState.Recovered || familyRecoveryState === SetupRecoveryState.Recovered
      ? SetupRecoveryState.Recovered
      : parsedPairing.recoveryState === SetupRecoveryState.InProgress ||
          familyRecoveryState === SetupRecoveryState.InProgress
        ? SetupRecoveryState.InProgress
        : parsedPairing.recoveryState === SetupRecoveryState.Required ||
            familyRecoveryState === SetupRecoveryState.Required
          ? SetupRecoveryState.Required
          : SetupRecoveryState.Normal;

  const custodyBlockedByRecovery =
    parsedInput.recoveryOperation !== null &&
    (recoveryDataCustodyHandoffState(parsedInput.recoveryOperation) !== RecoveryDataCustodyHandoffState.None ||
      (recoveryRequiresAuditedSupport(parsedInput.recoveryOperation) &&
        recoveryState !== SetupRecoveryState.Recovered));

  const custodySyncPending =
    parsedInput.custodySyncPending ||
    childServiceState === SetupChildServiceState.Offline ||
    parsedInput.networkReachabilityState === SetupNetworkReachabilityState.OfflineChild;

  const dataCustodySyncState = custodyBlockedByRecovery
    ? SetupDataCustodySyncState.Blocked
    : recoveryState !== SetupRecoveryState.Normal && recoveryState !== SetupRecoveryState.Recovered
      ? custodySyncPending
        ? SetupDataCustodySyncState.SyncPending
        : SetupDataCustodySyncState.Blocked
      : custodySyncPending
        ? SetupDataCustodySyncState.SyncPending
        : SetupDataCustodySyncState.Synced;

  return SetupRecoveryProjectionSchema.parse({
    accountState:
      parsedPairing.accountState === SetupAccountReadinessState.WrongAccount
        ? SetupAccountReadinessState.WrongAccount
        : recoveryState === SetupRecoveryState.Normal || recoveryState === SetupRecoveryState.Recovered
          ? parsedPairing.accountState
          : SetupAccountReadinessState.RecoveryRequired,
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
  pairingState: Infer<typeof SetupPairingStateSchema>,
  failureReason: Infer<typeof SetupPairingFailureReasonSchema> | null,
  accountState: Infer<typeof SetupAccountReadinessStateSchema>,
  recoveryState: Infer<typeof SetupRecoveryStateSchema>
): SetupPairingProjection {
  return SetupPairingProjectionSchema.parse({
    pairingState,
    failureReason,
    accountState,
    recoveryState,
  });
}

function evaluateRequiredPairingStepUp(
  input: SetupFamilyReadinessInput
):
  | { kind: 'not-required' }
  | { kind: 'satisfied' }
  | { kind: 'pending' }
  | {
      kind: 'rejected';
      pairingState: Infer<typeof SetupPairingStateSchema>;
      failureReason: Infer<typeof SetupPairingFailureReasonSchema>;
    } {
  if (input.setupInvite.state !== SetupInviteState.Accepted || !requiresParentStepUp(input.householdAuthorityInput.action)) {
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
      pairingState: Infer<typeof SetupPairingStateSchema>;
      failureReason: Infer<typeof SetupPairingFailureReasonSchema>;
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
  pairingState: Infer<typeof SetupPairingStateSchema>,
  failureReason: Infer<typeof SetupPairingFailureReasonSchema>
): {
  kind: 'rejected';
  pairingState: Infer<typeof SetupPairingStateSchema>;
  failureReason: Infer<typeof SetupPairingFailureReasonSchema>;
} {
  return {
    kind: 'rejected',
    pairingState,
    failureReason,
  };
}

function pairingStateForStepUpFailure(
  failureReason: Infer<typeof SetupPairingFailureReasonSchema>
): Infer<typeof SetupPairingStateSchema> {
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
): Infer<typeof SetupAccountReadinessStateSchema> {
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
  failureReason: Infer<typeof HouseholdAuthorizationFailureReasonSchema> | null
): Infer<typeof SetupAccountReadinessStateSchema> {
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
  failureReason: Infer<typeof HouseholdAuthorizationFailureReasonSchema> | null
): Infer<typeof SetupPairingStateSchema> {
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
  failureReason: Infer<typeof HouseholdAuthorizationFailureReasonSchema> | null
): Infer<typeof SetupPairingFailureReasonSchema> | null {
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
): Infer<typeof SetupRecoveryStateSchema> {
  if (recoveryOperation === null) {
    return SetupRecoveryState.Normal;
  }

  if (recoveryOperation.state === FamilyRecoveryState.Revoked) {
    return SetupRecoveryState.Required;
  }

  if (recoveryOperation.bundleFailureReason !== null) {
    return SetupRecoveryState.Required;
  }

  switch (recoveryOperation.bundleState) {
    case RecoveryBundleState.PreviewOnly:
    case RecoveryBundleState.ApplyPending:
      return SetupRecoveryState.InProgress;
    case RecoveryBundleState.PartialRestore:
    case RecoveryBundleState.Rejected:
    case RecoveryBundleState.ManualRequired:
      return SetupRecoveryState.Required;
    case RecoveryBundleState.Applied:
      return deviceTrustStateForRecoveryOperation(recoveryOperation) === DeviceTrustState.Pending &&
        recoveryDataCustodyHandoffState(recoveryOperation) === RecoveryDataCustodyHandoffState.None
        ? SetupRecoveryState.Recovered
        : SetupRecoveryState.InProgress;
    default:
      break;
  }

  switch (recoveryOperation.state) {
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
