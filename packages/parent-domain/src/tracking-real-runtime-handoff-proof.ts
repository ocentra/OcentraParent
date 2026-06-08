import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingRealRuntimeHandoffTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRealRuntimeHandoffAreaSchema = Schema.Literal(
  'physical-device-background-and-geofence',
  'child-device-runtime-execution',
  'full-product-parent-child-ui-runtime',
  'authority-enrolled-hard-control-runtime',
  'provider-delivery-receipt-runtime',
  'retention-product-runtime-enforcement',
  'production-durable-workers-and-storage',
  'escalation-runtime-workers-and-storage'
);

export const TrackingRealRuntimeHandoffProofTierSchema = Schema.Literal(
  'P4_PHYSICAL_DEVICE',
  'P4_MANUAL_PROVIDER_RUNTIME',
  'P4_PRODUCTION_RUNTIME'
);

export const TrackingRealRuntimeHandoffStatusSchema = Schema.Literal('manual-required', 'artifact-set-present');

export const TrackingRealRuntimeHandoffArtifactPathSchema = TrackingRealRuntimeHandoffTextSchema.pipe(
  Schema.brand('TrackingRealRuntimeHandoffArtifactPath')
);

export const TrackingRealRuntimeHandoffRowIdSchema = TrackingRealRuntimeHandoffTextSchema.pipe(
  Schema.brand('TrackingRealRuntimeHandoffRowId')
);

export const TrackingRealRuntimeHandoffRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRealRuntimeHandoffRowIdSchema,
    generatedAt: ParentTimestampSchema,
    handoffArea: TrackingRealRuntimeHandoffAreaSchema,
    sourceProofRef: TrackingRealRuntimeHandoffArtifactPathSchema,
    proofRoot: TrackingRealRuntimeHandoffArtifactPathSchema,
    requiredProofTier: TrackingRealRuntimeHandoffProofTierSchema,
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingRealRuntimeHandoffStatusSchema,
    requiredArtifacts: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    presentArtifacts: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    missingArtifacts: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    artifactSetComplete: Schema.Boolean,
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Real-runtime handoff rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Real-runtime handoff rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.artifactSetComplete ||
          'Real-runtime handoff status must match artifact completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.artifactSetComplete ? row.missingArtifacts.length === 0 : row.missingArtifacts.length > 0) ||
          'Real-runtime handoff completeness must match missing artifact count'
      )
    )
);

export const TrackingRealRuntimeHandoffProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-real-runtime-handoff-proof'),
    generatedAt: ParentTimestampSchema,
    requiredProofTier: Schema.Literal('P4_REAL_RUNTIME_HANDOFF'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    currentStatus: Schema.Literal('manual_required'),
    sourceGateRefs: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    closureProofRef: TrackingRealRuntimeHandoffArtifactPathSchema,
    handoffRows: Schema.Array(TrackingRealRuntimeHandoffRowSchema),
    summary: Schema.Struct({
      handoffRowCount: Schema.Number,
      requiredArtifactCount: Schema.Number,
      presentArtifactCount: Schema.Number,
      missingArtifactCount: Schema.Number,
      manualRequiredRowCount: Schema.Number,
      artifactSetPresentRowCount: Schema.Number,
      productReadyRowCount: Schema.Literal(0),
    }),
    productClaims: Schema.Struct({
      physicalDeviceClaimed: Schema.Literal(false),
      actualChildDeviceRuntimeClaimed: Schema.Literal(false),
      fullProductUiClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      providerDeliveryReceiptRuntimeClaimed: Schema.Literal(false),
      retentionProductRuntimeClaimed: Schema.Literal(false),
      productionWorkersClaimed: Schema.Literal(false),
      escalationRuntimeClaimed: Schema.Literal(false),
      productReadyClaimed: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.handoffRows.length === RequiredTrackingRealRuntimeHandoffGates.length ||
          'Real-runtime handoff proof must cover every required handoff gate'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.sourceGateRefs.length === RequiredTrackingRealRuntimeHandoffGates.length ||
          'Real-runtime handoff proof must cite every source gate'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.handoffRowCount === proof.handoffRows.length ||
          'Real-runtime handoff summary row count must match rows'
      )
    )
    .pipe(
      Schema.filter(
        (proof) => proof.summary.productReadyRowCount === 0 || 'Real-runtime handoff must not claim product-ready rows'
      )
    )
);

export type TrackingRealRuntimeHandoffProof = Infer<typeof TrackingRealRuntimeHandoffProofSchema>;
export type TrackingRealRuntimeHandoffRow = Infer<typeof TrackingRealRuntimeHandoffRowSchema>;

