import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreReportStatusProofReadModel,
  AppInstallPurchaseProviderStoreReportStatusProofSchema,
  AppInstallPurchaseProviderStoreReportStatusRowSchema,
  summarizeAppInstallPurchaseProviderStoreReportStatusProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-report-status-proof';

describe('app install and purchase provider store report status proof', () => {
  acceptsProviderStoreReportStatusRowsWithoutProviderOrPortalClaims();
  rejectsMissingProviderStoreReportStatusCoverageOrRefs();
  rejectsProviderPortalDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingProviderStoreReportStatusNonClaims();
});

function acceptsProviderStoreReportStatusRowsWithoutProviderOrPortalClaims(): void {
  it('accepts provider store report status rows linked to readiness and approval report refs', () => {
    const proof = AppInstallPurchaseProviderStoreReportStatusProofSchema.parse(
      AppInstallPurchaseProviderStoreReportStatusProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStoreReportStatusProof(proof)).toEqual({
      providerStoreReportStatusRows: 5,
      readyRows: 1,
      manualRequiredRows: 3,
      unavailableRows: 1,
      approvalReportLinkedRows: 5,
      providerExecutedRows: 0,
      portalRows: 0,
    });
    expect(
      proof.providerStoreReportStatusRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreExecutionReadinessState}:${row.providerStoreReportStatusState}`
      )
    ).toEqual([
      'windows:microsoft-store:provider-store-execution-ready:provider-store-report-status-ready',
      'macos:mac-app-store:manual-required:manual-required',
      'linux:linux-package-manager:unavailable:unavailable',
      'android:google-play:manual-required:manual-required',
      'ios:apple-app-store:manual-required:manual-required',
    ]);
    for (const row of proof.providerStoreReportStatusRows) {
      expect(row.sourceProviderStoreExecutionReadinessRowId).toContain(row.platform);
      expect(row.sourceApprovalReportDomainRowIds).toHaveLength(4);
      expect(row.sourceApprovalReportDomainStates).toEqual([
        'approval-report-ready',
        'approval-report-ready',
        'approval-report-ready',
        'approval-report-manual-review',
      ]);
      expect(row.sourceReportRuntimeRefs.length).toBeGreaterThan(0);
      expect(row.sourceAuditEventRefs.length).toBeGreaterThan(0);
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.portalReportUiClaim).toBe('not-implemented');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no provider API execution');
      expect(row.claimBoundary).toContain('no runtime report delivery');
    }
  });
}

function rejectsMissingProviderStoreReportStatusCoverageOrRefs(): void {
  it('rejects proofs that omit provider store report status rows or required refs', () => {
    const proof = AppInstallPurchaseProviderStoreReportStatusProofReadModel;
    const row = proof.providerStoreReportStatusRows[0];

    expect(
      AppInstallPurchaseProviderStoreReportStatusProofSchema.safeParse({
        ...proof,
        providerStoreReportStatusRows: proof.providerStoreReportStatusRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreReportStatusRowSchema.safeParse({
        ...row,
        sourceProviderStoreExecutionReadinessRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreReportStatusRowSchema.safeParse({
        ...row,
        sourceApprovalReportDomainRowIds: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreReportStatusRowSchema.safeParse({
        ...row,
        sourceReportRuntimeRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStoreReportStatusRowSchema.safeParse({
        ...row,
        sourceAuditEventRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderPortalDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim provider portal delivery custody or blocking execution', () => {
    const row = AppInstallPurchaseProviderStoreReportStatusProofReadModel.providerStoreReportStatusRows[0];

    for (const invalidRow of [
      { ...row, providerStoreReportStatusState: 'provider-executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, portalReportUiClaim: 'implemented' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'provider store report status delivered through portal and contacted provider' },
    ]) {
      expect(AppInstallPurchaseProviderStoreReportStatusRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingProviderStoreReportStatusNonClaims(): void {
  it('rejects provider store report status proof when required non-claims are removed', () => {
    const proof = AppInstallPurchaseProviderStoreReportStatusProofReadModel;

    for (const claim of [
      'no-provider-api-execution',
      'no-store-integration',
      'no-billing-provider-contact',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-runtime-report-delivery',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStoreReportStatusProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}
