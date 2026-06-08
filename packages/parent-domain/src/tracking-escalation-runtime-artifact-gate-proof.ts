import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  RequiredTrackingEscalationRuntimeReadinessBlockers,
  TrackingEscalationRuntimeReadinessBlockerIdSchema,
  TrackingEscalationRuntimeReadinessBlockerProofSchema,
  TrackingEscalationRuntimeReadinessBlockerReferenceSchema,
  type TrackingEscalationRuntimeReadinessBlockerProof,
} from './tracking-escalation-runtime-readiness-blocker-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingEscalationRuntimeArtifactGateTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingEscalationRuntimeArtifactGateStatusSchema = Schema.Literal(
  'manual-required',
  'artifact-set-present'
);

export const TrackingEscalationRuntimeArtifactGatePathSchema = TrackingEscalationRuntimeArtifactGateTextSchema.pipe(
  Schema.brand('TrackingEscalationRuntimeArtifactGatePath')
);

export const TrackingEscalationRuntimeArtifactGateRowIdSchema = TrackingEscalationRuntimeArtifactGateTextSchema.pipe(
  Schema.brand('TrackingEscalationRuntimeArtifactGateRowId')
);

export const TrackingEscalationRuntimeArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingEscalationRuntimeArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    proofRoot: TrackingEscalationRuntimeArtifactGatePathSchema,
    requiredProofTier: Schema.Literal('P4_MANUAL_ESCALATION_RUNTIME'),
    currentProofTier: Schema.Literal('P1_FIXTURE_SIMULATION'),
    status: TrackingEscalationRuntimeArtifactGateStatusSchema,
    sourceRuntimeReadinessProofRef: TrackingEscalationRuntimeArtifactGatePathSchema,
    sourceRuntimeReadinessBlockers: Schema.Array(TrackingEscalationRuntimeReadinessBlockerIdSchema),
    requiredArtifacts: Schema.Array(TrackingEscalationRuntimeReadinessBlockerReferenceSchema),
    presentArtifacts: Schema.Array(TrackingEscalationRuntimeReadinessBlockerReferenceSchema),
    missingArtifacts: Schema.Array(TrackingEscalationRuntimeReadinessBlockerReferenceSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    escalationRuntimeArtifactSetComplete: Schema.Boolean,
    productionEscalationWorkerRuntimeClaimed: Schema.Literal(false),
    productionQuietHoursTimerRuntimeClaimed: Schema.Literal(false),
    providerDeliveryRuntimeClaimed: Schema.Literal(false),
    providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
    parentNotificationHistoryRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
    durableEscalationStorageClaimed: Schema.Literal(false),
    emergencyAutoContactPolicyClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.sourceRuntimeReadinessBlockers.length === RequiredTrackingEscalationRuntimeReadinessBlockers.length ||
          'Escalation runtime artifact rows need every runtime readiness blocker source'
      )
    )
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Escalation runtime rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Escalation runtime rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.escalationRuntimeArtifactSetComplete ||
          'Escalation runtime artifact status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.escalationRuntimeArtifactSetComplete
            ? row.missingArtifacts.length === 0
            : row.missingArtifacts.length > 0) ||
          'Escalation runtime artifact completeness must match missing artifact count'
      )
    )
);

export const TrackingEscalationRuntimeArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-escalation-runtime-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingEscalationRuntimeArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      escalationRuntimeArtifactGateChecked: Schema.Literal(true),
      runtimeReadinessBlockerSourceObserved: Schema.Literal(true),
      noProductionEscalationWorkerRuntimeClaim: Schema.Literal(true),
      noProductionQuietHoursTimerRuntimeClaim: Schema.Literal(true),
      noProviderDeliveryRuntimeClaim: Schema.Literal(true),
      noProviderReceiptIngestionRuntimeClaim: Schema.Literal(true),
      noParentNotificationHistoryRuntimeClaim: Schema.Literal(true),
      noChildDeviceDeliveryRuntimeClaim: Schema.Literal(true),
      noDurableEscalationStorageClaim: Schema.Literal(true),
      noEmergencyAutoContactPolicyClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      productionEscalationWorkerRuntimeClaimed: Schema.Literal(false),
      productionQuietHoursTimerRuntimeClaimed: Schema.Literal(false),
      providerDeliveryRuntimeClaimed: Schema.Literal(false),
      providerReceiptIngestionRuntimeClaimed: Schema.Literal(false),
      parentNotificationHistoryRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryRuntimeClaimed: Schema.Literal(false),
      durableEscalationStorageClaimed: Schema.Literal(false),
      emergencyAutoContactPolicyClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === 1 &&
          proof.rows.some((row) => row.proofRoot === RequiredTrackingEscalationRuntimeArtifactPlan.proofRoot)) ||
        'Escalation runtime artifact gate must cover the tracking escalation proof root'
    )
  )
);

