import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { AgentTrackingRetentionSettingsWriteDefaults } from '@ocentra-parent/agent-protocol-domain/tracking-retention-settings-write-command';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';
import {
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';
import {
  type TrackingRetentionSettingsWriterBoundaryRow,
  buildTrackingRetentionSettingsWriterBoundaryProof,
} from './tracking-retention-settings-writer-boundary-proof';

export const TrackingRetentionSettingsMutationIdSchema = brandedNonEmptyStringSchema('TrackingRetentionSettingsMutationId');

export const TrackingRetentionSettingsMutationStateSchema = withParser(
  Schema.Literal('service-mutation-executed', 'remote-disabled-preserved')
);

export const TrackingRetentionSettingsMutationRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    mutationId: TrackingRetentionSettingsMutationIdSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    mutationState: TrackingRetentionSettingsMutationStateSchema,
    generatedAt: ParentTimestampSchema,
    sourceWriterIntentRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    retentionProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    readModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    appliedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    appliedDeleteAfterAlertResolved: Schema.Boolean,
    parentExportPrepared: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    parentIntentAuthorized: Schema.Literal(true),
    localValidationClaimed: Schema.Literal(true),
    writerBoundaryClaimed: Schema.Literal(true),
    serviceMutationPreflightClaimed: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(true),
    portalWritableUiClaimed: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter((row) => row.sourceWriterIntentRefs.length > 0 || 'Mutation rows need source writer intent refs')
    )
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Mutation rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== AgentTrackingRetentionSettingsWriteDefaults.SettingsKindRetentionWindow ||
          row.appliedRetentionWindowHours !== null ||
          'Retention window mutations must apply a retention window'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'delete-after-alert-setting' ||
          row.appliedDeleteAfterAlertResolved ||
          'Delete-after-alert mutations must apply delete-after-alert'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'parent-export-setting' ||
          row.parentExportPrepared ||
          'Parent export mutations must prepare parent export'
      )
    )
);

export const TrackingRetentionSettingsMutationProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-settings-mutation-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionSettingsMutationRowSchema),
    proofClaims: Schema.Struct({
      localValidationClaimed: Schema.Literal(true),
      writerBoundaryClaimed: Schema.Literal(true),
      serviceMutationPreflightClaimed: Schema.Literal(true),
      serviceMutationExecuted: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      portalWritableUiClaimed: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter((proof) => proof.rows.length >= 5 || 'Tracking retention mutation proof needs all settings rows')
  )
);

export type TrackingRetentionSettingsMutationRow = Infer<typeof TrackingRetentionSettingsMutationRowSchema>;
export type TrackingRetentionSettingsMutationProof = Infer<typeof TrackingRetentionSettingsMutationProofSchema>;

export function buildTrackingRetentionSettingsMutationProof(
  generatedAt: string
): TrackingRetentionSettingsMutationProof {
  const writerProof = buildTrackingRetentionSettingsWriterBoundaryProof(generatedAt);
  return TrackingRetentionSettingsMutationProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-settings-mutation-proof',
    generatedAt,
    rows: writerProof.rows.map((row) => mutationRow(row, generatedAt)),
    proofClaims: {
      localValidationClaimed: true,
      writerBoundaryClaimed: true,
      serviceMutationPreflightClaimed: true,
      serviceMutationExecuted: true,
    },
    productClaims: {
      productClaimReady: false,
      portalWritableUiClaimed: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
    },
  });
}

function mutationRow(
  writerRow: TrackingRetentionSettingsWriterBoundaryRow,
  generatedAt: string
): TrackingRetentionSettingsMutationRow {
  const remoteDisabled =
    writerRow.settingsKind === 'remote-sync-disabled-setting' ||
    writerRow.settingsKind === 'remote-ai-disabled-setting';
  return TrackingRetentionSettingsMutationRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    mutationId: writerRow.intentId.replace('write', 'mutation'),
    settingsKind: writerRow.settingsKind,
    mutationState: remoteDisabled ? 'remote-disabled-preserved' : 'service-mutation-executed',
    generatedAt,
    sourceWriterIntentRefs: [writerRow.intentId],
    sourceReadModelProofRefs: writerRow.sourceReadModelProofRefs,
    retentionProofRefs: writerRow.retentionProofRefs,
    readModelProofRefs: writerRow.readModelProofRefs,
    evidenceReferences: writerRow.evidenceReferences,
    reasonCodes: writerRow.reasonCodes,
    auditRefs: [...writerRow.auditRefs, `${writerRow.intentId}-executed-audit`],
    appliedRetentionWindowHours: writerRow.requestedRetentionWindowHours,
    appliedDeleteAfterAlertResolved: writerRow.requestedDeleteAfterAlertResolved,
    parentExportPrepared: writerRow.requestedParentExport,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    parentIntentAuthorized: true,
    localValidationClaimed: true,
    writerBoundaryClaimed: true,
    serviceMutationPreflightClaimed: true,
    serviceMutationExecuted: true,
    portalWritableUiClaimed: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}

