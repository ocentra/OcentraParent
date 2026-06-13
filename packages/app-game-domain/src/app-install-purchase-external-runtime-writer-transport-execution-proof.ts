import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel } from './app-install-purchase-runtime-transport-delivery-execution-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ExternalRuntimeWriterTransportExecutionProofVersion =
  'app-install-purchase-external-runtime-writer-transport-execution-proof';
const SourceRuntimeTransportDeliveryExecutionProofVersion =
  'app-install-purchase-runtime-transport-delivery-execution-proof';
const ExternalRuntimeWriterTransportExecutionTimestamp = '2026-06-08T02:59:00.000Z';
const ExternalRuntimeWriterTransportExecutionBoundary =
  'external runtime writer transport execution proof only; rows consume runtime transport delivery execution rows and create parent-owned external writer transport packet execution status refs that remain blocked until an external writer dispatch executor provider-store execution receipt platform adapter execution receipt and child-device transport receipt are real no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeWriterTransportExecutionActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceTransportExecutionStates = ['execution-withheld-missing-artifacts', 'manual-required'] as const;
const SourceTransportAttemptStates = ['not-started', 'manual-required'] as const;
const SourceDeliveryResultStates = ['result-not-recorded', 'manual-required'] as const;
const ExternalWriterTransportExecutionStates = ['transport-execution-blocked', 'manual-required'] as const;
const ExternalWriterTransportPacketStates = ['packet-withheld', 'manual-required'] as const;
const ExternalWriterTransportAckStates = ['ack-not-recorded', 'manual-required'] as const;
const ExternalWriterTransportExecutionBlockers = [
  'external-writer-dispatch-executor-missing',
  'provider-store-execution-receipt-missing',
  'platform-adapter-execution-receipt-missing',
  'child-device-transport-receipt-missing',
] as const;
const ExternalRuntimeWriterTransportExecutionNonClaims = [
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
const ExternalRuntimeWriterTransportExecutionBoundaryFragments = [
  'rows consume runtime transport delivery execution rows',
  'parent-owned external writer transport packet execution status refs',
  'remain blocked',
  'external writer dispatch executor',
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

export const AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeWriterTransportExecutionProofVersion)
);
const ExternalRuntimeWriterTransportExecutionActionSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterTransportExecutionActions)
);
const SourceTransportExecutionStateSchema = withParser(Schema.Literal(...SourceTransportExecutionStates));
const SourceTransportAttemptStateSchema = withParser(Schema.Literal(...SourceTransportAttemptStates));
const SourceDeliveryResultStateSchema = withParser(Schema.Literal(...SourceDeliveryResultStates));
const ExternalWriterTransportExecutionStateSchema = withParser(
  Schema.Literal(...ExternalWriterTransportExecutionStates)
);
const ExternalWriterTransportPacketStateSchema = withParser(Schema.Literal(...ExternalWriterTransportPacketStates));
const ExternalWriterTransportAckStateSchema = withParser(Schema.Literal(...ExternalWriterTransportAckStates));
const ExternalWriterTransportExecutionBlockerSchema = withParser(
  Schema.Literal(...ExternalWriterTransportExecutionBlockers)
);
const ExternalRuntimeWriterTransportExecutionNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterTransportExecutionNonClaims)
);
const ExternalRuntimeWriterTransportExecutionExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeWriterTransportExecutionDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeWriterTransportExecutionIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeWriterTransportExecutionAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeWriterTransportExecutionCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));

const ExternalRuntimeWriterTransportExecutionRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowId');
const ExternalRuntimeWriterTransportExecutionRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterTransportExecutionRef');
const ExternalRuntimeWriterTransportExecutionAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterTransportExecutionAuditRef');
const ExternalRuntimeWriterTransportExecutionBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterTransportExecutionBoundary');

const ExternalRuntimeWriterTransportExecutionRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchemaVersionSchema,
  externalRuntimeWriterTransportExecutionRowId: ExternalRuntimeWriterTransportExecutionRowIdSchema,
  sourceRuntimeTransportDeliveryExecutionProofVersion: Schema.Literal(
    SourceRuntimeTransportDeliveryExecutionProofVersion
  ),
  sourceRuntimeTransportDeliveryExecutionRowId: ExternalRuntimeWriterTransportExecutionRefSchema,
  sourceDecisionAction: ExternalRuntimeWriterTransportExecutionActionSchema,
  sourceRuntimeTransportExecutionState: SourceTransportExecutionStateSchema,
  sourceRuntimeTransportAttemptState: SourceTransportAttemptStateSchema,
  sourceRuntimeDeliveryResultState: SourceDeliveryResultStateSchema,
  sourceParentOwnedTransportExecutionAttemptRef: ExternalRuntimeWriterTransportExecutionRefSchema,
  sourceParentOwnedDeliveryResultReceiptRef: ExternalRuntimeWriterTransportExecutionRefSchema,
  sourceChildDeviceReceiptHandoffRef: ExternalRuntimeWriterTransportExecutionRefSchema,
  parentOwnedExternalWriterTransportPacketRef: ExternalRuntimeWriterTransportExecutionRefSchema,
  parentOwnedExternalWriterTransportExecutionStatusRef: ExternalRuntimeWriterTransportExecutionRefSchema,
  parentOwnedExternalWriterTransportAckRef: ExternalRuntimeWriterTransportExecutionRefSchema,
  externalWriterTransportExecutionState: ExternalWriterTransportExecutionStateSchema,
  externalWriterTransportPacketState: ExternalWriterTransportPacketStateSchema,
  externalWriterTransportAckState: ExternalWriterTransportAckStateSchema,
  requiredExternalWriterTransportExecutionBlockers: Schema.Array(ExternalWriterTransportExecutionBlockerSchema),
  externalWriterDispatchExecutorProofRefs: Schema.Array(ExternalRuntimeWriterTransportExecutionRefSchema),
  providerStoreExecutionReceiptProofRefs: Schema.Array(ExternalRuntimeWriterTransportExecutionRefSchema),
  platformAdapterExecutionReceiptProofRefs: Schema.Array(ExternalRuntimeWriterTransportExecutionRefSchema),
  childDeviceTransportReceiptProofRefs: Schema.Array(ExternalRuntimeWriterTransportExecutionRefSchema),
  transportExecutionBlockedReasonRefs: Schema.Array(ExternalRuntimeWriterTransportExecutionRefSchema),
  externalWriterTransportExecutionAuditEventRefs: Schema.Array(ExternalRuntimeWriterTransportExecutionAuditRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeWriterTransportExecutionExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeWriterTransportExecutionDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeWriterTransportExecutionDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeWriterTransportExecutionExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeWriterTransportExecutionIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeWriterTransportExecutionIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeWriterTransportExecutionAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeWriterTransportExecutionDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeWriterTransportExecutionDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeWriterTransportExecutionIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeWriterTransportExecutionCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeWriterTransportExecutionIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeWriterTransportExecutionBoundarySchema,
  transportExecutionCheckedAt: ParentTimestampSchema,
});

type ExternalRuntimeWriterTransportExecutionRowCandidate = Infer<
  typeof ExternalRuntimeWriterTransportExecutionRowBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema = withParser(
  ExternalRuntimeWriterTransportExecutionRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeWriterTransportExecutionRowIsHonest(row) ||
        'Expected external runtime writer transport execution rows to keep packets blocked until real transport receipts exist'
    )
  )
);

const ExternalRuntimeWriterTransportExecutionProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchemaVersionSchema,
  sourceRuntimeTransportDeliveryExecutionProofVersion: Schema.Literal(
    SourceRuntimeTransportDeliveryExecutionProofVersion
  ),
  externalRuntimeWriterTransportExecutionRows: Schema.Array(
    AppInstallPurchaseExternalRuntimeWriterTransportExecutionRowSchema
  ),
  nonClaims: Schema.Array(ExternalRuntimeWriterTransportExecutionNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeWriterTransportExecutionRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeWriterTransportExecutionProof = Infer<
  typeof ExternalRuntimeWriterTransportExecutionProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchema = withParser(
  ExternalRuntimeWriterTransportExecutionProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeWriterTransportExecutionProofIsHonest(proof) ||
        'Expected app install/purchase external runtime writer transport execution proof to preserve blocked transport packets and non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeWriterTransportExecutionKnownGaps = [
  'External writer transport execution rows are parent-owned packet/status refs only; no external writer dispatch executor, provider/store execution receipt, platform adapter execution receipt, or child-device transport receipt exists.',
  'External writer transport packets remain blocked or manual-required until external writer dispatch executor, provider/store execution receipt, platform adapter execution receipt, and child-device transport receipt proof refs become real artifacts.',
  'Product capability checklist Install/purchase approval row records this external runtime writer transport execution proof as non-claim evidence while provider/store execution, platform adapter execution, child-device transport receipt, and portal approval/report UI artifacts remain required.',
] as const;

export const AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel =
  AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofSchema.parse({
    schemaVersion: ExternalRuntimeWriterTransportExecutionProofVersion,
    sourceRuntimeTransportDeliveryExecutionProofVersion: SourceRuntimeTransportDeliveryExecutionProofVersion,
    externalRuntimeWriterTransportExecutionRows:
      AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel.runtimeTransportDeliveryExecutionRows.map(
        externalRuntimeWriterTransportExecutionRow
      ),
    nonClaims: ExternalRuntimeWriterTransportExecutionNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeWriterTransportExecutionKnownGaps,
    updatedAt: ExternalRuntimeWriterTransportExecutionTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeWriterTransportExecutionProof(
  proof: AppInstallPurchaseExternalRuntimeWriterTransportExecutionProof
) {
  return {
    externalRuntimeWriterTransportExecutionRows: proof.externalRuntimeWriterTransportExecutionRows.length,
    blockedTransportExecutionRows: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalWriterTransportExecutionState === 'transport-execution-blocked'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalWriterTransportExecutionState === 'manual-required'
    ).length,
    withheldTransportPackets: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalWriterTransportPacketState === 'packet-withheld'
    ).length,
    recordedTransportAcks: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) =>
        row.externalWriterTransportAckState !== 'ack-not-recorded' &&
        row.externalWriterTransportAckState !== 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.externalRuntimeWriterTransportExecutionRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
  } as const;
}

