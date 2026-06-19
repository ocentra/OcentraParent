import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  RequiredTrackingRetentionRuntimeArtifactRefs,
  RequiredTrackingRetentionRuntimeProductBlockers,
} from './tracking-retention-product-readiness-proof';

export const TrackingRetentionRuntimeArtifactGateStatusSchema = Schema.Literal(
  'manual-required',
  'artifact-set-present'
);

export const TrackingRetentionRuntimeArtifactGatePathSchema = brandedNonEmptyStringSchema('TrackingRetentionRuntimeArtifactGatePath');

export const TrackingRetentionRuntimeArtifactGateRowIdSchema = brandedNonEmptyStringSchema('TrackingRetentionRuntimeArtifactGateRowId');

export const TrackingRetentionRuntimeArtifactGateArtifactRefSchema =
  brandedNonEmptyStringSchema('TrackingRetentionRuntimeArtifactGateArtifactRef');

export const TrackingRetentionRuntimeArtifactGateBlockerSchema = Schema.Literal(
  ...RequiredTrackingRetentionRuntimeProductBlockers
);

export const TrackingRetentionRuntimeArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRetentionRuntimeArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingRetentionRuntimeArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_PRODUCTION_RUNTIME'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingRetentionRuntimeArtifactGateStatusSchema,
    sourceProductReadinessProofRef: TrackingRetentionRuntimeArtifactGatePathSchema,
    sourceProductReadinessBlockers: Schema.Array(TrackingRetentionRuntimeArtifactGateBlockerSchema),
    requiredArtifacts: Schema.Array(TrackingRetentionRuntimeArtifactGateArtifactRefSchema),
    presentArtifacts: Schema.Array(TrackingRetentionRuntimeArtifactGateArtifactRefSchema),
    missingArtifacts: Schema.Array(TrackingRetentionRuntimeArtifactGateArtifactRefSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    retentionRuntimeArtifactSetComplete: Schema.Boolean,
    writableProductSettingsExecutionClaimed: Schema.Literal(false),
    platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.sourceProductReadinessBlockers.length === RequiredTrackingRetentionRuntimeProductBlockers.length ||
          'Retention runtime rows need all retention runtime product blockers'
      )
    )
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Retention runtime rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Retention runtime rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.retentionRuntimeArtifactSetComplete ||
          'Retention runtime artifact status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.retentionRuntimeArtifactSetComplete
            ? row.missingArtifacts.length === 0
            : row.missingArtifacts.length > 0) ||
          'Retention runtime artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingRetentionRuntimeArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-runtime-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionRuntimeArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      retentionRuntimeArtifactGateChecked: Schema.Literal(true),
      productReadinessBlockerSourceObserved: Schema.Literal(true),
      noWritableProductSettingsExecutionClaim: Schema.Literal(true),
      noPlatformRuntimeRetentionEnforcementClaim: Schema.Literal(true),
      noChildDeviceDeliveryClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noNotificationReceiptClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductionWorkerClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      writableProductSettingsExecutionClaimed: Schema.Literal(false),
      platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingRetentionRuntimeArtifactPlan.proofRoot)) ||
        'Retention runtime artifact gate must cover the retention runtime proof root'
    )
  )
);

export type TrackingRetentionRuntimeArtifactGateProof = Infer<typeof TrackingRetentionRuntimeArtifactGateProofSchema>;
export type TrackingRetentionRuntimeArtifactGateRow = Infer<typeof TrackingRetentionRuntimeArtifactGateRowSchema>;

export interface TrackingRetentionRuntimeArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingRetentionRuntimeArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof',
  sourceProductReadinessProofRef:
    'output/tracking-plan-proof/07-retention-and-custody-model/24-retention-product-readiness-proof.json',
  sourceProductReadinessBlockers: RequiredTrackingRetentionRuntimeProductBlockers,
  requiredArtifacts: RequiredTrackingRetentionRuntimeArtifactRefs,
} as const;

export function buildTrackingRetentionRuntimeArtifactGateProof(
  generatedAt: string,
  inventory: TrackingRetentionRuntimeArtifactInventory
): TrackingRetentionRuntimeArtifactGateProof {
  return TrackingRetentionRuntimeArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-runtime-artifact-gate-proof',
    generatedAt,
    rows: [retentionRuntimeArtifactRow(generatedAt, inventory)],
    proofClaims: {
      retentionRuntimeArtifactGateChecked: true,
      productReadinessBlockerSourceObserved: true,
      noWritableProductSettingsExecutionClaim: true,
      noPlatformRuntimeRetentionEnforcementClaim: true,
      noChildDeviceDeliveryClaim: true,
      noProviderDeliveryClaim: true,
      noNotificationReceiptClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProductionWorkerClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      writableProductSettingsExecutionClaimed: false,
      platformRuntimeRetentionEnforcementClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function retentionRuntimeArtifactRow(
  generatedAt: string,
  inventory: TrackingRetentionRuntimeArtifactInventory
): TrackingRetentionRuntimeArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = RequiredTrackingRetentionRuntimeArtifactPlan.requiredArtifacts;
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const retentionRuntimeArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingRetentionRuntimeArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-retention-runtime-artifacts',
    generatedAt,
    proofRoot: RequiredTrackingRetentionRuntimeArtifactPlan.proofRoot,
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: retentionRuntimeArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    sourceProductReadinessProofRef: RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessProofRef,
    sourceProductReadinessBlockers: [...RequiredTrackingRetentionRuntimeArtifactPlan.sourceProductReadinessBlockers],
    requiredArtifacts: [...requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-retention-runtime-artifacts-audit'],
    retentionRuntimeArtifactSetComplete,
    writableProductSettingsExecutionClaimed: false,
    platformRuntimeRetentionEnforcementClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

