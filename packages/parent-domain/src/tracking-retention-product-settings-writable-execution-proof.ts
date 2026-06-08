import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  TrackingRetentionLocalServiceStateProofSchema,
  type TrackingRetentionLocalServiceStateRow,
} from './tracking-retention-local-service-state-proof';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';

const TrackingRetentionWritableExecutionTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRetentionProductSettingsWritableExecutionArtifactRefSchema =
  TrackingRetentionWritableExecutionTextSchema.pipe(
    Schema.brand('TrackingRetentionProductSettingsWritableExecutionArtifactRef')
  );

export const TrackingRetentionProductSettingsWritableExecutionRowIdSchema =
  TrackingRetentionWritableExecutionTextSchema.pipe(
    Schema.brand('TrackingRetentionProductSettingsWritableExecutionRowId')
  );

export const TrackingRetentionProductSettingsWritableExecutionRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRetentionProductSettingsWritableExecutionRowIdSchema,
    generatedAt: ParentTimestampSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    sourceLocalServiceStateProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceWriteCommandProofRef: TrackingRetentionSettingsProofRefSchema,
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceMutationProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    outputArtifactRef: TrackingRetentionProductSettingsWritableExecutionArtifactRefSchema,
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    localServiceStateRevision: Schema.Number.pipe(Schema.int(), Schema.positive()),
    durableSettingsStoreRef: TrackingRetentionWritableExecutionTextSchema,
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    writeCommandAccepted: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(true),
    localServiceStateReadbackClaimed: Schema.Literal(true),
    durableSettingsPersisted: Schema.Literal(true),
    localProductSettingsWritableExecutionObserved: Schema.Literal(true),
    portalWritableUiClaimed: Schema.Literal(false),
    platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Writable execution rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) => row.sourceReadModelProofRefs.length > 0 || 'Writable execution rows need read-model proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.sourceMutationProofRefs.length > 0 || 'Writable execution rows need mutation proof refs'
      )
    )
);

export const TrackingRetentionProductSettingsWritableExecutionProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-product-settings-writable-execution-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionProductSettingsWritableExecutionRowSchema),
    proofClaims: Schema.Struct({
      writeCommandAccepted: Schema.Literal(true),
      serviceMutationExecuted: Schema.Literal(true),
      localServiceStateReadbackClaimed: Schema.Literal(true),
      durableSettingsPersisted: Schema.Literal(true),
      localProductSettingsWritableExecutionObserved: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      portalWritableUiClaimed: Schema.Literal(false),
      platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length > 0 || 'Writable execution proof needs at least one source row'))
);

export type TrackingRetentionProductSettingsWritableExecutionProof = Infer<
  typeof TrackingRetentionProductSettingsWritableExecutionProofSchema
>;
export type TrackingRetentionProductSettingsWritableExecutionRow = Infer<
  typeof TrackingRetentionProductSettingsWritableExecutionRowSchema
>;

export const TrackingRetentionProductSettingsWritableExecutionArtifactRef =
  'tracking-retention/product-settings-writable-execution.json' as const;

export function buildTrackingRetentionProductSettingsWritableExecutionProof(
  generatedAt: string,
  sourceLocalServiceStateProofRef: string,
  localServiceStateProof: unknown
): TrackingRetentionProductSettingsWritableExecutionProof {
  const parsedLocalServiceStateProof = TrackingRetentionLocalServiceStateProofSchema.parse(localServiceStateProof);
  return TrackingRetentionProductSettingsWritableExecutionProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-product-settings-writable-execution-proof',
    generatedAt,
    rows: parsedLocalServiceStateProof.rows.map((row) =>
      writableExecutionRow(generatedAt, sourceLocalServiceStateProofRef, row)
    ),
    proofClaims: {
      writeCommandAccepted: true,
      serviceMutationExecuted: true,
      localServiceStateReadbackClaimed: true,
      durableSettingsPersisted: true,
      localProductSettingsWritableExecutionObserved: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      portalWritableUiClaimed: false,
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

function writableExecutionRow(
  generatedAt: string,
  sourceLocalServiceStateProofRef: string,
  row: TrackingRetentionLocalServiceStateRow
): TrackingRetentionProductSettingsWritableExecutionRow {
  return TrackingRetentionProductSettingsWritableExecutionRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `${String(row.stateProofId)}-product-settings-writable-execution`,
    generatedAt,
    settingsKind: row.settingsKind,
    sourceLocalServiceStateProofRef,
    sourceWriteCommandProofRef: row.sourceWriteCommandProofRef,
    sourceReadModelProofRefs: row.sourceReadModelProofRefs,
    sourceMutationProofRefs: row.sourceMutationProofRefs,
    outputArtifactRef: TrackingRetentionProductSettingsWritableExecutionArtifactRef,
    auditRefs: [`${String(row.stateProofId)}-product-settings-writable-execution-audit`],
    localServiceStateRevision: row.localServiceStateRevision,
    durableSettingsStoreRef: String(row.durableSettingsStoreRef),
    appliedRetentionWindowHours: row.appliedRetentionWindowHours,
    appliedDeleteAfterAlertResolved: row.appliedDeleteAfterAlertResolved,
    parentExportPrepared: row.parentExportPrepared,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    writeCommandAccepted: true,
    serviceMutationExecuted: true,
    localServiceStateReadbackClaimed: true,
    durableSettingsPersisted: true,
    localProductSettingsWritableExecutionObserved: true,
    portalWritableUiClaimed: false,
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
