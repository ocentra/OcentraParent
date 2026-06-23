import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel } from './app-install-purchase-external-runtime-delivery-handoff-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const WriterDeliveryBoundaryProofVersion = 'app-install-purchase-external-runtime-writer-delivery-boundary-proof';
const SourceDeliveryHandoffProofVersion = 'app-install-purchase-external-runtime-delivery-handoff-proof';
const WriterDeliveryBoundaryTimestamp = '2026-06-07T10:45:00.000Z';
const WriterDeliveryBoundary =
  'external runtime writer delivery boundary proof only; required external writer transport proof refs platform adapter proof refs provider store proof refs and child-device delivery proof refs are recorded no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const WriterDeliveryBoundaryActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceDeliveryHandoffStates = ['handoff-packet-ready', 'manual-required'] as const;
const WriterDeliveryBoundaryStates = ['runtime-writer-delivery-prerequisites-ready', 'manual-required'] as const;
const WriterDeliveryBoundaryNonClaims = [
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
const WriterDeliveryBoundaryFragments = [
  'required external writer transport proof refs',
  'platform adapter proof refs',
  'provider store proof refs',
  'child-device delivery proof refs',
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

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchemaVersionSchema = withParser(
  Schema.Literal(WriterDeliveryBoundaryProofVersion)
);
const WriterDeliveryBoundaryActionSchema = withParser(Schema.Literal(...WriterDeliveryBoundaryActions));
const SourceDeliveryHandoffStateSchema = withParser(Schema.Literal(...SourceDeliveryHandoffStates));
const WriterDeliveryBoundaryStateSchema = withParser(Schema.Literal(...WriterDeliveryBoundaryStates));
const WriterDeliveryBoundaryExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const WriterDeliveryBoundaryDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const WriterDeliveryBoundaryIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const WriterDeliveryBoundaryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const WriterDeliveryBoundaryCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const WriterDeliveryBoundaryNonClaimSchema = withParser(Schema.Literal(...WriterDeliveryBoundaryNonClaims));

const WriterDeliveryBoundaryRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowId'
);
const WriterDeliveryBoundaryRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRef'
);
const WriterDeliveryBoundaryAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryAuditRef'
);
const WriterDeliveryBoundaryClaimBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryClaimBoundary'
);

const WriterDeliveryBoundaryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchemaVersionSchema,
  externalRuntimeWriterDeliveryBoundaryRowId: WriterDeliveryBoundaryRowIdSchema,
  sourceExternalRuntimeDeliveryHandoffProofVersion: Schema.Literal(SourceDeliveryHandoffProofVersion),
  sourceExternalRuntimeDeliveryHandoffRowId: WriterDeliveryBoundaryRefSchema,
  sourceDecisionAction: WriterDeliveryBoundaryActionSchema,
  sourceExternalRuntimeDeliveryHandoffState: SourceDeliveryHandoffStateSchema,
  sourceExternalRuntimeHandoffPacketRef: WriterDeliveryBoundaryRefSchema,
  sourceExternalRuntimeWriterQueueRef: WriterDeliveryBoundaryRefSchema,
  sourceExternalRuntimeWriterDispatchAuditEventRefs: Schema.Array(WriterDeliveryBoundaryAuditRefSchema),
  sourceReportRuntimeRefs: Schema.Array(WriterDeliveryBoundaryRefSchema),
  externalRuntimeWriterDeliveryBoundaryState: WriterDeliveryBoundaryStateSchema,
  requiredExternalWriterTransportProofRefs: Schema.Array(WriterDeliveryBoundaryRefSchema),
  requiredPlatformAdapterProofRefs: Schema.Array(WriterDeliveryBoundaryRefSchema),
  requiredProviderStoreProofRefs: Schema.Array(WriterDeliveryBoundaryRefSchema),
  requiredChildDeviceDeliveryProofRefs: Schema.Array(WriterDeliveryBoundaryRefSchema),
  externalRuntimeWriterDeliveryReadinessAuditEventRefs: Schema.Array(WriterDeliveryBoundaryAuditRefSchema),
  externalRuntimeWriterExecutionClaim: WriterDeliveryBoundaryExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: WriterDeliveryBoundaryDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: WriterDeliveryBoundaryDeliveryClaimSchema,
  providerApiExecutionClaim: WriterDeliveryBoundaryExecutionClaimSchema,
  storeIntegrationClaim: WriterDeliveryBoundaryIntegrationClaimSchema,
  platformInterceptionClaim: WriterDeliveryBoundaryIntegrationClaimSchema,
  platformAdapterClaim: WriterDeliveryBoundaryAdapterClaimSchema,
  childDeviceDeliveryClaim: WriterDeliveryBoundaryDeliveryClaimSchema,
  runtimeReportDeliveryClaim: WriterDeliveryBoundaryDeliveryClaimSchema,
  appBlockingClaim: WriterDeliveryBoundaryIntegrationClaimSchema,
  childDataCustody: WriterDeliveryBoundaryCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: WriterDeliveryBoundaryIntegrationClaimSchema,
  claimBoundary: WriterDeliveryBoundaryClaimBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type WriterDeliveryBoundaryRowCandidate = Infer<typeof WriterDeliveryBoundaryRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema = withParser(
  WriterDeliveryBoundaryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        writerDeliveryBoundaryRowIsHonest(row) ||
        'Expected external runtime writer delivery boundary rows to record required writer, platform, provider/store, and child delivery proof refs without delivery claims'
    )
  )
);

const WriterDeliveryBoundaryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchemaVersionSchema,
  sourceExternalRuntimeDeliveryHandoffProofVersion: Schema.Literal(SourceDeliveryHandoffProofVersion),
  externalRuntimeWriterDeliveryBoundaryRows: Schema.Array(
    AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryRowSchema
  ),
  nonClaims: Schema.Array(WriterDeliveryBoundaryNonClaimSchema),
  knownGaps: Schema.Array(WriterDeliveryBoundaryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProof = Infer<
  typeof WriterDeliveryBoundaryProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchema = withParser(
  WriterDeliveryBoundaryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        writerDeliveryBoundaryProofIsHonest(proof) ||
        'Expected app install/purchase external runtime writer delivery boundary proof to preserve unimplemented delivery boundaries'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryKnownGaps = [
  'External runtime writer delivery boundary rows record required external writer transport, platform adapter, provider/store, and child-device delivery proof refs only; no external runtime writer execution or delivery is implemented.',
  'Parent action runtime delivery, provider/store execution, store integration, platform interception/adapters, child-device delivery, runtime report delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI, external runtime writer delivery, platform/provider/store adapters, and child delivery transport exist.',
] as const;

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel =
  AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofSchema.parse({
    schemaVersion: WriterDeliveryBoundaryProofVersion,
    sourceExternalRuntimeDeliveryHandoffProofVersion: SourceDeliveryHandoffProofVersion,
    externalRuntimeWriterDeliveryBoundaryRows:
      AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel.externalRuntimeDeliveryHandoffRows.map(
        writerDeliveryBoundaryRow
      ),
    nonClaims: WriterDeliveryBoundaryNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryKnownGaps,
    updatedAt: WriterDeliveryBoundaryTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProof(
  proof: AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProof
) {
  return {
    externalRuntimeWriterDeliveryBoundaryRows: proof.externalRuntimeWriterDeliveryBoundaryRows.length,
    prerequisiteReadyRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryBoundaryState === 'runtime-writer-delivery-prerequisites-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryBoundaryState === 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
    childDeviceDeliveredRows: proof.externalRuntimeWriterDeliveryBoundaryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function writerDeliveryBoundaryRow(
  row: (typeof AppInstallPurchaseExternalRuntimeDeliveryHandoffProofReadModel.externalRuntimeDeliveryHandoffRows)[number]
) {
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: WriterDeliveryBoundaryProofVersion,
    externalRuntimeWriterDeliveryBoundaryRowId: `external-runtime-writer-delivery-boundary-${row.sourceDecisionAction}`,
    sourceExternalRuntimeDeliveryHandoffProofVersion: SourceDeliveryHandoffProofVersion,
    sourceExternalRuntimeDeliveryHandoffRowId: row.externalRuntimeDeliveryHandoffRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeDeliveryHandoffState: row.externalRuntimeDeliveryHandoffState,
    sourceExternalRuntimeHandoffPacketRef: row.externalRuntimeHandoffPacketRef,
    sourceExternalRuntimeWriterQueueRef: row.externalRuntimeWriterQueueRef,
    sourceExternalRuntimeWriterDispatchAuditEventRefs: row.externalRuntimeWriterDispatchAuditEventRefs,
    sourceReportRuntimeRefs: row.reportRuntimeRefs,
    externalRuntimeWriterDeliveryBoundaryState: manual
      ? 'manual-required'
      : 'runtime-writer-delivery-prerequisites-ready',
    requiredExternalWriterTransportProofRefs: requiredRefs('external-writer-transport-proof', row.sourceDecisionAction),
    requiredPlatformAdapterProofRefs: requiredRefs('platform-adapter-proof', row.sourceDecisionAction),
    requiredProviderStoreProofRefs: requiredRefs('provider-store-execution-proof', row.sourceDecisionAction),
    requiredChildDeviceDeliveryProofRefs: requiredRefs('child-device-delivery-proof', row.sourceDecisionAction),
    externalRuntimeWriterDeliveryReadinessAuditEventRefs: uniqueRefs([
      ...row.externalRuntimeWriterDispatchAuditEventRefs,
      `external-runtime-writer-delivery-boundary-audit-${row.sourceDecisionAction}`,
    ]),
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
    claimBoundary: WriterDeliveryBoundary,
    linkedAt: WriterDeliveryBoundaryTimestamp,
  } as const;
}

function requiredRefs(prefix: string, action: (typeof WriterDeliveryBoundaryActions)[number]) {
  return [`${prefix}-${action}`];
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function writerDeliveryBoundaryRowIsHonest(row: WriterDeliveryBoundaryRowCandidate): boolean {
  return (
    writerDeliveryBoundaryMatchesHandoff(row) &&
    writerDeliveryBoundaryRefsAreComplete(row) &&
    writerDeliveryBoundaryClaimsStayUnimplemented(row) &&
    writerDeliveryBoundaryIsExplicit(row.claimBoundary)
  );
}

function writerDeliveryBoundaryMatchesHandoff(row: WriterDeliveryBoundaryRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.sourceExternalRuntimeDeliveryHandoffState === 'manual-required' &&
      row.externalRuntimeWriterDeliveryBoundaryState === 'manual-required'
    );
  }
  return (
    row.sourceExternalRuntimeDeliveryHandoffState === 'handoff-packet-ready' &&
    row.externalRuntimeWriterDeliveryBoundaryState === 'runtime-writer-delivery-prerequisites-ready'
  );
}

function writerDeliveryBoundaryRefsAreComplete(row: WriterDeliveryBoundaryRowCandidate): boolean {
  return (
    row.sourceExternalRuntimeDeliveryHandoffProofVersion === SourceDeliveryHandoffProofVersion &&
    row.sourceExternalRuntimeDeliveryHandoffRowId.length > 0 &&
    row.sourceExternalRuntimeHandoffPacketRef.length > 0 &&
    row.sourceExternalRuntimeWriterQueueRef.length > 0 &&
    row.sourceExternalRuntimeWriterDispatchAuditEventRefs.length > 0 &&
    row.sourceReportRuntimeRefs.length > 0 &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.requiredChildDeviceDeliveryProofRefs.length > 0 &&
    row.externalRuntimeWriterDeliveryReadinessAuditEventRefs.length > 0
  );
}

function writerDeliveryBoundaryClaimsStayUnimplemented(row: WriterDeliveryBoundaryRowCandidate): boolean {
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

function writerDeliveryBoundaryProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProof
): boolean {
  const actions = new Set(proof.externalRuntimeWriterDeliveryBoundaryRows.map((row) => row.sourceDecisionAction));
  const states = new Set(
    proof.externalRuntimeWriterDeliveryBoundaryRows.map((row) => row.externalRuntimeWriterDeliveryBoundaryState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeDeliveryHandoffProofVersion === SourceDeliveryHandoffProofVersion &&
    proof.externalRuntimeWriterDeliveryBoundaryRows.length === WriterDeliveryBoundaryActions.length &&
    WriterDeliveryBoundaryActions.every((action) => actions.has(action)) &&
    WriterDeliveryBoundaryStates.every((state) => states.has(state)) &&
    WriterDeliveryBoundaryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeWriterDeliveryBoundaryRows.every(writerDeliveryBoundaryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function writerDeliveryBoundaryIsExplicit(boundary: typeof WriterDeliveryBoundaryClaimBoundarySchema.Type): boolean {
  return WriterDeliveryBoundaryFragments.every((fragment) => boundary.includes(fragment));
}
