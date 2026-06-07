import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingProductionWorkerRuntimeArtifactPlan,
  TrackingProductionWorkerRuntimeArtifactGateRowSchema,
  buildTrackingProductionWorkerRuntimeArtifactGateProof,
} from '../src/tracking-production-worker-runtime-artifact-gate-proof';

describe('tracking production worker runtime artifact gate proof', () => {
  it('keeps production worker runtime manual-required when artifacts are missing', () => {
    const proof = buildTrackingProductionWorkerRuntimeArtifactGateProof('2026-06-07T22:50:00.000Z', {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PRODUCTION_RUNTIME');
    expect(proof.rows[0].currentProofTier).toBe('P3_LOCAL_DEV_MACHINE');
    expect(proof.rows[0].productionWorkerArtifactSetComplete).toBe(false);
    expect(proof.rows[0].missingArtifacts).toEqual([
      ...RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts,
    ]);
    expect(proof.productClaims.locationUploadWorkerRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productionAuditDurableStorageClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('marks artifact-set-present only when every required production artifact is present', () => {
    const proof = buildTrackingProductionWorkerRuntimeArtifactGateProof('2026-06-07T22:50:00.000Z', {
      presentArtifacts: RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].productionWorkerArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.providerReceiptWorkerRuntimeClaimed).toBe(false);
    expect(proof.productClaims.childDeviceDeliveryWorkerRuntimeClaimed).toBe(false);
    expect(proof.productClaims.authorityStatusWorkerRuntimeClaimed).toBe(false);
  });

  it('rejects rows that claim production worker runtime without product proof', () => {
    const invalid = TrackingProductionWorkerRuntimeArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-production-worker-runtime-artifacts-invalid',
      generatedAt: '2026-06-07T22:50:00.000Z',
      proofRoot: RequiredTrackingProductionWorkerRuntimeArtifactPlan.proofRoot,
      requiredProofTier: 'P4_PRODUCTION_RUNTIME',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      requiredArtifacts: ['tracking-production/location-upload-worker-runtime.json'],
      presentArtifacts: [],
      missingArtifacts: ['tracking-production/location-upload-worker-runtime.json'],
      auditRefs: ['tracking-production-worker-runtime-artifacts-invalid-audit'],
      productionWorkerArtifactSetComplete: false,
      locationUploadWorkerRuntimeClaimed: true,
      retentionCleanupWorkerRuntimeClaimed: false,
      notificationOutboxWorkerRuntimeClaimed: false,
      escalationTimeoutWorkerRuntimeClaimed: false,
      providerReceiptWorkerRuntimeClaimed: false,
      childDeviceDeliveryWorkerRuntimeClaimed: false,
      authorityStatusWorkerRuntimeClaimed: false,
      productionAuditDurableStorageClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryReceiptRuntimeClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});
