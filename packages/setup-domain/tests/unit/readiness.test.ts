import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  createSetupReadinessChecklist,
  deriveSetupChildInstallJourneyStage,
  deriveSetupReadinessOverallState,
  isSetupReadinessReady,
  setupReadinessNeedsManualRecovery,
  SetupAccountReadinessState,
  SetupChildAppReadinessState,
  SetupChildInstallJourneyStage,
  SetupChildInstallState,
  SetupChildServiceState,
  SetupDataCustodySyncState,
  SetupNetworkReachabilityState,
  SetupParentAppReadinessState,
  SetupPermissionReadinessState,
  SetupPolicyBaselineState,
  SetupReadinessOverallState,
  SetupReadinessReportSchema,
  SetupRecoveryOperationSchema,
  SetupRecoveryKind,
  SetupRecoveryState,
  type SetupReadinessReport,
} from '@ocentra-parent/schema-domain/setup-readiness';
import { SetupPairingState } from '@ocentra-parent/schema-domain/setup-pairing-intent';

const ReadinessReport = SetupReadinessReportSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readinessReportId: 'setup-readiness-report-1',
  family: {
    familyId: 'family-local-1',
  },
  parentAccount: {
    parentAccountId: 'parent-account-1',
  },
  parentDevice: {
    deviceId: 'parent-device-1',
    childProfileId: null,
    label: 'Mom phone',
    platform: 'android',
  },
  childProfile: {
    childProfileId: 'child-profile-1',
    displayName: 'Ari',
  },
  pairingIntentId: 'setup-pairing-intent-1',
  accountState: SetupAccountReadinessState.Ready,
  parentAppState: SetupParentAppReadinessState.Ready,
  childAppState: SetupChildAppReadinessState.Ready,
  childInstallState: SetupChildInstallState.Installed,
  childServiceState: SetupChildServiceState.ServiceStarted,
  permissionState: SetupPermissionReadinessState.Granted,
  pairingState: SetupPairingState.Trusted,
  policyBaselineState: SetupPolicyBaselineState.Applied,
  dataCustodySyncState: SetupDataCustodySyncState.Synced,
  networkReachabilityState: SetupNetworkReachabilityState.Reachable,
  recoveryState: SetupRecoveryState.Normal,
  observedAt: '2026-06-01T00:15:00Z',
  checklist: [],
});

function createReadinessReport(overrides: Partial<SetupReadinessReport> = {}) {
  return SetupReadinessReportSchema.parse({
    ...ReadinessReport,
    ...overrides,
  });
}

function createRecoveryOperation(overrides: Record<string, unknown> = {}) {
  return {
    schemaVersion: ParentContractSchemaVersion.V0_6,
    recoveryOperationId: 'setup-recovery-1',
    family: {
      familyId: 'family-local-1',
    },
    parentAccount: {
      parentAccountId: 'parent-account-1',
    },
    parentDevice: {
      deviceId: 'parent-device-1',
      childProfileId: null,
      label: 'Mom phone',
      platform: 'android',
    },
    childProfile: {
      childProfileId: 'child-profile-1',
      displayName: 'Ari',
    },
    childDevice: null,
    kind: SetupRecoveryKind.LostParentDevice,
    state: SetupRecoveryState.InProgress,
    sourcePairingState: SetupPairingState.Revoked,
    openedAt: '2026-06-01T00:15:00Z',
    resolvedAt: null,
    ...overrides,
  };
}

function expectReadyStateWhenAllDimensionsAreSatisfied() {
  expect(isSetupReadinessReady(ReadinessReport)).toBe(true);
  expect(deriveSetupReadinessOverallState(ReadinessReport)).toBe(SetupReadinessOverallState.Ready);
  expect(deriveSetupChildInstallJourneyStage(ReadinessReport)).toBe(SetupChildInstallJourneyStage.Paired);
}

function expectOfflineChildStateToBeDegraded() {
  expect(
    deriveSetupReadinessOverallState(
      createReadinessReport({
        childAppState: SetupChildAppReadinessState.Offline,
        childServiceState: SetupChildServiceState.Offline,
        networkReachabilityState: SetupNetworkReachabilityState.OfflineChild,
      })
    )
  ).toBe(SetupReadinessOverallState.Degraded);
}

function expectWrongAccountAndStalePairingStatesToBlockReadiness() {
  expect(
    deriveSetupReadinessOverallState(
      createReadinessReport({
        accountState: SetupAccountReadinessState.WrongAccount,
        pairingState: SetupPairingState.WrongHousehold,
      })
    )
  ).toBe(SetupReadinessOverallState.Blocked);
}

function expectAcceptedPairingToStayBlockedWithoutManualRecovery() {
  const report = createReadinessReport({
    pairingState: SetupPairingState.Accepted,
    recoveryState: SetupRecoveryState.Normal,
  });

  expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
  expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Paired);
  expect(setupReadinessNeedsManualRecovery(report)).toBe(false);
}

