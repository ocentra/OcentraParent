import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel } from './app-install-purchase-external-runtime-device-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';

const ExternalRuntimeDeliveryHandoffText = Schema.String.pipe(Schema.minLength(1));
const ExternalRuntimeDeliveryHandoffProofVersion = 'app-install-purchase-external-runtime-delivery-handoff-proof';
const SourceExternalRuntimeDeviceDeliveryProofVersion = 'app-install-purchase-external-runtime-device-delivery-proof';
const ExternalRuntimeDeliveryHandoffTimestamp = '2026-06-07T09:42:00.000Z';
const ExternalRuntimeDeliveryHandoffBoundary =
  'external runtime delivery handoff proof only; parent-owned handoff packet and queue refs are recorded no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeDeliveryHandoffActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceExternalRuntimeEvidenceStates = ['external-runtime-evidence-ready', 'manual-required'] as const;
const ExternalRuntimeDeliveryHandoffStates = ['handoff-packet-ready', 'manual-required'] as const;
const ExternalRuntimeDeliveryHandoffNonClaims = [
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
const ExternalRuntimeDeliveryHandoffBoundaryFragments = [
  'parent-owned handoff packet',
  'queue refs',
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

export const AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeDeliveryHandoffProofVersion)
);
const ExternalRuntimeDeliveryHandoffActionSchema = withParser(Schema.Literal(...ExternalRuntimeDeliveryHandoffActions));
const SourceExternalRuntimeEvidenceStateSchema = withParser(Schema.Literal(...SourceExternalRuntimeEvidenceStates));
const ExternalRuntimeDeliveryHandoffStateSchema = withParser(Schema.Literal(...ExternalRuntimeDeliveryHandoffStates));
const ExternalRuntimeDeliveryHandoffExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeDeliveryHandoffDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeDeliveryHandoffIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeDeliveryHandoffAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeDeliveryHandoffCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ExternalRuntimeDeliveryHandoffNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeDeliveryHandoffNonClaims)
);

const ExternalRuntimeDeliveryHandoffRowIdSchema = ExternalRuntimeDeliveryHandoffText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeDeliveryHandoffRowId')
);
const ExternalRuntimeDeliveryHandoffRefSchema = ExternalRuntimeDeliveryHandoffText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeDeliveryHandoffRef')
);
const ExternalRuntimeDeliveryHandoffAuditRefSchema = ExternalRuntimeDeliveryHandoffText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeDeliveryHandoffAuditRef')
);
const ExternalRuntimeDeliveryHandoffBoundarySchema = ExternalRuntimeDeliveryHandoffText.pipe(
  Schema.brand('AppInstallPurchaseExternalRuntimeDeliveryHandoffBoundary')
);

const ExternalRuntimeDeliveryHandoffRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchemaVersionSchema,
  externalRuntimeDeliveryHandoffRowId: ExternalRuntimeDeliveryHandoffRowIdSchema,
  sourceExternalRuntimeDeviceDeliveryProofVersion: Schema.Literal(SourceExternalRuntimeDeviceDeliveryProofVersion),
  sourceExternalRuntimeDeviceDeliveryRowId: ExternalRuntimeDeliveryHandoffRefSchema,
  sourceDecisionAction: ExternalRuntimeDeliveryHandoffActionSchema,
  sourceExternalRuntimeEvidenceState: SourceExternalRuntimeEvidenceStateSchema,
  sourceRuntimeWriterEnvelopeRef: ExternalRuntimeDeliveryHandoffRefSchema,
  sourceDeliveryResultReceiptRef: ExternalRuntimeDeliveryHandoffRefSchema,
  sourceExternalRuntimeWriterTargetRefs: Schema.Array(ExternalRuntimeDeliveryHandoffRefSchema),
  sourceChildDeliveryAuditEventRefs: Schema.Array(ExternalRuntimeDeliveryHandoffAuditRefSchema),
  externalRuntimeDeliveryHandoffState: ExternalRuntimeDeliveryHandoffStateSchema,
  externalRuntimeHandoffPacketRef: ExternalRuntimeDeliveryHandoffRefSchema,
  externalRuntimeWriterQueueRef: ExternalRuntimeDeliveryHandoffRefSchema,
  externalRuntimeWriterDispatchAuditEventRefs: Schema.Array(ExternalRuntimeDeliveryHandoffAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ExternalRuntimeDeliveryHandoffRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeDeliveryHandoffExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeDeliveryHandoffDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeDeliveryHandoffDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeDeliveryHandoffExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeDeliveryHandoffIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeDeliveryHandoffIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeDeliveryHandoffAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeDeliveryHandoffDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeDeliveryHandoffDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeDeliveryHandoffIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeDeliveryHandoffCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeDeliveryHandoffIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeDeliveryHandoffBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ExternalRuntimeDeliveryHandoffRowCandidate = Infer<typeof ExternalRuntimeDeliveryHandoffRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema = withParser(
  ExternalRuntimeDeliveryHandoffRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeDeliveryHandoffRowIsHonest(row) ||
        'Expected external runtime delivery handoff rows to record parent-owned packet and queue refs without external writer, provider, store, platform, child-device, report delivery, custody, or blocking claims'
    )
  )
);

const ExternalRuntimeDeliveryHandoffProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchemaVersionSchema,
  sourceExternalRuntimeDeviceDeliveryProofVersion: Schema.Literal(SourceExternalRuntimeDeviceDeliveryProofVersion),
  externalRuntimeDeliveryHandoffRows: Schema.Array(AppInstallPurchaseExternalRuntimeDeliveryHandoffRowSchema),
  nonClaims: Schema.Array(ExternalRuntimeDeliveryHandoffNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeDeliveryHandoffRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeDeliveryHandoffProof = Infer<
  typeof ExternalRuntimeDeliveryHandoffProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchema = withParser(
  ExternalRuntimeDeliveryHandoffProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeDeliveryHandoffProofIsHonest(proof) ||
        'Expected app install/purchase external runtime delivery handoff proof to cover parent actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeDeliveryHandoffKnownGaps = [
  'External runtime delivery handoff rows record deterministic parent-owned packet and queue refs only; no external writer process or delivery transport is implemented.',
  'Provider/store execution, store integration, platform interception/adapters, child-device delivery, runtime report delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI, external runtime writer delivery, and child delivery transport exist.',
] as const;

export const AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel =
  AppInstallPurchaseExternalRuntimeDeliveryHandoffProofSchema.parse({
    schemaVersion: ExternalRuntimeDeliveryHandoffProofVersion,
    sourceExternalRuntimeDeviceDeliveryProofVersion: SourceExternalRuntimeDeviceDeliveryProofVersion,
    externalRuntimeDeliveryHandoffRows:
      AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel.externalRuntimeDeviceDeliveryRows.map(
        externalRuntimeDeliveryHandoffRow
      ),
    nonClaims: ExternalRuntimeDeliveryHandoffNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeDeliveryHandoffKnownGaps,
    updatedAt: ExternalRuntimeDeliveryHandoffTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeDeliveryHandoffProof(
  proof: AppInstallPurchaseExternalRuntimeDeliveryHandoffProof
) {
  return {
    externalRuntimeDeliveryHandoffRows: proof.externalRuntimeDeliveryHandoffRows.length,
    handoffPacketReadyRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.externalRuntimeDeliveryHandoffState === 'handoff-packet-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.externalRuntimeDeliveryHandoffState === 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
    childDeviceDeliveredRows: proof.externalRuntimeDeliveryHandoffRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeDeliveryHandoffRow(
  row: (typeof AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel.externalRuntimeDeviceDeliveryRows)[number]
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: ExternalRuntimeDeliveryHandoffProofVersion,
    externalRuntimeDeliveryHandoffRowId: `external-runtime-delivery-handoff-${row.sourceDecisionAction}`,
    sourceExternalRuntimeDeviceDeliveryProofVersion: SourceExternalRuntimeDeviceDeliveryProofVersion,
    sourceExternalRuntimeDeviceDeliveryRowId: row.externalRuntimeDeviceDeliveryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeEvidenceState: row.externalRuntimeEvidenceState,
    sourceRuntimeWriterEnvelopeRef: row.sourceRuntimeWriterEnvelopeRef,
    sourceDeliveryResultReceiptRef: row.sourceDeliveryResultReceiptRef,
    sourceExternalRuntimeWriterTargetRefs: row.externalRuntimeWriterTargetRefs,
    sourceChildDeliveryAuditEventRefs: row.childDeliveryAuditEventRefs,
    externalRuntimeDeliveryHandoffState: manual ? 'manual-required' : 'handoff-packet-ready',
    externalRuntimeHandoffPacketRef: manual
      ? `manual-external-runtime-handoff-packet-${row.sourceDecisionAction}`
      : `parent-owned-external-runtime-handoff-packet-${row.sourceDecisionAction}`,
    externalRuntimeWriterQueueRef: manual
      ? `manual-external-runtime-writer-queue-${row.sourceDecisionAction}`
      : `parent-owned-external-runtime-writer-queue-${row.sourceDecisionAction}`,
    externalRuntimeWriterDispatchAuditEventRefs: uniqueRefs([
      ...row.externalRuntimeWriterAuditEventRefs,
      ...row.childDeliveryAuditEventRefs,
    ]),
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
    claimBoundary: ExternalRuntimeDeliveryHandoffBoundary,
    linkedAt: ExternalRuntimeDeliveryHandoffTimestamp,
  } as const;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function externalRuntimeDeliveryHandoffRowIsHonest(row: ExternalRuntimeDeliveryHandoffRowCandidate): boolean {
  return (
    externalRuntimeDeliveryHandoffMatchesEvidence(row) &&
    externalRuntimeDeliveryHandoffRefsAreComplete(row) &&
    externalRuntimeDeliveryHandoffClaimsStayUnimplemented(row) &&
    externalRuntimeDeliveryHandoffBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeDeliveryHandoffMatchesEvidence(row: ExternalRuntimeDeliveryHandoffRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.sourceExternalRuntimeEvidenceState === 'manual-required' &&
      row.externalRuntimeDeliveryHandoffState === 'manual-required'
    );
  }
  return (
    row.sourceExternalRuntimeEvidenceState === 'external-runtime-evidence-ready' &&
    row.externalRuntimeDeliveryHandoffState === 'handoff-packet-ready'
  );
}

function externalRuntimeDeliveryHandoffRefsAreComplete(row: ExternalRuntimeDeliveryHandoffRowCandidate): boolean {
  return (
    row.sourceExternalRuntimeDeviceDeliveryProofVersion === SourceExternalRuntimeDeviceDeliveryProofVersion &&
    row.sourceExternalRuntimeDeviceDeliveryRowId.length > 0 &&
    row.sourceRuntimeWriterEnvelopeRef.length > 0 &&
    row.sourceDeliveryResultReceiptRef.length > 0 &&
    row.sourceExternalRuntimeWriterTargetRefs.length > 0 &&
    row.sourceChildDeliveryAuditEventRefs.length > 0 &&
    row.externalRuntimeHandoffPacketRef.length > 0 &&
    row.externalRuntimeWriterQueueRef.length > 0 &&
    row.externalRuntimeWriterDispatchAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function externalRuntimeDeliveryHandoffClaimsStayUnimplemented(
  row: ExternalRuntimeDeliveryHandoffRowCandidate
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

function externalRuntimeDeliveryHandoffProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeDeliveryHandoffProof
): boolean {
  const actions = new Set(proof.externalRuntimeDeliveryHandoffRows.map((row) => row.sourceDecisionAction));
  const handoffStates = new Set(
    proof.externalRuntimeDeliveryHandoffRows.map((row) => row.externalRuntimeDeliveryHandoffState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeDeviceDeliveryProofVersion === SourceExternalRuntimeDeviceDeliveryProofVersion &&
    proof.externalRuntimeDeliveryHandoffRows.length === ExternalRuntimeDeliveryHandoffActions.length &&
    ExternalRuntimeDeliveryHandoffActions.every((action) => actions.has(action)) &&
    ExternalRuntimeDeliveryHandoffStates.every((state) => handoffStates.has(state)) &&
    ExternalRuntimeDeliveryHandoffNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeDeliveryHandoffRows.every(externalRuntimeDeliveryHandoffRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeDeliveryHandoffBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeDeliveryHandoffBoundarySchema.Type
): boolean {
  return ExternalRuntimeDeliveryHandoffBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
