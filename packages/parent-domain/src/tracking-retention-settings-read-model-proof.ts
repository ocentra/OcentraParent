import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingRetentionSettingsTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRetentionSettingsRowIdSchema = TrackingRetentionSettingsTextSchema.pipe(
  Schema.brand('TrackingRetentionSettingsRowId')
);

export const TrackingRetentionSettingsProofRefSchema = TrackingRetentionSettingsTextSchema.pipe(
  Schema.brand('TrackingRetentionSettingsProofRef')
);

export const TrackingRetentionSettingsKindSchema = withParser(
  Schema.Literal(
    'retention-window-setting',
    'delete-after-alert-setting',
    'parent-export-setting',
    'remote-sync-disabled-setting',
    'remote-ai-disabled-setting'
  )
);

export const TrackingRetentionSettingsStateSchema = withParser(
  Schema.Literal('settings-read-model-ready', 'manual-required')
);

export const TrackingRetentionSettingsProofTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingRetentionSettingsCustodySchema = withParser(
  Schema.Literal('child-device-local', 'parent-device-local', 'parent-owned-export', 'remote-disabled')
);

export const TrackingRetentionSettingsRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRetentionSettingsRowIdSchema,
    settingsKind: TrackingRetentionSettingsKindSchema,
    settingsState: TrackingRetentionSettingsStateSchema,
    requiredProofTier: TrackingRetentionSettingsProofTierSchema,
    currentProofTier: TrackingRetentionSettingsProofTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    retentionProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    readModelProofRefs: Schema.Array(TrackingRetentionSettingsProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    custodyScope: TrackingRetentionSettingsCustodySchema,
    retentionWindowHours: Schema.Union(Schema.Number.pipe(Schema.int(), Schema.positive()), Schema.Null),
    deleteAfterAlertResolved: Schema.Boolean,
    parentExportReady: Schema.Boolean,
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    settingsReadModelClaimed: Schema.Literal(true),
    portalUiClaimed: Schema.Literal(false),
    serviceMutationClaimed: Schema.Literal(false),
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
        (row) => row.sourceProofRefs.length > 0 || 'Tracking retention settings rows need source proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.retentionProofRefs.length > 0 || 'Tracking retention settings rows need retention proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.readModelProofRefs.length > 0 || 'Tracking retention settings rows need read-model proof refs'
      )
    )
    .pipe(
      Schema.filter((row) => row.evidenceReferences.length > 0 || 'Tracking retention settings rows need evidence refs')
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'delete-after-alert-setting' ||
          row.deleteAfterAlertResolved ||
          'Tracking delete-after-alert rows must expose the delete-after-alert setting'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'parent-export-setting' ||
          row.parentExportReady ||
          'Tracking parent export rows must expose parent export readiness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.settingsKind !== 'remote-sync-disabled-setting' ||
          row.custodyScope === 'remote-disabled' ||
          'Tracking remote sync disabled rows must use remote-disabled custody'
      )
    )
);

export const TrackingRetentionSettingsProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-settings-read-model-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingRetentionSettingsRowSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      serviceMutationClaimed: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter((proof) => proof.rows.length >= 5 || 'Tracking retention settings proof needs all settings rows')
  )
);

export type TrackingRetentionSettingsKind = Infer<typeof TrackingRetentionSettingsKindSchema>;
export type TrackingRetentionSettingsRow = Infer<typeof TrackingRetentionSettingsRowSchema>;
export type TrackingRetentionSettingsProof = Infer<typeof TrackingRetentionSettingsProofSchema>;
type TrackingRetentionSettingsEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingRetentionSettingsReadModelProof(generatedAt: string): TrackingRetentionSettingsProof {
  return TrackingRetentionSettingsProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-settings-read-model-proof',
    generatedAt,
    rows: settingsRows(generatedAt),
    productClaims: {
      productClaimReady: false,
      portalUiClaimed: false,
      serviceMutationClaimed: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
    },
  });
}

