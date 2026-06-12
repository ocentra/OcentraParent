import { describe, expect, it } from 'vitest';
import { ProductionSupportStatusBackendDurableQueueRuntimeReadModel } from '../../src/production-support-status-backend-durable-queue-runtime-read-model';
import {
  RequiredTrackingProductionDurableWorkerArtifactRefs,
  RequiredTrackingProductionDurableWorkersReadinessBlockers,
  TrackingProductionDurableWorkersReadinessBlockerProofSchema,
  buildTrackingProductionDurableWorkersReadinessBlockerProof,
} from '../../src/tracking-production-durable-workers-readiness-blocker-proof';

const generatedAt = '2026-06-07T22:15:00.000Z';
const proofId = 'tracking-production-durable-workers-readiness-blocker-proof';
const sourceProofRefs = [
  'packages/parent-domain/src/production-support-status-backend-durable-queue-runtime-proof.ts',
  'packages/parent-domain/src/production-support-status-backend-durable-queue-runtime-read-model.ts',
  'test-results/tracking-provider-runtime-readiness-blocker-proof/proof.json',
  'test-results/tracking-escalation-runtime-readiness-blocker-proof/proof.json',
  'test-results/tracking-retention-durable-settings-proof/proof.json',
];

describe('tracking production durable workers readiness blocker proof', () => {
  it('aggregates production support durable queue context into tracking production blockers', () => {
    const proof = buildProof();

    expect(proof.productionSupportDurableQueueRows).toBeGreaterThan(0);
    expect(proof.productionSupportManualClaimCount).toBeGreaterThan(0);
    expect(proof.requiredTrackingWorkerArtifactCount).toBe(RequiredTrackingProductionDurableWorkerArtifactRefs.length);
    expect(proof.presentTrackingWorkerArtifactCount).toBe(0);
    expect(proof.missingTrackingWorkerArtifactCount).toBe(RequiredTrackingProductionDurableWorkerArtifactRefs.length);
    expect(proof.requiredTrackingWorkerArtifactRefs).toEqual([...RequiredTrackingProductionDurableWorkerArtifactRefs]);
    expect(proof.presentTrackingWorkerArtifactRefs).toEqual([]);
    expect(proof.missingTrackingWorkerArtifactRefs).toEqual([...RequiredTrackingProductionDurableWorkerArtifactRefs]);
    expect(proof.blockers).toHaveLength(RequiredTrackingProductionDurableWorkersReadinessBlockers.length);
    expect(proof.productClaims.productionSupportBoundaryObserved).toBe(true);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('keeps every tracking production blocker tied to source and missing worker artifact refs', () => {
    const proof = buildProof();

    for (const row of proof.blockers) {
      expect(row.sourceProofRefs).toEqual(sourceProofRefs);
      expect(row.productionSupportBoundaryRefs).toContain('production-support-status-backend-execution-queue-proof');
      expect(row.blockingArtifactRefs).toEqual([...RequiredTrackingProductionDurableWorkerArtifactRefs]);
      expect(row.productionWorkersClaimed).toBe(false);
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('rejects tracking production proofs that overclaim worker readiness', () => {
    const proof = buildProof();
    const invalid = TrackingProductionDurableWorkersReadinessBlockerProofSchema.safeParse({
      ...proof,
      productClaims: {
        ...proof.productClaims,
        trackingLocationUploadWorkerClaimed: true,
      },
    });

    expect(invalid.success).toBe(false);
  });
});

function buildProof() {
  return buildTrackingProductionDurableWorkersReadinessBlockerProof(
    {
      generatedAt,
      proofId,
      sourceProofRefs,
      requiredTrackingWorkerArtifactRefs: RequiredTrackingProductionDurableWorkerArtifactRefs,
    },
    ProductionSupportStatusBackendDurableQueueRuntimeReadModel
  );
}
