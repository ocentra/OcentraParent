import { describe, expect, it } from 'vitest';
import {
  EvidenceRef,
  LocationEvidence,
  NearbyPlaceEvidence,
  RetentionPolicy,
  trackingReadModelSample,
} from './tracking-fixtures';
import {
  TrackingReadModelSchema,
  TrackingRetentionPolicySchema,
  applyTrackingRetentionDelete,
  applyTrackingRetentionExport,
  evaluateTrackingEvidenceQualityGate,
} from '../../src/tracking';

describe('tracking evidence quality gate', () => {
  it('passes when read model rows and retention proofs carry source evidence', () => {
    const readModel = TrackingReadModelSchema.parse(trackingReadModelSample());
    const gate = evaluateTrackingEvidenceQualityGate({
      readModel,
      retentionDeleteProof: applyTrackingRetentionDelete({
        readModel,
        generatedAt: '2026-06-03T03:00:00.000Z',
        deletedEvidenceIds: [LocationEvidence.evidenceId],
      }),
      retentionExportProof: applyTrackingRetentionExport({
        readModel,
        generatedAt: '2026-06-03T03:05:00.000Z',
        policy: TrackingRetentionPolicySchema.parse(RetentionPolicy),
      }),
    });

    expect(gate.passed).toBe(true);
    expect(gate.missingGates).toHaveLength(0);
    expect(gate.locationEvidenceReferenceCount).toBeGreaterThanOrEqual(2);
    expect(gate.geofenceTransitionCount).toBe(1);
    expect(gate.nearbyPlaceResultCount).toBe(1);
    expect(gate.retentionDeleteBeforeLocationRows).toBe(1);
    expect(gate.retentionDeleteAfterLocationRows).toBe(0);
    expect(gate.retentionExportedLocationRows).toBe(1);
  });

  it('reports missing gates without weakening the underlying parsers', () => {
    const readModel = TrackingReadModelSchema.parse({
      ...trackingReadModelSample(),
      locationRows: [{ ...LocationEvidence, evidence: [] }],
      nearbyPlaceRows: [{ ...NearbyPlaceEvidence, distanceMeters: null, evidence: [EvidenceRef] }],
      timeline: [],
      returned: 0,
    });
    const gate = evaluateTrackingEvidenceQualityGate({
      readModel,
      retentionDeleteProof: applyTrackingRetentionDelete({
        readModel,
        generatedAt: '2026-06-03T03:00:00.000Z',
        deletedEvidenceIds: [],
      }),
      retentionExportProof: applyTrackingRetentionExport({
        readModel,
        generatedAt: '2026-06-03T03:05:00.000Z',
        policy: TrackingRetentionPolicySchema.parse({ ...RetentionPolicy, exportAllowed: false }),
      }),
    });

    expect(gate.passed).toBe(false);
    expect(gate.missingGates).toContain('location-ui-evidence-refs');
    expect(gate.missingGates).toContain('nearby-place-provider-context');
    expect(gate.missingGates).toContain('retention-delete-before-after-proof');
    expect(gate.missingGates).toContain('retention-export-before-after-proof');
  });
});
