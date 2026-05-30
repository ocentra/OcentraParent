import { expect, it } from 'vitest';
import {
  V08CrossPlatformEnforcementCapabilityProofEntrySchema,
  V08CrossPlatformEnforcementCapabilityProofReadModel,
  V08CrossPlatformEnforcementCapabilityProofReadModelSchema,
  V08CrossPlatformEnforcementCapabilitySurface,
} from '../src/v0-8-cross-platform-enforcement-capability-proof';

it('captures cross-platform enforcement capability states without claim upgrades', () => {
  const readModel = V08CrossPlatformEnforcementCapabilityProofReadModelSchema.parse(
    V08CrossPlatformEnforcementCapabilityProofReadModel
  );
  const claimCounts = countBy(readModel.entries.map((entry) => entry.productClaimState));
  const platformCounts = countBy(readModel.entries.map((entry) => entry.platform));

  expect(readModel.entries).toHaveLength(15);
  expect(claimCounts).toEqual({
    'implemented-boundary': 4,
    'manual-required': 7,
    scaffold: 2,
    planned: 2,
  });
  expect(platformCounts).toEqual({
    windows: 6,
    linux: 1,
    macos: 1,
    android: 3,
    ios: 4,
  });
  expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
  expect(
    readModel.entries.every(
      (entry) =>
        !entry.broadBlockingClaimed &&
        !entry.exactUrlClaimed &&
        !entry.privilegedMobileClaimed &&
        !entry.productionDistributionClaimed
    )
  ).toBe(true);
});

it('keeps Windows proof scoped to implemented boundaries', () => {
  const ownedProcess = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsOwnedProcessTerminate);
  const appTimer = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsAppTimeLimitLifecycle);
  const broadApp = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsBroadInstalledAppBlocking);
  const networkDomain = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsNetworkDomainBlocking);

  expect(ownedProcess).toMatchObject({
    platform: 'windows',
    capability: 'owned-process-terminate',
    productClaimState: 'implemented-boundary',
    adapterExecutionState: 'executes-real-service',
  });
  expect(appTimer.claimBoundary).toContain('timer lifecycle');
  expect(broadApp).toMatchObject({
    capability: 'app-blocking',
    productClaimState: 'manual-required',
    adapterExecutionState: 'returns-manual-required',
  });
  expect(networkDomain.manualProofRequirements).toContain('host network filter adapter');
});

it('separates managed browser proof from unmanaged exact URL certainty', () => {
  const managed = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsManagedBrowserBoundary);
  const unmanaged = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsUnmanagedBrowserProcessBoundary);

  expect(managed.claimBoundary).toContain('Ocentra-owned managed browser boundary');
  expect(managed.exactUrlClaimed).toBe(false);
  expect(unmanaged.claimBoundary).toContain('cannot prove URL');
  expect(unmanaged.exactUrlClaimed).toBe(false);
});

it('records non-Windows and mobile privileged capabilities as scaffold, manual, or planned', () => {
  const linux = entryFor(V08CrossPlatformEnforcementCapabilitySurface.LinuxEnforcementAdapterScaffold);
  const macos = entryFor(V08CrossPlatformEnforcementCapabilitySurface.MacosEnforcementAdapterScaffold);
  const androidDeviceOwner = entryFor(V08CrossPlatformEnforcementCapabilitySurface.AndroidDeviceOwnerPolicy);
  const iosFamilyControls = entryFor(V08CrossPlatformEnforcementCapabilitySurface.IosFamilyControls);
  const iosStore = entryFor(V08CrossPlatformEnforcementCapabilitySurface.IosStoreDistribution);

  expect(linux).toMatchObject({ productClaimState: 'scaffold', adapterExecutionState: 'scaffold-only' });
  expect(macos.claimBoundary).toContain('cannot inherit Windows enforcement behavior');
  expect(androidDeviceOwner.manualProofRequirements).toContain('device-owner enrollment artifact');
  expect(iosFamilyControls.manualProofRequirements).toContain('Family Controls entitlement approval');
  expect(iosStore).toMatchObject({ capability: 'store-distribution', productClaimState: 'planned' });
});

it('rejects broad, exact URL, privileged mobile, and distribution claim upgrades', () => {
  const broadApp = entryFor(V08CrossPlatformEnforcementCapabilitySurface.WindowsBroadInstalledAppBlocking);
  const iosFamilyControls = entryFor(V08CrossPlatformEnforcementCapabilitySurface.IosFamilyControls);
  const iosStore = entryFor(V08CrossPlatformEnforcementCapabilitySurface.IosStoreDistribution);

  expect(() =>
    V08CrossPlatformEnforcementCapabilityProofEntrySchema.parse({
      ...broadApp,
      proofEntryId: 'invalid-broad-claim-upgrade',
      broadBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08CrossPlatformEnforcementCapabilityProofEntrySchema.parse({
      ...iosFamilyControls,
      proofEntryId: 'invalid-privileged-mobile-upgrade',
      privilegedMobileClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08CrossPlatformEnforcementCapabilityProofEntrySchema.parse({
      ...iosStore,
      proofEntryId: 'invalid-distribution-upgrade',
      productionDistributionClaimed: true,
    })
  ).toThrow();
});

function entryFor(surface: string) {
  const entry = V08CrossPlatformEnforcementCapabilityProofReadModel.entries.find(
    (candidate) => candidate.surface === surface
  );
  if (entry === undefined) {
    throw new Error(`Missing V0.8 cross-platform enforcement capability entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
