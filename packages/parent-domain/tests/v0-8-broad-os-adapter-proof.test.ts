import { expect, it } from 'vitest';
import {
  V08BroadOsAdapterProofEntrySchema,
  V08BroadOsAdapterProofReadModel,
  V08BroadOsAdapterProofReadModelSchema,
  V08BroadOsAdapterProofSurface,
} from '../src/v0-8-broad-os-adapter-proof';

it('captures the current broad OS adapter proof boundary without claim upgrades', () => {
  const readModel = V08BroadOsAdapterProofReadModelSchema.parse(V08BroadOsAdapterProofReadModel);
  const proofCounts = countBy(readModel.entries.map((entry) => entry.runtimeProofState));
  const platformCounts = countBy(readModel.entries.map((entry) => entry.platform));

  expect(readModel.entries).toHaveLength(13);
  expect(proofCounts).toEqual({
    'real-service-proof': 4,
    'manual-required': 7,
    unavailable: 1,
    'not-claimed': 1,
  });
  expect(platformCounts).toEqual({
    windows: 9,
    linux: 1,
    macos: 1,
    android: 1,
    ios: 1,
  });
  expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
  expect(readModel.entries.every((entry) => !entry.claimUpgradeAllowed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.broadOsBlockingClaimed && !entry.exactUrlClaimed)).toBe(true);
});

it('keeps proved Windows mechanics separate from broad app and browser claims', () => {
  const managedSession = entryFor(V08BroadOsAdapterProofSurface.WindowsManagedSessionIntervention);
  const ownedProcess = entryFor(V08BroadOsAdapterProofSurface.WindowsOwnedProcessGuardrail);
  const unmanagedBoundary = entryFor(V08BroadOsAdapterProofSurface.WindowsUnmanagedProcessBoundary);
  const appTimer = entryFor(V08BroadOsAdapterProofSurface.WindowsAppTimeLimitLifecycle);
  const broadApp = entryFor(V08BroadOsAdapterProofSurface.WindowsBroadInstalledAppBlocking);
  const exactEvidence = entryFor(V08BroadOsAdapterProofSurface.WindowsUnmanagedExactEvidence);

  expect(
    [managedSession, ownedProcess, unmanagedBoundary, appTimer].every(
      (entry) => entry.runtimeProofState === 'real-service-proof'
    )
  ).toBe(true);
  expect(managedSession.claimBoundary).toContain('managed browser path');
  expect(ownedProcess.claimBoundary).toContain('not broad installed-app blocking');
  expect(unmanagedBoundary.claimBoundary).toContain('process terminate/warn only');
  expect(broadApp).toMatchObject({
    runtimeProofState: 'manual-required',
    targetSupport: 'manual-proof-required',
  });
  expect(exactEvidence).toMatchObject({
    runtimeProofState: 'not-claimed',
    exactUrlClaimed: false,
    targetSupport: 'not-implemented',
  });
});

it('records target OS support explicitly instead of borrowing Windows proof', () => {
  const linux = entryFor(V08BroadOsAdapterProofSurface.LinuxBroadOsAdapter);
  const macos = entryFor(V08BroadOsAdapterProofSurface.MacosBroadOsAdapter);
  const android = entryFor(V08BroadOsAdapterProofSurface.AndroidChildOsAdapter);
  const ios = entryFor(V08BroadOsAdapterProofSurface.IosChildOsAdapter);

  expect(linux).toMatchObject({
    platform: 'linux',
    runtimeProofState: 'unavailable',
    targetSupport: 'unavailable-on-target',
  });
  expect(macos.manualProofRequirements).toContain('macOS permissions');
  expect(android.manualProofRequirements).toContain('device-owner or managed-profile proof');
  expect(ios.manualProofRequirements).toContain('Family Controls entitlement');
});

it('rejects broad OS blocking and exact URL upgrades without matching proof', () => {
  const broadApp = entryFor(V08BroadOsAdapterProofSurface.WindowsBroadInstalledAppBlocking);
  const exactEvidence = entryFor(V08BroadOsAdapterProofSurface.WindowsUnmanagedExactEvidence);

  expect(() =>
    V08BroadOsAdapterProofEntrySchema.parse({
      ...broadApp,
      proofEntryId: 'invalid-broad-os-upgrade',
      broadOsBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08BroadOsAdapterProofEntrySchema.parse({
      ...exactEvidence,
      proofEntryId: 'invalid-exact-url-upgrade',
      exactUrlClaimed: true,
      claimUpgradeAllowed: true,
    })
  ).toThrow();
});

function entryFor(surface: string) {
  const entry = V08BroadOsAdapterProofReadModel.entries.find((candidate) => candidate.surface === surface);
  if (entry === undefined) {
    throw new Error(`Missing V0.8 broad OS adapter proof entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
