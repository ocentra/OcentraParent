/* generated from crates/schema/src/app_install_purchase_delivery_runtime_helpers.ts.txt */

function uniqueRefsGenerated(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

export function summarizeAppInstallPurchaseChildArtifactDeliveryProofGenerated(proof: {
  readonly childPackageArtifacts: readonly { readonly childArtifactSourceState: string }[];
  readonly childDeliveryBoundaries: readonly { readonly childDeliveryClaim: string }[];
}) {
  return {
    childArtifactRows: proof.childPackageArtifacts.length,
    childDeliveryRows: proof.childDeliveryBoundaries.length,
    attachedChildArtifactRefs: proof.childPackageArtifacts.filter(
      (row) => row.childArtifactSourceState === 'child-package-artifact-ref-attached'
    ).length,
    unavailableChildArtifactRows: proof.childPackageArtifacts.filter(
      (row) => row.childArtifactSourceState === 'platform-unavailable'
    ).length,
    notDeliveredRows: proof.childDeliveryBoundaries.filter((row) => row.childDeliveryClaim === 'not-delivered').length,
  } as const;
}

export function buildAppInstallPurchaseChildPackageArtifactRowGenerated(
  row: {
    readonly platform: string;
    readonly storeSurface: string;
    readonly artifactRowId: string;
    readonly packageSourceArtifactRowId: string;
    readonly artifactRef: string;
    readonly sourcePackageArtifactState: string;
    readonly providerApiClaim: string;
    readonly platformAdapterClaim: string;
    readonly storeIntegrationClaim: string;
    readonly reportRefs: readonly string[];
    readonly requiredProofRefs: readonly string[];
  },
  schemaVersion: string,
  claimBoundary: string,
  attachedAt: string
) {
  return {
    schemaVersion,
    childArtifactRowId: `child-package-artifact-${row.platform}-${row.storeSurface}`,
    platform: row.platform,
    storeSurface: row.storeSurface,
    platformArtifactRowId: row.artifactRowId,
    packageSourceArtifactRowId: row.packageSourceArtifactRowId,
    platformArtifactRef: row.artifactRef,
    childPackageArtifactRef: `child-package-source-${row.platform}-${row.storeSurface}-artifact-ref`,
    packageSourceArtifactState: row.sourcePackageArtifactState,
    childArtifactSourceState:
      row.sourcePackageArtifactState === 'platform-unavailable'
        ? 'platform-unavailable'
        : 'child-package-artifact-ref-attached',
    childArtifactCaptureClaim: 'not-runtime-captured',
    deliveryState: row.sourcePackageArtifactState === 'platform-unavailable' ? 'unavailable' : 'manual-required',
    childDeliveryClaim: 'not-delivered',
    providerApiClaim: row.providerApiClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    interceptionClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    reportRefs: row.reportRefs,
    requiredProofRefs: row.requiredProofRefs,
    claimBoundary,
    attachedAt,
  } as const;
}

export function buildAppInstallPurchaseChildDeliveryBoundaryRowGenerated(
  row: {
    readonly childStateId: string;
    readonly requestId: string;
    readonly platform: string;
    readonly childVisibleStatus: string;
    readonly deliveryState: string;
    readonly runtimeDeliveryClaim: string;
    readonly auditEventRefs: readonly string[];
    readonly reportRefs: readonly string[];
  },
  schemaVersion: string,
  claimBoundary: string,
  attachedAt: string
) {
  return {
    schemaVersion,
    deliveryRowId: `child-delivery-boundary-${row.childVisibleStatus}`,
    sourceChildStateId: row.childStateId,
    requestId: row.requestId,
    platform: row.platform,
    childVisibleStatus: row.childVisibleStatus,
    deliveryState: row.deliveryState,
    childArtifactRef: `child-delivery-${row.childVisibleStatus}-artifact-ref`,
    childDeliveryClaim: row.runtimeDeliveryClaim,
    providerApiClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    auditEventRefs: row.auditEventRefs,
    reportRefs: row.reportRefs,
    claimBoundary,
    attachedAt,
  } as const;
}

export function childPackageArtifactRowIsHonestGenerated(
  row: {
    readonly packageSourceArtifactState: string;
    readonly childArtifactSourceState: string;
    readonly deliveryState: string;
    readonly childArtifactCaptureClaim: string;
    readonly childDeliveryClaim: string;
    readonly providerApiClaim: string;
    readonly platformAdapterClaim: string;
    readonly storeIntegrationClaim: string;
    readonly interceptionClaim: string;
    readonly childDataCustody: string;
    readonly reportRefs: readonly unknown[];
    readonly requiredProofRefs: readonly unknown[];
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.packageSourceArtifactState === 'platform-unavailable'
      ? row.childArtifactSourceState === 'platform-unavailable' && row.deliveryState === 'unavailable'
      : row.childArtifactSourceState === 'child-package-artifact-ref-attached' &&
        row.deliveryState === 'manual-required';
  return (
    stateMatches &&
    row.childArtifactCaptureClaim === 'not-runtime-captured' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.interceptionClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.reportRefs.length > 0 &&
    row.requiredProofRefs.length > 0 &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function childDeliveryBoundaryRowIsHonestGenerated(
  row: {
    readonly deliveryState: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly providerApiClaim: string;
    readonly platformAdapterClaim: string;
    readonly appBlockingClaim: string;
    readonly auditEventRefs: readonly unknown[];
    readonly reportRefs: readonly unknown[];
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.deliveryState === 'manual-required' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.auditEventRefs.length > 0 &&
    row.reportRefs.length > 0 &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function childArtifactDeliveryProofIsHonestGenerated(
  proof: {
    readonly sourcePlatformArtifactProofVersion: string;
    readonly sourceRuntimeProofVersion: string;
    readonly childPackageArtifacts: readonly {
      readonly platform: string;
      readonly storeSurface: string;
      readonly childVisibleStatus?: string;
    }[];
    readonly childDeliveryBoundaries: readonly { readonly childVisibleStatus: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourcePlatformArtifactProofVersion: string,
  sourceRuntimeProofVersion: string,
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredChildStatuses: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const platformKeys = new Set(
    proof.childPackageArtifacts.map((row) => `${row.platform}:${row.storeSurface}`)
  );
  const statuses = new Set(proof.childDeliveryBoundaries.map((row) => row.childVisibleStatus));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourcePlatformArtifactProofVersion === sourcePlatformArtifactProofVersion &&
    proof.sourceRuntimeProofVersion === sourceRuntimeProofVersion &&
    proof.childPackageArtifacts.length === requiredPlatformSources.length &&
    platformKeys.size === proof.childPackageArtifacts.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => platformKeys.has(`${platform}:${storeSurface}`)) &&
    proof.childDeliveryBoundaries.length === requiredChildStatuses.length &&
    requiredChildStatuses.every((status) => statuses.has(status)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofGenerated(proof: {
  readonly childDeviceDeliveryRuntimeWriterRows: readonly {
    readonly childDeliveryEnvelopeState: string;
    readonly sourcePackageSourceCaptureRefs: readonly unknown[];
    readonly runtimeWriterExecutionClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    childDeviceDeliveryRuntimeWriterRows: proof.childDeviceDeliveryRuntimeWriterRows.length,
    childDeliveryEnvelopeReadyRows: proof.childDeviceDeliveryRuntimeWriterRows.filter(
      (row) => row.childDeliveryEnvelopeState === 'child-delivery-envelope-ready'
    ).length,
    manualReviewRequiredRows: proof.childDeviceDeliveryRuntimeWriterRows.filter(
      (row) => row.childDeliveryEnvelopeState === 'manual-review-required'
    ).length,
    packageSourceCaptureLinkedRows: proof.childDeviceDeliveryRuntimeWriterRows.filter(
      (row) => row.sourcePackageSourceCaptureRefs.length > 0
    ).length,
    runtimeWriterExecutedRows: proof.childDeviceDeliveryRuntimeWriterRows.filter(
      (row) => row.runtimeWriterExecutionClaim !== 'not-executed'
    ).length,
    childDeviceDeliveredRows: proof.childDeviceDeliveryRuntimeWriterRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseChildDeviceDeliveryRuntimeWriterRowGenerated(
  row: {
    readonly runtimeWriterDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeWriterDeliveryState: string;
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly runtimeWriterDeliveryClaim: string;
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  sourceRuntimeWriterDeliveryProofVersion: string,
  sourcePackageSourceCaptureStatusProofVersion: string,
  sourcePackageSourceCaptureRefs: readonly string[],
  sourcePackageSourceCaptureStatuses: readonly string[],
  childDeliveryTargetRefs: readonly string[],
  packageSourceAuditEventRefs: readonly string[],
  claimBoundary: string,
  linkedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-child-device-delivery-runtime-writer-proof',
    childDeviceDeliveryRuntimeWriterRowId: `child-device-delivery-runtime-writer-${row.sourceDecisionAction}`,
    sourceRuntimeWriterDeliveryProofVersion,
    sourceRuntimeWriterDeliveryRowId: row.runtimeWriterDeliveryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeWriterDeliveryState: row.runtimeWriterDeliveryState,
    sourcePackageSourceCaptureStatusProofVersion,
    sourcePackageSourceCaptureRefs,
    sourcePackageSourceCaptureStatuses,
    childDeliveryEnvelopeState: manual ? 'manual-review-required' : 'child-delivery-envelope-ready',
    childDeliveryTargetRefs,
    runtimeWriterAuditEventRefs: row.auditEventRefs,
    packageSourceAuditEventRefs,
    reportRuntimeRefs: row.reportRuntimeRefs,
    runtimeWriterExecutionClaim: 'not-executed',
    runtimeWriterDeliveryClaim: row.runtimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function childDeviceDeliveryRuntimeWriterRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeWriterDeliveryState: string;
    readonly sourcePackageSourceCaptureStatusProofVersion: string;
    readonly sourcePackageSourceCaptureRefs: readonly unknown[];
    readonly sourcePackageSourceCaptureStatuses: readonly string[];
    readonly childDeliveryEnvelopeState: string;
    readonly childDeliveryTargetRefs: readonly unknown[];
    readonly runtimeWriterAuditEventRefs: readonly unknown[];
    readonly packageSourceAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourcePackageSourceCaptureStatusProofVersion: string,
  requiredPackageSourceCaptureStatuses: readonly string[],
  boundaryFragments: readonly string[]
) {
  const statuses = new Set(row.sourcePackageSourceCaptureStatuses);
  const envelopeMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.sourceRuntimeWriterDeliveryState === 'manual-review-required' &&
        row.childDeliveryEnvelopeState === 'manual-review-required'
      : row.sourceRuntimeWriterDeliveryState === 'writer-envelope-ready' &&
        row.childDeliveryEnvelopeState === 'child-delivery-envelope-ready';
  return (
    envelopeMatches &&
    row.sourcePackageSourceCaptureStatusProofVersion === sourcePackageSourceCaptureStatusProofVersion &&
    row.sourcePackageSourceCaptureRefs.length === row.sourcePackageSourceCaptureStatuses.length &&
    requiredPackageSourceCaptureStatuses.every((status) => statuses.has(status)) &&
    row.childDeliveryTargetRefs.length > 0 &&
    row.runtimeWriterAuditEventRefs.length > 0 &&
    row.packageSourceAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.runtimeWriterExecutionClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function childDeviceDeliveryRuntimeWriterProofIsHonestGenerated(
  proof: {
    readonly sourceRuntimeWriterDeliveryProofVersion: string;
    readonly sourcePackageSourceCaptureStatusProofVersion: string;
    readonly childDeviceDeliveryRuntimeWriterRows: readonly {
      readonly sourceDecisionAction: string;
      readonly childDeliveryEnvelopeState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceRuntimeWriterDeliveryProofVersion: string,
  sourcePackageSourceCaptureStatusProofVersion: string,
  requiredDecisionActions: readonly string[],
  requiredChildDeliveryEnvelopeStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(
    proof.childDeviceDeliveryRuntimeWriterRows.map((row) => row.sourceDecisionAction)
  );
  const envelopeStates = new Set(
    proof.childDeviceDeliveryRuntimeWriterRows.map((row) => row.childDeliveryEnvelopeState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterDeliveryProofVersion === sourceRuntimeWriterDeliveryProofVersion &&
    proof.sourcePackageSourceCaptureStatusProofVersion === sourcePackageSourceCaptureStatusProofVersion &&
    proof.childDeviceDeliveryRuntimeWriterRows.length === requiredDecisionActions.length &&
    requiredDecisionActions.every((action) => actions.has(action)) &&
    requiredChildDeliveryEnvelopeStates.every((state) => envelopeStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseChildDeviceDeliveryReadinessProofGenerated(proof: {
  readonly childDeviceDeliveryReadinessRows: readonly {
    readonly childDeviceDeliveryReadinessState: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    childDeviceDeliveryReadinessRows: proof.childDeviceDeliveryReadinessRows.length,
    deliveryEvidenceReadyRows: proof.childDeviceDeliveryReadinessRows.filter(
      (row) => row.childDeviceDeliveryReadinessState === 'delivery-evidence-ready'
    ).length,
    manualProofRequiredRows: proof.childDeviceDeliveryReadinessRows.filter(
      (row) => row.childDeviceDeliveryReadinessState === 'manual-proof-required'
    ).length,
    platformUnavailableRows: proof.childDeviceDeliveryReadinessRows.filter(
      (row) => row.childDeviceDeliveryReadinessState === 'platform-unavailable'
    ).length,
    policyBlockedRows: proof.childDeviceDeliveryReadinessRows.filter(
      (row) => row.childDeviceDeliveryReadinessState === 'policy-blocked'
    ).length,
    childDeviceDeliveredRows: proof.childDeviceDeliveryReadinessRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function childDeviceDeliveryReadinessStateGenerated(platform: string) {
  if (platform === 'windows') return 'delivery-evidence-ready' as const;
  if (platform === 'macos') return 'manual-proof-required' as const;
  if (platform === 'linux') return 'platform-unavailable' as const;
  return 'policy-blocked' as const;
}

export function buildAppInstallPurchaseChildDeviceDeliveryReadinessRowGenerated(
  row: {
    readonly platform: string;
    readonly platformLimitationActionRowId: string;
    readonly parentLimitationActionRef: string;
    readonly parentVisibleReportStatusRefs: readonly string[];
  },
  adapterRow: {
    readonly packageSourceAdapterExecutionRowId: string;
    readonly requiredProofRefs: readonly string[];
  },
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  sourceChildDeliveryRuntimeWriterRowIds: readonly string[],
  sourcePackageSourceAdapterExecutionProofVersion: string,
  sourcePlatformLimitationActionProofVersion: string,
  claimBoundary: string,
  recordedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-child-device-delivery-readiness-proof',
    childDeviceDeliveryReadinessRowId: `child-device-delivery-readiness-${row.platform}`,
    platform: row.platform,
    childDeviceDeliveryReadinessState: childDeviceDeliveryReadinessStateGenerated(row.platform),
    sourceChildDeviceDeliveryRuntimeWriterProofVersion,
    sourceChildDeliveryRuntimeWriterRowIds,
    sourcePackageSourceAdapterExecutionProofVersion,
    sourcePackageSourceAdapterExecutionRowId: adapterRow.packageSourceAdapterExecutionRowId,
    sourcePlatformLimitationActionProofVersion,
    sourcePlatformLimitationActionRowId: row.platformLimitationActionRowId,
    requiredDeliveryProofRefs: [
      ...adapterRow.requiredProofRefs,
      row.parentLimitationActionRef,
      `child-device-delivery-proof-required-${row.platform}`,
    ],
    parentVisibleStatusRefs: [...row.parentVisibleReportStatusRefs, row.parentLimitationActionRef],
    childDeviceDeliveryClaim: 'not-delivered',
    runtimeWriterExecutionClaim: 'not-executed',
    runtimeWriterDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    recordedAt,
  } as const;
}

export function childDeviceDeliveryReadinessRowIsHonestGenerated(
  row: {
    readonly sourceChildDeliveryRuntimeWriterRowIds: readonly unknown[];
    readonly requiredDeliveryProofRefs: readonly unknown[];
    readonly parentVisibleStatusRefs: readonly unknown[];
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  return (
    row.sourceChildDeliveryRuntimeWriterRowIds.length >= 4 &&
    row.requiredDeliveryProofRefs.length > 0 &&
    row.parentVisibleStatusRefs.length > 0 &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterExecutionClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function childDeviceDeliveryReadinessProofIsHonestGenerated(
  proof: {
    readonly childDeviceDeliveryReadinessRows: readonly {
      readonly platform: string;
      readonly childDeviceDeliveryReadinessState: string;
    }[];
    readonly nonClaims: readonly string[];
  },
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const platforms = new Set(proof.childDeviceDeliveryReadinessRows.map((row) => row.platform));
  const nonClaims = new Set(proof.nonClaims);
  return (
    platforms.size === 5 &&
    requiredStates.every((state) =>
      proof.childDeviceDeliveryReadinessRows.some((row) => row.childDeviceDeliveryReadinessState === state)
    ) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim))
  );
}

export function summarizeAppInstallPurchaseParentActionRuntimeHandoffProofGenerated(proof: {
  readonly runtimeHandoffRows: readonly {
    readonly runtimeHandoffStatus: string;
    readonly platformAdapterBoundaryRefs: readonly unknown[];
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly childDeliveryClaim: string;
  }[];
}) {
  return {
    runtimeHandoffRows: proof.runtimeHandoffRows.length,
    queuedRuntimeWriterRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffStatus === 'queued-for-runtime-writer'
    ).length,
    manualReviewRequiredRows: proof.runtimeHandoffRows.filter(
      (row) => row.runtimeHandoffStatus === 'manual-review-required'
    ).length,
    platformBoundaryLinkedRows: proof.runtimeHandoffRows.filter(
      (row) => row.platformAdapterBoundaryRefs.length > 0
    ).length,
    runtimeDeliveredRows: proof.runtimeHandoffRows.filter(
      (row) => row.parentActionRuntimeDeliveryClaim !== 'not-delivered'
    ).length,
    childDeliveredRows: proof.runtimeHandoffRows.filter((row) => row.childDeliveryClaim !== 'not-delivered').length,
  } as const;
}

export function buildAppInstallPurchaseParentActionRuntimeHandoffRowGenerated(
  row: {
    readonly parentReviewActionRowId: string;
    readonly sourceDecisionAction: string;
    readonly sourceRequestKind: string;
    readonly resultingApprovalState: string;
    readonly parentActionReferenceId: string | null;
    readonly auditEventRefs: readonly string[];
    readonly sourceReportRuntimeRefs: readonly string[];
    readonly portalApprovalUiClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  sourceParentReviewActionProofVersion: string,
  sourcePlatformAdapterBoundaryProofVersion: string,
  platformAdapterBoundaryRefs: readonly string[],
  claimBoundary: string,
  linkedAt: string
) {
  const reviewNeeded = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-parent-action-runtime-handoff-proof',
    runtimeHandoffRowId: `parent-action-runtime-handoff-${row.sourceDecisionAction}`,
    sourceParentReviewActionProofVersion,
    sourceParentReviewActionRowId: row.parentReviewActionRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRequestKind: row.sourceRequestKind,
    resultingApprovalState: row.resultingApprovalState,
    parentActionReferenceId: row.parentActionReferenceId,
    runtimeHandoffStatus: reviewNeeded ? 'manual-review-required' : 'queued-for-runtime-writer',
    runtimeActionWriterClaim: reviewNeeded ? 'manual-required' : 'not-implemented',
    parentActionRuntimeDeliveryClaim: 'not-delivered',
    sourcePlatformAdapterBoundaryProofVersion,
    platformAdapterBoundaryRefs,
    auditEventRefs: row.auditEventRefs,
    reportRuntimeRefs: row.sourceReportRuntimeRefs,
    portalApprovalUiClaim: row.portalApprovalUiClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function parentActionRuntimeHandoffRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly parentActionReferenceId: string | null;
    readonly runtimeHandoffStatus: string;
    readonly runtimeActionWriterClaim: string;
    readonly sourceParentReviewActionProofVersion: string;
    readonly sourcePlatformAdapterBoundaryProofVersion: string;
    readonly platformAdapterBoundaryRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly portalApprovalUiClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceParentReviewActionProofVersion: string,
  sourcePlatformAdapterBoundaryProofVersion: string,
  expectedPlatformAdapterRefCount: number,
  boundaryFragments: readonly string[]
) {
  const actionMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.parentActionReferenceId === null &&
        row.runtimeHandoffStatus === 'manual-review-required' &&
        row.runtimeActionWriterClaim === 'manual-required'
      : row.parentActionReferenceId !== null &&
        row.runtimeHandoffStatus === 'queued-for-runtime-writer' &&
        row.runtimeActionWriterClaim === 'not-implemented';
  return (
    actionMatches &&
    row.sourceParentReviewActionProofVersion === sourceParentReviewActionProofVersion &&
    row.sourcePlatformAdapterBoundaryProofVersion === sourcePlatformAdapterBoundaryProofVersion &&
    row.platformAdapterBoundaryRefs.length === expectedPlatformAdapterRefCount &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function parentActionRuntimeHandoffProofIsHonestGenerated(
  proof: {
    readonly sourceParentReviewActionProofVersion: string;
    readonly sourcePlatformAdapterBoundaryProofVersion: string;
    readonly runtimeHandoffRows: readonly {
      readonly sourceDecisionAction: string;
      readonly runtimeHandoffStatus: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceParentReviewActionProofVersion: string,
  sourcePlatformAdapterBoundaryProofVersion: string,
  requiredActions: readonly string[],
  requiredStatuses: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.runtimeHandoffRows.map((row) => row.sourceDecisionAction));
  const statuses = new Set(proof.runtimeHandoffRows.map((row) => row.runtimeHandoffStatus));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentReviewActionProofVersion === sourceParentReviewActionProofVersion &&
    proof.sourcePlatformAdapterBoundaryProofVersion === sourcePlatformAdapterBoundaryProofVersion &&
    proof.runtimeHandoffRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredStatuses.every((status) => statuses.has(status)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseParentActionDeliveryReadinessProofGenerated(proof: {
  readonly parentActionDeliveryReadinessRows: readonly {
    readonly parentActionDeliveryReadinessState: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterRowId: string;
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly runtimeWriterExecutionClaim: string;
  }[];
}) {
  return {
    parentActionDeliveryReadinessRows: proof.parentActionDeliveryReadinessRows.length,
    parentActionDeliveryReadyRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.parentActionDeliveryReadinessState === 'parent-action-delivery-ready'
    ).length,
    manualReviewRequiredRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.parentActionDeliveryReadinessState === 'manual-review-required'
    ).length,
    childEnvelopeLinkedRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.sourceChildDeviceDeliveryRuntimeWriterRowId.length > 0
    ).length,
    parentActionDeliveredRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.parentActionRuntimeDeliveryClaim !== 'not-delivered'
    ).length,
    runtimeWriterExecutedRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.runtimeWriterExecutionClaim !== 'not-executed'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseParentActionDeliveryReadinessRowGenerated(
  row: {
    readonly runtimeHandoffRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeHandoffStatus: string;
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  childEnvelopeRow: {
    readonly childDeviceDeliveryRuntimeWriterRowId: string;
    readonly childDeliveryEnvelopeState: string;
    readonly childDeliveryTargetRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
  },
  sourceParentActionRuntimeHandoffProofVersion: string,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  claimBoundary: string,
  linkedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-parent-action-delivery-readiness-proof',
    parentActionDeliveryReadinessRowId: `parent-action-delivery-readiness-${row.sourceDecisionAction}`,
    sourceParentActionRuntimeHandoffProofVersion,
    sourceParentActionRuntimeHandoffRowId: row.runtimeHandoffRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeHandoffStatus: row.runtimeHandoffStatus,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterRowId: childEnvelopeRow.childDeviceDeliveryRuntimeWriterRowId,
    sourceChildDeliveryEnvelopeState: childEnvelopeRow.childDeliveryEnvelopeState,
    parentActionDeliveryReadinessState: manual ? 'manual-review-required' : 'parent-action-delivery-ready',
    parentActionAuditEventRefs: row.auditEventRefs,
    childDeliveryTargetRefs: childEnvelopeRow.childDeliveryTargetRefs,
    reportRuntimeRefs: uniqueRefsGenerated([...row.reportRuntimeRefs, ...childEnvelopeRow.reportRuntimeRefs]),
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    runtimeWriterExecutionClaim: childEnvelopeRow.runtimeWriterExecutionClaim,
    runtimeWriterDeliveryClaim: childEnvelopeRow.runtimeWriterDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: childEnvelopeRow.platformAdapterClaim,
    childDeviceDeliveryClaim: childEnvelopeRow.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: childEnvelopeRow.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function parentActionDeliveryReadinessRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeHandoffStatus: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterProofVersion: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterRowId: string;
    readonly sourceChildDeliveryEnvelopeState: string;
    readonly parentActionDeliveryReadinessState: string;
    readonly parentActionAuditEventRefs: readonly unknown[];
    readonly childDeliveryTargetRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.sourceRuntimeHandoffStatus === 'manual-review-required' &&
        row.sourceChildDeliveryEnvelopeState === 'manual-review-required' &&
        row.parentActionDeliveryReadinessState === 'manual-review-required'
      : row.sourceRuntimeHandoffStatus === 'queued-for-runtime-writer' &&
        row.sourceChildDeliveryEnvelopeState === 'child-delivery-envelope-ready' &&
        row.parentActionDeliveryReadinessState === 'parent-action-delivery-ready';
  return (
    stateMatches &&
    row.sourceChildDeviceDeliveryRuntimeWriterProofVersion === sourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    row.sourceChildDeviceDeliveryRuntimeWriterRowId.length > 0 &&
    row.childDeliveryTargetRefs.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterExecutionClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function parentActionDeliveryReadinessProofIsHonestGenerated(
  proof: {
    readonly sourceParentActionRuntimeHandoffProofVersion: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterProofVersion: string;
    readonly parentActionDeliveryReadinessRows: readonly {
      readonly sourceDecisionAction: string;
      readonly sourceRuntimeHandoffStatus: string;
      readonly parentActionDeliveryReadinessState: string;
      readonly sourceChildDeliveryEnvelopeState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceParentActionRuntimeHandoffProofVersion: string,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  requiredActions: readonly string[],
  requiredRuntimeHandoffStatuses: readonly string[],
  requiredParentActionDeliveryReadinessStates: readonly string[],
  requiredChildDeliveryEnvelopeStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(
    proof.parentActionDeliveryReadinessRows.map((row) => row.sourceDecisionAction)
  );
  const sourceStatuses = new Set(
    proof.parentActionDeliveryReadinessRows.map((row) => row.sourceRuntimeHandoffStatus)
  );
  const readinessStates = new Set(
    proof.parentActionDeliveryReadinessRows.map((row) => row.parentActionDeliveryReadinessState)
  );
  const childEnvelopeStates = new Set(
    proof.parentActionDeliveryReadinessRows.map((row) => row.sourceChildDeliveryEnvelopeState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentActionRuntimeHandoffProofVersion === sourceParentActionRuntimeHandoffProofVersion &&
    proof.sourceChildDeviceDeliveryRuntimeWriterProofVersion === sourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    proof.parentActionDeliveryReadinessRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredRuntimeHandoffStatuses.every((status) => sourceStatuses.has(status)) &&
    requiredParentActionDeliveryReadinessStates.every((state) => readinessStates.has(state)) &&
    requiredChildDeliveryEnvelopeStates.every((state) => childEnvelopeStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseRuntimeWriterDeliveryProofGenerated(proof: {
  readonly runtimeWriterDeliveryRows: readonly {
    readonly runtimeWriterDeliveryState: string;
    readonly sourceStoreStatusHandoffRefs: readonly unknown[];
    readonly runtimeWriterImplementationClaim: string;
    readonly parentActionRuntimeDeliveryClaim: string;
  }[];
}) {
  return {
    runtimeWriterDeliveryRows: proof.runtimeWriterDeliveryRows.length,
    writerEnvelopeReadyRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.runtimeWriterDeliveryState === 'writer-envelope-ready'
    ).length,
    manualReviewRequiredRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.runtimeWriterDeliveryState === 'manual-review-required'
    ).length,
    storeStatusLinkedRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.sourceStoreStatusHandoffRefs.length > 0
    ).length,
    writerImplementedRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.runtimeWriterImplementationClaim !== 'not-implemented'
    ).length,
    runtimeDeliveredRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.parentActionRuntimeDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseRuntimeWriterDeliveryRowGenerated(
  row: {
    readonly runtimeHandoffRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeHandoffStatus: string;
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  sourceParentActionRuntimeHandoffProofVersion: string,
  sourceStoreStatusHandoffProofVersion: string,
  sourceStoreStatusHandoffRefs: readonly string[],
  sourceStoreStatusHandoffStates: readonly string[],
  storeStatusHandoffEvidenceRefs: readonly string[],
  reportRuntimeRefs: readonly string[],
  claimBoundary: string,
  linkedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-runtime-writer-delivery-proof',
    runtimeWriterDeliveryRowId: `runtime-writer-delivery-${row.sourceDecisionAction}`,
    sourceParentActionRuntimeHandoffProofVersion,
    sourceParentActionRuntimeHandoffRowId: row.runtimeHandoffRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeHandoffStatus: row.runtimeHandoffStatus,
    sourceStoreStatusHandoffProofVersion,
    sourceStoreStatusHandoffRefs,
    sourceStoreStatusHandoffStates,
    storeStatusHandoffEvidenceRefs,
    auditEventRefs: row.auditEventRefs,
    reportRuntimeRefs,
    runtimeWriterDeliveryState: manual ? 'manual-review-required' : 'writer-envelope-ready',
    runtimeWriterQueueState: manual ? 'manual-required' : 'not-implemented',
    runtimeWriterImplementationClaim: 'not-implemented',
    runtimeWriterDeliveryClaim: 'not-delivered',
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    storeStatusHandoffDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function runtimeWriterDeliveryRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeHandoffStatus: string;
    readonly sourceStoreStatusHandoffProofVersion: string;
    readonly sourceStoreStatusHandoffRefs: readonly unknown[];
    readonly sourceStoreStatusHandoffStates: readonly string[];
    readonly storeStatusHandoffEvidenceRefs: readonly unknown[];
    readonly auditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly runtimeWriterDeliveryState: string;
    readonly runtimeWriterQueueState: string;
    readonly runtimeWriterImplementationClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly storeStatusHandoffDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceStoreStatusHandoffProofVersion: string,
  expectedStoreStatusRowCount: number,
  requiredStoreStatusHandoffStates: readonly string[],
  boundaryFragments: readonly string[]
) {
  const storeStates = new Set(row.sourceStoreStatusHandoffStates);
  const parentActionMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.sourceRuntimeHandoffStatus === 'manual-review-required' &&
        row.runtimeWriterDeliveryState === 'manual-review-required' &&
        row.runtimeWriterQueueState === 'manual-required'
      : row.sourceRuntimeHandoffStatus === 'queued-for-runtime-writer' &&
        row.runtimeWriterDeliveryState === 'writer-envelope-ready' &&
        row.runtimeWriterQueueState === 'not-implemented';
  return (
    parentActionMatches &&
    row.sourceStoreStatusHandoffProofVersion === sourceStoreStatusHandoffProofVersion &&
    row.sourceStoreStatusHandoffRefs.length === expectedStoreStatusRowCount &&
    row.sourceStoreStatusHandoffStates.length === expectedStoreStatusRowCount &&
    requiredStoreStatusHandoffStates.every((state) => storeStates.has(state)) &&
    row.storeStatusHandoffEvidenceRefs.length >= 4 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.runtimeWriterImplementationClaim === 'not-implemented' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.storeStatusHandoffDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function runtimeWriterDeliveryProofIsHonestGenerated(
  proof: {
    readonly sourceParentActionRuntimeHandoffProofVersion: string;
    readonly sourceStoreStatusHandoffProofVersion: string;
    readonly runtimeWriterDeliveryRows: readonly {
      readonly sourceDecisionAction: string;
      readonly sourceRuntimeHandoffStatus: string;
      readonly runtimeWriterDeliveryState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceParentActionRuntimeHandoffProofVersion: string,
  sourceStoreStatusHandoffProofVersion: string,
  requiredDecisionActions: readonly string[],
  requiredSourceRuntimeHandoffStatuses: readonly string[],
  requiredRuntimeWriterDeliveryStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.runtimeWriterDeliveryRows.map((row) => row.sourceDecisionAction));
  const sourceStatuses = new Set(
    proof.runtimeWriterDeliveryRows.map((row) => row.sourceRuntimeHandoffStatus)
  );
  const deliveryStates = new Set(
    proof.runtimeWriterDeliveryRows.map((row) => row.runtimeWriterDeliveryState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentActionRuntimeHandoffProofVersion === sourceParentActionRuntimeHandoffProofVersion &&
    proof.sourceStoreStatusHandoffProofVersion === sourceStoreStatusHandoffProofVersion &&
    proof.runtimeWriterDeliveryRows.length === requiredDecisionActions.length &&
    requiredDecisionActions.every((action) => actions.has(action)) &&
    requiredSourceRuntimeHandoffStatuses.every((status) => sourceStatuses.has(status)) &&
    requiredRuntimeWriterDeliveryStates.every((state) => deliveryStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProofGenerated(proof: {
  readonly runtimeWriterExecutionDeliveryRows: readonly {
    readonly runtimeWriterEnvelopeState: string;
    readonly runtimeWriterExecutionDeliveryState: string;
    readonly providerApiExecutionClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    runtimeWriterExecutionDeliveryRows: proof.runtimeWriterExecutionDeliveryRows.length,
    parentOwnedEnvelopeRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.runtimeWriterEnvelopeState === 'parent-owned-envelope-written'
    ).length,
    deliveryResultReceiptRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.runtimeWriterExecutionDeliveryState === 'delivery-result-recorded'
    ).length,
    manualRequiredRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.runtimeWriterExecutionDeliveryState === 'manual-required'
    ).length,
    providerExecutedRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseRuntimeWriterExecutionDeliveryRowGenerated(
  row: {
    readonly runtimeWriterDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly auditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly interceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  readinessRow: {
    readonly parentActionDeliveryReadinessRowId: string;
    readonly parentActionAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  sourceRuntimeWriterDeliveryProofVersion: string,
  sourceParentActionDeliveryReadinessProofVersion: string,
  claimBoundary: string,
  recordedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  const actionReceiptClaim = manual ? 'manual-required' : 'parent-owned-delivery-result-recorded';
  return {
    schemaVersion: 'app-install-purchase-runtime-writer-execution-delivery-proof',
    runtimeWriterExecutionDeliveryRowId: `runtime-writer-execution-delivery-${row.sourceDecisionAction}`,
    sourceRuntimeWriterDeliveryProofVersion,
    sourceRuntimeWriterDeliveryRowId: row.runtimeWriterDeliveryRowId,
    sourceParentActionDeliveryReadinessProofVersion,
    sourceParentActionDeliveryReadinessRowId: readinessRow.parentActionDeliveryReadinessRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    runtimeWriterEnvelopeState: manual ? 'manual-required' : 'parent-owned-envelope-written',
    runtimeWriterEnvelopeRef: `parent-owned-runtime-writer-envelope-${row.sourceDecisionAction}`,
    runtimeWriterExecutionDeliveryState: manual ? 'manual-required' : 'delivery-result-recorded',
    deliveryResultReceiptRef: `parent-owned-runtime-writer-receipt-${row.sourceDecisionAction}`,
    deliveryResultAuditEventRefs: row.auditEventRefs,
    parentActionAuditEventRefs: readinessRow.parentActionAuditEventRefs,
    reportRuntimeRefs: uniqueRefsGenerated([...row.reportRuntimeRefs, ...readinessRow.reportRuntimeRefs]),
    runtimeWriterExecutionClaim: actionReceiptClaim,
    runtimeWriterDeliveryClaim: actionReceiptClaim,
    parentActionRuntimeDeliveryClaim: actionReceiptClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.interceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    recordedAt,
  } as const;
}

export function runtimeWriterExecutionDeliveryRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeWriterDeliveryProofVersion: string;
    readonly sourceRuntimeWriterDeliveryRowId: string;
    readonly sourceParentActionDeliveryReadinessProofVersion: string;
    readonly sourceParentActionDeliveryReadinessRowId: string;
    readonly runtimeWriterEnvelopeState: string;
    readonly runtimeWriterExecutionDeliveryState: string;
    readonly runtimeWriterEnvelopeRef: string;
    readonly deliveryResultReceiptRef: string;
    readonly deliveryResultAuditEventRefs: readonly unknown[];
    readonly parentActionAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly runtimeWriterExecutionClaim: string;
    readonly runtimeWriterDeliveryClaim: string;
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceRuntimeWriterDeliveryProofVersion: string,
  sourceParentActionDeliveryReadinessProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const actionMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.runtimeWriterEnvelopeState === 'manual-required' &&
        row.runtimeWriterExecutionDeliveryState === 'manual-required' &&
        row.runtimeWriterExecutionClaim === 'manual-required' &&
        row.runtimeWriterDeliveryClaim === 'manual-required' &&
        row.parentActionRuntimeDeliveryClaim === 'manual-required'
      : row.runtimeWriterEnvelopeState === 'parent-owned-envelope-written' &&
        row.runtimeWriterExecutionDeliveryState === 'delivery-result-recorded' &&
        row.runtimeWriterExecutionClaim === 'parent-owned-delivery-result-recorded' &&
        row.runtimeWriterDeliveryClaim === 'parent-owned-delivery-result-recorded' &&
        row.parentActionRuntimeDeliveryClaim === 'parent-owned-delivery-result-recorded';
  return (
    actionMatches &&
    row.sourceRuntimeWriterDeliveryProofVersion === sourceRuntimeWriterDeliveryProofVersion &&
    row.sourceRuntimeWriterDeliveryRowId.length > 0 &&
    row.sourceParentActionDeliveryReadinessProofVersion === sourceParentActionDeliveryReadinessProofVersion &&
    row.sourceParentActionDeliveryReadinessRowId.length > 0 &&
    row.runtimeWriterEnvelopeRef.length > 0 &&
    row.deliveryResultReceiptRef.length > 0 &&
    row.deliveryResultAuditEventRefs.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function runtimeWriterExecutionDeliveryProofIsHonestGenerated(
  proof: {
    readonly sourceRuntimeWriterDeliveryProofVersion: string;
    readonly sourceParentActionDeliveryReadinessProofVersion: string;
    readonly runtimeWriterExecutionDeliveryRows: readonly {
      readonly sourceDecisionAction: string;
      readonly runtimeWriterEnvelopeState: string;
      readonly runtimeWriterExecutionDeliveryState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceRuntimeWriterDeliveryProofVersion: string,
  sourceParentActionDeliveryReadinessProofVersion: string,
  requiredDecisionActions: readonly string[],
  requiredRuntimeWriterEnvelopeStates: readonly string[],
  requiredRuntimeWriterExecutionDeliveryStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(
    proof.runtimeWriterExecutionDeliveryRows.map((row) => row.sourceDecisionAction)
  );
  const envelopeStates = new Set(
    proof.runtimeWriterExecutionDeliveryRows.map((row) => row.runtimeWriterEnvelopeState)
  );
  const deliveryStates = new Set(
    proof.runtimeWriterExecutionDeliveryRows.map((row) => row.runtimeWriterExecutionDeliveryState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterDeliveryProofVersion === sourceRuntimeWriterDeliveryProofVersion &&
    proof.sourceParentActionDeliveryReadinessProofVersion === sourceParentActionDeliveryReadinessProofVersion &&
    proof.runtimeWriterExecutionDeliveryRows.length === requiredDecisionActions.length &&
    requiredDecisionActions.every((action) => actions.has(action)) &&
    requiredRuntimeWriterEnvelopeStates.every((state) => envelopeStates.has(state)) &&
    requiredRuntimeWriterExecutionDeliveryStates.every((state) => deliveryStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseRuntimeReportWriterDeliveryProofGenerated(proof: {
  readonly runtimeReportWriterDeliveryRows: readonly {
    readonly runtimeReportWriterDeliveryState: string;
    readonly runtimeReportWriterReceiptState: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly portalReportUiClaim: string;
  }[];
}) {
  return {
    runtimeReportWriterDeliveryRows: proof.runtimeReportWriterDeliveryRows.length,
    reportDeliveryReadyRows: proof.runtimeReportWriterDeliveryRows.filter(
      (row) => row.runtimeReportWriterDeliveryState === 'report-delivery-ready'
    ).length,
    reportReceiptRows: proof.runtimeReportWriterDeliveryRows.filter(
      (row) => row.runtimeReportWriterReceiptState === 'parent-owned-report-receipt-recorded'
    ).length,
    manualRequiredRows: proof.runtimeReportWriterDeliveryRows.filter(
      (row) => row.runtimeReportWriterDeliveryState === 'manual-required'
    ).length,
    externallyDeliveredRows: proof.runtimeReportWriterDeliveryRows.filter(
      (row) => row.runtimeReportDeliveryClaim !== 'not-delivered'
    ).length,
    portalUiRows: proof.runtimeReportWriterDeliveryRows.filter((row) => row.portalReportUiClaim !== 'not-claimed')
      .length,
  } as const;
}

export function buildAppInstallPurchaseRuntimeReportWriterDeliveryRowGenerated(
  row: {
    readonly runtimeWriterExecutionDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly deliveryResultReceiptRef: string;
    readonly deliveryResultAuditEventRefs: readonly string[];
    readonly parentActionAuditEventRefs: readonly string[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  reportRows: ReadonlyArray<{
    readonly reportRuntimeRowId: string;
    readonly outputReportRef: string;
    readonly sourceReportRef: string;
  }>,
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  sourceReportRuntimeProofVersion: string,
  claimBoundary: string,
  recordedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-runtime-report-writer-delivery-proof',
    runtimeReportWriterDeliveryRowId: `runtime-report-writer-delivery-${row.sourceDecisionAction}`,
    sourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryRowId: row.runtimeWriterExecutionDeliveryRowId,
    sourceReportRuntimeProofVersion,
    sourceReportRuntimeRowIds: reportRows.map((reportRow) => reportRow.reportRuntimeRowId),
    sourceDecisionAction: row.sourceDecisionAction,
    runtimeReportWriterDeliveryState: manual ? 'manual-required' : 'report-delivery-ready',
    runtimeReportWriterReceiptState: manual ? 'manual-required' : 'parent-owned-report-receipt-recorded',
    runtimeReportWriterOutputRef: `parent-owned-runtime-report-output-${row.sourceDecisionAction}`,
    runtimeReportWriterReceiptRef: `parent-owned-runtime-report-receipt-${row.sourceDecisionAction}`,
    reportCompilerOutputRefs: uniqueRefsGenerated(reportRows.map((reportRow) => reportRow.outputReportRef)),
    runtimeWriterReceiptRef: row.deliveryResultReceiptRef,
    runtimeWriterAuditEventRefs: row.deliveryResultAuditEventRefs,
    parentActionAuditEventRefs: row.parentActionAuditEventRefs,
    reportAuditEventRefs: reportRows.map((reportRow) => reportRow.sourceReportRef),
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    portalReportUiClaim: 'not-claimed',
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    recordedAt,
  } as const;
}

export function runtimeReportWriterDeliveryRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeWriterExecutionDeliveryProofVersion: string;
    readonly sourceRuntimeWriterExecutionDeliveryRowId: string;
    readonly sourceReportRuntimeProofVersion: string;
    readonly sourceReportRuntimeRowIds: readonly unknown[];
    readonly runtimeReportWriterDeliveryState: string;
    readonly runtimeReportWriterReceiptState: string;
    readonly runtimeReportWriterOutputRef: string;
    readonly runtimeReportWriterReceiptRef: string;
    readonly reportCompilerOutputRefs: readonly unknown[];
    readonly runtimeWriterReceiptRef: string;
    readonly runtimeWriterAuditEventRefs: readonly unknown[];
    readonly parentActionAuditEventRefs: readonly unknown[];
    readonly reportAuditEventRefs: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformInterceptionClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly portalReportUiClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  sourceReportRuntimeProofVersion: string,
  expectedReportRowCount: number,
  boundaryFragments: readonly string[]
) {
  const actionMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.runtimeReportWriterDeliveryState === 'manual-required' &&
        row.runtimeReportWriterReceiptState === 'manual-required'
      : row.runtimeReportWriterDeliveryState === 'report-delivery-ready' &&
        row.runtimeReportWriterReceiptState === 'parent-owned-report-receipt-recorded';
  return (
    actionMatches &&
    row.sourceRuntimeWriterExecutionDeliveryProofVersion === sourceRuntimeWriterExecutionDeliveryProofVersion &&
    row.sourceRuntimeWriterExecutionDeliveryRowId.length > 0 &&
    row.sourceReportRuntimeProofVersion === sourceReportRuntimeProofVersion &&
    row.sourceReportRuntimeRowIds.length === expectedReportRowCount &&
    row.runtimeReportWriterOutputRef.length > 0 &&
    row.runtimeReportWriterReceiptRef.length > 0 &&
    row.reportCompilerOutputRefs.length > 0 &&
    row.runtimeWriterReceiptRef.length > 0 &&
    row.runtimeWriterAuditEventRefs.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportAuditEventRefs.length > 0 &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function runtimeReportWriterDeliveryProofIsHonestGenerated(
  proof: {
    readonly sourceRuntimeWriterExecutionDeliveryProofVersion: string;
    readonly sourceReportRuntimeProofVersion: string;
    readonly runtimeReportWriterDeliveryRows: readonly {
      readonly sourceDecisionAction: string;
      readonly runtimeReportWriterDeliveryState: string;
      readonly runtimeReportWriterReceiptState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  sourceReportRuntimeProofVersion: string,
  requiredActions: readonly string[],
  requiredDeliveryStates: readonly string[],
  requiredReceiptStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(
    proof.runtimeReportWriterDeliveryRows.map((row) => row.sourceDecisionAction)
  );
  const deliveryStates = new Set(
    proof.runtimeReportWriterDeliveryRows.map((row) => row.runtimeReportWriterDeliveryState)
  );
  const receiptStates = new Set(
    proof.runtimeReportWriterDeliveryRows.map((row) => row.runtimeReportWriterReceiptState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterExecutionDeliveryProofVersion === sourceRuntimeWriterExecutionDeliveryProofVersion &&
    proof.sourceReportRuntimeProofVersion === sourceReportRuntimeProofVersion &&
    proof.runtimeReportWriterDeliveryRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredDeliveryStates.every((state) => deliveryStates.has(state)) &&
    requiredReceiptStates.every((state) => receiptStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}
