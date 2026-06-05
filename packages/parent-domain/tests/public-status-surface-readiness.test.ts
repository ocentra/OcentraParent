import { describe, expect, it } from 'vitest';
import {
  PublicStatusSurfaceReadinessProofSchema,
  PublicStatusSurfaceReadinessReadModel,
  PublicStatusSurfaceReadinessRowSchema,
  summarizePublicStatusSurfaceReadinessRows,
} from '../src/public-status-surface-readiness';

describe('public status surface readiness', () => {
  acceptsPublicStatusSurfaceReadinessRows();
  rejectsSensitivePublicSurfaceData();
  rejectsIncompleteSurfaceCoverage();
  preservesRuntimeAndCustodyNonClaims();
});

function acceptsPublicStatusSurfaceReadinessRows(): void {
  it('accepts public website download account and support status readiness rows without runtime claims', () => {
    const proof = PublicStatusSurfaceReadinessProofSchema.parse(PublicStatusSurfaceReadinessReadModel);

    expect(summarizePublicStatusSurfaceReadinessRows(proof.rows)).toEqual({
      'family-public-site': 1,
      'public-download': 1,
      'release-status': 1,
      'update-status': 1,
      'account-status': 1,
      'subscription-status': 1,
      'support-status': 1,
    });
    expect(proof.publicWebsiteRuntimeClaim).toBe('not-implemented');
    expect(proof.accountBackendRuntimeClaim).toBe('backend-required');
    expect(proof.billingProviderRuntimeClaim).toBe('not-implemented');
    expect(proof.supportBackendUploadClaim).toBe('manual-required');
    expect(proof.productionSlaClaim).toBe('not-implemented');
    expect(proof.legalExecutionClaim).toBe('manual-required');
    expect(proof.remoteSupportSessionClaim).toBe('not-implemented');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsSensitivePublicSurfaceData(): void {
  it('rejects rows that allow child activity or omit provider secret exclusion', () => {
    const accountStatus = requiredRow('account-status');

    expect(
      PublicStatusSurfaceReadinessRowSchema.safeParse({
        ...accountStatus,
        supportSafeDataClasses: [...accountStatus.supportSafeDataClasses, 'child-activity-evidence'],
      }).success
    ).toBe(false);
    expect(
      PublicStatusSurfaceReadinessRowSchema.safeParse({
        ...accountStatus,
        forbiddenDataClasses: accountStatus.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteSurfaceCoverage(): void {
  it('rejects proof that omits a required public status surface', () => {
    expect(
      PublicStatusSurfaceReadinessProofSchema.safeParse({
        ...PublicStatusSurfaceReadinessReadModel,
        rows: PublicStatusSurfaceReadinessReadModel.rows.filter((row) => row.surface !== 'support-status'),
      }).success
    ).toBe(false);
  });
}

function preservesRuntimeAndCustodyNonClaims(): void {
  it('rejects proof that drops account backend or custody non-claims', () => {
    expect(
      PublicStatusSurfaceReadinessProofSchema.safeParse({
        ...PublicStatusSurfaceReadinessReadModel,
        nonClaims: PublicStatusSurfaceReadinessReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-account-backend-runtime'
        ),
      }).success
    ).toBe(false);
    expect(
      PublicStatusSurfaceReadinessProofSchema.safeParse({
        ...PublicStatusSurfaceReadinessReadModel,
        childActivityCustodyClaim: 'backend-required',
      }).success
    ).toBe(false);
  });
}

function requiredRow(surface: 'account-status'): (typeof PublicStatusSurfaceReadinessReadModel.rows)[number] {
  const row = PublicStatusSurfaceReadinessReadModel.rows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing public status surface readiness row: ${surface}`);
  }
  return row;
}
