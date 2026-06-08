import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingRealRuntimeHandoffTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingRealRuntimeHandoffAreaSchema = Schema.Literal(
  'android-physical-background-and-geofence',
  'ios-physical-background-and-region',
  'child-device-runtime-execution',
  'full-product-parent-child-ui-runtime',
  'authority-enrolled-hard-control-runtime',
  'provider-delivery-receipt-runtime',
  'retention-product-runtime-enforcement',
  'production-durable-workers-and-storage',
  'escalation-runtime-workers-and-storage'
);

export const TrackingRealRuntimeHandoffProofTierSchema = Schema.Literal(
  'P4_PHYSICAL_DEVICE',
  'P4_MANUAL_PROVIDER_RUNTIME',
  'P4_PRODUCTION_RUNTIME'
);

export const TrackingRealRuntimeHandoffStatusSchema = Schema.Literal('manual-required', 'artifact-set-present');

export const TrackingRealRuntimeHandoffReadinessCategorySchema = Schema.Literal(
  'physical-device-required',
  'manual-provider-runtime-required',
  'production-runtime-required'
);

export const TrackingRealRuntimeHandoffBlockerSchema = Schema.Literal(
  'android-physical-background-proof-required',
  'ios-physical-region-proof-required',
  'retention-writable-product-settings-required',
  'retention-platform-runtime-enforcement-required',
  'actual-child-device-runtime-required',
  'full-product-parent-child-ui-required',
  'authority-enrollment-proof-required',
  'provider-delivery-receipt-runtime-required',
  'production-durable-workers-required'
);

export const TrackingRealRuntimeHandoffArtifactPathSchema = TrackingRealRuntimeHandoffTextSchema.pipe(
  Schema.brand('TrackingRealRuntimeHandoffArtifactPath')
);

export const TrackingRealRuntimeHandoffRowIdSchema = TrackingRealRuntimeHandoffTextSchema.pipe(
  Schema.brand('TrackingRealRuntimeHandoffRowId')
);

export const TrackingRealRuntimeHandoffCommandSchema = TrackingRealRuntimeHandoffTextSchema.pipe(
  Schema.brand('TrackingRealRuntimeHandoffCommand')
);

export const TrackingRealRuntimeHandoffRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRealRuntimeHandoffRowIdSchema,
    generatedAt: ParentTimestampSchema,
    handoffArea: TrackingRealRuntimeHandoffAreaSchema,
    blockerId: TrackingRealRuntimeHandoffBlockerSchema,
    sourceProofRef: TrackingRealRuntimeHandoffArtifactPathSchema,
    proofRoot: TrackingRealRuntimeHandoffArtifactPathSchema,
    requiredProofTier: TrackingRealRuntimeHandoffProofTierSchema,
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingRealRuntimeHandoffStatusSchema,
    requiredArtifacts: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    presentArtifacts: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    missingArtifacts: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    readinessCategory: TrackingRealRuntimeHandoffReadinessCategorySchema,
    ciRunnable: Schema.Literal(false),
    requiredValidationCommands: Schema.Array(TrackingRealRuntimeHandoffCommandSchema),
    artifactAcceptanceNotes: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    artifactSetComplete: Schema.Boolean,
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Real-runtime handoff rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          (row.requiredValidationCommands.length > 0 && row.artifactAcceptanceNotes.length > 0) ||
          'Real-runtime handoff rows need validation commands and artifact acceptance notes'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Real-runtime handoff rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.artifactSetComplete ||
          'Real-runtime handoff status must match artifact completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.artifactSetComplete ? row.missingArtifacts.length === 0 : row.missingArtifacts.length > 0) ||
          'Real-runtime handoff completeness must match missing artifact count'
      )
    )
);

