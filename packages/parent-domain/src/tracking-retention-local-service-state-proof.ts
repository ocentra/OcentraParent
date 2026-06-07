import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';

const TrackingRetentionLocalStateTextSchema = Schema.String.pipe(Schema.minLength(1));
const AgentProtocolSchemaVersion = 1;

export const TrackingRetentionLocalServiceStateProofIdSchema = TrackingRetentionLocalStateTextSchema.pipe(
  Schema.brand('TrackingRetentionLocalServiceStateProofId')
);

export const TrackingRetentionLocalServiceStateSnapshotRefSchema = TrackingRetentionLocalStateTextSchema.pipe(
  Schema.brand('TrackingRetentionLocalServiceStateSnapshotRef')
);

export const TrackingRetentionLocalServiceStateWriteResultSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    commandId: TrackingRetentionLocalStateTextSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    writeState: Schema.Literal('service-write-command-accepted'),
    sourceWriterIntentRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceMutationProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    localServiceStateSnapshotRef: TrackingRetentionLocalServiceStateSnapshotRefSchema,
    durableSettingsPersisted: Schema.Literal(false),
    commandTransportClaimed: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(true),
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
        (result) => result.sourceReadModelProofRefs.length > 0 || 'Local state proof needs source read-model refs'
      )
    )
    .pipe(
      Schema.filter(
        (result) => result.sourceMutationProofRefs.length > 0 || 'Local state proof needs source mutation refs'
      )
    )
);

export const TrackingRetentionLocalServiceStateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    stateProofId: TrackingRetentionLocalServiceStateProofIdSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    generatedAt: ParentTimestampSchema,
    sourceWriteCommandProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceWriterIntentRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceMutationProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    localServiceStateSnapshotRef: TrackingRetentionLocalServiceStateSnapshotRefSchema,
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    writeCommandAccepted: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(true),
    localServiceStateReadbackClaimed: Schema.Literal(true),
    durableSettingsPersisted: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Local state rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'retention-window-setting' ||
          row.appliedRetentionWindowHours !== null ||
          'Retention-window local state readback must include the applied window'
      )
    )
);

export const TrackingRetentionLocalServiceStateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-local-service-state-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionLocalServiceStateRowSchema),
    proofClaims: Schema.Struct({
      writeCommandAccepted: Schema.Literal(true),
      serviceMutationExecuted: Schema.Literal(true),
      localServiceStateRevisionRecorded: Schema.Literal(true),
      localServiceStateReadbackClaimed: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      durableSettingsPersisted: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length > 0 || 'Local service state proof needs at least one row'))
);

export type TrackingRetentionLocalServiceStateWriteResult = Infer<
  typeof TrackingRetentionLocalServiceStateWriteResultSchema
>;
export type TrackingRetentionLocalServiceStateRow = Infer<typeof TrackingRetentionLocalServiceStateRowSchema>;
export type TrackingRetentionLocalServiceStateProof = Infer<typeof TrackingRetentionLocalServiceStateProofSchema>;

export function buildTrackingRetentionLocalServiceStateProof(
  generatedAt: string,
  sourceWriteCommandProofRef: string,
  writeResult: unknown
): TrackingRetentionLocalServiceStateProof {
  const parsedWriteResult = TrackingRetentionLocalServiceStateWriteResultSchema.parse(writeResult);
  return TrackingRetentionLocalServiceStateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-local-service-state-proof',
    generatedAt,
    rows: [localStateRow(generatedAt, sourceWriteCommandProofRef, parsedWriteResult)],
    proofClaims: {
      writeCommandAccepted: true,
      serviceMutationExecuted: true,
      localServiceStateRevisionRecorded: true,
      localServiceStateReadbackClaimed: true,
    },
    productClaims: {
      durableSettingsPersisted: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
      productClaimReady: false,
    },
  });
}

function localStateRow(
  generatedAt: string,
  sourceWriteCommandProofRef: string,
  writeResult: TrackingRetentionLocalServiceStateWriteResult
): TrackingRetentionLocalServiceStateRow {
  return TrackingRetentionLocalServiceStateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    stateProofId: `${writeResult.commandId}-local-state-readback`,
    settingsKind: writeResult.settingsKind,
    generatedAt,
    sourceWriteCommandProofRef,
    sourceWriterIntentRefs: writeResult.sourceWriterIntentRefs,
    sourceReadModelProofRefs: writeResult.sourceReadModelProofRefs,
    sourceMutationProofRefs: writeResult.sourceMutationProofRefs,
    auditRefs: [`${writeResult.commandId}-local-state-readback-audit`],
    localServiceStateRevision: writeResult.localServiceStateRevision,
    localServiceStateSnapshotRef: writeResult.localServiceStateSnapshotRef,
    appliedRetentionWindowHours: writeResult.appliedRetentionWindowHours,
    appliedDeleteAfterAlertResolved: writeResult.appliedDeleteAfterAlertResolved,
    parentExportPrepared: writeResult.parentExportPrepared,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    writeCommandAccepted: true,
    serviceMutationExecuted: true,
    localServiceStateReadbackClaimed: true,
    durableSettingsPersisted: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}
