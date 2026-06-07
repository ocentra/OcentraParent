import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingRetentionRuntimeArtifactPlan,
  TrackingRetentionRuntimeArtifactGateRowSchema,
  buildTrackingRetentionRuntimeArtifactGateProof,
} from '../src/tracking-retention-runtime-artifact-gate-proof';

describe('tracking retention runtime artifact gate proof', () => {
  it('keeps retention runtime manual-required when required artifacts are missing', () => {
    const proof = buildTrackingRetentionRuntimeArtifactGateProof('2026-06-07T23:45:00.000Z', {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PRODUCTION_RUNTIME');
    expect(proof.rows[0].currentProofTier).toBe('P3_LOCAL_DEV_MACHINE');
    expect(proof.rows[0].sourceProductReadinessBlockers).toEqual([
      ...RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessBlockers,
    ]);
    expect(proof.rows[0].missingArtifacts).toEqual([...RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts]);
    expect(proof.productClaims.writableProductSettingsExecutionClaimed).toBe(false);
    expect(proof.productClaims.platformRuntimeRetentionEnforcementClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('marks artifact-set-present only when each retention runtime artifact exists', () => {
    const proof = buildTrackingRetentionRuntimeArtifactGateProof('2026-06-07T23:45:00.000Z', {
      presentArtifacts: RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].retentionRuntimeArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.notificationReceiptClaimed).toBe(false);
    expect(proof.productClaims.authorityProofClaimed).toBe(false);
    expect(proof.productClaims.productionWorkerClaimed).toBe(false);
  });

  it('rejects rows that claim retention runtime execution without product proof', () => {
    const invalid = TrackingRetentionRuntimeArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-retention-runtime-artifacts-invalid',
      generatedAt: '2026-06-07T23:45:00.000Z',
      proofRoot: RequiredTrackingRetentionRuntimeArtifactPlan.proofRoot,
      requiredProofTier: 'P4_PRODUCTION_RUNTIME',
      currentProofTier: 'P3_LOCAL_DEV_MACHINE',
      status: 'manual-required',
      sourceProductReadinessProofRef: RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessProofRef,
      sourceProductReadinessBlockers: [...RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessBlockers],
      requiredArtifacts: ['tracking-retention/product-settings-writable-execution.json'],
      presentArtifacts: [],
      missingArtifacts: ['tracking-retention/product-settings-writable-execution.json'],
      auditRefs: ['tracking-retention-runtime-artifacts-invalid-audit'],
      retentionRuntimeArtifactSetComplete: false,
      writableProductSettingsExecutionClaimed: true,
      platformRuntimeRetentionEnforcementClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});
