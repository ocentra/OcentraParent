import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel } from './app-install-purchase-provider-store-execution-readiness-proof';
import { AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel } from './app-install-purchase-runtime-report-writer-delivery-proof';
import { ParentPlatformSchema, ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ProviderStoreReportStatusRuntimeProofVersion = 'app-install-purchase-provider-store-report-status-runtime-proof';
const SourceProviderStoreExecutionReadinessProofVersion =
  'app-install-purchase-provider-store-execution-readiness-proof';
const SourceRuntimeReportWriterDeliveryProofVersion = 'app-install-purchase-runtime-report-writer-delivery-proof';
const ProviderStoreReportStatusRuntimeTimestamp = '2026-06-06T02:58:00.000Z';
const ProviderStoreReportStatusRuntimeBoundary =
  'provider store report status runtime proof only; links provider store readiness rows to parent-owned runtime report writer receipts no provider API execution no store integration no portal report UI no external runtime report delivery no platform adapter implementation no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const ProviderStoreReportStatusRuntimeStates = [
  'provider-store-report-status-ready',
  'manual-required',
  'unavailable',
] as const;
const ProviderStoreReportStatusRuntimeNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-portal-report-ui',
  'no-external-runtime-report-delivery',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ProviderStoreReportStatusRuntimeBoundaryFragments = [
  'provider store readiness rows',
  'runtime report writer receipts',
  'no provider API execution',
  'no store integration',
  'no portal report UI',
  'no external runtime report delivery',
  'no platform adapter implementation',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchemaVersionSchema = withParser(
  Schema.Literal(ProviderStoreReportStatusRuntimeProofVersion)
);
const ProviderStoreReportStatusRuntimeStoreSurfaceSchema = withParser(
  Schema.Literal('microsoft-store', 'mac-app-store', 'linux-package-manager', 'google-play', 'apple-app-store')
);
const ProviderStoreReportStatusRuntimeReadinessStateSchema = withParser(
  Schema.Literal('provider-store-execution-ready', 'manual-required', 'unavailable')
);
const ProviderStoreReportStatusRuntimeStateSchema = withParser(
  Schema.Literal(...ProviderStoreReportStatusRuntimeStates)
);
const ProviderStoreReportStatusRuntimeClaimSchema = withParser(Schema.Literal('not-claimed'));
const ProviderStoreReportStatusRuntimeExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ProviderStoreReportStatusRuntimeAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ProviderStoreReportStatusRuntimeDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ProviderStoreReportStatusRuntimeCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ProviderStoreReportStatusRuntimeNonClaimSchema = withParser(
  Schema.Literal(...ProviderStoreReportStatusRuntimeNonClaims)
);

const ProviderStoreReportStatusRuntimeRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreReportStatusRuntimeRowId'
);
const ProviderStoreReportStatusRuntimeRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreReportStatusRuntimeRef'
);
const ProviderStoreReportStatusRuntimeBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseProviderStoreReportStatusRuntimeBoundary'
);

const ProviderStoreReportStatusRuntimeRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchemaVersionSchema,
  providerStoreReportStatusRuntimeRowId: ProviderStoreReportStatusRuntimeRowIdSchema,
  sourceProviderStoreExecutionReadinessProofVersion: Schema.Literal(SourceProviderStoreExecutionReadinessProofVersion),
  sourceProviderStoreExecutionReadinessRowId: ProviderStoreReportStatusRuntimeRefSchema,
  sourceRuntimeReportWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeReportWriterDeliveryProofVersion),
  sourceRuntimeReportWriterDeliveryRowIds: Schema.Array(ProviderStoreReportStatusRuntimeRefSchema),
  sourceRuntimeReportWriterReceiptRefs: Schema.Array(ProviderStoreReportStatusRuntimeRefSchema),
  platform: ParentPlatformSchema,
  storeSurface: ProviderStoreReportStatusRuntimeStoreSurfaceSchema,
  sourceProviderStoreExecutionReadinessState: ProviderStoreReportStatusRuntimeReadinessStateSchema,
  providerStoreReportStatusRuntimeState: ProviderStoreReportStatusRuntimeStateSchema,
  reportCompilerOutputRefs: Schema.Array(ProviderStoreReportStatusRuntimeRefSchema),
  reportReceiptRefs: Schema.Array(ProviderStoreReportStatusRuntimeRefSchema),
  requiredProofRefs: Schema.Array(ProviderStoreReportStatusRuntimeRefSchema),
  providerApiExecutionClaim: ProviderStoreReportStatusRuntimeExecutionClaimSchema,
  storeIntegrationClaim: ProviderStoreReportStatusRuntimeClaimSchema,
  portalReportUiClaim: ProviderStoreReportStatusRuntimeClaimSchema,
  runtimeReportDeliveryClaim: ProviderStoreReportStatusRuntimeDeliveryClaimSchema,
  platformAdapterClaim: ProviderStoreReportStatusRuntimeAdapterClaimSchema,
  childDeviceDeliveryClaim: ProviderStoreReportStatusRuntimeDeliveryClaimSchema,
  appBlockingClaim: ProviderStoreReportStatusRuntimeClaimSchema,
  childDataCustody: ProviderStoreReportStatusRuntimeCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ProviderStoreReportStatusRuntimeClaimSchema,
  claimBoundary: ProviderStoreReportStatusRuntimeBoundarySchema,
  evaluatedAt: ParentTimestampSchema,
});

