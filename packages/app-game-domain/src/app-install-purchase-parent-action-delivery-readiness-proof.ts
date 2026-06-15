import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel } from './app-install-purchase-child-device-delivery-runtime-writer-proof';
import { AppInstallPurchaseParentActionRuntimeHandoffProofReadModel } from './app-install-purchase-parent-action-runtime-handoff-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ParentActionDeliveryReadinessText = Schema.String.pipe(Schema.minLength(1));
const ParentActionDeliveryReadinessProofVersion = 'app-install-purchase-parent-action-delivery-readiness-proof';
const SourceParentActionRuntimeHandoffProofVersion = 'app-install-purchase-parent-action-runtime-handoff-proof';
const SourceChildDeviceDeliveryRuntimeWriterProofVersion =
  'app-install-purchase-child-device-delivery-runtime-writer-proof';
const ParentActionDeliveryReadinessTimestamp = '2026-06-05T16:35:00.000Z';
const ParentActionDeliveryReadinessClaimBoundary =
  'parent action delivery readiness proof only; no parent action runtime delivery no runtime writer execution no runtime writer delivery no provider API execution no store integration no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no child activity data no app blocking no Ocentra-hosted family data custody';
const RequiredDecisionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RequiredRuntimeHandoffStatuses = ['queued-for-runtime-writer', 'manual-review-required'] as const;
const RequiredChildDeliveryEnvelopeStates = ['child-delivery-envelope-ready', 'manual-review-required'] as const;
const ParentActionDeliveryReadinessStates = ['parent-action-delivery-ready', 'manual-review-required'] as const;
const ParentActionDeliveryReadinessNonClaims = [
  'no-parent-action-runtime-delivery',
  'no-runtime-writer-execution',
  'no-runtime-writer-delivery',
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
const ParentActionDeliveryReadinessBoundaryFragments = [
  'no parent action runtime delivery',
  'no runtime writer execution',
  'no runtime writer delivery',
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

export const AppInstallPurchaseParentActionDeliveryReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ParentActionDeliveryReadinessProofVersion)
);
const AppInstallPurchaseParentActionDeliveryReadinessActionSchema = withParser(
  Schema.Literal(...RequiredDecisionActions)
);
const AppInstallPurchaseParentActionDeliveryReadinessRuntimeStatusSchema = withParser(
  Schema.Literal(...RequiredRuntimeHandoffStatuses)
);
const AppInstallPurchaseParentActionDeliveryReadinessChildEnvelopeSchema = withParser(
  Schema.Literal(...RequiredChildDeliveryEnvelopeStates)
);
const AppInstallPurchaseParentActionDeliveryReadinessStateSchema = withParser(
  Schema.Literal(...ParentActionDeliveryReadinessStates)
);
const AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const AppInstallPurchaseParentActionDeliveryReadinessExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const AppInstallPurchaseParentActionDeliveryReadinessAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const AppInstallPurchaseParentActionDeliveryReadinessCustodyClaimSchema = withParser(
  Schema.Literal('no-child-activity-data')
);
const AppInstallPurchaseParentActionDeliveryReadinessNonClaimSchema = withParser(
  Schema.Literal(...ParentActionDeliveryReadinessNonClaims)
);

const ParentActionDeliveryReadinessRowIdSchema = ParentActionDeliveryReadinessText.pipe(
  Schema.brand('AppInstallPurchaseParentActionDeliveryReadinessRowId')
);
const ParentActionDeliveryReadinessRefSchema = ParentActionDeliveryReadinessText.pipe(
  Schema.brand('AppInstallPurchaseParentActionDeliveryReadinessRef')
);
const ParentActionDeliveryReadinessAuditRefSchema = ParentActionDeliveryReadinessText.pipe(
  Schema.brand('AppInstallPurchaseParentActionDeliveryReadinessAuditRef')
);
const ParentActionDeliveryReadinessClaimBoundarySchema = ParentActionDeliveryReadinessText.pipe(
  Schema.brand('AppInstallPurchaseParentActionDeliveryReadinessClaimBoundary')
);

const ParentActionDeliveryReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentActionDeliveryReadinessProofSchemaVersionSchema,
  parentActionDeliveryReadinessRowId: ParentActionDeliveryReadinessRowIdSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceParentActionRuntimeHandoffRowId: ParentActionDeliveryReadinessRefSchema,
  sourceDecisionAction: AppInstallPurchaseParentActionDeliveryReadinessActionSchema,
  sourceRuntimeHandoffStatus: AppInstallPurchaseParentActionDeliveryReadinessRuntimeStatusSchema,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  sourceChildDeviceDeliveryRuntimeWriterRowId: ParentActionDeliveryReadinessRefSchema,
  sourceChildDeliveryEnvelopeState: AppInstallPurchaseParentActionDeliveryReadinessChildEnvelopeSchema,
  parentActionDeliveryReadinessState: AppInstallPurchaseParentActionDeliveryReadinessStateSchema,
  parentActionAuditEventRefs: Schema.Array(ParentActionDeliveryReadinessAuditRefSchema),
  childDeliveryTargetRefs: Schema.Array(ParentActionDeliveryReadinessRefSchema),
  reportRuntimeRefs: Schema.Array(ParentActionDeliveryReadinessRefSchema),
  parentActionRuntimeDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  runtimeWriterExecutionClaim: AppInstallPurchaseParentActionDeliveryReadinessExecutionClaimSchema,
  runtimeWriterDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  providerApiExecutionClaim: AppInstallPurchaseParentActionDeliveryReadinessExecutionClaimSchema,
  storeIntegrationClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  platformAdapterClaim: AppInstallPurchaseParentActionDeliveryReadinessAdapterClaimSchema,
  childDeviceDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  runtimeReportDeliveryClaim: AppInstallPurchaseParentActionDeliveryReadinessDeliveryClaimSchema,
  interceptionClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  appBlockingClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  childDataCustody: AppInstallPurchaseParentActionDeliveryReadinessCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: AppInstallPurchaseParentActionDeliveryReadinessIntegrationClaimSchema,
  claimBoundary: ParentActionDeliveryReadinessClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ParentActionDeliveryReadinessRowCandidate = Infer<typeof ParentActionDeliveryReadinessRowBaseSchema>;

export const AppInstallPurchaseParentActionDeliveryReadinessRowSchema = withParser(
  ParentActionDeliveryReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        parentActionDeliveryReadinessRowIsHonest(row) ||
        'Expected parent action delivery readiness rows to link parent handoff and child envelope refs without delivery, writer, provider, adapter, custody, interception, or blocking claims'
    )
  )
);

const ParentActionDeliveryReadinessProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseParentActionDeliveryReadinessProofSchemaVersionSchema,
  sourceParentActionRuntimeHandoffProofVersion: Schema.Literal(SourceParentActionRuntimeHandoffProofVersion),
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  parentActionDeliveryReadinessRows: Schema.Array(AppInstallPurchaseParentActionDeliveryReadinessRowSchema),
  nonClaims: Schema.Array(AppInstallPurchaseParentActionDeliveryReadinessNonClaimSchema),
  knownGaps: Schema.Array(ParentActionDeliveryReadinessRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseParentActionDeliveryReadinessProof = Infer<
  typeof ParentActionDeliveryReadinessProofBaseSchema
>;

export const AppInstallPurchaseParentActionDeliveryReadinessProofSchema = withParser(
  ParentActionDeliveryReadinessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        parentActionDeliveryReadinessProofIsHonest(proof) ||
        'Expected app install/purchase parent action delivery readiness proof to cover review actions and preserve delivery non-claims'
    )
  )
);

export const AppInstallPurchaseParentActionDeliveryReadinessKnownGaps = [
  'Parent action delivery readiness rows are contract/proof rows only; no parent action runtime delivery worker is implemented.',
  'Runtime writer execution/delivery, provider/store execution, platform adapters, child-device delivery, runtime report delivery, interception, app blocking, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real parent action delivery runtime path exist.',
] as const;

