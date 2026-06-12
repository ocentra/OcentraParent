import { describe, expect, it } from 'vitest';
import {
  ProductionReleasePublicStatusProofReadModel,
  ProductionReleasePublicStatusProofSchema,
  ProductionReleasePublicSurfaceStatusSchema,
  summarizeProductionReleasePublicStatusSurfaces,
} from '../../src/production-release-public-status-proof';

describe('production release public status proof', () => {
  acceptsPublicReleaseStatusReadinessRows();
  rejectsPublicRuntimeOverclaims();
  rejectsSensitiveDataCustody();
  rejectsIncompletePublicSurfaceCoverage();
});

function acceptsPublicReleaseStatusReadinessRows(): void {
  it('accepts route and manual public download account status rows without production claims', () => {
    const proof = ProductionReleasePublicStatusProofSchema.parse(ProductionReleasePublicStatusProofReadModel);

    expect(summarizeProductionReleasePublicStatusSurfaces(proof.surfaces)).toEqual({
      'public-download': 1,
      'release-status': 1,
      'update-status': 1,
      'account-status': 1,
      'subscription-status': 1,
      'support-status': 1,
    });
    expect(proof.publicHostState).toBe('not-implemented');
    expect(proof.productionPublishingState).toBe('production-promotion-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
    expect(proof.publicSupportRuntimeClaim).toBe('not-implemented');
    expect(proof.nonClaims).toEqual([
      'no-public-website-runtime',
      'no-account-backend',
      'no-billing-provider-runtime',
      'no-production-publishing',
      'no-signing-store-proof',
      'no-updater-execution',
      'no-support-backend-upload',
      'no-child-activity-custody',
    ]);
  });
}

function rejectsPublicRuntimeOverclaims(): void {
  it('rejects public account status rows that claim backend implementation', () => {
    const accountStatus = requiredSurface('account-status');

    expect(
      ProductionReleasePublicSurfaceStatusSchema.safeParse({
        ...accountStatus,
        backendRuntimeState: 'implemented',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveDataCustody(): void {
  it('rejects public surfaces that allow child activity or omit forbidden custody classes', () => {
    const releaseStatus = requiredSurface('release-status');

    expect(
      ProductionReleasePublicSurfaceStatusSchema.safeParse({
        ...releaseStatus,
        allowedDataClasses: [...releaseStatus.allowedDataClasses, 'child-activity-evidence'],
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicSurfaceStatusSchema.safeParse({
        ...releaseStatus,
        forbiddenDataClasses: releaseStatus.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompletePublicSurfaceCoverage(): void {
  it('rejects proof that omits a required public support status surface', () => {
    expect(
      ProductionReleasePublicStatusProofSchema.safeParse({
        ...ProductionReleasePublicStatusProofReadModel,
        surfaces: ProductionReleasePublicStatusProofReadModel.surfaces.filter(
          (surface) => surface.surface !== 'support-status'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredSurface(surface: 'account-status' | 'release-status') {
  const row = ProductionReleasePublicStatusProofReadModel.surfaces.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing public surface row: ${surface}`);
  }
  return row;
}
