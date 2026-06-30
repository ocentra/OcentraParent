/* generated from crates/schema/src/app_install_purchase_report_status_helpers.ts.txt */

export function summarizeAppInstallPurchaseApprovalReportDomainProofGenerated(proof: {
  readonly approvalReportDomainRows: readonly {
    readonly approvalReportDomainState: string;
    readonly reportRuntimeLinked: boolean;
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
  }[];
}) {
  return {
    approvalReportDomainRows: proof.approvalReportDomainRows.length,
    readyRows: proof.approvalReportDomainRows.filter((row) => row.approvalReportDomainState === 'approval-report-ready').length,
    manualReviewRows: proof.approvalReportDomainRows.filter(
      (row) => row.approvalReportDomainState === 'approval-report-manual-review'
    ).length,
    unavailableRows: proof.approvalReportDomainRows.filter(
      (row) => row.approvalReportDomainState === 'approval-report-unavailable'
    ).length,
    reportLinkedRows: proof.approvalReportDomainRows.filter((row) => row.reportRuntimeLinked).length,
    portalApprovalUiRows: proof.approvalReportDomainRows.filter((row) => row.portalApprovalUiClaim !== 'not-implemented')
      .length,
    portalReportUiRows: proof.approvalReportDomainRows.filter((row) => row.portalReportUiClaim !== 'not-implemented')
      .length,
  } as const;
}

export function approvalReportDomainStateGenerated(input: {
  readonly sourceDecisionAction: string;
  readonly parentActionRecorded: boolean;
  readonly sourceReportRuntimeRefs: readonly string[];
}) {
  if (input.sourceDecisionAction === 'review-needed') {
    return 'approval-report-manual-review' as const;
  }
  return input.parentActionRecorded && input.sourceReportRuntimeRefs.length > 0
    ? ('approval-report-ready' as const)
    : ('approval-report-unavailable' as const);
}

export function buildAppInstallPurchaseApprovalReportDomainRowGenerated(
  row: {
    readonly parentReviewActionRowId: string;
    readonly sourceDecisionAction: string;
    readonly parentReviewActionState: string;
    readonly parentActionRecorded: boolean;
    readonly auditEventRefs: readonly string[];
  },
  reportRows: ReadonlyArray<{
    readonly reportRuntimeRowId: string;
    readonly reportSurface: string;
  }>,
  sourceParentReviewActionProofVersion: string,
  sourceReportRuntimeProofVersion: string,
  claimBoundary: string,
  linkedAt: string
) {
  const sourceReportRuntimeRefs = reportRows.map((reportRow) => reportRow.reportRuntimeRowId);
  return {
    schemaVersion: 'app-install-purchase-approval-report-domain-proof',
    approvalReportDomainRowId: `approval-report-domain-${row.sourceDecisionAction}`,
    sourceParentReviewActionProofVersion,
    sourceParentReviewActionRowId: row.parentReviewActionRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceParentReviewActionState: row.parentReviewActionState,
    sourceReportRuntimeProofVersion,
    sourceReportRuntimeRefs,
    sourceReportSurfaces: reportRows.map((reportRow) => reportRow.reportSurface),
    sourceAuditEventRefs: row.auditEventRefs,
    approvalReportDomainState: approvalReportDomainStateGenerated({
      sourceDecisionAction: row.sourceDecisionAction,
      parentActionRecorded: row.parentActionRecorded,
      sourceReportRuntimeRefs,
    }),
    parentActionRecorded: row.parentActionRecorded,
    reportRuntimeLinked: sourceReportRuntimeRefs.length === reportRows.length,
    domainReadModelClaim: 'domain-read-model-only',
    portalApprovalUiClaim: 'not-implemented',
    portalReportUiClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    interceptionClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    linkedAt,
  } as const;
}

