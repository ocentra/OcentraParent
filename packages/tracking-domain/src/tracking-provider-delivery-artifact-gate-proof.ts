import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingProviderDeliveryArtifactGateTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingProviderDeliveryArtifactGateStatusSchema = Schema.Literal(
  'manual-required',
  'artifact-set-present'
);

export const TrackingProviderDeliveryArtifactGatePathSchema = TrackingProviderDeliveryArtifactGateTextSchema.pipe(
  Schema.brand('TrackingProviderDeliveryArtifactGatePath')
);

export const TrackingProviderDeliveryArtifactGateRowIdSchema = TrackingProviderDeliveryArtifactGateTextSchema.pipe(
  Schema.brand('TrackingProviderDeliveryArtifactGateRowId')
);

export const TrackingProviderDeliveryArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingProviderDeliveryArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingProviderDeliveryArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_MANUAL_PROVIDER_RUNTIME'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingProviderDeliveryArtifactGateStatusSchema,
    requiredArtifacts: Schema.Array(TrackingProviderDeliveryArtifactGatePathSchema),
    presentArtifacts: Schema.Array(TrackingProviderDeliveryArtifactGatePathSchema),
    missingArtifacts: Schema.Array(TrackingProviderDeliveryArtifactGatePathSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    providerDeliveryArtifactSetComplete: Schema.Boolean,
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
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Provider delivery rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Provider delivery rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.providerDeliveryArtifactSetComplete ||
          'Provider delivery artifact set status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.providerDeliveryArtifactSetComplete
            ? row.missingArtifacts.length === 0
            : row.missingArtifacts.length > 0) ||
          'Provider delivery artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingProviderDeliveryArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-provider-delivery-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingProviderDeliveryArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      providerDeliveryArtifactGateChecked: Schema.Literal(true),
      noProviderDeliveryRuntimeClaim: Schema.Literal(true),
      noWebhookReceiptIngestionRuntimeClaim: Schema.Literal(true),
      noProviderCredentialsClaim: Schema.Literal(true),
      noAdapterDispatchClaim: Schema.Literal(true),
      noRetryExecutionRuntimeClaim: Schema.Literal(true),
      noQuietHoursTimerRuntimeClaim: Schema.Literal(true),
      noParentNotificationUiRuntimeClaim: Schema.Literal(true),
      noProductionDurableOutboxStorageClaim: Schema.Literal(true),
      noChildDeviceDeliveryClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
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
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingProviderDeliveryArtifactPlan.proofRoot)) ||
        'Provider delivery artifact gate must cover the required notification provider proof root'
    )
  )
);

export type TrackingProviderDeliveryArtifactGateProof = Infer<typeof TrackingProviderDeliveryArtifactGateProofSchema>;
export type TrackingProviderDeliveryArtifactGateRow = Infer<typeof TrackingProviderDeliveryArtifactGateRowSchema>;

export interface TrackingProviderDeliveryArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingProviderDeliveryArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof/notification-provider-delivery',
  requiredArtifacts: [
    '00-run-metadata.json',
    '01-provider-runtime-config-redacted.json',
    '02-credential-presence-attestation.json',
    '03-minimal-payload-snapshot.json',
    '04-provider-attempt.json',
    '05-provider-response.json',
    '06-receipt-webhook-event.json',
    '07-receipt-ingestion-result.json',
    '08-retry-quiet-hours-worker-log.txt',
    '09-parent-notification-ui-screenshot.png',
    '10-result-summary.md',
  ],
} as const;

export function buildTrackingProviderDeliveryArtifactGateProof(
  generatedAt: string,
  inventory: TrackingProviderDeliveryArtifactInventory
): TrackingProviderDeliveryArtifactGateProof {
  return TrackingProviderDeliveryArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-provider-delivery-artifact-gate-proof',
    generatedAt,
    rows: [providerDeliveryArtifactRow(generatedAt, inventory)],
    proofClaims: {
      providerDeliveryArtifactGateChecked: true,
      noProviderDeliveryRuntimeClaim: true,
      noWebhookReceiptIngestionRuntimeClaim: true,
      noProviderCredentialsClaim: true,
      noAdapterDispatchClaim: true,
      noRetryExecutionRuntimeClaim: true,
      noQuietHoursTimerRuntimeClaim: true,
      noParentNotificationUiRuntimeClaim: true,
      noProductionDurableOutboxStorageClaim: true,
      noChildDeviceDeliveryClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProductReadyClaim: true,
    },
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

function providerDeliveryArtifactRow(
  generatedAt: string,
  inventory: TrackingProviderDeliveryArtifactInventory
): TrackingProviderDeliveryArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = RequiredTrackingProviderDeliveryArtifactPlan.requiredArtifacts;
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const providerDeliveryArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingProviderDeliveryArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-provider-delivery-artifacts-notification-runtime',
    generatedAt,
    proofRoot: RequiredTrackingProviderDeliveryArtifactPlan.proofRoot,
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: providerDeliveryArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts: [...requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-provider-delivery-artifacts-notification-runtime-audit'],
    providerDeliveryArtifactSetComplete,
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
  });
}
