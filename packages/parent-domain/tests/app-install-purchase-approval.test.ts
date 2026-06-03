import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseApprovalContractProofSchema,
  AppInstallPurchaseApprovalDecisionSchema,
  AppInstallPurchaseApprovalPlatformSupportRowSchema,
  AppInstallPurchaseApprovalStateSnapshotSchema,
  AppInstallPurchaseApprovalStoreMetadataSchema,
} from '../src/app-install-purchase-approval';
import { AppInstallPurchaseApprovalContractProofReadModel } from '../src/app-install-purchase-approval-proof';

describe('app install and purchase approval contracts', () => {
  acceptsTheContractOnlyInstallPurchaseAndSubscriptionProof();
  rejectsPlatformStoreBillingRuntimePortalAndBypassOverclaims();
  rejectsUnscopedApprovalAndReviewDecisions();
  acceptsExpiredApprovalSnapshotsAndRejectsMismatchedExpiryState();
  rejectsUnavailableMetadataThatInventsStoreFields();
  rejectsPlatformRowsThatClaimInterceptionOrOmitManualRequirements();
});

function acceptsTheContractOnlyInstallPurchaseAndSubscriptionProof(): void {
  it('accepts request metadata approval audit and platform-state coverage without platform integration claims', () => {
    const proof = AppInstallPurchaseApprovalContractProofSchema.parse(AppInstallPurchaseApprovalContractProofReadModel);

    expect(requestKindCounts(proof)).toEqual({
      install: 1,
      purchase: 1,
      subscription: 1,
    });
    expect(platformStateCounts(proof)).toEqual({
      supported: 5,
      'manual-required': 24,
      unavailable: 6,
    });
    expect(metadataFreshnessCounts(proof)).toEqual({
      fresh: 1,
      stale: 1,
      'manual-required': 1,
    });
    expect(decisionActionCounts(proof)).toEqual({
      approve: 1,
      deny: 1,
      'time-box': 1,
      'review-needed': 1,
    });
    expect(proof.nonClaims).toEqual([
      'no-store-integration',
      'no-billing-entitlement-logic',
      'no-portal-ui',
      'no-platform-adapter',
      'no-store-policy-bypass',
      'no-real-install-or-purchase-interception',
      'not-generic-app-blocking',
    ]);
  });
}

function rejectsPlatformStoreBillingRuntimePortalAndBypassOverclaims(): void {
  it('rejects product claims for platform store integration billing entitlements runtime blocking portal UI or bypass', () => {
    const base = AppInstallPurchaseApprovalContractProofReadModel;

    for (const invalidProof of [
      { ...base, storeIntegrationClaim: 'claimed' },
      { ...base, billingEntitlementClaim: 'claimed' },
      { ...base, runtimeBlockingSeparation: 'generic-app-blocking' },
      { ...base, portalUiClaim: 'implemented' },
      { ...base, nonClaims: base.nonClaims.filter((claim) => claim !== 'no-store-policy-bypass') },
    ]) {
      expect(AppInstallPurchaseApprovalContractProofSchema.safeParse(invalidProof).success).toBe(false);
    }
  });
}

