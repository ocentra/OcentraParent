import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel,
  AppInstallPurchaseProviderStoreExecutionReadinessProofSchema,
  AppInstallPurchaseProviderStoreExecutionReadinessRowSchema,
  summarizeAppInstallPurchaseProviderStoreExecutionReadinessProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-execution-readiness-proof';

describe('app install and purchase provider store execution readiness proof', () => {
  acceptsProviderStoreExecutionReadinessRowsWithoutProviderClaims();
  rejectsMissingProviderStoreExecutionReadinessCoverageOrRefs();
  rejectsProviderStorePlatformDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingProviderStoreExecutionReadinessNonClaims();
});

function acceptsProviderStoreExecutionReadinessRowsWithoutProviderClaims(): void {
  it('accepts provider store execution readiness rows linked to source evidence and parent action readiness', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.parse(
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreExecutionReadinessProof(proof)).toEqual({
      providerStoreExecutionReadinessRows: 5,
      executionReadyRows: 1,
      manualRequiredRows: 3,
      unavailableRows: 1,
      packageSourceAdapterLinkedRows: 5,
      parentActionReadinessLinkedRows: 5,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.providerStoreExecutionReadinessRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceApiEntitlementEvidenceStatus}:${row.sourceStoreStatusHandoffState}:${row.sourcePackageSourceAdapterExecutionState}:${row.providerStoreExecutionReadinessState}`
      )
    ).toEqual([
      'windows:microsoft-store:approved-api-evidence-required:approved-api-status-proof-required:local-adapter-executed:provider-store-execution-ready',
      'macos:mac-app-store:manual-platform-review-required:manual-platform-status-review-required:manual-host-proof-required:manual-required',
      'linux:linux-package-manager:platform-unavailable:platform-store-status-unavailable:platform-unavailable:unavailable',
      'android:google-play:store-entitlement-evidence-required:store-entitlement-status-proof-required:device-management-required:manual-required',
      'ios:apple-app-store:store-entitlement-evidence-required:store-entitlement-status-proof-required:apple-entitlement-required:manual-required',
    ]);
    for (const row of proof.providerStoreExecutionReadinessRows) {
      expect(row.sourceApprovedApiEntitlementRowId).toContain(row.platform);
      expect(row.sourceStoreStatusHandoffRowId).toContain(row.platform);
      expect(row.sourcePackageSourceAdapterExecutionRowId).toContain(row.platform);
      expect(row.sourceParentActionDeliveryReadinessRefs).toHaveLength(4);
      expect(row.sourceParentActionDeliveryReadinessStates).toEqual([
        'parent-action-delivery-ready',
        'parent-action-delivery-ready',
        'parent-action-delivery-ready',
        'manual-review-required',
      ]);
      expect(row.approvedApiEvidenceRefs).toHaveLength(1);
      expect(row.entitlementEvidenceRefs).toHaveLength(1);
      expect(row.storeStatusHandoffEvidenceRefs.length).toBeGreaterThan(0);
      expect(row.packageSourceAdapterArtifactRefs.length).toBeGreaterThan(0);
      expect(row.parentActionAuditEventRefs).toHaveLength(4);
      expect(row.reportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.requiredProofRefs.length).toBeGreaterThan(0);
      expect(row.googlePlayExecutionClaim).toBe('not-executed');
      expect(row.appleAppStoreExecutionClaim).toBe('not-executed');
      expect(row.microsoftStoreExecutionClaim).toBe('not-executed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformInterceptionClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.runtimeWriterDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no billing provider contact');
      expect(row.claimBoundary).toContain('no platform interception');
    }
  });
}

function rejectsMissingProviderStoreExecutionReadinessCoverageOrRefs(): void {
  it('rejects proofs that omit provider store rows or required source refs', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel;
    const row = proof.providerStoreExecutionReadinessRows[0];

    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.safeParse({
        ...proof,
        providerStoreExecutionReadinessRows: proof.providerStoreExecutionReadinessRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        sourceApprovedApiEntitlementRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        sourceStoreStatusHandoffRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        sourcePackageSourceAdapterExecutionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        sourceParentActionDeliveryReadinessRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse({
        ...row,
        packageSourceAdapterArtifactRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim provider store platform delivery custody or blocking execution', () => {
    const row = AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows[0];

    for (const invalidRow of [
      { ...row, providerStoreExecutionReadinessState: 'executed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'provider store execution contacted Google Play and delivered to child device' },
    ]) {
      expect(AppInstallPurchaseProviderStoreExecutionReadinessRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingProviderStoreExecutionReadinessNonClaims(): void {
  it('rejects provider store execution readiness proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel;

    for (const claim of [
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-billing-provider-contact',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-runtime-writer-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStoreExecutionReadinessProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}
