import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseReportRuntimeProofReadModel,
  AppInstallPurchaseReportRuntimeProofSchema,
  AppInstallPurchaseReportRuntimeSurfaceRowSchema,
  summarizeAppInstallPurchaseReportRuntimeProof,
} from '../src/app-install-purchase-report-runtime-proof';

describe('app install and purchase report runtime proof', () => {
  acceptsCompilerStatusLinkedReportRowsWithoutDeliveryClaims();
  rejectsMissingReportSurfaceOrCompilerCoverage();
  rejectsPortalProviderDeliveryCustodyAndBlockingOverclaims();
  rejectsMissingReportRuntimeNonClaims();
});

function acceptsCompilerStatusLinkedReportRowsWithoutDeliveryClaims(): void {
  it('accepts report rows linked to compiler statuses without portal provider delivery custody or blocking claims', () => {
    const proof = AppInstallPurchaseReportRuntimeProofSchema.parse(AppInstallPurchaseReportRuntimeProofReadModel);

    expect(summarizeAppInstallPurchaseReportRuntimeProof(proof)).toEqual({
      reportRuntimeRows: 4,
      compilerLinkedRows: 4,
      outputReportRefs: 4,
      portalDeliveredRows: 0,
    });
    expect(proof.reportRuntimeRows.map((row) => row.reportSurface)).toEqual([
      'request-audit-history',
      'parent-decision-audit-history',
      'child-facing-state-report',
      'platform-limitation-report',
    ]);
    for (const row of proof.reportRuntimeRows) {
      expect(row.compilerStatuses).toEqual(['queued', 'running', 'succeeded', 'failed', 'expired', 'manual-required']);
      expect(row.compilerFinalResultStatuses).toEqual(['succeeded', 'failed', 'expired', 'manual-required']);
      expect(row.parentAuthorized).toBe(true);
      expect(row.rawChildEvidenceRequested).toBe(false);
      expect(row.rawEvidenceExcludedFromOutput).toBe(true);
      expect(row.childDetailMinimized).toBe(true);
      expect(row.tempDeletionConfirmed).toBe(true);
      expect(row.localEvidenceMutated).toBe(false);
      expect(row.ocentraHostedReportRetained).toBe(false);
      expect(row.runtimeReportDeliveryClaim).toBe('not-portal-delivered');
      expect(row.portalUiClaim).toBe('not-claimed');
      expect(row.providerApiClaim).toBe('not-claimed');
      expect(row.storeIntegrationClaim).toBe('not-claimed');
      expect(row.platformAdapterClaim).toBe('not-implemented');
      expect(row.childDeliveryClaim).toBe('not-delivered');
      expect(row.childDataCustody).toBe('no-child-activity-data');
      expect(row.appBlockingClaim).toBe('not-claimed');
      expect(row.claimBoundary).toContain('no portal report UI');
    }
  });
}

function rejectsMissingReportSurfaceOrCompilerCoverage(): void {
  it('rejects report proof rows that omit report surfaces or compiler lifecycle coverage', () => {
    const proof = AppInstallPurchaseReportRuntimeProofReadModel;
    const row = proof.reportRuntimeRows[0];

    expect(
      AppInstallPurchaseReportRuntimeProofSchema.safeParse({
        ...proof,
        reportRuntimeRows: proof.reportRuntimeRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseReportRuntimeSurfaceRowSchema.safeParse({
        ...row,
        compilerStatuses: row.compilerStatuses.filter((status) => status !== 'manual-required'),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseReportRuntimeSurfaceRowSchema.safeParse({
        ...row,
        compilerFinalResultStatuses: row.compilerFinalResultStatuses.filter((status) => status !== 'expired'),
      }).success
    ).toBe(false);
  });
}

function rejectsPortalProviderDeliveryCustodyAndBlockingOverclaims(): void {
  it('rejects rows that claim portal delivery provider store adapter custody mutation or app blocking behavior', () => {
    const row = AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows[0];

    for (const invalidRow of [
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, portalUiClaim: 'claimed' },
      { ...row, providerApiClaim: 'claimed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformAdapterClaim: 'implemented' },
      { ...row, childDeliveryClaim: 'delivered' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, localEvidenceMutated: true },
      { ...row, ocentraHostedReportRetained: true },
      { ...row, claimBoundary: 'report delivered to portal' },
    ]) {
      expect(AppInstallPurchaseReportRuntimeSurfaceRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingReportRuntimeNonClaims(): void {
  it('rejects report runtime proof when report delivery custody or blocking non-claims are removed', () => {
    const proof = AppInstallPurchaseReportRuntimeProofReadModel;

    for (const claim of [
      'no-portal-report-ui',
      'no-runtime-report-delivery',
      'no-provider-api',
      'no-child-activity-data',
      'no-app-blocking',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseReportRuntimeProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}
