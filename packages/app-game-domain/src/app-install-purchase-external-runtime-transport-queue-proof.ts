import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel } from './app-install-purchase-external-runtime-writer-delivery-blocker-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const ExternalRuntimeTransportQueueProofVersion = 'app-install-purchase-external-runtime-transport-queue-proof';
const SourceExternalRuntimeWriterDeliveryBlockerProofVersion =
  'app-install-purchase-external-runtime-writer-delivery-blocker-proof';
const ExternalRuntimeTransportQueueTimestamp = '2026-06-07T17:24:00.000Z';
const ExternalRuntimeTransportQueueBoundary =
  'external runtime transport queue proof only; queue rows are parent-owned dispatch guard entries and must not dispatch until external writer transport child-device transport provider-store execution and platform adapter proof refs are real no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeTransportQueueActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceDeliveryBlockerStates = ['blocked-runtime-prerequisites-missing', 'manual-required'] as const;
const SourceDeliveryAttemptStates = ['not-started'] as const;
const ExternalRuntimeTransportQueueStates = ['queued-blocked', 'manual-required'] as const;
const ExternalRuntimeTransportDispatchStates = ['dispatch-blocked', 'manual-required'] as const;
const ExternalRuntimeTransportRetryStates = ['not-scheduled', 'manual-required'] as const;
const ExternalRuntimeTransportQueueNonClaims = [
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
const ExternalRuntimeTransportQueueRequiredBlockers = [
  'external-writer-transport-proof-missing',
  'platform-adapter-proof-missing',
  'provider-store-execution-proof-missing',
  'child-device-transport-proof-missing',
] as const;
const ExternalRuntimeTransportQueueBoundaryFragments = [
  'parent-owned dispatch guard entries',
  'must not dispatch',
  'external writer transport',
  'child-device transport',
  'provider-store execution',
  'platform adapter proof refs',
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

export const AppInstallPurchaseExternalRuntimeTransportQueueProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeTransportQueueProofVersion)
);
const ExternalRuntimeTransportQueueActionSchema = withParser(Schema.Literal(...ExternalRuntimeTransportQueueActions));
const SourceDeliveryBlockerStateSchema = withParser(Schema.Literal(...SourceDeliveryBlockerStates));
const SourceDeliveryAttemptStateSchema = withParser(Schema.Literal(...SourceDeliveryAttemptStates));
const ExternalRuntimeTransportQueueStateSchema = withParser(Schema.Literal(...ExternalRuntimeTransportQueueStates));
const ExternalRuntimeTransportDispatchStateSchema = withParser(
  Schema.Literal(...ExternalRuntimeTransportDispatchStates)
);
const ExternalRuntimeTransportRetryStateSchema = withParser(Schema.Literal(...ExternalRuntimeTransportRetryStates));
const ExternalRuntimeTransportExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeTransportDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeTransportIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeTransportAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeTransportCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ExternalRuntimeTransportQueueNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeTransportQueueNonClaims)
);
const ExternalRuntimeTransportQueueRequiredBlockerSchema = withParser(
  Schema.Literal(...ExternalRuntimeTransportQueueRequiredBlockers)
);

const ExternalRuntimeTransportQueueRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeTransportQueueRowId');
const ExternalRuntimeTransportQueueRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeTransportQueueRef');
const ExternalRuntimeTransportQueueAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeTransportQueueAuditRef');
const ExternalRuntimeTransportQueueBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeTransportQueueBoundary');

const ExternalRuntimeTransportQueueRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeTransportQueueProofSchemaVersionSchema,
  externalRuntimeTransportQueueRowId: ExternalRuntimeTransportQueueRowIdSchema,
  sourceExternalRuntimeWriterDeliveryBlockerProofVersion: Schema.Literal(
    SourceExternalRuntimeWriterDeliveryBlockerProofVersion
  ),
  sourceExternalRuntimeWriterDeliveryBlockerRowId: ExternalRuntimeTransportQueueRefSchema,
  sourceDecisionAction: ExternalRuntimeTransportQueueActionSchema,
  sourceDeliveryBlockerState: SourceDeliveryBlockerStateSchema,
  sourceDeliveryAttemptState: SourceDeliveryAttemptStateSchema,
  sourceExternalRuntimeWriterQueueRef: ExternalRuntimeTransportQueueRefSchema,
  externalRuntimeTransportQueueState: ExternalRuntimeTransportQueueStateSchema,
  externalRuntimeTransportDispatchState: ExternalRuntimeTransportDispatchStateSchema,
  externalRuntimeTransportRetryState: ExternalRuntimeTransportRetryStateSchema,
  parentOwnedTransportQueueRef: ExternalRuntimeTransportQueueRefSchema,
  queueGuardAuditEventRefs: Schema.Array(ExternalRuntimeTransportQueueAuditRefSchema),
  requiredRuntimeBlockers: Schema.Array(ExternalRuntimeTransportQueueRequiredBlockerSchema),
  requiredExternalWriterTransportProofRefs: Schema.Array(ExternalRuntimeTransportQueueRefSchema),
  requiredChildDeviceTransportProofRefs: Schema.Array(ExternalRuntimeTransportQueueRefSchema),
  requiredProviderStoreProofRefs: Schema.Array(ExternalRuntimeTransportQueueRefSchema),
  requiredPlatformAdapterProofRefs: Schema.Array(ExternalRuntimeTransportQueueRefSchema),
  blockedDispatchReasonRefs: Schema.Array(ExternalRuntimeTransportQueueRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeTransportExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeTransportDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeTransportDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeTransportExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeTransportIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeTransportIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeTransportAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeTransportDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeTransportDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeTransportIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeTransportCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeTransportIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeTransportQueueBoundarySchema,
  queuedAt: ParentTimestampSchema,
});

