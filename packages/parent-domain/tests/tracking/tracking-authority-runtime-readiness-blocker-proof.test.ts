import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingAuthorityRuntimeReadinessBlockers,
  TrackingAuthorityRuntimeReadinessBlockerProofSchema,
  buildTrackingAuthorityRuntimeReadinessBlockerProof,
} from '../../src/tracking-authority-runtime-readiness-blocker-proof';
import { buildTrackingAuthorityEnrollmentManualRequiredProof } from '../../src/tracking-authority-enrollment-manual-required-proof';

const generatedAt = '2026-06-07T21:45:00.000Z';
const sourceProofRefs = ['test-results/tracking-authority-enrollment-manual-required-proof/proof.json'];

describe('tracking authority runtime readiness blocker proof', () => {
  it('turns authority enrollment evidence requirements into runtime blocker rows', () => {
    const proof = buildProof();

    expect(proof.authorityEnrollmentRows).toBe(5);
    expect(proof.authorityRequiredRows).toBe(4);
    expect(proof.manualRequiredRows).toBe(1);
    expect(proof.missingAuthorityRuntimeEvidenceCount).toBeGreaterThan(0);
    expect(proof.blockers).toHaveLength(RequiredTrackingAuthorityRuntimeReadinessBlockers.length);
    expect(proof.blockers.map((row) => row.blockerId)).toEqual(RequiredTrackingAuthorityRuntimeReadinessBlockers);
  });

  it('keeps every blocker tied to source authority rows and missing evidence', () => {
    const proof = buildProof();

    expect(proof.sourceProofRefs).toEqual(sourceProofRefs);
    for (const row of proof.blockers) {
      expect(row.sourceAuthorityRows).toHaveLength(5);
      expect(row.blockingEvidenceRefs).toContain('tracking-authority-android-device-owner-device-identity-proof');
      expect(row.authorityEnrollmentClaimed).toBe(false);
      expect(row.hardControlRuntimeClaimed).toBe(false);
      expect(row.physicalDeviceClaimed).toBe(false);
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('rejects proofs that claim product-ready authority behavior', () => {
    const proof = buildProof();
    const invalid = TrackingAuthorityRuntimeReadinessBlockerProofSchema.safeParse({
      ...proof,
      productClaims: {
        ...proof.productClaims,
        productClaimReady: true,
      },
    });

    expect(invalid.success).toBe(false);
  });
});

function buildProof() {
  return buildTrackingAuthorityRuntimeReadinessBlockerProof(
    {
      generatedAt,
      proofId: 'tracking-authority-runtime-readiness-blocker-proof',
      sourceProofRefs,
    },
    buildTrackingAuthorityEnrollmentManualRequiredProof(generatedAt)
  );
}
