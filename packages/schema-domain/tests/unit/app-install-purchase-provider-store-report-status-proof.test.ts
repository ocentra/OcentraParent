import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStoreReportStatusProofReadModel,
  AppInstallPurchaseProviderStoreReportStatusProofSchema,
  AppInstallPurchaseProviderStoreReportStatusRowSchema,
  summarizeAppInstallPurchaseProviderStoreReportStatusProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-report-status-proof';

describe('app install and purchase provider store report status proof', () => {
  acceptsProviderStoreReportStatusRows();
  rejectsMissingRefsOverclaimsAndMissingNonClaims();
});

function acceptsProviderStoreReportStatusRows(): void {
  it('accepts provider store report status rows that stay linked to readiness and approval-report inputs only', () => {
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

    const windowsRow = proof.providerStoreReportStatusRows.find((row) => row.platform === 'windows');
    if (windowsRow === undefined) {
      throw new Error('missing provider store report status row for windows');
    }

    expect(windowsRow.sourceApprovalReportDomainRowIds.length).toBeGreaterThan(0);
    expect(windowsRow.sourceApprovalReportDomainStates).toContain('approval-report-ready');
    expect(windowsRow.sourceApprovalReportDomainStates).toContain('approval-report-manual-review');
    expect(windowsRow.sourceReportRuntimeRefs.length).toBeGreaterThan(0);
    expect(windowsRow.sourceAuditEventRefs.length).toBeGreaterThan(0);

    for (const row of proof.providerStoreReportStatusRows) {
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
      expect(row.claimBoundary).toContain('no portal report UI');
    }
  });
}

function rejectsMissingRefsOverclaimsAndMissingNonClaims(): void {
  it('rejects report status rows that omit linked inputs or invent provider, portal, delivery, custody, or blocking claims', () => {
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

    for (const invalidRow of [
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, billingProviderContactClaim: 'executed' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'provider store report status is implemented' },
    ]) {
      expect(AppInstallPurchaseProviderStoreReportStatusRowSchema.safeParse(invalidRow).success).toBe(false);
    }

    expect(
      AppInstallPurchaseProviderStoreReportStatusProofSchema.safeParse({
        ...proof,
        nonClaims: proof.nonClaims.filter((claim) => claim !== 'no-provider-api-execution'),
      }).success
    ).toBe(false);
  });
}
