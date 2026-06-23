import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel } from './app-install-purchase-runtime-delivery-receipt-boundary-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';

const RuntimeTransportDeliveryExecutionProofVersion = 'app-install-purchase-runtime-transport-delivery-execution-proof';
const SourceReceiptBoundaryProofVersion = 'app-install-purchase-runtime-delivery-receipt-boundary-proof';
const RuntimeTransportDeliveryExecutionTimestamp = '2026-06-08T02:05:00.000Z';
const RuntimeTransportDeliveryExecutionBoundary =
  'runtime transport delivery execution proof only; rows consume runtime delivery receipt boundary rows and keep parent-owned transport execution attempts withheld until external writer dispatch execution provider-store execution receipt platform adapter execution receipt and child-device transport receipt refs are real no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const RuntimeTransportDeliveryExecutionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceReceiptBoundaryStates = ['receipt-blocked-waiting-runtime-artifacts', 'manual-required'] as const;
const SourceReceiptExpectationStates = ['receipt-missing', 'manual-required'] as const;
const RuntimeTransportExecutionStates = ['execution-withheld-missing-artifacts', 'manual-required'] as const;
const RuntimeTransportAttemptStates = ['not-started', 'manual-required'] as const;
const RuntimeDeliveryResultStates = ['result-not-recorded', 'manual-required'] as const;
const ChildReceiptHandoffStates = ['receipt-handoff-missing', 'manual-required'] as const;
const RuntimeTransportDeliveryExecutionBlockers = [
  'external-writer-dispatch-execution-missing',
  'provider-store-execution-receipt-missing',
  'platform-adapter-execution-receipt-missing',
  'child-device-transport-receipt-missing',
] as const;
const RuntimeTransportDeliveryExecutionNonClaims = [
  'no-external-runtime-writer-execution',
  'no-external-runtime-writer-delivery',
  'no-parent-action-runtime-delivery',
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
const RuntimeTransportDeliveryExecutionBoundaryFragments = [
  'rows consume runtime delivery receipt boundary rows',
  'parent-owned transport execution attempts withheld',
  'external writer dispatch execution',
  'provider-store execution receipt',
  'platform adapter execution receipt',
  'child-device transport receipt',
  'no external runtime writer execution',
  'no external runtime writer delivery',
  'no parent action runtime delivery',
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

export const AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchemaVersionSchema = withParser(
  Schema.Literal(RuntimeTransportDeliveryExecutionProofVersion)
);
const RuntimeTransportDeliveryExecutionActionSchema = withParser(
  Schema.Literal(...RuntimeTransportDeliveryExecutionActions)
);
const SourceReceiptBoundaryStateSchema = withParser(Schema.Literal(...SourceReceiptBoundaryStates));
const SourceReceiptExpectationStateSchema = withParser(Schema.Literal(...SourceReceiptExpectationStates));
const RuntimeTransportExecutionStateSchema = withParser(Schema.Literal(...RuntimeTransportExecutionStates));
const RuntimeTransportAttemptStateSchema = withParser(Schema.Literal(...RuntimeTransportAttemptStates));
const RuntimeDeliveryResultStateSchema = withParser(Schema.Literal(...RuntimeDeliveryResultStates));
const ChildReceiptHandoffStateSchema = withParser(Schema.Literal(...ChildReceiptHandoffStates));
const RuntimeTransportDeliveryExecutionBlockerSchema = withParser(
  Schema.Literal(...RuntimeTransportDeliveryExecutionBlockers)
);
const RuntimeTransportDeliveryExecutionNonClaimSchema = withParser(
  Schema.Literal(...RuntimeTransportDeliveryExecutionNonClaims)
);
const RuntimeTransportDeliveryExecutionExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const RuntimeTransportDeliveryExecutionDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const RuntimeTransportDeliveryExecutionIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const RuntimeTransportDeliveryExecutionAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const RuntimeTransportDeliveryExecutionCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));

const RuntimeTransportDeliveryExecutionRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeTransportDeliveryExecutionRowId'
);
const RuntimeTransportDeliveryExecutionRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeTransportDeliveryExecutionRef'
);
const RuntimeTransportDeliveryExecutionAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeTransportDeliveryExecutionAuditRef'
);
const RuntimeTransportDeliveryExecutionBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeTransportDeliveryExecutionBoundary'
);

const RuntimeTransportDeliveryExecutionRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchemaVersionSchema,
  runtimeTransportDeliveryExecutionRowId: RuntimeTransportDeliveryExecutionRowIdSchema,
  sourceReceiptBoundaryProofVersion: Schema.Literal(SourceReceiptBoundaryProofVersion),
  sourceReceiptBoundaryRowId: RuntimeTransportDeliveryExecutionRefSchema,
  sourceDecisionAction: RuntimeTransportDeliveryExecutionActionSchema,
  sourceReceiptBoundaryState: SourceReceiptBoundaryStateSchema,
  sourceChildDeviceTransportReceiptState: SourceReceiptExpectationStateSchema,
  sourceParentOwnedDispatchPacketRef: RuntimeTransportDeliveryExecutionRefSchema,
  sourceParentOwnedReceiptBoundaryRef: RuntimeTransportDeliveryExecutionRefSchema,
  parentOwnedTransportExecutionAttemptRef: RuntimeTransportDeliveryExecutionRefSchema,
  parentOwnedDeliveryResultReceiptRef: RuntimeTransportDeliveryExecutionRefSchema,
  childDeviceReceiptHandoffRef: RuntimeTransportDeliveryExecutionRefSchema,
  runtimeTransportExecutionState: RuntimeTransportExecutionStateSchema,
  runtimeTransportAttemptState: RuntimeTransportAttemptStateSchema,
  runtimeDeliveryResultState: RuntimeDeliveryResultStateSchema,
  childDeviceReceiptHandoffState: ChildReceiptHandoffStateSchema,
  requiredRuntimeExecutionBlockers: Schema.Array(RuntimeTransportDeliveryExecutionBlockerSchema),
  externalWriterDispatchExecutionProofRefs: Schema.Array(RuntimeTransportDeliveryExecutionRefSchema),
  providerStoreExecutionReceiptProofRefs: Schema.Array(RuntimeTransportDeliveryExecutionRefSchema),
  platformAdapterExecutionReceiptProofRefs: Schema.Array(RuntimeTransportDeliveryExecutionRefSchema),
  childDeviceTransportReceiptProofRefs: Schema.Array(RuntimeTransportDeliveryExecutionRefSchema),
  executionWithheldReasonRefs: Schema.Array(RuntimeTransportDeliveryExecutionRefSchema),
  runtimeTransportDeliveryExecutionAuditEventRefs: Schema.Array(RuntimeTransportDeliveryExecutionAuditRefSchema),
  externalRuntimeWriterExecutionClaim: RuntimeTransportDeliveryExecutionExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: RuntimeTransportDeliveryExecutionDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: RuntimeTransportDeliveryExecutionDeliveryClaimSchema,
  providerApiExecutionClaim: RuntimeTransportDeliveryExecutionExecutionClaimSchema,
  storeIntegrationClaim: RuntimeTransportDeliveryExecutionIntegrationClaimSchema,
  platformInterceptionClaim: RuntimeTransportDeliveryExecutionIntegrationClaimSchema,
  platformAdapterClaim: RuntimeTransportDeliveryExecutionAdapterClaimSchema,
  childDeviceDeliveryClaim: RuntimeTransportDeliveryExecutionDeliveryClaimSchema,
  runtimeReportDeliveryClaim: RuntimeTransportDeliveryExecutionDeliveryClaimSchema,
  appBlockingClaim: RuntimeTransportDeliveryExecutionIntegrationClaimSchema,
  childDataCustody: RuntimeTransportDeliveryExecutionCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: RuntimeTransportDeliveryExecutionIntegrationClaimSchema,
  claimBoundary: RuntimeTransportDeliveryExecutionBoundarySchema,
  executionBoundaryCheckedAt: ParentTimestampSchema,
});

