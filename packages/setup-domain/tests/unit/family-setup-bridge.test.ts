import { describe, expect, it } from 'vitest';
import {
  ActorAccountState,
  ChildProfileBindingState,
  DeviceAuthorityActionLiteral,
  DeviceOwnershipScope,
  DeviceTrustState,
  HouseholdAuthorityInputSchema,
  HouseholdMembershipState,
  HouseholdRole,
  ParentStepUpAssertionSchema,
  ParentStepUpMethod,
  SessionFreshnessState,
} from '@ocentra-parent/schema-domain/family-household-authority';
import {
  ParentActorRole,
  ParentContractSchemaVersion,
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RecoveryBundleFailureReason,
  RecoveryBundleHandoffTarget,
  RecoveryBundleState,
  RecoveryDeleteExportState,
  RecoveryIdentityProofState,
  RecoveryKind,
  RecoveryOperationSchema,
  RecoverySupportChannel,
} from '@ocentra-parent/schema-domain/family-restore-lifecycle';
import {
  SetupInvitePurpose,
  SetupInviteSchema,
  SetupInviteState,
} from '@ocentra-parent/schema-domain/family-setup-invite';
import {
  createSetupReadinessReportFromFamilyContext,
  createSetupRecoveryOperationFromFamilyRecovery,
  deriveSetupPairingProjectionFromFamilyContext,
} from '../../src/family-setup-bridge';
import {
  SetupFamilyRecoveryOperationInputSchema,
  SetupFamilyReadinessInputSchema,
} from '@ocentra-parent/schema-domain/family-setup-bridge';
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
} from '@ocentra-parent/schema-domain/setup-readiness';
import {
  SetupPairingApprovalChallengeSchema,
  SetupPairingApprovalResponseSchema,
  SetupPairingFailureReason,
  SetupPairingState,
} from '@ocentra-parent/schema-domain/setup-pairing-intent';

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
  action: DeviceAuthorityActionLiteral.PairChildDevice,
});

type ChecklistEntry = {
  readonly checklistItemId: string;
  readonly state?: string;
  readonly supportCode?: string;
};

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
  action: DeviceAuthorityActionLiteral.PairChildDevice,
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
  action: DeviceAuthorityActionLiteral.PairChildDevice,
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

const BaseFamilyRecoveryOperation = RecoveryOperationSchema.parse({
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
  bundleHandoffTarget: RecoveryBundleHandoffTarget.DeviceTrustRecoveryPersistence,
  bundleState: RecoveryBundleState.ApplyPending,
  bundleFailureReason: null,
  deleteExportState: RecoveryDeleteExportState.DeletePending,
  openedAt: '2026-06-13T20:01:00.000Z',
  closedAt: null,
});

function createSetupInvite(overrides: Record<string, unknown> = {}) {
  return SetupInviteSchema.parse({
    ...BaseInvite,
    ...overrides,
  });
}

function createHouseholdAuthorityInput(overrides: Record<string, unknown> = {}) {
  return HouseholdAuthorityInputSchema.parse({
    ...BaseAuthorityInput,
    ...overrides,
  });
}

function createSetupReadinessInput(overrides: Record<string, unknown> = {}) {
  return SetupFamilyReadinessInputSchema.parse({
    ...BaseInput,
    ...overrides,
  });
}

function createFamilyRecoveryOperation(overrides: Record<string, unknown> = {}) {
  return RecoveryOperationSchema.parse({
    ...BaseFamilyRecoveryOperation,
    ...overrides,
  });
}

function readinessChecklistEntry(report: { checklist: readonly ChecklistEntry[] }, checklistItemId: string) {
  return report.checklist.find((entry) => entry.checklistItemId === checklistItemId);
}

function expectTrustedReadyReportFromAcceptedInvite() {
  const report = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
      parentStepUpAssertion: LocalStepUpAssertion,
    })
  );

  expect(report.pairingState).toBe(SetupPairingState.Trusted);
  expect(report.accountState).toBe(SetupAccountReadinessState.Ready);
  expect(report.recoveryState).toBe(SetupRecoveryState.Normal);
  expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Ready);
  expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Paired);
  expect(readinessChecklistEntry(report, 'setup-pairing-state')?.state).toBe('complete');
}

function expectAcceptedInviteToStayBlockedUntilFreshStepUp() {
  const report = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
    })
  );

  expect(report.pairingState).toBe(SetupPairingState.Accepted);
  expect(report.accountState).toBe(SetupAccountReadinessState.Ready);
  expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
}

function expectReplayDetectionToMapToReplayRejectedPairing() {
  const input = createSetupReadinessInput({
    replayDetected: true,
  });
  const projection = deriveSetupPairingProjectionFromFamilyContext(input);
  const report = createSetupReadinessReportFromFamilyContext(input);

  expect(projection.pairingState).toBe(SetupPairingState.Replayed);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.ReplayRejected);
  expect(report.recoveryState).toBe(SetupRecoveryState.Required);
  expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
}

