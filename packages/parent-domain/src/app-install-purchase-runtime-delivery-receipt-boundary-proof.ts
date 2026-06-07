import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel } from './app-install-purchase-external-runtime-transport-dispatch-preflight-proof';
import { ParentTimestampSchema } from './reference-primitives';

const ReceiptBoundaryProofVersion = 'app-install-purchase-runtime-delivery-receipt-boundary-proof';
const SourceDispatchPreflightProofVersion = 'app-install-purchase-external-runtime-transport-dispatch-preflight-proof';
const ReceiptBoundaryTimestamp = '2026-06-07T19:18:00.000Z';
const ReceiptBoundaryText = Schema.String.pipe(Schema.minLength(1));
const ReceiptBoundary =
  'runtime delivery receipt boundary proof only; receipt rows consume parent-owned withheld dispatch packets and require external writer dispatch execution child-device transport receipt provider-store execution and platform adapter execution proof refs before any delivery receipt claim no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ReceiptBoundaryActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourcePreflightStates = ['blocked-waiting-runtime-artifacts', 'manual-required'] as const;
const SourcePacketStates = ['withheld', 'manual-required'] as const;
const ReceiptBoundaryStates = ['receipt-blocked-waiting-runtime-artifacts', 'manual-required'] as const;
const ReceiptExpectationStates = ['receipt-missing', 'manual-required'] as const;
const ReceiptReadinessStates = ['not-ready', 'manual-required'] as const;
const ReceiptArtifactBlockers = [
  'external-writer-dispatch-execution-missing',
  'provider-store-execution-receipt-missing',
  'platform-adapter-execution-receipt-missing',
  'child-device-transport-receipt-missing',
] as const;
const ReceiptBoundaryNonClaims = [
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
const ReceiptBoundaryFragments = [
  'receipt rows consume parent-owned withheld dispatch packets',
  'external writer dispatch execution',
  'child-device transport receipt',
  'provider-store execution',
  'platform adapter execution',
  'before any delivery receipt claim',
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

export const AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchemaVersionSchema = withParser(
  Schema.Literal(ReceiptBoundaryProofVersion)
);
const ReceiptBoundaryActionSchema = withParser(Schema.Literal(...ReceiptBoundaryActions));
const SourcePreflightStateSchema = withParser(Schema.Literal(...SourcePreflightStates));
const SourcePacketStateSchema = withParser(Schema.Literal(...SourcePacketStates));
const ReceiptBoundaryStateSchema = withParser(Schema.Literal(...ReceiptBoundaryStates));
const ReceiptExpectationStateSchema = withParser(Schema.Literal(...ReceiptExpectationStates));
const ReceiptReadinessStateSchema = withParser(Schema.Literal(...ReceiptReadinessStates));
const ReceiptArtifactBlockerSchema = withParser(Schema.Literal(...ReceiptArtifactBlockers));
const ReceiptBoundaryNonClaimSchema = withParser(Schema.Literal(...ReceiptBoundaryNonClaims));
const ReceiptExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ReceiptDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ReceiptIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ReceiptAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ReceiptCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));

const ReceiptBoundaryRowIdSchema = ReceiptBoundaryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowId')
);
const ReceiptBoundaryRefSchema = ReceiptBoundaryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRef')
);
const ReceiptBoundaryAuditRefSchema = ReceiptBoundaryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeDeliveryReceiptBoundaryAuditRef')
);
const ReceiptBoundaryClaimBoundarySchema = ReceiptBoundaryText.pipe(
  Schema.brand('AppInstallPurchaseRuntimeDeliveryReceiptBoundaryClaimBoundary')
);

const ReceiptBoundaryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchemaVersionSchema,
  runtimeDeliveryReceiptBoundaryRowId: ReceiptBoundaryRowIdSchema,
  sourceDispatchPreflightProofVersion: Schema.Literal(SourceDispatchPreflightProofVersion),
  sourceDispatchPreflightRowId: ReceiptBoundaryRefSchema,
  sourceDecisionAction: ReceiptBoundaryActionSchema,
  sourceDispatchPreflightState: SourcePreflightStateSchema,
  sourceDispatchPacketState: SourcePacketStateSchema,
  sourceParentOwnedDispatchPacketRef: ReceiptBoundaryRefSchema,
  parentOwnedReceiptBoundaryRef: ReceiptBoundaryRefSchema,
  childDeviceTransportReceiptExpectationRef: ReceiptBoundaryRefSchema,
  runtimeDeliveryReceiptBoundaryState: ReceiptBoundaryStateSchema,
  childDeviceTransportReceiptState: ReceiptExpectationStateSchema,
  runtimeDeliveryReceiptReadinessState: ReceiptReadinessStateSchema,
  requiredReceiptArtifactBlockers: Schema.Array(ReceiptArtifactBlockerSchema),
  externalWriterDispatchExecutionProofRefs: Schema.Array(ReceiptBoundaryRefSchema),
  providerStoreExecutionReceiptProofRefs: Schema.Array(ReceiptBoundaryRefSchema),
  platformAdapterExecutionReceiptProofRefs: Schema.Array(ReceiptBoundaryRefSchema),
  childDeviceTransportReceiptProofRefs: Schema.Array(ReceiptBoundaryRefSchema),
  receiptBlockedReasonRefs: Schema.Array(ReceiptBoundaryRefSchema),
  receiptBoundaryAuditEventRefs: Schema.Array(ReceiptBoundaryAuditRefSchema),
  externalRuntimeWriterExecutionClaim: ReceiptExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ReceiptDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ReceiptDeliveryClaimSchema,
  providerApiExecutionClaim: ReceiptExecutionClaimSchema,
  storeIntegrationClaim: ReceiptIntegrationClaimSchema,
  platformInterceptionClaim: ReceiptIntegrationClaimSchema,
  platformAdapterClaim: ReceiptAdapterClaimSchema,
  childDeviceDeliveryClaim: ReceiptDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ReceiptDeliveryClaimSchema,
  appBlockingClaim: ReceiptIntegrationClaimSchema,
  childDataCustody: ReceiptCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ReceiptIntegrationClaimSchema,
  claimBoundary: ReceiptBoundaryClaimBoundarySchema,
  receiptBoundaryCheckedAt: ParentTimestampSchema,
});

