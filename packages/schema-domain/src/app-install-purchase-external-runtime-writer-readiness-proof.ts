import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel } from './app-install-purchase-external-runtime-device-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ExternalRuntimeWriterReadinessProofVersion = 'app-install-purchase-external-runtime-writer-readiness-proof';
const SourceExternalRuntimeDeviceDeliveryProofVersion = 'app-install-purchase-external-runtime-device-delivery-proof';
const ExternalRuntimeWriterReadinessTimestamp = '2026-06-07T09:58:00.000Z';
const ExternalRuntimeWriterReadinessBoundary =
  'external runtime writer readiness proof only; parent-owned runtime writer envelope, delivery result receipt, external runtime target refs, audit refs, and report refs are classified for handoff no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeWriterReadinessActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const ExternalRuntimeWriterReadinessStates = ['writer-handoff-ready', 'manual-required'] as const;
const ExternalRuntimeWriterQueueStates = ['queue-preflight-ready', 'manual-required'] as const;
const SourceExternalRuntimeEvidenceStates = ['external-runtime-evidence-ready', 'manual-required'] as const;
const ExternalRuntimeWriterReadinessNonClaims = [
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
const ExternalRuntimeWriterReadinessBoundaryFragments = [
  'parent-owned runtime writer envelope',
  'delivery result receipt',
  'external runtime target refs',
  'audit refs',
  'report refs',
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

export const AppInstallPurchaseExternalRuntimeWriterReadinessProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeWriterReadinessProofVersion)
);
const ExternalRuntimeWriterReadinessActionSchema = withParser(Schema.Literal(...ExternalRuntimeWriterReadinessActions));
const ExternalRuntimeWriterReadinessStateSchema = withParser(Schema.Literal(...ExternalRuntimeWriterReadinessStates));
const ExternalRuntimeWriterQueueStateSchema = withParser(Schema.Literal(...ExternalRuntimeWriterQueueStates));
const SourceExternalRuntimeEvidenceStateSchema = withParser(Schema.Literal(...SourceExternalRuntimeEvidenceStates));
const ExternalRuntimeWriterExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeWriterDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeWriterIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeWriterAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeWriterCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ExternalRuntimeWriterReadinessNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterReadinessNonClaims)
);

const ExternalRuntimeWriterReadinessRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterReadinessRowId');
const ExternalRuntimeWriterReadinessRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterReadinessRef');
const ExternalRuntimeWriterReadinessAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterReadinessAuditRef');
const ExternalRuntimeWriterReadinessBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeWriterReadinessBoundary');

const ExternalRuntimeWriterReadinessRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterReadinessProofSchemaVersionSchema,
  externalRuntimeWriterReadinessRowId: ExternalRuntimeWriterReadinessRowIdSchema,
  sourceExternalRuntimeDeviceDeliveryProofVersion: Schema.Literal(SourceExternalRuntimeDeviceDeliveryProofVersion),
  sourceExternalRuntimeDeviceDeliveryRowId: ExternalRuntimeWriterReadinessRefSchema,
  sourceDecisionAction: ExternalRuntimeWriterReadinessActionSchema,
  sourceExternalRuntimeEvidenceState: SourceExternalRuntimeEvidenceStateSchema,
  sourceRuntimeWriterEnvelopeRef: ExternalRuntimeWriterReadinessRefSchema,
  sourceDeliveryResultReceiptRef: ExternalRuntimeWriterReadinessRefSchema,
  sourceExternalRuntimeWriterTargetRefs: Schema.Array(ExternalRuntimeWriterReadinessRefSchema),
  externalRuntimeWriterReadinessState: ExternalRuntimeWriterReadinessStateSchema,
  externalRuntimeWriterQueueState: ExternalRuntimeWriterQueueStateSchema,
  externalRuntimeWriterPreflightRef: ExternalRuntimeWriterReadinessRefSchema,
  externalRuntimeWriterReceiptRef: ExternalRuntimeWriterReadinessRefSchema,
  externalRuntimeWriterTargetRefs: Schema.Array(ExternalRuntimeWriterReadinessRefSchema),
  externalRuntimeWriterAuditEventRefs: Schema.Array(ExternalRuntimeWriterReadinessAuditRefSchema),
  childDeliveryAuditEventRefs: Schema.Array(ExternalRuntimeWriterReadinessAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ExternalRuntimeWriterReadinessRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeWriterExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeWriterDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeWriterDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeWriterExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeWriterIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeWriterIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeWriterAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeWriterDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeWriterDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeWriterIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeWriterCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeWriterIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeWriterReadinessBoundarySchema,
  classifiedAt: ParentTimestampSchema,
});

