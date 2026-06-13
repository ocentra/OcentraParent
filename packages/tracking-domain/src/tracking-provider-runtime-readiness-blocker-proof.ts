import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import {
  ParentContractSchemaVersion,
  ParentContractSchemaVersionSchema,
  ParentTimestampSchema,
} from '@ocentra-parent/family-domain/reference-primitives';
import {
  TrackingNotificationLocalOutboxReadinessReadModelSchema,
  type TrackingNotificationLocalOutboxReadinessReadModel,
} from './tracking-notification-local-outbox-readiness-proof';
import {
  TrackingNotificationReceiptBoundaryReadModelSchema,
  type TrackingNotificationReceiptBoundaryReadModel,
} from './tracking-notification-receipt-boundary-proof';
import {
  TrackingProviderDeliveryArtifactGateProofSchema,
  type TrackingProviderDeliveryArtifactGateProof,
} from './tracking-provider-delivery-artifact-gate-proof';
import {
  TrackingProviderNotificationProofReadModelSchema,
  type TrackingProviderNotificationProofReadModel,
} from './tracking-provider-notification-proof';

export const TrackingProviderRuntimeReadinessBlockerIdSchema = withParser(
  Schema.Literal(
    'provider-delivery-runtime',
    'webhook-receipt-ingestion-runtime',
    'provider-credentials',
    'adapter-dispatch',
    'retry-worker-runtime',
    'quiet-hours-timer-runtime',
    'parent-notification-ui-runtime',
    'production-durable-outbox-storage',
    'child-device-delivery',
    'physical-device-proof',
    'authority-proof',
    'product-ready-tracking'
  )
);

export const TrackingProviderRuntimeReadinessBlockerReferenceSchema = brandedNonEmptyStringSchema('TrackingProviderRuntimeReadinessBlockerReference');
export const TrackingProviderRuntimeReadinessBlockerProofIdSchema = brandedNonEmptyStringSchema('TrackingProviderRuntimeReadinessBlockerProofId');

export const TrackingProviderRuntimeReadinessBlockerStatusSchema = withParser(Schema.Literal('manual-required'));

const TrackingProviderRuntimeReadinessBlockerRowBaseSchema = Schema.Struct({
  blockerId: TrackingProviderRuntimeReadinessBlockerIdSchema,
  status: TrackingProviderRuntimeReadinessBlockerStatusSchema,
  sourceProofRefs: Schema.Array(TrackingProviderRuntimeReadinessBlockerReferenceSchema),
  blockingArtifactRefs: Schema.Array(TrackingProviderRuntimeReadinessBlockerReferenceSchema),
  requiredProofTier: Schema.Literal('P4_MANUAL_PROVIDER_RUNTIME'),
  currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
  productClaimReady: Schema.Literal(false),
});

export const TrackingProviderRuntimeReadinessBlockerRowSchema = withParser(
  TrackingProviderRuntimeReadinessBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) => row.sourceProofRefs.length > 0 && row.blockingArtifactRefs.length > 0 && row.productClaimReady === false
    )
  )
);

const TrackingProviderRuntimeReadinessBlockerProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingProviderRuntimeReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  proofMode: Schema.Literal('tracking-provider-runtime-readiness-blocker-proof'),
  sourceProofRefs: Schema.Array(TrackingProviderRuntimeReadinessBlockerReferenceSchema),
  providerNotificationRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  receiptBoundaryRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  localOutboxReadinessRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  requiredProviderRuntimeArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
  presentProviderRuntimeArtifactCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  missingProviderRuntimeArtifactCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  requiredProviderRuntimeArtifactRefs: Schema.Array(TrackingProviderRuntimeReadinessBlockerReferenceSchema).pipe(
    Schema.minItems(1)
  ),
  presentProviderRuntimeArtifactRefs: Schema.Array(TrackingProviderRuntimeReadinessBlockerReferenceSchema),
  missingProviderRuntimeArtifactRefs: Schema.Array(TrackingProviderRuntimeReadinessBlockerReferenceSchema).pipe(
    Schema.minItems(1)
  ),
  providerRuntimeArtifactSetComplete: Schema.Literal(false),
  blockers: Schema.Array(TrackingProviderRuntimeReadinessBlockerRowSchema),
  productClaims: Schema.Struct({
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    webhookReceiptIngestionRuntimeClaimed: Schema.Literal(false),
    providerCredentialsClaimed: Schema.Literal(false),
    adapterDispatchClaimed: Schema.Literal(false),
    retryExecutionRuntimeClaimed: Schema.Literal(false),
    quietHoursTimerRuntimeClaimed: Schema.Literal(false),
    parentNotificationUiRuntimeClaimed: Schema.Literal(false),
    productionDurableOutboxStorageClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }),
});

export const TrackingProviderRuntimeReadinessBlockerProofSchema = withParser(
  TrackingProviderRuntimeReadinessBlockerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        trackingProviderRuntimeReadinessProofIsHonest(proof) ||
        'Expected provider runtime blocker proof to consume provider, receipt, outbox, and artifact-gate refs while keeping product claims false'
    )
  )
);

export type TrackingProviderRuntimeReadinessBlockerId = Infer<typeof TrackingProviderRuntimeReadinessBlockerIdSchema>;
export type TrackingProviderRuntimeReadinessBlockerProof = Infer<
  typeof TrackingProviderRuntimeReadinessBlockerProofSchema
>;
export type TrackingProviderRuntimeReadinessBlockerRow = Infer<typeof TrackingProviderRuntimeReadinessBlockerRowSchema>;