export const AppInstallPurchaseParentActionDeliveryReadinessProofReadModel =
  AppInstallPurchaseParentActionDeliveryReadinessProofSchema.parse({
    schemaVersion: ParentActionDeliveryReadinessProofVersion,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion: SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    parentActionDeliveryReadinessRows:
      AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows.map(
        parentActionDeliveryReadinessRow
      ),
    nonClaims: ParentActionDeliveryReadinessNonClaims,
    knownGaps: AppInstallPurchaseParentActionDeliveryReadinessKnownGaps,
    updatedAt: ParentActionDeliveryReadinessTimestamp,
  });

export function summarizeAppInstallPurchaseParentActionDeliveryReadinessProof(
  proof: AppInstallPurchaseParentActionDeliveryReadinessProof
) {
  return {
    parentActionDeliveryReadinessRows: proof.parentActionDeliveryReadinessRows.length,
    parentActionDeliveryReadyRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.parentActionDeliveryReadinessState === 'parent-action-delivery-ready'
    ).length,
    manualReviewRequiredRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.parentActionDeliveryReadinessState === 'manual-review-required'
    ).length,
    childEnvelopeLinkedRows: proof.parentActionDeliveryReadinessRows.filter(childEnvelopeCoverageIsComplete).length,
    parentActionDeliveredRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.parentActionRuntimeDeliveryClaim !== 'not-delivered'
    ).length,
    runtimeWriterExecutedRows: proof.parentActionDeliveryReadinessRows.filter(
      (row) => row.runtimeWriterExecutionClaim !== 'not-executed'
    ).length,
  } as const;
}

function parentActionDeliveryReadinessRow(
  row: (typeof AppInstallPurchaseParentActionRuntimeHandoffProofReadModel.runtimeHandoffRows)[number]
) {
  const childEnvelopeRow = childDeliveryRowForAction(row.sourceDecisionAction);
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: ParentActionDeliveryReadinessProofVersion,
    parentActionDeliveryReadinessRowId: `parent-action-delivery-readiness-${row.sourceDecisionAction}`,
    sourceParentActionRuntimeHandoffProofVersion: SourceParentActionRuntimeHandoffProofVersion,
    sourceParentActionRuntimeHandoffRowId: row.runtimeHandoffRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeHandoffStatus: row.runtimeHandoffStatus,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion: SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterRowId: childEnvelopeRow.childDeviceDeliveryRuntimeWriterRowId,
    sourceChildDeliveryEnvelopeState: childEnvelopeRow.childDeliveryEnvelopeState,
    parentActionDeliveryReadinessState: manual ? 'manual-review-required' : 'parent-action-delivery-ready',
    parentActionAuditEventRefs: row.auditEventRefs,
    childDeliveryTargetRefs: childEnvelopeRow.childDeliveryTargetRefs,
    reportRuntimeRefs: uniqueRefs([...row.reportRuntimeRefs, ...childEnvelopeRow.reportRuntimeRefs]),
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    runtimeWriterExecutionClaim: childEnvelopeRow.runtimeWriterExecutionClaim,
    runtimeWriterDeliveryClaim: childEnvelopeRow.runtimeWriterDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformAdapterClaim: childEnvelopeRow.platformAdapterClaim,
    childDeviceDeliveryClaim: childEnvelopeRow.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: childEnvelopeRow.runtimeReportDeliveryClaim,
    interceptionClaim: row.interceptionClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: ParentActionDeliveryReadinessClaimBoundary,
    linkedAt: ParentActionDeliveryReadinessTimestamp,
  } as const;
}

