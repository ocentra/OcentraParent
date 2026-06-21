import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseApprovalReportDomainProofReadModel } from './app-install-purchase-approval-report-domain-proof';
import { AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel } from './app-install-purchase-runtime-report-writer-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ReportStatusReadModelProofVersion = 'app-install-purchase-report-status-read-model-handoff-proof';
const SourceRuntimeReportWriterDeliveryProofVersion = 'app-install-purchase-runtime-report-writer-delivery-proof';
const SourceApprovalReportDomainProofVersion = 'app-install-purchase-approval-report-domain-proof';
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
const ReportStatusReadModelBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseReportStatusReadModelBoundary');

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
  'Report status read-model rows are parent-domain proof rows only; no portal report UI or external runtime report delivery is implemented.',
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
  return {
    reportStatusReadModelRows: proof.reportStatusReadModelRows.length,
    readyRows: proof.reportStatusReadModelRows.filter(
      (row) => row.parentVisibleReportStatusState === 'parent-report-status-ready'
    ).length,
    manualRequiredRows: proof.reportStatusReadModelRows.filter(
      (row) => row.parentVisibleReportStatusState === 'manual-required'
    ).length,
    portalReportUiRows: proof.reportStatusReadModelRows.filter((row) => row.portalReportUiClaim !== 'not-implemented')
      .length,
    externallyDeliveredRows: proof.reportStatusReadModelRows.filter(
      (row) => row.runtimeReportDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function reportStatusReadModelRow(
  row: (typeof AppInstallPurchaseRuntimeReportWriterDeliveryProofReadModel.runtimeReportWriterDeliveryRows)[number]
) {
  const approvalRow = approvalReportDomainRowFor(row.sourceDecisionAction);
  return {
    schemaVersion: ReportStatusReadModelProofVersion,
    reportStatusReadModelRowId: `report-status-read-model-${row.sourceDecisionAction}`,
    sourceRuntimeReportWriterDeliveryProofVersion: SourceRuntimeReportWriterDeliveryProofVersion,
    sourceRuntimeReportWriterDeliveryRowId: row.runtimeReportWriterDeliveryRowId,
    sourceApprovalReportDomainProofVersion: SourceApprovalReportDomainProofVersion,
    sourceApprovalReportDomainRowId: approvalRow.approvalReportDomainRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceApprovalReportDomainState: approvalRow.approvalReportDomainState,
    sourceRuntimeReportWriterDeliveryState: row.runtimeReportWriterDeliveryState,
    sourceRuntimeReportWriterReceiptState: row.runtimeReportWriterReceiptState,
    parentVisibleReportStatusState:
      row.sourceDecisionAction === 'review-needed' ? 'manual-required' : 'parent-report-status-ready',
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
    claimBoundary: ReportStatusReadModelBoundary,
    recordedAt: ReportStatusReadModelTimestamp,
  } as const;
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
  return (
    reportStatusReadModelStateMatchesAction(row) &&
    reportStatusReadModelRefsAreComplete(row) &&
    reportStatusReadModelClaimsStayUnimplemented(row) &&
    ReportStatusReadModelBoundaryFragments.every((fragment) => row.claimBoundary.includes(fragment))
  );
}

function reportStatusReadModelStateMatchesAction(row: ReportStatusReadModelRowCandidate): boolean {
  return row.sourceDecisionAction === 'review-needed'
    ? row.parentVisibleReportStatusState === 'manual-required'
    : row.parentVisibleReportStatusState === 'parent-report-status-ready';
}

function reportStatusReadModelRefsAreComplete(row: ReportStatusReadModelRowCandidate): boolean {
  return (
    row.sourceRuntimeReportWriterDeliveryRowId.length > 0 &&
    row.sourceApprovalReportDomainRowId.length > 0 &&
    row.parentVisibleReportStatusRef.length > 0 &&
    row.parentVisibleReportReceiptRef.length > 0 &&
    row.reportAuditEventRefs.length > 0
  );
}

function reportStatusReadModelClaimsStayUnimplemented(row: ReportStatusReadModelRowCandidate): boolean {
  return (
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function reportStatusReadModelProofIsHonest(proof: AppInstallPurchaseReportStatusReadModelHandoffProof): boolean {
  const actions = new Set(proof.reportStatusReadModelRows.map((row) => row.sourceDecisionAction));
  const states = new Set(proof.reportStatusReadModelRows.map((row) => row.parentVisibleReportStatusState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeReportWriterDeliveryProofVersion === SourceRuntimeReportWriterDeliveryProofVersion &&
    proof.sourceApprovalReportDomainProofVersion === SourceApprovalReportDomainProofVersion &&
    proof.reportStatusReadModelRows.length === ReportStatusReadModelActions.length &&
    ReportStatusReadModelActions.every((action) => actions.has(action)) &&
    ReportStatusReadModelStates.every((state) => states.has(state)) &&
    ReportStatusReadModelNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.reportStatusReadModelRows.every(reportStatusReadModelRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

