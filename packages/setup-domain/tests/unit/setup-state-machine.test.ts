import { describe, expect, it } from 'vitest';
import { ParentContractSchemaVersion } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  SetupChildAppReadinessState,
  SetupNetworkReachabilityState,
  SetupParentAppReadinessState,
  SetupPermissionReadinessState,
  SetupPolicyBaselineState,
  SetupDataCustodySyncState,
  SetupAccountReadinessState,
  SetupReadinessOverallState,
  SetupReadinessReportSchema,
  SetupRecoveryState,
  type SetupReadinessReport,
} from '@ocentra-parent/schema-domain/setup-readiness';
import { SetupPairingState } from '@ocentra-parent/schema-domain/setup-pairing-intent';
import {
  canTransitionSetupFirstRunState,
  resolveSetupFirstRunState,
  SetupFirstRunScreenId,
  SetupFirstRunStateId,
  transitionSetupFirstRunState,
} from '@ocentra-parent/schema-domain/setup-state-machine';

const BaseReadinessReport = SetupReadinessReportSchema.parse({
  schemaVersion: ParentContractSchemaVersion.V0_6,
  readinessReportId: 'setup-readiness-report-state-machine-1',
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
    ...BaseReadinessReport,
    ...overrides,
  });
}

describe('setup first-run state machine', () => {
  it('moves from welcome to account entry and rejects skipped screens', () => {
    const state = transitionSetupFirstRunState({
      fromStateId: SetupFirstRunStateId.Welcome,
      toStateId: SetupFirstRunStateId.AccountEntry,
      readinessReport: null,
    });

    expect(state.stateId).toBe(SetupFirstRunStateId.AccountEntry);
    expect(state.screenId).toBe(SetupFirstRunScreenId.AccountEntry);
    expect(canTransitionSetupFirstRunState(SetupFirstRunStateId.Welcome, SetupFirstRunStateId.ChildProfile)).toBe(false);
  });

  it('routes readiness-gated states to degraded when the child is offline', () => {
    const state = resolveSetupFirstRunState({
      stateId: SetupFirstRunStateId.WaitingForChildDevice,
      readinessReport: createReadinessReport({
        childAppState: SetupChildAppReadinessState.Offline,
        networkReachabilityState: SetupNetworkReachabilityState.OfflineChild,
      }),
    });

    expect(state.stateId).toBe(SetupFirstRunStateId.SetupDegraded);
    expect(state.screenId).toBe(SetupFirstRunScreenId.Recovery);
    expect(state.degraded).toBe(true);
    expect(state.complete).toBe(false);
  });

  it('routes readiness-gated states to manual required when recovery work is open', () => {
    const state = resolveSetupFirstRunState({
      stateId: SetupFirstRunStateId.PermissionReadiness,
      readinessReport: createReadinessReport({
        recoveryState: SetupRecoveryState.Required,
        pairingState: SetupPairingState.Accepted,
      }),
    });

    expect(state.stateId).toBe(SetupFirstRunStateId.ManualRequired);
    expect(state.screenId).toBe(SetupFirstRunScreenId.ManualRequired);
    expect(state.manualRequired).toBe(true);
  });

  it('requires a readiness report before setup can complete', () => {
    expect(() =>
      transitionSetupFirstRunState({
        fromStateId: SetupFirstRunStateId.DataCustody,
        toStateId: SetupFirstRunStateId.SetupComplete,
        readinessReport: null,
      })
    ).toThrow('Setup first-run completion requires a readiness report.');
  });

  it('does not allow fake ready completion when readiness is still blocked', () => {
    const state = transitionSetupFirstRunState({
      fromStateId: SetupFirstRunStateId.DataCustody,
      toStateId: SetupFirstRunStateId.SetupComplete,
      readinessReport: createReadinessReport({
        pairingState: SetupPairingState.Accepted,
      }),
    });

    expect(state.stateId).toBe(SetupFirstRunStateId.SetupBlocked);
    expect(state.blocked).toBe(true);
    expect(state.complete).toBe(false);
  });

  it('allows setup completion only after the readiness vocabulary reports ready', () => {
    const state = transitionSetupFirstRunState({
      fromStateId: SetupFirstRunStateId.DataCustody,
      toStateId: SetupFirstRunStateId.SetupComplete,
      readinessReport: createReadinessReport(),
    });

    expect(state.stateId).toBe(SetupFirstRunStateId.SetupComplete);
    expect(state.screenId).toBe(SetupFirstRunScreenId.SetupComplete);
    expect(state.readinessState).toBe(SetupReadinessOverallState.Ready);
    expect(state.complete).toBe(true);
  });
});