export const TrackingRealRuntimeHandoffClosureAccountingSchema = withParser(
  Schema.Struct({
    fullProductUiLocalArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiClosureRetentionWritableExecutionRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    fullProductUiClosureChildRuntimeMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeRequiredArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    retentionRuntimePresentArtifactCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeMissingArtifactCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeManualRequiredRowCount: Schema.Number.pipe(Schema.int()),
    retentionRuntimeArtifactSetPresentRowCount: Schema.Number.pipe(Schema.int()),
    claimAuditPresentArtifactCount: Schema.Number.pipe(Schema.int()),
    claimAuditMissingArtifactCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditManualRequiredRowCount: Schema.Number.pipe(Schema.int(), Schema.positive()),
    claimAuditProductReadyRowCount: Schema.Literal(0),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (accounting) =>
          accounting.retentionRuntimeRequiredArtifactCount ===
            accounting.retentionRuntimePresentArtifactCount + accounting.retentionRuntimeMissingArtifactCount ||
          'Real-runtime closure accounting must classify every retention runtime artifact'
      )
    )
    .pipe(
      Schema.filter(
        (accounting) =>
          (accounting.retentionRuntimePresentArtifactCount >= 0 &&
            accounting.retentionRuntimeMissingArtifactCount >= 0 &&
            accounting.retentionRuntimeManualRequiredRowCount >= 0 &&
            accounting.retentionRuntimeArtifactSetPresentRowCount >= 0) ||
          'Real-runtime closure accounting cannot record negative retention runtime counts'
      )
    )
);

export const TrackingRealRuntimeHandoffProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-real-runtime-handoff-proof'),
    generatedAt: ParentTimestampSchema,
    requiredProofTier: Schema.Literal('P4_REAL_RUNTIME_HANDOFF'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    currentStatus: Schema.Literal('manual_required'),
    sourceGateRefs: Schema.Array(TrackingRealRuntimeHandoffArtifactPathSchema),
    closureProofRef: TrackingRealRuntimeHandoffArtifactPathSchema,
    closureAccounting: TrackingRealRuntimeHandoffClosureAccountingSchema,
    handoffRows: Schema.Array(TrackingRealRuntimeHandoffRowSchema),
    summary: Schema.Struct({
      handoffRowCount: Schema.Number,
      requiredArtifactCount: Schema.Number,
      presentArtifactCount: Schema.Number,
      missingArtifactCount: Schema.Number,
      requiredValidationCommandCount: Schema.Number,
      manualRequiredRowCount: Schema.Number,
      artifactSetPresentRowCount: Schema.Number,
      physicalDeviceRequiredRowCount: Schema.Number,
      manualProviderRuntimeRequiredRowCount: Schema.Number,
      productionRuntimeRequiredRowCount: Schema.Number,
      ciRunnableRowCount: Schema.Literal(0),
      productReadyRowCount: Schema.Literal(0),
    }),
    productClaims: Schema.Struct({
      physicalDeviceClaimed: Schema.Literal(false),
      actualChildDeviceRuntimeClaimed: Schema.Literal(false),
      fullProductUiClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      providerDeliveryReceiptRuntimeClaimed: Schema.Literal(false),
      retentionProductRuntimeClaimed: Schema.Literal(false),
      productionWorkersClaimed: Schema.Literal(false),
      escalationRuntimeClaimed: Schema.Literal(false),
      productReadyClaimed: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.handoffRows.length === RequiredTrackingRealRuntimeHandoffGates.length ||
          'Real-runtime handoff proof must cover every required handoff gate'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.sourceGateRefs.length === RequiredTrackingRealRuntimeHandoffGates.length ||
          'Real-runtime handoff proof must cite every source gate'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.handoffRowCount === proof.handoffRows.length ||
          'Real-runtime handoff summary row count must match rows'
      )
    )
    .pipe(
      Schema.filter(
        (proof) => proof.summary.productReadyRowCount === 0 || 'Real-runtime handoff must not claim product-ready rows'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.ciRunnableRowCount === 0 || 'Real-runtime handoff cannot mark P4/P5/P6 rows as CI-runnable'
      )
    )
);

