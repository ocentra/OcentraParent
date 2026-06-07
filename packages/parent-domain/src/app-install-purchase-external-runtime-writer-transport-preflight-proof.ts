import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel } from './app-install-purchase-external-runtime-writer-readiness-proof';
import { ParentTimestampSchema } from './reference-primitives';

const ExternalRuntimeWriterTransportPreflightText = Schema.String.pipe(Schema.minLength(1));
const ExternalRuntimeWriterTransportPreflightProofVersion =
  'app-install-purchase-external-runtime-writer-transport-preflight-proof';
const SourceExternalRuntimeWriterReadinessProofVersion = 'app-install-purchase-external-runtime-writer-readiness-proof';
const ExternalRuntimeWriterTransportPreflightTimestamp = '2026-06-07T11:48:00.000Z';
const ExternalRuntimeWriterTransportPreflightBoundary =
  'external runtime writer transport preflight proof only; parent-owned external writer transport, queue, child-device transport, platform adapter, and provider-store proof refs are required before delivery no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeWriterTransportPreflightActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceExternalRuntimeWriterReadinessStates = ['writer-handoff-ready', 'manual-required'] as const;
const SourceExternalRuntimeWriterQueueStates = ['queue-preflight-ready', 'manual-required'] as const;
const ExternalRuntimeWriterTransportPreflightStates = ['transport-preflight-ready', 'manual-required'] as const;
const ExternalRuntimeWriterTransportChannelStates = ['parent-owned-queue-ref-ready', 'manual-required'] as const;
const ExternalRuntimeWriterTransportPreflightNonClaims = [
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
const ExternalRuntimeWriterTransportPreflightBoundaryFragments = [
  'parent-owned external writer transport',
  'queue',
  'child-device transport',
  'platform adapter',
  'provider-store proof refs',
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

export const AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeWriterTransportPreflightProofVersion)
);
const ExternalRuntimeWriterTransportPreflightActionSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterTransportPreflightActions)
);
const SourceExternalRuntimeWriterReadinessStateSchema = withParser(
  Schema.Literal(...SourceExternalRuntimeWriterReadinessStates)
);
const SourceExternalRuntimeWriterQueueStateSchema = withParser(
  Schema.Literal(...SourceExternalRuntimeWriterQueueStates)
);
const ExternalRuntimeWriterTransportPreflightStateSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterTransportPreflightStates)
);
const ExternalRuntimeWriterTransportChannelStateSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterTransportChannelStates)
);
const ExternalRuntimeWriterTransportExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeWriterTransportDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeWriterTransportIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeWriterTransportAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeWriterTransportCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ExternalRuntimeWriterTransportPreflightNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterTransportPreflightNonClaims)
);

const ExternalRuntimeWriterTransportPreflightRowIdSchema = ExternalRuntimeWriterTransportPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeWriterTransportPreflightRowId')
);
const ExternalRuntimeWriterTransportPreflightRefSchema = ExternalRuntimeWriterTransportPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeWriterTransportPreflightRef')
);
const ExternalRuntimeWriterTransportPreflightAuditRefSchema = ExternalRuntimeWriterTransportPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeWriterTransportPreflightAuditRef')
);
const ExternalRuntimeWriterTransportPreflightBoundarySchema = ExternalRuntimeWriterTransportPreflightText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeWriterTransportPreflightBoundary')
);

const ExternalRuntimeWriterTransportPreflightRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchemaVersionSchema,
  externalRuntimeWriterTransportPreflightRowId: ExternalRuntimeWriterTransportPreflightRowIdSchema,
  sourceExternalRuntimeWriterReadinessProofVersion: Schema.Literal(SourceExternalRuntimeWriterReadinessProofVersion),
  sourceExternalRuntimeWriterReadinessRowId: ExternalRuntimeWriterTransportPreflightRefSchema,
  sourceDecisionAction: ExternalRuntimeWriterTransportPreflightActionSchema,
  sourceExternalRuntimeWriterReadinessState: SourceExternalRuntimeWriterReadinessStateSchema,
  sourceExternalRuntimeWriterQueueState: SourceExternalRuntimeWriterQueueStateSchema,
  sourceExternalRuntimeWriterPreflightRef: ExternalRuntimeWriterTransportPreflightRefSchema,
  sourceExternalRuntimeWriterReceiptRef: ExternalRuntimeWriterTransportPreflightRefSchema,
  sourceExternalRuntimeWriterTargetRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  externalRuntimeWriterTransportPreflightState: ExternalRuntimeWriterTransportPreflightStateSchema,
  externalRuntimeWriterTransportChannelState: ExternalRuntimeWriterTransportChannelStateSchema,
  externalRuntimeWriterTransportPreflightRef: ExternalRuntimeWriterTransportPreflightRefSchema,
  requiredExternalWriterTransportProofRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  requiredExternalWriterQueueProofRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  requiredChildDeviceTransportProofRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  requiredPlatformAdapterProofRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  requiredProviderStoreProofRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  externalRuntimeWriterAuditEventRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightAuditRefSchema),
  childDeliveryAuditEventRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeWriterTransportExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeWriterTransportDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeWriterTransportDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeWriterTransportExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeWriterTransportIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeWriterTransportIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeWriterTransportAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeWriterTransportDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeWriterTransportDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeWriterTransportIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeWriterTransportCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeWriterTransportIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeWriterTransportPreflightBoundarySchema,
  classifiedAt: ParentTimestampSchema,
});

