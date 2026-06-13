import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel } from './app-install-purchase-child-device-delivery-runtime-writer-proof';
import { AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel } from './app-install-purchase-runtime-writer-execution-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
const ExternalRuntimeDeviceDeliveryProofVersion = 'app-install-purchase-external-runtime-device-delivery-proof';
const SourceRuntimeWriterExecutionDeliveryProofVersion = 'app-install-purchase-runtime-writer-execution-delivery-proof';
const SourceChildDeviceDeliveryRuntimeWriterProofVersion =
  'app-install-purchase-child-device-delivery-runtime-writer-proof';
const ExternalRuntimeDeviceDeliveryTimestamp = '2026-06-07T06:18:00.000Z';
const ExternalRuntimeDeviceDeliveryBoundary =
  'external runtime device delivery evidence proof only; parent-owned runtime writer envelope, delivery result receipt, and child delivery envelope refs are linked no external runtime writer execution no external runtime writer delivery no parent action runtime delivery no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const ExternalRuntimeDeviceDeliveryActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const ExternalRuntimeEvidenceStates = ['external-runtime-evidence-ready', 'manual-required'] as const;
const SourceRuntimeWriterReceiptClaims = ['parent-owned-delivery-result-recorded', 'manual-required'] as const;
const SourceChildDeliveryEnvelopeStates = ['child-delivery-envelope-ready', 'manual-review-required'] as const;
const ExternalRuntimeDeviceDeliveryNonClaims = [
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
const ExternalRuntimeDeviceDeliveryBoundaryFragments = [
  'parent-owned runtime writer envelope',
  'delivery result receipt',
  'child delivery envelope',
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

export const AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(ExternalRuntimeDeviceDeliveryProofVersion)
);
const ExternalRuntimeDeviceDeliveryActionSchema = withParser(Schema.Literal(...ExternalRuntimeDeviceDeliveryActions));
const ExternalRuntimeEvidenceStateSchema = withParser(Schema.Literal(...ExternalRuntimeEvidenceStates));
const SourceRuntimeWriterReceiptClaimSchema = withParser(Schema.Literal(...SourceRuntimeWriterReceiptClaims));
const SourceChildDeliveryEnvelopeStateSchema = withParser(Schema.Literal(...SourceChildDeliveryEnvelopeStates));
const ExternalRuntimeDeviceDeliveryExecutionClaimSchema = withParser(Schema.Literal('not-executed'));
const ExternalRuntimeDeviceDeliveryDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const ExternalRuntimeDeviceDeliveryIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const ExternalRuntimeDeviceDeliveryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const ExternalRuntimeDeviceDeliveryCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const ExternalRuntimeDeviceDeliveryNonClaimSchema = withParser(
  Schema.Literal(...ExternalRuntimeDeviceDeliveryNonClaims)
);

const ExternalRuntimeDeviceDeliveryRowIdSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeDeviceDeliveryRowId');
const ExternalRuntimeDeviceDeliveryRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeDeviceDeliveryRef');
const ExternalRuntimeDeviceDeliveryAuditRefSchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeDeviceDeliveryAuditRef');
const ExternalRuntimeDeviceDeliveryBoundarySchema = brandedNonEmptyStringSchema('AppInstallPurchaseExternalRuntimeDeviceDeliveryBoundary');

const ExternalRuntimeDeviceDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchemaVersionSchema,
  externalRuntimeDeviceDeliveryRowId: ExternalRuntimeDeviceDeliveryRowIdSchema,
  sourceRuntimeWriterExecutionDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterExecutionDeliveryProofVersion),
  sourceRuntimeWriterExecutionDeliveryRowId: ExternalRuntimeDeviceDeliveryRefSchema,
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  sourceChildDeviceDeliveryRuntimeWriterRowId: ExternalRuntimeDeviceDeliveryRefSchema,
  sourceDecisionAction: ExternalRuntimeDeviceDeliveryActionSchema,
  sourceRuntimeWriterEnvelopeRef: ExternalRuntimeDeviceDeliveryRefSchema,
  sourceDeliveryResultReceiptRef: ExternalRuntimeDeviceDeliveryRefSchema,
  sourceRuntimeWriterReceiptClaim: SourceRuntimeWriterReceiptClaimSchema,
  sourceChildDeliveryEnvelopeState: SourceChildDeliveryEnvelopeStateSchema,
  sourceChildDeliveryTargetRefs: Schema.Array(ExternalRuntimeDeviceDeliveryRefSchema),
  externalRuntimeEvidenceState: ExternalRuntimeEvidenceStateSchema,
  externalRuntimeWriterTargetRefs: Schema.Array(ExternalRuntimeDeviceDeliveryRefSchema),
  externalRuntimeWriterAuditEventRefs: Schema.Array(ExternalRuntimeDeviceDeliveryAuditRefSchema),
  childDeliveryAuditEventRefs: Schema.Array(ExternalRuntimeDeviceDeliveryAuditRefSchema),
  reportRuntimeRefs: Schema.Array(ExternalRuntimeDeviceDeliveryRefSchema),
  externalRuntimeWriterExecutionClaim: ExternalRuntimeDeviceDeliveryExecutionClaimSchema,
  externalRuntimeWriterDeliveryClaim: ExternalRuntimeDeviceDeliveryDeliveryClaimSchema,
  parentActionRuntimeDeliveryClaim: ExternalRuntimeDeviceDeliveryDeliveryClaimSchema,
  providerApiExecutionClaim: ExternalRuntimeDeviceDeliveryExecutionClaimSchema,
  storeIntegrationClaim: ExternalRuntimeDeviceDeliveryIntegrationClaimSchema,
  platformInterceptionClaim: ExternalRuntimeDeviceDeliveryIntegrationClaimSchema,
  platformAdapterClaim: ExternalRuntimeDeviceDeliveryAdapterClaimSchema,
  childDeviceDeliveryClaim: ExternalRuntimeDeviceDeliveryDeliveryClaimSchema,
  runtimeReportDeliveryClaim: ExternalRuntimeDeviceDeliveryDeliveryClaimSchema,
  appBlockingClaim: ExternalRuntimeDeviceDeliveryIntegrationClaimSchema,
  childDataCustody: ExternalRuntimeDeviceDeliveryCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: ExternalRuntimeDeviceDeliveryIntegrationClaimSchema,
  claimBoundary: ExternalRuntimeDeviceDeliveryBoundarySchema,
  linkedAt: ParentTimestampSchema,
});