type ProviderStoreReportStatusRuntimeRowCandidate = Infer<typeof ProviderStoreReportStatusRuntimeRowBaseSchema>;

export const AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema = withParser(
  ProviderStoreReportStatusRuntimeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        providerStoreReportStatusRuntimeRowIsHonest(row) ||
        'Expected provider/store report status runtime rows to link provider/store readiness to runtime report writer receipts without execution, delivery, adapter, portal, custody, or blocking claims'
    )
  )
);

const ProviderStoreReportStatusRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchemaVersionSchema,
  sourceProviderStoreExecutionReadinessProofVersion: Schema.Literal(SourceProviderStoreExecutionReadinessProofVersion),
  sourceRuntimeReportWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeReportWriterDeliveryProofVersion),
  providerStoreReportStatusRuntimeRows: Schema.Array(AppInstallPurchaseProviderStoreReportStatusRuntimeRowSchema),
  nonClaims: Schema.Array(ProviderStoreReportStatusRuntimeNonClaimSchema),
  knownGaps: Schema.Array(ProviderStoreReportStatusRuntimeRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseProviderStoreReportStatusRuntimeProof = Infer<
  typeof ProviderStoreReportStatusRuntimeProofBaseSchema
>;

export const AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema = withParser(
  ProviderStoreReportStatusRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        providerStoreReportStatusRuntimeProofIsHonest(proof) ||
        'Expected app install/purchase provider/store report status runtime proof to cover store surfaces and preserve runtime report non-claims'
    )
  )
);

export const AppInstallPurchaseProviderStoreReportStatusRuntimeKnownGaps = [
  'Provider/store report status runtime rows are parent-domain proof rows only; no provider/store execution or portal report UI is implemented.',
  'External runtime report delivery, platform adapters, child-device delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
] as const;

export const AppInstallPurchaseProviderStoreReportStatusRuntimeProofReadModel =
  AppInstallPurchaseProviderStoreReportStatusRuntimeProofSchema.parse({
    schemaVersion: ProviderStoreReportStatusRuntimeProofVersion,
    sourceProviderStoreExecutionReadinessProofVersion: SourceProviderStoreExecutionReadinessProofVersion,
    sourceRuntimeReportWriterDeliveryProofVersion: SourceRuntimeReportWriterDeliveryProofVersion,
    providerStoreReportStatusRuntimeRows:
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows.map(
        providerStoreReportStatusRuntimeRow
      ),
    nonClaims: ProviderStoreReportStatusRuntimeNonClaims,
    knownGaps: AppInstallPurchaseProviderStoreReportStatusRuntimeKnownGaps,
    updatedAt: ProviderStoreReportStatusRuntimeTimestamp,
  });

