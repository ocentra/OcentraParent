import { describe, expect, it } from 'vitest';
import {
  AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel,
  AppInstallPurchaseProviderStorePlatformEvidenceProofSchema,
  AppInstallPurchaseProviderStorePlatformEvidenceRowSchema,
  buildAppInstallPurchaseProviderStorePlatformEvidenceProof,
  summarizeAppInstallPurchaseProviderStorePlatformEvidenceProof,
} from '@ocentra-parent/schema-domain/app-install-purchase-provider-store-platform-evidence-proof';
import { buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof } from '@ocentra-parent/schema-domain/app-install-purchase-windows-package-source-adapter-evidence';

describe('app install purchase provider/store platform evidence proof', () => {
  acceptsManualProviderStorePlatformEvidenceRowsWithExactMissingArtifacts();
  acceptsWindowsPackageSourceHandoffWithoutUpgradingProviderStoreClaims();
  rejectsMissingSourceLinksOrMissingArtifactRefs();
  rejectsProviderStorePlatformDeliveryPortalBlockingAndCustodyOverclaims();
  rejectsMissingRequiredNonClaims();
});

function acceptsManualProviderStorePlatformEvidenceRowsWithExactMissingArtifacts(): void {
  it('classifies provider/store platform evidence as manual unavailable or blocked before claims', () => {
    const proof = AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.parse(
      AppInstallPurchaseProviderStorePlatformEvidenceProofReadModel
    );

    expect(summarizeAppInstallPurchaseProviderStorePlatformEvidenceProof(proof)).toEqual({
      providerStorePlatformEvidenceRows: 5,
      manualRequiredRows: 2,
      platformUnavailableRows: 1,
      blockedBeforeClaimRows: 2,
      providerExecutedRows: 0,
      platformAdapterImplementedRows: 0,
      childDeliveredRows: 0,
    });
    expect(
      proof.providerStorePlatformEvidenceRows.map((row) => [
        row.platform,
        row.storeSurface,
        row.sourceProviderStoreExecutionPreflightState,
        row.sourceRuntimeHandoffState,
        row.providerStorePlatformEvidenceState,
      ])
    ).toEqual([
      [
        'windows',
        'microsoft-store',
        'preflight-ready',
        'windows-runtime-handoff-manual-required',
        'manual-provider-store-platform-evidence-required',
      ],
      [
        'macos',
        'mac-app-store',
        'manual-provider-proof-required',
        'manual-runtime-handoff-required',
        'manual-provider-store-platform-evidence-required',
      ],
      ['linux', 'linux-package-manager', 'provider-unavailable', 'platform-unavailable', 'platform-unavailable'],
      ['android', 'google-play', 'manual-provider-proof-required', 'blocked-before-claim', 'blocked-before-claim'],
      ['ios', 'apple-app-store', 'manual-provider-proof-required', 'blocked-before-claim', 'blocked-before-claim'],
    ]);

    const windowsRow = proof.providerStorePlatformEvidenceRows[0];
    expect(windowsRow.missingProviderStoreArtifactRefs).toEqual([
      'missing-microsoft-store-provider-credential-proof',
      'missing-microsoft-store-provider-api-response-proof',
      'missing-billing-provider-contact-proof',
    ]);
    expect(windowsRow.missingPlatformArtifactRefs).toEqual([
      'missing-windows-production-platform-adapter-execution-proof',
      'missing-windows-platform-interception-policy-proof',
    ]);
    expect(windowsRow.missingChildDeviceArtifactRefs).toEqual(['missing-windows-child-device-delivery-receipt-proof']);
  });
}

