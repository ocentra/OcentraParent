import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingProductReadinessClosureBlockers,
  RequiredTrackingProductReadinessClosureCoverageTags,
  TrackingProductReadinessClosureRowSchema,
  buildTrackingProductReadinessClosureProof,
} from '../src/tracking-product-readiness-closure-proof';

const GeneratedAt = '2026-06-07T16:30:00.000Z';

describe('tracking product readiness closure proof', () => {
  it('enumerates remaining product blockers while preserving local CI proof refs', () => {
    const proof = buildTrackingProductReadinessClosureProof(GeneratedAt, sourceProofs());

    expect(proof.sourceProofs.map((sourceProof) => sourceProof.coverageTag)).toEqual([
      ...RequiredTrackingProductReadinessClosureCoverageTags,
    ]);
    expect(proof.rows).toHaveLength(1);
    expect(proof.rows[0].remainingBlockers).toEqual([...RequiredTrackingProductReadinessClosureBlockers]);
    expect(proof.productClaims.localCiProofAccountingReady).toBe(true);
    expect(proof.productClaims.physicalAndroidBackgroundClaimed).toBe(false);
    expect(proof.productClaims.physicalIosBackgroundClaimed).toBe(false);
    expect(proof.productClaims.productReadyClaimed).toBe(false);
  });

  it('rejects product-ready overclaims', () => {
    const [row] = buildTrackingProductReadinessClosureProof(GeneratedAt, sourceProofs()).rows;

    expect(
      TrackingProductReadinessClosureRowSchema.safeParse({
        ...row,
        productReadyClaimed: true,
      }).success
    ).toBe(false);
  });

  it('rejects incomplete blocker accounting', () => {
    const [row] = buildTrackingProductReadinessClosureProof(GeneratedAt, sourceProofs()).rows;

    expect(
      TrackingProductReadinessClosureRowSchema.safeParse({
        ...row,
        remainingBlockers: RequiredTrackingProductReadinessClosureBlockers.slice(0, 2),
      }).success
    ).toBe(false);
  });
});

function sourceProofs() {
  return RequiredTrackingProductReadinessClosureCoverageTags.map((coverageTag) => ({
    coverageTag,
    proofRef: `output/tracking-plan-proof/${coverageTag}/proof.json`,
    status: 'proved',
    proofTier: 'P3_LOCAL_DEV_MACHINE',
  }));
}
