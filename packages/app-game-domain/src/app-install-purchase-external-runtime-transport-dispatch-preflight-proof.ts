import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel } from './app-install-purchase-external-runtime-transport-queue-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const DispatchPreflightProofVersion = 'app-install-purchase-external-runtime-transport-dispatch-preflight-proof';
const SourceTransportQueueProofVersion = 'app-install-purchase-external-runtime-transport-queue-proof';
const DispatchPreflightTimestamp = '2026-06-07T17:52:00.000Z';
const DispatchPreflightText = Schema.String.pipe(Schema.minLength(1));
const DispatchPreflightBoundary =
  'external runtime transport dispatch preflight proof only; dispatch packets are parent-owned withheld rows and must not leave the parent queue until external writer transport handler provider-store execution handler platform adapter execution handler and child-device transport receipt refs are real no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const DispatchPreflightActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceQueueStates = ['queued-blocked', 'manual-required'] as const;
const SourceDispatchStates = ['dispatch-blocked', 'manual-required'] as const;
const DispatchPreflightStates = ['blocked-waiting-runtime-artifacts', 'manual-required'] as const;
const DispatchPacketStates = ['withheld', 'manual-required'] as const;
const DispatchReadinessStates = ['not-ready', 'manual-required'] as const;
const DispatchArtifactBlockers = [
  'external-writer-transport-handler-missing',
  'provider-store-execution-handler-missing',
  'platform-adapter-execution-handler-missing',
  'child-device-transport-receipt-missing',
] as const;
const DispatchPreflightNonClaims = [
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
const DispatchPreflightBoundaryFragments = [
  'dispatch packets are parent-owned withheld rows',
  'must not leave the parent queue',
  'external writer transport handler',
  'provider-store execution handler',
  'platform adapter execution handler',
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

export const AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchemaVersionSchema = withParser(
  Schema.Literal(DispatchPreflightProofVersion)
);
const DispatchPreflightActionSchema = withParser(Schema.Literal(...DispatchPreflightActions));
const SourceQueueStateSchema = withParser(Schema.Literal(...SourceQueueStates));
const SourceDispatchStateSchema = withParser(Schema.Literal(...SourceDispatchStates));
const DispatchPreflightStateSchema = withParser(Schema.Literal(...DispatchPreflightStates));
const DispatchPacketStateSchema = withParser(Schema.Literal(...DispatchPacketStates));
const DispatchReadinessStateSchema = withParser(Schema.Literal(...DispatchReadinessStates));
const DispatchArtifactBlockerSchema = withParser(Schema.Literal(...DispatchArtifactBlockers));
const DispatchPreflightNonClaimSchema = withParser(Schema.Literal(...DispatchPreflightNonClaims));
const DispatchExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const DispatchDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const DispatchIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const DispatchAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const DispatchCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));

const DispatchPreflightRowIdSchema = DispatchPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowId')
);
const DispatchPreflightRefSchema = DispatchPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRef')
);
const DispatchPreflightAuditRefSchema = DispatchPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeTransportDispatchPreflightAuditRef')
);
const DispatchPreflightBoundarySchema = DispatchPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeTransportDispatchPreflightBoundary')
);

const DispatchPreflightRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchemaVersionSchema,
  externalRuntimeTransportDispatchPreflightRowId: DispatchPreflightRowIdSchema,
  sourceExternalRuntimeTransportQueueProofVersion: Schema.Literal(SourceTransportQueueProofVersion),
  sourceExternalRuntimeTransportQueueRowId: DispatchPreflightRefSchema,
  sourceDecisionAction: DispatchPreflightActionSchema,
  sourceTransportQueueState: SourceQueueStateSchema,
  sourceTransportDispatchState: SourceDispatchStateSchema,
  parentOwnedTransportQueueRef: DispatchPreflightRefSchema,
  parentOwnedDispatchPreflightRef: DispatchPreflightRefSchema,
  parentOwnedDispatchPacketRef: DispatchPreflightRefSchema,
  dispatchPreflightState: DispatchPreflightStateSchema,
  dispatchPacketState: DispatchPacketStateSchema,
  dispatchReadinessState: DispatchReadinessStateSchema,
  requiredDispatchArtifactBlockers: Schema.Array(DispatchArtifactBlockerSchema),
  externalWriterTransportHandlerProofRefs: Schema.Array(DispatchPreflightRefSchema),
  providerStoreExecutionHandlerProofRefs: Schema.Array(DispatchPreflightRefSchema),
  platformAdapterExecutionHandlerProofRefs: Schema.Array(DispatchPreflightRefSchema),
  childDeviceTransportReceiptProofRefs: Schema.Array(DispatchPreflightRefSchema),
  dispatchBlockedReasonRefs: Schema.Array(DispatchPreflightRefSchema),
  dispatchPreflightAuditEventRefs: Schema.Array(DispatchPreflightAuditRefSchema),
  externalRuntimeWriterExecutionClaim: DispatchExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: DispatchDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: DispatchDeliveryClaimSchema,
  providerApiExecutionClaim: DispatchExecutionClaimSchema,
  storeIntegrationClaim: DispatchIntegrationClaimSchema,
  platformInterceptionClaim: DispatchIntegrationClaimSchema,
  platformAdapterClaim: DispatchAdapterClaimSchema,
  childDeviceDeliveryClaim: DispatchDeliveryClaimSchema,
  runtimeReportDeliveryClaim: DispatchDeliveryClaimSchema,
  appBlockingClaim: DispatchIntegrationClaimSchema,
  childDataCustody: DispatchCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: DispatchIntegrationClaimSchema,
  claimBoundary: DispatchPreflightBoundarySchema,
  preflightedAt: ParentTimestampSchema,
});