type ReceiptBoundaryRowCandidate = Infer<typeof ReceiptBoundaryRowBaseSchema>;

export const AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema = withParser(
  ReceiptBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        receiptBoundaryRowIsHonest(row) ||
        'Expected runtime delivery receipt boundary rows to require execution and child receipt proof refs before any receipt claim'
    )
  )
);

const ReceiptBoundaryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchemaVersionSchema,
  sourceDispatchPreflightProofVersion: Schema.Literal(SourceDispatchPreflightProofVersion),
  runtimeDeliveryReceiptBoundaryRows: Schema.Array(AppInstallPurchaseRuntimeDeliveryReceiptBoundaryRowSchema),
  nonClaims: Schema.Array(ReceiptBoundaryNonClaimSchema),
  knownGaps: Schema.Array(ReceiptBoundaryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof = Infer<typeof ReceiptBoundaryProofBaseSchema>;

export const AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchema = withParser(
  ReceiptBoundaryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        receiptBoundaryProofIsHonest(proof) ||
        'Expected app install/purchase runtime delivery receipt boundary proof to preserve missing receipt blockers and non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeDeliveryReceiptBoundaryKnownGaps = [
  'Receipt boundary rows are parent-owned proof rows only; no external writer dispatch executor, provider/store execution receipt, platform adapter execution receipt, or child-device transport receipt exists.',
  'All receipt rows remain blocked or manual-required until external writer dispatch execution, provider/store execution receipt, platform adapter execution receipt, and child-device transport receipt proof refs become real artifacts.',
  'Product capability checklist, package export, and README deltas are intentionally not touched in this branch so E-C can finish the shared backend-runtime closure export/README/checklist work after PR531.',
] as const;

export const AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofReadModel =
  AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProofSchema.parse({
    schemaVersion: ReceiptBoundaryProofVersion,
    sourceDispatchPreflightProofVersion: SourceDispatchPreflightProofVersion,
    runtimeDeliveryReceiptBoundaryRows:
      AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel.externalRuntimeTransportDispatchPreflightRows.map(
        runtimeDeliveryReceiptBoundaryRow
      ),
    nonClaims: ReceiptBoundaryNonClaims,
    knownGaps: AppInstallPurchaseRuntimeDeliveryReceiptBoundaryKnownGaps,
    updatedAt: ReceiptBoundaryTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof(
  proof: AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof
) {
  return {
    runtimeDeliveryReceiptBoundaryRows: proof.runtimeDeliveryReceiptBoundaryRows.length,
    blockedReceiptRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.runtimeDeliveryReceiptBoundaryState === 'receipt-blocked-waiting-runtime-artifacts'
    ).length,
    manualRequiredRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.runtimeDeliveryReceiptBoundaryState === 'manual-required'
    ).length,
    receiptMissingRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.childDeviceTransportReceiptState === 'receipt-missing'
    ).length,
    readyReceiptRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) =>
        row.runtimeDeliveryReceiptReadinessState !== 'not-ready' &&
        row.runtimeDeliveryReceiptReadinessState !== 'manual-required'
    ).length,
    childDeviceDeliveredRows: proof.runtimeDeliveryReceiptBoundaryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function runtimeDeliveryReceiptBoundaryRow(
  row: (typeof AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel.externalRuntimeTransportDispatchPreflightRows)[number]
) {
  const manual = row.dispatchPreflightState === 'manual-required';
  return {
    schemaVersion: ReceiptBoundaryProofVersion,
    runtimeDeliveryReceiptBoundaryRowId: `runtime-delivery-receipt-boundary-${row.sourceDecisionAction}`,
    sourceDispatchPreflightProofVersion: SourceDispatchPreflightProofVersion,
    sourceDispatchPreflightRowId: row.externalRuntimeTransportDispatchPreflightRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceDispatchPreflightState: row.dispatchPreflightState,
    sourceDispatchPacketState: row.dispatchPacketState,
    sourceParentOwnedDispatchPacketRef: row.parentOwnedDispatchPacketRef,
    parentOwnedReceiptBoundaryRef: `parent-owned-runtime-delivery-receipt-boundary-${row.sourceDecisionAction}`,
    childDeviceTransportReceiptExpectationRef: `child-device-transport-receipt-required-${row.sourceDecisionAction}`,
    runtimeDeliveryReceiptBoundaryState: manual ? 'manual-required' : 'receipt-blocked-waiting-runtime-artifacts',
    childDeviceTransportReceiptState: manual ? 'manual-required' : 'receipt-missing',
    runtimeDeliveryReceiptReadinessState: manual ? 'manual-required' : 'not-ready',
    requiredReceiptArtifactBlockers: ReceiptArtifactBlockers,
    externalWriterDispatchExecutionProofRefs: row.externalWriterTransportHandlerProofRefs,
    providerStoreExecutionReceiptProofRefs: row.providerStoreExecutionHandlerProofRefs,
    platformAdapterExecutionReceiptProofRefs: row.platformAdapterExecutionHandlerProofRefs,
    childDeviceTransportReceiptProofRefs: row.childDeviceTransportReceiptProofRefs,
    receiptBlockedReasonRefs: [
      `missing-external-writer-dispatch-execution-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-receipt-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-receipt-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    receiptBoundaryAuditEventRefs: [
      ...row.dispatchPreflightAuditEventRefs,
      `runtime-delivery-receipt-boundary-audit-${row.sourceDecisionAction}`,
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
    claimBoundary: ReceiptBoundary,
    receiptBoundaryCheckedAt: ReceiptBoundaryTimestamp,
  } as const;
}

function receiptBoundaryRowIsHonest(row: ReceiptBoundaryRowCandidate): boolean {
  return (
    receiptBoundaryStatesMatchSource(row) &&
    receiptBoundaryRefsAreComplete(row) &&
    receiptBoundaryClaimsStayUnimplemented(row) &&
    receiptBoundaryIsExplicit(row.claimBoundary)
  );
}

function receiptBoundaryStatesMatchSource(row: ReceiptBoundaryRowCandidate): boolean {
  if (row.sourceDispatchPreflightState === 'manual-required') {
    return (
      row.sourceDispatchPacketState === 'manual-required' &&
      row.runtimeDeliveryReceiptBoundaryState === 'manual-required' &&
      row.childDeviceTransportReceiptState === 'manual-required' &&
      row.runtimeDeliveryReceiptReadinessState === 'manual-required'
    );
  }
  return (
    row.sourceDispatchPacketState === 'withheld' &&
    row.runtimeDeliveryReceiptBoundaryState === 'receipt-blocked-waiting-runtime-artifacts' &&
    row.childDeviceTransportReceiptState === 'receipt-missing' &&
    row.runtimeDeliveryReceiptReadinessState === 'not-ready'
  );
}

function receiptBoundaryRefsAreComplete(row: ReceiptBoundaryRowCandidate): boolean {
  return (
    row.sourceDispatchPreflightProofVersion === SourceDispatchPreflightProofVersion &&
    row.sourceDispatchPreflightRowId.length > 0 &&
    row.sourceParentOwnedDispatchPacketRef.length > 0 &&
    row.parentOwnedReceiptBoundaryRef.length > 0 &&
    row.childDeviceTransportReceiptExpectationRef.length > 0 &&
    ReceiptArtifactBlockers.every((blocker) => row.requiredReceiptArtifactBlockers.includes(blocker)) &&
    row.externalWriterDispatchExecutionProofRefs.length > 0 &&
    row.providerStoreExecutionReceiptProofRefs.length > 0 &&
    row.platformAdapterExecutionReceiptProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.receiptBlockedReasonRefs.length === ReceiptArtifactBlockers.length &&
    row.receiptBoundaryAuditEventRefs.length > 0
  );
}

function receiptBoundaryClaimsStayUnimplemented(row: ReceiptBoundaryRowCandidate): boolean {
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

function receiptBoundaryProofIsHonest(proof: AppInstallPurchaseRuntimeDeliveryReceiptBoundaryProof): boolean {
  const actions = new Set(proof.runtimeDeliveryReceiptBoundaryRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceDispatchPreflightProofVersion === SourceDispatchPreflightProofVersion &&
    proof.runtimeDeliveryReceiptBoundaryRows.length === ReceiptBoundaryActions.length &&
    ReceiptBoundaryActions.every((action) => actions.has(action)) &&
    ReceiptBoundaryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.runtimeDeliveryReceiptBoundaryRows.every(receiptBoundaryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function receiptBoundaryIsExplicit(boundary: typeof ReceiptBoundaryClaimBoundarySchema.Type): boolean {
  return ReceiptBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