export interface TrackingRealRuntimeHandoffGateInventory {
  readonly handoffArea: (typeof RequiredTrackingRealRuntimeHandoffGates)[number]['handoffArea'];
  readonly proofRoot: string;
  readonly requiredArtifacts: readonly string[];
  readonly presentArtifacts: readonly string[];
  readonly auditRefs: readonly string[];
}

export const RequiredTrackingRealRuntimeHandoffGates = [
  {
    handoffArea: 'physical-device-background-and-geofence',
    sourceProofRef: 'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'child-device-runtime-execution',
    sourceProofRef: 'test-results/tracking-child-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'full-product-parent-child-ui-runtime',
    sourceProofRef: 'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'authority-enrolled-hard-control-runtime',
    sourceProofRef: 'test-results/tracking-authority-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'provider-delivery-receipt-runtime',
    sourceProofRef: 'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
  },
  {
    handoffArea: 'retention-product-runtime-enforcement',
    sourceProofRef: 'test-results/tracking-retention-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
  },
  {
    handoffArea: 'production-durable-workers-and-storage',
    sourceProofRef: 'test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
  },
  {
    handoffArea: 'escalation-runtime-workers-and-storage',
    sourceProofRef: 'test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
  },
] as const;

export function buildTrackingRealRuntimeHandoffProof(
  generatedAt: string,
  inventories: readonly TrackingRealRuntimeHandoffGateInventory[]
): TrackingRealRuntimeHandoffProof {
  const handoffRows = RequiredTrackingRealRuntimeHandoffGates.map((gate) =>
    realRuntimeHandoffRow(generatedAt, gate, inventories)
  );

  return TrackingRealRuntimeHandoffProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-real-runtime-handoff-proof',
    generatedAt,
    requiredProofTier: 'P4_REAL_RUNTIME_HANDOFF',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    sourceGateRefs: RequiredTrackingRealRuntimeHandoffGates.map((gate) => gate.sourceProofRef),
    closureProofRef: 'test-results/tracking-product-readiness-closure-proof/proof.json',
    handoffRows,
    summary: summarizeRealRuntimeHandoffRows(handoffRows),
    productClaims: {
      physicalDeviceClaimed: false,
      actualChildDeviceRuntimeClaimed: false,
      fullProductUiClaimed: false,
      authorityClaimed: false,
      providerDeliveryReceiptRuntimeClaimed: false,
      retentionProductRuntimeClaimed: false,
      productionWorkersClaimed: false,
      escalationRuntimeClaimed: false,
      productReadyClaimed: false,
    },
  });
}

function realRuntimeHandoffRow(
  generatedAt: string,
  gate: (typeof RequiredTrackingRealRuntimeHandoffGates)[number],
  inventories: readonly TrackingRealRuntimeHandoffGateInventory[]
): TrackingRealRuntimeHandoffRow {
  const inventory = inventories.find((candidate) => candidate.handoffArea === gate.handoffArea);
  const requiredArtifacts = inventory?.requiredArtifacts ?? [];
  const presentArtifactSet = new Set(inventory?.presentArtifacts ?? []);
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const artifactSetComplete = missingArtifacts.length === 0;

  return TrackingRealRuntimeHandoffRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-real-runtime-handoff-${gate.handoffArea}`,
    generatedAt,
    handoffArea: gate.handoffArea,
    sourceProofRef: gate.sourceProofRef,
    proofRoot: inventory?.proofRoot ?? gate.sourceProofRef,
    requiredProofTier: gate.requiredProofTier,
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: artifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts,
    presentArtifacts,
    missingArtifacts,
    auditRefs: [...(inventory?.auditRefs ?? [`tracking-real-runtime-handoff-${gate.handoffArea}-audit`])],
    artifactSetComplete,
    productClaimReady: false,
  });
}

function summarizeRealRuntimeHandoffRows(
  handoffRows: readonly TrackingRealRuntimeHandoffRow[]
): TrackingRealRuntimeHandoffProof['summary'] {
  return {
    handoffRowCount: handoffRows.length,
    requiredArtifactCount: handoffRows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
    presentArtifactCount: handoffRows.reduce((total, row) => total + row.presentArtifacts.length, 0),
    missingArtifactCount: handoffRows.reduce((total, row) => total + row.missingArtifacts.length, 0),
    manualRequiredRowCount: handoffRows.filter((row) => row.status === 'manual-required').length,
    artifactSetPresentRowCount: handoffRows.filter((row) => row.status === 'artifact-set-present').length,
    productReadyRowCount: 0,
  };
}
