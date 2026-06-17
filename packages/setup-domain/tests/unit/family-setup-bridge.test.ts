import { describe, expect, it } from 'vitest';
import {
  ActorAccountState,
  ChildProfileBindingState,
  DeviceAuthorityAction,
  DeviceOwnershipScope,
  DeviceTrustState,
  HouseholdAuthorityInputSchema,
  HouseholdMembershipState,
  HouseholdRole,
  ParentStepUpAssertionSchema,
  ParentStepUpMethod,
  SessionFreshnessState,
} from '@ocentra-parent/family-domain/household-authority';
import { ParentActorRole, ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
import {
  RecoveryIdentityProofState,
  RecoveryKind,
  RecoveryOperationSchema,
  RecoverySupportChannel,
  SetupInvitePurpose,
  SetupInviteSchema,
  SetupInviteState,
} from '@ocentra-parent/family-domain/setup-lifecycle';
import {
  createSetupReadinessReportFromFamilyContext,
  createSetupRecoveryOperationFromFamilyRecovery,
  deriveSetupPairingProjectionFromFamilyContext,
  SetupFamilyRecoveryOperationInputSchema,
  SetupFamilyReadinessInputSchema,
} from '../../src/family-setup-bridge';
import {
  deriveSetupReadinessOverallState,
  SetupAccountReadinessState,
  deriveSetupChildInstallJourneyStage,
  SetupChildInstallJourneyStage,
  SetupChildInstallState,
  SetupChildServiceState,
  SetupDataCustodySyncState,
  SetupReadinessOverallState,
  SetupRecoveryKind,
  SetupRecoveryState,
} from '../../src/readiness';
import {
  SetupPairingApprovalChallengeSchema,
  SetupPairingApprovalResponseSchema,
  SetupPairingFailureReason,
  SetupPairingState,
} from '../../src/pairing-intent';

const BaseInvite = SetupInviteSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  inviteId: 'invite-child-pairing-1',
  family: { familyId: 'family-main' },
  invitedBy: { actorId: 'actor-owner', role: ParentActorRole.Parent },
  targetAccount: { parentAccountId: 'parent-account-1' },
  targetChildProfile: { childProfileId: 'child-1', displayName: 'Sam' },
  targetRole: HouseholdRole.ChildDeviceAgent,
  purpose: SetupInvitePurpose.ChildDevicePairing,
  state: SetupInviteState.Pending,
  expiresAt: '2026-06-14T15:00:00.000Z',
  singleUse: true,
});

const BaseAuthorityInput = HouseholdAuthorityInputSchema.parse({
  actorRole: HouseholdRole.ParentOwner,
  actorAccountState: ActorAccountState.Active,
  sameFamily: true,
  membershipState: HouseholdMembershipState.Active,
  childProfileBindingState: ChildProfileBindingState.Bound,
  deviceOwnershipScope: DeviceOwnershipScope.ChildProfileDevice,
  deviceTrustState: DeviceTrustState.Trusted,
  sessionFreshnessState: SessionFreshnessState.Fresh,
  capabilityGranted: true,
  action: DeviceAuthorityAction.PairChildDevice,
});

const BaseInput = SetupFamilyReadinessInputSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readinessReportId: 'setup-readiness-family-bridge-1',
  family: { familyId: 'family-main' },
  parentAccount: { parentAccountId: 'parent-account-1' },
  parentDevice: {
    deviceId: 'device-parent-1',
    childProfileId: null,
    label: 'Parent Phone',
    platform: 'android',
  },
  childProfile: { childProfileId: 'child-1', displayName: 'Sam' },
  pairingIntentId: 'pairing-intent-1',
  setupInvite: BaseInvite,
  householdAuthorityInput: BaseAuthorityInput,
  recoveryOperation: null,
  parentAppState: 'ready',
  childAppState: 'ready',
  childInstallState: SetupChildInstallState.Installed,
  childServiceState: SetupChildServiceState.ServiceStarted,
  permissionState: 'granted',
  policyBaselineState: 'applied',
  networkReachabilityState: 'reachable',
  custodySyncPending: false,
  replayDetected: false,
  staleCode: false,
  childDeviceRevoked: false,
  observedAt: '2026-06-13T20:00:00.000Z',
});
const LocalStepUpAssertion = ParentStepUpAssertionSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  stepUpAssertionId: 'step-up-assertion-1',
  family: BaseInput.family,
  parentAccount: BaseInput.parentAccount,
  actionDevice: BaseInput.parentDevice,
  approverDevice: BaseInput.parentDevice,
  targetChildProfile: BaseInput.childProfile,
  action: DeviceAuthorityAction.PairChildDevice,
  method: ParentStepUpMethod.Passkey,
  nonce: 'step-up-nonce-1',
  issuedAt: '2026-06-13T19:58:00.000Z',
  expiresAt: '2026-06-13T20:05:00.000Z',
});
const DesktopParentDevice = {
  deviceId: 'device-parent-desktop-1',
  childProfileId: null,
  label: 'Parent Desktop',
  platform: 'windows',
} as const;
const QrApprovalChallenge = SetupPairingApprovalChallengeSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  approvalChallengeId: 'pairing-approval-challenge-1',
  pairingIntentId: BaseInput.pairingIntentId,
  family: BaseInput.family,
  parentAccount: BaseInput.parentAccount,
  actionDevice: DesktopParentDevice,
  desktopSessionId: 'desktop-session-1',
  childProfile: BaseInput.childProfile,
  action: DeviceAuthorityAction.PairChildDevice,
  challengeNonce: 'pairing-approval-nonce-1',
  createdAt: '2026-06-13T19:58:00.000Z',
  expiresAt: '2026-06-13T20:05:00.000Z',
});
const QrApprovalResponse = SetupPairingApprovalResponseSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  approvalResponseId: 'pairing-approval-response-1',
  approvalChallengeId: QrApprovalChallenge.approvalChallengeId,
  pairingIntentId: QrApprovalChallenge.pairingIntentId,
  family: QrApprovalChallenge.family,
  parentAccount: QrApprovalChallenge.parentAccount,
  actionDevice: QrApprovalChallenge.actionDevice,
  desktopSessionId: QrApprovalChallenge.desktopSessionId,
  approvalDevice: BaseInput.parentDevice,
  childProfile: QrApprovalChallenge.childProfile,
  action: QrApprovalChallenge.action,
  challengeNonce: QrApprovalChallenge.challengeNonce,
  approvalMethod: ParentStepUpMethod.PhoneQrApproval,
  approvedAt: '2026-06-13T20:01:00.000Z',
});