function expectTypedReadinessChecklist() {
  const checklist = createSetupReadinessChecklist(
    createReadinessReport({
      childAppState: SetupChildAppReadinessState.Offline,
      childServiceState: SetupChildServiceState.Offline,
      networkReachabilityState: SetupNetworkReachabilityState.OfflineChild,
    })
  );

  expect(checklist).toHaveLength(10);
  expect(checklist.find((entry) => entry.checklistItemId === 'setup-child-install-state')?.state).toBe('complete');
  expect(checklist.find((entry) => entry.checklistItemId === 'setup-child-service-state')?.state).toBe('degraded');
  expect(checklist.find((entry) => entry.checklistItemId === 'setup-overall-state')?.supportCode).toBe('degraded');
}

function expectJourneyStageToUseExplicitInstallAndServiceStates() {
  const installedReport = createReadinessReport({
    childAppState: SetupChildAppReadinessState.Ready,
    childInstallState: SetupChildInstallState.Installed,
    childServiceState: SetupChildServiceState.NotStarted,
    permissionState: SetupPermissionReadinessState.Missing,
    pairingState: SetupPairingState.Displayed,
    policyBaselineState: SetupPolicyBaselineState.Missing,
  });
  const permissionedReport = createReadinessReport({
    childAppState: SetupChildAppReadinessState.Ready,
    childInstallState: SetupChildInstallState.Installed,
    childServiceState: SetupChildServiceState.ServiceStarted,
    permissionState: SetupPermissionReadinessState.Granted,
    pairingState: SetupPairingState.Displayed,
    policyBaselineState: SetupPolicyBaselineState.Missing,
  });
  const trustedReport = createReadinessReport({
    childAppState: SetupChildAppReadinessState.Ready,
    childInstallState: SetupChildInstallState.Installed,
    childServiceState: SetupChildServiceState.ServiceStarted,
    permissionState: SetupPermissionReadinessState.Granted,
    pairingState: SetupPairingState.Trusted,
    policyBaselineState: SetupPolicyBaselineState.Missing,
  });

  expect(deriveSetupChildInstallJourneyStage(installedReport)).toBe(SetupChildInstallJourneyStage.Installed);
  expect(deriveSetupChildInstallJourneyStage(permissionedReport)).toBe(SetupChildInstallJourneyStage.Permissioned);
  expect(deriveSetupChildInstallJourneyStage(trustedReport)).toBe(SetupChildInstallJourneyStage.Paired);
}

function expectManualRecoveryForNonNormalRecoveryFlows() {
  expect(setupReadinessNeedsManualRecovery(createReadinessReport({ recoveryState: SetupRecoveryState.Required }))).toBe(
    true
  );
}

function expectRecoveryOperationsToParseAndRejectEmptyIds() {
  expect(SetupRecoveryOperationSchema.safeParse(createRecoveryOperation()).success).toBe(true);
  expect(
    SetupRecoveryOperationSchema.safeParse(
      createRecoveryOperation({
        recoveryOperationId: '',
        kind: SetupRecoveryKind.StaleCode,
        state: SetupRecoveryState.Required,
        sourcePairingState: SetupPairingState.Expired,
      })
    ).success
  ).toBe(false);
}

function expectReadinessReportsToParseTypedChecklistItems() {
  expect(
    SetupReadinessReportSchema.safeParse({
      ...ReadinessReport,
      checklist: createSetupReadinessChecklist(ReadinessReport),
    }).success
  ).toBe(true);
}

describe('setup readiness contracts', () => {
  it('derives ready state when all readiness dimensions are satisfied', expectReadyStateWhenAllDimensionsAreSatisfied);

  it('treats offline child state as degraded instead of incorrectly ready', expectOfflineChildStateToBeDegraded);

  it(
    'blocks readiness on wrong-account and stale pairing states',
    expectWrongAccountAndStalePairingStatesToBlockReadiness
  );

  it(
    'keeps accepted pairing blocked without forcing manual recovery before parent confirmation',
    expectAcceptedPairingToStayBlockedWithoutManualRecovery
  );

  it('builds a typed readiness checklist with separate install and service checkpoints', expectTypedReadinessChecklist);

  it(
    'uses explicit install and service states instead of collapsing to legacy child ready state',
    expectJourneyStageToUseExplicitInstallAndServiceStates
  );

  it('marks non-normal recovery flows as manual recovery work', expectManualRecoveryForNonNormalRecoveryFlows);

  it(
    'parses recovery operations for lost parent device and rejects empty operation ids',
    expectRecoveryOperationsToParseAndRejectEmptyIds
  );

  it('parses readiness reports with typed checklist items', expectReadinessReportsToParseTypedChecklistItems);
});
