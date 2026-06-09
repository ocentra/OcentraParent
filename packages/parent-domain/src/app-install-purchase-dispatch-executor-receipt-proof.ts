import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExecutionReceiptGateProofReadModel } from './app-install-purchase-execution-receipt-gate-proof';
import { ParentTimestampSchema } from './reference-primitives';

const DispatchExecutorReceiptProofVersion = 'app-install-purchase-dispatch-executor-receipt-proof';
const SourceExecutionReceiptGateProofVersion = 'app-install-purchase-execution-receipt-gate-proof';
const DispatchExecutorReceiptTimestamp = '2026-06-08T22:04:00.000Z';
const DispatchExecutorReceiptText = Schema.String.pipe(Schema.minLength(1));
const DispatchExecutorReceiptBoundary =
  'dispatch executor receipt proof only; rows consume execution receipt gate rows and create parent-owned external writer dispatch executor receipt artifact requirements that remain blocked until a real external writer dispatch executor receipt is attached no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const DispatchExecutorReceiptActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceExecutionReceiptGateStates = ['blocked-missing-execution-receipts', 'manual-required'] as const;
const SourceExecutionReceiptFamilyStates = ['receipt-missing', 'manual-required'] as const;
const DispatchExecutorReceiptStates = ['dispatch-executor-receipt-blocked', 'manual-required'] as const;
const DispatchExecutorArtifactStates = ['artifact-missing', 'manual-required'] as const;
const DispatchExecutorRequiredArtifacts = [
  'external-writer-dispatch-executor-handler-proof',
  'external-writer-dispatch-executor-receipt-artifact',
  'external-writer-dispatch-executor-audit-artifact',
] as const;
const DispatchExecutorReceiptNonClaims = [
  'no-external-runtime-writer-execution',
  'no-external-runtime-writer-delivery',
  'no-parent-action-runtime-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const DispatchExecutorReceiptBoundaryFragments = [
  'rows consume execution receipt gate rows',
  'parent-owned external writer dispatch executor receipt artifact requirements',
  'remain blocked',
  'real external writer dispatch executor receipt',
  'no external runtime writer execution',
  'no external runtime writer delivery',
  'no parent action runtime delivery',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseDispatchExecutorReceiptProofSchemaVersionSchema = withParser(
  Schema.Literal(DispatchExecutorReceiptProofVersion)
);
const DispatchExecutorReceiptActionSchema = withParser(Schema.Literal(...DispatchExecutorReceiptActions));
const SourceExecutionReceiptGateStateSchema = withParser(Schema.Literal(...SourceExecutionReceiptGateStates));
const SourceExecutionReceiptFamilyStateSchema = withParser(Schema.Literal(...SourceExecutionReceiptFamilyStates));
const DispatchExecutorReceiptStateSchema = withParser(Schema.Literal(...DispatchExecutorReceiptStates));
const DispatchExecutorArtifactStateSchema = withParser(Schema.Literal(...DispatchExecutorArtifactStates));
const DispatchExecutorRequiredArtifactSchema = withParser(Schema.Literal(...DispatchExecutorRequiredArtifacts));
const DispatchExecutorReceiptNonClaimSchema = withParser(Schema.Literal(...DispatchExecutorReceiptNonClaims));
const DispatchExecutorReceiptExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const DispatchExecutorReceiptDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const DispatchExecutorReceiptIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const DispatchExecutorReceiptAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const DispatchExecutorReceiptCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));

const DispatchExecutorReceiptRowIdSchema = DispatchExecutorReceiptText.pipe(
  Schema.brand('AppInstallPurchaseDispatchExecutorReceiptRowId')
);
const DispatchExecutorReceiptRefSchema = DispatchExecutorReceiptText.pipe(
  Schema.brand('AppInstallPurchaseDispatchExecutorReceiptRef')
);
const DispatchExecutorReceiptAuditRefSchema = DispatchExecutorReceiptText.pipe(
  Schema.brand('AppInstallPurchaseDispatchExecutorReceiptAuditRef')
);
const DispatchExecutorReceiptBoundarySchema = DispatchExecutorReceiptText.pipe(
  Schema.brand('AppInstallPurchaseDispatchExecutorReceiptBoundary')
);

const DispatchExecutorReceiptRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseDispatchExecutorReceiptProofSchemaVersionSchema,
  dispatchExecutorReceiptRowId: DispatchExecutorReceiptRowIdSchema,
  sourceExecutionReceiptGateProofVersion: Schema.Literal(SourceExecutionReceiptGateProofVersion),
  sourceExecutionReceiptGateRowId: DispatchExecutorReceiptRefSchema,
  sourceDecisionAction: DispatchExecutorReceiptActionSchema,
  sourceExecutionReceiptGateState: SourceExecutionReceiptGateStateSchema,
  sourceExternalWriterDispatchExecutorReceiptState: SourceExecutionReceiptFamilyStateSchema,
  sourceExternalWriterDispatchExecutorReceiptProofRefs: Schema.Array(DispatchExecutorReceiptRefSchema),
  sourceExecutionReceiptGateBlockedReasonRefs: Schema.Array(DispatchExecutorReceiptRefSchema),
  dispatchExecutorReceiptState: DispatchExecutorReceiptStateSchema,
  dispatchExecutorReceiptArtifactState: DispatchExecutorArtifactStateSchema,
  requiredDispatchExecutorArtifacts: Schema.Array(DispatchExecutorRequiredArtifactSchema),
  dispatchExecutorHandlerProofRefs: Schema.Array(DispatchExecutorReceiptRefSchema),
  dispatchExecutorReceiptArtifactRefs: Schema.Array(DispatchExecutorReceiptRefSchema),
  dispatchExecutorAuditArtifactRefs: Schema.Array(DispatchExecutorReceiptAuditRefSchema),
  dispatchExecutorBlockedReasonRefs: Schema.Array(DispatchExecutorReceiptRefSchema),
  dispatchExecutorAuditEventRefs: Schema.Array(DispatchExecutorReceiptAuditRefSchema),
  externalRuntimeWriterExecutionClaim: DispatchExecutorReceiptExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: DispatchExecutorReceiptDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: DispatchExecutorReceiptDeliveryClaimSchema,
  providerApiExecutionClaim: DispatchExecutorReceiptExecutionClaimSchema,
  storeIntegrationClaim: DispatchExecutorReceiptIntegrationClaimSchema,
  platformInterceptionClaim: DispatchExecutorReceiptIntegrationClaimSchema,
  platformAdapterClaim: DispatchExecutorReceiptAdapterClaimSchema,
  childDeviceDeliveryClaim: DispatchExecutorReceiptDeliveryClaimSchema,
  runtimeReportDeliveryClaim: DispatchExecutorReceiptDeliveryClaimSchema,
  appBlockingClaim: DispatchExecutorReceiptIntegrationClaimSchema,
  childDataCustody: DispatchExecutorReceiptCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: DispatchExecutorReceiptIntegrationClaimSchema,
  claimBoundary: DispatchExecutorReceiptBoundarySchema,
  dispatchExecutorReceiptCheckedAt: ParentTimestampSchema,
});

type DispatchExecutorReceiptRowCandidate = Infer<typeof DispatchExecutorReceiptRowBaseSchema>;

export const AppInstallPurchaseDispatchExecutorReceiptRowSchema = withParser(
  DispatchExecutorReceiptRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        dispatchExecutorReceiptRowIsHonest(row) ||
        'Expected app install/purchase dispatch executor receipt rows to stay blocked until real executor artifacts exist'
    )
  )
);

const DispatchExecutorReceiptProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseDispatchExecutorReceiptProofSchemaVersionSchema,
  sourceExecutionReceiptGateProofVersion: Schema.Literal(SourceExecutionReceiptGateProofVersion),
  dispatchExecutorReceiptRows: Schema.Array(AppInstallPurchaseDispatchExecutorReceiptRowSchema),
  nonClaims: Schema.Array(DispatchExecutorReceiptNonClaimSchema),
  knownGaps: Schema.Array(DispatchExecutorReceiptRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseDispatchExecutorReceiptProof = Infer<typeof DispatchExecutorReceiptProofBaseSchema>;

export const AppInstallPurchaseDispatchExecutorReceiptProofSchema = withParser(
  DispatchExecutorReceiptProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        dispatchExecutorReceiptProofIsHonest(proof) ||
        'Expected app install/purchase dispatch executor receipt proof to preserve executor artifact blockers and non-claims'
    )
  )
);