type DispatchPreflightRowCandidate = Infer<typeof DispatchPreflightRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema = withParser(
  DispatchPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        dispatchPreflightRowIsHonest(row) ||
        'Expected external runtime transport dispatch preflight rows to keep packets withheld until real runtime artifacts exist'
    )
  )
);

const DispatchPreflightProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchemaVersionSchema,
  sourceExternalRuntimeTransportQueueProofVersion: Schema.Literal(SourceTransportQueueProofVersion),
  externalRuntimeTransportDispatchPreflightRows: Schema.Array(
    AppInstallPurchaseExternalRuntimeTransportDispatchPreflightRowSchema
  ),
  nonClaims: Schema.Array(DispatchPreflightNonClaimSchema),
  knownGaps: Schema.Array(DispatchPreflightRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProof = Infer<
  typeof DispatchPreflightProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchema = withParser(
  DispatchPreflightProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        dispatchPreflightProofIsHonest(proof) ||
        'Expected app install/purchase external runtime transport dispatch preflight proof to preserve withheld packets and non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeTransportDispatchPreflightKnownGaps = [
  'Dispatch preflight rows are parent-owned withheld packets only; no external writer transport handler, provider/store execution handler, platform adapter execution handler, or child-device transport receipt exists.',
  'All dispatch packets remain withheld until external writer transport handler, provider/store execution handler, platform adapter execution handler, and child-device transport receipt proof refs become real artifacts.',
  'Product capability checklist row update is deferred while the checklist path is locked; pending delta is to mention this dispatch preflight proof as a non-claim runtime delivery guard for the Install/purchase approval row.',
] as const;

export const AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofReadModel =
  AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProofSchema.parse({
    schemaVersion: DispatchPreflightProofVersion,
    sourceExternalRuntimeTransportQueueProofVersion: SourceTransportQueueProofVersion,
    externalRuntimeTransportDispatchPreflightRows:
      AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel.externalRuntimeTransportQueueRows.map(
        externalRuntimeTransportDispatchPreflightRow
      ),
    nonClaims: DispatchPreflightNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeTransportDispatchPreflightKnownGaps,
    updatedAt: DispatchPreflightTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeTransportDispatchPreflightProof(
  proof: AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProof
) {
  return {
    externalRuntimeTransportDispatchPreflightRows: proof.externalRuntimeTransportDispatchPreflightRows.length,
    blockedPreflightRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchPreflightState === 'blocked-waiting-runtime-artifacts'
    ).length,
    manualRequiredRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchPreflightState === 'manual-required'
    ).length,
    withheldDispatchPackets: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchPacketState === 'withheld'
    ).length,
    readyDispatchRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.dispatchReadinessState !== 'not-ready' && row.dispatchReadinessState !== 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeTransportDispatchPreflightRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeTransportDispatchPreflightRow(
  row: (typeof AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel.externalRuntimeTransportQueueRows)[number]
) {
  const manual = row.externalRuntimeTransportQueueState === 'manual-required';
  return {
    schemaVersion: DispatchPreflightProofVersion,
    externalRuntimeTransportDispatchPreflightRowId: `external-runtime-transport-dispatch-preflight-${row.sourceDecisionAction}`,
    sourceExternalRuntimeTransportQueueProofVersion: SourceTransportQueueProofVersion,
    sourceExternalRuntimeTransportQueueRowId: row.externalRuntimeTransportQueueRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceTransportQueueState: row.externalRuntimeTransportQueueState,
    sourceTransportDispatchState: row.externalRuntimeTransportDispatchState,
    parentOwnedTransportQueueRef: row.parentOwnedTransportQueueRef,
    parentOwnedDispatchPreflightRef: `parent-owned-external-runtime-dispatch-preflight-${row.sourceDecisionAction}`,
    parentOwnedDispatchPacketRef: `parent-owned-external-runtime-dispatch-packet-${row.sourceDecisionAction}`,
    dispatchPreflightState: manual ? 'manual-required' : 'blocked-waiting-runtime-artifacts',
    dispatchPacketState: manual ? 'manual-required' : 'withheld',
    dispatchReadinessState: manual ? 'manual-required' : 'not-ready',
    requiredDispatchArtifactBlockers: DispatchArtifactBlockers,
    externalWriterTransportHandlerProofRefs: row.requiredExternalWriterTransportProofRefs,
    providerStoreExecutionHandlerProofRefs: row.requiredProviderStoreProofRefs,
    platformAdapterExecutionHandlerProofRefs: row.requiredPlatformAdapterProofRefs,
    childDeviceTransportReceiptProofRefs: row.requiredChildDeviceTransportProofRefs,
    dispatchBlockedReasonRefs: [
      `missing-external-writer-transport-handler-${row.sourceDecisionAction}`,
      `missing-provider-store-execution-handler-${row.sourceDecisionAction}`,
      `missing-platform-adapter-execution-handler-${row.sourceDecisionAction}`,
      `missing-child-device-transport-receipt-${row.sourceDecisionAction}`,
    ],
    dispatchPreflightAuditEventRefs: [
      ...row.queueGuardAuditEventRefs,
      `external-runtime-transport-dispatch-preflight-audit-${row.sourceDecisionAction}`,
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
    claimBoundary: DispatchPreflightBoundary,
    preflightedAt: DispatchPreflightTimestamp,
  } as const;
}

function dispatchPreflightRowIsHonest(row: DispatchPreflightRowCandidate): boolean {
  return (
    dispatchPreflightStatesMatchSource(row) &&
    dispatchPreflightRefsAreComplete(row) &&
    dispatchPreflightClaimsStayUnimplemented(row) &&
    dispatchPreflightBoundaryIsExplicit(row.claimBoundary)
  );
}

function dispatchPreflightStatesMatchSource(row: DispatchPreflightRowCandidate): boolean {
  if (row.sourceTransportQueueState === 'manual-required') {
    return (
      row.sourceTransportDispatchState === 'manual-required' &&
      row.dispatchPreflightState === 'manual-required' &&
      row.dispatchPacketState === 'manual-required' &&
      row.dispatchReadinessState === 'manual-required'
    );
  }
  return (
    row.sourceTransportDispatchState === 'dispatch-blocked' &&
    row.dispatchPreflightState === 'blocked-waiting-runtime-artifacts' &&
    row.dispatchPacketState === 'withheld' &&
    row.dispatchReadinessState === 'not-ready'
  );
}

function dispatchPreflightRefsAreComplete(row: DispatchPreflightRowCandidate): boolean {
  return (
    row.sourceExternalRuntimeTransportQueueProofVersion === SourceTransportQueueProofVersion &&
    row.sourceExternalRuntimeTransportQueueRowId.length > 0 &&
    row.parentOwnedTransportQueueRef.length > 0 &&
    row.parentOwnedDispatchPreflightRef.length > 0 &&
    row.parentOwnedDispatchPacketRef.length > 0 &&
    DispatchArtifactBlockers.every((blocker) => row.requiredDispatchArtifactBlockers.includes(blocker)) &&
    row.externalWriterTransportHandlerProofRefs.length > 0 &&
    row.providerStoreExecutionHandlerProofRefs.length > 0 &&
    row.platformAdapterExecutionHandlerProofRefs.length > 0 &&
    row.childDeviceTransportReceiptProofRefs.length > 0 &&
    row.dispatchBlockedReasonRefs.length === DispatchArtifactBlockers.length &&
    row.dispatchPreflightAuditEventRefs.length > 0
  );
}

function dispatchPreflightClaimsStayUnimplemented(row: DispatchPreflightRowCandidate): boolean {
  return (
    runtimeClaimsStayUnimplemented(row) && platformClaimsStayUnimplemented(row) && custodyClaimsStayUnimplemented(row)
  );
}

function runtimeClaimsStayUnimplemented(row: DispatchPreflightRowCandidate): boolean {
  return (
    row.externalRuntimeWriterExecutionClaim === 'not-executed' &&
    row.externalRuntimeWriterDeliveryClaim === 'not-delivered' &&
    row.parentActionRuntimeDeliveryClaim === 'not-delivered' &&
    row.providerApiExecutionClaim === 'not-executed' &&
    row.childDeviceDeliveryClaim === 'not-delivered' &&
    row.runtimeReportDeliveryClaim === 'not-delivered'
  );
}

function platformClaimsStayUnimplemented(row: DispatchPreflightRowCandidate): boolean {
  return (
    row.storeIntegrationClaim === 'not-claimed' &&
    row.platformInterceptionClaim === 'not-claimed' &&
    row.platformAdapterClaim === 'not-implemented' &&
    row.appBlockingClaim === 'not-claimed'
  );
}

function custodyClaimsStayUnimplemented(row: DispatchPreflightRowCandidate): boolean {
  return row.childDataCustody === 'no-child-activity-data' && row.ocentraHostedFamilyDataCustodyClaim === 'not-claimed';
}

function dispatchPreflightProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeTransportDispatchPreflightProof
): boolean {
  const actions = new Set(proof.externalRuntimeTransportDispatchPreflightRows.map((row) => row.sourceDecisionAction));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeTransportQueueProofVersion === SourceTransportQueueProofVersion &&
    proof.externalRuntimeTransportDispatchPreflightRows.length === DispatchPreflightActions.length &&
    DispatchPreflightActions.every((action) => actions.has(action)) &&
    DispatchPreflightNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeTransportDispatchPreflightRows.every(dispatchPreflightRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function dispatchPreflightBoundaryIsExplicit(boundary: typeof DispatchPreflightBoundarySchema.Type): boolean {
  return DispatchPreflightBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
