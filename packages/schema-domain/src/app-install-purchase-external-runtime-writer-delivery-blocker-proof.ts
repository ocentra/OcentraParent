import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel } from './app-install-purchase-external-runtime-writer-delivery-boundary-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
const ExternalRuntimeWriterDeliveryBlockerProofVersion =
  'app-install-purchase-external-runtime-writer-delivery-blocker-proof';
const SourceExternalRuntimeWriterDeliveryBoundaryProofVersion =
  'app-install-purchase-external-runtime-writer-delivery-boundary-proof';
const ExternalRuntimeWriterDeliveryBlockerTimestamp = '2026-06-07T14:52:00.000Z';
const ExternalRuntimeWriterDeliveryBlockerBoundary =
  'external runtime writer delivery blocker proof only; delivery remains blocked until external writer transport platform adapter provider store execution and child-device transport proof refs are real no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeWriterDeliveryBlockerActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const SourceExternalRuntimeWriterDeliveryBoundaryStates = [
  'runtime-writer-delivery-prerequisites-ready',
  'manual-required',
] as const;
const ExternalRuntimeWriterDeliveryBlockerStates = [
  'blocked-runtime-prerequisites-missing',
  'manual-required',
] as const;
const ExternalRuntimeWriterDeliveryAttemptStates = ['not-started'] as const;
const ExternalRuntimeWriterDeliveryBlockerNonClaims = [
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
const ExternalRuntimeWriterDeliveryRequiredBlockers = [
  'external-writer-transport-proof-missing',
  'platform-adapter-proof-missing',
  'provider-store-execution-proof-missing',
  'child-device-transport-proof-missing',
] as const;
const ExternalRuntimeWriterDeliveryBlockerFragments = [
  'delivery remains blocked',
  'external writer transport',
  'platform adapter',
  'provider store execution',
  'child-device transport',
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

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeWriterDeliveryBlockerProofVersion)
);
const ExternalRuntimeWriterDeliveryBlockerActionSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterDeliveryBlockerActions)
);
const SourceExternalRuntimeWriterDeliveryBoundaryStateSchema = withParser(
  Schema.Literal(...SourceExternalRuntimeWriterDeliveryBoundaryStates)
);
const ExternalRuntimeWriterDeliveryBlockerStateSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterDeliveryBlockerStates)
);
const ExternalRuntimeWriterDeliveryAttemptStateSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterDeliveryAttemptStates)
);
const ExternalRuntimeWriterDeliveryExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeWriterDeliveryDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeWriterDeliveryIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeWriterDeliveryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeWriterDeliveryCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ExternalRuntimeWriterDeliveryBlockerNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterDeliveryBlockerNonClaims)
);
const ExternalRuntimeWriterDeliveryRequiredBlockerSchema = withParser(
  Schema.Literal(...ExternalRuntimeWriterDeliveryRequiredBlockers)
);

const ExternalRuntimeWriterDeliveryBlockerRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowId'
);
const ExternalRuntimeWriterDeliveryBlockerRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRef'
);
const ExternalRuntimeWriterDeliveryBlockerAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerAuditRef'
);
const ExternalRuntimeWriterDeliveryBlockerBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerBoundary'
);

const ExternalRuntimeWriterDeliveryBlockerRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchemaVersionSchema,
  externalRuntimeWriterDeliveryBlockerRowId: ExternalRuntimeWriterDeliveryBlockerRowIdSchema,
  sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: Schema.Literal(
    SourceExternalRuntimeWriterDeliveryBoundaryProofVersion
  ),
  sourceExternalRuntimeWriterDeliveryBoundaryRowId: ExternalRuntimeWriterDeliveryBlockerRefSchema,
  sourceDecisionAction: ExternalRuntimeWriterDeliveryBlockerActionSchema,
  sourceExternalRuntimeWriterDeliveryBoundaryState: SourceExternalRuntimeWriterDeliveryBoundaryStateSchema,
  sourceExternalRuntimeWriterQueueRef: ExternalRuntimeWriterDeliveryBlockerRefSchema,
  requiredExternalWriterTransportProofRefs: Schema.Array(ExternalRuntimeWriterDeliveryBlockerRefSchema),
  requiredPlatformAdapterProofRefs: Schema.Array(ExternalRuntimeWriterDeliveryBlockerRefSchema),
  requiredProviderStoreProofRefs: Schema.Array(ExternalRuntimeWriterDeliveryBlockerRefSchema),
  requiredChildDeviceDeliveryProofRefs: Schema.Array(ExternalRuntimeWriterDeliveryBlockerRefSchema),
  deliveryBlockerState: ExternalRuntimeWriterDeliveryBlockerStateSchema,
  deliveryAttemptState: ExternalRuntimeWriterDeliveryAttemptStateSchema,
  requiredRuntimeBlockers: Schema.Array(ExternalRuntimeWriterDeliveryRequiredBlockerSchema),
  manualBlockerRefs: Schema.Array(ExternalRuntimeWriterDeliveryBlockerRefSchema),
  deliveryBlockerAuditEventRefs: Schema.Array(ExternalRuntimeWriterDeliveryBlockerAuditRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeWriterDeliveryExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeWriterDeliveryDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeWriterDeliveryDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeWriterDeliveryExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeWriterDeliveryIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeWriterDeliveryIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeWriterDeliveryAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeWriterDeliveryDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeWriterDeliveryDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeWriterDeliveryIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeWriterDeliveryCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeWriterDeliveryIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeWriterDeliveryBlockerBoundarySchema,
  blockedAt: ParentTimestampSchema,
});

