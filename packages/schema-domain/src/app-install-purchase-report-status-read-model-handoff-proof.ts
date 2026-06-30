import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovalReportDomainProofReadModel } from './app-install-purchase-approval-report-domain-proof';
import { AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel } from './app-install-purchase-runtime-report-writer-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseReportStatusReadModelRowGenerated,
  reportStatusReadModelProofIsHonestGenerated,
  reportStatusReadModelRowIsHonestGenerated,
  summarizeAppInstallPurchaseReportStatusReadModelHandoffProofGenerated,
} from './generated/app-install-purchase-report-status-helpers';
const ReportStatusReadModelProofVersion = 'app-install-purchase-report-status-read-model-handoff-proof';
const SourceRuntimeReportWriterDeliveryProofVersion =
  AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.schemaVersion;
const SourceApprovalReportDomainProofVersion = AppInstallPurchaseApprovalReportDomainProofReadModel.schemaVersion;
const ReportStatusReadModelTimestamp = '2026-06-06T02:40:00.000Z';
const ReportStatusReadModelBoundary =
  'report status read-model handoff proof only; parent-visible report status rows link approval report domain rows to runtime report writer delivery rows no portal report UI no external runtime report delivery no provider API execution no store integration no platform adapter implementation no child-device delivery no app blocking no child activity data no Ocentra-hosted family data custody';
const ReportStatusReadModelActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const ReportStatusReadModelStates = ['parent-report-status-ready', 'manual-required'] as const;
const ReportStatusReadModelNonClaims = [
  'no-portal-report-ui',
  'no-external-runtime-report-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ReportStatusReadModelBoundaryFragments = [
  'parent-visible report status rows',
  'approval report domain rows',
  'runtime report writer delivery rows',
  'no portal report UI',
  'no external runtime report delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseReportStatusReadModelHandoffProofSchemaVersionSchema = withParser(
  Schema.Literal(ReportStatusReadModelProofVersion)
);
const ReportStatusReadModelActionSchema = withParser(Schema.Literal(...ReportStatusReadModelActions));
const ReportStatusReadModelStateSchema = withParser(Schema.Literal(...ReportStatusReadModelStates));
const ReportStatusReadModelRuntimeDeliveryStateSchema = withParser(
  Schema.Literal('report-delivery-ready', 'manual-required')
);
const ReportStatusReadModelReceiptStateSchema = withParser(
  Schema.Literal('parent-owned-report-receipt-recorded', 'manual-required')
);
const ReportStatusReadModelApprovalStateSchema = withParser(
  Schema.Literal('approval-report-ready', 'approval-report-manual-review')
);
const ReportStatusReadModelNotImplementedSchema = withParser(Schema.Literal('not-implemented'));
const ReportStatusReadModelNotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const ReportStatusReadModelNotExecutedSchema = withParser(Schema.Literal('not-executed'));
const ReportStatusReadModelNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const ReportStatusReadModelCustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const ReportStatusReadModelNonClaimSchema = withParser(Schema.Literal(...ReportStatusReadModelNonClaims));

const ReportStatusReadModelRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseReportStatusReadModelRowId');
const ReportStatusReadModelRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseReportStatusReadModelRef');
const ReportStatusReadModelBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseReportStatusReadModelBoundary'
);

const ReportStatusReadModelRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseReportStatusReadModelHandoffProofSchemaVersionSchema,
  reportStatusReadModelRowId: ReportStatusReadModelRowIdSchema,
  sourceRuntimeReportWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeReportWriterDeliveryProofVersion),
  sourceRuntimeReportWriterDeliveryRowId: ReportStatusReadModelRefSchema,
  sourceApprovalReportDomainProofVersion: Schema.Literal(SourceApprovalReportDomainProofVersion),
  sourceApprovalReportDomainRowId: ReportStatusReadModelRefSchema,
  sourceDecisionAction: ReportStatusReadModelActionSchema,
  sourceApprovalReportDomainState: ReportStatusReadModelApprovalStateSchema,
  sourceRuntimeReportWriterDeliveryState: ReportStatusReadModelRuntimeDeliveryStateSchema,
  sourceRuntimeReportWriterReceiptState: ReportStatusReadModelReceiptStateSchema,
  parentVisibleReportStatusState: ReportStatusReadModelStateSchema,
  parentVisibleReportStatusRef: ReportStatusReadModelRefSchema,
  parentVisibleReportReceiptRef: ReportStatusReadModelRefSchema,
  reportAuditEventRefs: Schema.Array(ReportStatusReadModelRefSchema),
  portalReportUiClaim: ReportStatusReadModelNotImplementedSchema,
  runtimeReportDeliveryClaim: ReportStatusReadModelNotDeliveredSchema,
  providerApiExecutionClaim: ReportStatusReadModelNotExecutedSchema,
  storeIntegrationClaim: ReportStatusReadModelNotClaimedSchema,
  platformAdapterClaim: ReportStatusReadModelNotImplementedSchema,
  childDeviceDeliveryClaim: ReportStatusReadModelNotDeliveredSchema,
  appBlockingClaim: ReportStatusReadModelNotClaimedSchema,
  childDataCustody: ReportStatusReadModelCustodySchema,
  ocentraHostedFamilyDataCustodyClaim: ReportStatusReadModelNotClaimedSchema,
  claimBoundary: ReportStatusReadModelBoundarySchema,
  recordedAt: ParentTimestampSchema,
});

