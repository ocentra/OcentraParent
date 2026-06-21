import { expect, it } from 'vitest';
import {
  V08BroadOsAdapterRuntimeProofEntrySchema,
  V08BroadOsAdapterRuntimeProofReadModel,
  V08BroadOsAdapterRuntimeProofReadModelSchema,
  V08BroadOsAdapterRuntimeSurface,
} from '@ocentra-parent/schema-domain/v0-8-broad-os-adapter-runtime-proof';

it('captures the final broad app domain and browser runtime proof pass without claim upgrades', () => {
  const readModel = V08BroadOsAdapterRuntimeProofReadModelSchema.parse(V08BroadOsAdapterRuntimeProofReadModel);
  const claimCounts = countBy(readModel.entries.map((entry) => entry.productClaimState));
  const evidenceCounts = countBy(readModel.entries.map((entry) => entry.evidenceState));
  const platformCounts = countBy(readModel.entries.map((entry) => entry.platform));

  expect(readModel.readModelId).toBe('v0-8-broad-os-adapter-runtime-proof');
  expect(readModel.entries).toHaveLength(10);
  expect(claimCounts).toEqual({
    'implemented-boundary': 2,
    'manual-required': 6,
    unavailable: 1,
    'not-claimed': 1,
  });
  expect(evidenceCounts).toEqual({
    'composite-runtime-proof': 2,
    'manual-artifact-required': 6,
    'target-unavailable': 1,
    'not-implemented': 1,
  });
  expect(platformCounts).toEqual({
    windows: 6,
    linux: 1,
    macos: 1,
    android: 1,
    ios: 1,
  });
  expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
  expect(readModel.entries.every((entry) => !entry.broadInstalledAppBlockingClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.networkDomainBlockingClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.managedBrowserExactUrlClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.unmanagedBrowserExactEvidenceClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.unsupportedPlatformClaimed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.mobilePrivilegeClaimed)).toBe(true);
});

it('links the final pass to the prior broad browser domain manual gate and product proof slices', () => {
  const sourceIds = new Set(V08BroadOsAdapterRuntimeProofReadModel.sourceReadModelIds);

  expect(sourceIds).toEqual(
    new Set([
      'v0-8-broad-os-adapter-proof',
      'v0-8-browser-domain-adapter-proof',
      'v0-8-os-adapter-manual-artifact-gates',
      'v0-8-os-adapter-product-proof',
    ])
  );
  expect(
    V08BroadOsAdapterRuntimeProofReadModel.entries.every((entry) =>
      entry.sourceProofIds.every((sourceId) => sourceIds.has(sourceId))
    )
  ).toBe(true);
});

it('keeps implemented runtime boundaries separate from broad app domain and exact browser claims', () => {
  const ownedRuntime = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsOwnedProcessAndTimerRuntimeBoundary);
  const managedSession = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsManagedBrowserSessionRuntimeBoundary);
  const broadApp = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsBroadInstalledAppRuntimeGate);
  const networkDomain = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsNetworkDomainRuntimeGate);
  const managedExact = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsManagedBrowserExactUrlRuntimeGate);
  const unmanagedExact = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsUnmanagedBrowserExactEvidenceRuntimeGap);

  expect([ownedRuntime, managedSession].every((entry) => entry.productClaimState === 'implemented-boundary')).toBe(
    true
  );
  expect(ownedRuntime.claimBoundary).toContain('not broad installed-app blocking');
  expect(managedSession.claimBoundary).toContain('does not prove exact active-tab URL enforcement');
  expect(broadApp.productClaimState).toBe('manual-required');
  expect(networkDomain.productClaimState).toBe('manual-required');
  expect(managedExact.productClaimState).toBe('manual-required');
  expect(unmanagedExact).toMatchObject({
    productClaimState: 'not-claimed',
    evidenceState: 'not-implemented',
    unmanagedBrowserExactEvidenceClaimed: false,
  });
});

it('records non Windows and mobile runtime support as unavailable or manual-required', () => {
  const linux = entryFor(V08BroadOsAdapterRuntimeSurface.LinuxHostRuntimeUnavailable);
  const macos = entryFor(V08BroadOsAdapterRuntimeSurface.MacosHostRuntimeManualGate);
  const android = entryFor(V08BroadOsAdapterRuntimeSurface.AndroidMobileRuntimeManualGate);
  const ios = entryFor(V08BroadOsAdapterRuntimeSurface.IosMobileRuntimeManualGate);

  expect(linux).toMatchObject({
    platform: 'linux',
    productClaimState: 'unavailable',
    evidenceState: 'target-unavailable',
  });
  expect(macos.manualProofRequirements).toContain(
    'macOS permission, package, service, apply, rollback, and audit artifacts'
  );
  expect(android.manualProofRequirements).toContain('device-owner or managed-profile artifact');
  expect(ios.manualProofRequirements).toContain('Family Controls entitlement artifact');
});

it('rejects app domain browser platform and mobile claim upgrades without final-pass proof', () => {
  const broadApp = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsBroadInstalledAppRuntimeGate);
  const networkDomain = entryFor(V08BroadOsAdapterRuntimeSurface.WindowsNetworkDomainRuntimeGate);
  const android = entryFor(V08BroadOsAdapterRuntimeSurface.AndroidMobileRuntimeManualGate);

  expect(() =>
    V08BroadOsAdapterRuntimeProofEntrySchema.parse({
      ...broadApp,
      proofEntryId: 'invalid-broad-app-upgrade',
      broadInstalledAppBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08BroadOsAdapterRuntimeProofEntrySchema.parse({
      ...networkDomain,
      proofEntryId: 'invalid-domain-upgrade',
      networkDomainBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08BroadOsAdapterRuntimeProofEntrySchema.parse({
      ...android,
      proofEntryId: 'invalid-mobile-upgrade',
      mobilePrivilegeClaimed: true,
    })
  ).toThrow();
});

function entryFor(surface: string) {
  const entry = V08BroadOsAdapterRuntimeProofReadModel.entries.find(
    (candidate) => candidate.runtimeSurface === surface
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 broad OS adapter runtime proof entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
