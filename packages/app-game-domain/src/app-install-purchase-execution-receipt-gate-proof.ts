import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel } from './app-install-purchase-external-runtime-writer-transport-execution-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ExecutionReceiptGateProofVersion = 'app-install-purchase-execution-receipt-gate-proof';
const SourceWriterTransportExecutionProofVersion =
  'app-install-purchase-external-runtime-writer-transport-execution-proof';
const ExecutionReceiptGateTimestamp = '2026-06-08T04:24:00.000Z';
const ExecutionReceiptGateBoundary =
  'execution receipt gate proof only; rows consume external runtime writer transport execution rows and keep product progress blocked until external writer dispatch executor receipt provider-store execution receipt platform adapter execution receipt and child-device transport receipt artifacts are attached no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExecutionReceiptGateActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceWriterTransportExecutionStates = ['transport-execution-blocked', 'manual-required'] as const;
const SourceWriterTransportAckStates = ['ack-not-recorded', 'manual-required'] as const;
const ExecutionReceiptGateStates = ['blocked-missing-execution-receipts', 'manual-required'] as const;
const ExecutionReceiptFamilyStates = ['receipt-missing', 'manual-required'] as const;
const ExecutionReceiptFamilies = [
  'external-writer-dispatch-executor-receipt',
  'provider-store-execution-receipt',
  'platform-adapter-execution-receipt',
  'child-device-transport-receipt',
] as const;
const ExecutionReceiptGateNonClaims = [
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
const ExecutionReceiptGateFragments = [
  'rows consume external runtime writer transport execution rows',
  'external writer dispatch executor receipt',
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

export const AppInstallPurchaseExecutionReceiptGateProofSchemaVersionSchema = withParser(
  Schema.Literal(ExecutionReceiptGateProofVersion)
);
const ExecutionReceiptGateActionSchema = withParser(Schema.Literal(...ExecutionReceiptGateActions));
const SourceWriterTransportExecutionStateSchema = withParser(Schema.Literal(...SourceWriterTransportExecutionStates));
const SourceWriterTransportAckStateSchema = withParser(Schema.Literal(...SourceWriterTransportAckStates));
const ExecutionReceiptGateStateSchema = withParser(Schema.Literal(...ExecutionReceiptGateStates));
const ExecutionReceiptFamilyStateSchema = withParser(Schema.Literal(...ExecutionReceiptFamilyStates));
const ExecutionReceiptFamilySchema = withParser(Schema.Literal(...ExecutionReceiptFamilies));
const ExecutionReceiptGateNonClaimSchema = withParser(Schema.Literal(...ExecutionReceiptGateNonClaims));
const ExecutionReceiptGateExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExecutionReceiptGateDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExecutionReceiptGateIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExecutionReceiptGateAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExecutionReceiptGateCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));

const ExecutionReceiptGateRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExecutionReceiptGateRowId');
const ExecutionReceiptGateRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExecutionReceiptGateRef');
const ExecutionReceiptGateAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExecutionReceiptGateAuditRef');
const ExecutionReceiptGateBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseExecutionReceiptGateBoundary');

const ExecutionReceiptGateRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExecutionReceiptGateProofSchemaVersionSchema,
  executionReceiptGateRowId: ExecutionReceiptGateRowIdSchema,
  sourceWriterTransportExecutionProofVersion: Schema.Literal(SourceWriterTransportExecutionProofVersion),
  sourceWriterTransportExecutionRowId: ExecutionReceiptGateRefSchema,
  sourceDecisionAction: ExecutionReceiptGateActionSchema,
  sourceWriterTransportExecutionState: SourceWriterTransportExecutionStateSchema,
  sourceWriterTransportAckState: SourceWriterTransportAckStateSchema,
  sourceExternalWriterTransportPacketRef: ExecutionReceiptGateRefSchema,
  sourceExternalWriterTransportExecutionStatusRef: ExecutionReceiptGateRefSchema,
  sourceExternalWriterTransportAckRef: ExecutionReceiptGateRefSchema,
  executionReceiptGateState: ExecutionReceiptGateStateSchema,
  externalWriterDispatchExecutorReceiptState: ExecutionReceiptFamilyStateSchema,
  providerStoreExecutionReceiptState: ExecutionReceiptFamilyStateSchema,
  platformAdapterExecutionReceiptState: ExecutionReceiptFamilyStateSchema,
  childDeviceTransportReceiptState: ExecutionReceiptFamilyStateSchema,
  requiredExecutionReceiptFamilies: Schema.Array(ExecutionReceiptFamilySchema),
  externalWriterDispatchExecutorReceiptProofRefs: Schema.Array(ExecutionReceiptGateRefSchema),
  providerStoreExecutionReceiptProofRefs: Schema.Array(ExecutionReceiptGateRefSchema),
  platformAdapterExecutionReceiptProofRefs: Schema.Array(ExecutionReceiptGateRefSchema),
  childDeviceTransportReceiptProofRefs: Schema.Array(ExecutionReceiptGateRefSchema),
  executionReceiptGateBlockedReasonRefs: Schema.Array(ExecutionReceiptGateRefSchema),
  executionReceiptGateAuditEventRefs: Schema.Array(ExecutionReceiptGateAuditRefSchema),
  externalRuntimeWriterExecutionClaim: ExecutionReceiptGateExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExecutionReceiptGateDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExecutionReceiptGateDeliveryClaimSchema,
  providerApiExecutionClaim: ExecutionReceiptGateExecutionClaimSchema,
  storeIntegrationClaim: ExecutionReceiptGateIntegrationClaimSchema,
  platformInterceptionClaim: ExecutionReceiptGateIntegrationClaimSchema,
  platformAdapterClaim: ExecutionReceiptGateAdapterClaimSchema,
  childDeviceDeliveryClaim: ExecutionReceiptGateDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExecutionReceiptGateDeliveryClaimSchema,
  appBlockingClaim: ExecutionReceiptGateIntegrationClaimSchema,
  childDataCustody: ExecutionReceiptGateCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExecutionReceiptGateIntegrationClaimSchema,
  claimBoundary: ExecutionReceiptGateBoundarySchema,
  executionReceiptGateCheckedAt: ParentTimestampSchema,
});

type ExecutionReceiptGateRowCandidate = Infer<typeof ExecutionReceiptGateRowBaseSchema>;

export const AppInstallPurchaseExecutionReceiptGateRowSchema = withParser(
  ExecutionReceiptGateRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        executionReceiptGateRowIsHonest(row) ||
        'Expected app install/purchase execution receipt gate rows to block progress until all execution receipt artifacts exist'
    )
  )
);

const ExecutionReceiptGateProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExecutionReceiptGateProofSchemaVersionSchema,
  sourceWriterTransportExecutionProofVersion: Schema.Literal(SourceWriterTransportExecutionProofVersion),
  executionReceiptGateRows: Schema.Array(AppInstallPurchaseExecutionReceiptGateRowSchema),
  nonClaims: Schema.Array(ExecutionReceiptGateNonClaimSchema),
  knownGaps: Schema.Array(ExecutionReceiptGateRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExecutionReceiptGateProof = Infer<typeof ExecutionReceiptGateProofBaseSchema>;

export const AppInstallPurchaseExecutionReceiptGateProofSchema = withParser(
  ExecutionReceiptGateProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        executionReceiptGateProofIsHonest(proof) ||
        'Expected app install/purchase execution receipt gate proof to preserve missing receipt families and non-claims'
    )
  )
);

export const AppInstallPurchaseExecutionReceiptGateKnownGaps = [
  'Execution receipt gate rows are parent-owned contract rows only; no external writer dispatch executor receipt, provider/store execution receipt, platform adapter execution receipt, or child-device transport receipt artifact exists.',
  'Rows stay blocked or manual-required until all four receipt families are attached as real proof artifacts.',
  'This proof does not upgrade app-install product claims and keeps portal UI, provider/store execution, platform adapter execution, child-device delivery, runtime report delivery, app blocking, and child activity custody unclaimed.',
] as const;

