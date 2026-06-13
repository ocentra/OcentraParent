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
  ProductionSupportStatusBackendDurableQueueRuntimeProofSchema,
  type ProductionSupportStatusBackendDurableQueueRuntimeProof,
} from '@ocentra-parent/production-domain/production-support-status-backend-durable-queue-runtime-proof';

export const TrackingProductionDurableWorkersReadinessBlockerIdSchema = withParser(
  Schema.Literal(
    'tracking-location-upload-worker-runtime',
    'tracking-retention-cleanup-worker-runtime',
    'tracking-notification-outbox-worker-runtime',
    'tracking-escalation-timeout-worker-runtime',
    'tracking-provider-receipt-worker-runtime',
    'tracking-child-device-delivery-worker-runtime',
    'tracking-authority-status-worker-runtime',
    'tracking-production-audit-durable-storage',
    'tracking-production-product-ready-closure'
  )
);

export const TrackingProductionDurableWorkersReadinessBlockerReferenceSchema =
  brandedNonEmptyStringSchema('TrackingProductionDurableWorkersReadinessBlockerReference');
export const TrackingProductionDurableWorkersReadinessBlockerProofIdSchema =
  brandedNonEmptyStringSchema('TrackingProductionDurableWorkersReadinessBlockerProofId');
export const TrackingProductionDurableWorkersReadinessBlockerStatusSchema = withParser(
  Schema.Literal('manual-required')
);

const TrackingProductionDurableWorkersReadinessBlockerRowBaseSchema = Schema.Struct({
  blockerId: TrackingProductionDurableWorkersReadinessBlockerIdSchema,
  status: TrackingProductionDurableWorkersReadinessBlockerStatusSchema,
  sourceProofRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  productionSupportBoundaryRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  blockingArtifactRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  requiredProofTier: Schema.Literal('P4_PRODUCTION_RUNTIME'),
  currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
  productionWorkersClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingProductionDurableWorkersReadinessBlockerRowSchema = withParser(
  TrackingProductionDurableWorkersReadinessBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        (row.sourceProofRefs.length > 0 &&
          row.productionSupportBoundaryRefs.length > 0 &&
          row.blockingArtifactRefs.length > 0 &&
          row.productionWorkersClaimed === false &&
          row.productClaimReady === false) ||
        'Expected tracking production blockers to cite source proof refs, production support boundary refs, and blocking artifacts without product claims'
    )
  )
);

const TrackingProductionDurableWorkersReadinessBlockerProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingProductionDurableWorkersReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  proofMode: Schema.Literal('tracking-production-durable-workers-readiness-blocker-proof'),
  sourceProofRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  productionSupportDurableQueueRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  productionSupportManualClaimCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
  requiredTrackingWorkerArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
  presentTrackingWorkerArtifactCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  missingTrackingWorkerArtifactCount: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  requiredTrackingWorkerArtifactRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  presentTrackingWorkerArtifactRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  missingTrackingWorkerArtifactRefs: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
  blockers: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerRowSchema),
  productClaims: Schema.Struct({
    productionSupportBoundaryObserved: Schema.Literal(true),
    trackingLocationUploadWorkerClaimed: Schema.Literal(false),
    trackingRetentionCleanupWorkerClaimed: Schema.Literal(false),
    trackingNotificationOutboxWorkerClaimed: Schema.Literal(false),
    trackingEscalationTimeoutWorkerClaimed: Schema.Literal(false),
    trackingProviderReceiptWorkerClaimed: Schema.Literal(false),
    trackingChildDeviceDeliveryWorkerClaimed: Schema.Literal(false),
    trackingAuthorityStatusWorkerClaimed: Schema.Literal(false),
    trackingProductionAuditDurableStorageClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }),
});

export const TrackingProductionDurableWorkersReadinessBlockerProofSchema = withParser(
  TrackingProductionDurableWorkersReadinessBlockerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        trackingProductionDurableWorkersReadinessProofIsHonest(proof) ||
        'Expected tracking production durable workers blocker proof to consume production support durable queue context while keeping tracking production claims false'
    )
  )
);

export type TrackingProductionDurableWorkersReadinessBlockerId = Infer<
  typeof TrackingProductionDurableWorkersReadinessBlockerIdSchema
>;
export type TrackingProductionDurableWorkersReadinessBlockerProof = Infer<
  typeof TrackingProductionDurableWorkersReadinessBlockerProofSchema
>;
export type TrackingProductionDurableWorkersReadinessBlockerRow = Infer<
  typeof TrackingProductionDurableWorkersReadinessBlockerRowSchema
>;

export type TrackingProductionDurableWorkersReadinessBlockerProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProofRefs: readonly string[];
  readonly requiredTrackingWorkerArtifactRefs: readonly string[];
  readonly presentTrackingWorkerArtifactRefs?: readonly string[];
};

type TrackingProductionDurableWorkersReadinessBlockerProofInput = Infer<
  typeof TrackingProductionDurableWorkersReadinessBlockerProofBaseSchema
>;

