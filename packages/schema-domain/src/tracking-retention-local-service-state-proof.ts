import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import {
  AgentTrackingDeleteAfterAlertResolutionState,
  AgentTrackingDurableSettingsPersistenceState,
  AgentTrackingExecutionClaimState,
  AgentTrackingParentExportState,
  AgentTrackingRetentionSettingsWriteDefaults,
  AgentTrackingRetentionSettingsWriteResultSchema,
} from './agent-tracking-retention-settings-write-command';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';

export const TrackingRetentionLocalServiceStateProofIdSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionLocalServiceStateProofId'
);

export const TrackingRetentionLocalServiceStateSnapshotRefSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionLocalServiceStateSnapshotRef'
);

export const TrackingRetentionLocalServiceStateWriteResultSchema = withParser(
  AgentTrackingRetentionSettingsWriteResultSchema.pipe(
    Schema.filter((result) => result.commandId.length > 0 || 'Local state proof needs a command id'),
    Schema.filter(
      (result) => result.sourceReadModelProofRefs.length > 0 || 'Local state proof needs source read-model refs'
    ),
    Schema.filter(
      (result) => result.sourceMutationProofRefs.length > 0 || 'Local state proof needs source mutation refs'
    ),
    Schema.filter(
      (result) =>
        result.durableSettingsPersistenceState === AgentTrackingDurableSettingsPersistenceState.Persisted ||
        'Local state proof needs durable settings persistence'
    ),
    Schema.filter(
      (result) =>
        result.commandTransportClaimState === AgentTrackingExecutionClaimState.Claimed ||
        'Local state proof needs command transport'
    ),
    Schema.filter(
      (result) =>
        result.serviceMutationExecutionState === AgentTrackingExecutionClaimState.Claimed ||
        'Local state proof needs service mutation execution'
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
    durableSettingsStoreRef: TrackingRetentionLocalServiceStateSnapshotRefSchema,
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    writeCommandAccepted: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(true),
    localServiceStateReadbackClaimed: Schema.Literal(true),
    durableSettingsPersisted: Schema.Boolean,
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
          row.settingsKind !== AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow ||
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
      durableSettingsPersisted: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      durableSettingsPersisted: Schema.Literal(true),
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
      durableSettingsPersisted: true,
    },
    productClaims: {
      durableSettingsPersisted: true,
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
    durableSettingsStoreRef: writeResult.durableSettingsStoreRef,
    appliedRetentionWindowHours: writeResult.appliedRetentionWindowHours,
    appliedDeleteAfterAlertResolved:
      writeResult.appliedDeleteAfterAlertResolutionState ===
      AgentTrackingDeleteAfterAlertResolutionState.DeleteAfterAlertResolved,
    parentExportPrepared: writeResult.parentExportState === AgentTrackingParentExportState.Prepared,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    writeCommandAccepted: true,
    serviceMutationExecuted: true,
    localServiceStateReadbackClaimed: true,
    durableSettingsPersisted:
      writeResult.durableSettingsPersistenceState === AgentTrackingDurableSettingsPersistenceState.Persisted,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}
