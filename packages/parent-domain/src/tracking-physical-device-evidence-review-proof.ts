import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import {
  TrackingPhysicalDeviceArtifactGateProofSchema,
  TrackingPhysicalDeviceArtifactGateRowIdSchema,
  TrackingPhysicalDeviceArtifactPathSchema,
  TrackingPhysicalDeviceArtifactPlatformSchema,
  type TrackingPhysicalDeviceArtifactGateProof,
  type TrackingPhysicalDeviceArtifactGateRow,
} from './tracking-physical-device-artifact-gate-proof';
import { TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingPhysicalDeviceEvidenceReviewTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingPhysicalDeviceEvidenceReviewRowIdSchema = TrackingPhysicalDeviceEvidenceReviewTextSchema.pipe(
  Schema.brand('TrackingPhysicalDeviceEvidenceReviewRowId')
);

export const TrackingPhysicalDeviceEvidenceReviewStatusSchema = Schema.Literal(
  'artifact-missing',
  'content-review-required'
);

export const TrackingPhysicalDeviceEvidenceReviewRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingPhysicalDeviceEvidenceReviewRowIdSchema,
    generatedAt: ParentTimestampSchema,
    platform: TrackingPhysicalDeviceArtifactPlatformSchema,
    sourceArtifactGateRowId: TrackingPhysicalDeviceArtifactGateRowIdSchema,
    proofRoot: TrackingPhysicalDeviceArtifactPathSchema,
    status: TrackingPhysicalDeviceEvidenceReviewStatusSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE_CONTENT_REVIEW'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    requiredArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    presentArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    missingArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    supportingStatusProofRef: TrackingPhysicalDeviceArtifactPathSchema,
    supportingStatusArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    acceptanceCriteria: Schema.Array(TrackingPhysicalDeviceEvidenceReviewTextSchema),
    manualValidationCommands: Schema.Array(TrackingPhysicalDeviceEvidenceReviewTextSchema),
    artifactAcceptanceNotes: Schema.Array(TrackingPhysicalDeviceEvidenceReviewTextSchema),
    artifactSetComplete: Schema.Boolean,
    physicalDeviceStatusObserved: Schema.Boolean,
    reviewerRequired: Schema.Literal(true),
    contentAccepted: Schema.Literal(false),
    physicalDeviceBehaviorClaimed: Schema.Literal(false),
    authorityEnrollmentClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.acceptanceCriteria.length >= 4 || 'Evidence review rows need criteria'))
    .pipe(
      Schema.filter((row) => row.manualValidationCommands.length >= 4 || 'Evidence review rows need manual commands')
    )
    .pipe(
      Schema.filter((row) => row.artifactAcceptanceNotes.length >= 4 || 'Evidence review rows need acceptance notes')
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Evidence review rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.artifactSetComplete
            ? row.status === 'content-review-required' && row.missingArtifacts.length === 0
            : row.status === 'artifact-missing' && row.missingArtifacts.length > 0) ||
          'Evidence review status must match artifact-set completeness'
      )
    )
);

export const TrackingPhysicalDeviceEvidenceReviewProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-physical-device-evidence-review-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingPhysicalDeviceEvidenceReviewRowSchema),
    summary: Schema.Struct({
      rowCount: Schema.Number,
      artifactMissingRows: Schema.Number,
      contentReviewRequiredRows: Schema.Number,
      contentAcceptedRows: Schema.Number,
      physicalDeviceBehaviorClaimedRows: Schema.Number,
      productReadyRows: Schema.Number,
      physicalDeviceStatusObservedRows: Schema.Number,
      supportingStatusArtifactCount: Schema.Number,
      acceptanceCriteriaCount: Schema.Number,
      manualValidationCommandCount: Schema.Number,
      artifactAcceptanceNoteCount: Schema.Number,
    }),
    productClaims: Schema.Struct({
      physicalDeviceBehaviorClaimed: Schema.Literal(false),
      authorityEnrollmentClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(Schema.filter((proof) => proof.rows.length === 2 || 'Physical evidence review must cover Android and iOS'))
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.rowCount === proof.rows.length &&
          proof.summary.contentAcceptedRows === 0 &&
          proof.summary.physicalDeviceBehaviorClaimedRows === 0 &&
          proof.summary.productReadyRows === 0 &&
          Object.values(proof.productClaims).every((claim) => claim === false)
      )
    )
);

