import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingProductionWorkerRuntimeArtifactPlan,
  buildTrackingProductionWorkerRuntimeArtifactGateProof,
} from '../../src/tracking-production-worker-runtime-artifact-gate-proof';
import {
  TrackingProductionWorkerRuntimePreflightRowSchema,
  buildTrackingProductionWorkerRuntimePreflightProof,
} from '../../src/tracking-production-worker-runtime-preflight-proof';

const GeneratedAt = '2026-06-08T13:10:00.000Z';

describe('tracking production worker runtime preflight proof', () => {
  it('derives one manual-required row for every missing production worker artifact', () => {
    const gateProof = buildTrackingProductionWorkerRuntimeArtifactGateProof(GeneratedAt, { presentArtifacts: [] });
    const proof = buildTrackingProductionWorkerRuntimePreflightProof(GeneratedAt, gateProof);

    expect(proof.rows.map((row) => row.sourceMissingArtifactRef)).toEqual([
      ...RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts,
    ]);
    expect(proof.summary.rowCount).toBe(8);
    expect(proof.summary.manualRequiredRowCount).toBe(8);
    expect(proof.summary.requiredArtifactCount).toBe(8);
    expect(proof.summary.presentArtifactCount).toBe(0);
    expect(proof.summary.missingArtifactCount).toBe(8);
    expect(proof.summary.productReadyRowCount).toBe(0);
    expect(Object.values(proof.productClaims).every((claim) => claim === false)).toBe(true);
  });

  it('rejects preflight generation once production artifacts are all present', () => {
    const gateProof = buildTrackingProductionWorkerRuntimeArtifactGateProof(GeneratedAt, {
      presentArtifacts: RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts,
    });

    expect(() => buildTrackingProductionWorkerRuntimePreflightProof(GeneratedAt, gateProof)).toThrow(
      'Production worker runtime preflight requires missing artifact'
    );
  });

  it('rejects rows that claim production worker runtime execution', () => {
    const gateProof = buildTrackingProductionWorkerRuntimeArtifactGateProof(GeneratedAt, { presentArtifacts: [] });
    const [row] = buildTrackingProductionWorkerRuntimePreflightProof(GeneratedAt, gateProof).rows;

    expect(
      TrackingProductionWorkerRuntimePreflightRowSchema.safeParse({
        ...row,
        locationUploadWorkerRuntimeClaimed: true,
      }).success
    ).toBe(false);
  });
});