type ExternalRuntimeDeviceDeliveryRowCandidate = Infer<typeof ExternalRuntimeDeviceDeliveryRowBaseSchema>;

export const AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema = withParser(
  ExternalRuntimeDeviceDeliveryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        externalRuntimeDeviceDeliveryRowIsHonest(row) ||
        'Expected external runtime device delivery evidence rows to link writer receipt and child envelope refs without external runtime, child-device, provider, store, platform, report delivery, custody, or blocking claims'
    )
  )
);

const ExternalRuntimeDeviceDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchemaVersionSchema,
  sourceRuntimeWriterExecutionDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterExecutionDeliveryProofVersion),
  sourceChildDeviceDeliveryRuntimeWriterProofVersion: Schema.Literal(
    SourceChildDeviceDeliveryRuntimeWriterProofVersion
  ),
  externalRuntimeDeviceDeliveryRows: Schema.Array(AppInstallPurchaseExternalRuntimeDeviceDeliveryRowSchema),
  nonClaims: Schema.Array(ExternalRuntimeDeviceDeliveryNonClaimSchema),
  knownGaps: Schema.Array(ExternalRuntimeDeviceDeliveryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseExternalRuntimeDeviceDeliveryProof = Infer<
  typeof ExternalRuntimeDeviceDeliveryProofBaseSchema
>;

export const AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchema = withParser(
  ExternalRuntimeDeviceDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        externalRuntimeDeviceDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase external runtime device delivery evidence proof to cover parent actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseExternalRuntimeDeviceDeliveryKnownGaps = [
  'External runtime device delivery rows are evidence links only; no external writer process or device delivery transport is implemented.',
  'Provider/store execution, store integration, platform interception/adapters, child-device delivery, runtime report delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI, external runtime writer delivery, and child delivery transport exist.',
] as const;

export const AppInstallPurchaseExternalRuntimeDeviceDeliveryProofReadModel =
  AppInstallPurchaseExternalRuntimeDeviceDeliveryProofSchema.parse({
    schemaVersion: ExternalRuntimeDeviceDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryProofVersion: SourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion: SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    externalRuntimeDeviceDeliveryRows:
      AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows.map(
        externalRuntimeDeviceDeliveryRow
      ),
    nonClaims: ExternalRuntimeDeviceDeliveryNonClaims,
    knownGaps: AppInstallPurchaseExternalRuntimeDeviceDeliveryKnownGaps,
    updatedAt: ExternalRuntimeDeviceDeliveryTimestamp,
  });

export function summarizeAppInstallPurchaseExternalRuntimeDeviceDeliveryProof(
  proof: AppInstallPurchaseExternalRuntimeDeviceDeliveryProof
) {
  return {
    externalRuntimeDeviceDeliveryRows: proof.externalRuntimeDeviceDeliveryRows.length,
    externalRuntimeEvidenceReadyRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.externalRuntimeEvidenceState === 'external-runtime-evidence-ready'
    ).length,
    manualRequiredRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.externalRuntimeEvidenceState === 'manual-required'
    ).length,
    externalRuntimeWriterDeliveredRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.externalRuntimeWriterDeliveryClaim !== 'not-delivered'
    ).length,
    childDeviceDeliveredRows: proof.externalRuntimeDeviceDeliveryRows.filter(
      (row) => row.childDeviceDeliveryClaim !== 'not-delivered'
    ).length,
  } as const;
}

