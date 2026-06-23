import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildArtifactDeliveryProofReadModel } from './app-install-purchase-child-artifact-delivery-proof';
import { AppInstallPurchasePlatformArtifactProofReadModel } from './app-install-purchase-platform-artifact-proof';
import {
  StatelessReportCompilerContractProofReadModel,
  StatelessReportCompilerSchemaVersionSchema,
} from '@ocentra-parent/schema-domain/stateless-report-compiler-status';
import { RequiredStatelessReportCompilerStatuses } from '@ocentra-parent/schema-domain/stateless-report-compiler-status-values';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ReportRuntimeSchemaVersion = 'app-install-purchase-report-runtime-proof';
const SourceChildArtifactProofVersion = 'app-install-purchase-child-artifact-delivery-proof';
const SourcePlatformArtifactProofVersion = 'app-install-purchase-platform-artifact-proof';
const ReportRuntimeTimestamp = '2026-06-05T00:48:00.000Z';
const ReportRuntimeClaimBoundary =
  'report runtime status proof only; no portal report UI no runtime report delivery no store integration no provider API no platform adapter no child-device delivery no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredReportSurfaces = [
  'request-audit-history',
  'parent-decision-audit-history',
  'child-facing-state-report',
  'platform-limitation-report',
] as const;
const RequiredFinalCompilerStatuses = ['succeeded', 'failed', 'expired', 'manual-required'] as const;
const ReportRuntimeNonClaims = [
  'no-portal-report-ui',
  'no-runtime-report-delivery',
  'no-store-integration',
  'no-provider-api',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-child-activity-data',
  'no-app-blocking',
  'no-child-device-mutation',
  'no-ocentra-hosted-family-data-custody',
] as const;

export const AppInstallPurchaseReportRuntimeProofSchemaVersionSchema = withParser(
  Schema.Literal(ReportRuntimeSchemaVersion)
);
const AppInstallPurchaseReportRuntimeSurfaceSchema = withParser(Schema.Literal(...RequiredReportSurfaces));
const AppInstallPurchaseReportRuntimeCompilerStatusSchema = withParser(
  Schema.Literal(...RequiredStatelessReportCompilerStatuses)
);
const AppInstallPurchaseReportRuntimeFinalStatusSchema = withParser(Schema.Literal(...RequiredFinalCompilerStatuses));
const AppInstallPurchaseReportRuntimeStatusClaimSchema = withParser(Schema.Literal('compiler-status-linked'));
const AppInstallPurchaseReportRuntimeDeliveryClaimSchema = withParser(Schema.Literal('not-portal-delivered'));
const AppInstallPurchaseReportRuntimePortalClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseReportRuntimeProviderApiClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseReportRuntimeStoreIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseReportRuntimeAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseReportRuntimeChildDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseReportRuntimeDataCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseReportRuntimeAppBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseReportRuntimeHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseReportRuntimeNonClaimSchema = withParser(Schema.Literal(...ReportRuntimeNonClaims));

const ReportRuntimeRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseReportRuntimeRowId');
const ReportRuntimeRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseReportRuntimeRef');
const ReportRuntimeChildArtifactRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseReportRuntimeChildArtifactRef'
);
const ReportRuntimeClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseReportRuntimeClaimBoundary');

const ReportRuntimeSurfaceRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseReportRuntimeProofSchemaVersionSchema,
  reportRuntimeRowId: ReportRuntimeRowIdSchema,
  reportSurface: AppInstallPurchaseReportRuntimeSurfaceSchema,
  sourceReportRef: ReportRuntimeRefSchema,
  sourceReportCompilerSchemaVersion: StatelessReportCompilerSchemaVersionSchema,
  compilerRequestId: ReportRuntimeRefSchema,
  compilerStatusRefs: Schema.Array(ReportRuntimeRefSchema),
  compilerStatuses: Schema.Array(AppInstallPurchaseReportRuntimeCompilerStatusSchema),
  compilerResultRefs: Schema.Array(ReportRuntimeRefSchema),
  compilerFinalResultStatuses: Schema.Array(AppInstallPurchaseReportRuntimeFinalStatusSchema),
  outputReportRef: ReportRuntimeRefSchema,
  childArtifactRefs: Schema.Array(ReportRuntimeChildArtifactRefSchema),
  parentAuthorized: Schema.Boolean,
  rawChildEvidenceRequested: Schema.Boolean,
  rawEvidenceExcludedFromOutput: Schema.Boolean,
  childDetailMinimized: Schema.Boolean,
  tempDeletionConfirmed: Schema.Boolean,
  localEvidenceMutated: Schema.Boolean,
  ocentraHostedReportRetained: Schema.Boolean,
  reportRuntimeStatusClaim: AppInstallPurchaseReportRuntimeStatusClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseReportRuntimeDeliveryClaimSchema,
  portalUiClaim: AppInstallPurchaseReportRuntimePortalClaimSchema,
  providerApiClaim: AppInstallPurchaseReportRuntimeProviderApiClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseReportRuntimeStoreIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseReportRuntimeAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchaseReportRuntimeChildDeliveryClaimSchema,
  childDataCustody: AppInstallPurchaseReportRuntimeDataCustodyClaimSchema,
  appBlockingClaim: AppInstallPurchaseReportRuntimeAppBlockingClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseReportRuntimeHostedCustodyClaimSchema,
  claimBoundary: ReportRuntimeClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ReportRuntimeSurfaceRowCandidate = Infer<typeof ReportRuntimeSurfaceRowBaseSchema>;

export const AppInstallPurchaseReportRuntimeSurfaceRowSchema = withParser(
  ReportRuntimeSurfaceRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        reportRuntimeSurfaceRowIsHonest(row) ||
        'Expected app install/purchase report runtime rows to link compiler statuses without portal, provider, store, adapter, delivery, custody, or blocking claims'
    )
  )
);

const ReportRuntimeProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseReportRuntimeProofSchemaVersionSchema,
  sourceChildArtifactProofVersion: Schema.Literal(SourceChildArtifactProofVersion),
  sourcePlatformArtifactProofVersion: Schema.Literal(SourcePlatformArtifactProofVersion),
  sourceReportCompilerSchemaVersion: StatelessReportCompilerSchemaVersionSchema,
  reportRuntimeRows: Schema.Array(AppInstallPurchaseReportRuntimeSurfaceRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseReportRuntimeNonClaimSchema),
  knownGaps: Schema.Array(ReportRuntimeRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseReportRuntimeProof = Infer<typeof ReportRuntimeProofBaseSchema>;

export const AppInstallPurchaseReportRuntimeProofSchema = withParser(
  ReportRuntimeProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        reportRuntimeProofIsHonest(proof) ||
        'Expected app install/purchase report runtime proof to cover report surfaces and preserve runtime non-claims'
    )
  )
);

export const AppInstallPurchaseReportRuntimeKnownGaps = [
  'Report runtime rows link app-install report surfaces to stateless report compiler status/result refs only; no portal report UI or report writer is implemented.',
  'The compiler proof remains parent-domain status proof only and does not perform provider reads, upload/download, cloud worker execution, or custody of family data.',
  'Child-device package capture, child delivery, platform adapters, store/provider APIs, real interception, and app blocking remain unimplemented.',
] as const;

export const AppInstallPurchaseReportRuntimeProofReadModel = AppInstallPurchaseReportRuntimeProofSchema.parse({
  schemaVersion: ReportRuntimeSchemaVersion,
  sourceChildArtifactProofVersion: SourceChildArtifactProofVersion,
  sourcePlatformArtifactProofVersion: SourcePlatformArtifactProofVersion,
  sourceReportCompilerSchemaVersion: StatelessReportCompilerContractProofReadModel.schemaVersion,
  reportRuntimeRows:
    AppInstallPurchasePlatformArtifactProofReadModel.reportRuntimeEvidence.map(reportRuntimeSurfaceRow),
  nonClaims: ReportRuntimeNonClaims,
  knownGaps: AppInstallPurchaseReportRuntimeKnownGaps,
  updatedAt: ReportRuntimeTimestamp,
});

