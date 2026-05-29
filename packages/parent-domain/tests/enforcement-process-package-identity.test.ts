import { describe, expect, it } from 'vitest';
import { EnforcementCapabilityState } from '../src/enforcement';
import { V08HostAdapterProofPreflightMatrix } from '../src/enforcement-host-adapter-preflight';
import {
  EnforcementProcessPackageBridgeState,
  EnforcementProcessPackageIdentityBridgeEntrySchema,
  EnforcementProcessPackageIdentityMatrixSchema,
  EnforcementProcessPackageProofPoint,
  V08ProcessPackageIdentityProofBridgeMatrix,
} from '../src/enforcement-process-package-identity';

describe('process and package identity proof bridge contracts', () => {
  it('captures Windows identity prerequisites without upgrading broad app enforcement', () => {
    const matrix = EnforcementProcessPackageIdentityMatrixSchema.parse(V08ProcessPackageIdentityProofBridgeMatrix);
    const stateCounts = countBy(matrix.entries.map((entry) => entry.bridgeState));
    const evidenceCounts = countBy(matrix.entries.map((entry) => entry.evidenceClass));

    expect(matrix.entries).toHaveLength(9);
    expect(stateCounts).toEqual({
      'manual-required': 7,
      unavailable: 1,
      'not-claimed': 1,
    });
    expect(evidenceCounts).toEqual({
      inventory: 2,
      process: 1,
      executable: 1,
      package: 2,
      'publisher-signature': 1,
      rollback: 1,
      audit: 1,
    });
    expect(new Set(matrix.entries.map((entry) => entry.bridgeId)).size).toBe(matrix.entries.length);
    expect(
      matrix.entries.every(
        (entry) => entry.capabilityState === 'manual-required' || entry.capabilityState === 'unavailable'
      )
    ).toBe(true);
  });

  it('links every bridge entry to the process-package preflight gate and preserves source truth', () => {
    const preflightIds = new Set(V08HostAdapterProofPreflightMatrix.entries.map((entry) => entry.preflightId));

    for (const entry of V08ProcessPackageIdentityProofBridgeMatrix.entries) {
      expect(entry.preflightGate).toBe('process-package-identity');
      expect(entry.preflightIds.every((preflightId) => preflightIds.has(preflightId))).toBe(true);
      expect(entry.requiredEvidenceArtifacts.length).toBeGreaterThanOrEqual(3);
      expect(entry.manualProofSteps.length).toBeGreaterThanOrEqual(2);
      expect(entry.acceptanceSignals.length).toBeGreaterThanOrEqual(2);
      expect(entry.unsafeUpgradeExamples.length).toBeGreaterThanOrEqual(2);
    }

    expect(entryFor(EnforcementProcessPackageProofPoint.InstalledAppInventory).hostEvidenceRequirement).toContain(
      'real Windows host source'
    );
    expect(entryFor(EnforcementProcessPackageProofPoint.PackageIdentity).hostEvidenceRequirement).toContain(
      'silently upgrading unknown apps'
    );
    expect(entryFor(EnforcementProcessPackageProofPoint.PublisherSignature).fallbackBehavior).toContain(
      'do not invent trust state'
    );
  });

  it('keeps unsupported identity and rollback states honest', () => {
    expect(entryFor(EnforcementProcessPackageProofPoint.UnsupportedIdentity)).toMatchObject({
      bridgeState: EnforcementProcessPackageBridgeState.Unavailable,
      capabilityState: EnforcementCapabilityState.Unavailable,
      proofLevel: 'manual-proof-required',
      runtimeOwner: 'manual-proof',
    });
    expect(entryFor(EnforcementProcessPackageProofPoint.RollbackReadiness)).toMatchObject({
      bridgeState: EnforcementProcessPackageBridgeState.NotClaimed,
      capabilityState: EnforcementCapabilityState.ManualRequired,
      proofLevel: 'not-proved',
      runtimeOwner: 'not-implemented',
    });
  });
});

describe('process and package identity proof bridge rejection cases', () => {
  it('rejects manual-required identity entries that omit required artifacts', () => {
    const inventory = entryFor(EnforcementProcessPackageProofPoint.InstalledAppInventory);

    expect(() =>
      EnforcementProcessPackageIdentityBridgeEntrySchema.parse({
        ...inventory,
        bridgeId: 'invalid-installed-inventory',
        requiredEvidenceArtifacts: [],
      })
    ).toThrow();
  });

  it('rejects rollback readiness when it is upgraded to manual proof without implementation', () => {
    const rollback = entryFor(EnforcementProcessPackageProofPoint.RollbackReadiness);

    expect(() =>
      EnforcementProcessPackageIdentityBridgeEntrySchema.parse({
        ...rollback,
        bridgeId: 'invalid-rollback-upgrade',
        bridgeState: 'manual-required',
        proofLevel: 'manual-proof-required',
        runtimeOwner: 'manual-proof',
      })
    ).toThrow();
  });
});

function entryFor(proofPoint: string) {
  const entry = V08ProcessPackageIdentityProofBridgeMatrix.entries.find(
    (candidate) => candidate.proofPoint === proofPoint
  );
  if (entry === undefined) {
    throw new Error(`Missing process/package bridge entry: ${proofPoint}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
