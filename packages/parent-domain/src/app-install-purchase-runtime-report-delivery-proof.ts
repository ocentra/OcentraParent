import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { ParentTimestampSchema } from './reference-primitives';

const RuntimeReportDeliveryText = Schema.String.pipe(Schema.minLength(1));
const RuntimeReportDeliveryProofVersion = 'app-install-purchase-runtime-report-delivery-proof';
const SourceReportRuntimeProofVersion = 'app-install-purchase-report-runtime-proof';
const RuntimeReportDeliveryTimestamp = '2026-06-05T22:59:00.000Z';
const RuntimeReportDeliveryBoundary =
  'runtime report delivery proof only; no portal report UI no provider API execution no store integration no platform adapter no child-device delivery no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredRuntimeReportDeliveryStates = ['runtime-report-delivered'] as const;
const RequiredRuntimeReportDeliveryNonClaims = [
  'no-portal-report-ui',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;
const RuntimeReportDeliveryBoundaryFragments = [
  'no portal report UI',
  'no provider API execution',
  'no store integration',
  'no platform adapter',
  'no child-device delivery',
  'no child activity data',
  'no app blocking',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseRuntimeReportDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(RuntimeReportDeliveryProofVersion)
);
const AppInstallPurchaseRuntimeReportDeliveryStateSchema = withParser(
  Schema.Literal(...RequiredRuntimeReportDeliveryStates)
);
const AppInstallPurchaseRuntimeReportDeliveryNonClaimSchema = withParser(
  Schema.Literal(...RequiredRuntimeReportDeliveryNonClaims)
);
const AppInstallPurchaseRuntimeReportDeliveryClaimSchema = withParser(Schema.Literal('parent-runtime-delivered'));
const AppInstallPurchaseRuntimeReportDeliveryPortalClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeReportDeliveryNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeReportDeliveryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseRuntimeReportDeliveryChildDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseRuntimeReportDeliveryCustodySchema = withParser(Schema.Literal('no-child-activity-data'));

const RuntimeReportDeliveryRowIdSchema = RuntimeReportDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeReportDeliveryRowId')
);
const RuntimeReportDeliveryRefSchema = RuntimeReportDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeReportDeliveryRef')
);
const RuntimeReportDeliveryBoundarySchema = RuntimeReportDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeReportDeliveryBoundary')
);

const RuntimeReportDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeReportDeliveryProofSchemaVersionSchema,
  runtimeReportDeliveryRowId: RuntimeReportDeliveryRowIdSchema,
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  sourceReportRuntimeRowId: RuntimeReportDeliveryRefSchema,
  reportSurface: RuntimeReportDeliveryRefSchema,
  compilerRequestId: RuntimeReportDeliveryRefSchema,
  compilerOutputReportRef: RuntimeReportDeliveryRefSchema,
  runtimeReportReceiptRef: RuntimeReportDeliveryRefSchema,
  sourceChildArtifactRefs: Schema.Array(RuntimeReportDeliveryRefSchema),
  deliveryState: AppInstallPurchaseRuntimeReportDeliveryStateSchema,
  parentAuthorized: Schema.Boolean,
  rawEvidenceExcludedFromOutput: Schema.Boolean,
  childDetailMinimized: Schema.Boolean,
  tempDeletionConfirmed: Schema.Boolean,
  localEvidenceMutated: Schema.Boolean,
  ocentraHostedReportRetained: Schema.Boolean,
  runtimeReportDeliveryClaim: AppInstallPurchaseRuntimeReportDeliveryClaimSchema,
  portalReportUiClaim: AppInstallPurchaseRuntimeReportDeliveryPortalClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseRuntimeReportDeliveryNotClaimedSchema,
  storeIntegrationClaim: AppInstallPurchaseRuntimeReportDeliveryNotClaimedSchema,
  platformAdapterClaim: AppInstallPurchaseRuntimeReportDeliveryAdapterClaimSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseRuntimeReportDeliveryChildDeliveryClaimSchema,
  childDataCustody: AppInstallPurchaseRuntimeReportDeliveryCustodySchema,
  appBlockingClaim: AppInstallPurchaseRuntimeReportDeliveryNotClaimedSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseRuntimeReportDeliveryNotClaimedSchema,
  claimBoundary: RuntimeReportDeliveryBoundarySchema,
  deliveredAt: ParentTimestampSchema,
});

type RuntimeReportDeliveryRowCandidate = Infer<typeof RuntimeReportDeliveryRowBaseSchema>;

export const AppInstallPurchaseRuntimeReportDeliveryRowSchema = withParser(
  RuntimeReportDeliveryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        runtimeReportDeliveryRowIsHonest(row) ||
        'Expected app install/purchase runtime report delivery rows to deliver parent-owned report refs without portal, provider, store, adapter, child-device, custody, or blocking claims'
    )
  )
);

const RuntimeReportDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeReportDeliveryProofSchemaVersionSchema,
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  runtimeReportDeliveryRows: Schema.Array(AppInstallPurchaseRuntimeReportDeliveryRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseRuntimeReportDeliveryNonClaimSchema),
  knownGaps: Schema.Array(RuntimeReportDeliveryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeReportDeliveryProof = Infer<typeof RuntimeReportDeliveryProofBaseSchema>;

export const AppInstallPurchaseRuntimeReportDeliveryProofSchema = withParser(
  RuntimeReportDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeReportDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase runtime report delivery proof to cover every report surface and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeReportDeliveryKnownGaps = [
  'Runtime report delivery rows are parent-owned report receipt proof only; no portal report UI is implemented.',
  'The proof does not execute provider APIs, store integrations, platform adapters, child-device delivery, or app blocking.',
  'Child activity data custody and Ocentra-hosted family data custody remain unclaimed.',
] as const;

export const AppInstallPurchaseRuntimeReportDeliveryProofReadModel =
  AppInstallPurchaseRuntimeReportDeliveryProofSchema.parse({
    schemaVersion: RuntimeReportDeliveryProofVersion,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    runtimeReportDeliveryRows:
      AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.map(runtimeReportDeliveryRow),
    nonClaims: RequiredRuntimeReportDeliveryNonClaims,
    knownGaps: AppInstallPurchaseRuntimeReportDeliveryKnownGaps,
    updatedAt: RuntimeReportDeliveryTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeReportDeliveryProof(
  proof: AppInstallPurchaseRuntimeReportDeliveryProof
) {
  return {
    runtimeReportDeliveryRows: proof.runtimeReportDeliveryRows.length,
    deliveredRows: proof.runtimeReportDeliveryRows.filter((row) => row.deliveryState === 'runtime-report-delivered')
      .length,
    receiptRows: proof.runtimeReportDeliveryRows.filter((row) => row.runtimeReportReceiptRef.length > 0).length,
    portalReportUiRows: proof.runtimeReportDeliveryRows.filter((row) => row.portalReportUiClaim !== 'not-claimed')
      .length,
    childDeviceDeliveryRows: proof.runtimeReportDeliveryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function runtimeReportDeliveryRow(
  row: (typeof AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows)[number]
) {
  return {
    schemaVersion: RuntimeReportDeliveryProofVersion,
    runtimeReportDeliveryRowId: `runtime-report-delivery-${row.reportSurface}`,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    sourceReportRuntimeRowId: row.reportRuntimeRowId,
    reportSurface: row.reportSurface,
    compilerRequestId: row.compilerRequestId,
    compilerOutputReportRef: row.outputReportRef,
    runtimeReportReceiptRef: `runtime-report-receipt-${row.reportSurface}`,
    sourceChildArtifactRefs: row.childArtifactRefs,
    deliveryState: 'runtime-report-delivered',
    parentAuthorized: row.parentAuthorized,
    rawEvidenceExcludedFromOutput: row.rawEvidenceExcludedFromOutput,
    childDetailMinimized: row.childDetailMinimized,
    tempDeletionConfirmed: row.tempDeletionConfirmed,
    localEvidenceMutated: row.localEvidenceMutated,
    ocentraHostedReportRetained: row.ocentraHostedReportRetained,
    runtimeReportDeliveryClaim: 'parent-runtime-delivered',
    portalReportUiClaim: 'not-claimed',
    providerApiExecutionClaim: 'not-claimed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    childDataCustody: 'no-child-activity-data',
    appBlockingClaim: 'not-claimed',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: RuntimeReportDeliveryBoundary,
    deliveredAt: RuntimeReportDeliveryTimestamp,
  } as const;
}

function runtimeReportDeliveryRowIsHonest(row: RuntimeReportDeliveryRowCandidate): boolean {
  return (
    row.deliveryState === 'runtime-report-delivered' &&
    row.runtimeReportReceiptRef.length > 0 &&
    row.compilerOutputReportRef.length > 0 &&
    row.sourceChildArtifactRefs.length > 0 &&
    runtimeReportDeliveryCustodyIsSafe(row) &&
    runtimeReportDeliveryClaimsStayUnimplemented(row) &&
    RuntimeReportDeliveryBoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function runtimeReportDeliveryCustodyIsSafe(row: RuntimeReportDeliveryRowCandidate): boolean {
  return (
    row.parentAuthorized &&
    row.rawEvidenceExcludedFromOutput &&
    row.childDetailMinimized &&
    row.tempDeletionConfirmed &&
    !row.localEvidenceMutated &&
    !row.ocentraHostedReportRetained
  );
}

function runtimeReportDeliveryClaimsStayUnimplemented(row: RuntimeReportDeliveryRowCandidate): boolean {
  return (
    row.runtimeReportDeliveryClaim === 'parent-runtime-delivered' &&
    row.portalReportUiClaim === 'not-claimed' &&
    row.providerApiExecutionClaim === 'not-claimed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function runtimeReportDeliveryProofIsHonest(proof: AppInstallPurchaseRuntimeReportDeliveryProof): boolean {
  return (
    proof.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    proof.runtimeReportDeliveryRows.length === AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length &&
    proof.runtimeReportDeliveryRows.every(runtimeReportDeliveryRowIsHonest) &&
    RequiredRuntimeReportDeliveryNonClaims.every((claim) => proof.nonClaims.includes(claim)) &&
    proof.knownGaps.length > 0
  );
}