function settingsRows(timestamp: string): readonly TrackingRetentionSettingsRow[] {
  return [
    row({
      rowId: 'tracking-retention-settings-row-retention-window',
      settingsKind: 'retention-window-setting',
      generatedAt: timestamp,
      evidenceReferences: [evidence('tracking-retention-settings-evidence-window', 'query-store-summary', timestamp)],
      custodyScope: 'parent-device-local',
      retentionWindowHours: 168,
      deleteAfterAlertResolved: false,
      parentExportReady: false,
      reasonCodes: ['tracking-retention-window-read-model-ready'],
      auditRefs: ['tracking-retention-settings-audit-window'],
    }),
    row({
      rowId: 'tracking-retention-settings-row-delete-after-alert',
      settingsKind: 'delete-after-alert-setting',
      generatedAt: timestamp,
      evidenceReferences: [
        evidence('tracking-retention-settings-evidence-delete-after-alert', 'query-store-summary', timestamp),
      ],
      custodyScope: 'parent-device-local',
      retentionWindowHours: null,
      deleteAfterAlertResolved: true,
      parentExportReady: false,
      reasonCodes: ['tracking-delete-after-alert-read-model-ready'],
      auditRefs: ['tracking-retention-settings-audit-delete-after-alert'],
    }),
    retentionSettingRow('parent-export-setting', timestamp),
    remoteDisabledRow('remote-sync-disabled-setting', timestamp),
    remoteDisabledRow('remote-ai-disabled-setting', timestamp),
  ];
}

function retentionSettingRow(
  settingsKind: Extract<TrackingRetentionSettingsKind, 'parent-export-setting'>,
  timestamp: string
): TrackingRetentionSettingsRow {
  return row({
    rowId: 'tracking-retention-settings-row-parent-export',
    settingsKind,
    generatedAt: timestamp,
    evidenceReferences: [evidence('tracking-retention-settings-evidence-export', 'query-store-summary', timestamp)],
    custodyScope: 'parent-owned-export',
    retentionWindowHours: null,
    deleteAfterAlertResolved: false,
    parentExportReady: true,
    reasonCodes: ['tracking-parent-export-read-model-ready'],
    auditRefs: ['tracking-retention-settings-audit-export'],
  });
}

function remoteDisabledRow(
  settingsKind: Extract<TrackingRetentionSettingsKind, 'remote-sync-disabled-setting' | 'remote-ai-disabled-setting'>,
  timestamp: string
): TrackingRetentionSettingsRow {
  const syncRow = settingsKind === 'remote-sync-disabled-setting';
  return row({
    rowId: syncRow
      ? 'tracking-retention-settings-row-remote-sync-disabled'
      : 'tracking-retention-settings-row-remote-ai-disabled',
    settingsKind,
    generatedAt: timestamp,
    evidenceReferences: [
      evidence(
        syncRow
          ? 'tracking-retention-settings-evidence-remote-sync-disabled'
          : 'tracking-retention-settings-evidence-remote-ai-disabled',
        'query-store-summary',
        timestamp
      ),
    ],
    custodyScope: 'remote-disabled',
    retentionWindowHours: null,
    deleteAfterAlertResolved: false,
    parentExportReady: false,
    reasonCodes: [
      syncRow ? 'tracking-remote-sync-disabled-read-model-ready' : 'tracking-remote-ai-disabled-read-model-ready',
    ],
    auditRefs: [
      syncRow
        ? 'tracking-retention-settings-audit-remote-sync-disabled'
        : 'tracking-retention-settings-audit-remote-ai-disabled',
    ],
  });
}

function row(input: {
  readonly rowId: string;
  readonly settingsKind: TrackingRetentionSettingsKind;
  readonly generatedAt: string;
  readonly evidenceReferences: readonly TrackingRetentionSettingsEvidence[];
  readonly custodyScope: TrackingRetentionSettingsRow['custodyScope'];
  readonly retentionWindowHours: number | null;
  readonly deleteAfterAlertResolved: boolean;
  readonly parentExportReady: boolean;
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
}): TrackingRetentionSettingsRow {
  return TrackingRetentionSettingsRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: input.rowId,
    settingsKind: input.settingsKind,
    settingsState: 'settings-read-model-ready',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
    ],
    retentionProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/07-retention-and-custody-model/17-retention-export-proof.json',
    ],
    readModelProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/14-retention-delete-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
    ],
    evidenceReferences: input.evidenceReferences,
    custodyScope: input.custodyScope,
    retentionWindowHours: input.retentionWindowHours,
    deleteAfterAlertResolved: input.deleteAfterAlertResolved,
    parentExportReady: input.parentExportReady,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    settingsReadModelClaimed: true,
    portalUiClaimed: false,
    serviceMutationClaimed: false,
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
  kind: TrackingRetentionSettingsEvidence['kind'],
  observedAt: string
): TrackingRetentionSettingsEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}