type ExternalRuntimeWriterTransportPreflightRowCandidate = Infer<
  typeof ExternalRuntimeWriterTransportPreflightRowBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterTransportPreflightRowSchema = withParser(
  ExternalRuntimeWriterTransportPreflightRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeWriterTransportPreflightRowIsHonest(row) ||
        'Expected external runtime writer transport preflight rows to require transport, queue, platform, provider/store, and child-device proof refs without external writer delivery claims'
    )
  )
);

const ExternalRuntimeWriterTransportPreflightProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchemaVersionSchema,
  sourceExternalRuntimeWriterReadinessProofVersion: Schema.Literal(SourceExternalRuntimeWriterReadinessProofVersion),
  externalRuntimeWriterTransportPreflightRows: Schema.Array(
    AppInstallPurchaseExternalRuntimeWriterTransportPreflightRowSchema
  ),
  nonClaims: Schema.Array(ExternalRuntimeWriterTransportPreflightNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeWriterTransportPreflightRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeWriterTransportPreflightProof = Infer<
  typeof ExternalRuntimeWriterTransportPreflightProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchema = withParser(
  ExternalRuntimeWriterTransportPreflightProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeWriterTransportPreflightProofIsHonest(proof) ||
        'Expected app install/purchase external runtime writer transport preflight proof to cover parent actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeWriterTransportPreflightKnownGaps = [
  'Transport preflight rows require external writer transport, queue, platform adapter, provider/store, and child-device delivery proof refs before any external runtime writer delivery claim.',
  'No external writer process, transport execution, parent action runtime delivery, provider/store execution, platform adapter implementation, child-device delivery, runtime report delivery, app blocking, child activity data, or Ocentra-hosted family custody is implemented.',
  'Review-needed remains manual-required until portal approval UI, external runtime writer delivery, and child delivery transport exist.',
] as const;

export const AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofReadModel =
  AppInstallPurchaseExternalRuntimeWriterTransportPreflightProofSchema.parse({
    schemaVersion: ExternalRuntimeWriterTransportPreflightProofVersion,
    sourceExternalRuntimeWriterReadinessProofVersion: SourceExternalRuntimeWriterReadinessProofVersion,
    externalRuntimeWriterTransportPreflightRows:
      AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel.externalRuntimeWriterReadinessRows.map(
        externalRuntimeWriterTransportPreflightRow
      ),
    nonClaims: ExternalRuntimeWriterTransportPreflightNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeWriterTransportPreflightKnownGaps,
    updatedAt: ExternalRuntimeWriterTransportPreflightTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeWriterTransportPreflightProof(
  proof: AppInstallPurchaseExternalRuntimeWriterTransportPreflightProof
) {
  return {
    externalRuntimeWriterTransportPreflightRows: proof.externalRuntimeWriterTransportPreflightRows.length,
    transportPreflightReadyRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterTransportPreflightState === 'transport-preflight-ready'
    ).length,
    parentOwnedQueueRefReadyRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterTransportChannelState === 'parent-owned-queue-ref-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterTransportPreflightState === 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterTransportPreflightRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeWriterTransportPreflightRow(
  row: (typeof AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel.externalRuntimeWriterReadinessRows)[number]
) {
  const ready =
    row.externalRuntimeWriterReadinessState === 'writer-handoff-ready' &&
    row.externalRuntimeWriterQueueState === 'queue-preflight-ready';
  return {
    schemaVersion: ExternalRuntimeWriterTransportPreflightProofVersion,
    externalRuntimeWriterTransportPreflightRowId: `external-runtime-writer-transport-preflight-${row.sourceDecisionAction}`,
    sourceExternalRuntimeWriterReadinessProofVersion: SourceExternalRuntimeWriterReadinessProofVersion,
    sourceExternalRuntimeWriterReadinessRowId: row.externalRuntimeWriterReadinessRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeWriterReadinessState: row.externalRuntimeWriterReadinessState,
    sourceExternalRuntimeWriterQueueState: row.externalRuntimeWriterQueueState,
    sourceExternalRuntimeWriterPreflightRef: row.externalRuntimeWriterPreflightRef,
    sourceExternalRuntimeWriterReceiptRef: row.externalRuntimeWriterReceiptRef,
    sourceExternalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
    externalRuntimeWriterTransportPreflightState: ready ? 'transport-preflight-ready' : 'manual-required',
    externalRuntimeWriterTransportChannelState: ready ? 'parent-owned-queue-ref-ready' : 'manual-required',
    externalRuntimeWriterTransportPreflightRef: `external-runtime-writer-transport-preflight-ref-${row.sourceDecisionAction}`,
    requiredExternalWriterTransportProofRefs: [
      `external-runtime-writer-transport-proof-${row.sourceDecisionAction}`,
      row.externalRuntimeWriterPreflightRef,
    ],
    requiredExternalWriterQueueProofRefs: [
      `external-runtime-writer-queue-proof-${row.sourceDecisionAction}`,
      row.externalRuntimeWriterReceiptRef,
    ],
    requiredChildDeviceTransportProofRefs: [
      `external-runtime-writer-child-device-transport-proof-${row.sourceDecisionAction}`,
      ...row.childDeliveryAuditEventRefs,
    ],
    requiredPlatformAdapterProofRefs: [`external-runtime-writer-platform-adapter-proof-${row.sourceDecisionAction}`],
    requiredProviderStoreProofRefs: [`external-runtime-writer-provider-store-proof-${row.sourceDecisionAction}`],
    externalRuntimeWriterAuditEventRefs: row.externalRuntimeWriterAuditEventRefs,
    childDeliveryAuditEventRefs: row.childDeliveryAuditEventRefs,
    reportRuntimeRefs: row.reportRuntimeRefs,
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
    claimBoundary: ExternalRuntimeWriterTransportPreflightBoundary,
    classifiedAt: ExternalRuntimeWriterTransportPreflightTimestamp,
  } as const;
}

function externalRuntimeWriterTransportPreflightRowIsHonest(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
): boolean {
  return (
    externalRuntimeWriterTransportPreflightMatchesSource(row) &&
    externalRuntimeWriterTransportPreflightRefsAreComplete(row) &&
    externalRuntimeWriterTransportPreflightClaimsStayUnimplemented(row) &&
    externalRuntimeWriterTransportPreflightBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeWriterTransportPreflightMatchesSource(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
): boolean {
  if (
    row.sourceExternalRuntimeWriterReadinessState === 'manual-required' ||
    row.sourceExternalRuntimeWriterQueueState === 'manual-required'
  ) {
    return (
      row.externalRuntimeWriterTransportPreflightState === 'manual-required' &&
      row.externalRuntimeWriterTransportChannelState === 'manual-required'
    );
  }
  return (
    row.externalRuntimeWriterTransportPreflightState === 'transport-preflight-ready' &&
    row.externalRuntimeWriterTransportChannelState === 'parent-owned-queue-ref-ready'
  );
}

function externalRuntimeWriterTransportPreflightRefsAreComplete(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
): boolean {
  return (
    externalRuntimeWriterTransportPreflightSourceRefsAreComplete(row) &&
    externalRuntimeWriterTransportPreflightRequiredRefsAreComplete(row) &&
    externalRuntimeWriterTransportPreflightAuditRefsAreComplete(row)
  );
}

function externalRuntimeWriterTransportPreflightSourceRefsAreComplete(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
): boolean {
  return (
    row.sourceExternalRuntimeWriterReadinessProofVersion === SourceExternalRuntimeWriterReadinessProofVersion &&
    row.sourceExternalRuntimeWriterReadinessRowId.length > 0 &&
    row.sourceExternalRuntimeWriterPreflightRef.length > 0 &&
    row.sourceExternalRuntimeWriterReceiptRef.length > 0 &&
    row.sourceExternalRuntimeWriterTargetRefs.length > 0
  );
}

function externalRuntimeWriterTransportPreflightRequiredRefsAreComplete(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
): boolean {
  return (
    row.externalRuntimeWriterTransportPreflightRef.length > 0 &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredExternalWriterQueueProofRefs.length > 0 &&
    row.requiredChildDeviceTransportProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0
  );
}

function externalRuntimeWriterTransportPreflightAuditRefsAreComplete(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
): boolean {
  return (
    row.externalRuntimeWriterAuditEventRefs.length > 0 &&
    row.childDeliveryAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function externalRuntimeWriterTransportPreflightClaimsStayUnimplemented(
  row: ExternalRuntimeWriterTransportPreflightRowCandidate
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

function externalRuntimeWriterTransportPreflightProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeWriterTransportPreflightProof
): boolean {
  const actions = new Set(proof.externalRuntimeWriterTransportPreflightRows.map((row) => row.sourceDecisionAction));
  const preflightStates = new Set(
    proof.externalRuntimeWriterTransportPreflightRows.map((row) => row.externalRuntimeWriterTransportPreflightState)
  );
  const channelStates = new Set(
    proof.externalRuntimeWriterTransportPreflightRows.map((row) => row.externalRuntimeWriterTransportChannelState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeWriterReadinessProofVersion === SourceExternalRuntimeWriterReadinessProofVersion &&
    proof.externalRuntimeWriterTransportPreflightRows.length ===
      ExternalRuntimeWriterTransportPreflightActions.length &&
    ExternalRuntimeWriterTransportPreflightActions.every((action) => actions.has(action)) &&
    ExternalRuntimeWriterTransportPreflightStates.every((state) => preflightStates.has(state)) &&
    ExternalRuntimeWriterTransportChannelStates.every((state) => channelStates.has(state)) &&
    ExternalRuntimeWriterTransportPreflightNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeWriterTransportPreflightRows.every(externalRuntimeWriterTransportPreflightRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeWriterTransportPreflightBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeWriterTransportPreflightBoundarySchema.Type
) {
  return ExternalRuntimeWriterTransportPreflightBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
