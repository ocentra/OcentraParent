import { describe, expect, it } from 'vitest';
import {
  ProductionReleasePublicRuntimeAdapterRowSchema,
  ProductionReleasePublicRuntimeHandoffProofSchema,
  ProductionReleasePublicRuntimeHandoffRowSchema,
  summarizeProductionReleasePublicRuntimeAdapters,
  summarizeProductionReleasePublicRuntimeHandoffs,
} from '../src/production-release-public-runtime-handoff';
import { ProductionReleasePublicRuntimeHandoffReadModel } from '../src/production-release-public-runtime-handoff-read-model';

describe('production release public runtime handoff', () => {
  acceptsPublicRuntimeHandoffReadinessRows();
  rejectsPublicRuntimeOverclaims();
  rejectsSensitiveDataCustody();
  rejectsIncompleteRuntimeHandoffCoverage();
});

function acceptsPublicRuntimeHandoffReadinessRows(): void {
  it('accepts public download account status runtime handoff rows without live runtime claims', () => {
    const proof = ProductionReleasePublicRuntimeHandoffProofSchema.parse(
      ProductionReleasePublicRuntimeHandoffReadModel
    );

    expect(summarizeProductionReleasePublicRuntimeHandoffs(proof.handoffRows)).toEqual({
      'public-download': 1,
      'release-status': 1,
      'update-status': 1,
      'account-status': 1,
      'subscription-status': 1,
      'support-status': 1,
    });
    expect(summarizeProductionReleasePublicRuntimeAdapters(proof.adapterRows)).toEqual({
      'public-website-runtime': 1,
      'download-status-backend': 1,
      'release-publishing-pipeline': 1,
      'updater-status-runtime': 1,
      'account-backend': 1,
      'billing-provider-runtime': 1,
      'support-backend-upload': 1,
    });
    expect(proof.publicWebsiteRuntimeClaim).toBe('not-implemented');
    expect(proof.accountBackendRuntimeClaim).toBe('backend-required');
    expect(proof.billingProviderRuntimeClaim).toBe('not-implemented');
    expect(proof.supportBackendUploadClaim).toBe('manual-required');
    expect(proof.productionPublishingState).toBe('production-promotion-required');
    expect(proof.signingStoreProofState).toBe('manual-required');
    expect(proof.updaterExecutionState).toBe('manual-required');
    expect(proof.childActivityCustodyClaim).toBe('not-implemented');
  });
}

function rejectsPublicRuntimeOverclaims(): void {
  it('rejects route/backend implementation and production execution claims', () => {
    const accountStatus = requiredHandoff('account-status');
    const billingProvider = requiredAdapter('billing-provider-runtime');

    expect(
      ProductionReleasePublicRuntimeHandoffRowSchema.safeParse({
        ...accountStatus,
        routeState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicRuntimeHandoffRowSchema.safeParse({
        ...accountStatus,
        backendAdapterState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicRuntimeAdapterRowSchema.safeParse({
        ...billingProvider,
        adapterState: 'implemented',
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicRuntimeAdapterRowSchema.safeParse({
        ...billingProvider,
        executionClaim: 'executed',
      }).success
    ).toBe(false);
  });
}

function rejectsSensitiveDataCustody(): void {
  it('rejects public handoff rows that allow child activity or omit provider secret exclusion', () => {
    const supportStatus = requiredHandoff('support-status');

    expect(
      ProductionReleasePublicRuntimeHandoffRowSchema.safeParse({
        ...supportStatus,
        supportSafeDataClasses: [...supportStatus.supportSafeDataClasses, 'child-activity-evidence'],
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicRuntimeHandoffRowSchema.safeParse({
        ...supportStatus,
        forbiddenDataClasses: supportStatus.forbiddenDataClasses.filter(
          (dataClass) => dataClass !== 'provider-secrets'
        ),
      }).success
    ).toBe(false);
  });
}

function rejectsIncompleteRuntimeHandoffCoverage(): void {
  it('rejects proof that omits support status or an account backend non-claim', () => {
    expect(
      ProductionReleasePublicRuntimeHandoffProofSchema.safeParse({
        ...ProductionReleasePublicRuntimeHandoffReadModel,
        handoffRows: ProductionReleasePublicRuntimeHandoffReadModel.handoffRows.filter(
          (row) => row.surface !== 'support-status'
        ),
      }).success
    ).toBe(false);
    expect(
      ProductionReleasePublicRuntimeHandoffProofSchema.safeParse({
        ...ProductionReleasePublicRuntimeHandoffReadModel,
        nonClaims: ProductionReleasePublicRuntimeHandoffReadModel.nonClaims.filter(
          (nonClaim) => nonClaim !== 'no-account-backend-runtime'
        ),
      }).success
    ).toBe(false);
  });
}

function requiredHandoff(
  surface: 'account-status' | 'support-status'
): (typeof ProductionReleasePublicRuntimeHandoffReadModel.handoffRows)[number] {
  const row = ProductionReleasePublicRuntimeHandoffReadModel.handoffRows.find((entry) => entry.surface === surface);
  if (row === undefined) {
    throw new Error(`missing public runtime handoff row: ${surface}`);
  }
  return row;
}

function requiredAdapter(
  adapter: 'billing-provider-runtime'
): (typeof ProductionReleasePublicRuntimeHandoffReadModel.adapterRows)[number] {
  const row = ProductionReleasePublicRuntimeHandoffReadModel.adapterRows.find((entry) => entry.adapter === adapter);
  if (row === undefined) {
    throw new Error(`missing public runtime adapter row: ${adapter}`);
  }
  return row;
}