describe('setup family bridge', () => {
  it('produces a trusted ready report from an accepted child-device invite', () => {
    const report = createSetupReadinessReportFromFamilyContext({
      ...BaseInput,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
      parentStepUpAssertion: LocalStepUpAssertion,
    });

    expect(report.pairingState).toBe(SetupPairingState.Trusted);
    expect(report.accountState).toBe(SetupAccountReadinessState.Ready);
    expect(report.recoveryState).toBe(SetupRecoveryState.Normal);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Ready);
    expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Paired);
    expect(report.checklist.find((entry) => entry.checklistItemId === 'setup-pairing-state')?.state).toBe('complete');
  });

  it('keeps an accepted child invite blocked until fresh step-up is supplied even when the parent device is trusted', () => {
    const report = createSetupReadinessReportFromFamilyContext({
      ...BaseInput,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
    });

    expect(report.pairingState).toBe(SetupPairingState.Accepted);
    expect(report.accountState).toBe(SetupAccountReadinessState.Ready);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
  });

  it('maps replay detection into blocked replay-rejected pairing state', () => {
    const input = {
      ...BaseInput,
      replayDetected: true,
    };

    const projection = deriveSetupPairingProjectionFromFamilyContext(input);
    const report = createSetupReadinessReportFromFamilyContext(input);

    expect(projection.pairingState).toBe(SetupPairingState.Replayed);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.ReplayRejected);
    expect(report.recoveryState).toBe(SetupRecoveryState.Required);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
  });

  it('maps stale signed hello into an explicit pairing rejection state', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      staleCode: true,
    });

    expect(projection.pairingState).toBe(SetupPairingState.StaleSignedHello);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.StaleSignedHello);
    expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
  });

  it('keeps an accepted child invite pending until the parent confirms device trust', () => {
    const input = SetupFamilyReadinessInputSchema.parse({
      ...BaseInput,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
      householdAuthorityInput: HouseholdAuthorityInputSchema.parse({
        ...BaseAuthorityInput,
        deviceTrustState: DeviceTrustState.Pending,
      }),
    });

    const projection = deriveSetupPairingProjectionFromFamilyContext(input);
    const report = createSetupReadinessReportFromFamilyContext(input);

    expect(projection.pairingState).toBe(SetupPairingState.Accepted);
    expect(projection.failureReason).toBeNull();
    expect(projection.recoveryState).toBe(SetupRecoveryState.Normal);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
    expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Paired);
    expect(report.checklist.find((entry) => entry.checklistItemId === 'setup-pairing-state')?.supportCode).toBe(
      'accepted'
    );
  });

  it('accepts a phone QR approval bridge as fresh step-up for a desktop pairing action', () => {
    const report = createSetupReadinessReportFromFamilyContext({
      ...BaseInput,
      parentDevice: DesktopParentDevice,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
      pairingApprovalChallenge: QrApprovalChallenge,
      pairingApprovalResponse: QrApprovalResponse,
    });

    expect(report.pairingState).toBe(SetupPairingState.Trusted);
    expect(report.accountState).toBe(SetupAccountReadinessState.Ready);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Ready);
  });

  it('rejects expired phone QR approvals instead of silently trusting the pairing action', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      parentDevice: DesktopParentDevice,
      observedAt: '2026-06-13T20:06:00.000Z',
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
      pairingApprovalChallenge: QrApprovalChallenge,
      pairingApprovalResponse: QrApprovalResponse,
    });

    expect(projection.pairingState).toBe(SetupPairingState.Expired);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.ApprovalExpired);
  });

  it('marks wrong-account pairing when the invite targets a different parent account', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext(
      SetupFamilyReadinessInputSchema.parse({
        ...BaseInput,
        setupInvite: SetupInviteSchema.parse({
          ...BaseInvite,
          state: SetupInviteState.Accepted,
        }),
        parentAccount: { parentAccountId: 'parent-account-9' },
      })
    );

    expect(projection.pairingState).toBe(SetupPairingState.Untrusted);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.WrongAccount);
    expect(projection.accountState).toBe(SetupAccountReadinessState.WrongAccount);
  });

  it('maps cross-family authority rejection into wrong-household setup state', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      householdAuthorityInput: HouseholdAuthorityInputSchema.parse({
        ...BaseAuthorityInput,
        sameFamily: false,
      }),
    });

    expect(projection.pairingState).toBe(SetupPairingState.WrongHousehold);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.WrongHousehold);
    expect(projection.accountState).toBe(SetupAccountReadinessState.RecoveryRequired);
  });

  it('maps wrong-device-scope authority rejection into explicit pairing state', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      householdAuthorityInput: HouseholdAuthorityInputSchema.parse({
        ...BaseAuthorityInput,
        deviceOwnershipScope: DeviceOwnershipScope.OtherDevice,
      }),
    });

    expect(projection.pairingState).toBe(SetupPairingState.WrongDevice);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.WrongDevice);
    expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
  });

  it('maps unbound child-profile authority rejection into explicit pairing state', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      householdAuthorityInput: HouseholdAuthorityInputSchema.parse({
        ...BaseAuthorityInput,
        childProfileBindingState: ChildProfileBindingState.Missing,
      }),
    });

    expect(projection.pairingState).toBe(SetupPairingState.AnonymousDevice);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.AnonymousDevice);
    expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
  });

  it('maps role rejection into explicit parent-role-required pairing state', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      householdAuthorityInput: HouseholdAuthorityInputSchema.parse({
        ...BaseAuthorityInput,
        actorRole: HouseholdRole.Observer,
      }),
    });

    expect(projection.pairingState).toBe(SetupPairingState.ParentRoleRequired);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.ParentRoleRequired);
    expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
  });

  it('treats offline child reachability as explicit pairing recovery work', () => {
    const projection = deriveSetupPairingProjectionFromFamilyContext({
      ...BaseInput,
      childServiceState: SetupChildServiceState.Offline,
      childAppState: 'offline',
      networkReachabilityState: 'offline-child',
    });

    expect(projection.pairingState).toBe(SetupPairingState.Untrusted);
    expect(projection.failureReason).toBe(SetupPairingFailureReason.OfflineChild);
    expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
  });

  it('surfaces install progression even when the legacy child app state still says ready', () => {
    const report = createSetupReadinessReportFromFamilyContext({
      ...BaseInput,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
      childAppState: 'ready',
      childInstallState: SetupChildInstallState.Installed,
      childServiceState: SetupChildServiceState.NotStarted,
      permissionState: 'missing',
      policyBaselineState: 'missing',
    });

    expect(report.childInstallState).toBe(SetupChildInstallState.Installed);
    expect(report.childServiceState).toBe(SetupChildServiceState.NotStarted);
    expect(report.pairingState).toBe(SetupPairingState.Accepted);
    expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Installed);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
  });

  it('keeps support-assisted recovery blocked until custody handoff is cleared and emits a setup recovery operation', () => {
    const familyRecovery = RecoveryOperationSchema.parse({
      schemaVersion: ParentContractSchemaVersion.V0_6,
      recoveryOperationId: 'family-recovery-1',
      family: { familyId: 'family-main' },
      requestedBy: { actorId: 'actor-owner', role: ParentActorRole.Parent },
      requesterMembershipState: HouseholdMembershipState.Active,
      relatedAccount: { parentAccountId: 'parent-account-1' },
      relatedDevice: {
        deviceId: 'device-child-1',
        childProfileId: 'child-1',
        label: 'Sam Android',
        platform: 'android',
      },
      kind: RecoveryKind.CompromisedAccount,
      state: 'approved',
      ownerApprovalRequired: false,
      identityProofState: RecoveryIdentityProofState.Verified,
      supportChannel: RecoverySupportChannel.SupportAssisted,
      deleteExportHandoffRequired: true,
      openedAt: '2026-06-13T20:01:00.000Z',
      closedAt: null,
    });

    const report = createSetupReadinessReportFromFamilyContext({
      ...BaseInput,
      setupInvite: SetupInviteSchema.parse({
        ...BaseInvite,
        state: SetupInviteState.Accepted,
      }),
      recoveryOperation: familyRecovery,
    });

    const recoveryOperation = createSetupRecoveryOperationFromFamilyRecovery(
      SetupFamilyRecoveryOperationInputSchema.parse({
        recoveryOperationId: 'setup-recovery-1',
        setupRecoveryKind: SetupRecoveryKind.PermissionLoss,
        parentAccount: BaseInput.parentAccount,
        parentDevice: BaseInput.parentDevice,
        childProfile: BaseInput.childProfile,
        childDevice: {
          deviceId: 'device-child-1',
          childProfileId: 'child-1',
          label: 'Sam Android',
          platform: 'android',
        },
        sourcePairingState: SetupPairingState.Accepted,
        familyRecoveryOperation: familyRecovery,
      })
    );

    expect(report.recoveryState).toBe(SetupRecoveryState.InProgress);
    expect(report.dataCustodySyncState).toBe(SetupDataCustodySyncState.Blocked);
    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
    expect(recoveryOperation.kind).toBe(SetupRecoveryKind.PermissionLoss);
    expect(recoveryOperation.state).toBe(SetupRecoveryState.InProgress);
    expect(recoveryOperation.resolvedAt).toBeNull();
  });
});