function childDeliveryRowForAction(action: (typeof RequiredDecisionActions)[number]) {
  return AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel.childDeviceDeliveryRuntimeWriterRows.find(
    (row) => row.sourceDecisionAction === action
  )!;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function parentActionDeliveryReadinessRowIsHonest(row: ParentActionDeliveryReadinessRowCandidate): boolean {
  return (
    parentActionDeliveryReadinessMatchesSourceState(row) &&
    childEnvelopeCoverageIsComplete(row) &&
    parentActionDeliveryReadinessRefsAreComplete(row) &&
    parentActionDeliveryReadinessClaimsStayUnimplemented(row) &&
    parentActionDeliveryReadinessBoundaryIsExplicit(row.claimBoundary)
  );
}

function parentActionDeliveryReadinessMatchesSourceState(row: ParentActionDeliveryReadinessRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.sourceRuntimeHandoffStatus === 'manual-review-required' &&
      row.sourceChildDeliveryEnvelopeState === 'manual-review-required' &&
      row.parentActionDeliveryReadinessState === 'manual-review-required'
    );
  }
  return (
    row.sourceRuntimeHandoffStatus === 'queued-for-runtime-writer' &&
    row.sourceChildDeliveryEnvelopeState === 'child-delivery-envelope-ready' &&
    row.parentActionDeliveryReadinessState === 'parent-action-delivery-ready'
  );
}

function childEnvelopeCoverageIsComplete(row: ParentActionDeliveryReadinessRowCandidate): boolean {
  return (
    row.sourceChildDeviceDeliveryRuntimeWriterProofVersion === SourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    row.sourceChildDeviceDeliveryRuntimeWriterRowId.length > 0 &&
    row.childDeliveryTargetRefs.length > 0
  );
}

function parentActionDeliveryReadinessRefsAreComplete(row: ParentActionDeliveryReadinessRowCandidate): boolean {
  return (
    row.sourceParentActionRuntimeHandoffProofVersion === SourceParentActionRuntimeHandoffProofVersion &&
    row.sourceParentActionRuntimeHandoffRowId.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function parentActionDeliveryReadinessClaimsStayUnimplemented(row: ParentActionDeliveryReadinessRowCandidate): boolean {
  return (
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.runtimeWriterExecutionClaim === 'not-executed' &&
    row.runtimeWriterDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.interceptionClaim === 'not-claimed' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function parentActionDeliveryReadinessProofIsHonest(
  proof: AppInstallPurchaseParentActionDeliveryReadinessProof
): boolean {
  const actions = new Set(proof.parentActionDeliveryReadinessRows.map((row) => row.sourceDecisionAction));
  const sourceStatuses = new Set(proof.parentActionDeliveryReadinessRows.map((row) => row.sourceRuntimeHandoffStatus));
  const readinessStates = new Set(
    proof.parentActionDeliveryReadinessRows.map((row) => row.parentActionDeliveryReadinessState)
  );
  const childEnvelopeStates = new Set(
    proof.parentActionDeliveryReadinessRows.map((row) => row.sourceChildDeliveryEnvelopeState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceParentActionRuntimeHandoffProofVersion === SourceParentActionRuntimeHandoffProofVersion &&
    proof.sourceChildDeviceDeliveryRuntimeWriterProofVersion === SourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    proof.parentActionDeliveryReadinessRows.length === RequiredDecisionActions.length &&
    RequiredDecisionActions.every((action) => actions.has(action)) &&
    RequiredRuntimeHandoffStatuses.every((status) => sourceStatuses.has(status)) &&
    ParentActionDeliveryReadinessStates.every((state) => readinessStates.has(state)) &&
    RequiredChildDeliveryEnvelopeStates.every((state) => childEnvelopeStates.has(state)) &&
    ParentActionDeliveryReadinessNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.parentActionDeliveryReadinessRows.every((row) => parentActionDeliveryReadinessRowIsHonest(row)) &&
    proof.knownGaps.length > 0
  );
}

function parentActionDeliveryReadinessBoundaryIsExplicit(
  boundary: typeof ParentActionDeliveryReadinessClaimBoundarySchema.Type
) {
  return ParentActionDeliveryReadinessBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