type ExternalRuntimeWriterReadinessRowCandidate = Infer<typeof ExternalRuntimeWriterReadinessRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema = withParser(
  ExternalRuntimeWriterReadinessRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeWriterReadinessRowIsHonest(row) ||
        'Expected external runtime writer readiness rows to classify source writer evidence without execution, delivery, provider, store, platform, child, report, custody, or blocking claims'
    )
  )
);

const ExternalRuntimeWriterReadinessProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterReadinessProofSchemaVersionSchema,
  sourceExternalRuntimeDeviceDeliveryProofVersion: Schema.Literal(SourceExternalRuntimeDeviceDeliveryProofVersion),
  externalRuntimeWriterReadinessRows: Schema.Array(AppInstallPurchaseExternalRuntimeWriterReadinessRowSchema),
  nonClaims: Schema.Array(ExternalRuntimeWriterReadinessNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeWriterReadinessRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeWriterReadinessProof = Infer<
  typeof ExternalRuntimeWriterReadinessProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterReadinessProofSchema = withParser(
  ExternalRuntimeWriterReadinessProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeWriterReadinessProofIsHonest(proof) ||
        'Expected app install/purchase external runtime writer readiness proof to cover parent actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeWriterReadinessKnownGaps = [
  'External runtime writer readiness rows classify parent-owned evidence for handoff only; no external writer process, queue, transport, or device delivery is implemented.',
  'Provider/store execution, store integration, platform interception/adapters, child-device delivery, runtime report delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI, external runtime writer delivery, and child delivery transport exist.',
] as const;

export const AppInstallPurchaseExternalRuntimeWriterReadinessProofReadModel =
  AppInstallPurchaseExternalRuntimeWriterReadinessProofSchema.parse({
    schemaVersion: ExternalRuntimeWriterReadinessProofVersion,
    sourceExternalRuntimeDeviceDeliveryProofVersion: SourceExternalRuntimeDeviceDeliveryProofVersion,
    externalRuntimeWriterReadinessRows:
      AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel.externalRuntimeDeviceDeliveryRows.map(
        externalRuntimeWriterReadinessRow
      ),
    nonClaims: ExternalRuntimeWriterReadinessNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeWriterReadinessKnownGaps,
    updatedAt: ExternalRuntimeWriterReadinessTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeWriterReadinessProof(
  proof: AppInstallPurchaseExternalRuntimeWriterReadinessProof
) {
  return {
    externalRuntimeWriterReadinessRows: proof.externalRuntimeWriterReadinessRows.length,
    writerHandoffReadyRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterReadinessState === 'writer-handoff-ready'
    ).length,
    queuePreflightReadyRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterQueueState === 'queue-preflight-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterReadinessState === 'manual-required'
    ).length,
    externalRuntimeWriterExecutedRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterExecutionClaim !== 'not-executed'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterReadinessRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeWriterReadinessRow(
  row: (typeof AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel.externalRuntimeDeviceDeliveryRows)[number]
) {
  const ready = row.externalRuntimeEvidenceState === 'external-runtime-evidence-ready';
  return {
    schemaVersion: ExternalRuntimeWriterReadinessProofVersion,
    externalRuntimeWriterReadinessRowId: `external-runtime-writer-readiness-${row.sourceDecisionAction}`,
    sourceExternalRuntimeDeviceDeliveryProofVersion: SourceExternalRuntimeDeviceDeliveryProofVersion,
    sourceExternalRuntimeDeviceDeliveryRowId: row.externalRuntimeDeviceDeliveryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeEvidenceState: row.externalRuntimeEvidenceState,
    sourceRuntimeWriterEnvelopeRef: row.sourceRuntimeWriterEnvelopeRef,
    sourceDeliveryResultReceiptRef: row.sourceDeliveryResultReceiptRef,
    sourceExternalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
    externalRuntimeWriterReadinessState: ready ? 'writer-handoff-ready' : 'manual-required',
    externalRuntimeWriterQueueState: ready ? 'queue-preflight-ready' : 'manual-required',
    externalRuntimeWriterPreflightRef: `external-runtime-writer-preflight-${row.sourceDecisionAction}`,
    externalRuntimeWriterReceiptRef: `external-runtime-writer-readiness-receipt-${row.sourceDecisionAction}`,
    externalRuntimeWriterTargetRefs: uniqueRefs([
      row.sourceRuntimeWriterEnvelopeRef,
      row.sourceDeliveryResultReceiptRef,
      ...row.externalRuntimeWriterTargetRefs,
    ]),
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
    claimBoundary: ExternalRuntimeWriterReadinessBoundary,
    classifiedAt: ExternalRuntimeWriterReadinessTimestamp,
  } as const;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function externalRuntimeWriterReadinessRowIsHonest(row: ExternalRuntimeWriterReadinessRowCandidate): boolean {
  return (
    externalRuntimeWriterReadinessMatchesSource(row) &&
    externalRuntimeWriterReadinessRefsAreComplete(row) &&
    externalRuntimeWriterReadinessClaimsStayUnimplemented(row) &&
    externalRuntimeWriterReadinessBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeWriterReadinessMatchesSource(row: ExternalRuntimeWriterReadinessRowCandidate): boolean {
  if (row.sourceExternalRuntimeEvidenceState === 'manual-required') {
    return (
      row.externalRuntimeWriterReadinessState === 'manual-required' &&
      row.externalRuntimeWriterQueueState === 'manual-required'
    );
  }
  return (
    row.externalRuntimeWriterReadinessState === 'writer-handoff-ready' &&
    row.externalRuntimeWriterQueueState === 'queue-preflight-ready'
  );
}

function externalRuntimeWriterReadinessRefsAreComplete(row: ExternalRuntimeWriterReadinessRowCandidate): boolean {
  return (
    row.sourceExternalRuntimeDeviceDeliveryProofVersion === SourceExternalRuntimeDeviceDeliveryProofVersion &&
    row.sourceExternalRuntimeDeviceDeliveryRowId.length > 0 &&
    row.sourceRuntimeWriterEnvelopeRef.length > 0 &&
    row.sourceDeliveryResultReceiptRef.length > 0 &&
    row.sourceExternalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterPreflightRef.length > 0 &&
    row.externalRuntimeWriterReceiptRef.length > 0 &&
    row.externalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterAuditEventRefs.length > 0 &&
    row.childDeliveryAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function externalRuntimeWriterReadinessClaimsStayUnimplemented(
  row: ExternalRuntimeWriterReadinessRowCandidate
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

function externalRuntimeWriterReadinessProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeWriterReadinessProof
): boolean {
  const actions = new Set(proof.externalRuntimeWriterReadinessRows.map((row) => row.sourceDecisionAction));
  const readinessStates = new Set(
    proof.externalRuntimeWriterReadinessRows.map((row) => row.externalRuntimeWriterReadinessState)
  );
  const queueStates = new Set(
    proof.externalRuntimeWriterReadinessRows.map((row) => row.externalRuntimeWriterQueueState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeDeviceDeliveryProofVersion === SourceExternalRuntimeDeviceDeliveryProofVersion &&
    proof.externalRuntimeWriterReadinessRows.length === ExternalRuntimeWriterReadinessActions.length &&
    ExternalRuntimeWriterReadinessActions.every((action) => actions.has(action)) &&
    ExternalRuntimeWriterReadinessStates.every((state) => readinessStates.has(state)) &&
    ExternalRuntimeWriterQueueStates.every((state) => queueStates.has(state)) &&
    ExternalRuntimeWriterReadinessNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeWriterReadinessRows.every(externalRuntimeWriterReadinessRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeWriterReadinessBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeWriterReadinessBoundarySchema.Type
) {
  return ExternalRuntimeWriterReadinessBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

