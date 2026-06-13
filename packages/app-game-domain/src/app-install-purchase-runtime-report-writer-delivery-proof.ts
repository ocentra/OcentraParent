import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel } from './app-install-purchase-runtime-writer-execution-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const RuntimeReportWriterDeliveryProofVersion = 'app-install-purchase-runtime-report-writer-delivery-proof';
const SourceRuntimeWriterExecutionDeliveryProofVersion = 'app-install-purchase-runtime-writer-execution-delivery-proof';
const SourceReportRuntimeProofVersion = 'app-install-purchase-report-runtime-proof';
const RuntimeReportWriterDeliveryTimestamp = '2026-06-05T20:40:00.000Z';
const RuntimeReportWriterDeliveryBoundary =
  'runtime report writer delivery proof only; parent-owned report delivery rows link runtime writer receipts to report runtime compiler output no portal report UI no external runtime report delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const RuntimeReportWriterDeliveryActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RuntimeReportWriterDeliveryStates = ['report-delivery-ready', 'manual-required'] as const;
const RuntimeReportWriterReceiptStates = ['parent-owned-report-receipt-recorded', 'manual-required'] as const;
const RuntimeReportWriterDeliveryNonClaims = [
  'no-portal-report-ui',
  'no-external-runtime-report-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-real-install-or-purchase-interception',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const RuntimeReportWriterDeliveryBoundaryFragments = [
  'parent-owned report delivery rows',
  'runtime writer receipts',
  'report runtime compiler output',
  'no portal report UI',
  'no external runtime report delivery',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseRuntimeReportWriterDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(RuntimeReportWriterDeliveryProofVersion)
);
const RuntimeReportWriterDeliveryActionSchema = withParser(Schema.Literal(...RuntimeReportWriterDeliveryActions));
const RuntimeReportWriterDeliveryStateSchema = withParser(Schema.Literal(...RuntimeReportWriterDeliveryStates));
const RuntimeReportWriterReceiptStateSchema = withParser(Schema.Literal(...RuntimeReportWriterReceiptStates));
const RuntimeReportWriterDeliveryProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const RuntimeReportWriterDeliveryIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const RuntimeReportWriterDeliveryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const RuntimeReportWriterDeliveryDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const RuntimeReportWriterDeliveryCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const RuntimeReportWriterDeliveryNonClaimSchema = withParser(Schema.Literal(...RuntimeReportWriterDeliveryNonClaims));

const RuntimeReportWriterDeliveryRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeReportWriterDeliveryRowId');
const RuntimeReportWriterDeliveryRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeReportWriterDeliveryRef');
const RuntimeReportWriterDeliveryAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeReportWriterDeliveryAuditRef');
const RuntimeReportWriterDeliveryBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeReportWriterDeliveryBoundary');

const RuntimeReportWriterDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeReportWriterDeliveryProofSchemaVersionSchema,
  runtimeReportWriterDeliveryRowId: RuntimeReportWriterDeliveryRowIdSchema,
  sourceRuntimeWriterExecutionDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterExecutionDeliveryProofVersion),
  sourceRuntimeWriterExecutionDeliveryRowId: RuntimeReportWriterDeliveryRefSchema,
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  sourceReportRuntimeRowIds: Schema.Array(RuntimeReportWriterDeliveryRefSchema),
  sourceDecisionAction: RuntimeReportWriterDeliveryActionSchema,
  runtimeReportWriterDeliveryState: RuntimeReportWriterDeliveryStateSchema,
  runtimeReportWriterReceiptState: RuntimeReportWriterReceiptStateSchema,
  runtimeReportWriterOutputRef: RuntimeReportWriterDeliveryRefSchema,
  runtimeReportWriterReceiptRef: RuntimeReportWriterDeliveryRefSchema,
  reportCompilerOutputRefs: Schema.Array(RuntimeReportWriterDeliveryRefSchema),
  runtimeWriterReceiptRef: RuntimeReportWriterDeliveryRefSchema,
  runtimeWriterAuditEventRefs: Schema.Array(RuntimeReportWriterDeliveryAuditRefSchema),
  parentActionAuditEventRefs: Schema.Array(RuntimeReportWriterDeliveryAuditRefSchema),
  reportAuditEventRefs: Schema.Array(RuntimeReportWriterDeliveryAuditRefSchema),
  providerApiExecutionClaim: RuntimeReportWriterDeliveryProviderClaimSchema,
  storeIntegrationClaim: RuntimeReportWriterDeliveryIntegrationClaimSchema,
  platformInterceptionClaim: RuntimeReportWriterDeliveryIntegrationClaimSchema,
  platformAdapterClaim: RuntimeReportWriterDeliveryAdapterClaimSchema,
  childDeviceDeliveryClaim: RuntimeReportWriterDeliveryDeliveryClaimSchema,
  runtimeReportDeliveryClaim: RuntimeReportWriterDeliveryDeliveryClaimSchema,
  portalReportUiClaim: RuntimeReportWriterDeliveryIntegrationClaimSchema,
  appBlockingClaim: RuntimeReportWriterDeliveryIntegrationClaimSchema,
  childDataCustody: RuntimeReportWriterDeliveryCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: RuntimeReportWriterDeliveryIntegrationClaimSchema,
  claimBoundary: RuntimeReportWriterDeliveryBoundarySchema,
  recordedAt: ParentTimestampSchema,
});

