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
} from '@ocentra-parent/schema-domain/family-reference-primitives';
import {
  TrackingEscalationReadinessReadModelSchema,
  type TrackingEscalationReadinessReadModel,
} from './tracking-escalation-readiness-proof';
import {
  TrackingProviderRuntimeReadinessBlockerProofSchema,
  type TrackingProviderRuntimeReadinessBlockerProof,
} from './tracking-provider-runtime-readiness-blocker-proof';

export const TrackingEscalationRuntimeReadinessBlockerIdSchema = withParser(
  Schema.Literal(
    'production-escalation-worker-runtime',
    'production-quiet-hours-timer-runtime',
    'provider-delivery-runtime',
    'provider-receipt-ingestion-runtime',
    'provider-credentials',
    'parent-notification-history-runtime',
    'child-device-delivery-runtime',
    'durable-escalation-storage',
    'physical-device-proof',
    'authority-proof',
    'emergency-auto-contact-policy',
    'product-ready-tracking-escalation'
  )
);

export const TrackingEscalationRuntimeReadinessBlockerReferenceSchema =
  brandedNonEmptyStringSchema('TrackingEscalationRuntimeReadinessBlockerReference');
export const TrackingEscalationRuntimeReadinessBlockerProofIdSchema =
  brandedNonEmptyStringSchema('TrackingEscalationRuntimeReadinessBlockerProofId');

export const TrackingEscalationRuntimeReadinessBlockerStatusSchema = withParser(Schema.Literal('manual-required'));

const TrackingEscalationRuntimeReadinessBlockerRowBaseSchema = Schema.Struct({
  blockerId: TrackingEscalationRuntimeReadinessBlockerIdSchema,
  status: TrackingEscalationRuntimeReadinessBlockerStatusSchema,
  sourceProofRefs: Schema.Array(TrackingEscalationRuntimeReadinessBlockerReferenceSchema),
  blockingArtifactRefs: Schema.Array(TrackingEscalationRuntimeReadinessBlockerReferenceSchema),
  requiredProofTier: Schema.Literal('P4_MANUAL_ESCALATION_RUNTIME'),
  currentProofTier: Schema.Literal('P1_FIXTURE_SIMULATION'),
  productClaimReady: Schema.Literal(false),
});

export const TrackingEscalationRuntimeReadinessBlockerRowSchema = withParser(
  TrackingEscalationRuntimeReadinessBlockerRowBaseSchema.pipe(
    Schema.filter(
      (row) => row.sourceProofRefs.length > 0 && row.blockingArtifactRefs.length > 0 && row.productClaimReady === false
    )
  )
);

const TrackingEscalationRuntimeReadinessBlockerProofBaseSchema = Schema.Struct({
  schemaVersion: ParentContractSchemaVersionSchema,
  proofId: TrackingEscalationRuntimeReadinessBlockerProofIdSchema,
  generatedAt: ParentTimestampSchema,
  proofMode: Schema.Literal('tracking-escalation-runtime-readiness-blocker-proof'),
  sourceProofRefs: Schema.Array(TrackingEscalationRuntimeReadinessBlockerReferenceSchema),
  escalationReadinessRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  escalationManualRequiredRows: Schema.Number.pipe(Schema.int(), Schema.nonNegative()),
  providerRuntimeBlockerRows: Schema.Number.pipe(Schema.int(), Schema.positive()),
  blockers: Schema.Array(TrackingEscalationRuntimeReadinessBlockerRowSchema),
  productClaims: Schema.Struct({
    productionEscalationWorkerRuntimeClaimed: Schema.Literal(false),
    productionQuietHoursTimerRuntimeClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
    providerCredentialsClaimed: Schema.Literal(false),
    parentNotificationHistoryRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
    durableEscalationStorageClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    emergencyAutoContactPolicyClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  }),
});

export const TrackingEscalationRuntimeReadinessBlockerProofSchema = withParser(
  TrackingEscalationRuntimeReadinessBlockerProofBaseSchema.pipe(
    Schema.filter(
      (proof) =>
        trackingEscalationRuntimeReadinessBlockerProofIsHonest(proof) ||
        'Expected escalation runtime blocker proof to consume escalation/provider refs and keep every runtime/product claim false'
    )
  )
);

export type TrackingEscalationRuntimeReadinessBlockerId = Infer<
  typeof TrackingEscalationRuntimeReadinessBlockerIdSchema
>;
export type TrackingEscalationRuntimeReadinessBlockerProof = Infer<
  typeof TrackingEscalationRuntimeReadinessBlockerProofSchema
