import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingEvidenceTraceSchema, TrackingPolicySchemaVersion } from './tracking-location-policy';
import { TrackingPolicyAuditRefSchema, TrackingPolicyReasonCodeSchema } from './tracking-location-policy-primitives';

const TrackingHostedStorageDefaultTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingHostedStorageDefaultRowIdSchema = TrackingHostedStorageDefaultTextSchema.pipe(
  Schema.brand('TrackingHostedStorageDefaultRowId')
);

export const TrackingHostedStorageDefaultProofRefSchema = TrackingHostedStorageDefaultTextSchema.pipe(
  Schema.brand('TrackingHostedStorageDefaultProofRef')
);

export const TrackingHostedStorageDefaultKindSchema = withParser(
  Schema.Literal(
    'journal-local-default',
    'sqlite-read-model-local-default',
    'parent-export-local-default',
    'ai-context-stored-ref-local-default',
    'remote-sync-disabled-default'
  )
);

export const TrackingHostedStorageDefaultStateSchema = withParser(
  Schema.Literal('hosted-storage-not-default', 'manual-required')
);

export const TrackingHostedStorageDefaultTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingHostedStorageDefaultCustodySchema = withParser(
  Schema.Literal('child-device-local', 'parent-device-local', 'parent-owned-export', 'remote-disabled')
);

export const TrackingHostedStorageDefaultRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingHostedStorageDefaultRowIdSchema,
    boundaryKind: TrackingHostedStorageDefaultKindSchema,
    boundaryState: TrackingHostedStorageDefaultStateSchema,
    requiredProofTier: TrackingHostedStorageDefaultTierSchema,
    currentProofTier: TrackingHostedStorageDefaultTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingHostedStorageDefaultProofRefSchema),
    journalProofRefs: Schema.Array(TrackingHostedStorageDefaultProofRefSchema),
    readModelProofRefs: Schema.Array(TrackingHostedStorageDefaultProofRefSchema),
    retentionProofRefs: Schema.Array(TrackingHostedStorageDefaultProofRefSchema),
    aiConsumerProofRefs: Schema.Array(TrackingHostedStorageDefaultProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    defaultCustody: TrackingHostedStorageDefaultCustodySchema,
    ocentraHostedStorageDefault: Schema.Literal(false),
    rawLocationRemoteUploadEnabled: Schema.Literal(false),
    sqliteSnapshotRemoteUploadEnabled: Schema.Literal(false),
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    parentOwnedExportRequired: Schema.Boolean,
    storedRefConsumerRequired: Schema.Boolean,
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    hostedStorageBoundaryClaimed: Schema.Literal(true),
    portalUiClaimed: Schema.Literal(false),
    serviceMutationClaimed: Schema.Literal(false),
    platformRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    notificationReceiptClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productionBehaviorClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.sourceProofRefs.length > 0 || 'Tracking storage rows need source proof refs'))
    .pipe(Schema.filter((row) => row.journalProofRefs.length > 0 || 'Tracking storage rows need journal proof refs'))
    .pipe(
      Schema.filter((row) => row.readModelProofRefs.length > 0 || 'Tracking storage rows need read-model proof refs')
    )
    .pipe(Schema.filter((row) => row.evidenceReferences.length > 0 || 'Tracking storage rows need evidence references'))
    .pipe(
      Schema.filter(
        (row) =>
          row.boundaryKind !== 'parent-export-local-default' ||
          (row.defaultCustody === 'parent-owned-export' && row.parentOwnedExportRequired) ||
          'Tracking parent export rows must require parent-owned export custody'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.boundaryKind !== 'ai-context-stored-ref-local-default' ||
          (row.aiConsumerProofRefs.length > 0 && row.storedRefConsumerRequired) ||
          'Tracking AI context rows must require stored-ref consumer proof'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.boundaryKind !== 'remote-sync-disabled-default' ||
          row.defaultCustody === 'remote-disabled' ||
          'Tracking remote-sync default rows must use remote-disabled custody'
      )
    )
);

export const TrackingHostedStorageDefaultProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-hosted-storage-default-boundary-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingHostedStorageDefaultRowSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      ocentraHostedStorageDefault: Schema.Literal(false),
      rawLocationRemoteUploadEnabled: Schema.Literal(false),
      sqliteSnapshotRemoteUploadEnabled: Schema.Literal(false),
      remoteSyncEnabled: Schema.Literal(false),
      remoteAiEnabled: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      serviceMutationClaimed: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      productionBehaviorClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter((proof) => proof.rows.length >= 5 || 'Tracking hosted storage default proof needs all boundary rows')
  )
);

export type TrackingHostedStorageDefaultKind = Infer<typeof TrackingHostedStorageDefaultKindSchema>;
export type TrackingHostedStorageDefaultRow = Infer<typeof TrackingHostedStorageDefaultRowSchema>;
export type TrackingHostedStorageDefaultProof = Infer<typeof TrackingHostedStorageDefaultProofSchema>;
type TrackingHostedStorageDefaultEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingHostedStorageDefaultBoundaryProof(generatedAt: string): TrackingHostedStorageDefaultProof {
  return TrackingHostedStorageDefaultProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-hosted-storage-default-boundary-proof',
    generatedAt,
    rows: buildHostedStorageDefaultRows(generatedAt),
    productClaims: {
      productClaimReady: false,
      ocentraHostedStorageDefault: false,
      rawLocationRemoteUploadEnabled: false,
      sqliteSnapshotRemoteUploadEnabled: false,
      remoteSyncEnabled: false,
      remoteAiEnabled: false,
      portalUiClaimed: false,
      serviceMutationClaimed: false,
      platformRuntimeClaimed: false,
      childDeviceDeliveryClaimed: false,
      providerDeliveryClaimed: false,
      notificationReceiptClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
      productionBehaviorClaimed: false,
    },
  });
}