export function summarizeAppInstallPurchaseReportRuntimeProof(proof: AppInstallPurchaseReportRuntimeProof) {
  return {
    reportRuntimeRows: proof.reportRuntimeRows.length,
    compilerLinkedRows: proof.reportRuntimeRows.filter(
      (row: AppInstallPurchaseReportRuntimeProof['reportRuntimeRows'][number]) =>
        row.reportRuntimeStatusClaim === 'compiler-status-linked'
    ).length,
    outputReportRefs: proof.reportRuntimeRows.filter(
      (row: AppInstallPurchaseReportRuntimeProof['reportRuntimeRows'][number]) => row.outputReportRef.length > 0
    ).length,
    portalDeliveredRows: proof.reportRuntimeRows.filter(
      (row: AppInstallPurchaseReportRuntimeProof['reportRuntimeRows'][number]) =>
        row.runtimeReportDeliveryClaim !== 'not-portal-delivered'
    ).length,
  } as const;
}

function reportRuntimeSurfaceRow(
  row: (typeof AppInstallPurchasePlatformArtifactProofReadModel.reportRuntimeEvidence)[number]
) {
  const successfulResult = successfulCompilerResult();
  return {
    schemaVersion: ReportRuntimeSchemaVersion,
    reportRuntimeRowId: `app-install-report-runtime-${row.reportSurface}`,
    reportSurface: row.reportSurface,
    sourceReportRef: row.reportRefs[0],
    sourceReportCompilerSchemaVersion: StatelessReportCompilerContractProofReadModel.schemaVersion,
    compilerRequestId: StatelessReportCompilerContractProofReadModel.request.requestId,
    compilerStatusRefs: StatelessReportCompilerContractProofReadModel.statuses.map((status) => status.statusRef),
    compilerStatuses: StatelessReportCompilerContractProofReadModel.statuses.map((status) => status.status),
    compilerResultRefs: StatelessReportCompilerContractProofReadModel.results.map((result) => result.resultRef),
    compilerFinalResultStatuses: StatelessReportCompilerContractProofReadModel.results.map((result) => result.status),
    outputReportRef: successfulResult.outputReportRef,
    childArtifactRefs: AppInstallPurchaseChildArtifactDeliveryProofReadModel.childDeliveryBoundaries.map(
      (deliveryRow) => deliveryRow.childArtifactRef
    ),
    parentAuthorized: StatelessReportCompilerContractProofReadModel.request.parentAuthorized,
    rawChildEvidenceRequested: StatelessReportCompilerContractProofReadModel.request.rawChildEvidenceRequested,
    rawEvidenceExcludedFromOutput: successfulResult.redaction.rawEvidenceExcludedFromOutput,
    childDetailMinimized: successfulResult.redaction.childDetailMinimized,
    tempDeletionConfirmed: successfulResult.tempArtifacts.deletionConfirmed,
    localEvidenceMutated: successfulResult.localEvidenceMutated,
    ocentraHostedReportRetained: successfulResult.ocentraHostedReportRetained,
    reportRuntimeStatusClaim: 'compiler-status-linked',
    runtimeReportDeliveryClaim: 'not-portal-delivered',
    portalUiClaim: 'not-claimed',
    providerApiClaim: row.providerApiClaim,
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: row.platformAdapterClaim,
    childDeliveryClaim: 'not-delivered',
    childDataCustody: 'no-child-activity-data',
    appBlockingClaim: 'not-claimed',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: ReportRuntimeClaimBoundary,
    linkedAt: ReportRuntimeTimestamp,
  } as const;
}

function successfulCompilerResult() {
  const result = StatelessReportCompilerContractProofReadModel.results.find((row) => row.status === 'succeeded');
  if (result === undefined || result.outputReportRef === null) {
    throw new Error('missing succeeded stateless report compiler output ref');
  }
  return result;
}