export const AppInstallPurchaseExecutionReceiptGateProofReadModel =
  AppInstallPurchaseExecutionReceiptGateProofSchema.parse({
    schemaVersion: ExecutionReceiptGateProofVersion,
    sourceWriterTransportExecutionProofVersion: SourceWriterTransportExecutionProofVersion,
    executionReceiptGateRows:
      AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel.externalRuntimeWriterTransportExecutionRows.map(
        executionReceiptGateRow
      ),
    nonClaims: ExecutionReceiptGateNonClaims,
    knownGaps: AppInstallPurchaseExecutionReceiptGateKnownGaps,
    updatedAt: ExecutionReceiptGateTimestamp,
  });

export function summarizeAppInstallPurchaseExecutionReceiptGateProof(
  proof: AppInstallPurchaseExecutionReceiptGateProof
) {
  return {
    executionReceiptGateRows: proof.executionReceiptGateRows.length,
    blockedReceiptGateRows: proof.executionReceiptGateRows.filter(
      (row) => row.executionReceiptGateState === 'blocked-missing-execution-receipts'
    ).length,
    manualRequiredRows: proof.executionReceiptGateRows.filter(
      (row) => row.executionReceiptGateState === 'manual-required'
    ).length,
    acceptedExecutionReceiptFamilies: proof.executionReceiptGateRows.filter(executionReceiptGateRowHasAcceptedReceipt)
      .length,
    childDeviceDeliveredRows: proof.executionReceiptGateRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function executionReceiptGateRow(
  row: (typeof AppInstallPurchaseExternalRuntimeWriterTransportExecutionProofReadModel.externalRuntimeWriterTransportExecutionRows)[number]
) {
  const manual = row.externalWriterTransportExecutionState === 'manual-required';
  return {
    schemaVersion: ExecutionReceiptGateProofVersion,
    executionReceiptGateRowId: `execution-receipt-gate-${row.sourceDecisionAction}`,
    sourceWriterTransportExecutionProofVersion: SourceWriterTransportExecutionProofVersion,
    sourceWriterTransportExecutionRowId: row.externalRuntimeWriterTransportExecutionRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceWriterTransportExecutionState: row.externalWriterTransportExecutionState,
    sourceWriterTransportAckState: row.externalWriterTransportAckState,
    sourceExternalWriterTransportPacketRef: row.parentOwnedExternalWriterTransportPacketRef,
    sourceExternalWriterTransportExecutionStatusRef: row.parentOwnedExternalWriterTransportExecutionStatusRef,
    sourceExternalWriterTransportAckRef: row.parentOwnedExternalWriterTransportAckRef,
    executionReceiptGateState: manual ? 'manual-required' : 'blocked-missing-execution-receipts',
    externalWriterDispatchExecutorReceiptState: manual ? 'manual-required' : 'receipt-missing',
    providerStoreExecutionReceiptState: manual ? 'manual-required' : 'receipt-missing',
    platformAdapterExecutionReceiptState: manual ? 'manual-required' : 'receipt-missing',
    childDeviceTransportReceiptState: manual ? 'manual-required' : 'receipt-missing',
    requiredExecutionReceiptFamilies: ExecutionReceiptFamilies,
    externalWriterDispatchExecutorReceiptProofRefs: row.externalWriterDispatchExecutorProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionReceiptProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionReceiptProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    executionReceiptGateBlockedReasonRefs: [
      `missing-external-writer-dispatch-executor-receipt-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    executionReceiptGateAuditEventRefs: [
      ...row.externalWriterTransportExecutionAuditEventRefs,
      `execution-receipt-gate-audit-${row.sourceDecisionAction}`,
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
    claimBoundary: ExecutionReceiptGateBoundary,
    executionReceiptGateCheckedAt: ExecutionReceiptGateTimestamp,
  } as const;
}

function executionReceiptGateRowHasAcceptedReceipt(row: ExecutionReceiptGateRowCandidate): boolean {
  return (
    executionReceiptFamilyIsAccepted(row.externalWriterDispatchExecutorReceiptState) ||
    executionReceiptFamilyIsAccepted(row.providerStoreExecutionReceiptState) ||
    executionReceiptFamilyIsAccepted(row.platformAdapterExecutionReceiptState) ||
    executionReceiptFamilyIsAccepted(row.childDeviceTransportReceiptState)
  );
}

function executionReceiptFamilyIsAccepted(state: typeof ExecutionReceiptFamilyStateSchema.Type): boolean {
  return state !== 'receipt-missing' && state !== 'manual-required';
}

function executionReceiptGateRowIsHonest(row: ExecutionReceiptGateRowCandidate): boolean {
  return (
    executionReceiptGateStatesMatchSource(row) &&
    executionReceiptGateRefsAreComplete(row) &&
    executionReceiptGateClaimsStayUnimplemented(row) &&
    executionReceiptGateBoundaryIsExplicit(row.claimBoundary)
  );
}

function executionReceiptGateStatesMatchSource(row: ExecutionReceiptGateRowCandidate): boolean {
  if (row.sourceWriterTransportExecutionState === 'manual-required') {
    return executionReceiptGateManualStatesMatch(row);
  }
  return executionReceiptGateBlockedStatesMatch(row);
}

function executionReceiptGateManualStatesMatch(row: ExecutionReceiptGateRowCandidate): boolean {
  return (
    row.sourceWriterTransportAckState === 'manual-required' &&
    row.executionReceiptGateState === 'manual-required' &&
    row.externalWriterDispatchExecutorReceiptState === 'manual-required' &&
    row.providerStoreExecutionReceiptState === 'manual-required' &&
    row.platformAdapterExecutionReceiptState === 'manual-required' &&
    row.childDeviceTransportReceiptState === 'manual-required'
  );
}

function executionReceiptGateBlockedStatesMatch(row: ExecutionReceiptGateRowCandidate): boolean {
  return (
    row.sourceWriterTransportExecutionState === 'transport-execution-blocked' &&
    row.sourceWriterTransportAckState === 'ack-not-recorded' &&
    row.executionReceiptGateState === 'blocked-missing-execution-receipts' &&
    row.externalWriterDispatchExecutorReceiptState === 'receipt-missing' &&
    row.providerStoreExecutionReceiptState === 'receipt-missing' &&
    row.platformAdapterExecutionReceiptState === 'receipt-missing' &&
    row.childDeviceTransportReceiptState === 'receipt-missing'
  );
}

function executionReceiptGateRefsAreComplete(row: ExecutionReceiptGateRowCandidate): boolean {
  return (
    row.sourceWriterTransportExecutionProofVersion === SourceWriterTransportExecutionProofVersion &&
    row.sourceWriterTransportExecutionRowId.length > 0 &&
    row.sourceExternalWriterTransportPacketRef.length > 0 &&
    row.sourceExternalWriterTransportExecutionStatusRef.length > 0 &&
    row.sourceExternalWriterTransportAckRef.length > 0 &&
    ExecutionReceiptFamilies.every((family) => row.requiredExecutionReceiptFamilies.includes(family)) &&
    row.externalWriterDispatchExecutorReceiptProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.executionReceiptGateBlockedReasonRefs.length === ExecutionReceiptFamilies.length &&
    row.executionReceiptGateAuditEventRefs.length > 0
  );
}

function executionReceiptGateClaimsStayUnimplemented(row: ExecutionReceiptGateRowCandidate): boolean {
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

function executionReceiptGateProofIsHonest(proof: AppInstallPurchaseExecutionReceiptGateProof): boolean {
  const actions = new Set(proof.executionReceiptGateRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceWriterTransportExecutionProofVersion === SourceWriterTransportExecutionProofVersion &&
    proof.executionReceiptGateRows.length === ExecutionReceiptGateActions.length &&
    ExecutionReceiptGateActions.every((action) => actions.has(action)) &&
    ExecutionReceiptGateNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.executionReceiptGateRows.every(executionReceiptGateRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function executionReceiptGateBoundaryIsExplicit(boundary: typeof ExecutionReceiptGateBoundarySchema.Type): boolean {
  return ExecutionReceiptGateFragments.every((fragment) => boundary.includes(fragment));
}