export type TrackingProviderRuntimeReadinessBlockerProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProofRefs: readonly string[];
};

type TrackingProviderRuntimeReadinessBlockerProofInput = Infer<
  typeof TrackingProviderRuntimeReadinessBlockerProofBaseSchema
>;

export function buildTrackingProviderRuntimeReadinessBlockerProof(
  options: TrackingProviderRuntimeReadinessBlockerProofOptions,
  providerProof: TrackingProviderNotificationProofReadModel,
  receiptProof: TrackingNotificationReceiptBoundaryReadModel,
  localOutboxProof: TrackingNotificationLocalOutboxReadinessReadModel,
  artifactGateProof: TrackingProviderDeliveryArtifactGateProof
): TrackingProviderRuntimeReadinessBlockerProof {
  const parsedProviderProof = TrackingProviderNotificationProofReadModelSchema.parse(providerProof);
  const parsedReceiptProof = TrackingNotificationReceiptBoundaryReadModelSchema.parse(receiptProof);
  const parsedLocalOutboxProof = TrackingNotificationLocalOutboxReadinessReadModelSchema.parse(localOutboxProof);
  const parsedArtifactGate = TrackingProviderDeliveryArtifactGateProofSchema.parse(artifactGateProof);
  const requiredArtifacts = uniqueRefs(parsedArtifactGate.rows.flatMap((row) => row.requiredArtifacts));
  const presentArtifacts = uniqueRefs(parsedArtifactGate.rows.flatMap((row) => row.presentArtifacts));
  const missingArtifacts = uniqueRefs(parsedArtifactGate.rows.flatMap((row) => row.missingArtifacts));
  const sourceProofRefs = uniqueRefs(options.sourceProofRefs);

  return TrackingProviderRuntimeReadinessBlockerProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    proofMode: 'tracking-provider-runtime-readiness-blocker-proof',
    sourceProofRefs,
    providerNotificationRows: parsedProviderProof.rows.length,
    receiptBoundaryRows: parsedReceiptProof.rows.length,
    localOutboxReadinessRows: parsedLocalOutboxProof.rows.length,
    requiredProviderRuntimeArtifactCount: requiredArtifacts.length,
    presentProviderRuntimeArtifactCount: presentArtifacts.length,
    missingProviderRuntimeArtifactCount: missingArtifacts.length,
    requiredProviderRuntimeArtifactRefs: requiredArtifacts,
    presentProviderRuntimeArtifactRefs: presentArtifacts,
    missingProviderRuntimeArtifactRefs: missingArtifacts,
    providerRuntimeArtifactSetComplete: false,
    blockers: RequiredTrackingProviderRuntimeReadinessBlockers.map((blockerId) =>
      buildBlockerRow(blockerId, sourceProofRefs, missingArtifacts)
    ),
    productClaims: {
      providerDeliveryRuntimeClaimed: false,
      webhookReceiptIngestionRuntimeClaimed: false,
      providerCredentialsClaimed: false,
      adapterDispatchClaimed: false,
      retryExecutionRuntimeClaimed: false,
      quietHoursTimerRuntimeClaimed: false,
      parentNotificationUiRuntimeClaimed: false,
      productionDurableOutboxStorageClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productClaimReady: false,
    },
  });
}

export const RequiredTrackingProviderRuntimeReadinessBlockers = [
  'provider-delivery-runtime',
  'webhook-receipt-ingestion-runtime',
  'provider-credentials',
  'adapter-dispatch',
  'retry-worker-runtime',
  'quiet-hours-timer-runtime',
  'parent-notification-ui-runtime',
  'production-durable-outbox-storage',
  'child-device-delivery',
  'physical-device-proof',
  'authority-proof',
  'product-ready-tracking',
] as const;

function buildBlockerRow(
  blockerId: TrackingProviderRuntimeReadinessBlockerId,
  sourceProofRefs: readonly string[],
  missingArtifacts: readonly string[]
): TrackingProviderRuntimeReadinessBlockerRow {
  return TrackingProviderRuntimeReadinessBlockerRowSchema.parse({
    blockerId,
    status: 'manual-required',
    sourceProofRefs,
    blockingArtifactRefs: missingArtifacts,
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    productClaimReady: false,
  });
}

function trackingProviderRuntimeReadinessProofIsHonest(
  proof: TrackingProviderRuntimeReadinessBlockerProofInput
): boolean {
  return (
    proof.sourceProofRefs.length >= 4 &&
    proof.requiredProviderRuntimeArtifactCount > 0 &&
    proof.requiredProviderRuntimeArtifactCount ===
      proof.presentProviderRuntimeArtifactCount + proof.missingProviderRuntimeArtifactCount &&
    proof.requiredProviderRuntimeArtifactRefs.length === proof.requiredProviderRuntimeArtifactCount &&
    proof.presentProviderRuntimeArtifactRefs.length === proof.presentProviderRuntimeArtifactCount &&
    proof.missingProviderRuntimeArtifactRefs.length === proof.missingProviderRuntimeArtifactCount &&
    proof.providerRuntimeArtifactSetComplete === false &&
    proof.missingProviderRuntimeArtifactCount > 0 &&
    proof.blockers.length === RequiredTrackingProviderRuntimeReadinessBlockers.length &&
    proof.blockers.every((row) => row.status === 'manual-required') &&
    Object.values(proof.productClaims).every((claim) => claim === false)
  );
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}