export function approvalReportDomainRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly parentActionRecorded: boolean;
    readonly sourceParentReviewActionRowId: string;
    readonly sourceReportRuntimeRefs: readonly string[];
    readonly sourceReportSurfaces: readonly string[];
    readonly sourceAuditEventRefs: readonly string[];
    readonly approvalReportDomainState: string;
    readonly reportRuntimeLinked: boolean;
    readonly domainReadModelClaim: string;
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedReportRuntimeRowCount: number,
  boundaryFragments: readonly string[]
) {
  const expectedState = approvalReportDomainStateGenerated({
    sourceDecisionAction: row.sourceDecisionAction,
    parentActionRecorded: row.parentActionRecorded,
    sourceReportRuntimeRefs: row.sourceReportRuntimeRefs,
  });
  return (
    row.sourceParentReviewActionRowId.length > 0 &&
    row.sourceReportRuntimeRefs.length === expectedReportRuntimeRowCount &&
    row.sourceReportSurfaces.length === expectedReportRuntimeRowCount &&
    row.sourceAuditEventRefs.length > 0 &&
    row.reportRuntimeLinked &&
    row.approvalReportDomainState === expectedState &&
    row.domainReadModelClaim === 'domain-read-model-only' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function approvalReportDomainProofIsHonestGenerated(
  proof: {
    readonly sourceParentReviewActionProofVersion: string;
    readonly sourceReportRuntimeProofVersion: string;
    readonly approvalReportDomainRows: readonly {
      readonly approvalReportDomainState: string;
      readonly sourceDecisionAction: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceParentReviewActionProofVersion: string,
  sourceReportRuntimeProofVersion: string,
  expectedRowCount: number,
  requiredNonClaims: readonly string[]
) {
  const states = new Set(proof.approvalReportDomainRows.map((row) => row.approvalReportDomainState));
  const decisions = new Set(proof.approvalReportDomainRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentReviewActionProofVersion === sourceParentReviewActionProofVersion &&
    proof.sourceReportRuntimeProofVersion === sourceReportRuntimeProofVersion &&
    proof.approvalReportDomainRows.length === expectedRowCount &&
    decisions.has('approve') &&
    decisions.has('deny') &&
    decisions.has('time-box') &&
    decisions.has('review-needed') &&
    states.has('approval-report-ready') &&
    states.has('approval-report-manual-review') &&
    !states.has('approval-report-unavailable') &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseReportRuntimeProofGenerated(proof: {
  readonly reportRuntimeRows: readonly {
    readonly reportRuntimeStatusClaim: string;
    readonly outputReportRef: string;
    readonly runtimeReportDeliveryClaim: string;
  }[];
}) {
  return {
    reportRuntimeRows: proof.reportRuntimeRows.length,
    compilerLinkedRows: proof.reportRuntimeRows.filter((row) => row.reportRuntimeStatusClaim === 'compiler-status-linked')
      .length,
    outputReportRefs: proof.reportRuntimeRows.filter((row) => row.outputReportRef.length > 0).length,
    portalDeliveredRows: proof.reportRuntimeRows.filter((row) => row.runtimeReportDeliveryClaim !== 'not-portal-delivered')
      .length,
  } as const;
}

export function buildAppInstallPurchaseReportRuntimeSurfaceRowGenerated(
  row: {
    readonly reportSurface: string;
    readonly reportRefs: readonly string[];
  },
  compilerProof: {
    readonly schemaVersion: string;
    readonly request: { readonly requestId: string; readonly parentAuthorized: boolean; readonly rawChildEvidenceRequested: boolean };
    readonly statuses: readonly { readonly statusRef: string; readonly status: string }[];
    readonly results: readonly {
      readonly resultRef: string;
      readonly status: string;
      readonly outputReportRef: string | null;
      readonly redaction: { readonly rawEvidenceExcludedFromOutput: boolean; readonly childDetailMinimized: boolean };
      readonly tempArtifacts: { readonly deletionConfirmed: boolean };
      readonly localEvidenceMutated: boolean;
      readonly ocentraHostedReportRetained: boolean;
    }[];
  },
  childArtifactRefs: readonly string[],
  auditEventRefs: readonly string[],
  claimBoundary: string,
  linkedAt: string
) {
  const successfulResult = compilerProof.results.find((result) => result.status === 'succeeded');
  if (!successfulResult || successfulResult.outputReportRef === null) {
    throw new Error('missing succeeded stateless report compiler output ref');
  }
  return {
    schemaVersion: 'app-install-purchase-report-runtime-proof',
    reportRuntimeRowId: `app-install-report-runtime-${row.reportSurface}`,
    reportSurface: row.reportSurface,
    sourceReportRef: row.reportRefs[0],
    sourceReportCompilerSchemaVersion: compilerProof.schemaVersion,
    compilerRequestId: compilerProof.request.requestId,
    compilerStatusRefs: compilerProof.statuses.map((status) => status.statusRef),
    compilerStatuses: compilerProof.statuses.map((status) => status.status),
    compilerResultRefs: compilerProof.results.map((result) => result.resultRef),
    compilerFinalResultStatuses: compilerProof.results.map((result) => result.status),
    outputReportRef: successfulResult.outputReportRef,
    childArtifactRefs,
    parentAuthorized: compilerProof.request.parentAuthorized,
    rawChildEvidenceRequested: compilerProof.request.rawChildEvidenceRequested,
    rawEvidenceExcludedFromOutput: successfulResult.redaction.rawEvidenceExcludedFromOutput,
    childDetailMinimized: successfulResult.redaction.childDetailMinimized,
    tempDeletionConfirmed: successfulResult.tempArtifacts.deletionConfirmed,
    localEvidenceMutated: successfulResult.localEvidenceMutated,
    ocentraHostedReportRetained: successfulResult.ocentraHostedReportRetained,
    reportRuntimeStatusClaim: 'compiler-status-linked',
    runtimeReportDeliveryClaim: 'not-portal-delivered',
    portalUiClaim: 'not-claimed',
    providerApiClaim: 'not-claimed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: 'not-delivered',
    childDataCustody: 'no-child-activity-data',
    appBlockingClaim: 'not-claimed',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    auditEventRefs,
    reportRefs: row.reportRefs,
    claimBoundary,
    linkedAt,
  } as const;
}

export function reportRuntimeSurfaceRowIsHonestGenerated(
  row: {
    readonly compilerStatusRefs: readonly unknown[];
    readonly compilerStatuses: readonly string[];
    readonly compilerResultRefs: readonly unknown[];
    readonly compilerFinalResultStatuses: readonly string[];
    readonly childArtifactRefs: readonly unknown[];
    readonly parentAuthorized: boolean;
    readonly rawChildEvidenceRequested: boolean;
    readonly rawEvidenceExcludedFromOutput: boolean;
    readonly childDetailMinimized: boolean;
    readonly tempDeletionConfirmed: boolean;
    readonly localEvidenceMutated: boolean;
    readonly ocentraHostedReportRetained: boolean;
    readonly reportRuntimeStatusClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly portalUiClaim: string;
    readonly providerApiClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeliveryClaim: string;
    readonly childDataCustody: string;
    readonly appBlockingClaim: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  requiredCompilerStatuses: readonly string[],
  requiredFinalStatuses: readonly string[],
  boundaryFragments: readonly string[]
) {
  const statuses = new Set(row.compilerStatuses);
  const finalStatuses = new Set(row.compilerFinalResultStatuses);
  return (
    row.compilerStatusRefs.length === row.compilerStatuses.length &&
    row.compilerResultRefs.length === row.compilerFinalResultStatuses.length &&
    row.childArtifactRefs.length > 0 &&
    requiredCompilerStatuses.every((status) => statuses.has(status)) &&
    requiredFinalStatuses.every((status) => finalStatuses.has(status)) &&
    row.parentAuthorized &&
    !row.rawChildEvidenceRequested &&
    row.rawEvidenceExcludedFromOutput &&
    row.childDetailMinimized &&
    row.tempDeletionConfirmed &&
    !row.localEvidenceMutated &&
    !row.ocentraHostedReportRetained &&
    row.reportRuntimeStatusClaim === 'compiler-status-linked' &&
    row.runtimeReportDeliveryClaim === 'not-portal-delivered' &&
    row.portalUiClaim === 'not-claimed' &&
    row.providerApiClaim === 'not-claimed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function reportRuntimeProofIsHonestGenerated(
  proof: {
    readonly sourceChildArtifactProofVersion: string;
    readonly sourcePlatformArtifactProofVersion: string;
    readonly sourceReportCompilerSchemaVersion: string;
    readonly reportRuntimeRows: readonly { readonly reportSurface: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceChildArtifactProofVersion: string,
  sourcePlatformArtifactProofVersion: string,
  sourceReportCompilerSchemaVersion: string,
  requiredSurfaces: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const surfaces = new Set(proof.reportRuntimeRows.map((row) => row.reportSurface));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceChildArtifactProofVersion === sourceChildArtifactProofVersion &&
    proof.sourcePlatformArtifactProofVersion === sourcePlatformArtifactProofVersion &&
    proof.sourceReportCompilerSchemaVersion === sourceReportCompilerSchemaVersion &&
    proof.reportRuntimeRows.length === requiredSurfaces.length &&
    requiredSurfaces.every((surface) => surfaces.has(surface)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseProviderStoreReportStatusProofGenerated(proof: {
  readonly providerStoreReportStatusRows: readonly {
    readonly providerStoreReportStatusState: string;
    readonly sourceApprovalReportDomainRowIds: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
  }[];
}) {
  return {
    providerStoreReportStatusRows: proof.providerStoreReportStatusRows.length,
    readyRows: proof.providerStoreReportStatusRows.filter(
      (row) => row.providerStoreReportStatusState === 'provider-store-report-status-ready'
    ).length,
    manualRequiredRows: proof.providerStoreReportStatusRows.filter((row) => row.providerStoreReportStatusState === 'manual-required')
      .length,
    unavailableRows: proof.providerStoreReportStatusRows.filter((row) => row.providerStoreReportStatusState === 'unavailable').length,
    approvalReportLinkedRows: proof.providerStoreReportStatusRows.filter((row) => row.sourceApprovalReportDomainRowIds.length > 0)
      .length,
    providerExecutedRows: proof.providerStoreReportStatusRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed')
      .length,
    portalRows: proof.providerStoreReportStatusRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-implemented' || row.portalReportUiClaim !== 'not-implemented'
    ).length,
  } as const;
}

export function providerStoreReportStatusStateGenerated(state: string) {
  if (state === 'provider-store-execution-ready') {
    return 'provider-store-report-status-ready' as const;
  }
  if (state === 'unavailable') {
    return 'unavailable' as const;
  }
  return 'manual-required' as const;
}

export function buildAppInstallPurchaseProviderStoreReportStatusRowGenerated(
  row: {
    readonly providerStoreExecutionReadinessRowId: string;
    readonly providerStoreExecutionReadinessState: string;
    readonly reportRuntimeRefs: readonly string[];
    readonly parentActionAuditEventRefs: readonly string[];
    readonly platform: string;
    readonly storeSurface: string;
  },
  approvalReportRows: ReadonlyArray<{
    readonly approvalReportDomainRowId: string;
    readonly approvalReportDomainState: string;
    readonly sourceReportRuntimeRefs: readonly string[];
    readonly sourceAuditEventRefs: readonly string[];
  }>,
  sourceProviderStoreExecutionReadinessProofVersion: string,
  sourceApprovalReportDomainProofVersion: string,
  claimBoundary: string,
  evaluatedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-provider-store-report-status-proof',
    providerStoreReportStatusRowId: `provider-store-report-status-${row.platform}-${row.storeSurface}`,
    sourceProviderStoreExecutionReadinessProofVersion,
    sourceProviderStoreExecutionReadinessRowId: row.providerStoreExecutionReadinessRowId,
    sourceProviderStoreExecutionReadinessState: row.providerStoreExecutionReadinessState,
    sourceApprovalReportDomainProofVersion,
    sourceApprovalReportDomainRowIds: approvalReportRows.map((reportRow) => reportRow.approvalReportDomainRowId),
    sourceApprovalReportDomainStates: approvalReportRows.map((reportRow) => reportRow.approvalReportDomainState),
    sourceReportRuntimeRefs: uniqueRefsGenerated([
      ...row.reportRuntimeRefs,
      ...approvalReportRows.flatMap((reportRow) => reportRow.sourceReportRuntimeRefs),
    ]),
    sourceAuditEventRefs: uniqueRefsGenerated([
      ...row.parentActionAuditEventRefs,
      ...approvalReportRows.flatMap((reportRow) => reportRow.sourceAuditEventRefs),
    ]),
    platform: row.platform,
    storeSurface: row.storeSurface,
    providerStoreReportStatusState: providerStoreReportStatusStateGenerated(row.providerStoreExecutionReadinessState),
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    billingProviderContactClaim: 'not-executed',
    portalApprovalUiClaim: 'not-implemented',
    portalReportUiClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary,
    evaluatedAt,
  } as const;
}

export function providerStoreReportStatusRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreExecutionReadinessState: string;
    readonly sourceProviderStoreExecutionReadinessRowId: string;
    readonly sourceApprovalReportDomainRowIds: readonly unknown[];
    readonly sourceApprovalReportDomainStates: readonly string[];
    readonly sourceReportRuntimeRefs: readonly unknown[];
    readonly sourceAuditEventRefs: readonly unknown[];
    readonly providerStoreReportStatusState: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly billingProviderContactClaim: string;
    readonly portalApprovalUiClaim: string;
    readonly portalReportUiClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedApprovalReportRowCount: number,
  boundaryFragments: readonly string[]
) {
  const states = new Set(row.sourceApprovalReportDomainStates);
  return (
    row.sourceProviderStoreExecutionReadinessRowId.length > 0 &&
    row.providerStoreReportStatusState === providerStoreReportStatusStateGenerated(row.sourceProviderStoreExecutionReadinessState) &&
    row.sourceApprovalReportDomainRowIds.length === expectedApprovalReportRowCount &&
    states.has('approval-report-ready') &&
    states.has('approval-report-manual-review') &&
    row.sourceReportRuntimeRefs.length > 0 &&
    row.sourceAuditEventRefs.length > 0 &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.billingProviderContactClaim === 'not-executed' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function providerStoreReportStatusProofIsHonestGenerated(
  proof: {
    readonly providerStoreReportStatusRows: readonly { readonly providerStoreReportStatusState: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  expectedRowCount: number,
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const states = new Set(proof.providerStoreReportStatusRows.map((row) => row.providerStoreReportStatusState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.providerStoreReportStatusRows.length === expectedRowCount &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseReportStatusReadModelHandoffProofGenerated(proof: {
  readonly reportStatusReadModelRows: readonly {
    readonly parentVisibleReportStatusState: string;
    readonly portalReportUiClaim: string;
    readonly runtimeReportDeliveryClaim: string;
  }[];
}) {
  return {
    reportStatusReadModelRows: proof.reportStatusReadModelRows.length,
    readyRows: proof.reportStatusReadModelRows.filter((row) => row.parentVisibleReportStatusState === 'parent-report-status-ready')
      .length,
    manualRequiredRows: proof.reportStatusReadModelRows.filter((row) => row.parentVisibleReportStatusState === 'manual-required')
      .length,
    portalReportUiRows: proof.reportStatusReadModelRows.filter((row) => row.portalReportUiClaim !== 'not-implemented').length,
    externallyDeliveredRows: proof.reportStatusReadModelRows.filter((row) => row.runtimeReportDeliveryClaim !== 'not-delivered')
      .length,
  } as const;
}

export function buildAppInstallPurchaseReportStatusReadModelRowGenerated(
  row: {
    readonly runtimeReportWriterDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeReportWriterDeliveryState: string;
    readonly runtimeReportWriterReceiptRef: string;
    readonly reportAuditEventRefs: readonly string[];
    readonly runtimeReportDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  approvalRow: {
    readonly approvalReportDomainRowId: string;
    readonly approvalReportDomainState: string;
  },
  sourceRuntimeReportWriterDeliveryProofVersion: string,
  sourceApprovalReportDomainProofVersion: string,
  claimBoundary: string,
  recordedAt: string
) {
  const parentVisibleReportStatusState =
    row.sourceDecisionAction === 'review-needed' ? 'manual-required' : 'parent-report-status-ready';
  return {
    schemaVersion: 'app-install-purchase-report-status-read-model-handoff-proof',
    reportStatusReadModelRowId: `report-status-read-model-${row.sourceDecisionAction}`,
    sourceRuntimeReportWriterDeliveryProofVersion,
    sourceRuntimeReportWriterDeliveryRowId: row.runtimeReportWriterDeliveryRowId,
    sourceApprovalReportDomainProofVersion,
    sourceApprovalReportDomainRowId: approvalRow.approvalReportDomainRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceApprovalReportDomainState: approvalRow.approvalReportDomainState,
    sourceRuntimeReportWriterDeliveryState: row.runtimeReportWriterDeliveryState,
    sourceRuntimeReportWriterReceiptState: row.runtimeReportWriterDeliveryState === 'report-delivery-ready'
      ? 'parent-owned-report-receipt-recorded'
      : 'manual-required',
    parentVisibleReportStatusState,
    parentVisibleReportStatusRef: `parent-visible-report-status-${row.sourceDecisionAction}`,
    parentVisibleReportReceiptRef: row.runtimeReportWriterReceiptRef,
    reportAuditEventRefs: row.reportAuditEventRefs,
    portalReportUiClaim: 'not-implemented',
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    recordedAt,
  } as const;
}

export function reportStatusReadModelRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeReportWriterDeliveryRowId: string;
    readonly sourceApprovalReportDomainRowId: string;
    readonly parentVisibleReportStatusState: string;
    readonly parentVisibleReportStatusRef: string;
    readonly parentVisibleReportReceiptRef: string;
    readonly reportAuditEventRefs: readonly unknown[];
    readonly portalReportUiClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly childDeviceDeliveryClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  boundaryFragments: readonly string[]
) {
  const expectedState = row.sourceDecisionAction === 'review-needed' ? 'manual-required' : 'parent-report-status-ready';
  return (
    row.sourceRuntimeReportWriterDeliveryRowId.length > 0 &&
    row.sourceApprovalReportDomainRowId.length > 0 &&
    row.parentVisibleReportStatusState === expectedState &&
    row.parentVisibleReportStatusRef.length > 0 &&
    row.parentVisibleReportReceiptRef.length > 0 &&
    row.reportAuditEventRefs.length > 0 &&
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function reportStatusReadModelProofIsHonestGenerated(
  proof: {
    readonly reportStatusReadModelRows: readonly {
      readonly sourceDecisionAction: string;
      readonly parentVisibleReportStatusState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  requiredActions: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.reportStatusReadModelRows.map((row) => row.sourceDecisionAction));
  const states = new Set(proof.reportStatusReadModelRows.map((row) => row.parentVisibleReportStatusState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.reportStatusReadModelRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseStoreStatusHandoffProofGenerated(proof: {
  readonly storeStatusHandoffRows: readonly {
    readonly storeStatusHandoffState: string;
    readonly sourceParentActionRuntimeHandoffRefs: readonly unknown[];
    readonly statusHandoffDeliveryClaim: string;
  }[];
}) {
  return {
    storeStatusHandoffRows: proof.storeStatusHandoffRows.length,
    approvedApiRequiredRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'approved-api-status-proof-required'
    ).length,
    entitlementRequiredRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'store-entitlement-status-proof-required'
    ).length,
    manualRequiredRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'manual-platform-status-review-required'
    ).length,
    unavailableRows: proof.storeStatusHandoffRows.filter(
      (row) => row.storeStatusHandoffState === 'platform-store-status-unavailable'
    ).length,
    parentActionRuntimeLinkedRows: proof.storeStatusHandoffRows.filter(
      (row) => row.sourceParentActionRuntimeHandoffRefs.length > 0
    ).length,
    deliveredRows: proof.storeStatusHandoffRows.filter((row) => row.statusHandoffDeliveryClaim !== 'not-delivered').length,
  } as const;
}

export function storeStatusHandoffStateGenerated(adapterEvidenceState: string) {
  if (adapterEvidenceState === 'approved-api-adapter-evidence-required') {
    return 'approved-api-status-proof-required' as const;
  }
  if (adapterEvidenceState === 'entitlement-adapter-evidence-required') {
    return 'store-entitlement-status-proof-required' as const;
  }
  if (adapterEvidenceState === 'manual-platform-review-required') {
    return 'manual-platform-status-review-required' as const;
  }
  return 'platform-store-status-unavailable' as const;
}

export function buildAppInstallPurchaseStoreStatusHandoffRowGenerated(
  row: {
    readonly adapterBoundaryRowId: string;
    readonly platform: string;
    readonly storeSurface: string;
    readonly adapterEvidenceState: string;
    readonly adapterRuntimeState: string;
    readonly approvedApiEvidenceRef: string;
    readonly entitlementEvidenceRef: string;
    readonly limitationReportRef: string;
    readonly adapterReadinessEvidenceRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
  },
  runtimeHandoffRows: ReadonlyArray<{
    readonly runtimeHandoffRowId: string;
    readonly runtimeHandoffStatus: string;
  }>,
  sourcePlatformAdapterBoundaryProofVersion: string,
  sourceParentActionRuntimeHandoffProofVersion: string,
  claimBoundary: string,
  handedOffAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-store-status-handoff-proof',
    storeStatusHandoffRowId: `store-status-handoff-${row.platform}-${row.storeSurface}`,
    sourcePlatformAdapterBoundaryProofVersion,
    sourcePlatformAdapterBoundaryRowId: row.adapterBoundaryRowId,
    sourceParentActionRuntimeHandoffProofVersion,
    sourceParentActionRuntimeHandoffRefs: runtimeHandoffRows.map((handoffRow) => handoffRow.runtimeHandoffRowId),
    sourceParentActionRuntimeStatuses: runtimeHandoffRows.map((handoffRow) => handoffRow.runtimeHandoffStatus),
    platform: row.platform,
    storeSurface: row.storeSurface,
    sourceAdapterEvidenceState: row.adapterEvidenceState,
    sourceAdapterRuntimeState: row.adapterRuntimeState,
    storeStatusHandoffState: storeStatusHandoffStateGenerated(row.adapterEvidenceState),
    storeStatusRuntimeState: row.adapterRuntimeState,
    storeStatusHandoffEvidenceRefs: [
      row.approvedApiEvidenceRef,
      row.entitlementEvidenceRef,
      row.limitationReportRef,
      ...row.adapterReadinessEvidenceRefs,
    ],
    sourceReportRuntimeRefs: row.reportRuntimeRefs,
    storeStatusHandoffClaim: 'status-handoff-proof-only',
    statusHandoffDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: 'not-implemented',
    parentActionRuntimeDeliveryClaim: 'not-delivered',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    handedOffAt,
  } as const;
}

export function storeStatusHandoffRowIsHonestGenerated(
  row: {
    readonly sourceAdapterEvidenceState: string;
    readonly sourceAdapterRuntimeState: string;
    readonly sourcePlatformAdapterBoundaryRowId: string;
    readonly sourceParentActionRuntimeHandoffRefs: readonly unknown[];
    readonly sourceParentActionRuntimeStatuses: readonly string[];
    readonly storeStatusHandoffState: string;
    readonly storeStatusRuntimeState: string;
    readonly storeStatusHandoffClaim: string;
    readonly statusHandoffDeliveryClaim: string;
    readonly providerApiExecutionClaim: string;
    readonly storeIntegrationClaim: string;
    readonly platformAdapterClaim: string;
    readonly parentActionRuntimeDeliveryClaim: string;
    readonly childDeliveryClaim: string;
    readonly runtimeReportDeliveryClaim: string;
    readonly interceptionClaim: string;
    readonly appBlockingClaim: string;
    readonly childDataCustody: string;
    readonly ocentraHostedFamilyDataCustodyClaim: string;
    readonly storeStatusHandoffEvidenceRefs: readonly unknown[];
    readonly sourceReportRuntimeRefs: readonly unknown[];
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  expectedRuntimeHandoffRowCount: number,
  requiredRuntimeStatuses: readonly string[],
  boundaryFragments: readonly string[]
) {
  const statuses = new Set(row.sourceParentActionRuntimeStatuses);
  const expectedState = storeStatusHandoffStateGenerated(row.sourceAdapterEvidenceState);
  const runtimeStateMatches =
    row.sourceAdapterEvidenceState === 'manual-platform-review-required'
      ? row.sourceAdapterRuntimeState === 'manual-required' && row.storeStatusRuntimeState === 'manual-required'
      : row.sourceAdapterEvidenceState === 'platform-unavailable'
        ? row.sourceAdapterRuntimeState === 'unavailable' && row.storeStatusRuntimeState === 'unavailable'
        : row.sourceAdapterRuntimeState === 'not-implemented' && row.storeStatusRuntimeState === 'not-implemented';
  return (
    row.sourcePlatformAdapterBoundaryRowId.length > 0 &&
    row.sourceParentActionRuntimeHandoffRefs.length === expectedRuntimeHandoffRowCount &&
    requiredRuntimeStatuses.every((status) => statuses.has(status)) &&
    row.storeStatusHandoffEvidenceRefs.length >= 4 &&
    row.sourceReportRuntimeRefs.length > 0 &&
    row.storeStatusHandoffState === expectedState &&
    runtimeStateMatches &&
    row.storeStatusHandoffClaim === 'status-handoff-proof-only' &&
    row.statusHandoffDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed' &&
    boundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

export function storeStatusHandoffProofIsHonestGenerated(
  proof: {
    readonly storeStatusHandoffRows: readonly { readonly storeStatusHandoffState: string; readonly platform: string; readonly storeSurface: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  requiredPlatformSources: readonly (readonly [string, string])[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const keys = new Set(proof.storeStatusHandoffRows.map((row) => `${row.platform}:${row.storeSurface}`));
  const states = new Set(proof.storeStatusHandoffRows.map((row) => row.storeStatusHandoffState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.storeStatusHandoffRows.length === requiredPlatformSources.length &&
    keys.size === proof.storeStatusHandoffRows.length &&
    requiredPlatformSources.every(([platform, storeSurface]) => keys.has(`${platform}:${storeSurface}`)) &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseLimitationSummaryProofGenerated(proof: {
  readonly limitationSummaryRows: readonly {
    readonly limitationSummaryState: string;
    readonly sourceProviderStoreReportStatusRowIds: readonly unknown[];
    readonly sourceReportStatusReadModelRowIds: readonly unknown[];
    readonly providerApiExecutionClaim: string;
    readonly runtimeReportDeliveryClaim: string;
  }[];
}) {
  return {
    limitationSummaryRows: proof.limitationSummaryRows.length,
    readyRows: proof.limitationSummaryRows.filter((row) => row.limitationSummaryState === 'ready').length,
    manualRequiredRows: proof.limitationSummaryRows.filter((row) => row.limitationSummaryState === 'manual-required').length,
    unavailableRows: proof.limitationSummaryRows.filter((row) => row.limitationSummaryState === 'unavailable').length,
    sourceProviderStoreRows: proof.limitationSummaryRows.flatMap((row) => row.sourceProviderStoreReportStatusRowIds).length,
    sourceReportStatusRows: proof.limitationSummaryRows.flatMap((row) => row.sourceReportStatusReadModelRowIds).length,
    providerExecutedRows: proof.limitationSummaryRows.filter((row) => row.providerApiExecutionClaim !== 'not-executed').length,
    externallyDeliveredRows: proof.limitationSummaryRows.filter((row) => row.runtimeReportDeliveryClaim !== 'not-delivered')
      .length,
  } as const;
}

export function providerStateMapsToSummaryGenerated(state: string) {
  if (state === 'provider-store-report-status-ready') {
    return 'ready' as const;
  }
  if (state === 'unavailable') {
    return 'unavailable' as const;
  }
  return 'manual-required' as const;
}

export function reportStateMapsToSummaryGenerated(state: string) {
  return state === 'parent-report-status-ready' ? ('ready' as const) : ('manual-required' as const);
}

export function buildAppInstallPurchaseLimitationSummaryRowGenerated(
  state: 'ready' | 'manual-required' | 'unavailable',
  providerRows: ReadonlyArray<{
    readonly providerStoreReportStatusRowId: string;
    readonly providerStoreReportStatusState: string;
    readonly sourceAuditEventRefs: readonly string[];
  }>,
  reportRows: ReadonlyArray<{
    readonly reportStatusReadModelRowId: string;
    readonly parentVisibleReportStatusState: string;
    readonly reportAuditEventRefs: readonly string[];
  }>,
  sourceProviderStoreReportStatusProofVersion: string,
  sourceReportStatusReadModelProofVersion: string,
  claimBoundary: string,
  summarizedAt: string
) {
  return {
    schemaVersion: 'app-install-purchase-limitation-summary-proof',
    limitationSummaryRowId: `app-install-limitation-summary-${state}`,
    limitationSummaryState: state,
    sourceProviderStoreReportStatusProofVersion,
    sourceProviderStoreReportStatusRowIds: providerRows.map((row) => row.providerStoreReportStatusRowId),
    sourceProviderStoreReportStatusStates: providerRows.map((row) => row.providerStoreReportStatusState),
    sourceReportStatusReadModelProofVersion,
    sourceReportStatusReadModelRowIds: reportRows.map((row) => row.reportStatusReadModelRowId),
    sourceReportStatusReadModelStates: reportRows.map((row) => row.parentVisibleReportStatusState),
    sourceAuditEventRefs: uniqueRefsGenerated([
      ...providerRows.flatMap((row) => row.sourceAuditEventRefs),
      ...reportRows.flatMap((row) => row.reportAuditEventRefs),
    ]),
    parentVisibleSummaryRef: `parent-visible-app-install-limitation-summary-${state}`,
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
    summarizedAt,
  } as const;
}

export function limitationSummaryRowIsHonestGenerated(
  row: {
    readonly sourceProviderStoreReportStatusProofVersion: string;
    readonly sourceProviderStoreReportStatusStates: readonly string[];
    readonly sourceReportStatusReadModelProofVersion: string;
    readonly sourceReportStatusReadModelStates: readonly string[];
    readonly sourceAuditEventRefs: readonly unknown[];
    readonly parentVisibleSummaryRef: string;
    readonly limitationSummaryState: string;
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
  sourceProviderStoreReportStatusProofVersion: string,
  sourceReportStatusReadModelProofVersion: string,
  boundaryFragments: readonly string[]
) {
  return (
    row.sourceProviderStoreReportStatusProofVersion === sourceProviderStoreReportStatusProofVersion &&
    row.sourceReportStatusReadModelProofVersion === sourceReportStatusReadModelProofVersion &&
    row.parentVisibleSummaryRef.length > 0 &&
    row.sourceAuditEventRefs.length > 0 &&
    row.sourceProviderStoreReportStatusStates.every(
      (state) => providerStateMapsToSummaryGenerated(state) === row.limitationSummaryState
    ) &&
    row.sourceReportStatusReadModelStates.every(
      (state) => reportStateMapsToSummaryGenerated(state) === row.limitationSummaryState
    ) &&
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

export function limitationSummaryProofIsHonestGenerated(
  proof: {
    readonly sourceProviderStoreReportStatusProofVersion: string;
    readonly sourceReportStatusReadModelProofVersion: string;
    readonly limitationSummaryRows: readonly { readonly limitationSummaryState: string }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceProviderStoreReportStatusProofVersion: string,
  sourceReportStatusReadModelProofVersion: string,
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const states = new Set(proof.limitationSummaryRows.map((row) => row.limitationSummaryState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProviderStoreReportStatusProofVersion === sourceProviderStoreReportStatusProofVersion &&
    proof.sourceReportStatusReadModelProofVersion === sourceReportStatusReadModelProofVersion &&
    proof.limitationSummaryRows.length === requiredStates.length &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

function uniqueRefsGenerated(refs: readonly string[]) {
  return Array.from(new Set(refs));
}
