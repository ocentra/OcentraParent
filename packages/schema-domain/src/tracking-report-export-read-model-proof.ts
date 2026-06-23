import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { AgentTrackingRetentionSettingsWriteDefaults } from './agent-tracking-retention-settings-write-command';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingEvidenceTraceSchema } from './tracking-location-policy';
import {
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

export const TrackingReportExportReadModelPacketIdSchema = brandedNonEmptyStringSchema(
  'TrackingReportExportReadModelPacketId'
);

export const TrackingReportExportReadModelProofRefSchema = brandedNonEmptyStringSchema(
  'TrackingReportExportReadModelProofRef'
);

export const TrackingReportExportReadModelKindSchema = withParser(
  Schema.Literal(
    'redacted-report-export-packet',
    'retention-audit-export-packet',
    'family-dashboard-summary-packet',
    'policy-drill-in-export-packet'
  )
);

export const TrackingReportExportReadModelStateSchema = withParser(
  Schema.Literal('export-read-model-ready', 'manual-required')
);

export const TrackingReportExportReadModelProofTierSchema = withParser(
  Schema.Literal('P1_FIXTURE_SIMULATION', 'P2_HOSTED_CI', 'P3_LOCAL_DEV_MACHINE')
);

export const TrackingReportExportCustodyScopeSchema = withParser(
  Schema.Literal('parent-owned-local-export', 'parent-owned-redacted-report', 'manual-required')
);

export const TrackingReportExportReadModelPacketSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    packetId: TrackingReportExportReadModelPacketIdSchema,
    packetKind: TrackingReportExportReadModelKindSchema,
    packetState: TrackingReportExportReadModelStateSchema,
    requiredProofTier: TrackingReportExportReadModelProofTierSchema,
    currentProofTier: TrackingReportExportReadModelProofTierSchema,
    generatedAt: ParentTimestampSchema,
    sourceProofRefs: Schema.Array(TrackingReportExportReadModelProofRefSchema),
    serviceReadModelProofRefs: Schema.Array(TrackingReportExportReadModelProofRefSchema),
    reportConsumerProofRefs: Schema.Array(TrackingReportExportReadModelProofRefSchema),
    dashboardRollupProofRefs: Schema.Array(TrackingReportExportReadModelProofRefSchema),
    retentionSettingsProofRefs: Schema.Array(TrackingReportExportReadModelProofRefSchema),
    evidenceReferences: Schema.Array(TrackingEvidenceTraceSchema),
    exportedRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    redactedEvidenceRefCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    custodyScope: TrackingReportExportCustodyScopeSchema,
    redactionState: Schema.Literal('evidence-refs-only'),
    remoteSyncEnabled: Schema.Literal(false),
    remoteAiEnabled: Schema.Literal(false),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    reportExportReadModelClaimed: Schema.Literal(true),
    rawLocationPayloadClaimed: Schema.Literal(false),
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
        (packet) => packet.sourceProofRefs.length > 0 || 'Tracking report export packets need source proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (packet) =>
          packet.serviceReadModelProofRefs.length > 0 ||
          'Tracking report export packets need service read-model proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (packet) =>
          packet.reportConsumerProofRefs.length > 0 || 'Tracking report export packets need report consumer proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (packet) =>
          packet.dashboardRollupProofRefs.length > 0 ||
          'Tracking report export packets need dashboard rollup proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (packet) =>
          packet.retentionSettingsProofRefs.length > 0 ||
          'Tracking report export packets need retention settings proof refs'
      )
    )
    .pipe(
      Schema.filter(
        (packet) => packet.evidenceReferences.length > 0 || 'Tracking report export packets need evidence refs'
      )
    )
    .pipe(
      Schema.filter(
        (packet) =>
          packet.redactedEvidenceRefCount <= packet.exportedRowCount ||
          'Tracking report export packets cannot cite more redacted refs than exported rows'
      )
    )
    .pipe(
      Schema.filter(
        (packet) =>
          packet.packetKind !== 'retention-audit-export-packet' ||
          packet.custodyScope === 'parent-owned-local-export' ||
          'Tracking retention export packets need parent-owned local export custody'
      )
    )
);

export const TrackingReportExportReadModelProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-report-export-read-model-proof'),
    generatedAt: ParentTimestampSchema,
    packets: Schema.Array(TrackingReportExportReadModelPacketSchema),
    productClaims: Schema.Struct({
      productClaimReady: Schema.Literal(false),
      rawLocationPayloadClaimed: Schema.Literal(false),
      portalUiClaimed: Schema.Literal(false),
      serviceMutationClaimed: Schema.Literal(false),
      platformRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      notificationReceiptClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.packets.length >= 4 || 'Tracking report export proof needs all packet rows'))
);

export type TrackingReportExportReadModelKind = Infer<typeof TrackingReportExportReadModelKindSchema>;
export type TrackingReportExportReadModelPacket = Infer<typeof TrackingReportExportReadModelPacketSchema>;
export type TrackingReportExportReadModelProof = Infer<typeof TrackingReportExportReadModelProofSchema>;
type TrackingReportExportReadModelEvidence = Infer<typeof TrackingEvidenceTraceSchema>;

