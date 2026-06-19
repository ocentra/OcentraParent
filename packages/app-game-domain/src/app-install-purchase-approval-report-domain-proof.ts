import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentReviewActionProofReadModel } from './app-install-purchase-parent-review-action-proof';
import { AppInstallPurchaseReportRuntimeProofReadModel } from './app-install-purchase-report-runtime-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ApprovalReportDomainProofVersion = 'app-install-purchase-approval-report-domain-proof';
const SourceParentReviewActionProofVersion = 'app-install-purchase-parent-review-action-proof';
const SourceReportRuntimeProofVersion = 'app-install-purchase-report-runtime-proof';
const ApprovalReportDomainTimestamp = '2026-06-05T22:24:00.000Z';
const ApprovalReportDomainClaimBoundary =
  'approval report domain proof only; no portal approval UI no portal report UI no runtime report delivery no provider API execution no store integration no platform adapter no child-device delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const RequiredApprovalReportStates = [
  'approval-report-ready',
  'approval-report-manual-review',
  'approval-report-unavailable',
] as const;
const RequiredApprovalReportNonClaims = [
  'no-portal-approval-ui',
  'no-portal-report-ui',
  'no-runtime-report-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter',
  'no-child-device-delivery',
  'no-real-install-or-purchase-interception',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const ApprovalReportBoundaryFragments = [
  'no portal approval UI',
  'no portal report UI',
  'no runtime report delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter',
  'no child-device delivery',
  'no real install or purchase interception',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseApprovalReportDomainProofSchemaVersionSchema = withParser(
  Schema.Literal(ApprovalReportDomainProofVersion)
);
const AppInstallPurchaseApprovalReportDomainStateSchema = withParser(Schema.Literal(...RequiredApprovalReportStates));
const AppInstallPurchaseApprovalReportDomainRuntimeClaimSchema = withParser(Schema.Literal('domain-read-model-only'));
const AppInstallPurchaseApprovalReportDomainMissingClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseApprovalReportDomainNotClaimedSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseApprovalReportDomainNotDeliveredSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseApprovalReportDomainNotExecutedSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseApprovalReportDomainCustodySchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseApprovalReportDomainNonClaimSchema = withParser(
  Schema.Literal(...RequiredApprovalReportNonClaims)
);

const ApprovalReportDomainRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalReportDomainRowId');
const ApprovalReportDomainRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalReportDomainRef');
const ApprovalReportDomainClaimBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseApprovalReportDomainClaimBoundary');

const ApprovalReportDomainRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalReportDomainProofSchemaVersionSchema,
  approvalReportDomainRowId: ApprovalReportDomainRowIdSchema,
  sourceParentReviewActionProofVersion: Schema.Literal(SourceParentReviewActionProofVersion),
  sourceParentReviewActionRowId: ApprovalReportDomainRefSchema,
  sourceDecisionAction: ApprovalReportDomainRefSchema,
  sourceParentReviewActionState: ApprovalReportDomainRefSchema,
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  sourceReportRuntimeRefs: Schema.Array(ApprovalReportDomainRefSchema),
  sourceReportSurfaces: Schema.Array(ApprovalReportDomainRefSchema),
  sourceAuditEventRefs: Schema.Array(ApprovalReportDomainRefSchema),
  approvalReportDomainState: AppInstallPurchaseApprovalReportDomainStateSchema,
  parentActionRecorded: Schema.Boolean,
  reportRuntimeLinked: Schema.Boolean,
  domainReadModelClaim: AppInstallPurchaseApprovalReportDomainRuntimeClaimSchema,
  portalApprovalUiClaim: AppInstallPurchaseApprovalReportDomainMissingClaimSchema,
  portalReportUiClaim: AppInstallPurchaseApprovalReportDomainMissingClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseApprovalReportDomainNotDeliveredSchema,
  providerApiExecutionClaim: AppInstallPurchaseApprovalReportDomainNotExecutedSchema,
  storeIntegrationClaim: AppInstallPurchaseApprovalReportDomainNotClaimedSchema,
  platformAdapterClaim: AppInstallPurchaseApprovalReportDomainMissingClaimSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseApprovalReportDomainNotDeliveredSchema,
  interceptionClaim: AppInstallPurchaseApprovalReportDomainNotClaimedSchema,
  appBlockingClaim: AppInstallPurchaseApprovalReportDomainNotClaimedSchema,
  childDataCustody: AppInstallPurchaseApprovalReportDomainCustodySchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseApprovalReportDomainNotClaimedSchema,
  claimBoundary: ApprovalReportDomainClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ApprovalReportDomainRowCandidate = Infer<typeof ApprovalReportDomainRowBaseSchema>;

export const AppInstallPurchaseApprovalReportDomainRowSchema = withParser(
  ApprovalReportDomainRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        approvalReportDomainRowIsHonest(row) ||
        'Expected app install/purchase approval report domain rows to link parent actions to report-runtime refs without portal, delivery, provider, adapter, custody, interception, or blocking claims'
    )
  )
);

const ApprovalReportDomainProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseApprovalReportDomainProofSchemaVersionSchema,
  sourceParentReviewActionProofVersion: Schema.Literal(SourceParentReviewActionProofVersion),
  sourceReportRuntimeProofVersion: Schema.Literal(SourceReportRuntimeProofVersion),
  approvalReportDomainRows: Schema.Array(AppInstallPurchaseApprovalReportDomainRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseApprovalReportDomainNonClaimSchema),
  knownGaps: Schema.Array(ApprovalReportDomainRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseApprovalReportDomainProof = Infer<typeof ApprovalReportDomainProofBaseSchema>;

export const AppInstallPurchaseApprovalReportDomainProofSchema = withParser(
  ApprovalReportDomainProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        approvalReportDomainProofIsHonest(proof) ||
        'Expected app install/purchase approval report domain proof to cover approval/report states and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseApprovalReportDomainKnownGaps = [
  'Approval/report domain rows are parent-domain read-model proof only; no portal approval UI or portal report UI is implemented.',
  'Report runtime refs are compiler-status proof rows only and do not deliver reports to a runtime or child device.',
  'Provider/store execution, platform adapters, real install or purchase interception, app blocking, child delivery, child activity custody, and hosted family data custody remain unimplemented.',
] as const;

export const AppInstallPurchaseApprovalReportDomainProofReadModel =
  AppInstallPurchaseApprovalReportDomainProofSchema.parse({
    schemaVersion: ApprovalReportDomainProofVersion,
    sourceParentReviewActionProofVersion: SourceParentReviewActionProofVersion,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    approvalReportDomainRows:
      AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows.map(approvalReportDomainRow),
    nonClaims: RequiredApprovalReportNonClaims,
    knownGaps: AppInstallPurchaseApprovalReportDomainKnownGaps,
    updatedAt: ApprovalReportDomainTimestamp,
  });

export function summarizeAppInstallPurchaseApprovalReportDomainProof(
  proof: AppInstallPurchaseApprovalReportDomainProof
) {
  return {
    approvalReportDomainRows: proof.approvalReportDomainRows.length,
    readyRows: proof.approvalReportDomainRows.filter((row) => row.approvalReportDomainState === 'approval-report-ready')
      .length,
    manualReviewRows: proof.approvalReportDomainRows.filter(
      (row) => row.approvalReportDomainState === 'approval-report-manual-review'
    ).length,
    unavailableRows: proof.approvalReportDomainRows.filter(
      (row) => row.approvalReportDomainState === 'approval-report-unavailable'
    ).length,
    reportLinkedRows: proof.approvalReportDomainRows.filter((row) => row.reportRuntimeLinked).length,
    portalApprovalUiRows: proof.approvalReportDomainRows.filter(
      (row) => row.portalApprovalUiClaim !== 'not-implemented'
    ).length,
    portalReportUiRows: proof.approvalReportDomainRows.filter((row) => row.portalReportUiClaim !== 'not-implemented')
      .length,
  } as const;
}

function approvalReportDomainRow(
  row: (typeof AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows)[number]
) {
  const reportRows = AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows;
  return {
    schemaVersion: ApprovalReportDomainProofVersion,
    approvalReportDomainRowId: `approval-report-domain-${row.sourceDecisionAction}`,
    sourceParentReviewActionProofVersion: SourceParentReviewActionProofVersion,
    sourceParentReviewActionRowId: row.parentReviewActionRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceParentReviewActionState: row.parentReviewActionState,
    sourceReportRuntimeProofVersion: SourceReportRuntimeProofVersion,
    sourceReportRuntimeRefs: reportRows.map((reportRow) => reportRow.reportRuntimeRowId),
    sourceReportSurfaces: reportRows.map((reportRow) => reportRow.reportSurface),
    sourceAuditEventRefs: row.auditEventRefs,
    approvalReportDomainState: approvalReportDomainState(row),
    parentActionRecorded: row.parentActionRecorded,
    reportRuntimeLinked: reportRows.length === 4,
    domainReadModelClaim: 'domain-read-model-only',
    portalApprovalUiClaim: 'not-implemented',
    portalReportUiClaim: 'not-implemented',
    runtimeReportDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: 'not-executed',
    storeIntegrationClaim: 'not-claimed',
    platformAdapterClaim: 'not-implemented',
    childDeviceDeliveryClaim: 'not-delivered',
    interceptionClaim: 'not-claimed',
    appBlockingClaim: 'not-claimed',
    childDataCustody: 'no-child-activity-data',
    ocentraHostedFamilyDataCustodyClaim: 'not-claimed',
    claimBoundary: ApprovalReportDomainClaimBoundary,
    linkedAt: ApprovalReportDomainTimestamp,
  } as const;
}

function approvalReportDomainState(
  row: (typeof AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows)[number]
) {
  if (row.parentActionRuntimeClaim === 'manual-review-state-only') {
    return 'approval-report-manual-review';
  }
  return row.sourceReportRuntimeRefs.length > 0 ? 'approval-report-ready' : 'approval-report-unavailable';
}

function approvalReportDomainRowIsHonest(row: ApprovalReportDomainRowCandidate): boolean {
  return (
    approvalReportDomainStateMatchesAction(row) &&
    approvalReportDomainRefsAreComplete(row) &&
    approvalReportDomainClaimsStayUnimplemented(row) &&
    approvalReportDomainBoundaryIsExplicit(row.claimBoundary)
  );
}

function approvalReportDomainStateMatchesAction(row: ApprovalReportDomainRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return !row.parentActionRecorded && row.approvalReportDomainState === 'approval-report-manual-review';
  }
  return row.parentActionRecorded && row.approvalReportDomainState === 'approval-report-ready';
}

