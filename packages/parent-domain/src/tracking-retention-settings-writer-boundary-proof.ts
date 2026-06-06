import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import {
  type TrackingRetentionSettingsKind,
  TrackingRetentionSettingsKindSchema,
  TrackingRetentionSettingsProofRefSchema,
} from './tracking-retention-settings-read-model-proof';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingRetentionSettingsWriterTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRetentionSettingsWriteIntentIdSchema = TrackingRetentionSettingsWriterTextSchema.pipe(
  Schema.brand('TrackingRetentionSettingsWriteIntentId')
);

export const TrackingRetentionSettingsWriteActionSchema = withParser(
  Schema.Literal(
    'set-retention-window',
    'enable-delete-after-alert',
    'prepare-parent-export',
    'keep-remote-sync-disabled',
    'keep-remote-ai-disabled'
  )
);

export const TrackingRetentionSettingsWriterStateSchema = withParser(
  Schema.Literal('writer-preflight-ready', 'manual-required')
);

export const TrackingRetentionSettingsWriterBoundaryRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    intentId: TrackingRetentionSettingsWriteIntentIdSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    writeAction: TrackingRetentionSettingsWriteActionSchema,
    writerState: TrackingRetentionSettingsWriterStateSchema,
    generatedAt: ParentTimestampSchema,
    sourceReadModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    retentionProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    readModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    requestedRetentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    requestedDeleteAfterAlertResolved: Schema.Boolean,
    requestedParentExport: Schema.Boolean,
    requestedRemoteSyncEnabled: Schema.Literal(false),
    requestedRemoteAiEnabled: Schema.Literal(false),
    parentIntentAuthorized: Schema.Literal(true),
    localValidationClaimed: Schema.Literal(true),
    writerBoundaryClaimed: Schema.Literal(true),
    serviceMutationPreflightClaimed: Schema.Literal(true),
    serviceMutationExecuted: Schema.Literal(false),
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
          'Tracking retention settings writer rows need source read-model proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.retentionProofRefs.length > 0 || 'Tracking retention settings writer rows need retention proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.readModelProofRefs.length > 0 || 'Tracking retention settings writer rows need read-model proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.evidenceReferences.length > 0 || 'Tracking retention settings writer rows need evidence refs'
      )
    )
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Tracking retention settings writer rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'retention-window-setting' ||
          (row.writeAction === 'set-retention-window' && row.requestedRetentionWindowHours !== null) ||
          'Tracking retention writer window rows must set a retention window'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'delete-after-alert-setting' ||
          (row.writeAction === 'enable-delete-after-alert' && row.requestedDeleteAfterAlertResolved) ||
          'Tracking retention writer delete rows must request delete-after-alert'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'parent-export-setting' ||
          (row.writeAction === 'prepare-parent-export' && row.requestedParentExport) ||
          'Tracking retention writer export rows must request parent export'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'remote-sync-disabled-setting' ||
          row.writeAction === 'keep-remote-sync-disabled' ||
          'Tracking retention writer remote-sync rows must keep remote sync disabled'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'remote-ai-disabled-setting' ||
          row.writeAction === 'keep-remote-ai-disabled' ||
          'Tracking retention writer remote-AI rows must keep remote AI disabled'
      )
    )
);

export const TrackingRetentionSettingsWriterBoundaryProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-settings-writer-boundary-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionSettingsWriterBoundaryRowSchema),
    proofClaims: Schema.Struct({
      localValidationClaimed: Schema.Literal(true),
      writerBoundaryClaimed: Schema.Literal(true),
      serviceMutationPreflightClaimed: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      serviceMutationExecuted: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length >= 5 || 'Tracking retention writer proof needs all settings rows'))
);

export type TrackingRetentionSettingsWriteAction = Infer<typeof TrackingRetentionSettingsWriteActionSchema>;
export type TrackingRetentionSettingsWriterBoundaryRow = Infer<typeof TrackingRetentionSettingsWriterBoundaryRowSchema>;
export type TrackingRetentionSettingsWriterBoundaryProof = Infer<
  typeof TrackingRetentionSettingsWriterBoundaryProofSchema