export const AppInstallPurchaseDispatchExecutorReceiptKnownGaps = [
  'Dispatch executor receipt rows are parent-owned artifact requirement rows only; no external writer dispatch executor has executed.',
  'Rows stay blocked or manual-required until handler proof, receipt artifact proof, and audit artifact proof exist for the external writer dispatch executor.',
] as const;

export const AppInstallPurchaseDispatchExecutorReceiptProofReadModel =
  AppInstallPurchaseDispatchExecutorReceiptProofSchema.parse({
    schemaVersion: DispatchExecutorReceiptProofVersion,
    sourceExecutionReceiptGateProofVersion: SourceExecutionReceiptGateProofVersion,
    dispatchExecutorReceiptRows:
      AppInstallPurchaseExecutionReceiptGateProofReadModel.executionReceiptGateRows.map(dispatchExecutorReceiptRow),
    nonClaims: DispatchExecutorReceiptNonClaims,
    knownGaps: AppInstallPurchaseDispatchExecutorReceiptKnownGaps,
    updatedAt: DispatchExecutorReceiptTimestamp,
  });

export function summarizeAppInstallPurchaseDispatchExecutorReceiptProof(
  proof: AppInstallPurchaseDispatchExecutorReceiptProof
) {
  return {
    dispatchExecutorReceiptRows: proof.dispatchExecutorReceiptRows.length,
    blockedDispatchExecutorRows: proof.dispatchExecutorReceiptRows.filter(
      (row) => row.dispatchExecutorReceiptState === 'dispatch-executor-receipt-blocked'
    ).length,
    manualRequiredRows: proof.dispatchExecutorReceiptRows.filter(
      (row) => row.dispatchExecutorReceiptState === 'manual-required'
    ).length,
    acceptedDispatchExecutorArtifacts: proof.dispatchExecutorReceiptRows.filter(
      (row) =>
        row.dispatchExecutorReceiptArtifactState !== 'artifact-missing' &&
        row.dispatchExecutorReceiptArtifactState !== 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.dispatchExecutorReceiptRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
    childDeviceDeliveredRows: proof.dispatchExecutorReceiptRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function dispatchExecutorReceiptRow(
  row: (typeof AppInstallPurchaseExecutionReceiptGateProofReadModel.executionReceiptGateRows)[number]
) {
  const manual = row.executionReceiptGateState === 'manual-required';
  return {
    schemaVersion: DispatchExecutorReceiptProofVersion,
    dispatchExecutorReceiptRowId: `dispatch-executor-receipt-${row.sourceDecisionAction}`,
    sourceExecutionReceiptGateProofVersion: SourceExecutionReceiptGateProofVersion,
    sourceExecutionReceiptGateRowId: row.executionReceiptGateRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExecutionReceiptGateState: row.executionReceiptGateState,
    sourceExternalWriterDispatchExecutorReceiptState: row.externalWriterDispatchExecutorReceiptState,
    sourceExternalWriterDispatchExecutorReceiptProofRefs: row.externalWriterDispatchExecutorReceiptProofRefs,
    sourceExecutionReceiptGateBlockedReasonRefs: row.executionReceiptGateBlockedReasonRefs,
    dispatchExecutorReceiptState: manual ? 'manual-required' : 'dispatch-executor-receipt-blocked',
    dispatchExecutorReceiptArtifactState: manual ? 'manual-required' : 'artifact-missing',
    requiredDispatchExecutorArtifacts: DispatchExecutorRequiredArtifacts,
    dispatchExecutorHandlerProofRefs: row.externalWriterDispatchExecutorReceiptProofRefs,
    dispatchExecutorReceiptArtifactRefs: [`dispatch-executor-receipt-artifact-required-${row.sourceDecisionAction}`],
    dispatchExecutorAuditArtifactRefs: [`dispatch-executor-audit-artifact-required-${row.sourceDecisionAction}`],
    dispatchExecutorBlockedReasonRefs: [
      `missing-dispatch-executor-handler-proof-${row.sourceDecisionAction}`,
      `missing-dispatch-executor-receipt-artifact-${row.sourceDecisionAction}`,
      `missing-dispatch-executor-audit-artifact-${row.sourceDecisionAction}`,
    ],
    dispatchExecutorAuditEventRefs: [
      ...row.executionReceiptGateAuditEventRefs,
      `dispatch-executor-receipt-audit-${row.sourceDecisionAction}`,
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
    claimBoundary: DispatchExecutorReceiptBoundary,
    dispatchExecutorReceiptCheckedAt: DispatchExecutorReceiptTimestamp,
  } as const;
}

function dispatchExecutorReceiptRowIsHonest(row: DispatchExecutorReceiptRowCandidate): boolean {
  return (
    dispatchExecutorReceiptStatesMatchSource(row) &&
    dispatchExecutorReceiptRefsAreComplete(row) &&
    dispatchExecutorReceiptClaimsStayUnimplemented(row) &&
    dispatchExecutorReceiptBoundaryIsExplicit(row.claimBoundary)
  );
}

function dispatchExecutorReceiptStatesMatchSource(row: DispatchExecutorReceiptRowCandidate): boolean {
  if (row.sourceExecutionReceiptGateState === 'manual-required') {
    return (
      row.sourceExternalWriterDispatchExecutorReceiptState === 'manual-required' &&
      row.dispatchExecutorReceiptState === 'manual-required' &&
      row.dispatchExecutorReceiptArtifactState === 'manual-required'
    );
  }
  return (
    row.sourceExecutionReceiptGateState === 'blocked-missing-execution-receipts' &&
    row.sourceExternalWriterDispatchExecutorReceiptState === 'receipt-missing' &&
    row.dispatchExecutorReceiptState === 'dispatch-executor-receipt-blocked' &&
    row.dispatchExecutorReceiptArtifactState === 'artifact-missing'
  );
}

function dispatchExecutorReceiptRefsAreComplete(row: DispatchExecutorReceiptRowCandidate): boolean {
  return (
    row.sourceExecutionReceiptGateProofVersion === SourceExecutionReceiptGateProofVersion &&
    row.sourceExecutionReceiptGateRowId.length > 0 &&
    row.sourceExternalWriterDispatchExecutorReceiptProofRefs.length > 0 &&
    row.sourceExecutionReceiptGateBlockedReasonRefs.length > 0 &&
    DispatchExecutorRequiredArtifacts.every((artifact) => row.requiredDispatchExecutorArtifacts.includes(artifact)) &&
    row.dispatchExecutorHandlerProofRefs.length > 0 &&
    row.dispatchExecutorReceiptArtifactRefs.length > 0 &&
    row.dispatchExecutorAuditArtifactRefs.length > 0 &&
    row.dispatchExecutorBlockedReasonRefs.length === DispatchExecutorRequiredArtifacts.length &&
    row.dispatchExecutorAuditEventRefs.length > 0
  );
}

function dispatchExecutorReceiptClaimsStayUnimplemented(row: DispatchExecutorReceiptRowCandidate): boolean {
  return (
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
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function dispatchExecutorReceiptProofIsHonest(proof: AppInstallPurchaseDispatchExecutorReceiptProof): boolean {
  const actions = new Set(proof.dispatchExecutorReceiptRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExecutionReceiptGateProofVersion === SourceExecutionReceiptGateProofVersion &&
    proof.dispatchExecutorReceiptRows.length === DispatchExecutorReceiptActions.length &&
    DispatchExecutorReceiptActions.every((action) => actions.has(action)) &&
    DispatchExecutorReceiptNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.dispatchExecutorReceiptRows.every(dispatchExecutorReceiptRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function dispatchExecutorReceiptBoundaryIsExplicit(
  boundary: typeof DispatchExecutorReceiptBoundarySchema.Type
): boolean {
  return DispatchExecutorReceiptBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
