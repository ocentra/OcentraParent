/* generated from crates/schema/src/app_install_purchase_platform_evidence_helpers.ts.txt */

export function summarizeAppInstallPurchasePlatformLimitationActionProofGenerated(proof: {
  readonly platformLimitationActionRows: readonly {
    readonly platformLimitationActionState: string;
    readonly sourceReportStatusReadModelRowIds: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
  }[];
}) {
  return {
    platformLimitationActionRows: proof.platformLimitationActionRows.length,
    readyRows: proof.platformLimitationActionRows.filter(
      (row) => row.platformLimitationActionState === 'parent-action-ready'
    ).length,
    manualRequiredRows: proof.platformLimitationActionRows.filter(
      (row) => row.platformLimitationActionState === 'manual-required'
    ).length,
    unavailableRows: proof.platformLimitationActionRows.filter(
      (row) => row.platformLimitationActionState === 'unavailable'
    ).length,
    reportStatusLinkedRows: proof.platformLimitationActionRows.filter(
      (row) => row.sourceReportStatusReadModelRowIds.length > 0
    ).length,
    providerExecutedRows: proof.platformLimitationActionRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    portalRows: proof.platformLimitationActionRows.filter(
      (row) =>
        row.portalApprovalUiClaim !== 'not-implemented' || row.portalReportUiClaim !== 'not-implemented'
    ).length,
  } as const;
}

export function platformLimitationActionStateGenerated(sourceState: string) {
  if (sourceState === 'provider-store-report-status-ready') {
    return 'parent-action-ready' as const;
  }
  if (sourceState === 'unavailable') {
    return 'unavailable' as const;
  }
  return 'manual-required' as const;
}

export function buildAppInstallPurchasePlatformLimitationActionRowGenerated(
  sourceRow: {
    readonly providerStoreReportStatusRowId: string;
    readonly providerStoreReportStatusState: string;
    readonly sourceAuditEventRefs: readonly string[];
    readonly platform: string;
    readonly storeSurface: string;
  },
  reportRows: ReadonlyArray<{
    readonly reportStatusReadModelRowId: string;
    readonly parentVisibleReportStatusState: string;
    readonly parentVisibleReportStatusRef: string;
    readonly reportAuditEventRefs: readonly string[];
  }>,
  sourceProviderStoreReportStatusProofVersion: string,
  sourceReportStatusReadModelProofVersion: string,
  claimBoundary: string,
  recordedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-platform-limitation-action-proof',
    platformLimitationActionRowId: `platform-limitation-action-${sourceRow.platform}-${sourceRow.storeSurface}`,
    sourceProviderStoreReportStatusProofVersion,
    sourceProviderStoreReportStatusRowId: sourceRow.providerStoreReportStatusRowId,
    sourceProviderStoreReportStatusState: sourceRow.providerStoreReportStatusState,
    sourceReportStatusReadModelProofVersion,
    sourceReportStatusReadModelRowIds: reportRows.map((row) => row.reportStatusReadModelRowId),
    sourceReportStatusReadModelStates: reportRows.map((row) => row.parentVisibleReportStatusState),
    parentVisibleReportStatusRefs: reportRows.map((row) => row.parentVisibleReportStatusRef),
    auditEventRefs: uniqueRefsGenerated([
      ...sourceRow.sourceAuditEventRefs,
      ...reportRows.flatMap((row) => row.reportAuditEventRefs),
    ]),
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    platformLimitationActionState: platformLimitationActionStateGenerated(sourceRow.providerStoreReportStatusState),
    parentLimitationActionRef: `parent-limitation-action-${sourceRow.platform}-${sourceRow.storeSurface}`,
    portalApprovalUiClaim: 'not-implemented',
    portalReportUiClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    billingProviderContactClaim: 'not-executed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    recordedAt,
  } as const;
}

