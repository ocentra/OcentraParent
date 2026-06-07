import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingChildRuntimeArtifactGateTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingChildRuntimeArtifactGateStatusSchema = Schema.Literal('manual-required', 'artifact-set-present');

export const TrackingChildRuntimeArtifactGatePathSchema = TrackingChildRuntimeArtifactGateTextSchema.pipe(
  Schema.brand('TrackingChildRuntimeArtifactGatePath')
);

export const TrackingChildRuntimeArtifactGateRowIdSchema = TrackingChildRuntimeArtifactGateTextSchema.pipe(
  Schema.brand('TrackingChildRuntimeArtifactGateRowId')
);

export const TrackingChildRuntimeArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingChildRuntimeArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingChildRuntimeArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingChildRuntimeArtifactGateStatusSchema,
    requiredArtifacts: Schema.Array(TrackingChildRuntimeArtifactGatePathSchema),
    presentArtifacts: Schema.Array(TrackingChildRuntimeArtifactGatePathSchema),
    missingArtifacts: Schema.Array(TrackingChildRuntimeArtifactGatePathSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    childRuntimeArtifactSetComplete: Schema.Boolean,
    childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
    childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
    renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
    parentReceiptRuntimeClaimed: Schema.Literal(false),
    runtimeObservationClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Child runtime rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Child runtime rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.childRuntimeArtifactSetComplete ||
          'Child runtime artifact set status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.childRuntimeArtifactSetComplete ? row.missingArtifacts.length === 0 : row.missingArtifacts.length > 0) ||
          'Child runtime artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingChildRuntimeArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-child-runtime-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingChildRuntimeArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      childRuntimeArtifactGateChecked: Schema.Literal(true),
      noChildDeviceDeliveryRuntimeClaim: Schema.Literal(true),
      noChildDeviceExecutionRuntimeClaim: Schema.Literal(true),
      noRenderedChildDeviceUiRuntimeClaim: Schema.Literal(true),
      noParentReceiptRuntimeClaim: Schema.Literal(true),
      noRuntimeObservationClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      childDeviceExecutionRuntimeClaimed: Schema.Literal(false),
      renderedChildDeviceUiRuntimeClaimed: Schema.Literal(false),
      parentReceiptRuntimeClaimed: Schema.Literal(false),
      runtimeObservationClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingChildRuntimeArtifactPlan.proofRoot)) ||
        'Child runtime artifact gate must cover the required child-device runtime proof root'
    )
  )
);

export type TrackingChildRuntimeArtifactGateProof = Infer<typeof TrackingChildRuntimeArtifactGateProofSchema>;
export type TrackingChildRuntimeArtifactGateRow = Infer<typeof TrackingChildRuntimeArtifactGateRowSchema>;

export interface TrackingChildRuntimeArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingChildRuntimeArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof/child-device-runtime-execution',
  requiredArtifacts: [
    '00-run-metadata.json',
    '01-child-device-metadata.json',
    '02-delivery-envelope.json',
    '03-execution-result.json',
    '04-visible-child-ui-snapshot.png',
    '05-parent-receipt.json',
    '06-runtime-observation.ndjson',
    '07-permission-consent-state.json',
    '08-device-log.txt',
    '09-result-summary.md',
  ],
} as const;

export function buildTrackingChildRuntimeArtifactGateProof(
  generatedAt: string,
  inventory: TrackingChildRuntimeArtifactInventory
): TrackingChildRuntimeArtifactGateProof {
  return TrackingChildRuntimeArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-child-runtime-artifact-gate-proof',
    generatedAt,
    rows: [childRuntimeArtifactRow(generatedAt, inventory)],
    proofClaims: {
      childRuntimeArtifactGateChecked: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noChildDeviceExecutionRuntimeClaim: true,
      noRenderedChildDeviceUiRuntimeClaim: true,
      noParentReceiptRuntimeClaim: true,
      noRuntimeObservationClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      childDeviceDeliveryRuntimeClaimed: false,
      childDeviceExecutionRuntimeClaimed: false,
      renderedChildDeviceUiRuntimeClaimed: false,
      parentReceiptRuntimeClaimed: false,
      runtimeObservationClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function childRuntimeArtifactRow(
  generatedAt: string,
  inventory: TrackingChildRuntimeArtifactInventory
): TrackingChildRuntimeArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = RequiredTrackingChildRuntimeArtifactPlan.requiredArtifacts;
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const childRuntimeArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingChildRuntimeArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-child-runtime-artifacts-device-execution',
    generatedAt,
    proofRoot: RequiredTrackingChildRuntimeArtifactPlan.proofRoot,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: childRuntimeArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts: [...requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-child-runtime-artifacts-device-execution-audit'],
    childRuntimeArtifactSetComplete,
    childDeviceDeliveryRuntimeClaimed: false,
    childDeviceExecutionRuntimeClaimed: false,
    renderedChildDeviceUiRuntimeClaimed: false,
    parentReceiptRuntimeClaimed: false,
    runtimeObservationClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}