function externalRuntimeDeviceDeliveryRow(
  row: (typeof AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel.runtimeWriterExecutionDeliveryRows)[number]
) {
  const childDeliveryRow = childDeviceDeliveryRuntimeWriterRowFor(row.sourceDecisionAction);
  const manual = row.sourceDecisionAction === 'review-needed';
  return {
    schemaVersion: ExternalRuntimeDeviceDeliveryProofVersion,
    externalRuntimeDeviceDeliveryRowId: `external-runtime-device-delivery-${row.sourceDecisionAction}`,
    sourceRuntimeWriterExecutionDeliveryProofVersion: SourceRuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterExecutionDeliveryRowId: row.runtimeWriterExecutionDeliveryRowId,
    sourceChildDeviceDeliveryRuntimeWriterProofVersion: SourceChildDeviceDeliveryRuntimeWriterProofVersion,
    sourceChildDeviceDeliveryRuntimeWriterRowId: childDeliveryRow.childDeviceDeliveryRuntimeWriterRowId,
    sourceDecisionAction: row.sourceDecisionAction,
    sourceRuntimeWriterEnvelopeRef: row.runtimeWriterEnvelopeRef,
    sourceDeliveryResultReceiptRef: row.deliveryResultReceiptRef,
    sourceRuntimeWriterReceiptClaim: row.runtimeWriterExecutionClaim,
    sourceChildDeliveryEnvelopeState: childDeliveryRow.childDeliveryEnvelopeState,
    sourceChildDeliveryTargetRefs: childDeliveryRow.childDeliveryTargetRefs,
    externalRuntimeEvidenceState: manual ? 'manual-required' : 'external-runtime-evidence-ready',
    externalRuntimeWriterTargetRefs: uniqueRefs([
      row.runtimeWriterEnvelopeRef,
      row.deliveryResultReceiptRef,
      ...childDeliveryRow.childDeliveryTargetRefs,
    ]),
    externalRuntimeWriterAuditEventRefs: uniqueRefs([
      ...row.deliveryResultAuditEventRefs,
      ...row.parentActionAuditEventRefs,
    ]),
    childDeliveryAuditEventRefs: uniqueRefs([
      ...childDeliveryRow.runtimeWriterAuditEventRefs,
      ...childDeliveryRow.packageSourceAuditEventRefs,
    ]),
    reportRuntimeRefs: uniqueRefs([...row.reportRuntimeRefs, ...childDeliveryRow.reportRuntimeRefs]),
    externalRuntimeWriterExecutionClaim: 'not-executed',
    externalRuntimeWriterDeliveryClaim: 'not-delivered',
    parentActionRuntimeDeliveryClaim: 'not-delivered',
    providerApiExecutionClaim: row.providerApiExecutionClaim,
    storeIntegrationClaim: row.storeIntegrationClaim,
    platformInterceptionClaim: row.platformInterceptionClaim,
    platformAdapterClaim: row.platformAdapterClaim,
    childDeviceDeliveryClaim: row.childDeviceDeliveryClaim,
    runtimeReportDeliveryClaim: row.runtimeReportDeliveryClaim,
    appBlockingClaim: row.appBlockingClaim,
    childDataCustody: row.childDataCustody,
    ocentraHostedFamilyDataCustodyClaim: row.ocentraHostedFamilyDataCustodyClaim,
    claimBoundary: ExternalRuntimeDeviceDeliveryBoundary,
    linkedAt: ExternalRuntimeDeviceDeliveryTimestamp,
  } as const;
}