type RuntimeTransportDeliveryExecutionRowCandidate = Infer<typeof RuntimeTransportDeliveryExecutionRowBaseSchema>;

export const AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema = withParser(
  RuntimeTransportDeliveryExecutionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        runtimeTransportDeliveryExecutionRowIsHonest(row) ||
        'Expected runtime transport delivery execution rows to keep execution withheld until real transport receipt artifacts exist'
    )
  )
);

const RuntimeTransportDeliveryExecutionProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchemaVersionSchema,
  sourceReceiptBoundaryProofVersion: Schema.Literal(SourceReceiptBoundaryProofVersion),
  runtimeTransportDeliveryExecutionRows: Schema.Array(AppInstallPurchaseRuntimeTransportDeliveryExecutionRowSchema),
  nonClaims: Schema.Array(RuntimeTransportDeliveryExecutionNonClaimSchema),
  knownGaps: Schema.Array(RuntimeTransportDeliveryExecutionRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeTransportDeliveryExecutionProof = Infer<
  typeof RuntimeTransportDeliveryExecutionProofBaseSchema
>;

export const AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchema = withParser(
  RuntimeTransportDeliveryExecutionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeTransportDeliveryExecutionProofIsHonest(proof) ||
        'Expected app install/purchase runtime transport delivery execution proof to preserve withheld execution rows and non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeTransportDeliveryExecutionKnownGaps = [
  'Runtime transport delivery execution rows are parent-owned proof rows only; no external writer dispatch executor, provider/store execution receipt, platform adapter execution receipt, or child-device transport receipt exists.',
  'Execution attempts remain withheld or manual-required until external writer dispatch execution, provider/store execution receipt, platform adapter execution receipt, and child-device transport receipt proof refs become real artifacts.',
  'Product capability checklist Install/purchase approval row now records this transport execution proof as non-claim evidence while provider/store execution, platform adapter execution, and child-device transport receipt artifacts remain required.',
] as const;

export const AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel =
  AppInstallPurchaseRuntimeTransportDeliveryExecutionProofSchema.parse({
    schemaVersion: RuntimeTransportDeliveryExecutionProofVersion,
    sourceReceiptBoundaryProofVersion: SourceReceiptBoundaryProofVersion,
    runtimeTransportDeliveryExecutionRows:
      AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel.runtimeDeliveryReceiptBoundaryRows.map(
        runtimeTransportDeliveryExecutionRow
      ),
    nonClaims: RuntimeTransportDeliveryExecutionNonClaims,
    knownGaps: AppInstallPurchaseRuntimeTransportDeliveryExecutionKnownGaps,
    updatedAt: RuntimeTransportDeliveryExecutionTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeTransportDeliveryExecutionProof(
  proof: AppInstallPurchaseRuntimeTransportDeliveryExecutionProof
) {
  return {
    runtimeTransportDeliveryExecutionRows: proof.runtimeTransportDeliveryExecutionRows.length,
    withheldExecutionRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) => row.runtimeTransportExecutionState === 'execution-withheld-missing-artifacts'
    ).length,
    manualRequiredRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) => row.runtimeTransportExecutionState === 'manual-required'
    ).length,
    transportAttemptsStartedRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) =>
        row.runtimeTransportAttemptState !== 'not-started' && row.runtimeTransportAttemptState !== 'manual-required'
    ).length,
    deliveryResultRecordedRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) =>
        row.runtimeDeliveryResultState !== 'result-not-recorded' && row.runtimeDeliveryResultState !== 'manual-required'
    ).length,
    childDeviceReceiptHandoffReadyRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) =>
        row.childDeviceReceiptHandoffState !== 'receipt-handoff-missing' &&
        row.childDeviceReceiptHandoffState !== 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.runtimeTransportDeliveryExecutionRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function runtimeTransportDeliveryExecutionRow(
  row: (typeof AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel.runtimeDeliveryReceiptBoundaryRows)[number]
) {
  const manual = row.runtimeDeliveryReceiptBoundaryState === 'manual-required';
  return {
    schemaVersion: RuntimeTransportDeliveryExecutionProofVersion,
    runtimeTransportDeliveryExecutionRowId: `runtime-transport-delivery-execution-${row.sourceDecisionAction}`,
    sourceReceiptBoundaryProofVersion: SourceReceiptBoundaryProofVersion,
    sourceReceiptBoundaryRowId: row.runtimeDeliveryReceiptBoundaryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceReceiptBoundaryState: row.runtimeDeliveryReceiptBoundaryState,
    sourceChildDeviceTransportReceiptState: row.childDeviceTransportReceiptState,
    sourceParentOwnedDispatchPacketRef: row.sourceParentOwnedDispatchPacketRef,
    sourceParentOwnedReceiptBoundaryRef: row.parentOwnedReceiptBoundaryRef,
    parentOwnedTransportExecutionAttemptRef: `parent-owned-runtime-transport-execution-attempt-${row.sourceDecisionAction}`,
    parentOwnedDeliveryResultReceiptRef: `parent-owned-runtime-delivery-result-receipt-${row.sourceDecisionAction}`,
    childDeviceReceiptHandoffRef: `child-device-transport-receipt-handoff-${row.sourceDecisionAction}`,
    runtimeTransportExecutionState: manual ? 'manual-required' : 'execution-withheld-missing-artifacts',
    runtimeTransportAttemptState: manual ? 'manual-required' : 'not-started',
    runtimeDeliveryResultState: manual ? 'manual-required' : 'result-not-recorded',
    childDeviceReceiptHandoffState: manual ? 'manual-required' : 'receipt-handoff-missing',
    requiredRuntimeExecutionBlockers: RuntimeTransportDeliveryExecutionBlockers,
    externalWriterDispatchExecutionProofRefs: row.externalWriterDispatchExecutionProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    executionWithheldReasonRefs: [
      `missing-external-writer-dispatch-execution-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    runtimeTransportDeliveryExecutionAuditEventRefs: [
      ...row.receiptBoundaryAuditEventRefs,
      `runtime-transport-delivery-execution-audit-${row.sourceDecisionAction}`,
    ],
    externalRuntimeWriterExecutionClaim: row.externalRuntimeWriterExecutionClaim,
    externalRuntimeWriterDeliveryClaim: row.externalRuntimeWriterDeliveryClaim,
    parentActionRuntimeDeliveryClaim: row.parentActionRuntimeDeliveryClaim,
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: RuntimeTransportDeliveryExecutionBoundary,
    executionBoundaryCheckedAt: RuntimeTransportDeliveryExecutionTimestamp,
  } as const;
}

function runtimeTransportDeliveryExecutionRowIsHonest(row: RuntimeTransportDeliveryExecutionRowCandidate): boolean {
  return (
    runtimeTransportDeliveryExecutionStatesMatchSource(row) &&
    runtimeTransportDeliveryExecutionRefsAreComplete(row) &&
    runtimeTransportDeliveryExecutionClaimsStayUnimplemented(row) &&
    runtimeTransportDeliveryExecutionBoundaryIsExplicit(row.claimBoundary)
  );
}

function runtimeTransportDeliveryExecutionStatesMatchSource(
  row: RuntimeTransportDeliveryExecutionRowCandidate
): boolean {
  if (row.sourceReceiptBoundaryState === 'manual-required') {
    return (
      row.sourceChildDeviceTransportReceiptState === 'manual-required' &&
      row.runtimeTransportExecutionState === 'manual-required' &&
      row.runtimeTransportAttemptState === 'manual-required' &&
      row.runtimeDeliveryResultState === 'manual-required' &&
      row.childDeviceReceiptHandoffState === 'manual-required'
    );
  }
  return (
    row.sourceReceiptBoundaryState === 'receipt-blocked-waiting-runtime-artifacts' &&
    row.sourceChildDeviceTransportReceiptState === 'receipt-missing' &&
    row.runtimeTransportExecutionState === 'execution-withheld-missing-artifacts' &&
    row.runtimeTransportAttemptState === 'not-started' &&
    row.runtimeDeliveryResultState === 'result-not-recorded' &&
    row.childDeviceReceiptHandoffState === 'receipt-handoff-missing'
  );
}

function runtimeTransportDeliveryExecutionRefsAreComplete(row: RuntimeTransportDeliveryExecutionRowCandidate): boolean {
  return (
    runtimeTransportDeliveryExecutionSourceRefsAreComplete(row) &&
    runtimeTransportDeliveryExecutionOwnedRefsAreComplete(row) &&
    runtimeTransportDeliveryExecutionRequiredRefsAreComplete(row)
  );
}

function runtimeTransportDeliveryExecutionSourceRefsAreComplete(
  row: RuntimeTransportDeliveryExecutionRowCandidate
): boolean {
  return (
    row.sourceReceiptBoundaryProofVersion === SourceReceiptBoundaryProofVersion &&
    row.sourceReceiptBoundaryRowId.length > 0 &&
    row.sourceParentOwnedDispatchPacketRef.length > 0 &&
    row.sourceParentOwnedReceiptBoundaryRef.length > 0
  );
}

function runtimeTransportDeliveryExecutionOwnedRefsAreComplete(
  row: RuntimeTransportDeliveryExecutionRowCandidate
): boolean {
  return (
    row.parentOwnedTransportExecutionAttemptRef.length > 0 &&
    row.parentOwnedDeliveryResultReceiptRef.length > 0 &&
    row.childDeviceReceiptHandoffRef.length > 0 &&
    row.executionWithheldReasonRefs.length === RuntimeTransportDeliveryExecutionBlockers.length &&
    row.runtimeTransportDeliveryExecutionAuditEventRefs.length > 0
  );
}

function runtimeTransportDeliveryExecutionRequiredRefsAreComplete(
  row: RuntimeTransportDeliveryExecutionRowCandidate
): boolean {
  return (
    RuntimeTransportDeliveryExecutionBlockers.every((blocker) =>
      row.requiredRuntimeExecutionBlockers.includes(blocker)
    ) &&
    row.externalWriterDispatchExecutionProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0
  );
}

function runtimeTransportDeliveryExecutionClaimsStayUnimplemented(
  row: RuntimeTransportDeliveryExecutionRowCandidate
): boolean {
  return (
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
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

function runtimeTransportDeliveryExecutionProofIsHonest(
  proof: AppInstallPurchaseRuntimeTransportDeliveryExecutionProof
): boolean {
  const actions = new Set(proof.runtimeTransportDeliveryExecutionRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceReceiptBoundaryProofVersion === SourceReceiptBoundaryProofVersion &&
    proof.runtimeTransportDeliveryExecutionRows.length === RuntimeTransportDeliveryExecutionActions.length &&
    RuntimeTransportDeliveryExecutionActions.every((action) => actions.has(action)) &&
    RuntimeTransportDeliveryExecutionNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.runtimeTransportDeliveryExecutionRows.every(runtimeTransportDeliveryExecutionRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function runtimeTransportDeliveryExecutionBoundaryIsExplicit(
  boundary: typeof RuntimeTransportDeliveryExecutionBoundarySchema.Type
): boolean {
  return RuntimeTransportDeliveryExecutionBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
