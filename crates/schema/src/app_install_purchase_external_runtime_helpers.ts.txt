/* generated from crates/schema/src/app_install_purchase_external_runtime_helpers.ts.txt */

function uniqueRefsGenerated(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

export function summarizeAppInstallPurchaseExternalRuntimeDeviceDeliveryProofGenerated(proof: {
  readonly externalRuntimeDeviceDeliveryRows: readonly {
    readonly externalRuntimeEvidenceState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeDeviceDeliveryRows: proof.externalRuntimeDeviceDeliveryRows.length,
    externalRuntimeEvidenceReadyRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.externalRuntimeEvidenceState === 'external-runtime-evidence-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.externalRuntimeEvidenceState === 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
    childDeviceDeliveredRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeDeviceDeliveryRowGenerated(
  row: {
    readonly runtimeWriterExecutionDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeWriterEnvelopeRef: string;
    readonly deliveryResultReceiptRef: string;
    readonly runtimeWriterExecutionClaim: string;
    readonly deliveryResultAuditEventRefs: readonly string[];
    readonly parentActionAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
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
  childDeliveryRow: {
    readonly childDeviceDeliveryRuntimeWriterRowId: string;
    readonly childDeliveryEnvelopeState: string;
    readonly childDeliveryTargetRefs: readonly string[];
    readonly runtimeWriterAuditEventRefs: readonly string[];
    readonly packageSourceAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
  },
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  claimBoundary: string,
  linkedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-device-delivery-proof',
    externalRuntimeDeviceDeliveryRowId: `external-runtime-device-delivery-${row.sourceDecisionAction}`,
    sourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryRowId: row.runtimeWriterExecutionDeliveryRowId,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterRowId: childDeliveryRow.childDeviceDeliveryRuntimeWriterRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeWriterEnvelopeRef: row.runtimeWriterEnvelopeRef,
    sourceDeliveryResultReceiptRef: row.deliveryResultReceiptRef,
    sourceRuntimeWriterReceiptClaim: row.runtimeWriterExecutionClaim,
    sourceChildDeliveryEnvelopeState: childDeliveryRow.childDeliveryEnvelopeState,
    sourceChildDeliveryTargetRefs: childDeliveryRow.childDeliveryTargetRefs,
    externalRuntimeEvidenceState: manual ? 'manual-required' : 'external-runtime-evidence-ready',
    externalRuntimeWriterTargetRefs: uniqueRefsGenerated([
      row.runtimeWriterEnvelopeRef,
      row.deliveryResultReceiptRef,
      ...childDeliveryRow.childDeliveryTargetRefs,
    ]),
    externalRuntimeWriterAuditEventRefs: uniqueRefsGenerated([
      ...row.deliveryResultAuditEventRefs,
      ...row.parentActionAuditEventRefs,
    ]),
    childDeliveryAuditEventRefs: uniqueRefsGenerated([
      ...childDeliveryRow.runtimeWriterAuditEventRefs,
      ...childDeliveryRow.packageSourceAuditEventRefs,
    ]),
    reportRuntimeRefs: uniqueRefsGenerated([...row.reportRuntimeRefs, ...childDeliveryRow.reportRuntimeRefs]),
    externalRuntimeWriterExecutionClaim: 'not-executed',
    externalRuntimeWriterDeliveryClaim: 'not-delivered',
    parentActionRuntimeDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function externalRuntimeDeviceDeliveryRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceRuntimeWriterExecutionDeliveryProofVersion: string;
    readonly sourceRuntimeWriterExecutionDeliveryRowId: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterProofVersion: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterRowId: string;
    readonly sourceRuntimeWriterEnvelopeRef: string;
    readonly sourceDeliveryResultReceiptRef: string;
    readonly sourceRuntimeWriterReceiptClaim: string;
    readonly sourceChildDeliveryEnvelopeState: string;
    readonly sourceChildDeliveryTargetRefs: readonly unknown[];
    readonly externalRuntimeEvidenceState: string;
    readonly externalRuntimeWriterTargetRefs: readonly unknown[];
    readonly externalRuntimeWriterAuditEventRefs: readonly unknown[];
    readonly childDeliveryAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const evidenceMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.sourceRuntimeWriterReceiptClaim === 'manual-required' &&
        row.sourceChildDeliveryEnvelopeState === 'manual-review-required' &&
        row.externalRuntimeEvidenceState === 'manual-required'
      : row.sourceRuntimeWriterReceiptClaim === 'parent-owned-delivery-result-recorded' &&
        row.sourceChildDeliveryEnvelopeState === 'child-delivery-envelope-ready' &&
        row.externalRuntimeEvidenceState === 'external-runtime-evidence-ready';
  return (
    evidenceMatches &&
    row.sourceRuntimeWriterExecutionDeliveryProofVersion === sourceRuntimeWriterExecutionDeliveryProofVersion &&
    row.sourceRuntimeWriterExecutionDeliveryRowId.length > 0 &&
    row.sourceChildDeviceDeliveryRuntimeWriterProofVersion === sourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    row.sourceChildDeviceDeliveryRuntimeWriterRowId.length > 0 &&
    row.sourceRuntimeWriterEnvelopeRef.length > 0 &&
    row.sourceDeliveryResultReceiptRef.length > 0 &&
    row.sourceChildDeliveryTargetRefs.length > 0 &&
    row.externalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterAuditEventRefs.length > 0 &&
    row.childDeliveryAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeDeviceDeliveryProofIsHonestGenerated(
  proof: {
    readonly sourceRuntimeWriterExecutionDeliveryProofVersion: string;
    readonly sourceChildDeviceDeliveryRuntimeWriterProofVersion: string;
    readonly externalRuntimeDeviceDeliveryRows: readonly {
      readonly sourceDecisionAction: string;
      readonly externalRuntimeEvidenceState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceRuntimeWriterExecutionDeliveryProofVersion: string,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: string,
  requiredActions: readonly string[],
  requiredEvidenceStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeDeviceDeliveryRows.map((row) => row.sourceDecisionAction));
  const evidenceStates = new Set(proof.externalRuntimeDeviceDeliveryRows.map((row) => row.externalRuntimeEvidenceState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterExecutionDeliveryProofVersion === sourceRuntimeWriterExecutionDeliveryProofVersion &&
    proof.sourceChildDeviceDeliveryRuntimeWriterProofVersion === sourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    proof.externalRuntimeDeviceDeliveryRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredEvidenceStates.every((state) => evidenceStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeDeliveryHandoffProofGenerated(proof: {
  readonly externalRuntimeDeliveryHandoffRows: readonly {
    readonly externalRuntimeDeliveryHandoffState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeDeliveryHandoffRows: proof.externalRuntimeDeliveryHandoffRows.length,
    handoffPacketReadyRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.externalRuntimeDeliveryHandoffState === 'handoff-packet-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.externalRuntimeDeliveryHandoffState === 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
    childDeviceDeliveredRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeDeliveryHandoffRowGenerated(
  row: {
    readonly externalRuntimeDeviceDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly externalRuntimeEvidenceState: string;
    readonly sourceRuntimeWriterEnvelopeRef: string;
    readonly sourceDeliveryResultReceiptRef: string;
    readonly externalRuntimeWriterTargetRefs: readonly string[];
    readonly childDeliveryAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeDeviceDeliveryProofVersion: string,
  claimBoundary: string,
  linkedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-delivery-handoff-proof',
    externalRuntimeDeliveryHandoffRowId: `external-runtime-delivery-handoff-${row.sourceDecisionAction}`,
    sourceExternalRuntimeDeviceDeliveryProofVersion,
    sourceExternalRuntimeDeviceDeliveryRowId: row.externalRuntimeDeviceDeliveryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeEvidenceState: row.externalRuntimeEvidenceState,
    sourceRuntimeWriterEnvelopeRef: row.sourceRuntimeWriterEnvelopeRef,
    sourceDeliveryResultReceiptRef: row.sourceDeliveryResultReceiptRef,
    sourceExternalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
    sourceChildDeliveryAuditEventRefs: row.childDeliveryAuditEventRefs,
    externalRuntimeDeliveryHandoffState: manual ? 'manual-required' : 'handoff-packet-ready',
    externalRuntimeHandoffPacketRef: manual
      ? `manual-external-runtime-handoff-packet-${row.sourceDecisionAction}`
      : `parent-owned-external-runtime-handoff-packet-${row.sourceDecisionAction}`,
    externalRuntimeWriterQueueRef: manual
      ? `manual-external-runtime-writer-queue-${row.sourceDecisionAction}`
      : `parent-owned-external-runtime-writer-queue-${row.sourceDecisionAction}`,
    externalRuntimeWriterDispatchAuditEventRefs: uniqueRefsGenerated([
      ...row.externalRuntimeWriterAuditEventRefs,
      ...row.childDeliveryAuditEventRefs,
    ]),
    reportRuntimeRefs: row.reportRuntimeRefs,
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function externalRuntimeDeliveryHandoffRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceExternalRuntimeDeviceDeliveryProofVersion: string;
    readonly sourceExternalRuntimeDeviceDeliveryRowId: string;
    readonly sourceExternalRuntimeEvidenceState: string;
    readonly sourceRuntimeWriterEnvelopeRef: string;
    readonly sourceDeliveryResultReceiptRef: string;
    readonly sourceExternalRuntimeWriterTargetRefs: readonly unknown[];
    readonly sourceChildDeliveryAuditEventRefs: readonly unknown[];
    readonly externalRuntimeDeliveryHandoffState: string;
    readonly externalRuntimeHandoffPacketRef: string;
    readonly externalRuntimeWriterQueueRef: string;
    readonly externalRuntimeWriterDispatchAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  sourceExternalRuntimeDeviceDeliveryProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.sourceExternalRuntimeEvidenceState === 'manual-required' &&
        row.externalRuntimeDeliveryHandoffState === 'manual-required'
      : row.sourceExternalRuntimeEvidenceState === 'external-runtime-evidence-ready' &&
        row.externalRuntimeDeliveryHandoffState === 'handoff-packet-ready';
  return (
    stateMatches &&
    row.sourceExternalRuntimeDeviceDeliveryProofVersion === sourceExternalRuntimeDeviceDeliveryProofVersion &&
    row.sourceExternalRuntimeDeviceDeliveryRowId.length > 0 &&
    row.sourceRuntimeWriterEnvelopeRef.length > 0 &&
    row.sourceDeliveryResultReceiptRef.length > 0 &&
    row.sourceExternalRuntimeWriterTargetRefs.length > 0 &&
    row.sourceChildDeliveryAuditEventRefs.length > 0 &&
    row.externalRuntimeHandoffPacketRef.length > 0 &&
    row.externalRuntimeWriterQueueRef.length > 0 &&
    row.externalRuntimeWriterDispatchAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeDeliveryHandoffProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeDeviceDeliveryProofVersion: string;
    readonly externalRuntimeDeliveryHandoffRows: readonly {
      readonly sourceDecisionAction: string;
      readonly externalRuntimeDeliveryHandoffState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeDeviceDeliveryProofVersion: string,
  requiredActions: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeDeliveryHandoffRows.map((row) => row.sourceDecisionAction));
  const handoffStates = new Set(proof.externalRuntimeDeliveryHandoffRows.map((row) => row.externalRuntimeDeliveryHandoffState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeDeviceDeliveryProofVersion === sourceExternalRuntimeDeviceDeliveryProofVersion &&
    proof.externalRuntimeDeliveryHandoffRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredStates.every((state) => handoffStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeWriterReadinessProofGenerated(proof: {
  readonly externalRuntimeWriterReadinessRows: readonly {
    readonly externalRuntimeWriterReadinessState: string;
    readonly externalRuntimeWriterQueueState: string;
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeWriterReadinessRows: proof.externalRuntimeWriterReadinessRows.length,
    writerHandoffReadyRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterReadinessState === 'writer-handoff-ready'
    ).length,
    queuePreflightReadyRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterQueueState === 'queue-preflight-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterReadinessState === 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeWriterReadinessRowGenerated(
  row: {
    readonly externalRuntimeDeviceDeliveryRowId: string;
    readonly sourceDecisionAction: string;
    readonly externalRuntimeEvidenceState: string;
    readonly sourceRuntimeWriterEnvelopeRef: string;
    readonly sourceDeliveryResultReceiptRef: string;
    readonly externalRuntimeWriterTargetRefs: readonly string[];
    readonly externalRuntimeWriterAuditEventRefs: readonly string[];
    readonly childDeliveryAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeDeviceDeliveryProofVersion: string,
  claimBoundary: string,
  classifiedAt: string
) {
  const ready = row.externalRuntimeEvidenceState === 'external-runtime-evidence-ready';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-writer-readiness-proof',
    externalRuntimeWriterReadinessRowId: `external-runtime-writer-readiness-${row.sourceDecisionAction}`,
    sourceExternalRuntimeDeviceDeliveryProofVersion,
    sourceExternalRuntimeDeviceDeliveryRowId: row.externalRuntimeDeviceDeliveryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeEvidenceState: row.externalRuntimeEvidenceState,
    sourceRuntimeWriterEnvelopeRef: row.sourceRuntimeWriterEnvelopeRef,
    sourceDeliveryResultReceiptRef: row.sourceDeliveryResultReceiptRef,
    sourceExternalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
    externalRuntimeWriterReadinessState: ready ? 'writer-handoff-ready' : 'manual-required',
    externalRuntimeWriterQueueState: ready ? 'queue-preflight-ready' : 'manual-required',
    externalRuntimeWriterPreflightRef: `external-runtime-writer-preflight-${row.sourceDecisionAction}`,
    externalRuntimeWriterReceiptRef: `external-runtime-writer-readiness-receipt-${row.sourceDecisionAction}`,
    externalRuntimeWriterTargetRefs: uniqueRefsGenerated([
      row.sourceRuntimeWriterEnvelopeRef,
      row.sourceDeliveryResultReceiptRef,
      ...row.externalRuntimeWriterTargetRefs,
    ]),
    externalRuntimeWriterAuditEventRefs: row.externalRuntimeWriterAuditEventRefs,
    childDeliveryAuditEventRefs: row.childDeliveryAuditEventRefs,
    reportRuntimeRefs: row.reportRuntimeRefs,
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    classifiedAt,
  } as const;
}

export function externalRuntimeWriterReadinessRowIsHonestGenerated(
  row: {
    readonly sourceExternalRuntimeDeviceDeliveryProofVersion: string;
    readonly sourceExternalRuntimeDeviceDeliveryRowId: string;
    readonly sourceExternalRuntimeEvidenceState: string;
    readonly sourceRuntimeWriterEnvelopeRef: string;
    readonly sourceDeliveryResultReceiptRef: string;
    readonly sourceExternalRuntimeWriterTargetRefs: readonly unknown[];
    readonly externalRuntimeWriterReadinessState: string;
    readonly externalRuntimeWriterQueueState: string;
    readonly externalRuntimeWriterPreflightRef: string;
    readonly externalRuntimeWriterReceiptRef: string;
    readonly externalRuntimeWriterTargetRefs: readonly unknown[];
    readonly externalRuntimeWriterAuditEventRefs: readonly unknown[];
    readonly childDeliveryAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  sourceExternalRuntimeDeviceDeliveryProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceExternalRuntimeEvidenceState === 'manual-required'
      ? row.externalRuntimeWriterReadinessState === 'manual-required' &&
        row.externalRuntimeWriterQueueState === 'manual-required'
      : row.externalRuntimeWriterReadinessState === 'writer-handoff-ready' &&
        row.externalRuntimeWriterQueueState === 'queue-preflight-ready';
  return (
    stateMatches &&
    row.sourceExternalRuntimeDeviceDeliveryProofVersion === sourceExternalRuntimeDeviceDeliveryProofVersion &&
    row.sourceExternalRuntimeDeviceDeliveryRowId.length > 0 &&
    row.sourceRuntimeWriterEnvelopeRef.length > 0 &&
    row.sourceDeliveryResultReceiptRef.length > 0 &&
    row.sourceExternalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterPreflightRef.length > 0 &&
    row.externalRuntimeWriterReceiptRef.length > 0 &&
    row.externalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterAuditEventRefs.length > 0 &&
    row.childDeliveryAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeWriterReadinessProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeDeviceDeliveryProofVersion: string;
    readonly externalRuntimeWriterReadinessRows: readonly {
      readonly sourceDecisionAction: string;
      readonly externalRuntimeWriterReadinessState: string;
      readonly externalRuntimeWriterQueueState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeDeviceDeliveryProofVersion: string,
  requiredActions: readonly string[],
  requiredReadinessStates: readonly string[],
  requiredQueueStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeWriterReadinessRows.map((row) => row.sourceDecisionAction));
  const readinessStates = new Set(proof.externalRuntimeWriterReadinessRows.map((row) => row.externalRuntimeWriterReadinessState));
  const queueStates = new Set(proof.externalRuntimeWriterReadinessRows.map((row) => row.externalRuntimeWriterQueueState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeDeviceDeliveryProofVersion === sourceExternalRuntimeDeviceDeliveryProofVersion &&
    proof.externalRuntimeWriterReadinessRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredReadinessStates.every((state) => readinessStates.has(state)) &&
    requiredQueueStates.every((state) => queueStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofGenerated(proof: {
  readonly externalRuntimeWriterDeliveryBoundaryRows: readonly {
    readonly externalRuntimeWriterDeliveryBoundaryState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeWriterDeliveryBoundaryRows: proof.externalRuntimeWriterDeliveryBoundaryRows.length,
    prerequisiteReadyRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryBoundaryState === 'runtime-writer-delivery-prerequisites-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryBoundaryState === 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
    childDeviceDeliveredRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowGenerated(
  row: {
    readonly externalRuntimeDeliveryHandoffRowId: string;
    readonly sourceDecisionAction: string;
    readonly externalRuntimeDeliveryHandoffState: string;
    readonly externalRuntimeHandoffPacketRef: string;
    readonly externalRuntimeWriterQueueRef: string;
    readonly externalRuntimeWriterDispatchAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeDeliveryHandoffProofVersion: string,
  claimBoundary: string,
  linkedAt: string
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-writer-delivery-boundary-proof',
    externalRuntimeWriterDeliveryBoundaryRowId: `external-runtime-writer-delivery-boundary-${row.sourceDecisionAction}`,
    sourceExternalRuntimeDeliveryHandoffProofVersion: sourceExternalRuntimeDeliveryHandoffProofVersion,
    sourceExternalRuntimeDeliveryHandoffRowId: row.externalRuntimeDeliveryHandoffRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeDeliveryHandoffState: row.externalRuntimeDeliveryHandoffState,
    sourceExternalRuntimeHandoffPacketRef: row.externalRuntimeHandoffPacketRef,
    sourceExternalRuntimeWriterQueueRef: row.externalRuntimeWriterQueueRef,
    sourceExternalRuntimeWriterDispatchAuditEventRefs: row.externalRuntimeWriterDispatchAuditEventRefs,
    sourceReportRuntimeRefs: row.reportRuntimeRefs,
    externalRuntimeWriterDeliveryBoundaryState: manual
      ? 'manual-required'
      : 'runtime-writer-delivery-prerequisites-ready',
    requiredExternalWriterTransportProofRefs: [`external-writer-transport-proof-${row.sourceDecisionAction}`],
    requiredPlatformAdapterProofRefs: [`platform-adapter-proof-${row.sourceDecisionAction}`],
    requiredProviderStoreProofRefs: [`provider-store-execution-proof-${row.sourceDecisionAction}`],
    requiredChildDeviceDeliveryProofRefs: [`child-device-delivery-proof-${row.sourceDecisionAction}`],
    externalRuntimeWriterDeliveryReadinessAuditEventRefs: uniqueRefsGenerated([
      ...row.externalRuntimeWriterDispatchAuditEventRefs,
      `external-runtime-writer-delivery-boundary-audit-${row.sourceDecisionAction}`,
    ]),
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    linkedAt,
  } as const;
}

export function externalRuntimeWriterDeliveryBoundaryRowIsHonestGenerated(
  row: {
    readonly sourceDecisionAction: string;
    readonly sourceExternalRuntimeDeliveryHandoffProofVersion: string;
    readonly sourceExternalRuntimeDeliveryHandoffRowId: string;
    readonly sourceExternalRuntimeDeliveryHandoffState: string;
    readonly sourceExternalRuntimeHandoffPacketRef: string;
    readonly sourceExternalRuntimeWriterQueueRef: string;
    readonly sourceExternalRuntimeWriterDispatchAuditEventRefs: readonly unknown[];
    readonly sourceReportRuntimeRefs: readonly unknown[];
    readonly externalRuntimeWriterDeliveryBoundaryState: string;
    readonly requiredExternalWriterTransportProofRefs: readonly unknown[];
    readonly requiredPlatformAdapterProofRefs: readonly unknown[];
    readonly requiredProviderStoreProofRefs: readonly unknown[];
    readonly requiredChildDeviceDeliveryProofRefs: readonly unknown[];
    readonly externalRuntimeWriterDeliveryReadinessAuditEventRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  sourceExternalRuntimeDeliveryHandoffProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceDecisionAction === 'review-needed'
      ? row.sourceExternalRuntimeDeliveryHandoffState === 'manual-required' &&
        row.externalRuntimeWriterDeliveryBoundaryState === 'manual-required'
      : row.sourceExternalRuntimeDeliveryHandoffState === 'handoff-packet-ready' &&
        row.externalRuntimeWriterDeliveryBoundaryState === 'runtime-writer-delivery-prerequisites-ready';
  return (
    stateMatches &&
    row.sourceExternalRuntimeDeliveryHandoffProofVersion === sourceExternalRuntimeDeliveryHandoffProofVersion &&
    row.sourceExternalRuntimeDeliveryHandoffRowId.length > 0 &&
    row.sourceExternalRuntimeHandoffPacketRef.length > 0 &&
    row.sourceExternalRuntimeWriterQueueRef.length > 0 &&
    row.sourceExternalRuntimeWriterDispatchAuditEventRefs.length > 0 &&
    row.sourceReportRuntimeRefs.length > 0 &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.requiredChildDeviceDeliveryProofRefs.length > 0 &&
    row.externalRuntimeWriterDeliveryReadinessAuditEventRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeWriterDeliveryBoundaryProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeDeliveryHandoffProofVersion: string;
    readonly externalRuntimeWriterDeliveryBoundaryRows: readonly {
      readonly sourceDecisionAction: string;
      readonly externalRuntimeWriterDeliveryBoundaryState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeDeliveryHandoffProofVersion: string,
  requiredActions: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeWriterDeliveryBoundaryRows.map((row) => row.sourceDecisionAction));
  const states = new Set(proof.externalRuntimeWriterDeliveryBoundaryRows.map((row) => row.externalRuntimeWriterDeliveryBoundaryState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeDeliveryHandoffProofVersion === sourceExternalRuntimeDeliveryHandoffProofVersion &&
    proof.externalRuntimeWriterDeliveryBoundaryRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofGenerated(proof: {
  readonly externalRuntimeWriterDeliveryBlockerRows: readonly {
    readonly deliveryBlockerState: string;
    readonly deliveryAttemptState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeWriterDeliveryBlockerRows: proof.externalRuntimeWriterDeliveryBlockerRows.length,
    blockedRuntimePrerequisiteRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.deliveryBlockerState === 'blocked-runtime-prerequisites-missing'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.deliveryBlockerState === 'manual-required'
    ).length,
    deliveryAttemptStartedRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.deliveryAttemptState !== 'not-started'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowGenerated(
  row: {
    readonly externalRuntimeWriterDeliveryBoundaryRowId: string;
    readonly sourceDecisionAction: string;
    readonly externalRuntimeWriterDeliveryBoundaryState: string;
    readonly sourceExternalRuntimeWriterQueueRef: string;
    readonly requiredExternalWriterTransportProofRefs: readonly string[];
    readonly requiredPlatformAdapterProofRefs: readonly string[];
    readonly requiredProviderStoreProofRefs: readonly string[];
    readonly requiredChildDeviceDeliveryProofRefs: readonly string[];
    readonly externalRuntimeWriterDeliveryReadinessAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: string,
  requiredRuntimeBlockers: readonly string[],
  claimBoundary: string,
  blockedAt: string
) {
  const manual = row.externalRuntimeWriterDeliveryBoundaryState === 'manual-required';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-writer-delivery-blocker-proof',
    externalRuntimeWriterDeliveryBlockerRowId: `external-runtime-writer-delivery-blocker-${row.sourceDecisionAction}`,
    sourceExternalRuntimeWriterDeliveryBoundaryProofVersion,
    sourceExternalRuntimeWriterDeliveryBoundaryRowId: row.externalRuntimeWriterDeliveryBoundaryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeWriterDeliveryBoundaryState: row.externalRuntimeWriterDeliveryBoundaryState,
    sourceExternalRuntimeWriterQueueRef: row.sourceExternalRuntimeWriterQueueRef,
    requiredExternalWriterTransportProofRefs: row.requiredExternalWriterTransportProofRefs,
    requiredPlatformAdapterProofRefs: row.requiredPlatformAdapterProofRefs,
    requiredProviderStoreProofRefs: row.requiredProviderStoreProofRefs,
    requiredChildDeviceDeliveryProofRefs: row.requiredChildDeviceDeliveryProofRefs,
    deliveryBlockerState: manual ? 'manual-required' : 'blocked-runtime-prerequisites-missing',
    deliveryAttemptState: 'not-started',
    requiredRuntimeBlockers,
    manualBlockerRefs: [
      `missing-external-writer-transport-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-${row.sourceDecisionAction}`,
      `missing-child-device-transport-${row.sourceDecisionAction}`,
    ],
    deliveryBlockerAuditEventRefs: uniqueRefsGenerated([
      ...row.externalRuntimeWriterDeliveryReadinessAuditEventRefs,
      `external-runtime-writer-delivery-blocker-audit-${row.sourceDecisionAction}`,
    ]),
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    blockedAt,
  } as const;
}

export function externalRuntimeWriterDeliveryBlockerRowIsHonestGenerated(
  row: {
    readonly sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: string;
    readonly sourceExternalRuntimeWriterDeliveryBoundaryRowId: string;
    readonly sourceExternalRuntimeWriterDeliveryBoundaryState: string;
    readonly sourceExternalRuntimeWriterQueueRef: string;
    readonly requiredExternalWriterTransportProofRefs: readonly unknown[];
    readonly requiredPlatformAdapterProofRefs: readonly unknown[];
    readonly requiredProviderStoreProofRefs: readonly unknown[];
    readonly requiredChildDeviceDeliveryProofRefs: readonly unknown[];
    readonly deliveryBlockerState: string;
    readonly deliveryAttemptState: string;
    readonly requiredRuntimeBlockers: readonly string[];
    readonly manualBlockerRefs: readonly unknown[];
    readonly deliveryBlockerAuditEventRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: string,
  requiredRuntimeBlockers: readonly string[],
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceExternalRuntimeWriterDeliveryBoundaryState === 'manual-required'
      ? row.deliveryBlockerState === 'manual-required'
      : row.deliveryBlockerState === 'blocked-runtime-prerequisites-missing';
  return (
    stateMatches &&
    row.sourceExternalRuntimeWriterDeliveryBoundaryProofVersion === sourceExternalRuntimeWriterDeliveryBoundaryProofVersion &&
    row.sourceExternalRuntimeWriterDeliveryBoundaryRowId.length > 0 &&
    row.sourceExternalRuntimeWriterQueueRef.length > 0 &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.requiredChildDeviceDeliveryProofRefs.length > 0 &&
    requiredRuntimeBlockers.every((blocker) => row.requiredRuntimeBlockers.includes(blocker)) &&
    row.manualBlockerRefs.length === requiredRuntimeBlockers.length &&
    row.deliveryBlockerAuditEventRefs.length > 0 &&
    row.deliveryAttemptState === 'not-started' &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeWriterDeliveryBlockerProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: string;
    readonly externalRuntimeWriterDeliveryBlockerRows: readonly {
      readonly sourceDecisionAction: string;
      readonly deliveryBlockerState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: string,
  requiredActions: readonly string[],
  requiredStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeWriterDeliveryBlockerRows.map((row) => row.sourceDecisionAction));
  const states = new Set(proof.externalRuntimeWriterDeliveryBlockerRows.map((row) => row.deliveryBlockerState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeWriterDeliveryBoundaryProofVersion === sourceExternalRuntimeWriterDeliveryBoundaryProofVersion &&
    proof.externalRuntimeWriterDeliveryBlockerRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredStates.every((state) => states.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeTransportQueueProofGenerated(proof: {
  readonly externalRuntimeTransportQueueRows: readonly {
    readonly externalRuntimeTransportQueueState: string;
    readonly externalRuntimeTransportDispatchState: string;
    readonly externalRuntimeTransportRetryState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeTransportQueueRows: proof.externalRuntimeTransportQueueRows.length,
    queuedBlockedRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportQueueState === 'queued-blocked'
    ).length,
    manualRequiredRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportQueueState === 'manual-required'
    ).length,
    dispatchBlockedRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportDispatchState === 'dispatch-blocked'
    ).length,
    retryScheduledRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportRetryState !== 'not-scheduled'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeTransportQueueRowGenerated(
  row: {
    readonly externalRuntimeWriterDeliveryBlockerRowId: string;
    readonly sourceDecisionAction: string;
    readonly deliveryBlockerState: string;
    readonly deliveryAttemptState: string;
    readonly sourceExternalRuntimeWriterQueueRef: string;
    readonly requiredRuntimeBlockers: readonly string[];
    readonly requiredExternalWriterTransportProofRefs: readonly string[];
    readonly requiredChildDeviceDeliveryProofRefs: readonly string[];
    readonly requiredProviderStoreProofRefs: readonly string[];
    readonly requiredPlatformAdapterProofRefs: readonly string[];
    readonly manualBlockerRefs: readonly string[];
    readonly deliveryBlockerAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeWriterDeliveryBlockerProofVersion: string,
  claimBoundary: string,
  queuedAt: string
) {
  const manual = row.deliveryBlockerState === 'manual-required';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-transport-queue-proof',
    externalRuntimeTransportQueueRowId: `external-runtime-transport-queue-${row.sourceDecisionAction}`,
    sourceExternalRuntimeWriterDeliveryBlockerProofVersion,
    sourceExternalRuntimeWriterDeliveryBlockerRowId: row.externalRuntimeWriterDeliveryBlockerRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceDeliveryBlockerState: row.deliveryBlockerState,
    sourceDeliveryAttemptState: row.deliveryAttemptState,
    sourceExternalRuntimeWriterQueueRef: row.sourceExternalRuntimeWriterQueueRef,
    externalRuntimeTransportQueueState: manual ? 'manual-required' : 'queued-blocked',
    externalRuntimeTransportDispatchState: manual ? 'manual-required' : 'dispatch-blocked',
    externalRuntimeTransportRetryState: manual ? 'manual-required' : 'not-scheduled',
    parentOwnedTransportQueueRef: `parent-owned-external-runtime-transport-queue-${row.sourceDecisionAction}`,
    queueGuardAuditEventRefs: uniqueRefsGenerated([
      ...row.deliveryBlockerAuditEventRefs,
      `external-runtime-transport-queue-guard-audit-${row.sourceDecisionAction}`,
    ]),
    requiredRuntimeBlockers: row.requiredRuntimeBlockers,
    requiredExternalWriterTransportProofRefs: row.requiredExternalWriterTransportProofRefs,
    requiredChildDeviceTransportProofRefs: row.requiredChildDeviceDeliveryProofRefs,
    requiredProviderStoreProofRefs: row.requiredProviderStoreProofRefs,
    requiredPlatformAdapterProofRefs: row.requiredPlatformAdapterProofRefs,
    blockedDispatchReasonRefs: row.manualBlockerRefs,
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    queuedAt,
  } as const;
}

export function externalRuntimeTransportQueueRowIsHonestGenerated(
  row: {
    readonly sourceExternalRuntimeWriterDeliveryBlockerProofVersion: string;
    readonly sourceExternalRuntimeWriterDeliveryBlockerRowId: string;
    readonly sourceDeliveryAttemptState: string;
    readonly parentOwnedTransportQueueRef: string;
    readonly queueGuardAuditEventRefs: readonly unknown[];
    readonly requiredRuntimeBlockers: readonly string[];
    readonly requiredExternalWriterTransportProofRefs: readonly unknown[];
    readonly requiredChildDeviceTransportProofRefs: readonly unknown[];
    readonly requiredProviderStoreProofRefs: readonly unknown[];
    readonly requiredPlatformAdapterProofRefs: readonly unknown[];
    readonly blockedDispatchReasonRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
    readonly sourceDeliveryBlockerState: string;
    readonly externalRuntimeTransportQueueState: string;
    readonly externalRuntimeTransportDispatchState: string;
    readonly externalRuntimeTransportRetryState: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceExternalRuntimeWriterDeliveryBlockerProofVersion: string,
  requiredRuntimeBlockers: readonly string[],
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceDeliveryBlockerState === 'manual-required'
      ? row.externalRuntimeTransportQueueState === 'manual-required' &&
        row.externalRuntimeTransportDispatchState === 'manual-required' &&
        row.externalRuntimeTransportRetryState === 'manual-required'
      : row.externalRuntimeTransportQueueState === 'queued-blocked' &&
        row.externalRuntimeTransportDispatchState === 'dispatch-blocked' &&
        row.externalRuntimeTransportRetryState === 'not-scheduled';
  return (
    stateMatches &&
    row.sourceExternalRuntimeWriterDeliveryBlockerProofVersion ===
      sourceExternalRuntimeWriterDeliveryBlockerProofVersion &&
    row.sourceExternalRuntimeWriterDeliveryBlockerRowId.length > 0 &&
    row.sourceDeliveryAttemptState === 'not-started' &&
    row.parentOwnedTransportQueueRef.length > 0 &&
    row.queueGuardAuditEventRefs.length > 0 &&
    requiredRuntimeBlockers.every((blocker) => row.requiredRuntimeBlockers.includes(blocker)) &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredChildDeviceTransportProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.blockedDispatchReasonRefs.length === requiredRuntimeBlockers.length &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeTransportQueueProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeWriterDeliveryBlockerProofVersion: string;
    readonly externalRuntimeTransportQueueRows: readonly {
      readonly sourceDecisionAction: string;
      readonly externalRuntimeTransportQueueState: string;
      readonly externalRuntimeTransportDispatchState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeWriterDeliveryBlockerProofVersion: string,
  requiredActions: readonly string[],
  requiredQueueStates: readonly string[],
  requiredDispatchStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeTransportQueueRows.map((row) => row.sourceDecisionAction));
  const queueStates = new Set(
    proof.externalRuntimeTransportQueueRows.map((row) => row.externalRuntimeTransportQueueState)
  );
  const dispatchStates = new Set(
    proof.externalRuntimeTransportQueueRows.map((row) => row.externalRuntimeTransportDispatchState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeWriterDeliveryBlockerProofVersion ===
      sourceExternalRuntimeWriterDeliveryBlockerProofVersion &&
    proof.externalRuntimeTransportQueueRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredQueueStates.every((state) => queueStates.has(state)) &&
    requiredDispatchStates.every((state) => dispatchStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofGenerated(proof: {
  readonly externalRuntimeTransportDispatchPreflightRows: readonly {
    readonly dispatchPreflightState: string;
    readonly dispatchPacketState: string;
    readonly dispatchReadinessState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeTransportDispatchPreflightRows: proof.externalRuntimeTransportDispatchPreflightRows.length,
    blockedPreflightRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchPreflightState === 'blocked-waiting-runtime-artifacts'
    ).length,
    manualRequiredRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchPreflightState === 'manual-required'
    ).length,
    withheldDispatchPackets: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchPacketState === 'withheld'
    ).length,
    readyDispatchRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchReadinessState !== 'not-ready' && row.dispatchReadinessState !== 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowGenerated(
  row: {
    readonly externalRuntimeTransportQueueRowId: string;
    readonly sourceDecisionAction: string;
    readonly externalRuntimeTransportQueueState: string;
    readonly externalRuntimeTransportDispatchState: string;
    readonly parentOwnedTransportQueueRef: string;
    readonly requiredExternalWriterTransportProofRefs: readonly string[];
    readonly requiredProviderStoreProofRefs: readonly string[];
    readonly requiredPlatformAdapterProofRefs: readonly string[];
    readonly requiredChildDeviceTransportProofRefs: readonly string[];
    readonly queueGuardAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeTransportQueueProofVersion: string,
  requiredDispatchArtifactBlockers: readonly string[],
  claimBoundary: string,
  preflightedAt: string
) {
  const manual = row.externalRuntimeTransportQueueState === 'manual-required';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-transport-dispatch-preflight-proof',
    externalRuntimeTransportDispatchPreflightRowId: `external-runtime-transport-dispatch-preflight-${row.sourceDecisionAction}`,
    sourceExternalRuntimeTransportQueueProofVersion,
    sourceExternalRuntimeTransportQueueRowId: row.externalRuntimeTransportQueueRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceTransportQueueState: row.externalRuntimeTransportQueueState,
    sourceTransportDispatchState: row.externalRuntimeTransportDispatchState,
    parentOwnedTransportQueueRef: row.parentOwnedTransportQueueRef,
    parentOwnedDispatchPreflightRef: `parent-owned-external-runtime-dispatch-preflight-${row.sourceDecisionAction}`,
    parentOwnedDispatchPacketRef: `parent-owned-external-runtime-dispatch-packet-${row.sourceDecisionAction}`,
    dispatchPreflightState: manual ? 'manual-required' : 'blocked-waiting-runtime-artifacts',
    dispatchPacketState: manual ? 'manual-required' : 'withheld',
    dispatchReadinessState: manual ? 'manual-required' : 'not-ready',
    requiredDispatchArtifactBlockers,
    externalWriterTransportHandlerProofRefs: row.requiredExternalWriterTransportProofRefs,
    providerStoreExecutionHandlerProofRefs: row.requiredProviderStoreProofRefs,
    platformAdapterExecutionHandlerProofRefs: row.requiredPlatformAdapterProofRefs,
    childDeviceTransportReceiptProofRefs: row.requiredChildDeviceTransportProofRefs,
    dispatchBlockedReasonRefs: [
      `missing-external-writer-transport-handler-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-handler-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-handler-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    dispatchPreflightAuditEventRefs: [
      ...row.queueGuardAuditEventRefs,
      `external-runtime-transport-dispatch-preflight-audit-${row.sourceDecisionAction}`,
    ],
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    preflightedAt,
  } as const;
}

export function dispatchPreflightRowIsHonestGenerated(
  row: {
    readonly sourceExternalRuntimeTransportQueueProofVersion: string;
    readonly sourceExternalRuntimeTransportQueueRowId: string;
    readonly parentOwnedTransportQueueRef: string;
    readonly parentOwnedDispatchPreflightRef: string;
    readonly parentOwnedDispatchPacketRef: string;
    readonly requiredDispatchArtifactBlockers: readonly string[];
    readonly externalWriterTransportHandlerProofRefs: readonly unknown[];
    readonly providerStoreExecutionHandlerProofRefs: readonly unknown[];
    readonly platformAdapterExecutionHandlerProofRefs: readonly unknown[];
    readonly childDeviceTransportReceiptProofRefs: readonly unknown[];
    readonly dispatchBlockedReasonRefs: readonly unknown[];
    readonly dispatchPreflightAuditEventRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
    readonly sourceTransportQueueState: string;
    readonly sourceTransportDispatchState: string;
    readonly dispatchPreflightState: string;
    readonly dispatchPacketState: string;
    readonly dispatchReadinessState: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceExternalRuntimeTransportQueueProofVersion: string,
  requiredDispatchArtifactBlockers: readonly string[],
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceTransportQueueState === 'manual-required'
      ? row.sourceTransportDispatchState === 'manual-required' &&
        row.dispatchPreflightState === 'manual-required' &&
        row.dispatchPacketState === 'manual-required' &&
        row.dispatchReadinessState === 'manual-required'
      : row.sourceTransportDispatchState === 'dispatch-blocked' &&
        row.dispatchPreflightState === 'blocked-waiting-runtime-artifacts' &&
        row.dispatchPacketState === 'withheld' &&
        row.dispatchReadinessState === 'not-ready';
  return (
    stateMatches &&
    row.sourceExternalRuntimeTransportQueueProofVersion === sourceExternalRuntimeTransportQueueProofVersion &&
    row.sourceExternalRuntimeTransportQueueRowId.length > 0 &&
    row.parentOwnedTransportQueueRef.length > 0 &&
    row.parentOwnedDispatchPreflightRef.length > 0 &&
    row.parentOwnedDispatchPacketRef.length > 0 &&
    requiredDispatchArtifactBlockers.every((blocker) =>
      row.requiredDispatchArtifactBlockers.includes(blocker)
    ) &&
    row.externalWriterTransportHandlerProofRefs.length > 0 &&
    row.providerStoreExecutionHandlerProofRefs.length > 0 &&
    row.platformAdapterExecutionHandlerProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.dispatchBlockedReasonRefs.length === requiredDispatchArtifactBlockers.length &&
    row.dispatchPreflightAuditEventRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function dispatchPreflightProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeTransportQueueProofVersion: string;
    readonly externalRuntimeTransportDispatchPreflightRows: readonly {
      readonly sourceDecisionAction: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeTransportQueueProofVersion: string,
  requiredActions: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(
    proof.externalRuntimeTransportDispatchPreflightRows.map((row) => row.sourceDecisionAction)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeTransportQueueProofVersion === sourceExternalRuntimeTransportQueueProofVersion &&
    proof.externalRuntimeTransportDispatchPreflightRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeWriterTransportPreflightProofGenerated(proof: {
  readonly externalRuntimeWriterTransportPreflightRows: readonly {
    readonly externalRuntimeWriterTransportPreflightState: string;
    readonly externalRuntimeWriterTransportChannelState: string;
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
  }[];
}) {
  return {
    externalRuntimeWriterTransportPreflightRows: proof.externalRuntimeWriterTransportPreflightRows.length,
    transportPreflightReadyRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterTransportPreflightState === 'transport-preflight-ready'
    ).length,
    parentOwnedQueueRefReadyRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterTransportChannelState === 'parent-owned-queue-ref-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterTransportPreflightState === 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeWriterTransportPreflightRowGenerated(
  row: {
    readonly externalRuntimeWriterReadinessRowId: string;
    readonly sourceDecisionAction: string;
    readonly externalRuntimeWriterReadinessState: string;
    readonly externalRuntimeWriterQueueState: string;
    readonly externalRuntimeWriterPreflightRef: string;
    readonly externalRuntimeWriterReceiptRef: string;
    readonly externalRuntimeWriterTargetRefs: readonly string[];
    readonly externalRuntimeWriterAuditEventRefs: readonly string[];
    readonly childDeliveryAuditEventRefs: readonly string[];
    readonly reportRuntimeRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceExternalRuntimeWriterReadinessProofVersion: string,
  claimBoundary: string,
  classifiedAt: string
) {
  const ready =
    row.externalRuntimeWriterReadinessState === 'writer-handoff-ready' &&
    row.externalRuntimeWriterQueueState === 'queue-preflight-ready';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-writer-transport-preflight-proof',
    externalRuntimeWriterTransportPreflightRowId: `external-runtime-writer-transport-preflight-${row.sourceDecisionAction}`,
    sourceExternalRuntimeWriterReadinessProofVersion,
    sourceExternalRuntimeWriterReadinessRowId: row.externalRuntimeWriterReadinessRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeWriterReadinessState: row.externalRuntimeWriterReadinessState,
    sourceExternalRuntimeWriterQueueState: row.externalRuntimeWriterQueueState,
    sourceExternalRuntimeWriterPreflightRef: row.externalRuntimeWriterPreflightRef,
    sourceExternalRuntimeWriterReceiptRef: row.externalRuntimeWriterReceiptRef,
    sourceExternalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
    externalRuntimeWriterTransportPreflightState: ready ? 'transport-preflight-ready' : 'manual-required',
    externalRuntimeWriterTransportChannelState: ready ? 'parent-owned-queue-ref-ready' : 'manual-required',
    externalRuntimeWriterTransportPreflightRef: `external-runtime-writer-transport-preflight-ref-${row.sourceDecisionAction}`,
    requiredExternalWriterTransportProofRefs: [
      `external-runtime-writer-transport-proof-${row.sourceDecisionAction}`,
      row.externalRuntimeWriterPreflightRef,
    ],
    requiredExternalWriterQueueProofRefs: [
      `external-runtime-writer-queue-proof-${row.sourceDecisionAction}`,
      row.externalRuntimeWriterReceiptRef,
    ],
    requiredChildDeviceTransportProofRefs: [
      `external-runtime-writer-child-device-transport-proof-${row.sourceDecisionAction}`,
      ...row.childDeliveryAuditEventRefs,
    ],
    requiredPlatformAdapterProofRefs: [`external-runtime-writer-platform-adapter-proof-${row.sourceDecisionAction}`],
    requiredProviderStoreProofRefs: [`external-runtime-writer-provider-store-proof-${row.sourceDecisionAction}`],
    externalRuntimeWriterAuditEventRefs: row.externalRuntimeWriterAuditEventRefs,
    childDeliveryAuditEventRefs: row.childDeliveryAuditEventRefs,
    reportRuntimeRefs: row.reportRuntimeRefs,
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    classifiedAt,
  } as const;
}

export function externalRuntimeWriterTransportPreflightRowIsHonestGenerated(
  row: {
    readonly sourceExternalRuntimeWriterReadinessProofVersion: string;
    readonly sourceExternalRuntimeWriterReadinessRowId: string;
    readonly sourceExternalRuntimeWriterPreflightRef: string;
    readonly sourceExternalRuntimeWriterReceiptRef: string;
    readonly sourceExternalRuntimeWriterTargetRefs: readonly unknown[];
    readonly externalRuntimeWriterTransportPreflightRef: string;
    readonly requiredExternalWriterTransportProofRefs: readonly unknown[];
    readonly requiredExternalWriterQueueProofRefs: readonly unknown[];
    readonly requiredChildDeviceTransportProofRefs: readonly unknown[];
    readonly requiredPlatformAdapterProofRefs: readonly unknown[];
    readonly requiredProviderStoreProofRefs: readonly unknown[];
    readonly externalRuntimeWriterAuditEventRefs: readonly unknown[];
    readonly childDeliveryAuditEventRefs: readonly unknown[];
    readonly reportRuntimeRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
    readonly sourceExternalRuntimeWriterReadinessState: string;
    readonly sourceExternalRuntimeWriterQueueState: string;
    readonly externalRuntimeWriterTransportPreflightState: string;
    readonly externalRuntimeWriterTransportChannelState: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceExternalRuntimeWriterReadinessProofVersion: string,
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceExternalRuntimeWriterReadinessState === 'manual-required' ||
    row.sourceExternalRuntimeWriterQueueState === 'manual-required'
      ? row.externalRuntimeWriterTransportPreflightState === 'manual-required' &&
        row.externalRuntimeWriterTransportChannelState === 'manual-required'
      : row.externalRuntimeWriterTransportPreflightState === 'transport-preflight-ready' &&
        row.externalRuntimeWriterTransportChannelState === 'parent-owned-queue-ref-ready';
  return (
    stateMatches &&
    row.sourceExternalRuntimeWriterReadinessProofVersion === sourceExternalRuntimeWriterReadinessProofVersion &&
    row.sourceExternalRuntimeWriterReadinessRowId.length > 0 &&
    row.sourceExternalRuntimeWriterPreflightRef.length > 0 &&
    row.sourceExternalRuntimeWriterReceiptRef.length > 0 &&
    row.sourceExternalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterTransportPreflightRef.length > 0 &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredExternalWriterQueueProofRefs.length > 0 &&
    row.requiredChildDeviceTransportProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.externalRuntimeWriterAuditEventRefs.length > 0 &&
    row.childDeliveryAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeWriterTransportPreflightProofIsHonestGenerated(
  proof: {
    readonly sourceExternalRuntimeWriterReadinessProofVersion: string;
    readonly externalRuntimeWriterTransportPreflightRows: readonly {
      readonly sourceDecisionAction: string;
      readonly externalRuntimeWriterTransportPreflightState: string;
      readonly externalRuntimeWriterTransportChannelState: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceExternalRuntimeWriterReadinessProofVersion: string,
  requiredActions: readonly string[],
  requiredPreflightStates: readonly string[],
  requiredChannelStates: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(
    proof.externalRuntimeWriterTransportPreflightRows.map((row) => row.sourceDecisionAction)
  );
  const preflightStates = new Set(
    proof.externalRuntimeWriterTransportPreflightRows.map(
      (row) => row.externalRuntimeWriterTransportPreflightState
    )
  );
  const channelStates = new Set(
    proof.externalRuntimeWriterTransportPreflightRows.map((row) => row.externalRuntimeWriterTransportChannelState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeWriterReadinessProofVersion === sourceExternalRuntimeWriterReadinessProofVersion &&
    proof.externalRuntimeWriterTransportPreflightRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredPreflightStates.every((state) => preflightStates.has(state)) &&
    requiredChannelStates.every((state) => channelStates.has(state)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofGenerated(proof: {
  readonly runtimeDeliveryReceiptBoundaryRows: readonly {
    readonly runtimeDeliveryReceiptBoundaryState: string;
    readonly childDeviceTransportReceiptState: string;
    readonly runtimeDeliveryReceiptReadinessState: string;
    readonly childDeviceDeliveryClaim: string;
  }[];
}) {
  return {
    runtimeDeliveryReceiptBoundaryRows: proof.runtimeDeliveryReceiptBoundaryRows.length,
    blockedReceiptRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.runtimeDeliveryReceiptBoundaryState === 'receipt-blocked-waiting-runtime-artifacts'
    ).length,
    manualRequiredRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.runtimeDeliveryReceiptBoundaryState === 'manual-required'
    ).length,
    receiptMissingRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.childDeviceTransportReceiptState === 'receipt-missing'
    ).length,
    readyReceiptRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) =>
        row.runtimeDeliveryReceiptReadinessState !== 'not-ready' &&
        row.runtimeDeliveryReceiptReadinessState !== 'manual-required'
    ).length,
    childDeviceDeliveredRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowGenerated(
  row: {
    readonly externalRuntimeTransportDispatchPreflightRowId: string;
    readonly sourceDecisionAction: string;
    readonly dispatchPreflightState: string;
    readonly dispatchPacketState: string;
    readonly parentOwnedDispatchPacketRef: string;
    readonly externalWriterTransportHandlerProofRefs: readonly string[];
    readonly providerStoreExecutionHandlerProofRefs: readonly string[];
    readonly platformAdapterExecutionHandlerProofRefs: readonly string[];
    readonly childDeviceTransportReceiptProofRefs: readonly string[];
    readonly dispatchPreflightAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceDispatchPreflightProofVersion: string,
  requiredReceiptArtifactBlockers: readonly string[],
  claimBoundary: string,
  receiptBoundaryCheckedAt: string
) {
  const manual = row.dispatchPreflightState === 'manual-required';
  return {
    schemaVersion: 'app-install-purchase-runtime-delivery-receipt-boundary-proof',
    runtimeDeliveryReceiptBoundaryRowId: `runtime-delivery-receipt-boundary-${row.sourceDecisionAction}`,
    sourceDispatchPreflightProofVersion,
    sourceDispatchPreflightRowId: row.externalRuntimeTransportDispatchPreflightRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceDispatchPreflightState: row.dispatchPreflightState,
    sourceDispatchPacketState: row.dispatchPacketState,
    sourceParentOwnedDispatchPacketRef: row.parentOwnedDispatchPacketRef,
    parentOwnedReceiptBoundaryRef: `parent-owned-runtime-delivery-receipt-boundary-${row.sourceDecisionAction}`,
    childDeviceTransportReceiptExpectationRef: `child-device-transport-receipt-required-${row.sourceDecisionAction}`,
    runtimeDeliveryReceiptBoundaryState: manual ? 'manual-required' : 'receipt-blocked-waiting-runtime-artifacts',
    childDeviceTransportReceiptState: manual ? 'manual-required' : 'receipt-missing',
    runtimeDeliveryReceiptReadinessState: manual ? 'manual-required' : 'not-ready',
    requiredReceiptArtifactBlockers,
    externalWriterDispatchExecutionProofRefs: row.externalWriterTransportHandlerProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionHandlerProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionHandlerProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    receiptBlockedReasonRefs: [
      `missing-external-writer-dispatch-execution-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    receiptBoundaryAuditEventRefs: [
      ...row.dispatchPreflightAuditEventRefs,
      `runtime-delivery-receipt-boundary-audit-${row.sourceDecisionAction}`,
    ],
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    receiptBoundaryCheckedAt,
  } as const;
}

export function receiptBoundaryRowIsHonestGenerated(
  row: {
    readonly sourceDispatchPreflightProofVersion: string;
    readonly sourceDispatchPreflightRowId: string;
    readonly sourceParentOwnedDispatchPacketRef: string;
    readonly parentOwnedReceiptBoundaryRef: string;
    readonly childDeviceTransportReceiptExpectationRef: string;
    readonly requiredReceiptArtifactBlockers: readonly string[];
    readonly externalWriterDispatchExecutionProofRefs: readonly unknown[];
    readonly providerStoreExecutionReceiptProofRefs: readonly unknown[];
    readonly platformAdapterExecutionReceiptProofRefs: readonly unknown[];
    readonly childDeviceTransportReceiptProofRefs: readonly unknown[];
    readonly receiptBlockedReasonRefs: readonly unknown[];
    readonly receiptBoundaryAuditEventRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
    readonly sourceDispatchPreflightState: string;
    readonly sourceDispatchPacketState: string;
    readonly runtimeDeliveryReceiptBoundaryState: string;
    readonly childDeviceTransportReceiptState: string;
    readonly runtimeDeliveryReceiptReadinessState: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceDispatchPreflightProofVersion: string,
  requiredReceiptArtifactBlockers: readonly string[],
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceDispatchPreflightState === 'manual-required'
      ? row.sourceDispatchPacketState === 'manual-required' &&
        row.runtimeDeliveryReceiptBoundaryState === 'manual-required' &&
        row.childDeviceTransportReceiptState === 'manual-required' &&
        row.runtimeDeliveryReceiptReadinessState === 'manual-required'
      : row.sourceDispatchPacketState === 'withheld' &&
        row.runtimeDeliveryReceiptBoundaryState === 'receipt-blocked-waiting-runtime-artifacts' &&
        row.childDeviceTransportReceiptState === 'receipt-missing' &&
        row.runtimeDeliveryReceiptReadinessState === 'not-ready';
  return (
    stateMatches &&
    row.sourceDispatchPreflightProofVersion === sourceDispatchPreflightProofVersion &&
    row.sourceDispatchPreflightRowId.length > 0 &&
    row.sourceParentOwnedDispatchPacketRef.length > 0 &&
    row.parentOwnedReceiptBoundaryRef.length > 0 &&
    row.childDeviceTransportReceiptExpectationRef.length > 0 &&
    requiredReceiptArtifactBlockers.every((blocker) =>
      row.requiredReceiptArtifactBlockers.includes(blocker)
    ) &&
    row.externalWriterDispatchExecutionProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.receiptBlockedReasonRefs.length === requiredReceiptArtifactBlockers.length &&
    row.receiptBoundaryAuditEventRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function receiptBoundaryProofIsHonestGenerated(
  proof: {
    readonly sourceDispatchPreflightProofVersion: string;
    readonly runtimeDeliveryReceiptBoundaryRows: readonly {
      readonly sourceDecisionAction: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceDispatchPreflightProofVersion: string,
  requiredActions: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.runtimeDeliveryReceiptBoundaryRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceDispatchPreflightProofVersion === sourceDispatchPreflightProofVersion &&
    proof.runtimeDeliveryReceiptBoundaryRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseRuntimeTransportDeliveryExecutionProofGenerated(proof: {
  readonly runtimeTransportDeliveryExecutionRows: readonly {
    readonly runtimeTransportExecutionState: string;
    readonly runtimeTransportAttemptState: string;
    readonly runtimeDeliveryResultState: string;
    readonly childDeviceReceiptHandoffState: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
  }[];
}) {
  return {
    runtimeTransportDeliveryExecutionRows: proof.runtimeTransportDeliveryExecutionRows.length,
    withheldExecutionRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) => row.runtimeTransportExecutionState === 'execution-withheld-missing-artifacts'
    ).length,
    manualRequiredRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) => row.runtimeTransportExecutionState === 'manual-required'
    ).length,
    transportAttemptsStartedRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) =>
        row.runtimeTransportAttemptState !== 'not-started' &&
        row.runtimeTransportAttemptState !== 'manual-required'
    ).length,
    deliveryResultRecordedRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) =>
        row.runtimeDeliveryResultState !== 'result-not-recorded' &&
        row.runtimeDeliveryResultState !== 'manual-required'
    ).length,
    childDeviceReceiptHandoffReadyRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) =>
        row.childDeviceReceiptHandoffState !== 'receipt-handoff-missing' &&
        row.childDeviceReceiptHandoffState !== 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseRuntimeTransportDeliveryExecutionRowGenerated(
  row: {
    readonly runtimeDeliveryReceiptBoundaryRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeDeliveryReceiptBoundaryState: string;
    readonly childDeviceTransportReceiptState: string;
    readonly sourceParentOwnedDispatchPacketRef: string;
    readonly parentOwnedReceiptBoundaryRef: string;
    readonly externalWriterDispatchExecutionProofRefs: readonly string[];
    readonly providerStoreExecutionReceiptProofRefs: readonly string[];
    readonly platformAdapterExecutionReceiptProofRefs: readonly string[];
    readonly childDeviceTransportReceiptProofRefs: readonly string[];
    readonly receiptBoundaryAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceReceiptBoundaryProofVersion: string,
  requiredRuntimeExecutionBlockers: readonly string[],
  claimBoundary: string,
  executionBoundaryCheckedAt: string
) {
  const manual = row.runtimeDeliveryReceiptBoundaryState === 'manual-required';
  return {
    schemaVersion: 'app-install-purchase-runtime-transport-delivery-execution-proof',
    runtimeTransportDeliveryExecutionRowId: `runtime-transport-delivery-execution-${row.sourceDecisionAction}`,
    sourceReceiptBoundaryProofVersion,
    sourceReceiptBoundaryRowId: row.runtimeDeliveryReceiptBoundaryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceReceiptBoundaryState: row.runtimeDeliveryReceiptBoundaryState,
    sourceChildDeviceTransportReceiptState: row.childDeviceTransportReceiptState,
    sourceParentOwnedDispatchPacketRef: row.sourceParentOwnedDispatchPacketRef,
    sourceParentOwnedReceiptBoundaryRef: row.parentOwnedReceiptBoundaryRef,
    parentOwnedTransportExecutionAttemptRef: `parent-owned-runtime-transport-execution-attempt-${row.sourceDecisionAction}`,
    parentOwnedDeliveryResultReceiptRef: `parent-owned-runtime-delivery-result-receipt-${row.sourceDecisionAction}`,
    childDeviceReceiptHandoffRef: `child-device-transport-receipt-handoff-${row.sourceDecisionAction}`,
    runtimeTransportExecutionState: manual ? 'manual-required' : 'execution-withheld-missing-artifacts',
    runtimeTransportAttemptState: manual ? 'manual-required' : 'not-started',
    runtimeDeliveryResultState: manual ? 'manual-required' : 'result-not-recorded',
    childDeviceReceiptHandoffState: manual ? 'manual-required' : 'receipt-handoff-missing',
    requiredRuntimeExecutionBlockers,
    externalWriterDispatchExecutionProofRefs: row.externalWriterDispatchExecutionProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    executionWithheldReasonRefs: [
      `missing-external-writer-dispatch-execution-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    runtimeTransportDeliveryExecutionAuditEventRefs: [
      ...row.receiptBoundaryAuditEventRefs,
      `runtime-transport-delivery-execution-audit-${row.sourceDecisionAction}`,
    ],
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    executionBoundaryCheckedAt,
  } as const;
}

export function runtimeTransportDeliveryExecutionRowIsHonestGenerated(
  row: {
    readonly sourceReceiptBoundaryProofVersion: string;
    readonly sourceReceiptBoundaryRowId: string;
    readonly sourceParentOwnedDispatchPacketRef: string;
    readonly sourceParentOwnedReceiptBoundaryRef: string;
    readonly parentOwnedTransportExecutionAttemptRef: string;
    readonly parentOwnedDeliveryResultReceiptRef: string;
    readonly childDeviceReceiptHandoffRef: string;
    readonly requiredRuntimeExecutionBlockers: readonly string[];
    readonly externalWriterDispatchExecutionProofRefs: readonly unknown[];
    readonly providerStoreExecutionReceiptProofRefs: readonly unknown[];
    readonly platformAdapterExecutionReceiptProofRefs: readonly unknown[];
    readonly childDeviceTransportReceiptProofRefs: readonly unknown[];
    readonly executionWithheldReasonRefs: readonly unknown[];
    readonly runtimeTransportDeliveryExecutionAuditEventRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
    readonly sourceReceiptBoundaryState: string;
    readonly sourceChildDeviceTransportReceiptState: string;
    readonly runtimeTransportExecutionState: string;
    readonly runtimeTransportAttemptState: string;
    readonly runtimeDeliveryResultState: string;
    readonly childDeviceReceiptHandoffState: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceReceiptBoundaryProofVersion: string,
  requiredRuntimeExecutionBlockers: readonly string[],
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceReceiptBoundaryState === 'manual-required'
      ? row.sourceChildDeviceTransportReceiptState === 'manual-required' &&
        row.runtimeTransportExecutionState === 'manual-required' &&
        row.runtimeTransportAttemptState === 'manual-required' &&
        row.runtimeDeliveryResultState === 'manual-required' &&
        row.childDeviceReceiptHandoffState === 'manual-required'
      : row.sourceReceiptBoundaryState === 'receipt-blocked-waiting-runtime-artifacts' &&
        row.sourceChildDeviceTransportReceiptState === 'receipt-missing' &&
        row.runtimeTransportExecutionState === 'execution-withheld-missing-artifacts' &&
        row.runtimeTransportAttemptState === 'not-started' &&
        row.runtimeDeliveryResultState === 'result-not-recorded' &&
        row.childDeviceReceiptHandoffState === 'receipt-handoff-missing';
  return (
    stateMatches &&
    row.sourceReceiptBoundaryProofVersion === sourceReceiptBoundaryProofVersion &&
    row.sourceReceiptBoundaryRowId.length > 0 &&
    row.sourceParentOwnedDispatchPacketRef.length > 0 &&
    row.sourceParentOwnedReceiptBoundaryRef.length > 0 &&
    row.parentOwnedTransportExecutionAttemptRef.length > 0 &&
    row.parentOwnedDeliveryResultReceiptRef.length > 0 &&
    row.childDeviceReceiptHandoffRef.length > 0 &&
    requiredRuntimeExecutionBlockers.every((blocker) =>
      row.requiredRuntimeExecutionBlockers.includes(blocker)
    ) &&
    row.externalWriterDispatchExecutionProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.executionWithheldReasonRefs.length === requiredRuntimeExecutionBlockers.length &&
    row.runtimeTransportDeliveryExecutionAuditEventRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function runtimeTransportDeliveryExecutionProofIsHonestGenerated(
  proof: {
    readonly sourceReceiptBoundaryProofVersion: string;
    readonly runtimeTransportDeliveryExecutionRows: readonly {
      readonly sourceDecisionAction: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceReceiptBoundaryProofVersion: string,
  requiredActions: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.runtimeTransportDeliveryExecutionRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceReceiptBoundaryProofVersion === sourceReceiptBoundaryProofVersion &&
    proof.runtimeTransportDeliveryExecutionRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}

export function summarizeAppInstallPurchaseExternalRuntimeWriterTransportExecutionProofGenerated(proof: {
  readonly externalRuntimeWriterTransportExecutionRows: readonly {
    readonly externalWriterTransportExecutionState: string;
    readonly externalWriterTransportPacketState: string;
    readonly externalWriterTransportAckState: string;
    readonly externalRuntimeWriterExecutionClaim: string;
  }[];
}) {
  return {
    externalRuntimeWriterTransportExecutionRows: proof.externalRuntimeWriterTransportExecutionRows.length,
    blockedTransportExecutionRows: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalWriterTransportExecutionState === 'transport-execution-blocked'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalWriterTransportExecutionState === 'manual-required'
    ).length,
    withheldTransportPackets: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalWriterTransportPacketState === 'packet-withheld'
    ).length,
    recordedTransportAcks: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) =>
        row.externalWriterTransportAckState !== 'ack-not-recorded' &&
        row.externalWriterTransportAckState !== 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
  } as const;
}

export function buildAppInstallPurchaseExternalRuntimeWriterTransportExecutionRowGenerated(
  row: {
    readonly runtimeTransportDeliveryExecutionRowId: string;
    readonly sourceDecisionAction: string;
    readonly runtimeTransportExecutionState: string;
    readonly runtimeTransportAttemptState: string;
    readonly runtimeDeliveryResultState: string;
    readonly parentOwnedTransportExecutionAttemptRef: string;
    readonly parentOwnedDeliveryResultReceiptRef: string;
    readonly childDeviceReceiptHandoffRef: string;
    readonly externalWriterDispatchExecutionProofRefs: readonly string[];
    readonly providerStoreExecutionReceiptProofRefs: readonly string[];
    readonly platformAdapterExecutionReceiptProofRefs: readonly string[];
    readonly childDeviceTransportReceiptProofRefs: readonly string[];
    readonly runtimeTransportDeliveryExecutionAuditEventRefs: readonly string[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
  },
  sourceRuntimeTransportDeliveryExecutionProofVersion: string,
  requiredExternalWriterTransportExecutionBlockers: readonly string[],
  claimBoundary: string,
  transportExecutionCheckedAt: string
) {
  const manual = row.runtimeTransportExecutionState === 'manual-required';
  return {
    schemaVersion: 'app-install-purchase-external-runtime-writer-transport-execution-proof',
    externalRuntimeWriterTransportExecutionRowId: `external-runtime-writer-transport-execution-${row.sourceDecisionAction}`,
    sourceRuntimeTransportDeliveryExecutionProofVersion,
    sourceRuntimeTransportDeliveryExecutionRowId: row.runtimeTransportDeliveryExecutionRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeTransportExecutionState: row.runtimeTransportExecutionState,
    sourceRuntimeTransportAttemptState: row.runtimeTransportAttemptState,
    sourceRuntimeDeliveryResultState: row.runtimeDeliveryResultState,
    sourceParentOwnedTransportExecutionAttemptRef: row.parentOwnedTransportExecutionAttemptRef,
    sourceParentOwnedDeliveryResultReceiptRef: row.parentOwnedDeliveryResultReceiptRef,
    sourceChildDeviceReceiptHandoffRef: row.childDeviceReceiptHandoffRef,
    parentOwnedExternalWriterTransportPacketRef: `parent-owned-external-writer-transport-packet-${row.sourceDecisionAction}`,
    parentOwnedExternalWriterTransportExecutionStatusRef: `parent-owned-external-writer-transport-execution-status-${row.sourceDecisionAction}`,
    parentOwnedExternalWriterTransportAckRef: `parent-owned-external-writer-transport-ack-${row.sourceDecisionAction}`,
    externalWriterTransportExecutionState: manual ? 'manual-required' : 'transport-execution-blocked',
    externalWriterTransportPacketState: manual ? 'manual-required' : 'packet-withheld',
    externalWriterTransportAckState: manual ? 'manual-required' : 'ack-not-recorded',
    requiredExternalWriterTransportExecutionBlockers,
    externalWriterDispatchExecutorProofRefs: row.externalWriterDispatchExecutionProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    transportExecutionBlockedReasonRefs: [
      `missing-external-writer-dispatch-executor-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    externalWriterTransportExecutionAuditEventRefs: [
      ...row.runtimeTransportDeliveryExecutionAuditEventRefs,
      `external-runtime-writer-transport-execution-audit-${row.sourceDecisionAction}`,
    ],
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary,
    transportExecutionCheckedAt,
  } as const;
}

export function externalRuntimeWriterTransportExecutionRowIsHonestGenerated(
  row: {
    readonly sourceRuntimeTransportDeliveryExecutionProofVersion: string;
    readonly sourceRuntimeTransportDeliveryExecutionRowId: string;
    readonly sourceParentOwnedTransportExecutionAttemptRef: string;
    readonly sourceParentOwnedDeliveryResultReceiptRef: string;
    readonly sourceChildDeviceReceiptHandoffRef: string;
    readonly parentOwnedExternalWriterTransportPacketRef: string;
    readonly parentOwnedExternalWriterTransportExecutionStatusRef: string;
    readonly parentOwnedExternalWriterTransportAckRef: string;
    readonly requiredExternalWriterTransportExecutionBlockers: readonly string[];
    readonly externalWriterDispatchExecutorProofRefs: readonly unknown[];
    readonly providerStoreExecutionReceiptProofRefs: readonly unknown[];
    readonly platformAdapterExecutionReceiptProofRefs: readonly unknown[];
    readonly childDeviceTransportReceiptProofRefs: readonly unknown[];
    readonly transportExecutionBlockedReasonRefs: readonly unknown[];
    readonly externalWriterTransportExecutionAuditEventRefs: readonly unknown[];
    readonly externalRuntimeWriterExecutionClaim: string;
    readonly externalRuntimeWriterDeliveryClaim: string;
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
    readonly sourceRuntimeTransportExecutionState: string;
    readonly sourceRuntimeTransportAttemptState: string;
    readonly sourceRuntimeDeliveryResultState: string;
    readonly externalWriterTransportExecutionState: string;
    readonly externalWriterTransportPacketState: string;
    readonly externalWriterTransportAckState: string;
    readonly claimBoundary: { readonly includes: (fragment: string) => boolean };
  },
  sourceRuntimeTransportDeliveryExecutionProofVersion: string,
  requiredExternalWriterTransportExecutionBlockers: readonly string[],
  boundaryFragments: readonly string[]
) {
  const stateMatches =
    row.sourceRuntimeTransportExecutionState === 'manual-required'
      ? row.sourceRuntimeTransportAttemptState === 'manual-required' &&
        row.sourceRuntimeDeliveryResultState === 'manual-required' &&
        row.externalWriterTransportExecutionState === 'manual-required' &&
        row.externalWriterTransportPacketState === 'manual-required' &&
        row.externalWriterTransportAckState === 'manual-required'
      : row.sourceRuntimeTransportExecutionState === 'execution-withheld-missing-artifacts' &&
        row.sourceRuntimeTransportAttemptState === 'not-started' &&
        row.sourceRuntimeDeliveryResultState === 'result-not-recorded' &&
        row.externalWriterTransportExecutionState === 'transport-execution-blocked' &&
        row.externalWriterTransportPacketState === 'packet-withheld' &&
        row.externalWriterTransportAckState === 'ack-not-recorded';
  return (
    stateMatches &&
    row.sourceRuntimeTransportDeliveryExecutionProofVersion ===
      sourceRuntimeTransportDeliveryExecutionProofVersion &&
    row.sourceRuntimeTransportDeliveryExecutionRowId.length > 0 &&
    row.sourceParentOwnedTransportExecutionAttemptRef.length > 0 &&
    row.sourceParentOwnedDeliveryResultReceiptRef.length > 0 &&
    row.sourceChildDeviceReceiptHandoffRef.length > 0 &&
    row.parentOwnedExternalWriterTransportPacketRef.length > 0 &&
    row.parentOwnedExternalWriterTransportExecutionStatusRef.length > 0 &&
    row.parentOwnedExternalWriterTransportAckRef.length > 0 &&
    requiredExternalWriterTransportExecutionBlockers.every((blocker) =>
      row.requiredExternalWriterTransportExecutionBlockers.includes(blocker)
    ) &&
    row.externalWriterDispatchExecutorProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.transportExecutionBlockedReasonRefs.length ===
      requiredExternalWriterTransportExecutionBlockers.length &&
    row.externalWriterTransportExecutionAuditEventRefs.length > 0 &&
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

export function externalRuntimeWriterTransportExecutionProofIsHonestGenerated(
  proof: {
    readonly sourceRuntimeTransportDeliveryExecutionProofVersion: string;
    readonly externalRuntimeWriterTransportExecutionRows: readonly {
      readonly sourceDecisionAction: string;
    }[];
    readonly nonClaims: readonly string[];
    readonly knownGaps: readonly unknown[];
  },
  sourceRuntimeTransportDeliveryExecutionProofVersion: string,
  requiredActions: readonly string[],
  requiredNonClaims: readonly string[]
) {
  const actions = new Set(proof.externalRuntimeWriterTransportExecutionRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeTransportDeliveryExecutionProofVersion ===
      sourceRuntimeTransportDeliveryExecutionProofVersion &&
    proof.externalRuntimeWriterTransportExecutionRows.length === requiredActions.length &&
    requiredActions.every((action) => actions.has(action)) &&
    requiredNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.knownGaps.length > 0
  );
}
