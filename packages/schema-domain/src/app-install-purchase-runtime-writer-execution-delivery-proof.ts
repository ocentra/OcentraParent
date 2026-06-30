import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from '@ocentra-parent/schema-domain/effect';
import { AppInstallPurchaseParentActionDeliveryReadinessProofReadModel } from './app-install-purchase-parent-action-delivery-readiness-proof';
import { AppInstallPurchaseRuntimeWriterDeliveryProofReadModel } from './app-install-purchase-runtime-writer-delivery-proof';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  buildAppInstallPurchaseRuntimeWriterExecutionDeliveryRowGenerated,
  runtimeWriterExecutionDeliveryProofIsHonestGenerated,
  runtimeWriterExecutionDeliveryRowIsHonestGenerated,
  summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProofGenerated,
} from './generated/app-install-purchase-delivery-runtime-helpers';
const RuntimeWriterExecutionDeliveryProofVersion = 'app-install-purchase-runtime-writer-execution-delivery-proof';
const SourceRuntimeWriterDeliveryProofVersion = 'app-install-purchase-runtime-writer-delivery-proof';
const SourceParentActionDeliveryReadinessProofVersion = 'app-install-purchase-parent-action-delivery-readiness-proof';
const RuntimeWriterExecutionDeliveryTimestamp = '2026-06-05T19:55:00.000Z';
const RuntimeWriterExecutionDeliveryBoundary =
  'runtime writer execution delivery proof only; parent-owned runtime writer envelope and delivery result receipt are recorded no provider API execution no store integration no platform interception no platform adapter implementation no child-device delivery no runtime report delivery no real install or purchase interception no app blocking no child activity data no Ocentra-hosted family data custody';
const RuntimeWriterExecutionDeliveryActions = ['approve', 'deny', 'time-box', 'review-needed'] as const;
const RuntimeWriterExecutionDeliveryStates = ['delivery-result-recorded', 'manual-required'] as const;
const RuntimeWriterEnvelopeStates = ['parent-owned-envelope-written', 'manual-required'] as const;
const RuntimeWriterDeliveryReceiptClaims = ['parent-owned-delivery-result-recorded', 'manual-required'] as const;
const RuntimeWriterExecutionDeliveryNonClaims = [
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
const RuntimeWriterExecutionDeliveryBoundaryFragments = [
  'parent-owned runtime writer envelope',
  'delivery result receipt',
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

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchemaVersionSchema = withParser(
  Schema.Literal(RuntimeWriterExecutionDeliveryProofVersion)
);
const RuntimeWriterExecutionDeliveryActionSchema = withParser(Schema.Literal(...RuntimeWriterExecutionDeliveryActions));
const RuntimeWriterExecutionDeliveryStateSchema = withParser(Schema.Literal(...RuntimeWriterExecutionDeliveryStates));
const RuntimeWriterEnvelopeStateSchema = withParser(Schema.Literal(...RuntimeWriterEnvelopeStates));
const RuntimeWriterExecutionDeliveryReceiptClaimSchema = withParser(
  Schema.Literal(...RuntimeWriterDeliveryReceiptClaims)
);
const RuntimeWriterExecutionDeliveryProviderClaimSchema = withParser(Schema.Literal('not-executed'));
const RuntimeWriterExecutionDeliveryIntegrationClaimSchema = withParser(Schema.Literal('not-claimed'));
const RuntimeWriterExecutionDeliveryAdapterClaimSchema = withParser(Schema.Literal('not-implemented'));
const RuntimeWriterExecutionDeliveryDeliveryClaimSchema = withParser(Schema.Literal('not-delivered'));
const RuntimeWriterExecutionDeliveryCustodyClaimSchema = withParser(Schema.Literal('no-child-activity-data'));
const RuntimeWriterExecutionDeliveryNonClaimSchema = withParser(
  Schema.Literal(...RuntimeWriterExecutionDeliveryNonClaims)
);

const RuntimeWriterExecutionDeliveryRowIdSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeWriterExecutionDeliveryRowId'
);
const RuntimeWriterExecutionDeliveryRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeWriterExecutionDeliveryRef'
);
const RuntimeWriterExecutionDeliveryAuditRefSchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeWriterExecutionDeliveryAuditRef'
);
const RuntimeWriterExecutionDeliveryBoundarySchema = brandedNonEmptyStringSchema(
  'AppInstallPurchaseRuntimeWriterExecutionDeliveryBoundary'
);

const RuntimeWriterExecutionDeliveryRowBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchemaVersionSchema,
  runtimeWriterExecutionDeliveryRowId: RuntimeWriterExecutionDeliveryRowIdSchema,
  sourceRuntimeWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterDeliveryProofVersion),
  sourceRuntimeWriterDeliveryRowId: RuntimeWriterExecutionDeliveryRefSchema,
  sourceParentActionDeliveryReadinessProofVersion: Schema.Literal(SourceParentActionDeliveryReadinessProofVersion),
  sourceParentActionDeliveryReadinessRowId: RuntimeWriterExecutionDeliveryRefSchema,
  sourceDecisionAction: RuntimeWriterExecutionDeliveryActionSchema,
  runtimeWriterEnvelopeState: RuntimeWriterEnvelopeStateSchema,
  runtimeWriterEnvelopeRef: RuntimeWriterExecutionDeliveryRefSchema,
  runtimeWriterExecutionDeliveryState: RuntimeWriterExecutionDeliveryStateSchema,
  deliveryResultReceiptRef: RuntimeWriterExecutionDeliveryRefSchema,
  deliveryResultAuditEventRefs: Schema.Array(RuntimeWriterExecutionDeliveryAuditRefSchema),
  parentActionAuditEventRefs: Schema.Array(RuntimeWriterExecutionDeliveryAuditRefSchema),
  reportRuntimeRefs: Schema.Array(RuntimeWriterExecutionDeliveryRefSchema),
  runtimeWriterExecutionClaim: RuntimeWriterExecutionDeliveryReceiptClaimSchema,
  runtimeWriterDeliveryClaim: RuntimeWriterExecutionDeliveryReceiptClaimSchema,
  parentActionRuntimeDeliveryClaim: RuntimeWriterExecutionDeliveryReceiptClaimSchema,
  providerApiExecutionClaim: RuntimeWriterExecutionDeliveryProviderClaimSchema,
  storeIntegrationClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  platformInterceptionClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  platformAdapterClaim: RuntimeWriterExecutionDeliveryAdapterClaimSchema,
  childDeviceDeliveryClaim: RuntimeWriterExecutionDeliveryDeliveryClaimSchema,
  runtimeReportDeliveryClaim: RuntimeWriterExecutionDeliveryDeliveryClaimSchema,
  appBlockingClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  childDataCustody: RuntimeWriterExecutionDeliveryCustodyClaimSchema,
  ocentraHostedFamilyDataCustodyClaim: RuntimeWriterExecutionDeliveryIntegrationClaimSchema,
  claimBoundary: RuntimeWriterExecutionDeliveryBoundarySchema,
  recordedAt: ParentTimestampSchema,
});

type RuntimeWriterExecutionDeliveryRowCandidate = Infer<typeof RuntimeWriterExecutionDeliveryRowBaseSchema>;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema = withParser(
  RuntimeWriterExecutionDeliveryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        runtimeWriterExecutionDeliveryRowIsHonest(row) ||
        'Expected runtime writer execution delivery rows to record parent-owned envelopes and receipts without provider, store, platform, child-device, custody, report delivery, or blocking claims'
    )
  )
);

const RuntimeWriterExecutionDeliveryProofBaseSchema = Schema.Struct({
  schemaVersion: AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchemaVersionSchema,
  sourceRuntimeWriterDeliveryProofVersion: Schema.Literal(SourceRuntimeWriterDeliveryProofVersion),
  sourceParentActionDeliveryReadinessProofVersion: Schema.Literal(SourceParentActionDeliveryReadinessProofVersion),
  runtimeWriterExecutionDeliveryRows: Schema.Array(AppInstallPurchaseRuntimeWriterExecutionDeliveryRowSchema),
  nonClaims: Schema.Array(RuntimeWriterExecutionDeliveryNonClaimSchema),
  knownGaps: Schema.Array(RuntimeWriterExecutionDeliveryRefSchema),
  updatedAt: ParentTimestampSchema,
});

