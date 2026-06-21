import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import {
  RequiredTrackingProductionWorkerRuntimeArtifactPlan,
  type TrackingProductionWorkerRuntimeArtifactGateProof,
} from './tracking-production-worker-runtime-artifact-gate-proof';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import { TrackingProductionDurableWorkersReadinessBlockerReferenceSchema } from './tracking-production-durable-workers-readiness-blocker-proof';

export const TrackingProductionWorkerRuntimePreflightAreaSchema = Schema.Literal(
  'location-upload-worker-runtime',
  'retention-cleanup-worker-runtime',
  'notification-outbox-worker-runtime',
  'escalation-timeout-worker-runtime',
  'provider-receipt-worker-runtime',
  'child-device-delivery-worker-runtime',
  'authority-status-worker-runtime',
  'audit-durable-storage-runtime'
);

export const TrackingProductionWorkerRuntimePreflightStatusSchema = Schema.Literal('manual-required');

export const TrackingProductionWorkerRuntimePreflightRowIdSchema =
  brandedNonEmptyStringSchema('TrackingProductionWorkerRuntimePreflightRowId');

export const TrackingProductionWorkerRuntimePreflightPathSchema =
  brandedNonEmptyStringSchema('TrackingProductionWorkerRuntimePreflightPath');

export const TrackingProductionWorkerRuntimePreflightCommandSchema =
  brandedNonEmptyStringSchema('TrackingProductionWorkerRuntimePreflightCommand');

export const TrackingProductionWorkerRuntimePreflightCriterionSchema =
  brandedNonEmptyStringSchema('TrackingProductionWorkerRuntimePreflightCriterion');

