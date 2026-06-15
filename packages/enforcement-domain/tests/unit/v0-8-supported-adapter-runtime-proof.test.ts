import { describe, expect, it } from 'vitest';
import {
  V08SupportedAdapterRuntimeBoundary,
  V08SupportedAdapterRuntimeProofEntrySchema,
  V08SupportedAdapterRuntimeProofReadModel,
  V08SupportedAdapterRuntimeProofReadModelSchema,
} from '../../src/v0-8-supported-adapter-runtime-proof';

describe('V0.8 supported adapter runtime proof', () => {
  registerRuntimeStateTests();
  registerImplementedBoundaryTests();
  registerManualGateTests();
  registerPlatformStateTests();
  registerClaimUpgradeTests();
});

function registerRuntimeStateTests() {
  it('distinguishes every runtime state without unsupported claim upgrades', () => {
    const readModel = V08SupportedAdapterRuntimeProofReadModelSchema.parse(V08SupportedAdapterRuntimeProofReadModel);

    expect(readModel.readModelId).toBe('v0-8-supported-adapter-runtime-proof');
    expect(readModel.entries).toHaveLength(13);
    expect(countBy(readModel.entries.map((entry) => entry.runtimeState))).toEqual({
      'implemented-boundary': 2,
      'manual-required': 7,
      'not-claimed': 1,
      degraded: 1,
      unavailable: 1,
      unsupported: 1,
    });
    expect(countBy(readModel.entries.map((entry) => entry.platform))).toEqual({
      windows: 9,
      linux: 1,
      macos: 1,
      android: 1,
      ios: 1,
    });
    expect(readModel.entries.every((entry) => !entry.broadInstalledAppBlockingClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.networkDomainBlockingClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.exactActiveTabEnforcementClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.notificationDeliveryClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.tamperHardeningClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.mobileControlClaimed)).toBe(true);
    expect(readModel.entries.every((entry) => !entry.unsupportedPlatformBehaviorClaimed)).toBe(true);
  });
}

function registerImplementedBoundaryTests() {
  it('proves only app game owned process timer and network observe policy handoff boundaries', () => {
    const appGame = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsAppGameOwnedProcessTimeLimit);
    const networkObserve = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsNetworkFlowObservePolicyHandoff);

    expect(appGame).toMatchObject({
      runtimeState: 'implemented-boundary',
      adapterResult: 'supported-boundary-proved',
      targetIdentityState: 'process-session-evidence-backed',
      rollbackReferenceState: 'timer-recovery-backed',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'none',
    });
    expect(appGame.claimBoundary).toContain('not broad installed-app blocking');
    expect(networkObserve).toMatchObject({
      runtimeState: 'implemented-boundary',
      adapterResult: 'supported-boundary-proved',
      targetIdentityState: 'network-flow-evidence-backed',
      rollbackReferenceState: 'observe-only-not-needed',
      auditReferenceState: 'audit-reference-backed',
      refusalReason: 'none',
    });
    expect(networkObserve.claimBoundary).toContain('observe-only policy handoff');
  });
}

