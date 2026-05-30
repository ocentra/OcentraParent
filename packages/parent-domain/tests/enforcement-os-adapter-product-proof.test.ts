import { expect, it } from 'vitest';
import {
  V08OsAdapterProductProofEntrySchema,
  V08OsAdapterProductProofReadModel,
  V08OsAdapterProductProofReadModelSchema,
  V08OsAdapterProductProofSurface,
} from '../src/enforcement-os-adapter-product-proof';

it('captures the product proof read model without broad blocking or exact URL upgrades', () => {
  const readModel = V08OsAdapterProductProofReadModelSchema.parse(V08OsAdapterProductProofReadModel);
  const readinessCounts = countBy(readModel.entries.map((entry) => entry.readinessState));
  const capabilityCounts = countBy(readModel.entries.map((entry) => entry.capabilityState));
  const auditCounts = countBy(readModel.entries.map((entry) => entry.auditState));

  expect(readModel.entries).toHaveLength(12);
  expect(readinessCounts).toEqual({
    implemented: 6,
    'manual-required': 5,
    'not-claimed': 1,
  });
  expect(capabilityCounts).toEqual({
    supported: 6,
    'manual-required': 6,
  });
  expect(auditCounts).toEqual({
    journaled: 6,
    'manual-required': 5,
    unavailable: 1,
  });
  expect(new Set(readModel.entries.map((entry) => entry.proofEntryId)).size).toBe(readModel.entries.length);
  expect(readModel.entries.every((entry) => !entry.claimUpgradeAllowed)).toBe(true);
  expect(readModel.entries.every((entry) => !entry.broadBlockingClaimed && !entry.exactUrlClaimed)).toBe(true);
});

it('preserves restart recovery, parent cancel, audit, and rollback artifact boundaries', () => {
  const restartRecovery = entryFor(V08OsAdapterProductProofSurface.RestartRecovery);
  const parentCancel = entryFor(V08OsAdapterProductProofSurface.ParentCancelOverride);
  const auditCustody = entryFor(V08OsAdapterProductProofSurface.AuditCustody);
  const rollbackGate = entryFor(V08OsAdapterProductProofSurface.RollbackArtifactGate);

  expect(restartRecovery).toMatchObject({
    readinessState: 'implemented',
    timerRecoveryState: 'restart-recovered',
    auditState: 'journaled',
  });
  expect(parentCancel).toMatchObject({
    resultStatus: 'rolled-back',
    rollbackState: 'completed',
    parentOverrideState: 'cancel-supported',
  });
  expect(auditCustody.claimBoundary).toContain('not production anti-tamper hardening');
  expect(rollbackGate).toMatchObject({
    readinessState: 'manual-required',
    resultStatus: 'unavailable',
    rollbackState: 'unavailable',
  });
});

it('keeps browser truth boundaries explicit', () => {
  const managedExactUrl = entryFor(V08OsAdapterProductProofSurface.ManagedBrowserExactUrl);
  const unmanagedExactEvidence = entryFor(V08OsAdapterProductProofSurface.UnmanagedBrowserExactEvidence);
  const unmanagedProcessOnly = entryFor(V08OsAdapterProductProofSurface.UnmanagedBrowserProcessOnly);

  expect(managedExactUrl.exactUrlClaimed).toBe(false);
  expect(managedExactUrl.claimBoundary).toContain('managed browser boundary');
  expect(unmanagedProcessOnly.claimBoundary).toContain('process-only');
  expect(unmanagedExactEvidence).toMatchObject({
    readinessState: 'not-claimed',
    proofLevel: 'not-proved',
    runtimeOwner: 'not-implemented',
    resultStatus: 'no-op',
    exactUrlClaimed: false,
  });
});

it('rejects invalid broad-blocking or exact-url upgrades', () => {
  const broadAppBlocking = entryFor(V08OsAdapterProductProofSurface.BroadAppBlocking);
  const unmanagedExactEvidence = entryFor(V08OsAdapterProductProofSurface.UnmanagedBrowserExactEvidence);

  expect(() =>
    V08OsAdapterProductProofEntrySchema.parse({
      ...broadAppBlocking,
      proofEntryId: 'invalid-broad-upgrade',
      broadBlockingClaimed: true,
    })
  ).toThrow();
  expect(() =>
    V08OsAdapterProductProofEntrySchema.parse({
      ...unmanagedExactEvidence,
      proofEntryId: 'invalid-exact-url-upgrade',
      exactUrlClaimed: true,
      claimUpgradeAllowed: true,
    })
  ).toThrow();
});

function entryFor(surface: string) {
  const entry = V08OsAdapterProductProofReadModel.entries.find((candidate) => candidate.surface === surface);
  if (entry === undefined) {
    throw new Error(`Missing V0.8 OS-adapter proof entry: ${surface}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
