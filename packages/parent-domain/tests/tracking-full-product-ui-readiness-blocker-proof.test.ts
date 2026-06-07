import { describe, expect, it } from 'vitest';
import {
  RequiredTrackingFullProductUiReadinessBlockers,
  TrackingFullProductUiReadinessBlockerProofSchema,
  buildTrackingFullProductUiReadinessBlockerProof,
} from '../src/tracking-full-product-ui-readiness-blocker-proof';
import { buildTrackingChildRuntimeArtifactGateProof } from '../src/tracking-child-runtime-artifact-gate-proof';

const generatedAt = '2026-06-07T22:05:00.000Z';
const sourceProofRefs = [
  'test-results/tracking-hosted-ui-artifact-inventory-proof/proof.json',
  'test-results/tracking-child-runtime-artifact-gate-proof/proof.json',
];
const hostedScreenshotRefs = [
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-live-summary.png',
  'output/tracking-plan-proof/30-parent-and-child-ui-ux-surfaces/11-ui-snapshots/hosted-policy-tracking-child-runtime-ui.png',
];
const hostedAssertionRefs = ['visible-heading', 'child-runtime-hosted-only-boundary-visible'];
const fullProductUiArtifactRefs = [
  'output/tracking-plan-proof/product-parent-child-ui-runtime/01-parent-overview-runtime.png',
  'output/tracking-plan-proof/product-parent-child-ui-runtime/02-parent-device-detail-runtime.png',
];

describe('tracking full product UI readiness blocker proof', () => {
  it('turns hosted route evidence and child-runtime artifact gaps into product UI blockers', () => {
    const proof = buildProof();

    expect(proof.blockers).toHaveLength(RequiredTrackingFullProductUiReadinessBlockers.length);
    expect(proof.hostedScreenshotRefs).toEqual(hostedScreenshotRefs);
    expect(proof.hostedAssertionRefs).toEqual(hostedAssertionRefs);
    expect(proof.childRuntimeArtifactRows).toBe(1);
    expect(proof.missingChildRuntimeArtifactCount).toBeGreaterThan(0);
    expect(proof.missingFullProductUiArtifactCount).toBeGreaterThan(fullProductUiArtifactRefs.length);
    expect(proof.productClaims.hostedRouteOnlyClaimed).toBe(true);
    expect(proof.productClaims.productClaimReady).toBe(false);
  });

  it('keeps every blocker tied to hosted evidence and missing product UI artifacts', () => {
    const proof = buildProof();

    for (const row of proof.blockers) {
      expect(row.sourceProofRefs).toEqual(sourceProofRefs);
      expect(row.hostedUiArtifactRefs).toEqual([...hostedScreenshotRefs, ...hostedAssertionRefs]);
      expect(row.blockingArtifactRefs).toContain(
        'output/tracking-plan-proof/child-device-runtime-execution/00-run-metadata.json'
      );
      expect(row.fullProductUiClaimed).toBe(false);
      expect(row.productClaimReady).toBe(false);
    }
  });

  it('rejects product-ready claims', () => {
    const proof = buildProof();
    const invalid = TrackingFullProductUiReadinessBlockerProofSchema.safeParse({
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
  const childRuntimeGate = buildTrackingChildRuntimeArtifactGateProof(generatedAt, { presentArtifacts: [] });

  return buildTrackingFullProductUiReadinessBlockerProof(
    {
      generatedAt,
      proofId: 'tracking-full-product-ui-readiness-blocker-proof',
      sourceProofRefs,
      hostedScreenshotRefs,
      hostedAssertionRefs,
      fullProductUiArtifactRefs,
    },
    childRuntimeGate
  );
}