type ExternalRuntimeTransportQueueRowCandidate = Infer<typeof ExternalRuntimeTransportQueueRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeTransportQueueRowSchema = withParser(
  ExternalRuntimeTransportQueueRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeTransportQueueRowIsHonest(row) ||
        'Expected external runtime transport queue rows to block dispatch until real transport provider platform and child-device proof refs exist'
    )
  )
);

const ExternalRuntimeTransportQueueProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeTransportQueueProofSchemaVersionSchema,
  sourceExternalRuntimeWriterDeliveryBlockerProofVersion: Schema.Literal(
    SourceExternalRuntimeWriterDeliveryBlockerProofVersion
  ),
  externalRuntimeTransportQueueRows: Schema.Array(AppInstallPurchaseExternalRuntimeTransportQueueRowSchema),
  nonClaims: Schema.Array(ExternalRuntimeTransportQueueNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeTransportQueueRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeTransportQueueProof = Infer<
  typeof ExternalRuntimeTransportQueueProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeTransportQueueProofSchema = withParser(
  ExternalRuntimeTransportQueueProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeTransportQueueProofIsHonest(proof) ||
        'Expected app install/purchase external runtime transport queue proof to preserve dispatch blockers and non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeTransportQueueKnownGaps = [
  'Transport queue rows are parent-owned dispatch guard entries only; no external writer transport process, queue worker, retry worker, child-device transport, provider/store execution, or platform adapter execution is implemented.',
  'All dispatch stays blocked until external writer transport, provider/store execution, platform adapter execution, and child-device transport proof refs become real artifacts.',
  'Product capability checklist row update is deferred while E-C owns docs/product-capability-checklist.md; pending delta is to mention this transport queue proof as a runtime-oriented non-claim dispatch guard for the Install/purchase approval row.',
] as const;

export const AppInstallPurchaseExternalRuntimeTransportQueueProofReadModel =
  AppInstallPurchaseExternalRuntimeTransportQueueProofSchema.parse({
    schemaVersion: ExternalRuntimeTransportQueueProofVersion,
    sourceExternalRuntimeWriterDeliveryBlockerProofVersion: SourceExternalRuntimeWriterDeliveryBlockerProofVersion,
    externalRuntimeTransportQueueRows:
      AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel.externalRuntimeWriterDeliveryBlockerRows.map(
        externalRuntimeTransportQueueRow
      ),
    nonClaims: ExternalRuntimeTransportQueueNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeTransportQueueKnownGaps,
    updatedAt: ExternalRuntimeTransportQueueTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeTransportQueueProof(
  proof: AppInstallPurchaseExternalRuntimeTransportQueueProof
) {
  return {
    externalRuntimeTransportQueueRows: proof.externalRuntimeTransportQueueRows.length,
    queuedBlockedRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportQueueState === 'queued-blocked'
    ).length,
    manualRequiredRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportQueueState === 'manual-required'
    ).length,
    dispatchBlockedRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportDispatchState === 'dispatch-blocked'
    ).length,
    retryScheduledRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeTransportRetryState !== 'not-scheduled'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeTransportQueueRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeTransportQueueRow(
  row: (typeof AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel.externalRuntimeWriterDeliveryBlockerRows)[number]
) {
  const manual = row.deliveryBlockerState === 'manual-required';
  return {
    schemaVersion: ExternalRuntimeTransportQueueProofVersion,
    externalRuntimeTransportQueueRowId: `external-runtime-transport-queue-${row.sourceDecisionAction}`,
    sourceExternalRuntimeWriterDeliveryBlockerProofVersion: SourceExternalRuntimeWriterDeliveryBlockerProofVersion,
    sourceExternalRuntimeWriterDeliveryBlockerRowId: row.externalRuntimeWriterDeliveryBlockerRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceDeliveryBlockerState: row.deliveryBlockerState,
    sourceDeliveryAttemptState: row.deliveryAttemptState,
    sourceExternalRuntimeWriterQueueRef: row.sourceExternalRuntimeWriterQueueRef,
    externalRuntimeTransportQueueState: manual ? 'manual-required' : 'queued-blocked',
    externalRuntimeTransportDispatchState: manual ? 'manual-required' : 'dispatch-blocked',
    externalRuntimeTransportRetryState: manual ? 'manual-required' : 'not-scheduled',
    parentOwnedTransportQueueRef: `parent-owned-external-runtime-transport-queue-${row.sourceDecisionAction}`,
    queueGuardAuditEventRefs: uniqueRefs([
      ...row.deliveryBlockerAuditEventRefs,
      `external-runtime-transport-queue-guard-audit-${row.sourceDecisionAction}`,
    ]),
    requiredRuntimeBlockers: row.requiredRuntimeBlockers,
    requiredExternalWriterTransportProofRefs: row.requiredExternalWriterTransportProofRefs,
    requiredChildDeviceTransportProofRefs: row.requiredChildDeviceDeliveryProofRefs,
    requiredProviderStoreProofRefs: row.requiredProviderStoreProofRefs,
    requiredPlatformAdapterProofRefs: row.requiredPlatformAdapterProofRefs,
    blockedDispatchReasonRefs: row.manualBlockerRefs,
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
    claimBoundary: ExternalRuntimeTransportQueueBoundary,
    queuedAt: ExternalRuntimeTransportQueueTimestamp,
  } as const;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function externalRuntimeTransportQueueRowIsHonest(row: ExternalRuntimeTransportQueueRowCandidate): boolean {
  return (
    externalRuntimeTransportQueueMatchesSource(row) &&
    externalRuntimeTransportQueueRefsAreComplete(row) &&
    externalRuntimeTransportQueueClaimsStayUnimplemented(row) &&
    externalRuntimeTransportQueueBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeTransportQueueMatchesSource(row: ExternalRuntimeTransportQueueRowCandidate): boolean {
  if (row.sourceDeliveryBlockerState === 'manual-required') {
    return (
      row.externalRuntimeTransportQueueState === 'manual-required' &&
      row.externalRuntimeTransportDispatchState === 'manual-required' &&
      row.externalRuntimeTransportRetryState === 'manual-required'
    );
  }
  return (
    row.externalRuntimeTransportQueueState === 'queued-blocked' &&
    row.externalRuntimeTransportDispatchState === 'dispatch-blocked' &&
    row.externalRuntimeTransportRetryState === 'not-scheduled'
  );
}

function externalRuntimeTransportQueueRefsAreComplete(row: ExternalRuntimeTransportQueueRowCandidate): boolean {
  return (
    row.sourceExternalRuntimeWriterDeliveryBlockerProofVersion ===
      SourceExternalRuntimeWriterDeliveryBlockerProofVersion &&
    row.sourceExternalRuntimeWriterDeliveryBlockerRowId.length > 0 &&
    row.sourceDeliveryAttemptState === 'not-started' &&
    row.parentOwnedTransportQueueRef.length > 0 &&
    row.queueGuardAuditEventRefs.length > 0 &&
    ExternalRuntimeTransportQueueRequiredBlockers.every((blocker) => row.requiredRuntimeBlockers.includes(blocker)) &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredChildDeviceTransportProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.blockedDispatchReasonRefs.length === ExternalRuntimeTransportQueueRequiredBlockers.length
  );
}

function externalRuntimeTransportQueueClaimsStayUnimplemented(row: ExternalRuntimeTransportQueueRowCandidate): boolean {
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

function externalRuntimeTransportQueueProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeTransportQueueProof
): boolean {
  const actions = new Set(proof.externalRuntimeTransportQueueRows.map((row) => row.sourceDecisionAction));
  const queueStates = new Set(
    proof.externalRuntimeTransportQueueRows.map((row) => row.externalRuntimeTransportQueueState)
  );
  const dispatchStates = new Set(
    proof.externalRuntimeTransportQueueRows.map((row) => row.externalRuntimeTransportDispatchState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeWriterDeliveryBlockerProofVersion ===
      SourceExternalRuntimeWriterDeliveryBlockerProofVersion &&
    proof.externalRuntimeTransportQueueRows.length === ExternalRuntimeTransportQueueActions.length &&
    ExternalRuntimeTransportQueueActions.every((action) => actions.has(action)) &&
    ExternalRuntimeTransportQueueStates.every((state) => queueStates.has(state)) &&
    ExternalRuntimeTransportDispatchStates.every((state) => dispatchStates.has(state)) &&
    ExternalRuntimeTransportQueueNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeTransportQueueRows.every(externalRuntimeTransportQueueRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeTransportQueueBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeTransportQueueBoundarySchema.Type
): boolean {
  return ExternalRuntimeTransportQueueBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