>;
type TrackingRetentionSettingsWriterEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingRetentionSettingsWriterBoundaryProof(
  generatedAt: string
): TrackingRetentionSettingsWriterBoundaryProof {
  return TrackingRetentionSettingsWriterBoundaryProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-settings-writer-boundary-proof',
    generatedAt,
    rows: writerRows(generatedAt),
    proofClaims: {
      localValidationClaimed: true,
      writerBoundaryClaimed: true,
      serviceMutationPreflightClaimed: true,
    },
    productClaims: {
      productClaimReady: false,
      serviceMutationExecuted: false,
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

function writerRows(timestamp: string): readonly TrackingRetentionSettingsWriterBoundaryRow[] {
  return [
    writerRow({
      intentId: 'tracking-retention-settings-write-retention-window',
      settingsKind: 'retention-window-setting',
      writeAction: 'set-retention-window',
      generatedAt: timestamp,
      evidenceReferences: [evidence('tracking-retention-writer-evidence-window', 'query-store-summary', timestamp)],
      requestedRetentionWindowHours: 168,
      requestedDeleteAfterAlertResolved: false,
      requestedParentExport: false,
      reasonCodes: ['tracking-retention-window-writer-preflight-ready'],
      auditRefs: ['tracking-retention-writer-audit-window'],
    }),
    writerRow({
      intentId: 'tracking-retention-settings-write-delete-after-alert',
      settingsKind: 'delete-after-alert-setting',
      writeAction: 'enable-delete-after-alert',
      generatedAt: timestamp,
      evidenceReferences: [
        evidence('tracking-retention-writer-evidence-delete-after-alert', 'query-store-summary', timestamp),
      ],
      requestedRetentionWindowHours: null,
      requestedDeleteAfterAlertResolved: true,
      requestedParentExport: false,
      reasonCodes: ['tracking-delete-after-alert-writer-preflight-ready'],
      auditRefs: ['tracking-retention-writer-audit-delete-after-alert'],
    }),
    writerRow({
      intentId: 'tracking-retention-settings-write-parent-export',
      settingsKind: 'parent-export-setting',
      writeAction: 'prepare-parent-export',
      generatedAt: timestamp,
      evidenceReferences: [evidence('tracking-retention-writer-evidence-export', 'query-store-summary', timestamp)],
      requestedRetentionWindowHours: null,
      requestedDeleteAfterAlertResolved: false,
      requestedParentExport: true,
      reasonCodes: ['tracking-parent-export-writer-preflight-ready'],
      auditRefs: ['tracking-retention-writer-audit-export'],
    }),
    remoteDisabledWriterRow('remote-sync-disabled-setting', timestamp),
    remoteDisabledWriterRow('remote-ai-disabled-setting', timestamp),
  ];
}

function remoteDisabledWriterRow(
  settingsKind: Extract<TrackingRetentionSettingsKind, 'remote-sync-disabled-setting' | 'remote-ai-disabled-setting'>,
  timestamp: string
): TrackingRetentionSettingsWriterBoundaryRow {
  const syncRow = settingsKind === 'remote-sync-disabled-setting';
  return writerRow({
    intentId: syncRow
      ? 'tracking-retention-settings-write-remote-sync-disabled'
      : 'tracking-retention-settings-write-remote-ai-disabled',
    settingsKind,
    writeAction: syncRow ? 'keep-remote-sync-disabled' : 'keep-remote-ai-disabled',
    generatedAt: timestamp,
    evidenceReferences: [
      evidence(
        syncRow
          ? 'tracking-retention-writer-evidence-remote-sync-disabled'
          : 'tracking-retention-writer-evidence-remote-ai-disabled',
        'query-store-summary',
        timestamp
      ),
    ],
    requestedRetentionWindowHours: null,
    requestedDeleteAfterAlertResolved: false,
    requestedParentExport: false,
    reasonCodes: [syncRow ? 'tracking-remote-sync-writer-kept-disabled' : 'tracking-remote-ai-writer-kept-disabled'],
    auditRefs: [syncRow ? 'tracking-retention-writer-audit-remote-sync' : 'tracking-retention-writer-audit-remote-ai'],
  });
}

function writerRow(input: {
  readonly intentId: string;
  readonly settingsKind: TrackingRetentionSettingsKind;
  readonly writeAction: TrackingRetentionSettingsWriteAction;
  readonly generatedAt: string;
  readonly evidenceReferences: readonly TrackingRetentionSettingsWriterEvidence[];
  readonly requestedRetentionWindowHours: number | null;
  readonly requestedDeleteAfterAlertResolved: boolean;
  readonly requestedParentExport: boolean;
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
}): TrackingRetentionSettingsWriterBoundaryRow {
  return TrackingRetentionSettingsWriterBoundaryRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    intentId: input.intentId,
    settingsKind: input.settingsKind,
    writeAction: input.writeAction,
    writerState: 'writer-preflight-ready',
    generatedAt: input.generatedAt,
    sourceReadModelProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/18-retention-settings-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
    retentionProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json',
    ],
    readModelProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
    evidenceReferences: input.evidenceReferences,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    requestedRetentionWindowHours: input.requestedRetentionWindowHours,
    requestedDeleteAfterAlertResolved: input.requestedDeleteAfterAlertResolved,
    requestedParentExport: input.requestedParentExport,
    requestedRemoteSyncEnabled: false,
    requestedRemoteAiEnabled: false,
    parentIntentAuthorized: true,
    localValidationClaimed: true,
    writerBoundaryClaimed: true,
    serviceMutationPreflightClaimed: true,
    serviceMutationExecuted: false,
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

function evidence(
  evidenceReferenceId: string,
  kind: TrackingRetentionSettingsWriterEvidence['kind'],
  observedAt: string
): TrackingRetentionSettingsWriterEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}