export type AppInstallPurchaseRuntimeWriterExecutionDeliveryProof = Infer<
  typeof RuntimeWriterExecutionDeliveryProofBaseSchema
>;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema = withParser(
  RuntimeWriterExecutionDeliveryProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        runtimeWriterExecutionDeliveryProofIsHonest(proof) ||
        'Expected app install/purchase runtime writer execution delivery proof to cover parent actions and preserve non-claims'
    )
  )
);

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryKnownGaps = [
  'Runtime writer execution delivery rows record deterministic parent-owned envelopes and delivery result receipts only.',
  'Provider/store execution, platform interception/adapters, child-device delivery, runtime report delivery, app blocking, child activity data, and Ocentra-hosted family custody remain unimplemented.',
  'Review-needed remains manual-required until portal approval UI and a real parent approval action path exist.',
] as const;

export const AppInstallPurchaseRuntimeWriterExecutionDeliveryProofReadModel =
  AppInstallPurchaseRuntimeWriterExecutionDeliveryProofSchema.parse({
    schemaVersion: RuntimeWriterExecutionDeliveryProofVersion,
    sourceRuntimeWriterDeliveryProofVersion: SourceRuntimeWriterDeliveryProofVersion,
    sourceParentActionDeliveryReadinessProofVersion: SourceParentActionDeliveryReadinessProofVersion,
    runtimeWriterExecutionDeliveryRows:
      AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows.map(
        runtimeWriterExecutionDeliveryRow
      ),
    nonClaims: RuntimeWriterExecutionDeliveryNonClaims,
    knownGaps: AppInstallPurchaseRuntimeWriterExecutionDeliveryKnownGaps,
    updatedAt: RuntimeWriterExecutionDeliveryTimestamp,
  });

export function summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProof(
  proof: AppInstallPurchaseRuntimeWriterExecutionDeliveryProof
) {
  return summarizeAppInstallPurchaseRuntimeWriterExecutionDeliveryProofGenerated(proof);
}

function runtimeWriterExecutionDeliveryRow(
  row: (typeof AppInstallPurchaseRuntimeWriterDeliveryProofReadModel.runtimeWriterDeliveryRows)[number]
) {
  return buildAppInstallPurchaseRuntimeWriterExecutionDeliveryRowGenerated(
    row,
    parentActionDeliveryReadinessRowFor(row.sourceDecisionAction),
    SourceRuntimeWriterDeliveryProofVersion,
    SourceParentActionDeliveryReadinessProofVersion,
    RuntimeWriterExecutionDeliveryBoundary,
    RuntimeWriterExecutionDeliveryTimestamp
  );
}

function parentActionDeliveryReadinessRowFor(action: (typeof RuntimeWriterExecutionDeliveryActions)[number]) {
  return AppInstallPurchaseParentActionDeliveryReadinessProofReadModel.parentActionDeliveryReadinessRows.find(
    (row) => row.sourceDecisionAction === action
  )!;
}

function runtimeWriterExecutionDeliveryRowIsHonest(row: RuntimeWriterExecutionDeliveryRowCandidate): boolean {
  return runtimeWriterExecutionDeliveryRowIsHonestGenerated(
    row,
    SourceRuntimeWriterDeliveryProofVersion,
    SourceParentActionDeliveryReadinessProofVersion,
    RuntimeWriterExecutionDeliveryBoundaryFragments
  );
}

function runtimeWriterExecutionDeliveryProofIsHonest(
  proof: AppInstallPurchaseRuntimeWriterExecutionDeliveryProof
): boolean {
  return (
    runtimeWriterExecutionDeliveryProofIsHonestGenerated(
      proof,
      SourceRuntimeWriterDeliveryProofVersion,
      SourceParentActionDeliveryReadinessProofVersion,
      RuntimeWriterExecutionDeliveryActions,
      RuntimeWriterEnvelopeStates,
      RuntimeWriterExecutionDeliveryStates,
      RuntimeWriterExecutionDeliveryNonClaims
    ) &&
    proof.runtimeWriterExecutionDeliveryRows.every(runtimeWriterExecutionDeliveryRowIsHonest) &&
    proof.knownGaps.length > 0
  );
}
