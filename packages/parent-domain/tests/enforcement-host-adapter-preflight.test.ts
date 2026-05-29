import { describe, expect, it } from 'vitest';
import {
  EnforcementHostAdapterPreflightEntrySchema,
  EnforcementHostAdapterPreflightGate,
  EnforcementHostAdapterPreflightMatrixSchema,
  EnforcementHostAdapterPreflightStatus,
  V08HostAdapterProofPreflightMatrix,
} from '../src/enforcement-host-adapter-preflight';
import { EnforcementBroadAdapterCapability, V08BroadOsAdapterReadinessMatrix } from '../src/enforcement-readiness';

describe('host adapter proof preflight contracts', () => {
  it('captures manual preflight gates without upgrading broad adapter claims', () => {
    const matrix = EnforcementHostAdapterPreflightMatrixSchema.parse(V08HostAdapterProofPreflightMatrix);
    const statusCounts = countBy(matrix.entries.map((entry) => entry.preflightStatus));
    const claimCounts = countBy(matrix.entries.map((entry) => entry.productClaimState));
    const gateCounts = countBy(matrix.entries.map((entry) => entry.preflightGate));

    expect(matrix.entries).toHaveLength(6);
    expect(statusCounts).toEqual({
      'blocked-by-missing-artifact': 5,
      'not-claimable-from-current-proof': 1,
    });
    expect(claimCounts).toEqual({
      'manual-required': 5,
      'not-claimed': 1,
    });
    expect(gateCounts).toEqual({
      'process-package-identity': 1,
      'host-network-filter': 1,
      'managed-browser-boundary': 2,
      'explicit-browser-integration': 1,
      'rollback-anti-tamper': 1,
    });
    expect(new Set(matrix.entries.map((entry) => entry.preflightId)).size).toBe(matrix.entries.length);
  });

  it('links every preflight entry to the readiness matrix and preserves source boundaries', () => {
    for (const entry of V08HostAdapterProofPreflightMatrix.entries) {
      const readiness = readinessEntryFor(entry.readinessId);

      expect(entry.capability).toBe(readiness.capability);
      expect(entry.productClaimState === 'manual-required' || entry.productClaimState === 'not-claimed').toBe(true);
      expect(entry.requiredEvidenceArtifacts.length).toBeGreaterThanOrEqual(3);
      expect(entry.manualProofSteps.length).toBeGreaterThanOrEqual(2);
      expect(entry.unsafeUpgradeExamples.length).toBeGreaterThanOrEqual(2);
    }

    expect(entryFor(EnforcementBroadAdapterCapability.BroadAppBlocking).preflightGate).toBe(
      EnforcementHostAdapterPreflightGate.ProcessPackageIdentity
    );
    expect(entryFor(EnforcementBroadAdapterCapability.NetworkDomainBlocking).claimBoundary).toContain(
      'cannot be inferred from metadata'
    );
    expect(entryFor(EnforcementBroadAdapterCapability.ManagedBrowserExactUrlControl).prerequisite).toContain(
      'Active URL'
    );
    expect(entryFor(EnforcementBroadAdapterCapability.UnmanagedBrowserExactEvidence).productClaimState).toBe(
      'not-claimed'
    );
  });

  it('rejects broad preflight entries that omit required manual artifacts', () => {
    const broadApp = entryFor(EnforcementBroadAdapterCapability.BroadAppBlocking);

    expect(() =>
      EnforcementHostAdapterPreflightEntrySchema.parse({
        ...broadApp,
        preflightId: 'invalid-broad-app-preflight',
        requiredEvidenceArtifacts: [],
      })
    ).toThrow();
  });

  it('rejects unmanaged exact browser evidence when it is made claimable from current proof', () => {
    const unmanagedExact = entryFor(EnforcementBroadAdapterCapability.UnmanagedBrowserExactEvidence);

    expect(() =>
      EnforcementHostAdapterPreflightEntrySchema.parse({
        ...unmanagedExact,
        preflightId: 'invalid-unmanaged-exact-preflight',
        preflightStatus: EnforcementHostAdapterPreflightStatus.BlockedByMissingArtifact,
        productClaimState: 'manual-required',
        proofLevel: 'manual-proof-required',
        runtimeOwner: 'manual-proof',
      })
    ).toThrow();
  });
});

function entryFor(capability: string) {
  const entry = V08HostAdapterProofPreflightMatrix.entries.find((candidate) => candidate.capability === capability);
  if (entry === undefined) {
    throw new Error(`Missing preflight entry: ${capability}`);
  }
  return entry;
}

function readinessEntryFor(readinessId: string) {
  const entry = V08BroadOsAdapterReadinessMatrix.entries.find((candidate) => candidate.readinessId === readinessId);
  if (entry === undefined) {
    throw new Error(`Missing readiness entry: ${readinessId}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
