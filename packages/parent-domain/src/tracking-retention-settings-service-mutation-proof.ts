import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  type TrackingRetentionSettingsWriterBoundaryRow,
  TrackingRetentionSettingsWriteActionSchema,
  buildTrackingRetentionSettingsWriterBoundaryProof,
} from './tracking-retention-settings-writer-boundary-proof';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';

const TrackingRetentionSettingsServiceMutationTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRetentionSettingsServiceMutationIdSchema = TrackingRetentionSettingsServiceMutationTextSchema.pipe(
  Schema.brand('TrackingRetentionSettingsServiceMutationId')
);

export const TrackingRetentionSettingsServiceMutationRequestIdSchema =
  TrackingRetentionSettingsServiceMutationTextSchema.pipe(
    Schema.brand('TrackingRetentionSettingsServiceMutationRequestId')
  );

export const TrackingRetentionSettingsServiceMutationStateSchema = withParser(Schema.Literal('accepted'));

export const TrackingRetentionSettingsServiceMutationRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    requestId: TrackingRetentionSettingsServiceMutationRequestIdSchema,
    mutationId: TrackingRetentionSettingsServiceMutationIdSchema,
    intentId: TrackingRetentionSettingsServiceMutationTextSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    writeAction: TrackingRetentionSettingsWriteActionSchema,
    requestedValue: TrackingRetentionSettingsServiceMutationTextSchema,
    mutationState: TrackingRetentionSettingsServiceMutationStateSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    writerBoundaryProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    serviceCommandRegisteredClaimed: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(true),
    durablePersistenceClaimed: Schema.Literal(false),
    portalUiClaimed: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.sourceReadModelProofRefs.length > 0 ||
          'Tracking retention settings service mutation rows need source read-model proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.writerBoundaryProofRefs.length > 0 ||
          'Tracking retention settings service mutation rows need writer-boundary proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.evidenceReferences.length > 0 || 'Tracking retention settings service mutation rows need evidence refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.auditRefs.length > 0 || 'Tracking retention settings service mutation rows need audit refs'
      )
    )
);

export const TrackingRetentionSettingsServiceMutationProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-settings-service-mutation-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionSettingsServiceMutationRowSchema),
    proofClaims: Schema.Struct({
      serviceCommandRegisteredClaimed: Schema.Literal(true),
      serviceMutationExecuted: Schema.Literal(true),
      writerBoundaryProofConsumed: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      durablePersistenceClaimed: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) => proof.rows.length >= 5 || 'Tracking retention service mutation proof needs all settings rows'
    )
  )
);

export type TrackingRetentionSettingsServiceMutationRow = Infer<
  typeof TrackingRetentionSettingsServiceMutationRowSchema
>;
export type TrackingRetentionSettingsServiceMutationProof = Infer<
  typeof TrackingRetentionSettingsServiceMutationProofSchema
>;

export function buildTrackingRetentionSettingsServiceMutationProof(
  generatedAt: string
): TrackingRetentionSettingsServiceMutationProof {
  const writerProof = buildTrackingRetentionSettingsWriterBoundaryProof(generatedAt);
  return TrackingRetentionSettingsServiceMutationProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-settings-service-mutation-proof',
    generatedAt,
    rows: writerProof.rows.map((row) => serviceMutationRow(row, generatedAt)),
    proofClaims: {
      serviceCommandRegisteredClaimed: true,
      serviceMutationExecuted: true,
      writerBoundaryProofConsumed: true,
    },
    productClaims: {
      productClaimReady: false,
      durablePersistenceClaimed: false,
      portalUiClaimed: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
    },
  });
}

function serviceMutationRow(
  writerRow: TrackingRetentionSettingsWriterBoundaryRow,
  generatedAt: string
): TrackingRetentionSettingsServiceMutationRow {
  const requestId = requestIdFor(writerRow);
  return TrackingRetentionSettingsServiceMutationRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    requestId,
    mutationId: `${requestId}-service-result`,
    intentId: writerRow.intentId,
    settingsKind: writerRow.settingsKind,
    writeAction: writerRow.writeAction,
    requestedValue: requestedValueFor(writerRow),
    mutationState: 'accepted',
    generatedAt,
    sourceReadModelProofRefs: writerRow.sourceReadModelProofRefs,
    writerBoundaryProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/19-retention-settings-writer-boundary-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/25-retention-settings-writer-boundary-proof.json',
    ],
    evidenceReferences: writerRow.evidenceReferences,
    auditRefs: writerRow.auditRefs,
    serviceCommandRegisteredClaimed: true,
    serviceMutationExecuted: true,
    durablePersistenceClaimed: false,
    portalUiClaimed: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}

function requestIdFor(writerRow: TrackingRetentionSettingsWriterBoundaryRow): string {
  return `${writerRow.intentId}-request`;
}

function requestedValueFor(writerRow: TrackingRetentionSettingsWriterBoundaryRow): string {
  switch (writerRow.writeAction) {
    case 'set-retention-window':
      return String(writerRow.requestedRetentionWindowHours);
    case 'enable-delete-after-alert':
      return String(writerRow.requestedDeleteAfterAlertResolved);
    case 'prepare-parent-export':
      return String(writerRow.requestedParentExport);
    case 'keep-remote-sync-disabled':
      return String(writerRow.requestedRemoteSyncEnabled);
    case 'keep-remote-ai-disabled':
      return String(writerRow.requestedRemoteAiEnabled);
  }
}