function reportRuntimeSurfaceRowIsHonest(row: ReportRuntimeSurfaceRowCandidate): boolean {
  return (
    reportRuntimeClaimsStayUnimplemented(row) &&
    reportRuntimeCompilerEvidenceIsComplete(row) &&
    reportRuntimeCustodyIsSafe(row) &&
    reportRuntimeBoundaryIsExplicit(row.claimBoundary)
  );
}

function reportRuntimeClaimsStayUnimplemented(row: ReportRuntimeSurfaceRowCandidate): boolean {
  return (
    row.reportRuntimeStatusClaim === 'compiler-status-linked' &&
    row.runtimeReportDeliveryClaim === 'not-portal-delivered' &&
    row.portalUiClaim === 'not-claimed' &&
    row.providerApiClaim === 'not-claimed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function reportRuntimeCompilerEvidenceIsComplete(row: ReportRuntimeSurfaceRowCandidate): boolean {
  return (
    row.compilerStatusRefs.length === row.compilerStatuses.length &&
    row.compilerResultRefs.length === row.compilerFinalResultStatuses.length &&
    row.childArtifactRefs.length > 0 &&
    compilerStatusesAreComplete(row.compilerStatuses) &&
    finalCompilerStatusesAreComplete(row.compilerFinalResultStatuses)
  );
}

function reportRuntimeCustodyIsSafe(row: ReportRuntimeSurfaceRowCandidate): boolean {
  return (
    row.parentAuthorized &&
    !row.rawChildEvidenceRequested &&
    row.rawEvidenceExcludedFromOutput &&
    row.childDetailMinimized &&
    row.tempDeletionConfirmed &&
    !row.localEvidenceMutated &&
    !row.ocentraHostedReportRetained
  );
}

function reportRuntimeProofIsHonest(proof: AppInstallPurchaseReportRuntimeProof): boolean {
  return (
    proof.sourceChildArtifactProofVersion === SourceChildArtifactProofVersion &&
    proof.sourcePlatformArtifactProofVersion === SourcePlatformArtifactProofVersion &&
    proof.sourceReportCompilerSchemaVersion === StatelessReportCompilerContractProofReadModel.schemaVersion &&
    reportRuntimeRowsAreComplete(proof.reportRuntimeRows) &&
    nonClaimsAreComplete(proof.nonClaims) &&
    proof.knownGaps.length > 0
  );
}

function reportRuntimeRowsAreComplete(rows: readonly ReportRuntimeSurfaceRowCandidate[]): boolean {
  const surfaces = new Set(rows.map((row) => row.reportSurface));
  return (
    rows.length === RequiredReportSurfaces.length &&
    RequiredReportSurfaces.every((surface) => surfaces.has(surface)) &&
    rows.every((row) => reportRuntimeSurfaceRowIsHonest(row))
  );
}

function compilerStatusesAreComplete(statuses: readonly (typeof RequiredStatelessReportCompilerStatuses)[number][]) {
  const statusSet = new Set(statuses);
  return RequiredStatelessReportCompilerStatuses.every((status) => statusSet.has(status));
}

function finalCompilerStatusesAreComplete(statuses: readonly (typeof RequiredFinalCompilerStatuses)[number][]) {
  const statusSet = new Set(statuses);
  return RequiredFinalCompilerStatuses.every((status) => statusSet.has(status));
}

function nonClaimsAreComplete(nonClaims: readonly (typeof ReportRuntimeNonClaims)[number][]): boolean {
  const claimSet = new Set(nonClaims);
  return ReportRuntimeNonClaims.every((claim) => claimSet.has(claim));
}

function reportRuntimeBoundaryIsExplicit(boundary: typeof ReportRuntimeClaimBoundarySchema.Type): boolean {
  return (
    boundary.includes('no portal report UI') &&
    boundary.includes('no runtime report delivery') &&
    boundary.includes('no store integration') &&
    boundary.includes('no provider API') &&
    boundary.includes('no platform adapter') &&
    boundary.includes('no child-device delivery') &&
    boundary.includes('no child activity data') &&
    boundary.includes('no app blocking') &&
    boundary.includes('no Ocentra-hosted family data custody')
  );
}