function childDeviceDeliveryRuntimeWriterRowFor(action: (typeof ExternalRuntimeDeviceDeliveryActions)[number]) {
  return AppInstallPurchaseChildDeviceDeliveryRuntimeWriterProofReadModel.childDeviceDeliveryRuntimeWriterRows.find(
    (row) => row.sourceDecisionAction === action
  )!;
}

function uniqueRefs(refs: readonly string[]) {
  return Array.from(new Set(refs));
}

function externalRuntimeDeviceDeliveryRowIsHonest(row: ExternalRuntimeDeviceDeliveryRowCandidate): boolean {
  return (
    externalRuntimeEvidenceMatchesAction(row) &&
    externalRuntimeDeviceDeliveryRefsAreComplete(row) &&
    externalRuntimeDeviceDeliveryClaimsStayUnimplemented(row) &&
    externalRuntimeDeviceDeliveryBoundaryIsExplicit(row.claimBoundary)
  );
}

function externalRuntimeEvidenceMatchesAction(row: ExternalRuntimeDeviceDeliveryRowCandidate): boolean {
  if (row.sourceDecisionAction === 'review-needed') {
    return (
      row.sourceRuntimeWriterReceiptClaim === 'manual-required' &&
      row.sourceChildDeliveryEnvelopeState === 'manual-review-required' &&
      row.externalRuntimeEvidenceState === 'manual-required'
    );
  }
  return (
    row.sourceRuntimeWriterReceiptClaim === 'parent-owned-delivery-result-recorded' &&
    row.sourceChildDeliveryEnvelopeState === 'child-delivery-envelope-ready' &&
    row.externalRuntimeEvidenceState === 'external-runtime-evidence-ready'
  );
}

function externalRuntimeDeviceDeliveryRefsAreComplete(row: ExternalRuntimeDeviceDeliveryRowCandidate): boolean {
  return (
    row.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    row.sourceRuntimeWriterExecutionDeliveryRowId.length > 0 &&
    row.sourceChildDeviceDeliveryRuntimeWriterProofVersion === SourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    row.sourceChildDeviceDeliveryRuntimeWriterRowId.length > 0 &&
    row.sourceRuntimeWriterEnvelopeRef.length > 0 &&
    row.sourceDeliveryResultReceiptRef.length > 0 &&
    row.sourceChildDeliveryTargetRefs.length > 0 &&
    row.externalRuntimeWriterTargetRefs.length > 0 &&
    row.externalRuntimeWriterAuditEventRefs.length > 0 &&
    row.childDeliveryAuditEventRefs.length > 0 &&
    row.reportRuntimeRefs.length > 0
  );
}

function externalRuntimeDeviceDeliveryClaimsStayUnimplemented(row: ExternalRuntimeDeviceDeliveryRowCandidate): boolean {
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

function externalRuntimeDeviceDeliveryProofIsHonest(
  proof: AppInstallPurchaseExternalRuntimeDeviceDeliveryProof
): boolean {
  const actions = new Set(proof.externalRuntimeDeviceDeliveryRows.map((row) => row.sourceDecisionAction));
  const evidenceStates = new Set(
    proof.externalRuntimeDeviceDeliveryRows.map((row) => row.externalRuntimeEvidenceState)
  );
  const nonClaims = new Set(proof.nonClaims);
  return (
    proof.sourceRuntimeWriterExecutionDeliveryProofVersion === SourceRuntimeWriterExecutionDeliveryProofVersion &&
    proof.sourceChildDeviceDeliveryRuntimeWriterProofVersion === SourceChildDeviceDeliveryRuntimeWriterProofVersion &&
    proof.externalRuntimeDeviceDeliveryRows.length === ExternalRuntimeDeviceDeliveryActions.length &&
    ExternalRuntimeDeviceDeliveryActions.every((action) => actions.has(action)) &&
    ExternalRuntimeEvidenceStates.every((state) => evidenceStates.has(state)) &&
    ExternalRuntimeDeviceDeliveryNonClaims.every((claim) => nonClaims.has(claim)) &&
    proof.externalRuntimeDeviceDeliveryRows.every(externalRuntimeDeviceDeliveryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}

function externalRuntimeDeviceDeliveryBoundaryIsExplicit(
  boundary: typeof ExternalRuntimeDeviceDeliveryBoundarySchema.Type
) {
  return ExternalRuntimeDeviceDeliveryBoundaryFragments.every((fragment) => boundary.includes(fragment));
}