function expectStaleSignedHelloToMapToExplicitPairingRejection() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      staleCode: true,
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.StaleSignedHello);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.StaleSignedHello);
  expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
}

function expectPendingDeviceTrustToKeepAcceptedInviteBlocked() {
  const input = createSetupReadinessInput({
    setupInvite: createSetupInvite({
      state: SetupInviteState.Accepted,
    }),
    householdAuthorityInput: createHouseholdAuthorityInput({
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
  expect(readinessChecklistEntry(report, 'setup-pairing-state')?.supportCode).toBe('accepted');
}

function expectPhoneQrApprovalToActAsFreshStepUp() {
  const report = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      parentDevice: DesktopParentDevice,
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
      pairingApprovalChallenge: QrApprovalChallenge,
      pairingApprovalResponse: QrApprovalResponse,
    })
  );

  expect(report.pairingState).toBe(SetupPairingState.Trusted);
  expect(report.accountState).toBe(SetupAccountReadinessState.Ready);
  expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Ready);
}

function expectExpiredPhoneQrApprovalsToBeRejected() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      parentDevice: DesktopParentDevice,
      observedAt: '2026-06-13T20:06:00.000Z',
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
      pairingApprovalChallenge: QrApprovalChallenge,
      pairingApprovalResponse: QrApprovalResponse,
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.Expired);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.ApprovalExpired);
}

function expectDifferentTargetAccountToMapWrongAccount() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
      parentAccount: { parentAccountId: 'parent-account-9' },
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.Untrusted);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.WrongAccount);
  expect(projection.accountState).toBe(SetupAccountReadinessState.WrongAccount);
}

function expectCrossFamilyAuthorityToMapWrongHousehold() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      householdAuthorityInput: createHouseholdAuthorityInput({
        sameFamily: false,
      }),
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.WrongHousehold);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.WrongHousehold);
  expect(projection.accountState).toBe(SetupAccountReadinessState.RecoveryRequired);
}

function expectWrongDeviceScopeToMapExplicitPairingState() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      householdAuthorityInput: createHouseholdAuthorityInput({
        deviceOwnershipScope: DeviceOwnershipScope.OtherDevice,
      }),
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.WrongDevice);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.WrongDevice);
  expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
}

function expectMissingChildProfileBindingToMapAnonymousDevice() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      householdAuthorityInput: createHouseholdAuthorityInput({
        childProfileBindingState: ChildProfileBindingState.Missing,
      }),
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.AnonymousDevice);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.AnonymousDevice);
  expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
}

function expectRoleRejectionToMapParentRoleRequired() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      householdAuthorityInput: createHouseholdAuthorityInput({
        actorRole: HouseholdRole.Observer,
      }),
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.ParentRoleRequired);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.ParentRoleRequired);
  expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
}

function expectOfflineChildReachabilityToMapRecoveryWork() {
  const projection = deriveSetupPairingProjectionFromFamilyContext(
    createSetupReadinessInput({
      childServiceState: SetupChildServiceState.Offline,
      childAppState: 'offline',
      networkReachabilityState: 'offline-child',
    })
  );

  expect(projection.pairingState).toBe(SetupPairingState.Untrusted);
  expect(projection.failureReason).toBe(SetupPairingFailureReason.OfflineChild);
  expect(projection.recoveryState).toBe(SetupRecoveryState.Required);
}

function expectInstallProgressionToSurfaceAlongsideLegacyReadyState() {
  const report = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
      childAppState: 'ready',
      childInstallState: SetupChildInstallState.Installed,
      childServiceState: SetupChildServiceState.NotStarted,
      permissionState: 'missing',
      policyBaselineState: 'missing',
    })
  );

  expect(report.childInstallState).toBe(SetupChildInstallState.Installed);
  expect(report.childServiceState).toBe(SetupChildServiceState.NotStarted);
  expect(report.pairingState).toBe(SetupPairingState.Accepted);
  expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Installed);
  expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
}

function expectSupportAssistedRecoveryToStayBlockedUntilCustodyClears() {
  const familyRecovery = createFamilyRecoveryOperation();
  const report = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: createSetupInvite({
        state: SetupInviteState.Accepted,
      }),
      recoveryOperation: familyRecovery,
    })
  );
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
}

