import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentActionDeliveryReadinessProofReadModel } from './app-install-purchase-parent-action-delivery-readiness-proof';
import { AppInstallPurchaseRuntimeWriterDeliveryProofReadModel } from './app-install-purchase-runtime-writer-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const RuntimeWriterExecutionDeliveryProofVersion = 'app-install-purchase-runtime-writer-execution-delivery-proof';
const SourceRuntimeWriterDeliveryProofVersion = 'app-install-purchase-runtime-writer-delivery-proof';
const SourceParentActionDeliveryReadinessProofVersion = 'app-install-purchase-parent-action-delivery-readiness-proof';
const RuntimeWriterExecutionDeliveryTimestamp = '2026-06-05T19:55:00.000Z';
const RuntimeWriterExecutionDeliveryBoundary =
  'runtime writer execution delivery proof only; parent-owned runtime writer envelope and delivery result receipt are recorded no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const RuntimeWriterExecutionDeliveryActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RuntimeWriterExecutionDeliveryStates = ['delivery-result-recorded', 'manual-required'] as const;
const RuntimeWriterEnvelopeStates = ['parent-owned-envelope-written', 'manual-required'] as const;
const RuntimeWriterDeliveryReceiptClaims = ['parent-owned-delivery-result-recorded', 'manual-required'] as const;
const RuntimeWriterExecutionDeliveryNonClaims = [
  'no-provider-api-execution',
  'no-store-integration',
  'no-platform-interception',
  'no-platform-adapter-implementation',
  'no-child-device-delivery',
  'no-runtime-report-delivery',
  'no-real-install-or-purchase-interception',
  'no-app-blocking',
  'no-child-activity-data',
  'no-ocentra-hosted-family-data-custody',
] as const;
const RuntimeWriterExecutionDeliveryBoundaryFragments = [
  'parent-owned runtime writer envelope',
  'delivery result receipt',
  'no provider API execution',
  'no store integration',
  'no platform interception',
  'no platform adapter implementation',
  'no child-device delivery',
  'no runtime report delivery',
  'no real install or purchase interception',
  'no app blocking',
  'no child activity data',
  'no Ocentra-hosted family data custody',
] as const;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(RuntimeWriterExecutionDeliveryProofVersion)
);
const RuntimeWriterExecutionDeliveryActionSchema = withParser(Schema.Literal(...RuntimeWriterExecutionDeliveryActions));
const RuntimeWriterExecutionDeliveryStateSchema = withParser(Schema.Literal(...RuntimeWriterExecutionDeliveryStates));
const RuntimeWriterEnvelopeStateSchema = withParser(Schema.Literal(...RuntimeWriterEnvelopeStates));
const RuntimeWriterExecutionDeliveryReceiptClaimSchema = withParser(
  Schema.Literal(...RuntimeWriterDeliveryReceiptClaims)
);
const RuntimeWriterExecutionDeliveryProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const RuntimeWriterExecutionDeliveryIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const RuntimeWriterExecutionDeliveryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const RuntimeWriterExecutionDeliveryDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const RuntimeWriterExecutionDeliveryCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const RuntimeWriterExecutionDeliveryNonClaimSchema = withParser(
  Schema.Literal(...RuntimeWriterExecutionDeliveryNonClaims)
);

const RuntimeWriterExecutionDeliveryRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeWriterExecutionDeliveryRowId');
const RuntimeWriterExecutionDeliveryRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeWriterExecutionDeliveryRef');
const RuntimeWriterExecutionDeliveryAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeWriterExecutionDeliveryAuditRef');
const RuntimeWriterExecutionDeliveryBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseRuntimeWriterExecutionDeliveryBoundary');

const RuntimeWriterExecutionDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchemaVersionSchema,
  runtimeWriterExecutionDeliveryRowId: RuntimeWriterExecutionDeliveryRowIdSchema,
  sourceRuntimeWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterDeliveryProofVersion),
  sourceRuntimeWriterDeliveryRowId: RuntimeWriterExecutionDeliveryRefSchema,
  sourceParentActionDeliveryReadinessProofVersion: Schema.Literal(SourceParentActionDeliveryReadinessProofVersion),
  sourceParentActionDeliveryReadinessRowId: RuntimeWriterExecutionDeliveryRefSchema,
  sourceDecisionAction: RuntimeWriterExecutionDeliveryActionSchema,
  runtimeWriterEnvelopeState: RuntimeWriterEnvelopeStateSchema,
  runtimeWriterEnvelopeRef: RuntimeWriterExecutionDeliveryRefSchema,
  runtimeWriterExecutionDeliveryState: RuntimeWriterExecutionDeliveryStateSchema,
  deliveryResultReceiptRef: RuntimeWriterExecutionDeliveryRefSchema,
  deliveryResultAuditEventRefs: Schema.Array(RuntimeWriterExecutionDeliveryAuditRefSchema),
  parentActionAuditEventRefs: Schema.Array(RuntimeWriterExecutionDeliveryAuditRefSchema),
  reportRuntimeRefs: Schema.Array(RuntimeWriterExecutionDeliveryRefSchema),
  runtimeWriterExecutionClaim: RuntimeWriterExecutionDeliveryReceiptClaimSchema,
  runtimeWriterDeliveryClaim: RuntimeWriterExecutionDeliveryReceiptClaimSchema,
  parentActionRuntimeDeliveryClaim: RuntimeWriterExecutionDeliveryReceiptClaimSchema,
  providerApiExecutionClaim: RuntimeWriterExecutionDeliveryProviderClaimSchema,
  storeIntegrationClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  platformInterceptionClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  platformAdapterClaim: RuntimeWriterExecutionDeliveryAdapterClaimSchema,
  childDeviceDeliveryClaim: RuntimeWriterExecutionDeliveryDeliveryClaimSchema,
  runtimeReportDeliveryClaim: RuntimeWriterExecutionDeliveryDeliveryClaimSchema,
  appBlockingClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  childDataCustody: RuntimeWriterExecutionDeliveryCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  claimBoundary: RuntimeWriterExecutionDeliveryBoundarySchema,
  recordedAt: ParentTimestampSchema,
});

type RuntimeWriterExecutionDeliveryRowCandidate = Infer<typeof RuntimeWriterExecutionDeliveryRowBaseSchema>;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema = withParser(
  RuntimeWriterExecutionDeliveryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        runtimeWriterExecutionDeliveryRowIsHonest(row) ||
        'Expected runtime writer execution delivery rows to record parent-owned envelopes and receipts without provider, store, platform, child-device, custody, report delivery, or blocking claims'
    )
  )
);

const RuntimeWriterExecutionDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchemaVersionSchema,
  sourceRuntimeWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterDeliveryProofVersion),
  sourceParentActionDeliveryReadinessProofVersion: Schema.Literal(SourceParentActionDeliveryReadinessProofVersion),
  runtimeWriterExecutionDeliveryRows: Schema.Array(AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema),
  nonClaims: Schema.Array(RuntimeWriterExecutionDeliveryNonClaimSchema),
  knownGaps: Schema.Array(RuntimeWriterExecutionDeliveryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeWriterExecutionDeliveryProof = Infer<
  typeof RuntimeWriterExecutionDeliveryProofBaseSchema
>;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema = withParser(
  RuntimeWriterExecutionDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeWriterExecutionDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase runtime writer execution delivery proof to cover parent actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryKnownGaps = [
  'Runtime writer execution delivery rows record deterministic parent-owned envelopes and delivery result receipts only.',
  'Provider/store execution, platform interception/adapters, child-device delivery, runtime report delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real parent approval action path exist.',
] as const;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel =
  AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema.parse({
    schemaVersion: RuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterDeliveryProofVersion: SourceRuntimeWriterDeliveryProofVersion,
    sourceParentActionDeliveryReadinessProofVersion: SourceParentActionDeliveryReadinessProofVersion,
    runtimeWriterExecutionDeliveryRows:
      AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows.map(
        runtimeWriterExecutionDeliveryRow
      ),
    nonClaims: RuntimeWriterExecutionDeliveryNonClaims,
    knownGaps: AppInstallPurchaseRuntimeWriterExecutionDeliveryKnownGaps,
    updatedAt: RuntimeWriterExecutionDeliveryTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProof(
  proof: AppInstallPurchaseRuntimeWriterExecutionDeliveryProof
) {
  return {
    runtimeWriterExecutionDeliveryRows: proof.runtimeWriterExecutionDeliveryRows.length,
    parentOwnedEnvelopeRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.runtimeWriterEnvelopeState === 'parent-owned-envelope-written'
    ).length,
    deliveryResultReceiptRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.runtimeWriterExecutionDeliveryState === 'delivery-result-recorded'
    ).length,
    manualRequiredRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.runtimeWriterExecutionDeliveryState === 'manual-required'
    ).length,
    providerExecutedRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.providerApiExecutionClaim !== 'not-executed'
    ).length,
    childDeliveredRows: proof.runtimeWriterExecutionDeliveryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function runtimeWriterExecutionDeliveryRow(
  row: (typeof AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows)[number]
) {
  const readinessRow = parentActionDeliveryReadinessRowFor(row.sourceDecisionAction);
  const manual = row.sourceDecisionAction === 'review-needed';
  const actionReceiptClaim = manual ? 'manual-required' : 'parent-owned-delivery-result-recorded';
  return {
    schemaVersion: RuntimeWriterExecutionDeliveryProofVersion,
    runtimeWriterExecutionDeliveryRowId: `runtime-writer-execution-delivery-${row.sourceDecisionAction}`,
    sourceRuntimeWriterDeliveryProofVersion: SourceRuntimeWriterDeliveryProofVersion,
    sourceRuntimeWriterDeliveryRowId: row.runtimeWriterDeliveryRowId,
    sourceParentActionDeliveryReadinessProofVersion: SourceParentActionDeliveryReadinessProofVersion,
    sourceParentActionDeliveryReadinessRowId: readinessRow.parentActionDeliveryReadinessRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    runtimeWriterEnvelopeState: manual ? 'manual-required' : 'parent-owned-envelope-written',
    runtimeWriterEnvelopeRef: `parent-owned-runtime-writer-envelope-${row.sourceDecisionAction}`,
    runtimeWriterExecutionDeliveryState: manual ? 'manual-required' : 'delivery-result-recorded',
    deliveryResultReceiptRef: `parent-owned-runtime-writer-receipt-${row.sourceDecisionAction}`,
    deliveryResultAuditEventRefs: row.auditEventRefs,
    parentActionAuditEventRefs: readinessRow.parentActionAuditEventRefs,
    reportRuntimeRefs: uniqueRefs([...row.reportRuntimeRefs, ...readinessRow.reportRuntimeRefs]),
    runtimeWriterExecutionClaim: actionReceiptClaim,
    runtimeWriterDeliveryClaim: actionReceiptClaim,
    parentActionRuntimeDeliveryClaim: actionReceiptClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.interceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: RuntimeWriterExecutionDeliveryBoundary,
    recordedAt: RuntimeWriterExecutionDeliveryTimestamp,
  } as const;
}

function parentActionDeliveryReadinessRowFor(action: (typeof RuntimeWriterExecutionDeliveryActions)[number]) {
  return AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows.find(
    (row) => row.sourceDecisionAction === action
  )!;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function runtimeWriterExecutionDeliveryRowIsHonest(row: RuntimeWriterExecutionDeliveryRowCandidate): boolean {
  return (
    runtimeWriterExecutionDeliveryMatchesAction(row) &&
    runtimeWriterExecutionDeliveryRefsAreComplete(row) &&
    runtimeWriterExecutionDeliveryClaimsStayBounded(row) &&
    runtimeWriterExecutionDeliveryBoundaryIsExplicit(row.claimBoundary)
  );
}

function runtimeWriterExecutionDeliveryMatchesAction(row: RuntimeWriterExecutionDeliveryRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.runtimeWriterEnvelopeState === 'manual-required' &&
      row.runtimeWriterExecutionDeliveryState === 'manual-required' &&
      row.runtimeWriterExecutionClaim === 'manual-required' &&
      row.runtimeWriterDeliveryClaim === 'manual-required' &&
      row.parentActionRuntimeDeliveryClaim === 'manual-required'
    );
  }
  return (
    row.runtimeWriterEnvelopeState === 'parent-owned-envelope-written' &&
    row.runtimeWriterExecutionDeliveryState === 'delivery-result-recorded' &&
    row.runtimeWriterExecutionClaim === 'parent-owned-delivery-result-recorded' &&
    row.runtimeWriterDeliveryClaim === 'parent-owned-delivery-result-recorded' &&
    row.parentActionRuntimeDeliveryClaim === 'parent-owned-delivery-result-recorded'
  );
}

function runtimeWriterExecutionDeliveryRefsAreComplete(row: RuntimeWriterExecutionDeliveryRowCandidate): boolean {
  return (
    row.sourceRuntimeWriterDeliveryProofVersion === SourceRuntimeWriterDeliveryProofVersion &&
    row.sourceRuntimeWriterDeliveryRowId.length > 0 &&
    row.sourceParentActionDeliveryReadinessProofVersion === SourceParentActionDeliveryReadinessProofVersion &&
    row.sourceParentActionDeliveryReadinessRowId.length > 0 &&
    row.runtimeWriterEnvelopeRef.length > 0 &&
    row.deliveryResultReceiptRef.length > 0 &&
    row.deliveryResultAuditEventRefs.length > 0 &&
    row.parentActionAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function runtimeWriterExecutionDeliveryClaimsStayBounded(row: RuntimeWriterExecutionDeliveryRowCandidate): boolean {
  return (
    row.providerApiExecutionClaim === 'not-executed' &&
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered' &&
    row.appBlockingClaim === 'not-claimed' &&
    row.childDataCustody === 'no-child-activity-data' &&
    row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed'
  );
}

function runtimeWriterExecutionDeliveryProofIsHonest(
  proof: AppInstallPurchaseRuntimeWriterExecutionDeliveryProof
): boolean {
  const actions = new Set(proof.runtimeWriterExecutionDeliveryRows.map((row) => row.sourceDecisionAction));
  const envelopeStates = new Set(proof.runtimeWriterExecutionDeliveryRows.map((row) => row.runtimeWriterEnvelopeState));
  const deliveryStates = new Set(
    proof.runtimeWriterExecutionDeliveryRows.map((row) => row.runtimeWriterExecutionDeliveryState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterDeliveryProofVersion === SourceRuntimeWriterDeliveryProofVersion &&
    proof.sourceParentActionDeliveryReadinessProofVersion === SourceParentActionDeliveryReadinessProofVersion &&
    proof.runtimeWriterExecutionDeliveryRows.length === RuntimeWriterExecutionDeliveryActions.length &&
    RuntimeWriterExecutionDeliveryActions.every((action) => actions.has(action)) &&
    RuntimeWriterEnvelopeStates.every((state) => envelopeStates.has(state)) &&
    RuntimeWriterExecutionDeliveryStates.every((state) => deliveryStates.has(state)) &&
    RuntimeWriterExecutionDeliveryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.runtimeWriterExecutionDeliveryRows.every(runtimeWriterExecutionDeliveryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function runtimeWriterExecutionDeliveryBoundaryIsExplicit(
  boundary: typeof RuntimeWriterExecutionDeliveryBoundarySchema.Type
): boolean {
  return RuntimeWriterExecutionDeliveryBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

