import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildArtifactDeliveryProofReadModel } from './app-install-purchase-child-artifact-delivery-proof';
import { AppInstallPurchasePlatformArtifactProofReadModel } from './app-install-purchase-platform-artifact-proof';
import {
  StatelessReportCompilerContractProofReadModel,
  StatelessReportCompilerSchemaVersionSchema,
} from '@ocentra-parent/schema-domain/stateless-report-compiler-status';
import { RequiredStatelessReportCompilerStatuses } from '@ocentra-parent/schema-domain/stateless-report-compiler-status-values';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseReportRuntimeSurfaceRowGenerated,
  reportRuntimeProofIsHonestGenerated,
  reportRuntimeSurfaceRowIsHonestGenerated,
  summarizeAppInstallPurchaseReportRuntimeProofGenerated,
} from './generated/app-install-purchase-report-status-helpers';
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
  'The compiler proof remains rust-parent-runtime status proof only and does not perform provider reads, upload/download, cloud worker execution, or custody of family data.',
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
  return summarizeAppInstallPurchaseReportRuntimeProofGenerated(proof);
}

function reportRuntimeSurfaceRow(
  row: (typeof AppInstallPurchasePlatformArtifactProofReadModel.reportRuntimeEvidence)[number]
) {
  return buildAppInstallPurchaseReportRuntimeSurfaceRowGenerated(
    row,
    StatelessReportCompilerContractProofReadModel,
    AppInstallPurchaseChildArtifactDeliveryProofReadModel.childDeliveryBoundaries.map((deliveryRow) => deliveryRow.childArtifactRef),
    row.auditEventRefs,
    ReportRuntimeClaimBoundary,
    ReportRuntimeTimestamp
  );
}

function reportRuntimeSurfaceRowIsHonest(row: ReportRuntimeSurfaceRowCandidate): boolean {
  return reportRuntimeSurfaceRowIsHonestGenerated(
    row,
    RequiredStatelessReportCompilerStatuses,
    RequiredFinalCompilerStatuses,
    [
      'no portal report UI',
      'no runtime report delivery',
      'no store integration',
      'no provider API',
      'no platform adapter',
      'no child-device delivery',
      'no child activity data',
      'no app blocking',
      'no Ocentra-hosted family data custody',
    ]
  );
}

function reportRuntimeProofIsHonest(proof: AppInstallPurchaseReportRuntimeProof): boolean {
  return (
    reportRuntimeProofIsHonestGenerated(
      proof,
      SourceChildArtifactProofVersion,
      SourcePlatformArtifactProofVersion,
      StatelessReportCompilerContractProofReadModel.schemaVersion,
      RequiredReportSurfaces,
      ReportRuntimeNonClaims
    ) && proof.reportRuntimeRows.every(reportRuntimeSurfaceRowIsHonest)
  );
}