type ReportStatusReadModelRowCandidate = Infer<typeof ReportStatusReadModelRowBaseSchema>;

export const AppInstallPurchaseReportStatusReadModelHandoffRowSchema = withParser(
  ReportStatusReadModelRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        reportStatusReadModelRowIsHonest(row) ||
        'Expected app install/purchase report status read-model rows to link approval/report domain and runtime report writer refs without portal, delivery, provider, adapter, custody, or blocking claims'
    )
  )
);

const ReportStatusReadModelProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseReportStatusReadModelHandoffProofSchemaVersionSchema,
  sourceRuntimeReportWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeReportWriterDeliveryProofVersion),
  sourceApprovalReportDomainProofVersion: Schema.Literal(SourceApprovalReportDomainProofVersion),
  reportStatusReadModelRows: Schema.Array(AppInstallPurchaseReportStatusReadModelHandoffRowSchema),
  nonClaims: Schema.Array(ReportStatusReadModelNonClaimSchema),
  knownGaps: Schema.Array(ReportStatusReadModelRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseReportStatusReadModelHandoffProof = Infer<typeof ReportStatusReadModelProofBaseSchema>;

export const AppInstallPurchaseReportStatusReadModelHandoffProofSchema = withParser(
  ReportStatusReadModelProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        reportStatusReadModelProofIsHonest(proof) ||
        'Expected app install/purchase report status read-model handoff proof to cover parent-visible report status rows and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseReportStatusReadModelHandoffKnownGaps = [
  'Report status read-model rows are rust-parent-runtime proof rows only; no portal report UI or external runtime report delivery is implemented.',
  'Provider/store execution, platform adapters, child-device delivery, app blocking, child activity data, and hosted family data custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval/report UI and a real parent approval action path exist.',
] as const;

export const AppInstallPurchaseReportStatusReadModelHandoffProofReadModel =
  AppInstallPurchaseReportStatusReadModelHandoffProofSchema.parse({
    schemaVersion: ReportStatusReadModelProofVersion,
    sourceRuntimeReportWriterDeliveryProofVersion: SourceRuntimeReportWriterDeliveryProofVersion,
    sourceApprovalReportDomainProofVersion: SourceApprovalReportDomainProofVersion,
    reportStatusReadModelRows:
      AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.runtimeReportWriterDeliveryRows.map(
        reportStatusReadModelRow
      ),
    nonClaims: ReportStatusReadModelNonClaims,
    knownGaps: AppInstallPurchaseReportStatusReadModelHandoffKnownGaps,
    updatedAt: ReportStatusReadModelTimestamp,
  });

export function summarizeAppInstallPurchaseReportStatusReadModelHandoffProof(
  proof: AppInstallPurchaseReportStatusReadModelHandoffProof
) {
  return summarizeAppInstallPurchaseReportStatusReadModelHandoffProofGenerated(proof);
}

function reportStatusReadModelRow(
  row: (typeof AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.runtimeReportWriterDeliveryRows)[number]
) {
  return buildAppInstallPurchaseReportStatusReadModelRowGenerated(
    row,
    approvalReportDomainRowFor(row.sourceDecisionAction),
    SourceRuntimeReportWriterDeliveryProofVersion,
    SourceApprovalReportDomainProofVersion,
    ReportStatusReadModelBoundary,
    ReportStatusReadModelTimestamp
  );
}

function approvalReportDomainRowFor(action: (typeof ReportStatusReadModelActions)[number]) {
  const row = AppInstallPurchaseApprovalReportDomainProofReadModel.approvalReportDomainRows.find(
    (candidate) => candidate.sourceDecisionAction === action
  );
  if (row === undefined) {
    throw new Error(`missing approval report domain row for ${action}`);
  }
  return row;
}

function reportStatusReadModelRowIsHonest(row: ReportStatusReadModelRowCandidate): boolean {
  return reportStatusReadModelRowIsHonestGenerated(row, ReportStatusReadModelBoundaryFragments);
}

function reportStatusReadModelProofIsHonest(proof: AppInstallPurchaseReportStatusReadModelHandoffProof): boolean {
  return (
    reportStatusReadModelProofIsHonestGenerated(
      proof,
      ReportStatusReadModelActions,
      ReportStatusReadModelStates,
      ReportStatusReadModelNonClaims
    ) && proof.reportStatusReadModelRows.every(reportStatusReadModelRowIsHonest)
  );
}
