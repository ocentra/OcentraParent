import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import {
  RequiredTrackingAuthorityRuntimeReadinessBlockers,
  TrackingAuthorityRuntimeReadinessBlockerIdSchema,
  TrackingAuthorityRuntimeReadinessBlockerProofSchema,
  TrackingAuthorityRuntimeReadinessBlockerReferenceSchema,
  type TrackingAuthorityRuntimeReadinessBlockerProof,
} from './tracking-authority-runtime-readiness-blocker-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

export const TrackingAuthorityRuntimeArtifactGateStatusSchema = Schema.Literal(
  'manual-required',
  'artifact-set-present'
);

export const TrackingAuthorityRuntimeArtifactGatePathSchema = brandedNonEmptyStringSchema('TrackingAuthorityRuntimeArtifactGatePath');

export const TrackingAuthorityRuntimeArtifactGateRowIdSchema = brandedNonEmptyStringSchema('TrackingAuthorityRuntimeArtifactGateRowId');

export const TrackingAuthorityRuntimeArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingAuthorityRuntimeArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingAuthorityRuntimeArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    currentProofTier: Schema.Literal('P0_CONTRACT'),
    status: TrackingAuthorityRuntimeArtifactGateStatusSchema,
    sourceRuntimeReadinessProofRef: TrackingAuthorityRuntimeArtifactGatePathSchema,
    sourceRuntimeReadinessBlockers: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerIdSchema),
    requiredArtifacts: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerReferenceSchema),
    presentArtifacts: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerReferenceSchema),
    missingArtifacts: Schema.Array(TrackingAuthorityRuntimeReadinessBlockerReferenceSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    authorityRuntimeArtifactSetComplete: Schema.Boolean,
    authorityEnrollmentClaimed: Schema.Literal(false),
    hardControlRuntimeClaimed: Schema.Literal(false),
    parentVisibleAuthorityStatusClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.sourceRuntimeReadinessBlockers.length === RequiredTrackingAuthorityRuntimeReadinessBlockers.length ||
          'Authority runtime artifact rows need every runtime readiness blocker source'
      )
    )
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Authority runtime rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Authority runtime rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.authorityRuntimeArtifactSetComplete ||
          'Authority runtime artifact status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.authorityRuntimeArtifactSetComplete
            ? row.missingArtifacts.length === 0
            : row.missingArtifacts.length > 0) ||
          'Authority runtime artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingAuthorityRuntimeArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-authority-runtime-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingAuthorityRuntimeArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      authorityRuntimeArtifactGateChecked: Schema.Literal(true),
      runtimeReadinessBlockerSourceObserved: Schema.Literal(true),
      noAuthorityEnrollmentClaim: Schema.Literal(true),
      noHardControlRuntimeClaim: Schema.Literal(true),
      noParentVisibleAuthorityStatusClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noProductionWorkerClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      authorityEnrollmentClaimed: Schema.Literal(false),
      hardControlRuntimeClaimed: Schema.Literal(false),
      parentVisibleAuthorityStatusClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingAuthorityRuntimeArtifactPlan.proofRoot)) ||
        'Authority runtime artifact gate must cover the tracking authority proof root'
    )
  )
);

export type TrackingAuthorityRuntimeArtifactGateProof = Infer<typeof TrackingAuthorityRuntimeArtifactGateProofSchema>;
export type TrackingAuthorityRuntimeArtifactGateRow = Infer<typeof TrackingAuthorityRuntimeArtifactGateRowSchema>;

export interface TrackingAuthorityRuntimeArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingAuthorityRuntimeArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof',
  sourceRuntimeReadinessProofRef:
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/55-authority-runtime-readiness-blocker-proof.json',
  sourceRuntimeReadinessBlockers: RequiredTrackingAuthorityRuntimeReadinessBlockers,
} as const;

export function buildTrackingAuthorityRuntimeArtifactGateProof(
  generatedAt: string,
  runtimeReadinessProof: TrackingAuthorityRuntimeReadinessBlockerProof,
  inventory: TrackingAuthorityRuntimeArtifactInventory
): TrackingAuthorityRuntimeArtifactGateProof {
  const parsedRuntimeReadinessProof = TrackingAuthorityRuntimeReadinessBlockerProofSchema.parse(runtimeReadinessProof);

  return TrackingAuthorityRuntimeArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-authority-runtime-artifact-gate-proof',
    generatedAt,
    rows: [authorityRuntimeArtifactRow(generatedAt, parsedRuntimeReadinessProof, inventory)],
    proofClaims: {
      authorityRuntimeArtifactGateChecked: true,
      runtimeReadinessBlockerSourceObserved: true,
      noAuthorityEnrollmentClaim: true,
      noHardControlRuntimeClaim: true,
      noParentVisibleAuthorityStatusClaim: true,
      noPhysicalDeviceProofClaim: true,
      noProviderDeliveryClaim: true,
      noProductionWorkerClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      authorityEnrollmentClaimed: false,
      hardControlRuntimeClaimed: false,
      parentVisibleAuthorityStatusClaimed: false,
      physicalDeviceProofClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function authorityRuntimeArtifactRow(
  generatedAt: string,
  runtimeReadinessProof: TrackingAuthorityRuntimeReadinessBlockerProof,
  inventory: TrackingAuthorityRuntimeArtifactInventory
): TrackingAuthorityRuntimeArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = uniqueRefs(runtimeReadinessProof.blockers.flatMap((row) => row.blockingEvidenceRefs));
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const authorityRuntimeArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingAuthorityRuntimeArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-authority-runtime-artifacts',
    generatedAt,
    proofRoot: RequiredTrackingAuthorityRuntimeArtifactPlan.proofRoot,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P0_CONTRACT',
    status: authorityRuntimeArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    sourceRuntimeReadinessProofRef: RequiredTrackingAuthorityRuntimeArtifactPlan.sourceRuntimeReadinessProofRef,
    sourceRuntimeReadinessBlockers: [...RequiredTrackingAuthorityRuntimeArtifactPlan.sourceRuntimeReadinessBlockers],
    requiredArtifacts,
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-authority-runtime-artifacts-audit'],
    authorityRuntimeArtifactSetComplete,
    authorityEnrollmentClaimed: false,
    hardControlRuntimeClaimed: false,
    parentVisibleAuthorityStatusClaimed: false,
    physicalDeviceProofClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}