>;
export type TrackingEscalationRuntimeReadinessBlockerRow = Infer<
  typeof TrackingEscalationRuntimeReadinessBlockerRowSchema
>;

export type TrackingEscalationRuntimeReadinessBlockerProofOptions = {
  readonly generatedAt: string;
  readonly proofId: string;
  readonly sourceProofRefs: readonly string[];
};

type TrackingEscalationRuntimeReadinessBlockerProofInput = Infer<
  typeof TrackingEscalationRuntimeReadinessBlockerProofBaseSchema
>;

export function buildTrackingEscalationRuntimeReadinessBlockerProof(
  options: TrackingEscalationRuntimeReadinessBlockerProofOptions,
  escalationReadiness: TrackingEscalationReadinessReadModel,
  providerRuntimeBlocker: TrackingProviderRuntimeReadinessBlockerProof
): TrackingEscalationRuntimeReadinessBlockerProof {
  const parsedEscalationReadiness = TrackingEscalationReadinessReadModelSchema.parse(escalationReadiness);
  const parsedProviderRuntimeBlocker = TrackingProviderRuntimeReadinessBlockerProofSchema.parse(providerRuntimeBlocker);
  const sourceProofRefs = uniqueRefs(options.sourceProofRefs);
  const blockingArtifactRefs = uniqueRefs([
    ...sourceProofRefs,
    ...parsedProviderRuntimeBlocker.blockers.flatMap((row) => row.blockingArtifactRefs),
  ]);

  return TrackingEscalationRuntimeReadinessBlockerProofSchema.parse({
    schemaVersion: ParentContractSchemaVersion.V0_6,
    proofId: options.proofId,
    generatedAt: options.generatedAt,
    proofMode: 'tracking-escalation-runtime-readiness-blocker-proof',
    sourceProofRefs,
    escalationReadinessRows: parsedEscalationReadiness.rows.length,
    escalationManualRequiredRows: parsedEscalationReadiness.manualRequiredCount,
    providerRuntimeBlockerRows: parsedProviderRuntimeBlocker.blockers.length,
    blockers: RequiredTrackingEscalationRuntimeReadinessBlockers.map((blockerId) =>
      buildBlockerRow(blockerId, sourceProofRefs, blockingArtifactRefs)
    ),
    productClaims: {
      productionEscalationWorkerRuntimeClaimed: false,
      productionQuietHoursTimerRuntimeClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      providerReceiptIngestionRuntimeClaimed: false,
      providerCredentialsClaimed: false,
      parentNotificationHistoryRuntimeClaimed: false,
      childDeviceDeliveryRuntimeClaimed: false,
      durableEscalationStorageClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      emergencyAutoContactPolicyClaimed: false,
      productClaimReady: false,
    },
  });
}

export const RequiredTrackingEscalationRuntimeReadinessBlockers = [
  'production-escalation-worker-runtime',
  'production-quiet-hours-timer-runtime',
  'provider-delivery-runtime',
  'provider-receipt-ingestion-runtime',
  'provider-credentials',
  'parent-notification-history-runtime',
  'child-device-delivery-runtime',
  'durable-escalation-storage',
  'physical-device-proof',
  'authority-proof',
  'emergency-auto-contact-policy',
  'product-ready-tracking-escalation',
] as const;

function buildBlockerRow(
  blockerId: TrackingEscalationRuntimeReadinessBlockerId,
  sourceProofRefs: readonly string[],
  blockingArtifactRefs: readonly string[]
): TrackingEscalationRuntimeReadinessBlockerRow {
  return TrackingEscalationRuntimeReadinessBlockerRowSchema.parse({
    blockerId,
    status: 'manual-required',
    sourceProofRefs,
    blockingArtifactRefs,
    requiredProofTier: 'P4_MANUAL_ESCALATION_RUNTIME',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    productClaimReady: false,
  });
}

function trackingEscalationRuntimeReadinessBlockerProofIsHonest(
  proof: TrackingEscalationRuntimeReadinessBlockerProofInput
): boolean {
  return (
    proof.sourceProofRefs.length >= 2 &&
    proof.escalationReadinessRows > 0 &&
    proof.providerRuntimeBlockerRows > 0 &&
    proof.blockers.length === RequiredTrackingEscalationRuntimeReadinessBlockers.length &&
    proof.blockers.every((row) => row.status === 'manual-required') &&
    Object.values(proof.productClaims).every((claim) => claim === false)
  );
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}