export function buildTrackingProductionDurableWorkersReadinessBlockerProof(
  options: TrackingProductionDurableWorkersReadinessBlockerProofOptions,
  productionSupportDurableQueueProof: ProductionSupportStatusBackendDurableQueueRuntimeProof
): TrackingProductionDurableWorkersReadinessBlockerProof {
  const parsedProductionSupportProof = ProductionSupportStatusBackendDurableQueueRuntimeProofSchema.parse(
    productionSupportDurableQueueProof
  );
  const sourceProofRefs = uniqueRefs(options.sourceProofRefs);
  const productionSupportBoundaryRefs = uniqueRefs(parsedProductionSupportProof.sourceContractRefs);
  const requiredTrackingWorkerArtifactRefs = uniqueRefs(options.requiredTrackingWorkerArtifactRefs);
  const presentTrackingWorkerArtifactRefs = uniqueRefs(options.presentTrackingWorkerArtifactRefs ?? []).filter((ref) =>
    requiredTrackingWorkerArtifactRefs.includes(ref)
  );
  const missingTrackingWorkerArtifactRefs = requiredTrackingWorkerArtifactRefs.filter(
    (ref) => !presentTrackingWorkerArtifactRefs.includes(ref)
  );

  return TrackingProductionDurableWorkersReadinessBlockerProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    proofMode: 'tracking-production-durable-workers-readiness-blocker-proof',
    sourceProofRefs,
    productionSupportDurableQueueRows: parsedProductionSupportProof.rows.length,
    productionSupportManualClaimCount: productionSupportManualClaimCount(parsedProductionSupportProof),
    requiredTrackingWorkerArtifactCount: requiredTrackingWorkerArtifactRefs.length,
    presentTrackingWorkerArtifactCount: presentTrackingWorkerArtifactRefs.length,
    missingTrackingWorkerArtifactCount: missingTrackingWorkerArtifactRefs.length,
    requiredTrackingWorkerArtifactRefs,
    presentTrackingWorkerArtifactRefs,
    missingTrackingWorkerArtifactRefs,
    blockers: RequiredTrackingProductionDurableWorkersReadinessBlockers.map((blockerId) =>
      buildBlockerRow(blockerId, sourceProofRefs, productionSupportBoundaryRefs, requiredTrackingWorkerArtifactRefs)
    ),
    productClaims: {
      productionSupportBoundaryObserved: true,
      trackingLocationUploadWorkerClaimed: false,
      trackingRetentionCleanupWorkerClaimed: false,
      trackingNotificationOutboxWorkerClaimed: false,
      trackingEscalationTimeoutWorkerClaimed: false,
      trackingProviderReceiptWorkerClaimed: false,
      trackingChildDeviceDeliveryWorkerClaimed: false,
      trackingAuthorityStatusWorkerClaimed: false,
      trackingProductionAuditDurableStorageClaimed: false,
      productClaimReady: false,
    },
  });
}

export const RequiredTrackingProductionDurableWorkersReadinessBlockers = [
  'tracking-location-upload-worker-runtime',
  'tracking-retention-cleanup-worker-runtime',
  'tracking-notification-outbox-worker-runtime',
  'tracking-escalation-timeout-worker-runtime',
  'tracking-provider-receipt-worker-runtime',
  'tracking-child-device-delivery-worker-runtime',
  'tracking-authority-status-worker-runtime',
  'tracking-production-audit-durable-storage',
  'tracking-production-product-ready-closure',
] as const;

export const RequiredTrackingProductionDurableWorkerArtifactRefs = [
  'tracking-production/location-upload-worker-runtime.json',
  'tracking-production/retention-cleanup-worker-runtime.json',
  'tracking-production/notification-outbox-worker-runtime.json',
  'tracking-production/escalation-timeout-worker-runtime.json',
  'tracking-production/provider-receipt-worker-runtime.json',
  'tracking-production/child-device-delivery-worker-runtime.json',
  'tracking-production/authority-status-worker-runtime.json',
  'tracking-production/audit-durable-storage-runtime.json',
] as const;

function buildBlockerRow(
  blockerId: TrackingProductionDurableWorkersReadinessBlockerId,
  sourceProofRefs: readonly string[],
  productionSupportBoundaryRefs: readonly string[],
  blockingArtifactRefs: readonly string[]
): TrackingProductionDurableWorkersReadinessBlockerRow {
  return TrackingProductionDurableWorkersReadinessBlockerRowSchema.parse({
    blockerId,
    status: 'manual-required',
    sourceProofRefs,
    productionSupportBoundaryRefs,
    blockingArtifactRefs,
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    productionWorkersClaimed: false,
    productClaimReady: false,
  });
}

function trackingProductionDurableWorkersReadinessProofIsHonest(
  proof: TrackingProductionDurableWorkersReadinessBlockerProofInput
): boolean {
  return (
    proof.sourceProofRefs.length >= 5 &&
    proof.productionSupportDurableQueueRows > 0 &&
    proof.productionSupportManualClaimCount > 0 &&
    proof.requiredTrackingWorkerArtifactCount === RequiredTrackingProductionDurableWorkerArtifactRefs.length &&
    proof.requiredTrackingWorkerArtifactCount ===
      proof.presentTrackingWorkerArtifactCount + proof.missingTrackingWorkerArtifactCount &&
    proof.requiredTrackingWorkerArtifactRefs.length === proof.requiredTrackingWorkerArtifactCount &&
    proof.presentTrackingWorkerArtifactRefs.length === proof.presentTrackingWorkerArtifactCount &&
    proof.missingTrackingWorkerArtifactRefs.length === proof.missingTrackingWorkerArtifactCount &&
    proof.blockers.length === RequiredTrackingProductionDurableWorkersReadinessBlockers.length &&
    proof.blockers.every((row) => row.status === 'manual-required') &&
    proof.productClaims.productionSupportBoundaryObserved === true &&
    Object.entries(proof.productClaims)
      .filter(([key]) => key !== 'productionSupportBoundaryObserved')
      .every(([, claim]) => claim === false)
  );
}

function productionSupportManualClaimCount(proof: ProductionSupportStatusBackendDurableQueueRuntimeProof): number {
  return Object.entries(proof)
    .filter(([key]) => key.endsWith('Claim'))
    .filter(([, value]) => value === 'manual-required' || value === 'not-implemented').length;
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}