const RequiredTrackingProductionWorkerRuntimePreflightRows = [
  {
    rowId: 'tracking-production-worker-runtime-preflight-location-upload',
    area: 'location-upload-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/location-upload-worker-runtime.json',
    acceptanceCriteria: [
      'Production location upload worker accepts real runtime location observations from the approved queue.',
      'Worker execution writes durable upload status, retry state, and redacted audit references.',
      'Artifact includes worker input, durable output, runtime log, and parent-visible read-model reference.',
    ],
    manualValidationCommands: [
      'manual: run production location upload worker against approved durable queue/storage environment',
      'manual: capture location upload worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Local queue/read-model proof is not production location upload worker execution.',
      'Artifact must prove production-like worker execution without promoting product-ready tracking.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-location-upload-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-retention-cleanup',
    area: 'retention-cleanup-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/retention-cleanup-worker-runtime.json',
    acceptanceCriteria: [
      'Production retention cleanup worker executes against platform/runtime retention state.',
      'Worker output records cleanup decision, persisted result, durable audit pointer, and parent receipt state.',
      'Artifact includes retention config, cleanup execution result, runtime log, and product-claim boundary.',
    ],
    manualValidationCommands: [
      'manual: run production retention cleanup worker against approved durable storage',
      'manual: capture retention cleanup worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Retention setting write proof is not production retention cleanup worker execution.',
      'Artifact must prove cleanup worker execution and durable storage behavior together.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-retention-cleanup-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-notification-outbox',
    area: 'notification-outbox-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/notification-outbox-worker-runtime.json',
    acceptanceCriteria: [
      'Production notification outbox worker drains eligible tracking notifications from durable outbox state.',
      'Worker output records quiet-hours handling, retry state, delivery intent, and redacted provider boundary refs.',
      'Artifact includes outbox before/after snapshots, runtime log, and parent notification read-model reference.',
    ],
    manualValidationCommands: [
      'manual: run production notification outbox worker with approved durable outbox state',
      'manual: capture notification outbox worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Local outbox readiness proof is not production notification worker execution.',
      'Artifact must prove worker drain behavior without claiming provider delivery receipt runtime.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-notification-outbox-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-escalation-timeout',
    area: 'escalation-timeout-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/escalation-timeout-worker-runtime.json',
    acceptanceCriteria: [
      'Production escalation timeout worker evaluates expired child response windows from durable state.',
      'Worker output records timeout decision, escalation result, parent notification state, and retry/audit refs.',
      'Artifact includes queue state, timeout execution log, escalation output, and product-claim boundary.',
    ],
    manualValidationCommands: [
      'manual: run production escalation timeout worker with approved durable queue/storage',
      'manual: capture escalation timeout worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Escalation contract proof is not production timeout worker execution.',
      'Artifact must prove timeout worker execution and durable state transition.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-escalation-timeout-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-provider-receipt',
    area: 'provider-receipt-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/provider-receipt-worker-runtime.json',
    acceptanceCriteria: [
      'Production provider receipt worker ingests approved provider receipt events through the runtime boundary.',
      'Worker output records receipt normalization, durable custody, retry/error state, and redacted provider refs.',
      'Artifact includes provider attempt/receipt refs, worker log, persisted receipt state, and parent UI/read-model ref.',
    ],
    manualValidationCommands: [
      'manual: run production provider receipt worker with approved provider/runtime credentials',
      'manual: capture provider receipt worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Provider readiness blocker proof is not provider receipt worker runtime execution.',
      'Artifact must prove runtime receipt ingestion without exposing provider secrets.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-provider-receipt-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-child-device-delivery',
    area: 'child-device-delivery-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/child-device-delivery-worker-runtime.json',
    acceptanceCriteria: [
      'Production child-device delivery worker sends tracking requests through the approved child runtime path.',
      'Worker output records delivery envelope, child runtime receipt, retry/dead-letter state, and parent-visible status.',
      'Artifact includes delivery input, runtime delivery result, child response/ref, and durable audit pointer.',
    ],
    manualValidationCommands: [
      'manual: run production child-device delivery worker against approved child runtime',
      'manual: capture child-device delivery worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Hosted child runtime readiness proof is not production child-device delivery execution.',
      'Artifact must prove delivery on the child runtime path and durable custody.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-child-delivery-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-authority-status',
    area: 'authority-status-worker-runtime',
    sourceMissingArtifactRef: 'tracking-production/authority-status-worker-runtime.json',
    acceptanceCriteria: [
      'Production authority status worker reads approved authority/enrollment runtime state.',
      'Worker output records authority capability, unsupported/degraded state, parent status projection, and audit refs.',
      'Artifact includes authority input state, runtime query/log, persisted result, and product-claim boundary.',
    ],
    manualValidationCommands: [
      'manual: run production authority status worker against approved authority/runtime environment',
      'manual: capture authority status worker artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Authority readiness blockers do not satisfy authority status production worker execution.',
      'Artifact must prove authority status runtime query and durable read-model update.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-authority-status-audit'],
  },
  {
    rowId: 'tracking-production-worker-runtime-preflight-audit-storage',
    area: 'audit-durable-storage-runtime',
    sourceMissingArtifactRef: 'tracking-production/audit-durable-storage-runtime.json',
    acceptanceCriteria: [
      'Production audit durable storage records tracking worker custody across runtime writes and reads.',
      'Storage output records durable key, redacted payload/hash, retention/custody state, and replay/read-model refs.',
      'Artifact includes durable write/read evidence, audit snapshot, and no-product-ready claim boundary.',
    ],
    manualValidationCommands: [
      'manual: run production audit durable storage proof with approved durable storage',
      'manual: capture audit durable storage artifact under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Local SQLite/read-model evidence is not production audit durable storage proof.',
      'Artifact must prove durable storage custody without claiming full product readiness.',
    ],
    auditRefs: ['tracking-production-worker-runtime-preflight-audit-storage-audit'],
  },
] as const;

export const TrackingProductionWorkerRuntimePreflightRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingProductionWorkerRuntimePreflightRowIdSchema,
    generatedAt: ParentTimestampSchema,
    area: TrackingProductionWorkerRuntimePreflightAreaSchema,
    requiredProofTier: Schema.Literal('P4_PRODUCTION_RUNTIME'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingProductionWorkerRuntimePreflightStatusSchema,
    sourceRuntimeArtifactGateProofRef: TrackingProductionWorkerRuntimePreflightPathSchema,
    sourceMissingArtifactRef: TrackingProductionDurableWorkersReadinessBlockerReferenceSchema,
    acceptanceCriteria: Schema.Array(TrackingProductionWorkerRuntimePreflightCriterionSchema),
    manualValidationCommands: Schema.Array(TrackingProductionWorkerRuntimePreflightCommandSchema),
    requiredArtifacts: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
    presentArtifacts: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
    missingArtifacts: Schema.Array(TrackingProductionDurableWorkersReadinessBlockerReferenceSchema),
    artifactAcceptanceNotes: Schema.Array(TrackingProductionWorkerRuntimePreflightCriterionSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    locationUploadWorkerRuntimeClaimed: Schema.Literal(false),
    retentionCleanupWorkerRuntimeClaimed: Schema.Literal(false),
    notificationOutboxWorkerRuntimeClaimed: Schema.Literal(false),
    escalationTimeoutWorkerRuntimeClaimed: Schema.Literal(false),
    providerReceiptWorkerRuntimeClaimed: Schema.Literal(false),
    childDeviceDeliveryWorkerRuntimeClaimed: Schema.Literal(false),
    authorityStatusWorkerRuntimeClaimed: Schema.Literal(false),
    productionAuditDurableStorageClaimed: Schema.Literal(false),
    productionWorkersClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    providerDeliveryReceiptRuntimeClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter((row) => row.acceptanceCriteria.length >= 3 || 'Production worker preflight rows need criteria')
    )
    .pipe(
      Schema.filter(
        (row) => row.manualValidationCommands.length >= 2 || 'Production worker preflight rows need commands'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Production worker preflight rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.missingArtifacts.length > 0 || 'Production worker preflight rows remain manual-required'
      )
    )
);

export const TrackingProductionWorkerRuntimePreflightProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-production-worker-runtime-preflight-proof'),
    generatedAt: ParentTimestampSchema,
    sourceRuntimeArtifactGateProofRef: TrackingProductionWorkerRuntimePreflightPathSchema,
    rows: Schema.Array(TrackingProductionWorkerRuntimePreflightRowSchema),
    summary: Schema.Struct({
      rowCount: Schema.Number,
      manualRequiredRowCount: Schema.Number,
      requiredArtifactCount: Schema.Number,
      presentArtifactCount: Schema.Literal(0),
      missingArtifactCount: Schema.Number,
      productReadyRowCount: Schema.Literal(0),
    }),
    proofClaims: Schema.Struct({
      productionWorkerRuntimePreflightGenerated: Schema.Literal(true),
      runtimeArtifactGateObserved: Schema.Literal(true),
      productionRuntimeArtifactsStillMissing: Schema.Literal(true),
      noLocationUploadWorkerRuntimeClaim: Schema.Literal(true),
      noRetentionCleanupWorkerRuntimeClaim: Schema.Literal(true),
      noNotificationOutboxWorkerRuntimeClaim: Schema.Literal(true),
      noEscalationTimeoutWorkerRuntimeClaim: Schema.Literal(true),
      noProviderReceiptWorkerRuntimeClaim: Schema.Literal(true),
      noChildDeviceDeliveryWorkerRuntimeClaim: Schema.Literal(true),
      noAuthorityStatusWorkerRuntimeClaim: Schema.Literal(true),
      noProductionAuditDurableStorageClaim: Schema.Literal(true),
      noProductionWorkersClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryReceiptRuntimeClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      locationUploadWorkerRuntimeClaimed: Schema.Literal(false),
      retentionCleanupWorkerRuntimeClaimed: Schema.Literal(false),
      notificationOutboxWorkerRuntimeClaimed: Schema.Literal(false),
      escalationTimeoutWorkerRuntimeClaimed: Schema.Literal(false),
      providerReceiptWorkerRuntimeClaimed: Schema.Literal(false),
      childDeviceDeliveryWorkerRuntimeClaimed: Schema.Literal(false),
      authorityStatusWorkerRuntimeClaimed: Schema.Literal(false),
      productionAuditDurableStorageClaimed: Schema.Literal(false),
      productionWorkersClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      providerDeliveryReceiptRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.rows.length === RequiredTrackingProductionWorkerRuntimePreflightRows.length ||
          'Production worker runtime preflight must cover every production artifact row'
      )
    )
    .pipe(
      Schema.filter(
        (proof) => proof.summary.rowCount === proof.rows.length || 'Production worker preflight row count mismatch'
      )
    )
);

export type TrackingProductionWorkerRuntimePreflightProof = Infer<
  typeof TrackingProductionWorkerRuntimePreflightProofSchema
>;
export type TrackingProductionWorkerRuntimePreflightRow = Infer<
  typeof TrackingProductionWorkerRuntimePreflightRowSchema
>;

export const RequiredTrackingProductionWorkerRuntimePreflightPlan = {
  sourceRuntimeArtifactGateProofRef: 'test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json',
  rows: RequiredTrackingProductionWorkerRuntimePreflightRows,
} as const;

export function buildTrackingProductionWorkerRuntimePreflightProof(
  generatedAt: string,
  runtimeArtifactGateProof: TrackingProductionWorkerRuntimeArtifactGateProof
): TrackingProductionWorkerRuntimePreflightProof {
  assertRuntimeGateStillRequiresProductionArtifacts(runtimeArtifactGateProof);
  const rows = RequiredTrackingProductionWorkerRuntimePreflightRows.map((row) => preflightRow(generatedAt, row));

  return TrackingProductionWorkerRuntimePreflightProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-production-worker-runtime-preflight-proof',
    generatedAt,
    sourceRuntimeArtifactGateProofRef:
      RequiredTrackingProductionWorkerRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef,
    rows,
    summary: {
      rowCount: rows.length,
      manualRequiredRowCount: rows.filter((row) => row.status === 'manual-required').length,
      requiredArtifactCount: rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      presentArtifactCount: 0,
      missingArtifactCount: rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      productReadyRowCount: 0,
    },
    proofClaims: {
      productionWorkerRuntimePreflightGenerated: true,
      runtimeArtifactGateObserved: true,
      productionRuntimeArtifactsStillMissing: true,
      noLocationUploadWorkerRuntimeClaim: true,
      noRetentionCleanupWorkerRuntimeClaim: true,
      noNotificationOutboxWorkerRuntimeClaim: true,
      noEscalationTimeoutWorkerRuntimeClaim: true,
      noProviderReceiptWorkerRuntimeClaim: true,
      noChildDeviceDeliveryWorkerRuntimeClaim: true,
      noAuthorityStatusWorkerRuntimeClaim: true,
      noProductionAuditDurableStorageClaim: true,
      noProductionWorkersClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryReceiptRuntimeClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      locationUploadWorkerRuntimeClaimed: false,
      retentionCleanupWorkerRuntimeClaimed: false,
      notificationOutboxWorkerRuntimeClaimed: false,
      escalationTimeoutWorkerRuntimeClaimed: false,
      providerReceiptWorkerRuntimeClaimed: false,
      childDeviceDeliveryWorkerRuntimeClaimed: false,
      authorityStatusWorkerRuntimeClaimed: false,
      productionAuditDurableStorageClaimed: false,
      productionWorkersClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      providerDeliveryReceiptRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function assertRuntimeGateStillRequiresProductionArtifacts(
  runtimeArtifactGateProof: TrackingProductionWorkerRuntimeArtifactGateProof
): void {
  const [runtimeGateRow] = runtimeArtifactGateProof.rows;
  if (!runtimeGateRow) throw new Error('Production worker runtime artifact gate proof is missing its row.');
  for (const artifact of RequiredTrackingProductionWorkerRuntimeArtifactPlan.requiredArtifacts) {
    if (!runtimeGateRow.missingArtifacts.some((missingArtifact) => missingArtifact === artifact)) {
      throw new Error(`Production worker runtime preflight requires missing artifact ${artifact}.`);
    }
  }
  if (runtimeArtifactGateProof.productClaims.productClaimReady) {
    throw new Error('Production worker runtime preflight cannot run against product-ready production claims.');
  }
}

function preflightRow(
  generatedAt: string,
  row: (typeof RequiredTrackingProductionWorkerRuntimePreflightRows)[number]
): TrackingProductionWorkerRuntimePreflightRow {
  return TrackingProductionWorkerRuntimePreflightRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: row.rowId,
    generatedAt,
    area: row.area,
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual-required',
    sourceRuntimeArtifactGateProofRef:
      RequiredTrackingProductionWorkerRuntimePreflightPlan.sourceRuntimeArtifactGateProofRef,
    sourceMissingArtifactRef: row.sourceMissingArtifactRef,
    acceptanceCriteria: [...row.acceptanceCriteria],
    manualValidationCommands: [...row.manualValidationCommands],
    requiredArtifacts: [row.sourceMissingArtifactRef],
    presentArtifacts: [],
    missingArtifacts: [row.sourceMissingArtifactRef],
    artifactAcceptanceNotes: [...row.artifactAcceptanceNotes],
    auditRefs: [...row.auditRefs],
    locationUploadWorkerRuntimeClaimed: false,
    retentionCleanupWorkerRuntimeClaimed: false,
    notificationOutboxWorkerRuntimeClaimed: false,
    escalationTimeoutWorkerRuntimeClaimed: false,
    providerReceiptWorkerRuntimeClaimed: false,
    childDeviceDeliveryWorkerRuntimeClaimed: false,
    authorityStatusWorkerRuntimeClaimed: false,
    productionAuditDurableStorageClaimed: false,
    productionWorkersClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    providerDeliveryReceiptRuntimeClaimed: false,
    productClaimReady: false,
  });
}