export type TrackingRealRuntimeHandoffProof = Infer<typeof TrackingRealRuntimeHandoffProofSchema>;
export type TrackingRealRuntimeHandoffRow = Infer<typeof TrackingRealRuntimeHandoffRowSchema>;
export type TrackingRealRuntimeHandoffClosureAccounting = Infer<
  typeof TrackingRealRuntimeHandoffClosureAccountingSchema
>;

export interface TrackingRealRuntimeHandoffGateInventory {
  readonly handoffArea: (typeof RequiredTrackingRealRuntimeHandoffGates)[number]['handoffArea'];
  readonly proofRoot: string;
  readonly requiredArtifacts: readonly string[];
  readonly presentArtifacts: readonly string[];
  readonly auditRefs: readonly string[];
}

export const RequiredTrackingRealRuntimeHandoffGates = [
  {
    handoffArea: 'android-physical-background-and-geofence',
    blockerId: 'android-physical-background-proof-required',
    sourceProofRef: 'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
    sourceRowIds: ['tracking-physical-device-artifacts-android'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredValidationCommands: [
      'Run Android physical-device background location and geofence transition proof on enrolled child hardware',
      'Record device metadata, runtime permission state, location events, geofence transitions, logcat, parent UI receipt, and summary under output/tracking-plan-proof/android-background-geofence/',
    ],
    artifactAcceptanceNotes: [
      'Android artifacts must come from a physical child device or explicitly approved equivalent hardware run',
      'Foreground-only emulator samples do not satisfy this handoff row',
    ],
  },
  {
    handoffArea: 'ios-physical-background-and-region',
    blockerId: 'ios-physical-region-proof-required',
    sourceProofRef: 'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
    sourceRowIds: ['tracking-physical-device-artifacts-ios'],
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredValidationCommands: [
      'Run iOS physical-device Always authorization, region monitoring, background delivery, and relaunch proof',
      'Record device metadata, authorization state, region transitions, Xcode/device logs, screenshots, and summary under output/tracking-plan-proof/ios-region-monitoring/',
    ],
    artifactAcceptanceNotes: [
      'iOS artifacts must come from an entitled physical device run',
      'Simulator privacy disclosure or package-preview proof does not satisfy this handoff row',
    ],
  },
  {
    handoffArea: 'child-device-runtime-execution',
    blockerId: 'actual-child-device-runtime-required',
    sourceProofRef: 'test-results/tracking-child-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredValidationCommands: [
      'Run actual child-device check-in, consent, safe/help, and timeout execution flow on child runtime hardware',
      'Record delivery envelope, execution result, visible child UI, parent receipt, runtime observations, device log, and summary under output/tracking-plan-proof/child-device-runtime-execution/',
    ],
    artifactAcceptanceNotes: [
      'Hosted disclosure screenshots do not satisfy rendered child-device runtime UI',
      'The artifact set must prove delivery and execution, not only copy or readiness',
    ],
  },
  {
    handoffArea: 'full-product-parent-child-ui-runtime',
    blockerId: 'full-product-parent-child-ui-required',
    sourceProofRef: 'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredValidationCommands: [
      'Run full product parent and child UI runtime proof across parent overview, device detail, notifications, retention write, child check-in, child consent, safe/help, accessibility, and end-to-end trace',
      'Record the required product UI artifacts under output/tracking-plan-proof/product-parent-child-ui-runtime/',
    ],
    artifactAcceptanceNotes: [
      'Hosted-route screenshots only satisfy local/CI UI inventory, not full product runtime UI',
      'Child UI artifacts must come from rendered child runtime surfaces',
    ],
  },
  {
    handoffArea: 'authority-enrolled-hard-control-runtime',
    blockerId: 'authority-enrollment-proof-required',
    sourceProofRef: 'test-results/tracking-authority-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredValidationCommands: [
      'Run authority-enrolled hard-control tracking proof on enrolled child hardware',
      'Record enrollment status, control capability state, runtime observation, parent UI, device logs, and summary under output/tracking-plan-proof/authority-runtime/',
    ],
    artifactAcceptanceNotes: [
      'Manual-required authority rows and unsupported-platform states do not satisfy enrolled hard-control runtime proof',
      'The artifact set must prove authority status and runtime behavior together',
    ],
  },
  {
    handoffArea: 'provider-delivery-receipt-runtime',
    blockerId: 'provider-delivery-receipt-runtime-required',
    sourceProofRef: 'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
    requiredValidationCommands: [
      'Run provider delivery and receipt ingestion proof with approved credentials and redacted runtime config',
      'Record provider attempt/response, receipt webhook, ingestion result, retry/quiet-hours worker log, parent notification UI, and summary under output/tracking-plan-proof/provider-delivery-runtime/',
    ],
    artifactAcceptanceNotes: [
      'Local outbox or preference status proof does not satisfy provider delivery runtime',
      'Provider credentials must be attested without leaking secrets',
    ],
  },
  {
    handoffArea: 'retention-product-runtime-enforcement',
    blockerId: 'retention-platform-runtime-enforcement-required',
    sourceProofRef: 'test-results/tracking-retention-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredValidationCommands: [
      'Run retention product runtime enforcement proof against platform/runtime storage',
      'Record retention config, persisted cleanup/enforcement event, audit snapshot, UI/result evidence, and summary under output/tracking-plan-proof/retention-runtime/',
    ],
    artifactAcceptanceNotes: [
      'Local writable settings proof does not satisfy platform runtime retention enforcement',
      'The artifact set must show enforcement behavior, not only settings persistence',
    ],
  },
  {
    handoffArea: 'production-durable-workers-and-storage',
    blockerId: 'production-durable-workers-required',
    sourceProofRef: 'test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    requiredValidationCommands: [
      'Run production durable worker/storage proof for location upload, retention cleanup, notification outbox, escalation timeout, provider receipt, child-device delivery, authority status, and audit storage',
      'Record all production worker artifacts under output/tracking-plan-proof/tracking-production/',
    ],
    artifactAcceptanceNotes: [
      'Local durable stores and production-readiness blockers do not satisfy production worker runtime proof',
      'Artifacts must come from the approved production-like worker/storage environment',
    ],
  },
  {
    handoffArea: 'escalation-runtime-workers-and-storage',
    blockerId: 'production-durable-workers-required',
    sourceProofRef: 'test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json',
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    requiredValidationCommands: [
      'Run escalation timeout worker/storage runtime proof with real durable queue/storage evidence',
      'Record queue state, timeout execution, parent notification/escalation output, worker logs, and summary under output/tracking-plan-proof/escalation-runtime/',
    ],
    artifactAcceptanceNotes: [
      'Escalation contract or local fixture proof does not satisfy production worker/storage runtime',
      'Artifacts must prove timeout execution and durable storage behavior together',
    ],
  },
] as const;