function acceptsWindowsPackageSourceHandoffWithoutUpgradingProviderStoreClaims(): void {
  it('keeps Windows provider/store execution manual even when sanitized package-source evidence exists', () => {
    const runtimeProof = buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(hostEvidence(true));
    const proof = buildAppInstallPurchaseProviderStorePlatformEvidenceProof(runtimeProof);
    const windowsRow = proof.providerStorePlatformEvidenceRows[0];

    expect(windowsRow.sourceRuntimeHandoffState).toBe('windows-runtime-handoff-ready');
    expect(windowsRow.packageSourceEvidenceRefs).toEqual(['windows-package-source-host-evidence-artifact']);
    expect(windowsRow.providerStorePlatformEvidenceState).toBe('manual-provider-store-platform-evidence-required');
    expect(windowsRow.providerApiExecutionClaim).toBe('not-executed');
    expect(windowsRow.microsoftStoreExecutionClaim).toBe('not-executed');
    expect(windowsRow.storeIntegrationClaim).toBe('not-claimed');
    expect(windowsRow.productionPlatformAdapterClaim).toBe('not-implemented');
    expect(windowsRow.childDeviceDeliveryClaim).toBe('not-delivered');
  });
}

function rejectsMissingSourceLinksOrMissingArtifactRefs(): void {
  it('rejects rows that omit preflight runtime-handoff or missing-artifact evidence', () => {
    const proof = buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
      buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(hostEvidence(true))
    );
    const row = proof.providerStorePlatformEvidenceRows[0];

    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.safeParse({
        ...proof,
        providerStorePlatformEvidenceRows: proof.providerStorePlatformEvidenceRows.slice(1),
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        sourceProviderStoreExecutionPreflightRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        sourceWindowsPackageSourceRuntimeHandoffRowId: '',
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        missingProviderStoreArtifactRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        missingPlatformArtifactRefs: [],
      }).success
    ).toBe(false);
    expect(
      AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse({
        ...row,
        missingChildDeviceArtifactRefs: [],
      }).success
    ).toBe(false);
  });
}

function rejectsProviderStorePlatformDeliveryPortalBlockingAndCustodyOverclaims(): void {
  it('rejects rows that claim provider execution platform delivery portal blocking or custody', () => {
    const row = buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
      buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(hostEvidence(true))
    ).providerStorePlatformEvidenceRows[0];

    for (const invalidRow of [
      { ...row, productClaimApprovalClaim: 'claimed' },
      { ...row, googlePlayExecutionClaim: 'executed' },
      { ...row, appleAppStoreExecutionClaim: 'executed' },
      { ...row, microsoftStoreExecutionClaim: 'executed' },
      { ...row, billingProviderContactClaim: 'executed' },
      { ...row, providerApiExecutionClaim: 'executed' },
      { ...row, storeIntegrationClaim: 'claimed' },
      { ...row, platformInterceptionClaim: 'claimed' },
      { ...row, productionPlatformAdapterClaim: 'implemented' },
      { ...row, runtimeWriterExecutionClaim: 'executed' },
      { ...row, runtimeWriterDeliveryClaim: 'delivered' },
      { ...row, childDeviceDeliveryClaim: 'delivered' },
      { ...row, runtimeReportDeliveryClaim: 'delivered' },
      { ...row, portalApprovalUiClaim: 'claimed' },
      { ...row, portalReportUiClaim: 'claimed' },
      { ...row, appBlockingClaim: 'claimed' },
      { ...row, childDataCustody: 'child-activity-data-included' },
      { ...row, ocentraHostedFamilyDataCustodyClaim: 'claimed' },
      { ...row, claimBoundary: 'provider store platform evidence executes store APIs and delivers to child' },
    ]) {
      expect(AppInstallPurchaseProviderStorePlatformEvidenceRowSchema.safeParse(invalidRow).success).toBe(false);
    }
  });
}

function rejectsMissingRequiredNonClaims(): void {
  it('rejects proof when required non-claims are removed', () => {
    const proof = buildAppInstallPurchaseProviderStorePlatformEvidenceProof(
      buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProof(hostEvidence(true))
    );

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
      'no-runtime-writer-execution',
      'no-runtime-writer-delivery',
      'no-child-device-delivery',
      'no-runtime-report-delivery',
      'no-portal-approval-ui',
      'no-portal-report-ui',
      'no-app-blocking',
      'no-child-activity-data',
      'no-ocentra-hosted-family-data-custody',
    ] as const) {
      expect(
        AppInstallPurchaseProviderStorePlatformEvidenceProofSchema.safeParse({
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
    collectedAt: '2026-06-07T13:45:00.000Z',
  } as const;
}