export type TrackingEscalationRuntimeArtifactGateProof = Infer<typeof TrackingEscalationRuntimeArtifactGateProofSchema>;
export type TrackingEscalationRuntimeArtifactGateRow = Infer<typeof TrackingEscalationRuntimeArtifactGateRowSchema>;

export interface TrackingEscalationRuntimeArtifactInventory {
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingEscalationRuntimeArtifactPlan = {
  proofRoot: 'output/tracking-plan-proof',
  sourceRuntimeReadinessProofRef:
    'output/tracking-plan-proof/33-proof-gates-fixtures-rollout-and-pr-gate/53-escalation-runtime-readiness-blocker-proof.json',
  sourceRuntimeReadinessBlockers: RequiredTrackingEscalationRuntimeReadinessBlockers,
} as const;

export function buildTrackingEscalationRuntimeArtifactGateProof(
  generatedAt: string,
  runtimeReadinessProof: TrackingEscalationRuntimeReadinessBlockerProof,
  inventory: TrackingEscalationRuntimeArtifactInventory
): TrackingEscalationRuntimeArtifactGateProof {
  const parsedRuntimeReadinessProof = TrackingEscalationRuntimeReadinessBlockerProofSchema.parse(runtimeReadinessProof);

  return TrackingEscalationRuntimeArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-escalation-runtime-artifact-gate-proof',
    generatedAt,
    rows: [escalationRuntimeArtifactRow(generatedAt, parsedRuntimeReadinessProof, inventory)],
    proofClaims: {
      escalationRuntimeArtifactGateChecked: true,
      runtimeReadinessBlockerSourceObserved: true,
      noProductionEscalationWorkerRuntimeClaim: true,
      noProductionQuietHoursTimerRuntimeClaim: true,
      noProviderDeliveryRuntimeClaim: true,
      noProviderReceiptIngestionRuntimeClaim: true,
      noParentNotificationHistoryRuntimeClaim: true,
      noChildDeviceDeliveryRuntimeClaim: true,
      noDurableEscalationStorageClaim: true,
      noEmergencyAutoContactPolicyClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      productionEscalationWorkerRuntimeClaimed: false,
      productionQuietHoursTimerRuntimeClaimed: false,
      providerDeliveryRuntimeClaimed: false,
      providerReceiptIngestionRuntimeClaimed: false,
      parentNotificationHistoryRuntimeClaimed: false,
      childDeviceDeliveryRuntimeClaimed: false,
      durableEscalationStorageClaimed: false,
      emergencyAutoContactPolicyClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productClaimReady: false,
    },
  });
}

function escalationRuntimeArtifactRow(
  generatedAt: string,
  runtimeReadinessProof: TrackingEscalationRuntimeReadinessBlockerProof,
  inventory: TrackingEscalationRuntimeArtifactInventory
): TrackingEscalationRuntimeArtifactGateRow {
  const presentArtifactSet = new Set(inventory.presentArtifacts);
  const requiredArtifacts = uniqueRefs(runtimeReadinessProof.blockers.flatMap((row) => row.blockingArtifactRefs));
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const escalationRuntimeArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingEscalationRuntimeArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-escalation-runtime-artifacts',
    generatedAt,
    proofRoot: RequiredTrackingEscalationRuntimeArtifactPlan.proofRoot,
    requiredProofTier: 'P4_MANUAL_ESCALATION_RUNTIME',
    currentProofTier: 'P1_FIXTURE_SIMULATION',
    status: escalationRuntimeArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    sourceRuntimeReadinessProofRef: RequiredTrackingEscalationRuntimeArtifactPlan.sourceRuntimeReadinessProofRef,
    sourceRuntimeReadinessBlockers: [...RequiredTrackingEscalationRuntimeArtifactPlan.sourceRuntimeReadinessBlockers],
    requiredArtifacts,
    presentArtifacts,
    missingArtifacts,
    auditRefs: ['tracking-escalation-runtime-artifacts-audit'],
    escalationRuntimeArtifactSetComplete,
    productionEscalationWorkerRuntimeClaimed: false,
    productionQuietHoursTimerRuntimeClaimed: false,
    providerDeliveryRuntimeClaimed: false,
    providerReceiptIngestionRuntimeClaimed: false,
    parentNotificationHistoryRuntimeClaimed: false,
    childDeviceDeliveryRuntimeClaimed: false,
    durableEscalationStorageClaimed: false,
    emergencyAutoContactPolicyClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productClaimReady: false,
  });
}

function uniqueRefs(refs: readonly string[]): readonly string[] {
  return [...new Set(refs)];
}