export function buildTrackingRealRuntimeHandoffProof(
  generatedAt: string,
  inventories: readonly TrackingRealRuntimeHandoffGateInventory[],
  closureAccountingInput: TrackingRealRuntimeHandoffClosureAccounting
): TrackingRealRuntimeHandoffProof {
  const handoffRows = RequiredTrackingRealRuntimeHandoffGates.map((gate) =>
    realRuntimeHandoffRow(generatedAt, gate, inventories)
  );
  const closureAccounting = TrackingRealRuntimeHandoffClosureAccountingSchema.parse(closureAccountingInput);

  return TrackingRealRuntimeHandoffProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-real-runtime-handoff-proof',
    generatedAt,
    requiredProofTier: 'P4_REAL_RUNTIME_HANDOFF',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    sourceGateRefs: RequiredTrackingRealRuntimeHandoffGates.map((gate) => gate.sourceProofRef),
    closureProofRef: 'test-results/tracking-product-readiness-closure-proof/proof.json',
    closureAccounting,
    handoffRows,
    summary: summarizeRealRuntimeHandoffRows(handoffRows),
    productClaims: {
      physicalDeviceClaimed: false,
      actualChildDeviceRuntimeClaimed: false,
      fullProductUiClaimed: false,
      authorityClaimed: false,
      providerDeliveryReceiptRuntimeClaimed: false,
      retentionProductRuntimeClaimed: false,
      productionWorkersClaimed: false,
      escalationRuntimeClaimed: false,
      productReadyClaimed: false,
    },
  });
}

