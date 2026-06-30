/* generated from crates/schema/src/app_install_purchase_platform_provider_helpers.ts.txt */

export function summarizeAppInstallPurchaseApprovedApiEntitlementProofGenerated(proof: {
  readonly evidenceRows: readonly { readonly evidenceStatus: string }[];
}) {
  return {
    evidenceRows: proof.evidenceRows.length,
    approvedApiRequiredRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'approved-api-evidence-required').length,
    entitlementRequiredRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'store-entitlement-evidence-required')
      .length,
    manualReviewRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'manual-platform-review-required').length,
    unavailableRows: proof.evidenceRows.filter((row) => row.evidenceStatus === 'platform-unavailable').length,
  } as const;
}

export function apiEntitlementEvidenceStatusGenerated(platform: string, storeSurface: string) {
  if (platform === 'linux' || storeSurface === 'linux-package-manager') {
    return 'platform-unavailable' as const;
  }
  if (platform === 'android' || platform === 'ios') {
    return 'store-entitlement-evidence-required' as const;
  }
  if (platform === 'macos') {
    return 'manual-platform-review-required' as const;
  }
  return 'approved-api-evidence-required' as const;
}

export function apiEntitlementEvidenceSourceGenerated(status: string) {
  if (status === 'approved-api-evidence-required') return 'approved-store-api' as const;
  if (status === 'store-entitlement-evidence-required') return 'store-entitlement' as const;
  if (status === 'manual-platform-review-required') return 'manual-platform-review' as const;
  return 'not-available' as const;
}

export function buildAppInstallPurchaseApprovedApiEntitlementEvidenceRowGenerated(
  row: {
    readonly childArtifactRowId: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly childDeliveryClaim: string;
    readonly requiredProofRefs: readonly string[];
  },
  sourceChildArtifactProofVersion: string,
  claimBoundary: string,
  attachedAt: string
) {
  const evidenceStatus = apiEntitlementEvidenceStatusGenerated(row.platform, row.storeSurface);
  return {
    schemaVersion: 'app-install-purchase-approved-api-entitlement-proof',
    sourceChildArtifactProofVersion,
    evidenceRowId: `approved-api-entitlement-${row.platform}-${row.storeSurface}`,
    sourceChildArtifactRowId: row.childArtifactRowId,
    platform: row.platform,
    storeSurface: row.storeSurface,
    evidenceStatus,
    evidenceSource: apiEntitlementEvidenceSourceGenerated(evidenceStatus),
    approvedApiEvidenceRef: `${row.platform}-${row.storeSurface}-approved-api-evidence-ref`,
    entitlementEvidenceRef: `${row.platform}-${row.storeSurface}-entitlement-evidence-ref`,
    limitationReportRef: `${row.platform}-${row.storeSurface}-api-entitlement-limitation-report-ref`,
    auditEventRefs: [`${row.platform}-${row.storeSurface}-api-entitlement-audit-ref`],
    requiredProofRefs: [...row.requiredProofRefs, `${row.platform}-${row.storeSurface}-approved-api-proof-ref`],
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: 'not-delivered',
    interceptionClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    claimBoundary,
    attachedAt,
  } as const;
}