type ExternalRuntimeWriterDeliveryBlockerRowCandidate = Infer<typeof ExternalRuntimeWriterDeliveryBlockerRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema = withParser(
  ExternalRuntimeWriterDeliveryBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeWriterDeliveryBlockerRowIsHonest(row) ||
        'Expected external runtime writer delivery blocker rows to keep delivery blocked until real writer transport platform provider-store and child-device proof exists'
    )
  )
);

const ExternalRuntimeWriterDeliveryBlockerProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchemaVersionSchema,
  sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: Schema.Literal(
    SourceExternalRuntimeWriterDeliveryBoundaryProofVersion
  ),
  externalRuntimeWriterDeliveryBlockerRows: Schema.Array(
    AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerRowSchema
  ),
  nonClaims: Schema.Array(ExternalRuntimeWriterDeliveryBlockerNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeWriterDeliveryBlockerRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProof = Infer<
  typeof ExternalRuntimeWriterDeliveryBlockerProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchema = withParser(
  ExternalRuntimeWriterDeliveryBlockerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeWriterDeliveryBlockerProofIsHonest(proof) ||
        'Expected app install/purchase external runtime writer delivery blocker proof to preserve missing-runtime blockers and non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerKnownGaps = [
  'External runtime writer delivery remains blocked because real external writer transport, platform adapter execution, provider/store execution, and child-device transport proof refs are not implemented.',
  'Rows preserve the parent-owned queue and proof-ref boundary only; no external runtime writer execution, external writer delivery, child-device delivery, provider/store execution, platform interception, app blocking, child activity data, or Ocentra-hosted family custody is implemented.',
  'Product capability checklist row update is deferred while E-C owns docs/product-capability-checklist.md; pending delta is to mention this delivery-blocker proof as non-claim evidence for the Install/purchase approval row.',
] as const;

export const AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofReadModel =
  AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProofSchema.parse({
    schemaVersion: ExternalRuntimeWriterDeliveryBlockerProofVersion,
    sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: SourceExternalRuntimeWriterDeliveryBoundaryProofVersion,
    externalRuntimeWriterDeliveryBlockerRows:
      AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel.externalRuntimeWriterDeliveryBoundaryRows.map(
        externalRuntimeWriterDeliveryBlockerRow
      ),
    nonClaims: ExternalRuntimeWriterDeliveryBlockerNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerKnownGaps,
    updatedAt: ExternalRuntimeWriterDeliveryBlockerTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProof(
  proof: AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProof
) {
  return {
    externalRuntimeWriterDeliveryBlockerRows: proof.externalRuntimeWriterDeliveryBlockerRows.length,
    blockedRuntimePrerequisiteRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.deliveryBlockerState === 'blocked-runtime-prerequisites-missing'
    ).length,
    manualRequiredRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.deliveryBlockerState === 'manual-required'
    ).length,
    deliveryAttemptStartedRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.deliveryAttemptState !== 'not-started'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeWriterDeliveryBlockerRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeWriterDeliveryBlockerRow(
  row: (typeof AppInstallPurchaseExternalRuntimeWriterDeliveryBoundaryProofReadModel.externalRuntimeWriterDeliveryBoundaryRows)[number]
) {
  const manual = row.externalRuntimeWriterDeliveryBoundaryState === 'manual-required';
  return {
    schemaVersion: ExternalRuntimeWriterDeliveryBlockerProofVersion,
    externalRuntimeWriterDeliveryBlockerRowId: `external-runtime-writer-delivery-blocker-${row.sourceDecisionAction}`,
    sourceExternalRuntimeWriterDeliveryBoundaryProofVersion: SourceExternalRuntimeWriterDeliveryBoundaryProofVersion,
    sourceExternalRuntimeWriterDeliveryBoundaryRowId: row.externalRuntimeWriterDeliveryBoundaryRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceExternalRuntimeWriterDeliveryBoundaryState: row.externalRuntimeWriterDeliveryBoundaryState,
    sourceExternalRuntimeWriterQueueRef: row.sourceExternalRuntimeWriterQueueRef,
    requiredExternalWriterTransportProofRefs: row.requiredExternalWriterTransportProofRefs,
    requiredPlatformAdapterProofRefs: row.requiredPlatformAdapterProofRefs,
    requiredProviderStoreProofRefs: row.requiredProviderStoreProofRefs,
    requiredChildDeviceDeliveryProofRefs: row.requiredChildDeviceDeliveryProofRefs,
    deliveryBlockerState: manual ? 'manual-required' : 'blocked-runtime-prerequisites-missing',
    deliveryAttemptState: 'not-started',
    requiredRuntimeBlockers: ExternalRuntimeWriterDeliveryRequiredBlockers,
    manualBlockerRefs: manualBlockerRefs(row.sourceDecisionAction),
    deliveryBlockerAuditEventRefs: uniqueRefs([
      ...row.externalRuntimeWriterDeliveryReadinessAuditEventRefs,
      `external-runtime-writer-delivery-blocker-audit-${row.sourceDecisionAction}`,
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
    claimBoundary: ExternalRuntimeWriterDeliveryBlockerBoundary,
    blockedAt: ExternalRuntimeWriterDeliveryBlockerTimestamp,
  } as const;
}

function manualBlockerRefs(action: (typeof ExternalRuntimeWriterDeliveryBlockerActions)[number]) {
  return [
    `missing-external-writer-transport-${action}`,
    `missing-platform-adapter-execution-${action}`,
    `missing-provider-store-execution-${action}`,
    `missing-child-device-transport-${action}`,
  ];
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function externalRuntimeWriterDeliveryBlockerRowIsHonest(
  row: ExternalRuntimeWriterDeliveryBlockerRowCandidate
): boolean {
  return (
    externalRuntimeWriterDeliveryBlockerMatchesBoundary(row) &&
    externalRuntimeWriterDeliveryBlockerRefsAreComplete(row) &&
    externalRuntimeWriterDeliveryBlockerClaimsStayUnimplemented(row) &&
    externalRuntimeWriterDeliveryBlockerBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeWriterDeliveryBlockerMatchesBoundary(
  row: ExternalRuntimeWriterDeliveryBlockerRowCandidate
): boolean {
  if (row.sourceExternalRuntimeWriterDeliveryBoundaryState === 'manual-required') {
    return row.deliveryBlockerState === 'manual-required';
  }
  return row.deliveryBlockerState === 'blocked-runtime-prerequisites-missing';
}

function externalRuntimeWriterDeliveryBlockerRefsAreComplete(
  row: ExternalRuntimeWriterDeliveryBlockerRowCandidate
): boolean {
  return (
    row.sourceExternalRuntimeWriterDeliveryBoundaryProofVersion ===
      SourceExternalRuntimeWriterDeliveryBoundaryProofVersion &&
    row.sourceExternalRuntimeWriterDeliveryBoundaryRowId.length > 0 &&
    row.sourceExternalRuntimeWriterQueueRef.length > 0 &&
    row.requiredExternalWriterTransportProofRefs.length > 0 &&
    row.requiredPlatformAdapterProofRefs.length > 0 &&
    row.requiredProviderStoreProofRefs.length > 0 &&
    row.requiredChildDeviceDeliveryProofRefs.length > 0 &&
    ExternalRuntimeWriterDeliveryRequiredBlockers.every((blocker) => row.requiredRuntimeBlockers.includes(blocker)) &&
    row.manualBlockerRefs.length === ExternalRuntimeWriterDeliveryRequiredBlockers.length &&
    row.deliveryBlockerAuditEventRefs.length > 0 &&
    row.deliveryAttemptState === 'not-started'
  );
}

function externalRuntimeWriterDeliveryBlockerClaimsStayUnimplemented(
  row: ExternalRuntimeWriterDeliveryBlockerRowCandidate
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

function externalRuntimeWriterDeliveryBlockerProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeWriterDeliveryBlockerProof
): boolean {
  const actions = new Set(proof.externalRuntimeWriterDeliveryBlockerRows.map((row) => row.sourceDecisionAction));
  const states = new Set(proof.externalRuntimeWriterDeliveryBlockerRows.map((row) => row.deliveryBlockerState));
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceExternalRuntimeWriterDeliveryBoundaryProofVersion ===
      SourceExternalRuntimeWriterDeliveryBoundaryProofVersion &&
    proof.externalRuntimeWriterDeliveryBlockerRows.length === ExternalRuntimeWriterDeliveryBlockerActions.length &&
    ExternalRuntimeWriterDeliveryBlockerActions.every((action) => actions.has(action)) &&
    ExternalRuntimeWriterDeliveryBlockerStates.every((state) => states.has(state)) &&
    ExternalRuntimeWriterDeliveryBlockerNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeWriterDeliveryBlockerRows.every(externalRuntimeWriterDeliveryBlockerRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeWriterDeliveryBlockerBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeWriterDeliveryBlockerBoundarySchema.Type
): boolean {
  return ExternalRuntimeWriterDeliveryBlockerFragments.every((fragment) => boundary.includes(fragment));
}