function registerManualGateTests() {
  it('keeps broad app host network exact active tab and mobile states proof gated', () => {
    const broadApp = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsBroadInstalledAppBlockingManualGate);
    const hostNetwork = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainBlockingManualGate);
    const broadAppArtifact = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsBroadInstalledAppArtifactStatus);
    const networkArtifact = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsHostNetworkDomainArtifactStatus);
    const managedBrowserArtifact = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsManagedBrowserArtifactStatus);
    const exactActiveTab = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsManagedExactActiveTabNotClaimed);
    const android = entryFor(V08SupportedAdapterRuntimeBoundary.AndroidMobileControlManualGate);
    const ios = entryFor(V08SupportedAdapterRuntimeBoundary.IosMobileControlManualGate);

    expect(broadApp.runtimeState).toBe('manual-required');
    expect(broadApp.manualProofRequirements).toContain('host block apply artifact');
    expect(hostNetwork.runtimeState).toBe('manual-required');
    expect(hostNetwork.manualProofRequirements).toContain('host DNS or filter apply artifact');
    expect(broadAppArtifact).toMatchObject({
      runtimeState: 'manual-required',
      adapterCapability: 'broad-installed-app-artifact-status',
      adapterResult: 'manual-proof-required',
    });
    expect(broadAppArtifact.linkedProofCommands).toContain(
      'node scripts/test/v0-8-windows-adapter-artifact-ingestion-proof.mjs'
    );
    expect(broadAppArtifact.claimBoundary).toContain('manual review only');
    expect(networkArtifact.adapterCapability).toBe('host-network-domain-artifact-status');
    expect(networkArtifact.manualProofRequirements).toContain('network/domain filter rollback result');
    expect(managedBrowserArtifact.adapterCapability).toBe('managed-browser-artifact-status');
    expect(managedBrowserArtifact.claimBoundary).toContain('active-tab enforcement');
    expect(exactActiveTab).toMatchObject({
      runtimeState: 'not-claimed',
      adapterResult: 'not-claimed',
      exactActiveTabEnforcementClaimed: false,
    });
    expect(android.manualProofRequirements).toContain('device-owner or managed-profile artifact');
    expect(ios.manualProofRequirements).toContain('Family Controls entitlement artifact');
  });
}

function registerPlatformStateTests() {
  it('records unavailable unsupported and degraded adapter states explicitly', () => {
    const linux = entryFor(V08SupportedAdapterRuntimeBoundary.LinuxHostAdapterUnavailable);
    const macos = entryFor(V08SupportedAdapterRuntimeBoundary.MacosHostAdapterUnsupported);
    const degraded = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsAdapterPermissionDependencyDegraded);

    expect(linux).toMatchObject({
      runtimeState: 'unavailable',
      adapterResult: 'target-unavailable',
      platformSupportState: 'unavailable-on-target',
      refusalReason: 'target-unavailable',
    });
    expect(macos).toMatchObject({
      runtimeState: 'unsupported',
      adapterResult: 'unsupported-platform',
      platformSupportState: 'unsupported-platform',
      refusalReason: 'unsupported-platform',
    });
    expect(degraded).toMatchObject({
      runtimeState: 'degraded',
      adapterResult: 'degraded-permission-or-dependency',
      platformSupportState: 'degraded',
      refusalReason: 'permission-or-dependency-degraded',
    });
  });
}

function registerClaimUpgradeTests() {
  it('rejects broad blocking exact active tab notification tamper mobile and unsupported platform claim upgrades', () => {
    const appGame = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsAppGameOwnedProcessTimeLimit);
    const exactActiveTab = entryFor(V08SupportedAdapterRuntimeBoundary.WindowsManagedExactActiveTabNotClaimed);
    const android = entryFor(V08SupportedAdapterRuntimeBoundary.AndroidMobileControlManualGate);
    const macos = entryFor(V08SupportedAdapterRuntimeBoundary.MacosHostAdapterUnsupported);

    for (const invalidEntry of [
      { ...appGame, proofEntryId: 'invalid-broad-app-claim', broadInstalledAppBlockingClaimed: true },
      { ...appGame, proofEntryId: 'invalid-network-claim', networkDomainBlockingClaimed: true },
      { ...exactActiveTab, proofEntryId: 'invalid-exact-tab-claim', exactActiveTabEnforcementClaimed: true },
      { ...appGame, proofEntryId: 'invalid-notification-claim', notificationDeliveryClaimed: true },
      { ...appGame, proofEntryId: 'invalid-tamper-claim', tamperHardeningClaimed: true },
      { ...android, proofEntryId: 'invalid-mobile-claim', mobileControlClaimed: true },
      { ...macos, proofEntryId: 'invalid-unsupported-platform-claim', unsupportedPlatformBehaviorClaimed: true },
    ]) {
      expect(() => V08SupportedAdapterRuntimeProofEntrySchema.parse(invalidEntry)).toThrow();
    }
  });
}

function entryFor(runtimeBoundary: string) {
  const entry = V08SupportedAdapterRuntimeProofReadModel.entries.find(
    (candidate) => candidate.runtimeBoundary === runtimeBoundary
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 supported adapter runtime proof entry: ${runtimeBoundary}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