export function summarizeAppInstallPurchaseProviderStoreReportStatusRuntimeProof(
  proof: AppInstallPurchaseProviderStoreReportStatusRuntimeProof
) {
  return {
    providerStoreReportStatusRuntimeRows: proof.providerStoreReportStatusRuntimeRows.length,
    readyRows: proof.providerStoreReportStatusRuntimeRows.filter(
      (row) => row.providerStoreReportStatusRuntimeState === 'provider-store-report-status-ready'
    ).length,
    manualRequiredRows: proof.providerStoreReportStatusRuntimeRows.filter(
      (row) => row.providerStoreReportStatusRuntimeState === 'manual-required'
    ).length,
    unavailableRows: proof.providerStoreReportStatusRuntimeRows.filter(
      (row) => row.providerStoreReportStatusRuntimeState === 'unavailable'
    ).length,
    runtimeReportWriterLinkedRows: proof.providerStoreReportStatusRuntimeRows.filter(
      runtimeReportWriterCoverageIsComplete
    ).length,
    providerExecutedRows: proof.providerStoreReportStatusRuntimeRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    externallyDeliveredRows: proof.providerStoreReportStatusRuntimeRows.filter(
      (row) => row.runtimeReportDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function providerStoreReportStatusRuntimeRow(
  row: (typeof AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows)[number]
) {
  const runtimeReportRows = AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.runtimeReportWriterDeliveryRows;
  return {
    schemaVersion: ProviderStoreReportStatusRuntimeProofVersion,
    providerStoreReportStatusRuntimeRowId: `provider-store-report-status-runtime-${row.platform}-${row.storeSurface}`,
    sourceProviderStoreExecutionReadinessProofVersion: SourceProviderStoreExecutionReadinessProofVersion,
    sourceProviderStoreExecutionReadinessRowId: row.providerStoreExecutionReadinessRowId,
    sourceRuntimeReportWriterDeliveryProofVersion: SourceRuntimeReportWriterDeliveryProofVersion,
    sourceRuntimeReportWriterDeliveryRowIds: runtimeReportRows.map(
      (reportRow) => reportRow.runtimeReportWriterDeliveryRowId
    ),
    sourceRuntimeReportWriterReceiptRefs: runtimeReportRows.map((reportRow) => reportRow.runtimeReportWriterReceiptRef),
    platform: row.platform,
    storeSurface: row.storeSurface,
    sourceProviderStoreExecutionReadinessState: row.providerStoreExecutionReadinessState,
    providerStoreReportStatusRuntimeState: providerStoreReportStatusRuntimeState(
      row.providerStoreExecutionReadinessState
    ),
    reportCompilerOutputRefs: uniqueRefs(runtimeReportRows.flatMap((reportRow) => reportRow.reportCompilerOutputRefs)),
    reportReceiptRefs: runtimeReportRows.map((reportRow) => reportRow.runtimeReportWriterReceiptRef),
    requiredProofRefs: row.requiredProofRefs,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    portalReportUiClaim: 'not-claimed',
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: ProviderStoreReportStatusRuntimeBoundary,
    evaluatedAt: ProviderStoreReportStatusRuntimeTimestamp,
  } as const;
}

function providerStoreReportStatusRuntimeState(readinessState: string) {
  if (readinessState === 'provider-store-execution-ready') {
    return 'provider-store-report-status-ready';
  }
  return readinessState;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function providerStoreReportStatusRuntimeRowIsHonest(row: ProviderStoreReportStatusRuntimeRowCandidate): boolean {
  return (
    providerStoreReportStatusRuntimeState(row.sourceProviderStoreExecutionReadinessState) ===
      row.providerStoreReportStatusRuntimeState &&
    runtimeReportWriterCoverageIsComplete(row) &&
    row.requiredProofRefs.length > 0 &&
    providerStoreReportStatusRuntimeClaimsStayBounded(row) &&
    ProviderStoreReportStatusRuntimeBoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function runtimeReportWriterCoverageIsComplete(row: ProviderStoreReportStatusRuntimeRowCandidate): boolean {
  return (
    row.sourceRuntimeReportWriterDeliveryProofVersion === SourceRuntimeReportWriterDeliveryProofVersion &&
    row.sourceRuntimeReportWriterDeliveryRowIds.length ===
      AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.runtimeReportWriterDeliveryRows.length &&
    row.sourceRuntimeReportWriterReceiptRefs.length > 0 &&
    row.reportCompilerOutputRefs.length > 0 &&
    row.reportReceiptRefs.length > 0
  );
}

function providerStoreReportStatusRuntimeClaimsStayBounded(row: ProviderStoreReportStatusRuntimeRowCandidate): boolean {
  return (
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function providerStoreReportStatusRuntimeProofIsHonest(
  proof: AppInstallPurchaseProviderStoreReportStatusRuntimeProof
): boolean {
  const states = new Set(
    proof.providerStoreReportStatusRuntimeRows.map((row) => row.providerStoreReportStatusRuntimeState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceProviderStoreExecutionReadinessProofVersion === SourceProviderStoreExecutionReadinessProofVersion &&
    proof.sourceRuntimeReportWriterDeliveryProofVersion === SourceRuntimeReportWriterDeliveryProofVersion &&
    proof.providerStoreReportStatusRuntimeRows.length ===
      AppInstallPurchaseProviderStoreExecutionReadinessProofReadModel.providerStoreExecutionReadinessRows.length &&
    ProviderStoreReportStatusRuntimeStates.every((state) => states.has(state)) &&
    ProviderStoreReportStatusRuntimeNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.providerStoreReportStatusRuntimeRows.every(providerStoreReportStatusRuntimeRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
