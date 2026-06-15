import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentActionRuntimeHandoffProofReadModel } from './app-install-purchase-parent-action-runtime-handoff-proof';
import { AppInstallPurchaseStoreStatusHandoffProofReadModel } from './app-install-purchase-store-status-handoff-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const RuntimeWriterDeliveryText = Schema.String.pipe(Schema.minLength(1));
const RuntimeWriterDeliveryProofVersion = 'app-install-purchase-runtime-writer-delivery-proof';
const SourceParentActionRuntimeHandoffProofVersion = 'app-install-purchase-parent-action-runtime-handoff-proof';
const SourceStoreStatusHandoffProofVersion = 'app-install-purchase-store-status-handoff-proof';
const RuntimeWriterDeliveryTimestamp = '2026-06-05T08:50:00.000Z';
const RuntimeWriterDeliveryClaimBoundary =
  'runtime writer delivery proof only; no runtime writer implementation no runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredDecisionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredRuntimeWriterDeliveryStates = ['writer-envelope-ready', 'manual-review-required'] as const;
const RequiredSourceRuntimeHandoffStatuses = ['queued-for-runtime-writer', 'manual-review-required'] as const;
const RequiredStoreStatusHandoffStates = [
  'approved-api-status-proof-required',
  'store-entitlement-status-proof-required',
  'manual-platform-status-review-required',
  'platform-store-status-unavailable',
] as const;
const RuntimeWriterDeliveryNonClaims = [
  'no-runtime-writer-implementation',
  'no-runtime-writer-delivery',
  'no-parent-action-runtime-delivery',
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-child-activity-data',
  'no-app-blocking',
  'no-ocentra-hosted-family-data-custody',
] as const;
const RuntimeWriterDeliveryBoundaryFragments = [
  'no runtime writer implementation',
  'no runtime writer delivery',
  'no parent action runtime delivery',
  'no provider API execution',
  'no store integration',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no child activity data',
  'no app blocking',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseRuntimeWriterDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(RuntimeWriterDeliveryProofVersion)
);
const AppInstallPurchaseRuntimeWriterDeliveryActionSchema = withParser(Schema.Literal(...RequiredDecisionActions));
const AppInstallPurchaseRuntimeWriterDeliveryStateSchema = withParser(
  Schema.Literal(...RequiredRuntimeWriterDeliveryStates)
);
const AppInstallPurchaseRuntimeWriterSourceHandoffStatusSchema = withParser(
  Schema.Literal(...RequiredSourceRuntimeHandoffStatuses)
);
const AppInstallPurchaseRuntimeWriterStoreStatusSchema = withParser(
  Schema.Literal(...RequiredStoreStatusHandoffStates)
);
const AppInstallPurchaseRuntimeWriterQueueStateSchema = withParser(
  Schema.Literal('not-implemented', 'manual-required')
);
const AppInstallPurchaseRuntimeWriterImplementationClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseRuntimeWriterDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseRuntimeWriterProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseRuntimeWriterIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeWriterAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseRuntimeWriterInterceptionClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeWriterBlockingClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeWriterCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const AppInstallPurchaseRuntimeWriterHostedCustodyClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseRuntimeWriterNonClaimSchema = withParser(Schema.Literal(...RuntimeWriterDeliveryNonClaims));

const RuntimeWriterDeliveryRowIdSchema = RuntimeWriterDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeWriterDeliveryRowId')
);
const RuntimeWriterDeliveryRefSchema = RuntimeWriterDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeWriterDeliveryRef')
);
const RuntimeWriterDeliveryAuditRefSchema = RuntimeWriterDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeWriterDeliveryAuditRef')
);
const RuntimeWriterDeliveryClaimBoundarySchema = RuntimeWriterDeliveryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeWriterDeliveryClaimBoundary')
);

const RuntimeWriterDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeWriterDeliveryProofSchemaVersionSchema,
  runtimeWriterDeliveryRowId: RuntimeWriterDeliveryRowIdSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceParentActionRuntimeHandoffRowId: RuntimeWriterDeliveryRefSchema,
  sourceDecisionAction: AppInstallPurchaseRuntimeWriterDeliveryActionSchema,
  sourceRuntimeHandoffStatus: AppInstallPurchaseRuntimeWriterSourceHandoffStatusSchema,
  sourceStoreStatusHandoffProofVersion: Schema.Literal(SourceStoreStatusHandoffProofVersion),
  sourceStoreStatusHandoffRefs: Schema.Array(RuntimeWriterDeliveryRefSchema),
  sourceStoreStatusHandoffStates: Schema.Array(AppInstallPurchaseRuntimeWriterStoreStatusSchema),
  storeStatusHandoffEvidenceRefs: Schema.Array(RuntimeWriterDeliveryRefSchema),
  auditEventRefs: Schema.Array(RuntimeWriterDeliveryAuditRefSchema),
  reportRuntimeRefs: Schema.Array(RuntimeWriterDeliveryRefSchema),
  runtimeWriterDeliveryState: AppInstallPurchaseRuntimeWriterDeliveryStateSchema,
  runtimeWriterQueueState: AppInstallPurchaseRuntimeWriterQueueStateSchema,
  runtimeWriterImplementationClaim: AppInstallPurchaseRuntimeWriterImplementationClaimSchema,
  runtimeWriterDeliveryClaim: AppInstallPurchaseRuntimeWriterDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseRuntimeWriterDeliveryClaimSchema,
  storeStatusHandoffDeliveryClaim: AppInstallPurchaseRuntimeWriterDeliveryClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseRuntimeWriterProviderClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseRuntimeWriterIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseRuntimeWriterAdapterClaimSchema,
  childDeliveryClaim: AppInstallPurchaseRuntimeWriterDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseRuntimeWriterDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseRuntimeWriterInterceptionClaimSchema,
  appBlockingClaim: AppInstallPurchaseRuntimeWriterBlockingClaimSchema,
  childDataCustody: AppInstallPurchaseRuntimeWriterCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseRuntimeWriterHostedCustodyClaimSchema,
  claimBoundary: RuntimeWriterDeliveryClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type RuntimeWriterDeliveryRowCandidate = Infer<typeof RuntimeWriterDeliveryRowBaseSchema>;