export function buildTrackingReportExportReadModelProof(generatedAt: string): TrackingReportExportReadModelProof {
  return TrackingReportExportReadModelProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-report-export-read-model-proof',
    generatedAt,
    packets: [
      packet({
        packetId: 'tracking-report-export-packet-redacted-report',
        packetKind: 'redacted-report-export-packet',
        generatedAt,
        evidenceReferences: [
          evidence('tracking-report-export-evidence-redacted-report', 'query-store-summary', generatedAt),
        ],
        exportedRowCount: 6,
        redactedEvidenceRefCount: 6,
        custodyScope: 'parent-owned-redacted-report',
        reasonCodes: ['tracking-redacted-report-export-read-model-ready'],
        auditRefs: ['tracking-report-export-audit-redacted-report'],
      }),
      packet({
        packetId: 'tracking-report-export-packet-retention-audit',
        packetKind: 'retention-audit-export-packet',
        generatedAt,
        evidenceReferences: [
          evidence('tracking-report-export-evidence-retention-audit', 'query-store-summary', generatedAt),
        ],
        exportedRowCount: 5,
        redactedEvidenceRefCount: 5,
        custodyScope: 'parent-owned-local-export',
        reasonCodes: ['tracking-retention-audit-export-read-model-ready'],
        auditRefs: ['tracking-report-export-audit-retention-audit'],
      }),
      packet({
        packetId: 'tracking-report-export-packet-family-dashboard-summary',
        packetKind: 'family-dashboard-summary-packet',
        generatedAt,
        evidenceReferences: [
          evidence('tracking-report-export-evidence-family-dashboard', 'query-store-summary', generatedAt),
        ],
        exportedRowCount: 3,
        redactedEvidenceRefCount: 3,
        custodyScope: 'parent-owned-redacted-report',
        reasonCodes: ['tracking-family-dashboard-summary-export-ready'],
        auditRefs: ['tracking-report-export-audit-family-dashboard'],
      }),
      packet({
        packetId: 'tracking-report-export-packet-policy-drill-in',
        packetKind: 'policy-drill-in-export-packet',
        generatedAt,
        evidenceReferences: [
          evidence('tracking-report-export-evidence-policy-drill-in', 'policy-decision', generatedAt),
        ],
        exportedRowCount: 2,
        redactedEvidenceRefCount: 2,
        custodyScope: 'parent-owned-redacted-report',
        reasonCodes: ['tracking-policy-drill-in-export-ready'],
        auditRefs: ['tracking-report-export-audit-policy-drill-in'],
      }),
    ],
    productClaims: {
      productClaimReady: false,
      rawLocationPayloadClaimed: false,
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

function packet(input: {
  readonly packetId: string;
  readonly packetKind: TrackingReportExportReadModelKind;
  readonly generatedAt: string;
  readonly evidenceReferences: readonly TrackingReportExportReadModelEvidence[];
  readonly exportedRowCount: number;
  readonly redactedEvidenceRefCount: number;
  readonly custodyScope: TrackingReportExportReadModelPacket['custodyScope'];
  readonly reasonCodes: readonly string[];
  readonly auditRefs: readonly string[];
}): TrackingReportExportReadModelPacket {
  return TrackingReportExportReadModelPacketSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    packetId: input.packetId,
    packetKind: input.packetKind,
    packetState: 'export-read-model-ready',
    requiredProofTier: 'P2_HOSTED_CI',
    currentProofTier: 'P2_HOSTED_CI',
    generatedAt: input.generatedAt,
    sourceProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json',
      AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs[1],
    ],
    serviceReadModelProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/18-service-read-model-proof.json',
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/21-product-surface-summary-proof.json',
    ],
    reportConsumerProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/22-report-policy-consumer-proof.json',
    ],
    dashboardRollupProofRefs: [
      'output/tracking-plan-proof/32-journal-sqlite-and-read-model-proof/23-family-dashboard-rollup-proof.json',
    ],
    retentionSettingsProofRefs: [...AgentTrackingRetentionSettingsWriteDefaults.ReadModelProofRefs],
    evidenceReferences: input.evidenceReferences,
    exportedRowCount: input.exportedRowCount,
    redactedEvidenceRefCount: input.redactedEvidenceRefCount,
    custodyScope: input.custodyScope,
    redactionState: 'evidence-refs-only',
    remoteSyncEnabled: false,
    remoteAiEnabled: false,
    reasonCodes: input.reasonCodes,
    auditRefs: input.auditRefs,
    reportExportReadModelClaimed: true,
    rawLocationPayloadClaimed: false,
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
  kind: TrackingReportExportReadModelEvidence['kind'],
  observedAt: string
): TrackingReportExportReadModelEvidence {
  return TrackingEvidenceTraceSchema.parse({
    evidenceReferenceId,
    kind,
    observedAt,
  });
}
