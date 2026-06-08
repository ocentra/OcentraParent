import { describe, expect, it } from 'vitest';
import { buildTrackingAuthorityEnrollmentManualRequiredProof } from '../src/tracking-authority-enrollment-manual-required-proof';
import {
  buildTrackingAuthorityRuntimeReadinessBlockerProof,
  RequiredTrackingAuthorityRuntimeReadinessBlockers,
} from '../src/tracking-authority-runtime-readiness-blocker-proof';
import {
  buildTrackingAuthorityRuntimeArtifactGateProof,
  TrackingAuthorityRuntimeArtifactGateRowSchema,
} from '../src/tracking-authority-runtime-artifact-gate-proof';

const generatedAt = '2026-06-08T00:15:00.000Z';
const sourceProofRefs = ['test-results/tracking-authority-enrollment-manual-required-proof/proof.json'];

describe('tracking authority runtime artifact gate proof', () => {
  it('keeps authority runtime manual-required while enrolled-device artifacts are missing', () => {
    const authorityReadinessProof = buildAuthorityReadinessProof();
    const proof = buildTrackingAuthorityRuntimeArtifactGateProof(generatedAt, authorityReadinessProof, {
      presentArtifacts: [],
    });

    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].status).toBe('manual-required');
    expect(proof.rows[0].requiredProofTier).toBe('P4_PHYSICAL_DEVICE');
    expect(proof.rows[0].currentProofTier).toBe('P0_CONTRACT');
    expect(proof.rows[0].sourceRuntimeReadinessBlockers).toEqual([
      ...RequiredTrackingAuthorityRuntimeReadinessBlockers,
    ]);
    expect(proof.rows[0].requiredArtifacts).toContain('tracking-authority-android-device-owner-device-identity-proof');
    expect(proof.rows[0].missingArtifacts).toEqual(proof.rows[0].requiredArtifacts);
    expect(proof.productClaims.authorityEnrollmentClaimed).toBe(false);
    expect(proof.productClaims.hardControlRuntimeClaimed).toBe(false);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('marks artifact-set-present only when every authority runtime artifact exists', () => {
    const authorityReadinessProof = buildAuthorityReadinessProof();
    const requiredArtifacts = [...new Set(authorityReadinessProof.blockers.flatMap((row) => row.blockingEvidenceRefs))];
    const proof = buildTrackingAuthorityRuntimeArtifactGateProof(generatedAt, authorityReadinessProof, {
      presentArtifacts: requiredArtifacts,
    });

    expect(proof.rows[0].status).toBe('artifact-set-present');
    expect(proof.rows[0].authorityRuntimeArtifactSetComplete).toBe(true);
    expect(proof.rows[0].missingArtifacts).toEqual([]);
    expect(proof.productClaims.parentVisibleAuthorityStatusClaimed).toBe(false);
    expect(proof.productClaims.physicalDeviceProofClaimed).toBe(false);
    expect(proof.productClaims.productionWorkerClaimed).toBe(false);
  });

  it('rejects rows that claim authority runtime without product proof', () => {
    const invalid = TrackingAuthorityRuntimeArtifactGateRowSchema.safeParse({
      schemaVersion: 'v0.5-tracking',
      rowId: 'tracking-authority-runtime-artifacts-invalid',
      generatedAt,
      proofRoot: 'output/tracking-plan-proof',
      requiredProofTier: 'P4_PHYSICAL_DEVICE',
      currentProofTier: 'P0_CONTRACT',
      status: 'manual-required',
      sourceRuntimeReadinessProofRef:
        'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/55-authority-runtime-readiness-blocker-proof.json',
      sourceRuntimeReadinessBlockers: [...RequiredTrackingAuthorityRuntimeReadinessBlockers],
      requiredArtifacts: ['tracking-authority-android-device-owner-device-identity-proof'],
      presentArtifacts: [],
      missingArtifacts: ['tracking-authority-android-device-owner-device-identity-proof'],
      auditRefs: ['tracking-authority-runtime-artifacts-invalid-audit'],
      authorityRuntimeArtifactSetComplete: false,
      authorityEnrollmentClaimed: true,
      hardControlRuntimeClaimed: false,
      parentVisibleAuthorityStatusClaimed: false,
      physicalDeviceProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    });

    expect(invalid.success).toBe(false);
  });
});

function buildAuthorityReadinessProof() {
  return buildTrackingAuthorityRuntimeReadinessBlockerProof(
    {
      generatedAt,
      proofId: 'tracking-authority-runtime-readiness-blocker-proof',
      sourceProofRefs,
    },
    buildTrackingAuthorityEnrollmentManualRequiredProof(generatedAt)
  );
}