function realRuntimeHandoffRow(
  generatedAt: string,
  gate: (typeof RequiredTrackingRealRuntimeHandoffGates)[number],
  inventories: readonly TrackingRealRuntimeHandoffGateInventory[]
): TrackingRealRuntimeHandoffRow {
  const inventory = inventories.find((candidate) => candidate.handoffArea === gate.handoffArea);
  const requiredArtifacts = inventory?.requiredArtifacts ?? [];
  const presentArtifactSet = new Set(inventory?.presentArtifacts ?? []);
  const presentArtifacts = requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const artifactSetComplete = missingArtifacts.length === 0;

  return TrackingRealRuntimeHandoffRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-real-runtime-handoff-${gate.handoffArea}`,
    generatedAt,
    handoffArea: gate.handoffArea,
    blockerId: gate.blockerId,
    sourceProofRef: gate.sourceProofRef,
    proofRoot: inventory?.proofRoot ?? gate.sourceProofRef,
    requiredProofTier: gate.requiredProofTier,
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: artifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts,
    presentArtifacts,
    missingArtifacts,
    readinessCategory: readinessCategoryForTier(gate.requiredProofTier),
    ciRunnable: false,
    requiredValidationCommands: [...gate.requiredValidationCommands],
    artifactAcceptanceNotes: [...gate.artifactAcceptanceNotes],
    auditRefs: [...(inventory?.auditRefs ?? [`tracking-real-runtime-handoff-${gate.handoffArea}-audit`])],
    artifactSetComplete,
    productClaimReady: false,
  });
}

function readinessCategoryForTier(
  tier: (typeof RequiredTrackingRealRuntimeHandoffGates)[number]['requiredProofTier']
): TrackingRealRuntimeHandoffRow['readinessCategory'] {
  if (tier === 'P4_MANUAL_PROVIDER_RUNTIME') return 'manual-provider-runtime-required';
  if (tier === 'P4_PRODUCTION_RUNTIME') return 'production-runtime-required';
  return 'physical-device-required';
}

function summarizeRealRuntimeHandoffRows(
  handoffRows: readonly TrackingRealRuntimeHandoffRow[]
): TrackingRealRuntimeHandoffProof['summary'] {
  return {
    handoffRowCount: handoffRows.length,
    requiredArtifactCount: handoffRows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
    presentArtifactCount: handoffRows.reduce((total, row) => total + row.presentArtifacts.length, 0),
    missingArtifactCount: handoffRows.reduce((total, row) => total + row.missingArtifacts.length, 0),
    requiredValidationCommandCount: handoffRows.reduce(
      (total, row) => total + row.requiredValidationCommands.length,
      0
    ),
    manualRequiredRowCount: handoffRows.filter((row) => row.status === 'manual-required').length,
    artifactSetPresentRowCount: handoffRows.filter((row) => row.status === 'artifact-set-present').length,
    physicalDeviceRequiredRowCount: handoffRows.filter((row) => row.readinessCategory === 'physical-device-required')
      .length,
    manualProviderRuntimeRequiredRowCount: handoffRows.filter(
      (row) => row.readinessCategory === 'manual-provider-runtime-required'
    ).length,
    productionRuntimeRequiredRowCount: handoffRows.filter(
      (row) => row.readinessCategory === 'production-runtime-required'
    ).length,
    ciRunnableRowCount: 0,
    productReadyRowCount: 0,
  };
}
