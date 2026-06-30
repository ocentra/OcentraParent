import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel } from './app-install-purchase-runtime-writer-execution-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseRuntimeReportWriterDeliveryRowGenerated,
  runtimeReportWriterDeliveryProofIsHonestGenerated,
  runtimeReportWriterDeliveryRowIsHonestGenerated,
  summarizeAppInstallPurchaseRuntimeReportWriterDeliveryProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
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

const RuntimeReportWriterDeliveryRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeReportWriterDeliveryRowId'
);
const RuntimeReportWriterDeliveryRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeReportWriterDeliveryRef'
);
const RuntimeReportWriterDeliveryAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeReportWriterDeliveryAuditRef'
);
const RuntimeReportWriterDeliveryBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeReportWriterDeliveryBoundary'
);

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
  return summarizeAppInstallPurchaseRuntimeReportWriterDeliveryProofGenerated(proof);
}

function runtimeReportWriterDeliveryRow(
  row: (typeof AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows)[number]
) {
  const reportRows = AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows;
  return buildAppInstallPurchaseRuntimeReportWriterDeliveryRowGenerated(
    row,
    reportRows,
    SourceRuntimeWriterExecutionDeliveryProofVersion,
    SourceReportRuntimeProofVersion,
    RuntimeReportWriterDeliveryBoundary,
    RuntimeReportWriterDeliveryTimestamp
  );
}

function runtimeReportWriterDeliveryRowIsHonest(row: RuntimeReportWriterDeliveryRowCandidate): boolean {
  return runtimeReportWriterDeliveryRowIsHonestGenerated(
    row,
    SourceRuntimeWriterExecutionDeliveryProofVersion,
    SourceReportRuntimeProofVersion,
    AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length,
    RuntimeReportWriterDeliveryBoundaryFragments
  );
}

function runtimeReportWriterDeliveryProofIsHonest(proof: AppInstallPurchaseRuntimeReportWriterDeliveryProof): boolean {
  return (
    runtimeReportWriterDeliveryProofIsHonestGenerated(
      proof,
      SourceRuntimeWriterExecutionDeliveryProofVersion,
      SourceReportRuntimeProofVersion,
      RuntimeReportWriterDeliveryActions,
      RuntimeReportWriterDeliveryStates,
      RuntimeReportWriterReceiptStates,
      RuntimeReportWriterDeliveryNonClaims
    ) &&
    proof.runtimeReportWriterDeliveryRows.every(runtimeReportWriterDeliveryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
