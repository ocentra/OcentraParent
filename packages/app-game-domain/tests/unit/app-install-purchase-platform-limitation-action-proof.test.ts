import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchasePlatformLimitationActionProofReadModel,
  AppInstallPurchasePlatformLimitationActionProofSchema,
  AppInstallPurchasePlatformLimitationActionRowSchema,
  summarizeAppInstallPurchasePlatformLimitationActionProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-platform-limitation-action-proof';

describe('app install and purchase platform limitation action proof', () => {
  acceptsPlatformLimitationActionRowsWithoutPortalProviderOrDeliveryClaims();
  rejectsMissingPlatformLimitationCoverageOrRefs();
  rejectsPortalProviderAdapterDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingPlatformLimitationActionNonClaims();
});

function acceptsPlatformLimitationActionRowsWithoutPortalProviderOrDeliveryClaims(): void {
  it('accepts parent limitation action rows linked to provider store and report status refs', () => {
    const proof = AppInstallPurchasePlatformLimitationActionProofSchema.parse(
      AppInstallPurchasePlatformLimitationActionProofReadModel
    );

    expect(summarizeAppInstallPurchasePlatformLimitationActionProof(proof)).toEqual({
      platformLimitationActionRows: 5,
      readyRows: 1,
      manualRequiredRows: 3,
      unavailableRows: 1,
      reportStatusLinkedRows: 5,
      providerExecutedRows: 0,
      portalRows: 0,
    });
    expect(
      proof.platformLimitationActionRows.map(
        (row) =>
          `${row.platform}:${row.storeSurface}:${row.sourceProviderStoreReportStatusState}:${row.platformLimitationActionState}`
      )
    ).toEqual([
      'windows:microsoft-store:provider-store-report-status-ready:parent-action-ready',
      'macos:mac-app-store:manual-required:manual-required',
      'linux:linux-package-manager:unavailable:unavailable',
      'android:google-play:manual-required:manual-required',
      'ios:apple-app-store:manual-required:manual-required',
    ]);
    for (const row of proof.platformLimitationActionRows) {
      expect(row.sourceProviderStoreReportStatusRowId).toContain(row.platform);
      expect(row.sourceReportStatusReadModelRowIds).toHaveLength(4);
      expect(row.sourceReportStatusReadModelStates).toEqual([
        'parent-report-status-ready',
        'parent-report-status-ready',
        'parent-report-status-ready',
        'manual-required',
      ]);
      expect(row.parentVisibleReportStatusRefs).toHaveLength(4);
      expect(row.auditEventRefs.length).toBeGreaterThan(0);
      expect(row.parentLimitationActionRef).toContain(row.platform);
      expect(row.portalApprovalUiClaim).toBe('not-implemented');
      expect(row.portalReportUiClaim).toBe('not-implemented');
      expect(row.runtimeReportDeliveryClaim).toBe('not-delivered');
      expect(row.providerApiExecutionClaim).toBe('not-executed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.billingProviderContactClaim).toBe('not-executed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeviceDeliveryClaim).toBe('not-delivered');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.ocentraHostedFamilyDataCustodyClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('parent-visible limitation follow-up rows');
      expect(row.claimBoundary).toContain('no provider API execution');
    }
  });
}

function rejectsMissingPlatformLimitationCoverageOrRefs(): void {
  it('rejects proofs that omit platform limitation action rows or required refs', () => {
    const proof = AppInstallPurchasePlatformLimitationActionProofReadModel;
    const row = proof.platformLimitationActionRows[0];

    expect(
      AppInstallPurchasePlatformLimitationActionProofSchema.safeParse({
        ...proof,
        platformLimitationActionRows: proof.platformLimitationActionRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformLimitationActionRowSchema.safeParse({
        ...row,
        sourceProviderStoreReportStatusRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformLimitationActionRowSchema.safeParse({
        ...row,
        sourceReportStatusReadModelRowIds: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformLimitationActionRowSchema.safeParse({
        ...row,
        parentVisibleReportStatusRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchasePlatformLimitationActionRowSchema.safeParse({
        ...row,
        auditEventRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderAdapterDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim portal provider adapter delivery custody or blocking behavior', () => {
    const row = AppInstallPurchasePlatformLimitationActionProofReadModel.platformLimitationActionRows[0];

    for (const invalidRow of [
      { ...row, platformLimitationActionState: 'delivered' },
      { ...row, portalApprovalUiClaim: 'implemented' },
      { ...row, portalReportUiClaim: 'implemented' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, billingProviderContactClaim: 'contacted' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'limitation action delivered through portal provider adapter and child device' },
    ]) {
      expect(AppInstallPurchasePlatformLimitationActionRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingPlatformLimitationActionNonClaims(): void {
  it('rejects platform limitation action proof when required non-claims are removed', () => {
    const proof = AppInstallPurchasePlatformLimitationActionProofReadModel;

    for (const claim of [
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-external-runtime-report-delivery',
      'no-provider-api-execution',
      'no-store-integration',
      'no-billing-provider-contact',
      'no-platform-adapter-implementation',
      'no-child-device-delivery',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchasePlatformLimitationActionProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}