export type TrackingPhysicalDeviceEvidenceReviewProof = Infer<typeof TrackingPhysicalDeviceEvidenceReviewProofSchema>;
export type TrackingPhysicalDeviceEvidenceReviewRow = Infer<typeof TrackingPhysicalDeviceEvidenceReviewRowSchema>;

export function buildTrackingPhysicalDeviceEvidenceReviewProof(
  generatedAt: string,
  artifactGateProof: TrackingPhysicalDeviceArtifactGateProof
): TrackingPhysicalDeviceEvidenceReviewProof {
  const parsedGateProof = TrackingPhysicalDeviceArtifactGateProofSchema.parse(artifactGateProof);
  const rows = parsedGateProof.rows.map((row) => evidenceReviewRow(generatedAt, row));

  return TrackingPhysicalDeviceEvidenceReviewProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-physical-device-evidence-review-proof',
    generatedAt,
    rows,
    summary: summaryFor(rows),
    productClaims: {
      physicalDeviceBehaviorClaimed: false,
      authorityEnrollmentClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function evidenceReviewRow(
  generatedAt: string,
  gateRow: TrackingPhysicalDeviceArtifactGateRow
): TrackingPhysicalDeviceEvidenceReviewRow {
  const artifactSetComplete = gateRow.physicalArtifactSetComplete;

  return TrackingPhysicalDeviceEvidenceReviewRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-physical-device-evidence-review-${gateRow.platform}`,
    generatedAt,
    platform: gateRow.platform,
    sourceArtifactGateRowId: gateRow.rowId,
    proofRoot: gateRow.proofRoot,
    status: artifactSetComplete ? 'content-review-required' : 'artifact-missing',
    requiredProofTier: 'P4_PHYSICAL_DEVICE_CONTENT_REVIEW',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    requiredArtifacts: [...gateRow.requiredArtifacts],
    presentArtifacts: [...gateRow.presentArtifacts],
    missingArtifacts: [...gateRow.missingArtifacts],
    supportingStatusProofRef: gateRow.supportingStatusProofRef,
    supportingStatusArtifacts: [...gateRow.supportingStatusArtifacts],
    acceptanceCriteria: [...gateRow.acceptanceCriteria],
    manualValidationCommands: [...gateRow.manualValidationCommands],
    artifactAcceptanceNotes: [...gateRow.artifactAcceptanceNotes],
    artifactSetComplete,
    physicalDeviceStatusObserved: gateRow.physicalDeviceStatusObserved,
    reviewerRequired: true,
    contentAccepted: false,
    physicalDeviceBehaviorClaimed: false,
    authorityEnrollmentClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function summaryFor(rows: readonly TrackingPhysicalDeviceEvidenceReviewRow[]) {
  return {
    rowCount: rows.length,
    artifactMissingRows: rows.filter((row) => row.status === 'artifact-missing').length,
    contentReviewRequiredRows: rows.filter((row) => row.status === 'content-review-required').length,
    contentAcceptedRows: rows.filter((row) => row.contentAccepted).length,
    physicalDeviceBehaviorClaimedRows: rows.filter((row) => row.physicalDeviceBehaviorClaimed).length,
    productReadyRows: rows.filter((row) => row.productClaimReady).length,
    physicalDeviceStatusObservedRows: rows.filter((row) => row.physicalDeviceStatusObserved).length,
    supportingStatusArtifactCount: rows.reduce((total, row) => total + row.supportingStatusArtifacts.length, 0),
    acceptanceCriteriaCount: rows.reduce((total, row) => total + row.acceptanceCriteria.length, 0),
    manualValidationCommandCount: rows.reduce((total, row) => total + row.manualValidationCommands.length, 0),
    artifactAcceptanceNoteCount: rows.reduce((total, row) => total + row.artifactAcceptanceNotes.length, 0),
  };
}
