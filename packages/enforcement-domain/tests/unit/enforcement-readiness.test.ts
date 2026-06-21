import { describe, expect, it } from 'vitest';
import {
  EnforcementBroadAdapterCapability,
  EnforcementBroadAdapterReadinessEntrySchema,
  EnforcementBroadOsAdapterReadinessMatrixSchema,
  EnforcementReadinessState,
  V08BroadOsAdapterReadinessMatrix,
} from '@ocentra-parent/schema-domain/enforcement-readiness';

describe('broad OS adapter readiness contracts', () => {
  it('captures the V0.8 broad adapter readiness matrix without upgrading product claims', () => {
    const matrix = EnforcementBroadOsAdapterReadinessMatrixSchema.parse(V08BroadOsAdapterReadinessMatrix);
    const stateCounts = countBy(matrix.entries.map((entry) => entry.readinessState));

    expect(matrix.entries).toHaveLength(9);
    expect(stateCounts).toEqual({
      implemented: 3,
      'manual-required': 5,
      'not-claimed': 1,
    });
    expect(new Set(matrix.entries.map((entry) => entry.readinessId)).size).toBe(matrix.entries.length);
    expect(entryFor(EnforcementBroadAdapterCapability.BroadAppBlocking).runtimeOwner).toBe('manual-proof');
    expect(entryFor(EnforcementBroadAdapterCapability.NetworkDomainBlocking).capabilityState).toBe('manual-required');
    expect(entryFor(EnforcementBroadAdapterCapability.ManagedBrowserServiceCommand).claimBoundary).toContain(
      'not exact URL enforcement proof'
    );
    expect(entryFor(EnforcementBroadAdapterCapability.UnmanagedBrowserExactEvidence)).toMatchObject({
      readinessState: EnforcementReadinessState.NotClaimed,
      proofLevel: 'not-proved',
      runtimeOwner: 'not-implemented',
    });
  });

  it('rejects implemented broad app blocking without a supported capability state', () => {
    const broadApp = entryFor(EnforcementBroadAdapterCapability.BroadAppBlocking);

    expect(() =>
      EnforcementBroadAdapterReadinessEntrySchema.parse({
        ...broadApp,
        readinessId: 'invalid-broad-app-claim',
        readinessState: 'implemented',
        proofLevel: 'real-service-proof',
      })
    ).toThrow();
  });

  it('rejects not-claimed exact browser evidence when artifacts are missing', () => {
    const exactEvidence = entryFor(EnforcementBroadAdapterCapability.UnmanagedBrowserExactEvidence);

    expect(() =>
      EnforcementBroadAdapterReadinessEntrySchema.parse({
        ...exactEvidence,
        readinessId: 'invalid-unmanaged-evidence-claim',
        requiredArtifacts: [],
      })
    ).toThrow();
  });
});

function entryFor(capability: string) {
  const entry = V08BroadOsAdapterReadinessMatrix.entries.find((candidate) => candidate.capability === capability);
  if (entry === undefined) {
    throw new Error(`Missing readiness entry: ${capability}`);
  }
  return entry;
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