export function apiEntitlementEvidenceRowIsHonestGenerated(
  row: {
    readonly platform: string;
    readonly storeSurface: string;
    readonly evidenceStatus: string;
    readonly evidenceSource: string;
    readonly auditEventRefs: readonly unknown[];
    readonly requiredProofRefs: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.evidenceStatus === apiEntitlementEvidenceStatusGenerated(row.platform, row.storeSurface) &&
    row.evidenceSource === apiEntitlementEvidenceSourceGenerated(row.evidenceStatus) &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.auditEventRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function apiEntitlementProofIsHonestGenerated(
  proof: {
    readonly sourceChildArtifactProofVersion: string;
    readonly evidenceRows: readonly { readonly platform: string; readonly storeSurface: string; readonly evidenceStatus: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceChildArtifactProofVersion: string,
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredStatuses: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.evidenceRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const statuses = new Set(proof.evidenceRows.map((row) => row.evidenceStatus));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceChildArtifactProofVersion === sourceChildArtifactProofVersion &&
    proof.evidenceRows.length === requiredPlatformSources.length &&
    keys.size === proof.evidenceRows.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    requiredStatuses.every((status) => statuses.has(status)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchasePlatformArtifactProofGenerated(proof: {
  readonly platformStoreArtifacts: readonly { readonly artifactSourceState: string; readonly sourceStoreMetadataArtifactState: string }[];
  readonly reportRuntimeEvidence: readonly unknown[];
}) {
  return {
    platformArtifactRows: proof.platformStoreArtifacts.length,
    reportRuntimeEvidenceRows: proof.reportRuntimeEvidence.length,
    attachedPlatformArtifacts: proof.platformStoreArtifacts.filter((row) => row.artifactSourceState === 'parent-owned-artifact-attached')
      .length,
    unavailableStoreMetadataRows: proof.platformStoreArtifacts.filter(
      (row) => row.sourceStoreMetadataArtifactState === 'platform-unavailable'
    ).length,
  } as const;
}

export function buildAppInstallPurchasePlatformStoreArtifactRowGenerated(
  row: {
    readonly platform: string;
    readonly storeSurface: string;
    readonly platformSourceRowId: string;
    readonly packageSourceArtifactRowId: string;
    readonly storeMetadataArtifactState: string;
    readonly packageSourceArtifactState: string;
    readonly runtimeClaimState: string;
    readonly requiredProofRefs: readonly string[];
    readonly reportRefs: readonly string[];
  },
  claimBoundary: string,
  attachedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-platform-artifact-proof',
    artifactRowId: `platform-artifact-${row.platform}-${row.storeSurface}`,
    platform: row.platform,
    storeSurface: row.storeSurface,
    platformSourceRowId: row.platformSourceRowId,
    packageSourceArtifactRowId: row.packageSourceArtifactRowId,
    artifactRef: `parent-owned-${row.platform}-${row.storeSurface}-artifact-ref`,
    artifactKind:
      row.storeMetadataArtifactState === 'platform-unavailable'
        ? 'platform-limitation-report-artifact'
        : 'platform-store-metadata-artifact',
    artifactSourceState: 'parent-owned-artifact-attached',
    sourceStoreMetadataArtifactState: row.storeMetadataArtifactState,
    sourcePackageArtifactState: row.packageSourceArtifactState,
    runtimeClaimState: row.runtimeClaimState,
    storeIntegrationClaim: 'not-claimed',
    providerApiClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    requiredProofRefs: row.requiredProofRefs,
    reportRefs: row.reportRefs,
    claimBoundary,
    attachedAt,
  } as const;
}

export function buildAppInstallPurchaseReportRuntimeEvidenceRowGenerated(
  row: {
    readonly reportSurface: string;
    readonly runtimeReportClaim: string;
    readonly auditEventRefs: readonly string[];
    readonly reportRefs: readonly string[];
  },
  claimBoundary: string,
  attachedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-platform-artifact-proof',
    reportSurface: row.reportSurface,
    artifactRef: `parent-owned-${row.reportSurface}-runtime-evidence-ref`,
    artifactSourceState: 'parent-owned-artifact-attached',
    runtimeReportDeliveryClaim: row.runtimeReportClaim,
    providerApiClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    auditEventRefs: row.auditEventRefs,
    reportRefs: row.reportRefs,
    claimBoundary,
    attachedAt,
  } as const;
}

export function platformStoreArtifactRowIsHonestGenerated(
  row: {
    readonly sourceStoreMetadataArtifactState: string;
    readonly artifactKind: string;
    readonly artifactSourceState: string;
    readonly runtimeClaimState: string;
    readonly storeIntegrationClaim: string;
    readonly providerApiClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly requiredProofRefs: readonly unknown[];
    readonly reportRefs: readonly unknown[];
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.artifactSourceState === 'parent-owned-artifact-attached' &&
    row.requiredProofRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    row.runtimeClaimState === 'boundary-only' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    (row.sourceStoreMetadataArtifactState === 'platform-unavailable'
      ? row.artifactKind === 'platform-limitation-report-artifact'
      : row.artifactKind === 'platform-store-metadata-artifact') &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function reportRuntimeEvidenceRowIsHonestGenerated(
  row: {
    readonly artifactSourceState: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly providerApiClaim: string;
    readonly platformAdapterClaim: string;
    readonly auditEventRefs: readonly unknown[];
    readonly reportRefs: readonly unknown[];
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.artifactSourceState === 'parent-owned-artifact-attached' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function platformArtifactProofIsHonestGenerated(
  proof: {
    readonly sourceRuntimeProofVersion: string;
    readonly platformStoreArtifacts: readonly { readonly platform: string; readonly storeSurface: string }[];
    readonly reportRuntimeEvidence: readonly { readonly reportSurface: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceRuntimeProofVersion: string,
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredReportSurfaces: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.platformStoreArtifacts.map((row) => `${row.platform}:${row.storeSurface}`));
  const surfaces = new Set(proof.reportRuntimeEvidence.map((row) => row.reportSurface));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeProofVersion === sourceRuntimeProofVersion &&
    proof.platformStoreArtifacts.length === requiredPlatformSources.length &&
    keys.size === proof.platformStoreArtifacts.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    proof.reportRuntimeEvidence.length === requiredReportSurfaces.length &&
    requiredReportSurfaces.every((surface) => surfaces.has(surface)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function platformProofReadinessStateGenerated(platform: string) {
  if (platform === 'linux') return 'unavailable' as const;
  if (platform === 'android' || platform === 'ios') return 'policy-blocked' as const;
  return 'manual-proof-required' as const;
}

export function requiredPlatformProofEvidenceRefsGenerated(platform: string) {
  const refs = {
    windows: ['windows-host-package-source-proof', 'windows-guarded-adapter-proof'],
    macos: ['macos-signing-receipt-proof', 'macos-store-source-manual-proof'],
    linux: ['linux-package-manager-source-path-proof'],
    android: ['android-device-owner-or-managed-profile-proof', 'google-play-policy-review-proof'],
    ios: ['ios-family-controls-entitlement-proof', 'apple-review-proof'],
  } as const;
  return refs[platform as keyof typeof refs];
}

export function summarizeAppInstallPurchasePlatformProofReadinessGenerated(proof: {
  readonly platformProofReadinessRows: readonly { readonly platformProofReadinessState: string; readonly providerApiExecutionClaim: string; readonly platformAdapterClaim: string }[];
}) {
  return {
    platformRows: proof.platformProofReadinessRows.length,
    manualProofRequiredRows: proof.platformProofReadinessRows.filter((row) => row.platformProofReadinessState === 'manual-proof-required').length,
    policyBlockedRows: proof.platformProofReadinessRows.filter((row) => row.platformProofReadinessState === 'policy-blocked').length,
    unavailableRows: proof.platformProofReadinessRows.filter((row) => row.platformProofReadinessState === 'unavailable').length,
    providerExecutedRows: proof.platformProofReadinessRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed').length,
    adapterImplementedRows: proof.platformProofReadinessRows.filter((row) => row.platformAdapterClaim !== 'not-implemented').length,
  } as const;
}

export function buildAppInstallPurchasePlatformProofReadinessRowGenerated(
  platform: string,
  sourceLimitationSummaryProofVersion: string,
  sourceLimitationSummaryRowIds: readonly string[],
  claimBoundary: string,
  checkedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-platform-proof-readiness',
    platform,
    platformProofReadinessState: platformProofReadinessStateGenerated(platform),
    sourceLimitationSummaryProofVersion,
    sourceLimitationSummaryRowIds,
    requiredManualEvidenceRefs: requiredPlatformProofEvidenceRefsGenerated(platform),
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    checkedAt,
  } as const;
}

export function platformProofReadinessRowIsHonestGenerated(
  row: {
    readonly platform: string;
    readonly sourceLimitationSummaryRowIds: readonly unknown[];
    readonly platformProofReadinessState: string;
    readonly requiredManualEvidenceRefs: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedSummaryRowCount: number,
  boundaryFragments: readonly string[]
) {
  return (
    row.sourceLimitationSummaryRowIds.length === expectedSummaryRowCount &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.platformProofReadinessState === platformProofReadinessStateGenerated(row.platform) &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function platformProofReadinessProofIsHonestGenerated(
  proof: {
    readonly sourceLimitationSummaryProofVersion: string;
    readonly platformProofReadinessRows: readonly { readonly platform: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceLimitationSummaryProofVersion: string,
  platforms: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const platformSet = new Set(proof.platformProofReadinessRows.map((row) => row.platform));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceLimitationSummaryProofVersion === sourceLimitationSummaryProofVersion &&
    proof.platformProofReadinessRows.length === platforms.length &&
    platforms.every((platform) => platformSet.has(platform)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function storeSurfaceForPlatformGenerated(platform: string) {
  const surfaces = {
    windows: 'microsoft-store',
    macos: 'mac-app-store',
    linux: 'linux-package-manager',
    android: 'google-play',
    ios: 'apple-app-store',
  } as const;
  return surfaces[platform as keyof typeof surfaces];
}

export function storeManualEvidenceStateGenerated(sourceState: string) {
  if (sourceState === 'unavailable') return 'store-unavailable' as const;
  if (sourceState === 'policy-blocked') return 'store-policy-review-required' as const;
  return 'manual-evidence-required' as const;
}

export function summarizeAppInstallPurchaseStoreManualEvidenceGenerated(proof: {
  readonly storeManualEvidenceRows: readonly { readonly storeManualEvidenceState: string; readonly providerApiExecutionClaim: string; readonly storeIntegrationClaim: string }[];
}) {
  return {
    storeRows: proof.storeManualEvidenceRows.length,
    manualEvidenceRequiredRows: proof.storeManualEvidenceRows.filter((row) => row.storeManualEvidenceState === 'manual-evidence-required').length,
    policyReviewRequiredRows: proof.storeManualEvidenceRows.filter((row) => row.storeManualEvidenceState === 'store-policy-review-required').length,
    unavailableRows: proof.storeManualEvidenceRows.filter((row) => row.storeManualEvidenceState === 'store-unavailable').length,
    providerExecutedRows: proof.storeManualEvidenceRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed').length,
    storeIntegratedRows: proof.storeManualEvidenceRows.filter((row) => row.storeIntegrationClaim !== 'not-claimed').length,
  } as const;
}

export function buildAppInstallPurchaseStoreManualEvidenceRowGenerated(
  sourceRow: {
    readonly platform: string;
    readonly platformProofReadinessState: string;
    readonly requiredManualEvidenceRefs: readonly string[];
  },
  sourcePlatformProofReadinessProofVersion: string,
  claimBoundary: string,
  checkedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-store-manual-evidence-proof',
    platform: sourceRow.platform,
    storeSurface: storeSurfaceForPlatformGenerated(sourceRow.platform),
    sourcePlatformProofReadinessProofVersion,
    sourcePlatformProofReadinessState: sourceRow.platformProofReadinessState,
    sourceManualEvidenceRefs: sourceRow.requiredManualEvidenceRefs,
    storeManualEvidenceState: storeManualEvidenceStateGenerated(sourceRow.platformProofReadinessState),
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    checkedAt,
  } as const;
}

export function storeManualEvidenceRowIsHonestGenerated(
  row: {
    readonly platform: string;
    readonly sourcePlatformProofReadinessProofVersion: string;
    readonly sourcePlatformProofReadinessState: string;
    readonly sourceManualEvidenceRefs: readonly unknown[];
    readonly storeManualEvidenceState: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourcePlatformProofReadinessProofVersion: string,
  boundaryFragments: readonly string[]
) {
  return (
    row.sourcePlatformProofReadinessProofVersion === sourcePlatformProofReadinessProofVersion &&
    row.sourceManualEvidenceRefs.length > 0 &&
    row.storeManualEvidenceState === storeManualEvidenceStateGenerated(row.sourcePlatformProofReadinessState ?? '') &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function storeManualEvidenceProofIsHonestGenerated(
  proof: {
    readonly sourcePlatformProofReadinessProofVersion: string;
    readonly storeManualEvidenceRows: readonly { readonly platform: string; readonly storeSurface: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourcePlatformProofReadinessProofVersion: string,
  platforms: readonly string[],
  storeSurfaces: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const platformSet = new Set(proof.storeManualEvidenceRows.map((row) => row.platform));
  const surfaceSet = new Set(proof.storeManualEvidenceRows.map((row) => row.storeSurface));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourcePlatformProofReadinessProofVersion === sourcePlatformProofReadinessProofVersion &&
    proof.storeManualEvidenceRows.length === platforms.length &&
    platforms.every((platform) => platformSet.has(platform)) &&
    storeSurfaces.every((surface) => surfaceSet.has(surface)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function providerStoreApiExecutionStateGenerated(
  sourceProviderStoreProductClaimState: string,
  sourceProviderStorePreflightState: string,
  sourcePlatformLimitationFallbackState: string
) {
  if (sourceProviderStorePreflightState === 'provider-unavailable') return 'unavailable' as const;
  if (sourcePlatformLimitationFallbackState === 'unsupported-platform-limitation-fallback-blocked') {
    return 'blocked-before-claim' as const;
  }
  if (
    sourceProviderStoreProductClaimState === 'provider-store-proof-required' &&
    sourcePlatformLimitationFallbackState === 'fallback-parent-workflow-ready'
  ) {
    return 'execution-ready' as const;
  }
  return 'manual-required' as const;
}

export function summarizeAppInstallPurchaseProviderStoreApiExecutionProofGenerated(proof: {
  readonly providerStoreApiExecutionRows: readonly { readonly providerStoreApiExecutionState: string; readonly providerApiExecutionClaim: string; readonly productClaimApprovalClaim: string }[];
}) {
  return {
    providerStoreApiExecutionRows: proof.providerStoreApiExecutionRows.length,
    executionReadyRows: proof.providerStoreApiExecutionRows.filter((row) => row.providerStoreApiExecutionState === 'execution-ready').length,
    manualRequiredRows: proof.providerStoreApiExecutionRows.filter((row) => row.providerStoreApiExecutionState === 'manual-required').length,
    unavailableRows: proof.providerStoreApiExecutionRows.filter((row) => row.providerStoreApiExecutionState === 'unavailable').length,
    blockedBeforeClaimRows: proof.providerStoreApiExecutionRows.filter((row) => row.providerStoreApiExecutionState === 'blocked-before-claim').length,
    providerExecutedRows: proof.providerStoreApiExecutionRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed').length,
    productClaimApprovedRows: proof.providerStoreApiExecutionRows.filter((row) => row.productClaimApprovalClaim !== 'not-claimed').length,
  } as const;
}

export function buildAppInstallPurchaseProviderStoreApiExecutionRowGenerated(
  providerStoreRow: {
    readonly providerStoreProductClaimRowId: string;
    readonly providerStoreProductClaimState: string;
    readonly sourceProviderStorePreflightState: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly requiredProviderStoreExecutionRefs: readonly string[];
    readonly requiredProviderEvidenceRefs: readonly string[];
    readonly requiredPortalTestRefs: readonly string[];
    readonly requiredChildDeliveryRefs: readonly string[];
    readonly requiredPlatformAdapterRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  fallbackRow: {
    readonly platformLimitationFallbackRowId: string;
    readonly fallbackState: string;
    readonly fallbackParentWorkflowRefs: readonly string[];
    readonly requiredManualPlatformEvidenceRefs: readonly string[];
    readonly requiredPortalTestRefs: readonly string[];
    readonly requiredChildDeliveryRefs: readonly string[];
    readonly requiredPlatformAdapterRefs: readonly string[];
    readonly limitationRefs: readonly string[];
    readonly requiredProviderStoreExecutionRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-provider-store-api-execution-proof',
    providerStoreApiExecutionRowId: `app-install-provider-store-api-execution-${providerStoreRow.platform}-${providerStoreRow.storeSurface}`,
    sourceProviderStoreProofVersion: 'app-install-purchase-product-claim-provider-store-proof',
    sourceProviderStoreRowId: providerStoreRow.providerStoreProductClaimRowId,
    sourceProviderStoreProductClaimState: providerStoreRow.providerStoreProductClaimState,
    sourceProviderStorePreflightState: providerStoreRow.sourceProviderStorePreflightState,
    sourcePlatformLimitationFallbackProofVersion: 'app-install-purchase-product-claim-platform-limitation-fallback-proof',
    sourcePlatformLimitationFallbackRowId: fallbackRow.platformLimitationFallbackRowId,
    sourcePlatformLimitationFallbackState: fallbackRow.fallbackState,
    platform: providerStoreRow.platform,
    storeSurface: providerStoreRow.storeSurface,
    providerStoreApiExecutionState: providerStoreApiExecutionStateGenerated(
      providerStoreRow.providerStoreProductClaimState,
      providerStoreRow.sourceProviderStorePreflightState,
      fallbackRow.fallbackState
    ),
    providerApiExecutionEvidenceRefs: providerStoreRow.requiredProviderStoreExecutionRefs,
    providerCredentialRequirementRefs: providerStoreRow.requiredProviderEvidenceRefs,
    fallbackParentWorkflowRefs: fallbackRow.fallbackParentWorkflowRefs,
    manualPlatformEvidenceRefs: fallbackRow.requiredManualPlatformEvidenceRefs,
    requiredPortalTestRefs: uniqueRefsGenerated([...providerStoreRow.requiredPortalTestRefs, ...fallbackRow.requiredPortalTestRefs]),
    requiredChildDeliveryRefs: uniqueRefsGenerated([
      ...providerStoreRow.requiredChildDeliveryRefs,
      ...fallbackRow.requiredChildDeliveryRefs,
    ]),
    requiredPlatformAdapterRefs: uniqueRefsGenerated([
      ...providerStoreRow.requiredPlatformAdapterRefs,
      ...fallbackRow.requiredPlatformAdapterRefs,
    ]),
    blockerRefs: uniqueRefsGenerated([...fallbackRow.limitationRefs, ...fallbackRow.requiredProviderStoreExecutionRefs]),
    auditEventRefs: uniqueRefsGenerated([...providerStoreRow.auditEventRefs, ...fallbackRow.auditEventRefs]),
    reportRuntimeRefs: uniqueRefsGenerated([...providerStoreRow.reportRuntimeRefs, ...fallbackRow.reportRuntimeRefs]),
    productClaimApprovalClaim: 'not-claimed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    portalApprovalUiClaim: 'not-claimed',
    portalReportUiClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    evaluatedAt,
  } as const;
}

export function providerStoreApiExecutionRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreRowId: string;
    readonly sourcePlatformLimitationFallbackRowId: string;
    readonly sourceProviderStoreProductClaimState: string;
    readonly sourceProviderStorePreflightState: string;
    readonly sourcePlatformLimitationFallbackState: string;
    readonly providerStoreApiExecutionState: string;
    readonly providerApiExecutionEvidenceRefs: readonly unknown[];
    readonly providerCredentialRequirementRefs: readonly unknown[];
    readonly fallbackParentWorkflowRefs: readonly unknown[];
    readonly manualPlatformEvidenceRefs: readonly unknown[];
    readonly requiredPortalTestRefs: readonly unknown[];
    readonly requiredChildDeliveryRefs: readonly unknown[];
    readonly requiredPlatformAdapterRefs: readonly unknown[];
    readonly blockerRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly productClaimApprovalClaim: string;
    readonly googlePlayExecutionClaim: string;
    readonly appleAppStoreExecutionClaim: string;
    readonly microsoftStoreExecutionClaim: string;
    readonly billingProviderContactClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.sourceProviderStoreRowId.length > 0 &&
    row.sourcePlatformLimitationFallbackRowId.length > 0 &&
    row.providerStoreApiExecutionState === providerStoreApiExecutionStateGenerated(
      row.sourceProviderStoreProductClaimState,
      row.sourceProviderStorePreflightState,
      row.sourcePlatformLimitationFallbackState
    ) &&
    row.providerApiExecutionEvidenceRefs.length > 0 &&
    row.providerCredentialRequirementRefs.length > 0 &&
    row.fallbackParentWorkflowRefs.length > 0 &&
    row.manualPlatformEvidenceRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.requiredPlatformAdapterRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function providerStoreApiExecutionProofIsHonestGenerated(
  proof: {
    readonly providerStoreApiExecutionRows: readonly { readonly platform: string; readonly storeSurface: string; readonly providerStoreApiExecutionState: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  storeSurfaces: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.providerStoreApiExecutionRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.providerStoreApiExecutionRows.map((row) => row.providerStoreApiExecutionState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.providerStoreApiExecutionRows.length === storeSurfaces.length &&
    keys.size === proof.providerStoreApiExecutionRows.length &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function providerStoreExecutionReadinessStateGenerated(platform: string) {
  if (platform === 'windows') return 'provider-store-execution-ready' as const;
  if (platform === 'linux') return 'unavailable' as const;
  return 'manual-required' as const;
}

export function summarizeAppInstallPurchaseProviderStoreExecutionReadinessProofGenerated(proof: {
  readonly providerStoreExecutionReadinessRows: readonly {
    readonly providerStoreExecutionReadinessState: string;
    readonly sourceParentActionDeliveryReadinessRefs: readonly unknown[];
    readonly packageSourceAdapterArtifactRefs: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    providerStoreExecutionReadinessRows: proof.providerStoreExecutionReadinessRows.length,
    executionReadyRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.providerStoreExecutionReadinessState === 'provider-store-execution-ready'
    ).length,
    manualRequiredRows: proof.providerStoreExecutionReadinessRows.filter((row) => row.providerStoreExecutionReadinessState === 'manual-required').length,
    unavailableRows: proof.providerStoreExecutionReadinessRows.filter((row) => row.providerStoreExecutionReadinessState === 'unavailable').length,
    packageSourceAdapterLinkedRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.packageSourceAdapterArtifactRefs.length > 0
    ).length,
    parentActionReadinessLinkedRows: proof.providerStoreExecutionReadinessRows.filter(
      (row) => row.sourceParentActionDeliveryReadinessRefs.length > 0
    ).length,
    providerExecutedRows: proof.providerStoreExecutionReadinessRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed').length,
    childDeliveredRows: proof.providerStoreExecutionReadinessRows.filter((row) => row.childDeviceDeliveryClaim !== 'not-delivered').length,
  } as const;
}

export function buildAppInstallPurchaseProviderStoreExecutionReadinessRowGenerated(
  storeStatusRow: {
    readonly storeStatusHandoffRowId: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly storeStatusHandoffState: string;
    readonly storeStatusHandoffEvidenceRefs: readonly string[];
    readonly sourceReportRuntimeRefs: readonly string[];
  },
  apiEntitlementRow: {
    readonly evidenceRowId: string;
    readonly evidenceStatus: string;
    readonly approvedApiEvidenceRef: string;
    readonly entitlementEvidenceRef: string;
    readonly requiredProofRefs: readonly string[];
  },
  packageSourceAdapterRow: {
    readonly packageSourceAdapterExecutionRowId: string;
    readonly adapterExecutionState: string;
    readonly adapterExecutionArtifactRefs: readonly string[];
    readonly reportRefs: readonly string[];
    readonly requiredProofRefs: readonly string[];
  },
  parentActionReadinessRows: ReadonlyArray<{
    readonly parentActionDeliveryReadinessRowId: string;
    readonly parentActionDeliveryReadinessState: string;
    readonly parentActionAuditEventRefs: readonly string[];
  }>,
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-provider-store-execution-readiness-proof',
    providerStoreExecutionReadinessRowId: `provider-store-execution-readiness-${storeStatusRow.platform}-${storeStatusRow.storeSurface}`,
    sourceApprovedApiEntitlementProofVersion: 'app-install-purchase-approved-api-entitlement-proof',
    sourceApprovedApiEntitlementRowId: apiEntitlementRow.evidenceRowId,
    sourceStoreStatusHandoffProofVersion: 'app-install-purchase-store-status-handoff-proof',
    sourceStoreStatusHandoffRowId: storeStatusRow.storeStatusHandoffRowId,
    sourcePackageSourceAdapterExecutionProofVersion: 'app-install-purchase-package-source-adapter-execution-proof',
    sourcePackageSourceAdapterExecutionRowId: packageSourceAdapterRow.packageSourceAdapterExecutionRowId,
    sourceParentActionDeliveryReadinessProofVersion: 'app-install-purchase-parent-action-delivery-readiness-proof',
    sourceParentActionDeliveryReadinessRefs: parentActionReadinessRows.map((readinessRow) => readinessRow.parentActionDeliveryReadinessRowId),
    sourceParentActionDeliveryReadinessStates: parentActionReadinessRows.map((readinessRow) => readinessRow.parentActionDeliveryReadinessState),
    platform: storeStatusRow.platform,
    storeSurface: storeStatusRow.storeSurface,
    sourceApiEntitlementEvidenceStatus: apiEntitlementRow.evidenceStatus,
    sourceStoreStatusHandoffState: storeStatusRow.storeStatusHandoffState,
    sourcePackageSourceAdapterExecutionState: packageSourceAdapterRow.adapterExecutionState,
    providerStoreExecutionReadinessState: providerStoreExecutionReadinessStateGenerated(storeStatusRow.platform),
    approvedApiEvidenceRefs: [apiEntitlementRow.approvedApiEvidenceRef],
    entitlementEvidenceRefs: [apiEntitlementRow.entitlementEvidenceRef],
    storeStatusHandoffEvidenceRefs: storeStatusRow.storeStatusHandoffEvidenceRefs,
    packageSourceAdapterArtifactRefs: packageSourceAdapterRow.adapterExecutionArtifactRefs,
    parentActionAuditEventRefs: parentActionReadinessRows.flatMap((readinessRow) => readinessRow.parentActionAuditEventRefs),
    reportRuntimeRefs: uniqueRefsGenerated([...storeStatusRow.sourceReportRuntimeRefs, ...packageSourceAdapterRow.reportRefs]),
    requiredProofRefs: uniqueRefsGenerated([...apiEntitlementRow.requiredProofRefs, ...packageSourceAdapterRow.requiredProofRefs]),
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    evaluatedAt,
  } as const;
}

export function providerStoreExecutionReadinessRowIsHonestGenerated(
  row: {
    readonly platform: string;
    readonly providerStoreExecutionReadinessState: string;
    readonly sourceApprovedApiEntitlementRowId: string;
    readonly sourceStoreStatusHandoffRowId: string;
    readonly sourcePackageSourceAdapterExecutionRowId: string;
    readonly sourceParentActionDeliveryReadinessRefs: readonly unknown[];
    readonly sourceParentActionDeliveryReadinessStates: readonly string[];
    readonly approvedApiEvidenceRefs: readonly unknown[];
    readonly entitlementEvidenceRefs: readonly unknown[];
    readonly storeStatusHandoffEvidenceRefs: readonly unknown[];
    readonly packageSourceAdapterArtifactRefs: readonly unknown[];
    readonly parentActionAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly requiredProofRefs: readonly unknown[];
    readonly googlePlayExecutionClaim: string;
    readonly appleAppStoreExecutionClaim: string;
    readonly microsoftStoreExecutionClaim: string;
    readonly billingProviderContactClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedParentActionReadinessCount: number,
  boundaryFragments: readonly string[]
) {
  const readinessStates = new Set(row.sourceParentActionDeliveryReadinessStates);
  return (
    row.providerStoreExecutionReadinessState === providerStoreExecutionReadinessStateGenerated(row.platform) &&
    row.sourceApprovedApiEntitlementRowId.length > 0 &&
    row.sourceStoreStatusHandoffRowId.length > 0 &&
    row.sourcePackageSourceAdapterExecutionRowId.length > 0 &&
    row.sourceParentActionDeliveryReadinessRefs.length === expectedParentActionReadinessCount &&
    readinessStates.has('parent-action-delivery-ready') &&
    readinessStates.has('manual-review-required') &&
    row.approvedApiEvidenceRefs.length > 0 &&
    row.entitlementEvidenceRefs.length > 0 &&
    row.storeStatusHandoffEvidenceRefs.length > 0 &&
    row.packageSourceAdapterArtifactRefs.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function providerStoreExecutionReadinessProofIsHonestGenerated(
  proof: {
    readonly providerStoreExecutionReadinessRows: readonly { readonly platform: string; readonly storeSurface: string; readonly providerStoreExecutionReadinessState: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.providerStoreExecutionReadinessRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.providerStoreExecutionReadinessRows.map((row) => row.providerStoreExecutionReadinessState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.providerStoreExecutionReadinessRows.length === requiredPlatformSources.length &&
    keys.size === proof.providerStoreExecutionReadinessRows.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefsGenerated(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