type RuntimeReportWriterDeliveryRowCandidate = Infer<typeof RuntimeReportWriterDeliveryRowBaseSchema>;

export const AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema = withParser(
  RuntimeReportWriterDeliveryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        runtimeReportWriterDeliveryRowIsHonest(row) ||
        'Expected runtime report writer delivery rows to link runtime writer receipts to report runtime output without provider, store, platform, child-device, portal, custody, or blocking claims'
    )
  )
);

const RuntimeReportWriterDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeReportWriterDeliveryProofSchemaVersionSchema,
  sourceRuntimeWriterExecutionDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterExecutionDeliveryProofVersion),
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  runtimeReportWriterDeliveryRows: Schema.Array(AppInstallPurchaseRuntimeReportWriterDeliveryRowSchema),
  nonClaims: Schema.Array(RuntimeReportWriterDeliveryNonClaimSchema),
  knownGaps: Schema.Array(RuntimeReportWriterDeliveryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeReportWriterDeliveryProof = Infer<
  typeof RuntimeReportWriterDeliveryProofBaseSchema
>;

export const AppInstallPurchaseRuntimeReportWriterDeliveryProofSchema = withParser(
  RuntimeReportWriterDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeReportWriterDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase runtime report writer delivery proof to cover parent actions and preserve report delivery non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeReportWriterDeliveryKnownGaps = [
  'Runtime report writer delivery rows record parent-owned report delivery readiness and receipts only.',
  'Portal report UI, external runtime report delivery, provider/store execution, platform adapters, child-device delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real parent approval action path exist.',
] as const;

export const AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel =
  AppInstallPurchaseRuntimeReportWriterDeliveryProofSchema.parse({
    schemaVersion: RuntimeReportWriterDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryProofVersion: SourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    runtimeReportWriterDeliveryRows:
      AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows.map(
        runtimeReportWriterDeliveryRow
      ),
    nonClaims: RuntimeReportWriterDeliveryNonClaims,
    knownGaps: AppInstallPurchaseRuntimeReportWriterDeliveryKnownGaps,
    updatedAt: RuntimeReportWriterDeliveryTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeReportWriterDeliveryProof(
  proof: AppInstallPurchaseRuntimeReportWriterDeliveryProof
) {
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

function runtimeReportWriterDeliveryRow(
  row: (typeof AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows)[number]
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  const reportRows = AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows;
  return {
    schemaVersion: RuntimeReportWriterDeliveryProofVersion,
    runtimeReportWriterDeliveryRowId: `runtime-report-writer-delivery-${row.sourceDecisionAction}`,
    sourceRuntimeWriterExecutionDeliveryProofVersion: SourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryRowId: row.runtimeWriterExecutionDeliveryRowId,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    sourceReportRuntimeRowIds: reportRows.map((reportRow) => reportRow.reportRuntimeRowId),
    sourceDecisionAction: row.sourceDecisionAction,
    runtimeReportWriterDeliveryState: manual ? 'manual-required' : 'report-delivery-ready',
    runtimeReportWriterReceiptState: manual ? 'manual-required' : 'parent-owned-report-receipt-recorded',
    runtimeReportWriterOutputRef: `parent-owned-runtime-report-output-${row.sourceDecisionAction}`,
    runtimeReportWriterReceiptRef: `parent-owned-runtime-report-receipt-${row.sourceDecisionAction}`,
    reportCompilerOutputRefs: uniqueRefs(reportRows.map((reportRow) => reportRow.outputReportRef)),
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
    claimBoundary: RuntimeReportWriterDeliveryBoundary,
    recordedAt: RuntimeReportWriterDeliveryTimestamp,
  } as const;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function runtimeReportWriterDeliveryRowIsHonest(row: RuntimeReportWriterDeliveryRowCandidate): boolean {
  return (
    runtimeReportWriterDeliveryMatchesAction(row) &&
    runtimeReportWriterDeliveryRefsAreComplete(row) &&
    runtimeReportWriterDeliveryClaimsStayBounded(row) &&
    runtimeReportWriterDeliveryBoundaryIsExplicit(row.claimBoundary)
  );
}

function runtimeReportWriterDeliveryMatchesAction(row: RuntimeReportWriterDeliveryRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.runtimeReportWriterDeliveryState === 'manual-required' &&
      row.runtimeReportWriterReceiptState === 'manual-required'
    );
  }
  return (
    row.runtimeReportWriterDeliveryState === 'report-delivery-ready' &&
    row.runtimeReportWriterReceiptState === 'parent-owned-report-receipt-recorded'
  );
}

function runtimeReportWriterDeliveryRefsAreComplete(row: RuntimeReportWriterDeliveryRowCandidate): boolean {
  return (
    row.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    row.sourceRuntimeWriterExecutionDeliveryRowId.length > 0 &&
    row.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    row.sourceReportRuntimeRowIds.length === AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length &&
    row.runtimeReportWriterOutputRef.length > 0 &&
    row.runtimeReportWriterReceiptRef.length > 0 &&
    row.reportCompilerOutputRefs.length > 0 &&
    row.runtimeWriterReceiptRef.length > 0 &&
    row.runtimeWriterAuditEventRefs.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportAuditEventRefs.length > 0
  );
}

function runtimeReportWriterDeliveryClaimsStayBounded(row: RuntimeReportWriterDeliveryRowCandidate): boolean {
  return (
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function runtimeReportWriterDeliveryProofIsHonest(proof: AppInstallPurchaseRuntimeReportWriterDeliveryProof): boolean {
  const actions = new Set(proof.runtimeReportWriterDeliveryRows.map((row) => row.sourceDecisionAction));
  const deliveryStates = new Set(
    proof.runtimeReportWriterDeliveryRows.map((row) => row.runtimeReportWriterDeliveryState)
  );
  const receiptStates = new Set(
    proof.runtimeReportWriterDeliveryRows.map((row) => row.runtimeReportWriterReceiptState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    proof.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    proof.runtimeReportWriterDeliveryRows.length === RuntimeReportWriterDeliveryActions.length &&
    RuntimeReportWriterDeliveryActions.every((action) => actions.has(action)) &&
    RuntimeReportWriterDeliveryStates.every((state) => deliveryStates.has(state)) &&
    RuntimeReportWriterReceiptStates.every((state) => receiptStates.has(state)) &&
    RuntimeReportWriterDeliveryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.runtimeReportWriterDeliveryRows.every(runtimeReportWriterDeliveryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function runtimeReportWriterDeliveryBoundaryIsExplicit(
  boundary: typeof RuntimeReportWriterDeliveryBoundarySchema.Type
): boolean {
  return RuntimeReportWriterDeliveryBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