function approvalReportDomainRefsAreComplete(row: ApprovalReportDomainRowCandidate): boolean {
  return (
    row.sourceParentReviewActionRowId.length > 0 &&
    row.sourceReportRuntimeRefs.length === AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length &&
    row.sourceReportSurfaces.length === AppInstallPurchaseReportRuntimeProofReadModel.reportRuntimeRows.length &&
    row.sourceAuditEventRefs.length > 0 &&
    row.reportRuntimeLinked
  );
}

function approvalReportDomainClaimsStayUnimplemented(row: ApprovalReportDomainRowCandidate): boolean {
  return (
    row.domainReadModelClaim === 'domain-read-model-only' &&
    row.portalApprovalUiClaim === 'not-implemented' &&
    row.portalReportUiClaim === 'not-implemented' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function approvalReportDomainProofIsHonest(proof: AppInstallPurchaseApprovalReportDomainProof): boolean {
  const states = new Set(proof.approvalReportDomainRows.map((row) => row.approvalReportDomainState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentReviewActionProofVersion === SourceParentReviewActionProofVersion &&
    proof.sourceReportRuntimeProofVersion === SourceReportRuntimeProofVersion &&
    proof.approvalReportDomainRows.length ===
      AppInstallPurchaseParentReviewActionProofReadModel.parentReviewActionRows.length &&
    states.has('approval-report-ready') &&
    states.has('approval-report-manual-review') &&
    !states.has('approval-report-unavailable') &&
    RequiredApprovalReportNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.approvalReportDomainRows.every(approvalReportDomainRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function approvalReportDomainBoundaryIsExplicit(
  boundary: typeof ApprovalReportDomainClaimBoundarySchema.Type
): boolean {
  return ApprovalReportBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