function rejectsUnscopedApprovalAndReviewDecisions(): void {
  it('rejects approval decisions that lack parent action audit refs or review-needed reason', () => {
    const timeBoxDecision = decisionFor('time-box');
    const reviewNeededDecision = decisionFor('review-needed');

    expect(
      AppInstallPurchaseApprovalDecisionSchema.safeParse({
        ...timeBoxDecision,
        parentAction: null,
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalDecisionSchema.safeParse({
        ...timeBoxDecision,
        auditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalDecisionSchema.safeParse({
        ...reviewNeededDecision,
        resultingState: { ...reviewNeededDecision.resultingState, reviewReason: null },
      }).success
    ).toBe(false);
  });
}

function acceptsExpiredApprovalSnapshotsAndRejectsMismatchedExpiryState(): void {
  it('accepts expired approval snapshots and rejects mismatched expiry states', () => {
    const expiredSnapshot = {
      state: 'expired',
      expiryState: 'expired',
      expiresAt: '2026-06-10T07:10:00.000Z',
      reviewReason: null,
    };

    expect(AppInstallPurchaseApprovalStateSnapshotSchema.safeParse(expiredSnapshot).success).toBe(true);
    expect(
      AppInstallPurchaseApprovalStateSnapshotSchema.safeParse({
        ...expiredSnapshot,
        expiryState: 'not-expiring',
      }).success
    ).toBe(false);
  });
}

function rejectsUnavailableMetadataThatInventsStoreFields(): void {
  it('rejects unavailable or unknown store metadata when it invents listing publisher rating category or timestamps', () => {
    const metadata = AppInstallPurchaseApprovalContractProofReadModel.installRequest.storeMetadata;

    expect(
      AppInstallPurchaseApprovalStoreMetadataSchema.safeParse({
        ...metadata,
        sourceState: 'unavailable',
        freshness: 'unavailable',
        listingId: 'invented-store-listing',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalStoreMetadataSchema.safeParse({
        ...metadata,
        sourceState: 'unavailable',
        freshness: 'unavailable',
        listingId: null,
        appTitle: null,
        publisherName: null,
        category: null,
        ageRating: null,
        refreshedAt: null,
        staleAt: null,
      }).success
    ).toBe(true);
    expect(
      AppInstallPurchaseApprovalStoreMetadataSchema.safeParse({
        ...metadata,
        freshness: 'unknown',
        refreshedAt: null,
      }).success
    ).toBe(false);
  });
}

function rejectsPlatformRowsThatClaimInterceptionOrOmitManualRequirements(): void {
  it('rejects platform rows that claim interception support or omit manual requirements for gated states', () => {
    const androidRow = platformRowFor('android');

    expect(
      AppInstallPurchaseApprovalPlatformSupportRowSchema.safeParse({
        ...androidRow,
        installInterceptionState: 'supported',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalPlatformSupportRowSchema.safeParse({
        ...androidRow,
        manualRequirement: null,
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalPlatformSupportRowSchema.safeParse({
        ...androidRow,
        claimBoundary: 'store integration is implemented',
      }).success
    ).toBe(false);
  });
}

function decisionFor(decisionAction: 'approve' | 'deny' | 'time-box' | 'review-needed') {
  const decision = AppInstallPurchaseApprovalContractProofReadModel.approvalDecisions.find(
    (entry) => entry.decisionAction === decisionAction
  );
  if (decision === undefined) {
    throw new Error(`missing app install/purchase approval decision: ${decisionAction}`);
  }
  return decision;
}

function platformRowFor(platform: 'android') {
  const row = AppInstallPurchaseApprovalContractProofReadModel.platformSupportMatrix.find(
    (entry) => entry.platform === platform
  );
  if (row === undefined) {
    throw new Error(`missing app install/purchase approval platform row: ${platform}`);
  }
  return row;
}

function requestKindCounts(proof: typeof AppInstallPurchaseApprovalContractProofReadModel) {
  return countBy([
    proof.installRequest.requestKind,
    proof.purchaseRequest.requestKind,
    proof.subscriptionRequest.requestKind,
  ]);
}

function platformStateCounts(proof: typeof AppInstallPurchaseApprovalContractProofReadModel) {
  return countBy(
    proof.platformSupportMatrix.flatMap((row) => [
      row.contractRequestState,
      row.storeMetadataState,
      row.installInterceptionState,
      row.purchaseInterceptionState,
      row.subscriptionInterceptionState,
      row.childPendingState,
      row.approvalDeliveryState,
    ])
  );
}

function metadataFreshnessCounts(proof: typeof AppInstallPurchaseApprovalContractProofReadModel) {
  return countBy([
    proof.installRequest.storeMetadata.freshness,
    proof.purchaseRequest.storeMetadata.freshness,
    proof.subscriptionRequest.storeMetadata.freshness,
  ]);
}

function decisionActionCounts(proof: typeof AppInstallPurchaseApprovalContractProofReadModel) {
  return countBy(proof.approvalDecisions.map((decision) => decision.decisionAction));
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
