import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseApprovalContractProofSchema,
  AppInstallPurchaseApprovalDecisionSchema,
  AppInstallPurchaseApprovalPlatformSupportRowSchema,
  AppInstallPurchaseApprovalStateSnapshotSchema,
  AppInstallPurchaseApprovalStoreMetadataSchema,
} from '../src/app-install-purchase-approval';
import { AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema } from '../src/app-install-purchase-approval-platform-sources';
import { AppInstallPurchaseApprovalContractProofReadModel } from '../src/app-install-purchase-approval-proof';

describe('app install and purchase approval contracts', () => {
  acceptsTheContractOnlyInstallPurchaseAndSubscriptionProof();
  acceptsPlatformSourceMetadataLimitationsWithoutStoreApiClaims();
  acceptsChildFacingPendingAndResultStatesWithAuditReportRefs();
  acceptsAuditAndReportIntegrationStatusWithoutRuntimeClaims();
  rejectsPlatformStoreBillingRuntimePortalAndBypassOverclaims();
  rejectsPlatformSourceMetadataRowsThatClaimRuntimeOrOmitLimitationProof();
  rejectsChildFacingStatesThatInventDeliveryOrMismatchApprovalState();
  rejectsAuditReportIntegrationRowsThatOmitAuditReportRefsOrClaimPortalRuntime();
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
    expect(childVisibleStatusCounts(proof)).toEqual({
      'pending-parent-review-visible': 1,
      'approved-visible': 1,
      'denied-visible': 1,
      'time-box-visible': 1,
      'review-needed-visible': 1,
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

function acceptsPlatformSourceMetadataLimitationsWithoutStoreApiClaims(): void {
  it('accepts platform-source metadata limitation rows without store api or interception claims', () => {
    const proof = AppInstallPurchaseApprovalContractProofSchema.parse(AppInstallPurchaseApprovalContractProofReadModel);

    expect(
      proof.platformSourceMetadata.map((row) => [
        row.platform,
        row.storeSurface,
        row.sourceAuthority,
        row.metadataState,
        row.sourceEvidenceState,
      ])
    ).toEqual([
      ['windows', 'microsoft-store', 'microsoft-store-listing', 'manual-required', 'requires-store-artifact-proof'],
      ['macos', 'mac-app-store', 'mac-app-store-listing', 'manual-required', 'requires-store-artifact-proof'],
      ['linux', 'linux-package-manager', 'linux-package-manager-index', 'unavailable', 'platform-unavailable'],
      ['android', 'google-play', 'google-play-listing', 'manual-required', 'requires-approved-api-proof'],
      ['ios', 'apple-app-store', 'apple-app-store-listing', 'manual-required', 'requires-approved-api-proof'],
    ]);
    expect(proof.platformSourceMetadata.map((row) => [row.platform, row.requiredArtifacts.length])).toEqual([
      ['windows', 3],
      ['macos', 3],
      ['linux', 3],
      ['android', 3],
      ['ios', 3],
    ]);
    for (const row of proof.platformSourceMetadata) {
      expect(row.fieldsAvailableFromContract).toEqual([]);
      expect(row.fieldsRequiringPlatformProof).toEqual([
        'store-listing-id',
        'app-title',
        'publisher-name',
        'category',
        'age-rating',
        'price-display',
        'subscription-period',
        'source-url',
      ]);
      expect(row.requestKindCoverage).toEqual(['install', 'purchase', 'subscription']);
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.interceptionClaim).toBe('not-claimed');
      expect(row.limitationReportRef).toBe('app-install-purchase-platform-limitation-report-ref');
      expect(row.claimBoundary).toContain('no store integration');
      expect(row.claimBoundary).toContain('no real install or purchase interception');
    }
  });
}

function acceptsChildFacingPendingAndResultStatesWithAuditReportRefs(): void {
  it('accepts child-facing pending result and review-needed states only as manual-required contract delivery', () => {
    const proof = AppInstallPurchaseApprovalContractProofSchema.parse(AppInstallPurchaseApprovalContractProofReadModel);

    expect(proof.childFacingStates.map((state) => state.childVisibleStatus)).toEqual([
      'pending-parent-review-visible',
      'approved-visible',
      'denied-visible',
      'time-box-visible',
      'review-needed-visible',
    ]);
    for (const state of proof.childFacingStates) {
      expect(state.deliveryState).toBe('manual-required');
      expect(state.auditEventRefs.length).toBeGreaterThan(0);
      expect(state.reportRefs).toEqual(['app-install-purchase-child-facing-report-ref']);
      expect(state.claimBoundary).toContain('no platform adapter');
    }
  });
}

function acceptsAuditAndReportIntegrationStatusWithoutRuntimeClaims(): void {
  it('accepts audit and report integration status rows without portal or report runtime claims', () => {
    const proof = AppInstallPurchaseApprovalContractProofSchema.parse(AppInstallPurchaseApprovalContractProofReadModel);

    expect(proof.auditReportIntegration.map((row) => [row.surface, row.integrationState])).toEqual([
      ['request-audit-history', 'contract-only'],
      ['parent-decision-audit-history', 'contract-only'],
      ['child-facing-state-report', 'manual-required'],
      ['platform-limitation-report', 'manual-required'],
    ]);
    for (const row of proof.auditReportIntegration) {
      expect(row.auditEventRefs.length).toBeGreaterThan(0);
      expect(row.reportRefs.length).toBeGreaterThan(0);
      expect(row.claimBoundary).toContain('no portal runtime');
    }
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

function rejectsPlatformSourceMetadataRowsThatClaimRuntimeOrOmitLimitationProof(): void {
  it('rejects platform-source metadata rows that claim store integration or omit limitation proof', () => {
    const androidRow = platformSourceRowFor('android');

    expect(
      AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema.safeParse({
        ...androidRow,
        storeIntegrationClaim: 'claimed',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema.safeParse({
        ...androidRow,
        requiredArtifacts: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseApprovalPlatformSourceMetadataRowSchema.safeParse({
        ...androidRow,
        sourceEvidenceState: 'platform-unavailable',
      }).success
    ).toBe(false);
    expect(
      contractProofWithPlatformSourceMetadata(
        AppInstallPurchaseApprovalContractProofReadModel.platformSourceMetadata.slice(1)
      ).success
    ).toBe(false);
  });
}

function rejectsChildFacingStatesThatInventDeliveryOrMismatchApprovalState(): void {
  it('rejects child-facing states that claim delivery support or mismatch the approval state', () => {
    const childState = AppInstallPurchaseApprovalContractProofReadModel.childFacingStates[0];

    expect(
      contractProofWithChildState({
        ...childState,
        deliveryState: 'supported',
      }).success
    ).toBe(false);
    expect(
      contractProofWithChildState({
        ...childState,
        childVisibleStatus: 'approved-visible',
      }).success
    ).toBe(false);
    expect(
      contractProofWithChildState({
        ...childState,
        reportRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsAuditReportIntegrationRowsThatOmitAuditReportRefsOrClaimPortalRuntime(): void {
  it('rejects audit report integration rows missing audit report refs or no-portal-runtime boundaries', () => {
    const reportRow = AppInstallPurchaseApprovalContractProofReadModel.auditReportIntegration[0];

    expect(
      contractProofWithAuditReportRow({
        ...reportRow,
        auditEventRefs: [],
      }).success
    ).toBe(false);
    expect(
      contractProofWithAuditReportRow({
        ...reportRow,
        reportRefs: [],
      }).success
    ).toBe(false);
    expect(
      contractProofWithAuditReportRow({
        ...reportRow,
        claimBoundary: 'contract proof only',
      }).success
    ).toBe(false);
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

function platformSourceRowFor(platform: 'android') {
  const row = AppInstallPurchaseApprovalContractProofReadModel.platformSourceMetadata.find(
    (entry) => entry.platform === platform
  );
  if (row === undefined) {
    throw new Error(`missing app install/purchase approval platform-source row: ${platform}`);
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

function childVisibleStatusCounts(proof: typeof AppInstallPurchaseApprovalContractProofReadModel) {
  return countBy(proof.childFacingStates.map((state) => state.childVisibleStatus));
}

function contractProofWithChildState(childState: unknown) {
  return AppInstallPurchaseApprovalContractProofSchema.safeParse({
    ...AppInstallPurchaseApprovalContractProofReadModel,
    childFacingStates: [childState, ...AppInstallPurchaseApprovalContractProofReadModel.childFacingStates.slice(1)],
  });
}

function contractProofWithAuditReportRow(reportRow: unknown) {
  return AppInstallPurchaseApprovalContractProofSchema.safeParse({
    ...AppInstallPurchaseApprovalContractProofReadModel,
    auditReportIntegration: [
      reportRow,
      ...AppInstallPurchaseApprovalContractProofReadModel.auditReportIntegration.slice(1),
    ],
  });
}

function contractProofWithPlatformSourceMetadata(platformSourceMetadata: unknown) {
  return AppInstallPurchaseApprovalContractProofSchema.safeParse({
    ...AppInstallPurchaseApprovalContractProofReadModel,
    platformSourceMetadata,
  });
}

function countBy(values: readonly string[]) {
  return values.reduce<Record<string, number>>((counts, value) => {
    counts[value] = (counts[value] ?? 0) + 1;
    return counts;
  }, {});
}
