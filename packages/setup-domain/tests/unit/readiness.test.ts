import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/family-domain/reference-primitives';
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
} from '../../src/readiness';
import { SetupPairingState } from '../../src/pairing-intent';

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

describe('setup readiness contracts', () => {
  it('derives ready state when all readiness dimensions are satisfied', () => {
    expect(isSetupReadinessReady(ReadinessReport)).toBe(true);
    expect(deriveSetupReadinessOverallState(ReadinessReport)).toBe(SetupReadinessOverallState.Ready);
    expect(deriveSetupChildInstallJourneyStage(ReadinessReport)).toBe(SetupChildInstallJourneyStage.Paired);
  });

  it('treats offline child state as degraded instead of fake ready', () => {
    expect(
      deriveSetupReadinessOverallState(
        SetupReadinessReportSchema.parse({
          ...ReadinessReport,
          childAppState: SetupChildAppReadinessState.Offline,
          childServiceState: SetupChildServiceState.Offline,
          networkReachabilityState: SetupNetworkReachabilityState.OfflineChild,
        })
      )
    ).toBe(SetupReadinessOverallState.Degraded);
  });

  it('blocks readiness on wrong-account and stale pairing states', () => {
    expect(
      deriveSetupReadinessOverallState(
        SetupReadinessReportSchema.parse({
          ...ReadinessReport,
          accountState: SetupAccountReadinessState.WrongAccount,
          pairingState: SetupPairingState.WrongHousehold,
        })
      )
    ).toBe(SetupReadinessOverallState.Blocked);
  });

  it('keeps accepted pairing blocked without forcing manual recovery before parent confirmation', () => {
    const report = SetupReadinessReportSchema.parse({
      ...ReadinessReport,
      pairingState: SetupPairingState.Accepted,
      recoveryState: SetupRecoveryState.Normal,
    });

    expect(deriveSetupReadinessOverallState(report)).toBe(SetupReadinessOverallState.Blocked);
    expect(deriveSetupChildInstallJourneyStage(report)).toBe(SetupChildInstallJourneyStage.Paired);
    expect(setupReadinessNeedsManualRecovery(report)).toBe(false);
  });

  it('builds a typed readiness checklist with separate install and service checkpoints', () => {
    const checklist = createSetupReadinessChecklist(
      SetupReadinessReportSchema.parse({
        ...ReadinessReport,
        childAppState: SetupChildAppReadinessState.Offline,
        childServiceState: SetupChildServiceState.Offline,
        networkReachabilityState: SetupNetworkReachabilityState.OfflineChild,
      })
    );

    expect(checklist).toHaveLength(10);
    expect(checklist.find((entry) => entry.checklistItemId === 'setup-child-install-state')?.state).toBe('complete');
    expect(checklist.find((entry) => entry.checklistItemId === 'setup-child-service-state')?.state).toBe('degraded');
    expect(checklist.find((entry) => entry.checklistItemId === 'setup-overall-state')?.supportCode).toBe('degraded');
  });

  it('uses explicit install and service states instead of collapsing to legacy child ready state', () => {
    const installedReport = SetupReadinessReportSchema.parse({
      ...ReadinessReport,
      childAppState: SetupChildAppReadinessState.Ready,
      childInstallState: SetupChildInstallState.Installed,
      childServiceState: SetupChildServiceState.NotStarted,
      permissionState: SetupPermissionReadinessState.Missing,
      pairingState: SetupPairingState.Displayed,
      policyBaselineState: SetupPolicyBaselineState.Missing,
    });
    const permissionedReport = SetupReadinessReportSchema.parse({
      ...ReadinessReport,
      childAppState: SetupChildAppReadinessState.Ready,
      childInstallState: SetupChildInstallState.Installed,
      childServiceState: SetupChildServiceState.ServiceStarted,
      permissionState: SetupPermissionReadinessState.Granted,
      pairingState: SetupPairingState.Displayed,
      policyBaselineState: SetupPolicyBaselineState.Missing,
    });
    const trustedReport = SetupReadinessReportSchema.parse({
      ...ReadinessReport,
      childAppState: SetupChildAppReadinessState.Ready,
      childInstallState: SetupChildInstallState.Installed,
      childServiceState: SetupChildServiceState.ServiceStarted,
      permissionState: SetupPermissionReadinessState.Granted,
      pairingState: SetupPairingState.Trusted,
      policyBaselineState: SetupPolicyBaselineState.Missing,
    });

    expect(deriveSetupChildInstallJourneyStage(installedReport)).toBe(SetupChildInstallJourneyStage.Installed);
    expect(deriveSetupChildInstallJourneyStage(permissionedReport)).toBe(
      SetupChildInstallJourneyStage.Permissioned
    );
    expect(deriveSetupChildInstallJourneyStage(trustedReport)).toBe(SetupChildInstallJourneyStage.Paired);
  });

  it('marks non-normal recovery flows as manual recovery work', () => {
    expect(
      setupReadinessNeedsManualRecovery(
        SetupReadinessReportSchema.parse({
          ...ReadinessReport,
          recoveryState: SetupRecoveryState.Required,
        })
      )
    ).toBe(true);
  });

  it('parses recovery operations for lost parent device and rejects empty operation ids', () => {
    expect(
      SetupRecoveryOperationSchema.safeParse({
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
      }).success
    ).toBe(true);

    expect(
      SetupRecoveryOperationSchema.safeParse({
        schemaVersion: ParentContractSchemaVersion.V0_6,
        recoveryOperationId: '',
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
        kind: SetupRecoveryKind.StaleCode,
        state: SetupRecoveryState.Required,
        sourcePairingState: SetupPairingState.Expired,
        openedAt: '2026-06-01T00:15:00Z',
        resolvedAt: null,
      }).success
    ).toBe(false);
  });

  it('parses readiness reports with typed checklist items', () => {
    expect(
      SetupReadinessReportSchema.safeParse({
        ...ReadinessReport,
        checklist: createSetupReadinessChecklist(ReadinessReport),
      }).success
    ).toBe(true);
  });
});