function buildHostedStorageDefaultRows(generatedAt: string): readonly TrackingHostedStorageDefaultRow[] {
  return [
    row({
      rowId: 'tracking-hosted-storage-default-row-journal-local',
      boundaryKind: 'journal-local-default',
      generatedAt,
      defaultCustody: 'parent-device-local',
      evidenceId: 'tracking-hosted-storage-default-evidence-journal-local',
      evidenceKind: 'journal-event',
      reasonCodes: ['tracking-journal-local-default'],
      auditRefs: ['tracking-hosted-storage-default-audit-journal-local'],
    }),
    row({
      rowId: 'tracking-hosted-storage-default-row-sqlite-read-model-local',
      boundaryKind: 'sqlite-read-model-local-default',
      generatedAt,
      defaultCustody: 'parent-device-local',
      evidenceId: 'tracking-hosted-storage-default-evidence-sqlite-read-model-local',
      evidenceKind: 'query-store-summary',
      reasonCodes: ['tracking-sqlite-read-model-local-default'],
      auditRefs: ['tracking-hosted-storage-default-audit-sqlite-read-model-local'],
    }),
    parentExportRow(generatedAt),
    aiContextStoredRefRow(generatedAt),
    remoteSyncDisabledRow(generatedAt),
  ];
}

function parentExportRow(generatedAt: string): TrackingHostedStorageDefaultRow {
  return row({
    rowId: 'tracking-hosted-storage-default-row-parent-export-local',
    boundaryKind: 'parent-export-local-default',
    generatedAt,
    defaultCustody: 'parent-owned-export',
    evidenceId: 'tracking-hosted-storage-default-evidence-parent-export-local',
    evidenceKind: 'query-store-summary',
    parentOwnedExportRequired: true,
    reasonCodes: ['tracking-parent-export-local-default'],
    auditRefs: ['tracking-hosted-storage-default-audit-parent-export-local'],
  });
}

function aiContextStoredRefRow(generatedAt: string): TrackingHostedStorageDefaultRow {
  return row({
    rowId: 'tracking-hosted-storage-default-row-ai-context-stored-ref-local',
    boundaryKind: 'ai-context-stored-ref-local-default',
    generatedAt,
    defaultCustody: 'parent-device-local',
    evidenceId: 'tracking-hosted-storage-default-evidence-ai-context-stored-ref-local',
    evidenceKind: 'query-store-summary',
    aiConsumerProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/30-ai-stored-ref-consumer-proof.json',
    ],
    storedRefConsumerRequired: true,
    reasonCodes: ['tracking-ai-context-stored-ref-local-default'],
    auditRefs: ['tracking-hosted-storage-default-audit-ai-context-stored-ref-local'],
  });
}

function remoteSyncDisabledRow(generatedAt: string): TrackingHostedStorageDefaultRow {
  return row({
    rowId: 'tracking-hosted-storage-default-row-remote-sync-disabled',
    boundaryKind: 'remote-sync-disabled-default',
    generatedAt,
    defaultCustody: 'remote-disabled',
    evidenceId: 'tracking-hosted-storage-default-evidence-remote-sync-disabled',
    evidenceKind: 'query-store-summary',
    reasonCodes: ['tracking-remote-sync-disabled-default'],
    auditRefs: ['tracking-hosted-storage-default-audit-remote-sync-disabled'],
  });
}

function row(input: {
  readonly rowId: string;
  readonly boundaryKind: TrackingHostedStorageDefaultKind;
  readonly generatedAt: string;
  readonly defaultCustody: TrackingHostedStorageDefaultRow['defaultCustody'];
  readonly evidenceId: string;
  readonly evidenceKind: TrackingHostedStorageDefaultEvidence['kind'];
  readonly parentOwnedExportRequired?: boolean;
  readonly aiConsumerProofRefs?: readonly string[];
  readonly storedRefConsumerRequired?: boolean;
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
}): TrackingHostedStorageDefaultRow {
  return TrackingHostedStorageDefaultRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: input.rowId,
    boundaryKind: input.boundaryKind,
    boundaryState: 'hosted-storage-not-default',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/22-local-parent-defined-place-database/proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
    ],
    journalProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/10-journal-sqlite-proof.json',
    ],
    readModelProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
    ],
    retentionProofRefs: [
      'output/tracking-plan-proof/07-retention-and-custody-model/14-retention-delete-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/24-retention-settings-read-model-proof.json',
    ],
    aiConsumerProofRefs: [...(input.aiConsumerProofRefs ?? [])],
    evidenceReferences: [evidence(input.evidenceId, input.evidenceKind, input.generatedAt)],
    defaultCustody: input.defaultCustody,
    ocentraHostedStorageDefault: false,
    rawLocationRemoteUploadEnabled: false,
    sqliteSnapshotRemoteUploadEnabled: false,
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    parentOwnedExportRequired: input.parentOwnedExportRequired ?? false,
    storedRefConsumerRequired: input.storedRefConsumerRequired ?? false,
    reasonCodes: [...input.reasonCodes],
    auditRefs: [...input.auditRefs],
    hostedStorageBoundaryClaimed: true,
    portalUiClaimed: false,
    serviceMutationClaimed: false,
    platformRuntimeClaimed: false,
    childDeviceDeliveryClaimed: false,
    providerDeliveryClaimed: false,
    notificationReceiptClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productionBehaviorClaimed: false,
    productClaimReady: false,
  });
}

function evidence(
  evidenceReferenceId: string,
  kind: TrackingHostedStorageDefaultEvidence['kind'],
  observedAt: string
): TrackingHostedStorageDefaultEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}