function expectRecoveryToRequireAppliedBundleAndDeleteSettlement() {
  const acceptedInvite = createSetupInvite({
    state: SetupInviteState.Accepted,
  });
  const completedRecovery = createFamilyRecoveryOperation({
    recoveryOperationId: 'family-recovery-completed',
    state: 'completed',
    supportChannel: RecoverySupportChannel.SelfServe,
    bundleState: RecoveryBundleState.Applied,
    deleteExportState: RecoveryDeleteExportState.DeletePending,
    closedAt: '2026-06-13T20:09:00.000Z',
  });
  const pendingDeleteReport = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: acceptedInvite,
      parentStepUpAssertion: LocalStepUpAssertion,
      recoveryOperation: completedRecovery,
    })
  );
  const settledRecovery = createFamilyRecoveryOperation({
    ...completedRecovery,
    recoveryOperationId: 'family-recovery-completed-settled',
    deleteExportState: RecoveryDeleteExportState.DeleteConfirmed,
  });
  const settledReport = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: acceptedInvite,
      parentStepUpAssertion: LocalStepUpAssertion,
      recoveryOperation: settledRecovery,
    })
  );
  const wrongKeyRecovery = createFamilyRecoveryOperation({
    ...completedRecovery,
    recoveryOperationId: 'family-recovery-wrong-key',
    deleteExportHandoffRequired: false,
    bundleState: RecoveryBundleState.Rejected,
    bundleFailureReason: RecoveryBundleFailureReason.WrongKey,
    deleteExportState: RecoveryDeleteExportState.None,
  });
  const wrongKeyReport = createSetupReadinessReportFromFamilyContext(
    createSetupReadinessInput({
      setupInvite: acceptedInvite,
      parentStepUpAssertion: LocalStepUpAssertion,
      recoveryOperation: wrongKeyRecovery,
    })
  );

  expect(pendingDeleteReport.pairingState).toBe(SetupPairingState.Accepted);
  expect(pendingDeleteReport.recoveryState).toBe(SetupRecoveryState.InProgress);
  expect(pendingDeleteReport.dataCustodySyncState).toBe(SetupDataCustodySyncState.Blocked);
  expect(deriveSetupReadinessOverallState(pendingDeleteReport)).toBe(SetupReadinessOverallState.Blocked);
  expect(settledReport.pairingState).toBe(SetupPairingState.Recovered);
  expect(settledReport.recoveryState).toBe(SetupRecoveryState.Recovered);
  expect(settledReport.dataCustodySyncState).toBe(SetupDataCustodySyncState.Synced);
  expect(deriveSetupReadinessOverallState(settledReport)).toBe(SetupReadinessOverallState.Ready);
  expect(wrongKeyReport.recoveryState).toBe(SetupRecoveryState.Required);
  expect(wrongKeyReport.dataCustodySyncState).toBe(SetupDataCustodySyncState.Blocked);
  expect(deriveSetupReadinessOverallState(wrongKeyReport)).toBe(SetupReadinessOverallState.Blocked);
}

const SetupFamilyBridgeCases = [
  ['produces a trusted ready report from an accepted child-device invite', expectTrustedReadyReportFromAcceptedInvite],
  [
    'keeps an accepted child invite blocked until fresh step-up is supplied even when the parent device is trusted',
    expectAcceptedInviteToStayBlockedUntilFreshStepUp,
  ],
  [
    'maps replay detection into blocked replay-rejected pairing state',
    expectReplayDetectionToMapToReplayRejectedPairing,
  ],
  [
    'maps stale signed hello into an explicit pairing rejection state',
    expectStaleSignedHelloToMapToExplicitPairingRejection,
  ],
  [
    'keeps an accepted child invite pending until the parent confirms device trust',
    expectPendingDeviceTrustToKeepAcceptedInviteBlocked,
  ],
  [
    'accepts a phone QR approval bridge as fresh step-up for a desktop pairing action',
    expectPhoneQrApprovalToActAsFreshStepUp,
  ],
  [
    'rejects expired phone QR approvals instead of silently trusting the pairing action',
    expectExpiredPhoneQrApprovalsToBeRejected,
  ],
  [
    'marks wrong-account pairing when the invite targets a different parent account',
    expectDifferentTargetAccountToMapWrongAccount,
  ],
  [
    'maps cross-family authority rejection into wrong-household setup state',
    expectCrossFamilyAuthorityToMapWrongHousehold,
  ],
  [
    'maps wrong-device-scope authority rejection into explicit pairing state',
    expectWrongDeviceScopeToMapExplicitPairingState,
  ],
  [
    'maps unbound child-profile authority rejection into explicit pairing state',
    expectMissingChildProfileBindingToMapAnonymousDevice,
  ],
  ['maps role rejection into explicit parent-role-required pairing state', expectRoleRejectionToMapParentRoleRequired],
  [
    'treats offline child reachability as explicit pairing recovery work',
    expectOfflineChildReachabilityToMapRecoveryWork,
  ],
  [
    'surfaces install progression even when the legacy child app state still says ready',
    expectInstallProgressionToSurfaceAlongsideLegacyReadyState,
  ],
  [
    'keeps support-assisted recovery blocked until custody handoff is cleared and emits a setup recovery operation',
    expectSupportAssistedRecoveryToStayBlockedUntilCustodyClears,
  ],
  [
    'only marks recovery recovered after applied bundle and delete settlement, and blocks wrong-key restores',
    expectRecoveryToRequireAppliedBundleAndDeleteSettlement,
  ],
] as const;

describe('setup family bridge', () => {
  for (const [testName, runTest] of SetupFamilyBridgeCases) {
    it(testName, runTest);
  }
});