function externalRuntimeWriterTransportExecutionRow(
  row: (typeof AppInstallPurchaseRuntimeTransportDeliveryExecutionProofReadModel.runtimeTransportDeliveryExecutionRows)[number]
) {
  const manual = row.runtimeTransportExecutionState === 'manual-required';
  return {
    schemaVersion: ExternalRuntimeWriterTransportExecutionProofVersion,
    externalRuntimeWriterTransportExecutionRowId: `external-runtime-writer-transport-execution-${row.sourceDecisionAction}`,
    sourceRuntimeTransportDeliveryExecutionProofVersion: SourceRuntimeTransportDeliveryExecutionProofVersion,
    sourceRuntimeTransportDeliveryExecutionRowId: row.runtimeTransportDeliveryExecutionRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeTransportExecutionState: row.runtimeTransportExecutionState,
    sourceRuntimeTransportAttemptState: row.runtimeTransportAttemptState,
    sourceRuntimeDeliveryResultState: row.runtimeDeliveryResultState,
    sourceParentOwnedTransportExecutionAttemptRef: row.parentOwnedTransportExecutionAttemptRef,
    sourceParentOwnedDeliveryResultReceiptRef: row.parentOwnedDeliveryResultReceiptRef,
    sourceChildDeviceReceiptHandoffRef: row.childDeviceReceiptHandoffRef,
    parentOwnedExternalWriterTransportPacketRef: `parent-owned-external-writer-transport-packet-${row.sourceDecisionAction}`,
    parentOwnedExternalWriterTransportExecutionStatusRef: `parent-owned-external-writer-transport-execution-status-${row.sourceDecisionAction}`,
    parentOwnedExternalWriterTransportAckRef: `parent-owned-external-writer-transport-ack-${row.sourceDecisionAction}`,
    externalWriterTransportExecutionState: manual ? 'manual-required' : 'transport-execution-blocked',
    externalWriterTransportPacketState: manual ? 'manual-required' : 'packet-withheld',
    externalWriterTransportAckState: manual ? 'manual-required' : 'ack-not-recorded',
    requiredExternalWriterTransportExecutionBlockers: ExternalWriterTransportExecutionBlockers,
    externalWriterDispatchExecutorProofRefs: row.externalWriterDispatchExecutionProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    transportExecutionBlockedReasonRefs: [
      `missing-external-writer-dispatch-executor-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    externalWriterTransportExecutionAuditEventRefs: [
      ...row.runtimeTransportDeliveryExecutionAuditEventRefs,
      `external-runtime-writer-transport-execution-audit-${row.sourceDecisionAction}`,
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
    claimBoundary: ExternalRuntimeWriterTransportExecutionBoundary,
    transportExecutionCheckedAt: ExternalRuntimeWriterTransportExecutionTimestamp,
  } as const;
}

function externalRuntimeWriterTransportExecutionRowIsHonest(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
): boolean {
  return (
    externalRuntimeWriterTransportExecutionStatesMatchSource(row) &&
    externalRuntimeWriterTransportExecutionRefsAreComplete(row) &&
    externalRuntimeWriterTransportExecutionClaimsStayUnimplemented(row) &&
    externalRuntimeWriterTransportExecutionBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeWriterTransportExecutionStatesMatchSource(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
): boolean {
  if (row.sourceRuntimeTransportExecutionState === 'manual-required') {
    return (
      row.sourceRuntimeTransportAttemptState === 'manual-required' &&
      row.sourceRuntimeDeliveryResultState === 'manual-required' &&
      row.externalWriterTransportExecutionState === 'manual-required' &&
      row.externalWriterTransportPacketState === 'manual-required' &&
      row.externalWriterTransportAckState === 'manual-required'
    );
  }
  return (
    row.sourceRuntimeTransportExecutionState === 'execution-withheld-missing-artifacts' &&
    row.sourceRuntimeTransportAttemptState === 'not-started' &&
    row.sourceRuntimeDeliveryResultState === 'result-not-recorded' &&
    row.externalWriterTransportExecutionState === 'transport-execution-blocked' &&
    row.externalWriterTransportPacketState === 'packet-withheld' &&
    row.externalWriterTransportAckState === 'ack-not-recorded'
  );
}

function externalRuntimeWriterTransportExecutionRefsAreComplete(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
): boolean {
  return (
    externalRuntimeWriterTransportExecutionSourceRefsAreComplete(row) &&
    externalRuntimeWriterTransportExecutionOwnedRefsAreComplete(row) &&
    externalRuntimeWriterTransportExecutionRequiredRefsAreComplete(row)
  );
}

function externalRuntimeWriterTransportExecutionSourceRefsAreComplete(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
): boolean {
  return (
    row.sourceRuntimeTransportDeliveryExecutionProofVersion === SourceRuntimeTransportDeliveryExecutionProofVersion &&
    row.sourceRuntimeTransportDeliveryExecutionRowId.length > 0 &&
    row.sourceParentOwnedTransportExecutionAttemptRef.length > 0 &&
    row.sourceParentOwnedDeliveryResultReceiptRef.length > 0 &&
    row.sourceChildDeviceReceiptHandoffRef.length > 0
  );
}

function externalRuntimeWriterTransportExecutionOwnedRefsAreComplete(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
): boolean {
  return (
    row.parentOwnedExternalWriterTransportPacketRef.length > 0 &&
    row.parentOwnedExternalWriterTransportExecutionStatusRef.length > 0 &&
    row.parentOwnedExternalWriterTransportAckRef.length > 0 &&
    row.transportExecutionBlockedReasonRefs.length === ExternalWriterTransportExecutionBlockers.length &&
    row.externalWriterTransportExecutionAuditEventRefs.length > 0
  );
}

function externalRuntimeWriterTransportExecutionRequiredRefsAreComplete(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
): boolean {
  return (
    ExternalWriterTransportExecutionBlockers.every((blocker) =>
      row.requiredExternalWriterTransportExecutionBlockers.includes(blocker)
    ) &&
    row.externalWriterDispatchExecutorProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0
  );
}

function externalRuntimeWriterTransportExecutionClaimsStayUnimplemented(
  row: ExternalRuntimeWriterTransportExecutionRowCandidate
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

function externalRuntimeWriterTransportExecutionProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeWriterTransportExecutionProof
): boolean {
  const actions = new Set(proof.externalRuntimeWriterTransportExecutionRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeTransportDeliveryExecutionProofVersion === SourceRuntimeTransportDeliveryExecutionProofVersion &&
    proof.externalRuntimeWriterTransportExecutionRows.length ===
      ExternalRuntimeWriterTransportExecutionActions.length &&
    ExternalRuntimeWriterTransportExecutionActions.every((action) => actions.has(action)) &&
    ExternalRuntimeWriterTransportExecutionNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeWriterTransportExecutionRows.every(externalRuntimeWriterTransportExecutionRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeWriterTransportExecutionBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeWriterTransportExecutionBoundarySchema.Type
): boolean {
  return ExternalRuntimeWriterTransportExecutionBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

