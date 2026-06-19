import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  RequiredTrackingProductionDurableWorkerArtifactRefs,
  TrackingProductionDurableWorkersReadinessBlockerReferenceSchema,
} from './tracking-production-durable-workers-readiness-blocker-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

export const TrackingProductionWorkerRuntimeArtifactGateStatusSchema = Schema.Literal(
  'manual-required',
  'artifact-set-present'
);

export const TrackingProductionWorkerRuntimeArtifactGatePathSchema =
  brandedNonEmptyStringSchema('TrackingProductionWorkerRuntimeArtifactGatePath');

export const TrackingProductionWorkerRuntimeArtifactGateRowIdSchema =
  brandedNonEmptyStringSchema('TrackingProductionWorkerRuntimeArtifactGateRowId');

export const TrackingProductionWorkerRuntimeArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingProductionWorkerRuntimeArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingProductionWorkerRuntimeArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_PRODUCTION_RUNTIME'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingProductionWorkerRuntimeArtifactGateStatusSchema,
    requiredArtifacts: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
    presentArtifacts: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
    missingArtifacts: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    productionWorkerArtifactSetComplete: Schema.Boolean,
    locationUploadWorkerRuntimeClaimed: Schema.Literal(false),
    retentionCleanupWorkerRuntimeClaimed: Schema.Literal(false),
    notificationOutboxWorkerRuntimeClaimed: Schema.Literal(false),
    escalationTimeoutWorkerRuntimeClaimed: Schema.Literal(false),
    providerReceiptWorkerRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryWorkerRuntimeClaimed: Schema.Literal(false),
    authorityStatusWorkerRuntimeClaimed: Schema.Literal(false),
    productionAuditDurableStorageClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryReceiptRuntimeClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Production worker rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Production worker rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.productionWorkerArtifactSetComplete ||
          'Production worker artifact set status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.productionWorkerArtifactSetComplete
            ? row.missingArtifacts.length === 0
            : row.missingArtifacts.length > 0) ||
          'Production worker artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingProductionWorkerRuntimeArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-production-worker-runtime-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingProductionWorkerRuntimeArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      productionWorkerArtifactGateChecked: Schema.Literal(true),
      noLocationUploadWorkerRuntimeClaim: Schema.Literal(true),
      noRetentionCleanupWorkerRuntimeClaim: Schema.Literal(true),
      noNotificationOutboxWorkerRuntimeClaim: Schema.Literal(true),
      noEscalationTimeoutWorkerRuntimeClaim: Schema.Literal(true),
      noProviderReceiptWorkerRuntimeClaim: Schema.Literal(true),
      noChildDeviceDeliveryWorkerRuntimeClaim: Schema.Literal(true),
      noAuthorityStatusWorkerRuntimeClaim: Schema.Literal(true),
      noProductionAuditDurableStorageClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryReceiptRuntimeClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      locationUploadWorkerRuntimeClaimed: Schema.Literal(false),
      retentionCleanupWorkerRuntimeClaimed: Schema.Literal(false),
      notificationOutboxWorkerRuntimeClaimed: Schema.Literal(false),
      escalationTimeoutWorkerRuntimeClaimed: Schema.Literal(false),
      providerReceiptWorkerRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryWorkerRuntimeClaimed: Schema.Literal(false),
      authorityStatusWorkerRuntimeClaimed: Schema.Literal(false),
      productionAuditDurableStorageClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryReceiptRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingProductionWorkerRuntimeArtifactPlan.proofRoot)) ||
        'Production worker runtime artifact gate must cover the tracking production proof root'
    )
  )
);

export type TrackingProductionWorkerRuntimeArtifactGateProof = Infer<
  typeof TrackingProductionWorkerRuntimeArtifactGateProofSchema
>;
export type TrackingProductionWorkerRuntimeArtifactGateRow = Infer<
  typeof TrackingProductionWorkerRuntimeArtifactGateRowSchema
>;

export interface TrackingProductionWorkerRuntimeArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingProductionWorkerRuntimeArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof',
  requiredArtifacts: RequiredTrackingProductionDurableWorkerArtifactRefs,
} as const;

export function buildTrackingProductionWorkerRuntimeArtifactGateProof(
  generatedAt: string,
  inventory: TrackingProductionWorkerRuntimeArtifactInventory
): TrackingProductionWorkerRuntimeArtifactGateProof {
  return TrackingProductionWorkerRuntimeArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-production-worker-runtime-artifact-gate-proof',
    generatedAt,
    rows: [productionWorkerArtifactRow(generatedAt, inventory)],
    proofClaims: {
      productionWorkerArtifactGateChecked: true,
      noLocationUploadWorkerRuntimeClaim: true,
      noRetentionCleanupWorkerRuntimeClaim: true,
      noNotificationOutboxWorkerRuntimeClaim: true,
      noEscalationTimeoutWorkerRuntimeClaim: true,
      noProviderReceiptWorkerRuntimeClaim: true,
      noChildDeviceDeliveryWorkerRuntimeClaim: true,
      noAuthorityStatusWorkerRuntimeClaim: true,
      noProductionAuditDurableStorageClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryReceiptRuntimeClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      locationUploadWorkerRuntimeClaimed: false,
      retentionCleanupWorkerRuntimeClaimed: false,
      notificationOutboxWorkerRuntimeClaimed: false,
      escalationTimeoutWorkerRuntimeClaimed: false,
      providerReceiptWorkerRuntimeClaimed: false,
      childDeviceDeliveryWorkerRuntimeClaimed: false,
      authorityStatusWorkerRuntimeClaimed: false,
      productionAuditDurableStorageClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryReceiptRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function productionWorkerArtifactRow(
  generatedAt: string,
  inventory: TrackingProductionWorkerRuntimeArtifactInventory
): TrackingProductionWorkerRuntimeArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts;
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const productionWorkerArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingProductionWorkerRuntimeArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-production-worker-runtime-artifacts',
    generatedAt,
    proofRoot: RequiredTrackingProductionWorkerRuntimeArtifactPlan.proofRoot,
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: productionWorkerArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts: [...requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-production-worker-runtime-artifacts-audit'],
    productionWorkerArtifactSetComplete,
    locationUploadWorkerRuntimeClaimed: false,
    retentionCleanupWorkerRuntimeClaimed: false,
    notificationOutboxWorkerRuntimeClaimed: false,
    escalationTimeoutWorkerRuntimeClaimed: false,
    providerReceiptWorkerRuntimeClaimed: false,
    childDeviceDeliveryWorkerRuntimeClaimed: false,
    authorityStatusWorkerRuntimeClaimed: false,
    productionAuditDurableStorageClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryReceiptRuntimeClaimed: false,
    productClaimReady: false,
  });
}