export function platformLimitationActionRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreReportStatusState: string;
    readonly platformLimitationActionState: string;
    readonly sourceReportStatusReadModelRowIds: readonly unknown[];
    readonly sourceReportStatusReadModelStates: readonly string[];
    readonly parentVisibleReportStatusRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly billingProviderContactClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedReportRowCount: number,
  boundaryFragments: readonly string[]
) {
  const states = new Set(row.sourceReportStatusReadModelStates);
  return (
    row.platformLimitationActionState ===
      platformLimitationActionStateGenerated(row.sourceProviderStoreReportStatusState) &&
    row.sourceReportStatusReadModelRowIds.length === expectedReportRowCount &&
    states.has('parent-report-status-ready') &&
    states.has('manual-required') &&
    row.parentVisibleReportStatusRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function platformLimitationActionProofIsHonestGenerated(
  proof: {
    readonly platformLimitationActionRows: readonly { readonly platformLimitationActionState: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  expectedRowCount: number,
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const states = new Set(proof.platformLimitationActionRows.map((row) => row.platformLimitationActionState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.platformLimitationActionRows.length === expectedRowCount &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchasePlatformAdapterBoundaryProofGenerated(proof: {
  readonly adapterBoundaryRows: readonly {
    readonly adapterRuntimeState: string;
    readonly reportRuntimeRefs: readonly unknown[];
  }[];
}) {
  return {
    adapterBoundaryRows: proof.adapterBoundaryRows.length,
    notImplementedRows: proof.adapterBoundaryRows.filter((row) => row.adapterRuntimeState === 'not-implemented')
      .length,
    manualRequiredRows: proof.adapterBoundaryRows.filter((row) => row.adapterRuntimeState === 'manual-required')
      .length,
    unavailableRows: proof.adapterBoundaryRows.filter((row) => row.adapterRuntimeState === 'unavailable').length,
    reportRuntimeLinkedRows: proof.adapterBoundaryRows.filter((row) => row.reportRuntimeRefs.length > 0).length,
  } as const;
}

export function platformAdapterBoundaryEvidenceStateGenerated(status: string) {
  if (status === 'approved-api-evidence-required') {
    return 'approved-api-adapter-evidence-required' as const;
  }
  if (status === 'store-entitlement-evidence-required') {
    return 'entitlement-adapter-evidence-required' as const;
  }
  if (status === 'manual-platform-review-required') {
    return 'manual-platform-review-required' as const;
  }
  return 'platform-unavailable' as const;
}

export function platformAdapterBoundaryRuntimeStateGenerated(status: string) {
  if (status === 'platform-unavailable') {
    return 'unavailable' as const;
  }
  if (status === 'manual-platform-review-required') {
    return 'manual-required' as const;
  }
  return 'not-implemented' as const;
}

export function buildAppInstallPurchasePlatformAdapterBoundaryRowGenerated(
  sourceRow: {
    readonly evidenceRowId: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly evidenceStatus: string;
    readonly approvedApiEvidenceRef: string;
    readonly entitlementEvidenceRef: string;
    readonly limitationReportRef: string;
    readonly requiredProofRefs: readonly string[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
  },
  reportRuntimeRefs: readonly string[],
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-platform-adapter-boundary-proof',
    adapterBoundaryRowId: `platform-adapter-boundary-${sourceRow.platform}-${sourceRow.storeSurface}`,
    sourceApprovedApiEntitlementRowId: sourceRow.evidenceRowId,
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    adapterEvidenceState: platformAdapterBoundaryEvidenceStateGenerated(sourceRow.evidenceStatus),
    adapterRuntimeState: platformAdapterBoundaryRuntimeStateGenerated(sourceRow.evidenceStatus),
    approvedApiEvidenceRef: sourceRow.approvedApiEvidenceRef,
    entitlementEvidenceRef: sourceRow.entitlementEvidenceRef,
    limitationReportRef: sourceRow.limitationReportRef,
    reportRuntimeRefs,
    adapterReadinessEvidenceRefs: sourceRow.requiredProofRefs.map((proofRef) => `${proofRef}-adapter-readiness`),
    providerApiExecutionClaim: sourceRow.providerApiExecutionClaim,
    storeIntegrationClaim: sourceRow.storeIntegrationClaim,
    childDeliveryClaim: sourceRow.childDeliveryClaim,
    runtimeReportDeliveryClaim: sourceRow.runtimeReportDeliveryClaim,
    interceptionClaim: sourceRow.interceptionClaim,
    appBlockingClaim: sourceRow.appBlockingClaim,
    childDataCustody: sourceRow.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    evaluatedAt,
  } as const;
}

export function platformAdapterBoundaryRowIsHonestGenerated(
  row: {
    readonly adapterEvidenceState: string;
    readonly adapterRuntimeState: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly reportRuntimeRefs: readonly unknown[];
    readonly adapterReadinessEvidenceRefs: readonly unknown[];
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    ((row.adapterEvidenceState === 'platform-unavailable' && row.adapterRuntimeState === 'unavailable') ||
      (row.adapterEvidenceState === 'manual-platform-review-required' && row.adapterRuntimeState === 'manual-required') ||
      ((row.adapterEvidenceState === 'approved-api-adapter-evidence-required' ||
        row.adapterEvidenceState === 'entitlement-adapter-evidence-required') &&
        row.adapterRuntimeState === 'not-implemented')) &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    row.reportRuntimeRefs.length > 0 &&
    row.adapterReadinessEvidenceRefs.length > 0 &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function platformAdapterBoundaryProofIsHonestGenerated(
  proof: {
    readonly adapterBoundaryRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly adapterEvidenceState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredEvidenceStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.adapterBoundaryRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const evidenceStates = new Set(proof.adapterBoundaryRows.map((row) => row.adapterEvidenceState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.adapterBoundaryRows.length === requiredPlatformSources.length &&
    keys.size === proof.adapterBoundaryRows.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    requiredEvidenceStates.every((state) => evidenceStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchasePlatformAdapterEvidenceGapProofGenerated(proof: {
  readonly platformAdapterEvidenceGapRows: readonly {
    readonly platformAdapterEvidenceGapState: string;
    readonly realPlatformAdapterEvidenceState: string;
    readonly platformAdapterClaim: string;
    readonly productClaimApprovalClaim: string;
  }[];
}) {
  return {
    platformAdapterEvidenceGapRows: proof.platformAdapterEvidenceGapRows.length,
    adapterEvidenceGapRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'adapter-evidence-gap'
    ).length,
    manualAdapterEvidenceRequiredRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'manual-adapter-evidence-required'
    ).length,
    platformUnavailableRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterEvidenceGapState === 'blocked-before-claim'
    ).length,
    realAdapterEvidenceRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.realPlatformAdapterEvidenceState !== 'no-real-adapter-evidence-attached'
    ).length,
    adapterImplementedRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.platformAdapterClaim !== 'not-implemented'
    ).length,
    productClaimApprovedRows: proof.platformAdapterEvidenceGapRows.filter(
      (row) => row.productClaimApprovalClaim !== 'not-claimed'
    ).length,
  } as const;
}

export function platformAdapterEvidenceGapStateGenerated(
  sourceProviderStoreApiExecutionState: string,
  sourcePlatformProofReadinessState: string
) {
  if (
    sourceProviderStoreApiExecutionState === 'unavailable' ||
    sourcePlatformProofReadinessState === 'unavailable'
  ) {
    return 'platform-unavailable' as const;
  }
  if (
    sourceProviderStoreApiExecutionState === 'blocked-before-claim' ||
    sourcePlatformProofReadinessState === 'policy-blocked'
  ) {
    return 'blocked-before-claim' as const;
  }
  if (sourceProviderStoreApiExecutionState === 'manual-required') {
    return 'manual-adapter-evidence-required' as const;
  }
  return 'adapter-evidence-gap' as const;
}

export function platformAdapterEvidenceRefsGenerated(
  sourceRow: {
    readonly platform: string;
    readonly requiredPlatformAdapterRefs: readonly string[];
  },
  platformReadinessRow: {
    readonly requiredManualEvidenceRefs: readonly string[];
  }
) {
  const refs = {
    windows: ['windows-app-install-adapter-manual-proof', 'windows-store-source-adapter-boundary-proof'],
    macos: ['macos-app-install-adapter-manual-proof', 'macos-receipt-signing-adapter-proof'],
    linux: ['linux-package-manager-source-adapter-proof'],
    android: ['android-device-owner-managed-profile-adapter-proof', 'google-play-policy-adapter-proof'],
    ios: ['ios-family-controls-adapter-entitlement-proof', 'apple-review-platform-adapter-proof'],
  } as const;
  return uniqueRefsGenerated([
    ...sourceRow.requiredPlatformAdapterRefs,
    ...platformReadinessRow.requiredManualEvidenceRefs,
    ...refs[sourceRow.platform as keyof typeof refs],
  ]);
}

export function buildAppInstallPurchasePlatformAdapterEvidenceGapRowGenerated(
  sourceRow: {
    readonly platformAdapterEvidenceGapRowId?: string;
    readonly providerStoreApiExecutionRowId: string;
    readonly providerStoreApiExecutionState: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly providerApiExecutionEvidenceRefs: readonly string[];
    readonly requiredPlatformAdapterRefs: readonly string[];
    readonly manualPlatformEvidenceRefs: readonly string[];
    readonly providerCredentialRequirementRefs: readonly string[];
    readonly requiredPortalTestRefs: readonly string[];
    readonly requiredChildDeliveryRefs: readonly string[];
    readonly blockerRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  platformReadinessRow: {
    readonly platformProofReadinessState: string;
    readonly requiredManualEvidenceRefs: readonly string[];
  },
  sourceProviderStoreApiExecutionProofVersion: string,
  sourcePlatformProofReadinessProofVersion: string,
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-platform-adapter-evidence-gap-proof',
    platformAdapterEvidenceGapRowId:
      sourceRow.platformAdapterEvidenceGapRowId ??
      `app-install-platform-adapter-evidence-gap-${sourceRow.platform}-${sourceRow.storeSurface}`,
    sourceProviderStoreApiExecutionProofVersion,
    sourceProviderStoreApiExecutionRowId: sourceRow.providerStoreApiExecutionRowId,
    sourceProviderStoreApiExecutionState: sourceRow.providerStoreApiExecutionState,
    sourcePlatformProofReadinessProofVersion,
    sourcePlatformProofReadinessState: platformReadinessRow.platformProofReadinessState,
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    platformAdapterEvidenceGapState: platformAdapterEvidenceGapStateGenerated(
      sourceRow.providerStoreApiExecutionState,
      platformReadinessRow.platformProofReadinessState
    ),
    providerStoreApiExecutionEvidenceRefs: sourceRow.providerApiExecutionEvidenceRefs,
    requiredPlatformAdapterEvidenceRefs: platformAdapterEvidenceRefsGenerated(sourceRow, platformReadinessRow),
    requiredManualPlatformEvidenceRefs: uniqueRefsGenerated([
      ...sourceRow.manualPlatformEvidenceRefs,
      ...platformReadinessRow.requiredManualEvidenceRefs,
    ]),
    requiredProviderCredentialRefs: sourceRow.providerCredentialRequirementRefs,
    requiredPortalTestRefs: sourceRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: sourceRow.requiredChildDeliveryRefs,
    blockerRefs: uniqueRefsGenerated([...sourceRow.blockerRefs, ...sourceRow.requiredPlatformAdapterRefs]),
    auditEventRefs: sourceRow.auditEventRefs,
    reportRuntimeRefs: sourceRow.reportRuntimeRefs,
    realPlatformAdapterEvidenceState: 'no-real-adapter-evidence-attached',
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

export function platformAdapterEvidenceGapRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreApiExecutionRowId: string;
    readonly sourceProviderStoreApiExecutionState: string;
    readonly sourcePlatformProofReadinessState: string;
    readonly platformAdapterEvidenceGapState: string;
    readonly providerStoreApiExecutionEvidenceRefs: readonly unknown[];
    readonly requiredPlatformAdapterEvidenceRefs: readonly unknown[];
    readonly requiredManualPlatformEvidenceRefs: readonly unknown[];
    readonly requiredProviderCredentialRefs: readonly unknown[];
    readonly requiredPortalTestRefs: readonly unknown[];
    readonly requiredChildDeliveryRefs: readonly unknown[];
    readonly blockerRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly realPlatformAdapterEvidenceState: string;
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
    row.sourceProviderStoreApiExecutionRowId.length > 0 &&
    row.platformAdapterEvidenceGapState ===
      platformAdapterEvidenceGapStateGenerated(
        row.sourceProviderStoreApiExecutionState,
        row.sourcePlatformProofReadinessState
      ) &&
    row.providerStoreApiExecutionEvidenceRefs.length > 0 &&
    row.requiredPlatformAdapterEvidenceRefs.length > 0 &&
    row.requiredManualPlatformEvidenceRefs.length > 0 &&
    row.requiredProviderCredentialRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.realPlatformAdapterEvidenceState === 'no-real-adapter-evidence-attached' &&
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

export function platformAdapterEvidenceGapProofIsHonestGenerated(
  proof: {
    readonly platformAdapterEvidenceGapRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly platformAdapterEvidenceGapState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  storeSurfaces: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.platformAdapterEvidenceGapRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.platformAdapterEvidenceGapRows.map((row) => row.platformAdapterEvidenceGapState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.platformAdapterEvidenceGapRows.length === storeSurfaces.length &&
    keys.size === proof.platformAdapterEvidenceGapRows.length &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseProviderStoreExecutionPreflightProofGenerated(proof: {
  readonly providerStoreExecutionPreflightRows: readonly {
    readonly providerStoreExecutionPreflightState: string;
    readonly providerApiExecutionClaim: string;
    readonly runtimeDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    providerStoreExecutionPreflightRows: proof.providerStoreExecutionPreflightRows.length,
    preflightReadyRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerStoreExecutionPreflightState === 'preflight-ready'
    ).length,
    manualProviderProofRequiredRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerStoreExecutionPreflightState === 'manual-provider-proof-required'
    ).length,
    providerUnavailableRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerStoreExecutionPreflightState === 'provider-unavailable'
    ).length,
    providerExecutedRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    runtimeDeviceDeliveredRows: proof.providerStoreExecutionPreflightRows.filter(
      (row) => row.runtimeDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function providerStoreExecutionPreflightStateGenerated(sourceReadinessState: string) {
  if (sourceReadinessState === 'provider-store-execution-ready') {
    return 'preflight-ready' as const;
  }
  if (sourceReadinessState === 'unavailable') {
    return 'provider-unavailable' as const;
  }
  return 'manual-provider-proof-required' as const;
}

export function buildAppInstallPurchaseProviderStoreExecutionPreflightRowGenerated(
  sourceRow: {
    readonly providerStoreExecutionReadinessRowId: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly providerStoreExecutionReadinessState: string;
    readonly requiredProofRefs: readonly string[];
    readonly parentActionAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  runtimeWriterRows: ReadonlyArray<{
    readonly runtimeWriterExecutionDeliveryRowId: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly deliveryResultReceiptRef: string;
    readonly deliveryResultAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  }>,
  sourceProviderStoreExecutionReadinessProofVersion: string,
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-provider-store-execution-preflight-proof',
    providerStoreExecutionPreflightRowId: `provider-store-execution-preflight-${sourceRow.platform}-${sourceRow.storeSurface}`,
    sourceProviderStoreExecutionReadinessProofVersion,
    sourceProviderStoreExecutionReadinessRowId: sourceRow.providerStoreExecutionReadinessRowId,
    sourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryRowIds: runtimeWriterRows.map(
      (row) => row.runtimeWriterExecutionDeliveryRowId
    ),
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    sourceProviderStoreExecutionReadinessState: sourceRow.providerStoreExecutionReadinessState,
    sourceRuntimeWriterReceiptClaims: runtimeWriterRows.map((row) => row.runtimeWriterDeliveryClaim),
    providerStoreExecutionPreflightState: providerStoreExecutionPreflightStateGenerated(
      sourceRow.providerStoreExecutionReadinessState
    ),
    requiredProviderEvidenceRefs: sourceRow.requiredProofRefs,
    runtimeWriterReceiptRefs: runtimeWriterRows.map((row) => row.deliveryResultReceiptRef),
    auditEventRefs: uniqueRefsGenerated([
      ...sourceRow.parentActionAuditEventRefs,
      ...runtimeWriterRows.flatMap((row) => row.deliveryResultAuditEventRefs),
    ]),
    reportRuntimeRefs: uniqueRefsGenerated([
      ...sourceRow.reportRuntimeRefs,
      ...runtimeWriterRows.flatMap((row) => row.reportRuntimeRefs),
    ]),
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeDeviceDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    evaluatedAt,
  } as const;
}

export function providerStoreExecutionPreflightRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreExecutionReadinessRowId: string;
    readonly sourceProviderStoreExecutionReadinessState: string;
    readonly sourceRuntimeWriterExecutionDeliveryRowIds: readonly unknown[];
    readonly sourceRuntimeWriterReceiptClaims: readonly string[];
    readonly providerStoreExecutionPreflightState: string;
    readonly requiredProviderEvidenceRefs: readonly unknown[];
    readonly runtimeWriterReceiptRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly googlePlayExecutionClaim: string;
    readonly appleAppStoreExecutionClaim: string;
    readonly microsoftStoreExecutionClaim: string;
    readonly billingProviderContactClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly runtimeDeviceDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedRuntimeWriterRowCount: number,
  boundaryFragments: readonly string[]
) {
  return (
    row.sourceProviderStoreExecutionReadinessRowId.length > 0 &&
    row.providerStoreExecutionPreflightState ===
      providerStoreExecutionPreflightStateGenerated(row.sourceProviderStoreExecutionReadinessState) &&
    row.sourceRuntimeWriterExecutionDeliveryRowIds.length === expectedRuntimeWriterRowCount &&
    row.sourceRuntimeWriterReceiptClaims.includes('parent-owned-delivery-result-recorded') &&
    row.sourceRuntimeWriterReceiptClaims.includes('manual-required') &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.runtimeDeviceDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function providerStoreExecutionPreflightProofIsHonestGenerated(
  proof: {
    readonly providerStoreExecutionPreflightRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly providerStoreExecutionPreflightState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.providerStoreExecutionPreflightRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(
    proof.providerStoreExecutionPreflightRows.map((row) => row.providerStoreExecutionPreflightState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.providerStoreExecutionPreflightRows.length === requiredPlatformSources.length &&
    keys.size === proof.providerStoreExecutionPreflightRows.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseProviderStoreManualEvidencePacketProofGenerated(proof: {
  readonly manualEvidencePacketRows: readonly {
    readonly manualEvidencePacketState: string;
    readonly providerApiExecutionClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    manualEvidencePacketRows: proof.manualEvidencePacketRows.length,
    packetReadyRows: proof.manualEvidencePacketRows.filter(
      (row) => row.manualEvidencePacketState === 'manual-evidence-packet-ready'
    ).length,
    manualReviewRequiredRows: proof.manualEvidencePacketRows.filter(
      (row) => row.manualEvidencePacketState === 'manual-review-required'
    ).length,
    providerUnavailableRows: proof.manualEvidencePacketRows.filter(
      (row) => row.manualEvidencePacketState === 'provider-unavailable'
    ).length,
    providerExecutedRows: proof.manualEvidencePacketRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.manualEvidencePacketRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function manualEvidencePacketStateGenerated(platformState: string, preflightState: string) {
  if (platformState === 'unavailable' || preflightState === 'provider-unavailable') {
    return 'provider-unavailable' as const;
  }
  if (platformState === 'manual-proof-required' && preflightState === 'preflight-ready') {
    return 'manual-evidence-packet-ready' as const;
  }
  return 'manual-review-required' as const;
}

export function buildAppInstallPurchaseProviderStoreManualEvidencePacketRowGenerated(
  preflightRow: {
    readonly providerStoreExecutionPreflightRowId: string;
    readonly providerStoreExecutionPreflightState: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly requiredProviderEvidenceRefs: readonly string[];
    readonly runtimeWriterReceiptRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  platformRow: {
    readonly platformProofReadinessState: string;
    readonly requiredManualEvidenceRefs: readonly string[];
  },
  sourcePlatformProofReadinessVersion: string,
  sourceProviderStorePreflightVersion: string,
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-provider-store-manual-evidence-packet-proof',
    manualEvidencePacketRowId: `provider-store-manual-evidence-packet-${preflightRow.platform}-${preflightRow.storeSurface}`,
    sourcePlatformProofReadinessVersion,
    sourcePlatformProofReadinessState: platformRow.platformProofReadinessState,
    sourceProviderStorePreflightVersion,
    sourceProviderStorePreflightRowId: preflightRow.providerStoreExecutionPreflightRowId,
    sourceProviderStorePreflightState: preflightRow.providerStoreExecutionPreflightState,
    platform: preflightRow.platform,
    storeSurface: preflightRow.storeSurface,
    manualEvidencePacketState: manualEvidencePacketStateGenerated(
      platformRow.platformProofReadinessState,
      preflightRow.providerStoreExecutionPreflightState
    ),
    requiredManualEvidenceRefs: platformRow.requiredManualEvidenceRefs,
    requiredProviderEvidenceRefs: preflightRow.requiredProviderEvidenceRefs,
    runtimeWriterReceiptRefs: preflightRow.runtimeWriterReceiptRefs,
    auditEventRefs: preflightRow.auditEventRefs,
    reportRuntimeRefs: preflightRow.reportRuntimeRefs,
    providerApiExecutionClaim: 'not-executed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeWriterDeliveryClaim: 'not-delivered',
    runtimeReportDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    evaluatedAt,
  } as const;
}

export function providerStoreManualEvidencePacketRowIsHonestGenerated(
  row: {
    readonly sourceProviderStorePreflightRowId: string;
    readonly sourcePlatformProofReadinessState: string;
    readonly sourceProviderStorePreflightState: string;
    readonly manualEvidencePacketState: string;
    readonly requiredManualEvidenceRefs: readonly unknown[];
    readonly requiredProviderEvidenceRefs: readonly unknown[];
    readonly runtimeWriterReceiptRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly googlePlayExecutionClaim: string;
    readonly appleAppStoreExecutionClaim: string;
    readonly microsoftStoreExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.sourceProviderStorePreflightRowId.length > 0 &&
    row.manualEvidencePacketState ===
      manualEvidencePacketStateGenerated(
        row.sourcePlatformProofReadinessState,
        row.sourceProviderStorePreflightState
      ) &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.requiredProviderEvidenceRefs.length > 0 &&
    row.runtimeWriterReceiptRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function providerStoreManualEvidencePacketProofIsHonestGenerated(
  proof: {
    readonly manualEvidencePacketRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly manualEvidencePacketState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  storeSurfaces: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.manualEvidencePacketRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.manualEvidencePacketRows.map((row) => row.manualEvidencePacketState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.manualEvidencePacketRows.length === storeSurfaces.length &&
    keys.size === proof.manualEvidencePacketRows.length &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseProviderStorePlatformEvidenceProofGenerated(proof: {
  readonly providerStorePlatformEvidenceRows: readonly {
    readonly providerStorePlatformEvidenceState: string;
    readonly providerApiExecutionClaim: string;
    readonly productionPlatformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    providerStorePlatformEvidenceRows: proof.providerStorePlatformEvidenceRows.length,
    manualRequiredRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerStorePlatformEvidenceState === 'manual-provider-store-platform-evidence-required'
    ).length,
    platformUnavailableRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerStorePlatformEvidenceState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerStorePlatformEvidenceState === 'blocked-before-claim'
    ).length,
    providerExecutedRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    platformAdapterImplementedRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.productionPlatformAdapterClaim !== 'not-implemented'
    ).length,
    childDeliveredRows: proof.providerStorePlatformEvidenceRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function providerStorePlatformEvidenceStateGenerated(
  preflightState: string,
  runtimeHandoffState: string
) {
  if (preflightState === 'provider-unavailable' || runtimeHandoffState === 'platform-unavailable') {
    return 'platform-unavailable' as const;
  }
  if (runtimeHandoffState === 'blocked-before-claim') {
    return 'blocked-before-claim' as const;
  }
  return 'manual-provider-store-platform-evidence-required' as const;
}

export function missingProviderStorePlatformArtifactsGenerated(platform: string) {
  const refs = {
    windows: {
      providerStore: [
        'missing-microsoft-store-provider-credential-proof',
        'missing-microsoft-store-provider-api-response-proof',
        'missing-billing-provider-contact-proof',
      ],
      platform: [
        'missing-windows-production-platform-adapter-execution-proof',
        'missing-windows-platform-interception-policy-proof',
      ],
      childDevice: ['missing-windows-child-device-delivery-receipt-proof'],
    },
    macos: {
      providerStore: [
        'missing-mac-app-store-credential-proof',
        'missing-mac-app-store-receipt-response-proof',
        'missing-billing-provider-contact-proof',
      ],
      platform: ['missing-macos-signing-receipt-entitlement-proof', 'missing-macos-platform-adapter-execution-proof'],
      childDevice: ['missing-macos-child-device-delivery-receipt-proof'],
    },
    linux: {
      providerStore: ['missing-linux-package-manager-provider-proof'],
      platform: ['missing-tested-linux-distro-package-manager-source-proof'],
      childDevice: ['missing-linux-child-device-delivery-receipt-proof'],
    },
    android: {
      providerStore: ['missing-google-play-api-policy-proof', 'missing-google-play-provider-response-proof'],
      platform: ['missing-android-device-owner-managed-profile-proof', 'missing-android-platform-adapter-execution-proof'],
      childDevice: ['missing-android-child-device-delivery-receipt-proof'],
    },
    ios: {
      providerStore: ['missing-apple-app-store-family-controls-evidence-proof', 'missing-apple-review-proof'],
      platform: ['missing-ios-family-controls-entitlement-proof', 'missing-ios-platform-adapter-execution-proof'],
      childDevice: ['missing-ios-child-device-delivery-receipt-proof'],
    },
  } as const;
  return refs[platform as keyof typeof refs];
}

export function buildAppInstallPurchaseProviderStorePlatformEvidenceRowGenerated(
  preflightRow: {
    readonly providerStoreExecutionPreflightRowId: string;
    readonly providerStoreExecutionPreflightState: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly requiredProviderEvidenceRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  runtimeHandoffRow: {
    readonly runtimeHandoffRowId: string;
    readonly runtimeHandoffState: string;
    readonly packageSourceEvidenceRefs: readonly string[];
    readonly requiredPortalTestRefs: readonly string[];
    readonly blockerRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  sourceProviderStoreExecutionPreflightProofVersion: string,
  sourceWindowsPackageSourceRuntimeHandoffProofVersion: string,
  claimBoundary: string,
  evaluatedAt: string
) {
  const missingArtifacts = missingProviderStorePlatformArtifactsGenerated(preflightRow.platform);
  return {
    schemaVersion: 'app-install-purchase-provider-store-platform-evidence-proof',
    providerStorePlatformEvidenceRowId: `provider-store-platform-evidence-${preflightRow.platform}-${preflightRow.storeSurface}`,
    sourceProviderStoreExecutionPreflightProofVersion,
    sourceProviderStoreExecutionPreflightRowId: preflightRow.providerStoreExecutionPreflightRowId,
    sourceProviderStoreExecutionPreflightState: preflightRow.providerStoreExecutionPreflightState,
    sourceWindowsPackageSourceRuntimeHandoffProofVersion,
    sourceWindowsPackageSourceRuntimeHandoffRowId: runtimeHandoffRow.runtimeHandoffRowId,
    sourceRuntimeHandoffState: runtimeHandoffRow.runtimeHandoffState,
    platform: preflightRow.platform,
    storeSurface: preflightRow.storeSurface,
    providerStorePlatformEvidenceState: providerStorePlatformEvidenceStateGenerated(
      preflightRow.providerStoreExecutionPreflightState,
      runtimeHandoffRow.runtimeHandoffState
    ),
    packageSourceEvidenceRefs: runtimeHandoffRow.packageSourceEvidenceRefs,
    providerStorePreflightRefs: preflightRow.requiredProviderEvidenceRefs,
    missingProviderStoreArtifactRefs: missingArtifacts.providerStore,
    missingPlatformArtifactRefs: missingArtifacts.platform,
    missingChildDeviceArtifactRefs: missingArtifacts.childDevice,
    requiredPortalTestRefs: runtimeHandoffRow.requiredPortalTestRefs,
    blockerRefs: uniqueRefsGenerated([
      ...preflightRow.requiredProviderEvidenceRefs,
      ...runtimeHandoffRow.blockerRefs,
      ...missingArtifacts.providerStore,
      ...missingArtifacts.platform,
      ...missingArtifacts.childDevice,
    ]),
    auditEventRefs: uniqueRefsGenerated([...preflightRow.auditEventRefs, ...runtimeHandoffRow.auditEventRefs]),
    reportRuntimeRefs: uniqueRefsGenerated([
      ...preflightRow.reportRuntimeRefs,
      ...runtimeHandoffRow.reportRuntimeRefs,
    ]),
    productClaimApprovalClaim: 'not-claimed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    microsoftStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    productionPlatformAdapterClaim: 'not-implemented',
    runtimeWriterExecutionClaim: 'not-executed',
    runtimeWriterDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
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

export function providerStorePlatformEvidenceRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreExecutionPreflightRowId: string;
    readonly sourceProviderStoreExecutionPreflightState: string;
    readonly sourceWindowsPackageSourceRuntimeHandoffRowId: string;
    readonly sourceRuntimeHandoffState: string;
    readonly providerStorePlatformEvidenceState: string;
    readonly packageSourceEvidenceRefs: readonly unknown[];
    readonly providerStorePreflightRefs: readonly unknown[];
    readonly missingProviderStoreArtifactRefs: readonly unknown[];
    readonly missingPlatformArtifactRefs: readonly unknown[];
    readonly missingChildDeviceArtifactRefs: readonly unknown[];
    readonly requiredPortalTestRefs: readonly unknown[];
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
    readonly productionPlatformAdapterClaim: string;
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
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
    row.sourceProviderStoreExecutionPreflightRowId.length > 0 &&
    row.sourceWindowsPackageSourceRuntimeHandoffRowId.length > 0 &&
    row.providerStorePlatformEvidenceState ===
      providerStorePlatformEvidenceStateGenerated(
        row.sourceProviderStoreExecutionPreflightState,
        row.sourceRuntimeHandoffState
      ) &&
    row.packageSourceEvidenceRefs.length > 0 &&
    row.providerStorePreflightRefs.length > 0 &&
    row.missingProviderStoreArtifactRefs.length > 0 &&
    row.missingPlatformArtifactRefs.length > 0 &&
    row.missingChildDeviceArtifactRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
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
    row.productionPlatformAdapterClaim === 'not-implemented' &&
    row.runtimeWriterExecutionClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function providerStorePlatformEvidenceProofIsHonestGenerated(
  proof: {
    readonly providerStorePlatformEvidenceRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly providerStorePlatformEvidenceState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  storeSurfaces: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.providerStorePlatformEvidenceRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.providerStorePlatformEvidenceRows.map((row) => row.providerStorePlatformEvidenceState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.providerStorePlatformEvidenceRows.length === storeSurfaces.length &&
    keys.size === proof.providerStorePlatformEvidenceRows.length &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseWindowsPackageSourceAdapterEvidenceProofGenerated(proof: {
  readonly windowsPackageSourceAdapterEvidenceRows: readonly {
    readonly hostEvidenceState: string;
    readonly providerApiExecutionClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    windowsPackageSourceAdapterEvidenceRows: proof.windowsPackageSourceAdapterEvidenceRows.length,
    windowsHostEvidenceCollectedRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'windows-host-evidence-collected'
    ).length,
    windowsHostManualRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'windows-host-manual-required'
    ).length,
    manualAdapterEvidenceRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'manual-adapter-evidence-required'
    ).length,
    platformUnavailableRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.hostEvidenceState === 'blocked-before-claim'
    ).length,
    providerExecutedRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.windowsPackageSourceAdapterEvidenceRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function summarizeAppInstallPurchaseWindowsPackageSourceRuntimeHandoffProofGenerated(proof: {
  readonly runtimeHandoffRows: readonly {
    readonly runtimeHandoffState: string;
    readonly providerApiExecutionClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    runtimeHandoffRows: proof.runtimeHandoffRows.length,
    windowsRuntimeHandoffReadyRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffState === 'windows-runtime-handoff-ready'
    ).length,
    windowsRuntimeHandoffManualRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffState === 'windows-runtime-handoff-manual-required'
    ).length,
    manualRuntimeHandoffRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffState === 'manual-runtime-handoff-required'
    ).length,
    platformUnavailableRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffState === 'platform-unavailable'
    ).length,
    blockedBeforeClaimRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffState === 'blocked-before-claim'
    ).length,
    providerExecutedRows: proof.runtimeHandoffRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.runtimeHandoffRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function windowsPackageSourceHostEvidenceStateGenerated(
  sourcePlatformAdapterEvidenceGapState: string,
  platform: string,
  commandAvailable: boolean
) {
  if (platform === 'windows') {
    return commandAvailable ? 'windows-host-evidence-collected' as const : 'windows-host-manual-required' as const;
  }
  if (sourcePlatformAdapterEvidenceGapState === 'platform-unavailable') {
    return 'platform-unavailable' as const;
  }
  if (sourcePlatformAdapterEvidenceGapState === 'blocked-before-claim') {
    return 'blocked-before-claim' as const;
  }
  return 'manual-adapter-evidence-required' as const;
}

export function windowsPackageSourceHostEvidenceRefsGenerated(
  platform: string,
  hostEvidenceArtifactRef: string,
  requiredPlatformAdapterEvidenceRefs: readonly string[]
) {
  if (platform === 'windows') {
    return [hostEvidenceArtifactRef];
  }
  return requiredPlatformAdapterEvidenceRefs;
}

export function buildAppInstallPurchaseWindowsPackageSourceAdapterEvidenceRowGenerated(
  gapRow: {
    readonly platformAdapterEvidenceGapRowId: string;
    readonly platformAdapterEvidenceGapState: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly requiredManualPlatformEvidenceRefs: readonly string[];
    readonly blockerRefs: readonly string[];
    readonly providerStoreApiExecutionEvidenceRefs: readonly string[];
    readonly requiredProviderCredentialRefs: readonly string[];
    readonly requiredPortalTestRefs: readonly string[];
    readonly requiredChildDeliveryRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly requiredPlatformAdapterEvidenceRefs: readonly string[];
  },
  adapterExecutionRow: {
    readonly packageSourceAdapterExecutionRowId: string;
    readonly adapterExecutionState: string;
    readonly requiredProofRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRefs: readonly string[];
  },
  hostEvidenceArtifact: {
    readonly artifactRef: string;
    readonly commandAvailable: boolean;
  },
  sourcePlatformAdapterEvidenceGapProofVersion: string,
  sourcePackageSourceAdapterExecutionProofVersion: string,
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-windows-package-source-adapter-evidence',
    windowsPackageSourceAdapterEvidenceRowId: `windows-package-source-adapter-evidence-${gapRow.platform}-${gapRow.storeSurface}`,
    sourcePlatformAdapterEvidenceGapProofVersion,
    sourcePlatformAdapterEvidenceGapRowId: gapRow.platformAdapterEvidenceGapRowId,
    sourcePlatformAdapterEvidenceGapState: gapRow.platformAdapterEvidenceGapState,
    sourcePackageSourceAdapterExecutionProofVersion,
    sourcePackageSourceAdapterExecutionRowId: adapterExecutionRow.packageSourceAdapterExecutionRowId,
    sourcePackageSourceAdapterExecutionState: adapterExecutionRow.adapterExecutionState,
    platform: gapRow.platform,
    storeSurface: gapRow.storeSurface,
    hostEvidenceState: windowsPackageSourceHostEvidenceStateGenerated(
      gapRow.platformAdapterEvidenceGapState,
      gapRow.platform,
      hostEvidenceArtifact.commandAvailable
    ),
    hostEvidenceArtifactRefs: windowsPackageSourceHostEvidenceRefsGenerated(
      gapRow.platform,
      hostEvidenceArtifact.artifactRef,
      gapRow.requiredPlatformAdapterEvidenceRefs
    ),
    requiredManualEvidenceRefs: uniqueRefsGenerated([
      ...gapRow.requiredManualPlatformEvidenceRefs,
      ...gapRow.blockerRefs,
    ]),
    requiredProviderStoreEvidenceRefs: uniqueRefsGenerated([
      ...gapRow.providerStoreApiExecutionEvidenceRefs,
      ...gapRow.requiredProviderCredentialRefs,
    ]),
    requiredPortalTestRefs: gapRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: gapRow.requiredChildDeliveryRefs,
    blockerRefs: uniqueRefsGenerated([...gapRow.blockerRefs, ...adapterExecutionRow.requiredProofRefs]),
    auditEventRefs: uniqueRefsGenerated([...gapRow.auditEventRefs, ...adapterExecutionRow.auditEventRefs]),
    reportRuntimeRefs: uniqueRefsGenerated([...gapRow.reportRuntimeRefs, ...adapterExecutionRow.reportRefs]),
    productClaimApprovalClaim: 'not-claimed',
    microsoftStoreExecutionClaim: 'not-executed',
    googlePlayExecutionClaim: 'not-executed',
    appleAppStoreExecutionClaim: 'not-executed',
    billingProviderContactClaim: 'not-executed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    productionPlatformAdapterClaim: 'not-implemented',
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

export function windowsPackageSourceRuntimeHandoffStateGenerated(hostEvidenceState: string) {
  if (hostEvidenceState === 'windows-host-evidence-collected') {
    return 'windows-runtime-handoff-ready' as const;
  }
  if (hostEvidenceState === 'windows-host-manual-required') {
    return 'windows-runtime-handoff-manual-required' as const;
  }
  if (hostEvidenceState === 'platform-unavailable') {
    return 'platform-unavailable' as const;
  }
  if (hostEvidenceState === 'blocked-before-claim') {
    return 'blocked-before-claim' as const;
  }
  return 'manual-runtime-handoff-required' as const;
}

export function windowsPackageSourceRuntimeProbeStatusGenerated(
  platform: string,
  hostEvidenceState: string,
  commandAvailable: boolean
) {
  if (platform === 'windows') {
    return commandAvailable ? 'sanitized-command-available' as const : 'sanitized-command-unavailable' as const;
  }
  if (hostEvidenceState === 'platform-unavailable') {
    return 'unavailable' as const;
  }
  if (hostEvidenceState === 'blocked-before-claim') {
    return 'blocked-before-claim' as const;
  }
  return 'manual-required' as const;
}

export function buildAppInstallPurchaseWindowsPackageSourceRuntimeHandoffRowGenerated(
  sourceRow: {
    readonly windowsPackageSourceAdapterEvidenceRowId: string;
    readonly hostEvidenceState: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly hostEvidenceArtifactRefs: readonly string[];
    readonly requiredManualEvidenceRefs: readonly string[];
    readonly requiredProviderStoreEvidenceRefs: readonly string[];
    readonly requiredPortalTestRefs: readonly string[];
    readonly requiredChildDeliveryRefs: readonly string[];
    readonly blockerRefs: readonly string[];
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  hostEvidenceArtifact: {
    readonly commandAvailable: boolean;
  },
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-windows-package-source-adapter-evidence',
    runtimeHandoffRowId: `windows-package-source-runtime-handoff-${sourceRow.platform}-${sourceRow.storeSurface}`,
    sourceWindowsPackageSourceAdapterEvidenceRowId: sourceRow.windowsPackageSourceAdapterEvidenceRowId,
    sourceWindowsPackageSourceAdapterEvidenceState: sourceRow.hostEvidenceState,
    platform: sourceRow.platform,
    storeSurface: sourceRow.storeSurface,
    runtimeHandoffState: windowsPackageSourceRuntimeHandoffStateGenerated(sourceRow.hostEvidenceState),
    sanitizedCommandProbeStatus: windowsPackageSourceRuntimeProbeStatusGenerated(
      sourceRow.platform,
      sourceRow.hostEvidenceState,
      hostEvidenceArtifact.commandAvailable
    ),
    packageSourceEvidenceRefs: sourceRow.hostEvidenceArtifactRefs,
    requiredManualEvidenceRefs: sourceRow.requiredManualEvidenceRefs,
    requiredProviderStoreEvidenceRefs: sourceRow.requiredProviderStoreEvidenceRefs,
    requiredPortalTestRefs: sourceRow.requiredPortalTestRefs,
    requiredChildDeliveryRefs: sourceRow.requiredChildDeliveryRefs,
    blockerRefs: sourceRow.blockerRefs,
    auditEventRefs: sourceRow.auditEventRefs,
    reportRuntimeRefs: sourceRow.reportRuntimeRefs,
    productClaimApprovalClaim: 'not-claimed',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformInterceptionClaim: 'not-claimed',
    productionPlatformAdapterClaim: 'not-implemented',
    runtimeWriterExecutionClaim: 'not-executed',
    runtimeWriterDeliveryClaim: 'not-delivered',
    childDeviceDeliveryClaim: 'not-delivered',
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

export function windowsPackageSourceAdapterEvidenceRowIsHonestGenerated(
  row: {
    readonly sourcePlatformAdapterEvidenceGapRowId: string;
    readonly sourcePlatformAdapterEvidenceGapState: string;
    readonly sourcePackageSourceAdapterExecutionRowId: string;
    readonly platform: string;
    readonly hostEvidenceState: string;
    readonly hostEvidenceArtifactRefs: readonly unknown[];
    readonly requiredManualEvidenceRefs: readonly unknown[];
    readonly requiredProviderStoreEvidenceRefs: readonly unknown[];
    readonly requiredPortalTestRefs: readonly unknown[];
    readonly requiredChildDeliveryRefs: readonly unknown[];
    readonly blockerRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly productClaimApprovalClaim: string;
    readonly microsoftStoreExecutionClaim: string;
    readonly googlePlayExecutionClaim: string;
    readonly appleAppStoreExecutionClaim: string;
    readonly billingProviderContactClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly productionPlatformAdapterClaim: string;
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
    row.sourcePlatformAdapterEvidenceGapRowId.length > 0 &&
    row.sourcePackageSourceAdapterExecutionRowId.length > 0 &&
    row.hostEvidenceArtifactRefs.length > 0 &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.requiredProviderStoreEvidenceRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    ((row.platform === 'windows' &&
      (row.hostEvidenceState === 'windows-host-evidence-collected' ||
        row.hostEvidenceState === 'windows-host-manual-required')) ||
      (row.sourcePlatformAdapterEvidenceGapState === 'platform-unavailable' &&
        row.hostEvidenceState === 'platform-unavailable') ||
      (row.sourcePlatformAdapterEvidenceGapState === 'blocked-before-claim' &&
        row.hostEvidenceState === 'blocked-before-claim') ||
      (row.sourcePlatformAdapterEvidenceGapState === 'manual-adapter-evidence-required' &&
        row.hostEvidenceState === 'manual-adapter-evidence-required') ||
      (row.sourcePlatformAdapterEvidenceGapState === 'adapter-evidence-gap' &&
        row.hostEvidenceState === 'manual-adapter-evidence-required')) &&
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.microsoftStoreExecutionClaim === 'not-executed' &&
    row.googlePlayExecutionClaim === 'not-executed' &&
    row.appleAppStoreExecutionClaim === 'not-executed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.productionPlatformAdapterClaim === 'not-implemented' &&
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

export function windowsPackageSourceAdapterEvidenceProofIsHonestGenerated(
  proof: {
    readonly windowsPackageSourceAdapterEvidenceRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly hostEvidenceState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  storeSurfaces: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(
    proof.windowsPackageSourceAdapterEvidenceRows.map((row) => `${row.platform}:${row.storeSurface}`)
  );
  const states = new Set(proof.windowsPackageSourceAdapterEvidenceRows.map((row) => row.hostEvidenceState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.windowsPackageSourceAdapterEvidenceRows.length === storeSurfaces.length &&
    keys.size === proof.windowsPackageSourceAdapterEvidenceRows.length &&
    (states.has('windows-host-evidence-collected') !== states.has('windows-host-manual-required')) &&
    states.has('manual-adapter-evidence-required') &&
    states.has('platform-unavailable') &&
    states.has('blocked-before-claim') &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function windowsPackageSourceRuntimeHandoffRowIsHonestGenerated(
  row: {
    readonly sourceWindowsPackageSourceAdapterEvidenceRowId: string;
    readonly sourceWindowsPackageSourceAdapterEvidenceState: string;
    readonly runtimeHandoffState: string;
    readonly packageSourceEvidenceRefs: readonly unknown[];
    readonly requiredManualEvidenceRefs: readonly unknown[];
    readonly requiredProviderStoreEvidenceRefs: readonly unknown[];
    readonly requiredPortalTestRefs: readonly unknown[];
    readonly requiredChildDeliveryRefs: readonly unknown[];
    readonly blockerRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly productClaimApprovalClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly productionPlatformAdapterClaim: string;
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
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
    row.sourceWindowsPackageSourceAdapterEvidenceRowId.length > 0 &&
    row.runtimeHandoffState ===
      windowsPackageSourceRuntimeHandoffStateGenerated(row.sourceWindowsPackageSourceAdapterEvidenceState) &&
    row.packageSourceEvidenceRefs.length > 0 &&
    row.requiredManualEvidenceRefs.length > 0 &&
    row.requiredProviderStoreEvidenceRefs.length > 0 &&
    row.requiredPortalTestRefs.length > 0 &&
    row.requiredChildDeliveryRefs.length > 0 &&
    row.blockerRefs.length > 0 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.productClaimApprovalClaim === 'not-claimed' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.productionPlatformAdapterClaim === 'not-implemented' &&
    row.runtimeWriterExecutionClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.portalApprovalUiClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function windowsPackageSourceRuntimeHandoffProofIsHonestGenerated(
  proof: {
    readonly runtimeHandoffRows: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly runtimeHandoffState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  storeSurfaces: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.runtimeHandoffRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.runtimeHandoffRows.map((row) => row.runtimeHandoffState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.runtimeHandoffRows.length === storeSurfaces.length &&
    keys.size === proof.runtimeHandoffRows.length &&
    (states.has('windows-runtime-handoff-ready') !== states.has('windows-runtime-handoff-manual-required')) &&
    states.has('manual-runtime-handoff-required') &&
    states.has('platform-unavailable') &&
    states.has('blocked-before-claim') &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefsGenerated(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