export const AppInstallPurchaseRuntimeWriterDeliveryRowSchema = withParser(
  RuntimeWriterDeliveryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        runtimeWriterDeliveryRowIsHonest(row) ||
        'Expected runtime writer delivery rows to link parent action and store status handoffs without writer, delivery, provider, adapter, custody, interception, or blocking claims'
    )
  )
);

const RuntimeWriterDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeWriterDeliveryProofSchemaVersionSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceStoreStatusHandoffProofVersion: Schema.Literal(SourceStoreStatusHandoffProofVersion),
  runtimeWriterDeliveryRows: Schema.Array(AppInstallPurchaseRuntimeWriterDeliveryRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseRuntimeWriterNonClaimSchema),
  knownGaps: Schema.Array(RuntimeWriterDeliveryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeWriterDeliveryProof = Infer<typeof RuntimeWriterDeliveryProofBaseSchema>;

export const AppInstallPurchaseRuntimeWriterDeliveryProofSchema = withParser(
  RuntimeWriterDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeWriterDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase runtime writer delivery proof to cover review actions and preserve runtime writer delivery non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeWriterDeliveryKnownGaps = [
  'Runtime writer delivery rows are contract/readiness proof only; no runtime writer queue or delivery worker is implemented.',
  'Parent action runtime delivery, provider/store execution, platform adapters, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real runtime writer/delivery path exist.',
] as const;

export const AppInstallPurchaseRuntimeWriterDeliveryProofReadModel =
  AppInstallPurchaseRuntimeWriterDeliveryProofSchema.parse({
    schemaVersion: RuntimeWriterDeliveryProofVersion,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    sourceStoreStatusHandoffProofVersion: SourceStoreStatusHandoffProofVersion,
    runtimeWriterDeliveryRows:
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.map(runtimeWriterDeliveryRow),
    nonClaims: RuntimeWriterDeliveryNonClaims,
    knownGaps: AppInstallPurchaseRuntimeWriterDeliveryKnownGaps,
    updatedAt: RuntimeWriterDeliveryTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeWriterDeliveryProof(
  proof: AppInstallPurchaseRuntimeWriterDeliveryProof
) {
  return {
    runtimeWriterDeliveryRows: proof.runtimeWriterDeliveryRows.length,
    writerEnvelopeReadyRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.runtimeWriterDeliveryState === 'writer-envelope-ready'
    ).length,
    manualReviewRequiredRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.runtimeWriterDeliveryState === 'manual-review-required'
    ).length,
    storeStatusLinkedRows: proof.runtimeWriterDeliveryRows.filter(storeStatusCoverageIsComplete).length,
    writerImplementedRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.runtimeWriterImplementationClaim !== 'not-implemented'
    ).length,
    runtimeDeliveredRows: proof.runtimeWriterDeliveryRows.filter(
      (row) => row.parentActionRuntimeDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function runtimeWriterDeliveryRow(
  row: (typeof AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows)[number]
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: RuntimeWriterDeliveryProofVersion,
    runtimeWriterDeliveryRowId: `runtime-writer-delivery-${row.sourceDecisionAction}`,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    sourceParentActionRuntimeHandoffRowId: row.runtimeHandoffRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeHandoffStatus: row.runtimeHandoffStatus,
    sourceStoreStatusHandoffProofVersion: SourceStoreStatusHandoffProofVersion,
    sourceStoreStatusHandoffRefs: AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.map(
      (storeRow) => storeRow.storeStatusHandoffRowId
    ),
    sourceStoreStatusHandoffStates: AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.map(
      (storeRow) => storeRow.storeStatusHandoffState
    ),
    storeStatusHandoffEvidenceRefs: uniqueStoreStatusEvidenceRefs(),
    auditEventRefs: row.auditEventRefs,
    reportRuntimeRefs: uniqueReportRuntimeRefs(row.reportRuntimeRefs),
    runtimeWriterDeliveryState: manual ? 'manual-review-required' : 'writer-envelope-ready',
    runtimeWriterQueueState: manual ? 'manual-required' : 'not-implemented',
    runtimeWriterImplementationClaim: 'not-implemented',
    runtimeWriterDeliveryClaim: 'not-delivered',
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    storeStatusHandoffDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: 'not-implemented',
    childDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: RuntimeWriterDeliveryClaimBoundary,
    linkedAt: RuntimeWriterDeliveryTimestamp,
  } as const;
}

function uniqueStoreStatusEvidenceRefs() {
  return Array.from(
    new Set(
      AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.flatMap(
        (storeRow) => storeRow.storeStatusHandoffEvidenceRefs
      )
    )
  );
}

function uniqueReportRuntimeRefs(parentReportRefs: readonly string[]) {
  return Array.from(
    new Set([
      ...parentReportRefs,
      ...AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.flatMap(
        (storeRow) => storeRow.sourceReportRuntimeRefs
      ),
    ])
  );
}

function runtimeWriterDeliveryRowIsHonest(row: RuntimeWriterDeliveryRowCandidate): boolean {
  return (
    runtimeWriterDeliveryMatchesParentAction(row) &&
    storeStatusCoverageIsComplete(row) &&
    runtimeWriterDeliveryEvidenceIsComplete(row) &&
    runtimeWriterDeliveryClaimsStayUnimplemented(row) &&
    runtimeWriterDeliveryBoundaryIsExplicit(row.claimBoundary)
  );
}

function runtimeWriterDeliveryMatchesParentAction(row: RuntimeWriterDeliveryRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.sourceRuntimeHandoffStatus === 'manual-review-required' &&
      row.runtimeWriterDeliveryState === 'manual-review-required' &&
      row.runtimeWriterQueueState === 'manual-required'
    );
  }
  return (
    row.sourceRuntimeHandoffStatus === 'queued-for-runtime-writer' &&
    row.runtimeWriterDeliveryState === 'writer-envelope-ready' &&
    row.runtimeWriterQueueState === 'not-implemented'
  );
}

function storeStatusCoverageIsComplete(row: RuntimeWriterDeliveryRowCandidate): boolean {
  const storeStates = new Set(row.sourceStoreStatusHandoffStates);
  return (
    row.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    row.sourceStoreStatusHandoffRefs.length ===
      AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.length &&
    row.sourceStoreStatusHandoffStates.length ===
      AppInstallPurchaseStoreStatusHandoffProofReadModel.storeStatusHandoffRows.length &&
    RequiredStoreStatusHandoffStates.every((state) => storeStates.has(state))
  );
}

function runtimeWriterDeliveryEvidenceIsComplete(row: RuntimeWriterDeliveryRowCandidate): boolean {
  return (
    row.sourceParentActionRuntimeHandoffProofVersion === SourceParentActionRuntimeHandoffProofVersion &&
    row.sourceParentActionRuntimeHandoffRowId.length > 0 &&
    row.storeStatusHandoffEvidenceRefs.length >= 4 &&
    row.auditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function runtimeWriterDeliveryClaimsStayUnimplemented(row: RuntimeWriterDeliveryRowCandidate): boolean {
  return (
    runtimeWriterDeliveryExecutionClaimsStayUnimplemented(row) &&
    runtimeWriterDeliveryProductClaimsStayUnimplemented(row)
  );
}

function runtimeWriterDeliveryExecutionClaimsStayUnimplemented(row: RuntimeWriterDeliveryRowCandidate): boolean {
  return (
    row.runtimeWriterImplementationClaim === 'not-implemented' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.storeStatusHandoffDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed'
  );
}

function runtimeWriterDeliveryProductClaimsStayUnimplemented(row: RuntimeWriterDeliveryRowCandidate): boolean {
  return (
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function runtimeWriterDeliveryProofIsHonest(proof: AppInstallPurchaseRuntimeWriterDeliveryProof): boolean {
  const actions = new Set(proof.runtimeWriterDeliveryRows.map((row) => row.sourceDecisionAction));
  const sourceStatuses = new Set(proof.runtimeWriterDeliveryRows.map((row) => row.sourceRuntimeHandoffStatus));
  const deliveryStates = new Set(proof.runtimeWriterDeliveryRows.map((row) => row.runtimeWriterDeliveryState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentActionRuntimeHandoffProofVersion === SourceParentActionRuntimeHandoffProofVersion &&
    proof.sourceStoreStatusHandoffProofVersion === SourceStoreStatusHandoffProofVersion &&
    proof.runtimeWriterDeliveryRows.length === RequiredDecisionActions.length &&
    RequiredDecisionActions.every((action) => actions.has(action)) &&
    RequiredSourceRuntimeHandoffStatuses.every((status) => sourceStatuses.has(status)) &&
    RequiredRuntimeWriterDeliveryStates.every((state) => deliveryStates.has(state)) &&
    RuntimeWriterDeliveryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.runtimeWriterDeliveryRows.every((row) => runtimeWriterDeliveryRowIsHonest(row)) &&
    proof.knownGaps.length > 0
  );
}

function runtimeWriterDeliveryBoundaryIsExplicit(boundary: typeof RuntimeWriterDeliveryClaimBoundarySchema.Type) {
  return RuntimeWriterDeliveryBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
