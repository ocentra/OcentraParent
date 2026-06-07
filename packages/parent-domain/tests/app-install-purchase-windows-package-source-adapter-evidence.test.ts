import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofReadModel,
  AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofSchema,
  AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema,
  buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof,
  summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof,
} from '../src/app-install-purchase-windows-package-source-adapter-evidence';

describe('app install purchase Windows package-source adapter evidence proof', () => {
  acceptsWindowsHostEvidenceWithoutProviderStoreOrDeliveryClaims();
  acceptsManualWindowsHostEvidenceWhenTheHostCommandIsUnavailable();
  rejectsMissingSourceRowsOrHostEvidenceRefs();
  rejectsProviderStorePlatformPortalDeliveryBlockingAndCustodyOverclaims();
  rejectsMissingWindowsPackageSourceAdapterEvidenceNonClaims();
});

function acceptsWindowsHostEvidenceWithoutProviderStoreOrDeliveryClaims(): void {
  it('links Windows host command evidence to package-source adapter proof without upgrading store claims', () => {
    const proof = buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(hostEvidence(true));

    expect(summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(proof)).toEqual({
      windowsPackageSourceAdapterEvidenceRows: 5,
      windowsHostEvidenceCollectedRows: 1,
      windowsHostManualRows: 0,
      manualAdapterEvidenceRows: 1,
      platformUnavailableRows: 1,
      blockedBeforeClaimRows: 2,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.windowsPackageSourceAdapterEvidenceRows.map((row) => [
        row.platform,
        row.storeSurface,
        row.sourcePlatformAdapterEvidenceGapState,
        row.sourcePackageSourceAdapterExecutionState,
        row.hostEvidenceState,
      ])
    ).toEqual([
      [
        'windows',
        'microsoft-store',
        'adapter-evidence-gap',
        'local-adapter-executed',
        'windows-host-evidence-collected',
      ],
      [
        'macos',
        'mac-app-store',
        'manual-adapter-evidence-required',
        'manual-host-proof-required',
        'manual-adapter-evidence-required',
      ],
      ['linux', 'linux-package-manager', 'platform-unavailable', 'platform-unavailable', 'platform-unavailable'],
      ['android', 'google-play', 'blocked-before-claim', 'device-management-required', 'blocked-before-claim'],
      ['ios', 'apple-app-store', 'blocked-before-claim', 'apple-entitlement-required', 'blocked-before-claim'],
    ]);

    const windowsRow = proof.windowsPackageSourceAdapterEvidenceRows[0];
    expect(windowsRow.hostEvidenceArtifactRefs).toEqual(['windows-package-source-host-evidence-artifact']);
    expect(windowsRow.providerApiExecutionClaim).toBe('not-executed');
    expect(windowsRow.storeIntegrationClaim).toBe('not-claimed');
    expect(windowsRow.platformInterceptionClaim).toBe('not-claimed');
    expect(windowsRow.productionPlatformAdapterClaim).toBe('not-implemented');
    expect(windowsRow.childDeviceDeliveryClaim).toBe('not-delivered');
    expect(windowsRow.portalApprovalUiClaim).toBe('not-claimed');
    expect(windowsRow.appBlockingClaim).toBe('not-claimed');
    expect(windowsRow.childDataCustody).toBe('no-child-activity-data');
  });
}

function acceptsManualWindowsHostEvidenceWhenTheHostCommandIsUnavailable(): void {
  it('keeps the Windows row manual when the local host command is absent', () => {
    const proof = AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofSchema.parse(
      AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofReadModel
    );

    expect(summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(proof)).toEqual({
      windowsPackageSourceAdapterEvidenceRows: 5,
      windowsHostEvidenceCollectedRows: 0,
      windowsHostManualRows: 1,
      manualAdapterEvidenceRows: 1,
      platformUnavailableRows: 1,
      blockedBeforeClaimRows: 2,
      providerExecutedRows: 0,
      childDeliveredRows: 0,
    });
    expect(proof.windowsPackageSourceAdapterEvidenceRows[0].hostEvidenceState).toBe('windows-host-manual-required');
  });
}

function rejectsMissingSourceRowsOrHostEvidenceRefs(): void {
  it('rejects proofs that omit source linkage or host evidence refs', () => {
    const proof = buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(hostEvidence(true));
    const row = proof.windowsPackageSourceAdapterEvidenceRows[0];

    expect(
      AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofSchema.safeParse({
        ...proof,
        windowsPackageSourceAdapterEvidenceRows: proof.windowsPackageSourceAdapterEvidenceRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema.safeParse({
        ...row,
        sourcePlatformAdapterEvidenceGapRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema.safeParse({
        ...row,
        sourcePackageSourceAdapterExecutionRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema.safeParse({
        ...row,
        hostEvidenceArtifactRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema.safeParse({
        ...row,
        requiredProviderStoreEvidenceRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformPortalDeliveryBlockingAndCustodyOverclaims(): void {
  it('rejects rows that claim store execution platform interception delivery blocking or custody', () => {
    const row = buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(hostEvidence(true))
      .windowsPackageSourceAdapterEvidenceRows[0];

    for (const invalidRow of [
      { ...row, productClaimApprovalClaim: 'claimed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, productionPlatformAdapterClaim: 'implemented' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'Windows adapter proof executes Microsoft Store provider APIs and delivers to child' },
    ]) {
      expect(AppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingWindowsPackageSourceAdapterEvidenceNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProof(hostEvidence(true));

    for (const claim of [
      'no-product-claim-approval',
      'no-google-play-execution',
      'no-apple-app-store-execution',
      'no-microsoft-store-execution',
      'no-billing-provider-contact',
      'no-provider-api-execution',
      'no-store-integration',
      'no-platform-interception',
      'no-production-platform-adapter',
      'no-child-device-delivery',
      'no-runtime-writer-delivery',
      'no-runtime-report-delivery',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofSchema.safeParse({
          ...proof,
          nonClaims: proof.nonClaims.filter((nonClaim) => nonClaim !== claim),
        }).success
      ).toBe(false);
    }
  });
}

function hostEvidence(commandAvailable: boolean) {
  return {
    artifactRef: 'windows-package-source-host-evidence-artifact',
    hostPlatform: 'win32',
    commandName: 'Get-AppxPackage',
    commandAvailable,
    commandExitCode: commandAvailable ? 0 : 1,
    evidenceSummary: commandAvailable
      ? 'Get-AppxPackage is available on the Windows host for package-source inspection proof.'
      : 'Get-AppxPackage is unavailable, so Windows package-source evidence remains manual-required.',
    collectedAt: '2026-06-07T02:30:00.000Z',
  } as const;
}
