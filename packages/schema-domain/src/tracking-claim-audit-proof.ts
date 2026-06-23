import { type Infer, Schema, withParser, brandedNonEmptyStringSchema, NonEmptyStringSchema } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

export const TrackingClaimAuditAreaSchema = Schema.Literal(
  'android-physical-background-and-geofence',
  'ios-physical-background-and-region',
  'manual-desktop-location',
  'retention-product-settings-writable-runtime',
  'child-device-runtime-execution',
  'full-product-parent-child-ui-runtime',
  'authority-enrolled-hard-control-runtime',
  'provider-delivery-receipt-runtime',
  'retention-product-runtime-enforcement',
  'production-durable-workers-and-storage',
  'escalation-runtime-workers-and-storage'
);

export const TrackingClaimAuditStatusSchema = Schema.Literal('manual-required', 'artifact-set-present-review-required');

export const TrackingClaimAuditPathSchema = brandedNonEmptyStringSchema('TrackingClaimAuditPath');
export const TrackingClaimAuditRowIdSchema = brandedNonEmptyStringSchema('TrackingClaimAuditRowId');

export const TrackingClaimAuditRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingClaimAuditRowIdSchema,
    generatedAt: ParentTimestampSchema,
    auditArea: TrackingClaimAuditAreaSchema,
    sourceProofRef: TrackingClaimAuditPathSchema,
    supportingProofRefs: Schema.Array(TrackingClaimAuditPathSchema),
    proofRoot: TrackingClaimAuditPathSchema,
    requiredProofTier: NonEmptyStringSchema,
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingClaimAuditStatusSchema,
    acceptanceCriteria: Schema.Array(NonEmptyStringSchema),
    manualValidationCommands: Schema.Array(NonEmptyStringSchema),
    artifactAcceptanceNotes: Schema.Array(NonEmptyStringSchema),
    requiredArtifacts: Schema.Array(TrackingClaimAuditPathSchema),
    presentArtifacts: Schema.Array(TrackingClaimAuditPathSchema),
    missingArtifacts: Schema.Array(TrackingClaimAuditPathSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    artifactSetComplete: Schema.Boolean,
    claimApproved: Schema.Literal(false),
    physicalDeviceBehaviorClaimed: Schema.Literal(false),
    manualDesktopLocationClaimed: Schema.Literal(false),
    childDeviceRuntimeClaimed: Schema.Literal(false),
    fullProductUiClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    retentionProductRuntimeClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    escalationRuntimeClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Claim audit rows need artifacts'))
    .pipe(Schema.filter((row) => row.supportingProofRefs.length > 0 || 'Claim audit rows need proof refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Claim audit rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.artifactSetComplete ? row.missingArtifacts.length === 0 : row.missingArtifacts.length > 0) ||
          'Claim audit completeness must match missing artifact count'
      )
    )
);

export const TrackingClaimAuditProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-claim-audit-proof'),
    generatedAt: ParentTimestampSchema,
    requiredProofTier: Schema.Literal('P4_REAL_RUNTIME_HANDOFF'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    currentStatus: Schema.Literal('manual_required'),
    rows: Schema.Array(TrackingClaimAuditRowSchema),
    summary: Schema.Struct({
      rowCount: Schema.Number,
      requiredArtifactCount: Schema.Number,
      presentArtifactCount: Schema.Number,
      missingArtifactCount: Schema.Number,
      manualRequiredRowCount: Schema.Number,
      artifactSetPresentReviewRequiredRowCount: Schema.Number,
      physicalDeviceRequiredRowCount: Schema.Number,
      approvedManualRequiredRowCount: Schema.Number,
      manualProviderRuntimeRequiredRowCount: Schema.Number,
      productionRuntimeRequiredRowCount: Schema.Number,
      acceptanceCriteriaCount: Schema.Number,
      manualValidationCommandCount: Schema.Number,
      artifactAcceptanceNoteCount: Schema.Number,
      approvedClaimCount: Schema.Literal(0),
      productReadyRowCount: Schema.Literal(0),
    }),
    productClaims: Schema.Struct({
      physicalDeviceBehaviorClaimed: Schema.Literal(false),
      manualDesktopLocationClaimed: Schema.Literal(false),
      childDeviceRuntimeClaimed: Schema.Literal(false),
      fullProductUiClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      retentionProductRuntimeClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      escalationRuntimeClaimed: Schema.Literal(false),
      productReadyClaimed: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.rows.length === RequiredTrackingClaimAuditPlans.length ||
        'Tracking claim audit proof must cover every required claim area'
    )
  )
);

export type TrackingClaimAuditProof = Infer<typeof TrackingClaimAuditProofSchema>;
export type TrackingClaimAuditRow = Infer<typeof TrackingClaimAuditRowSchema>;

export interface TrackingClaimAuditInventory {
  readonly auditArea: (typeof RequiredTrackingClaimAuditPlans)[number]['auditArea'];
  readonly presentArtifacts: readonly string[];
}

export const RequiredTrackingClaimAuditPlans = [
  {
    auditArea: 'android-physical-background-and-geofence',
    sourceProofRef: 'test-results/tracking-physical-device-evidence-review-proof/proof.json',
    supportingProofRefs: [
      'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
      'test-results/tracking-physical-device-evidence-review-proof/proof.json',
    ],
    proofRoot: 'output/tracking-plan-proof/android-background-geofence',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: [
      '00-run-metadata.json',
      '01-device-metadata.json',
      '02-permission-state.json',
      '03-geofence-definition.json',
      '04-location-events.ndjson',
      '05-geofence-transitions.ndjson',
      '06-alert-decision.json',
      '07-parent-ui-screenshot.png',
      '08-logcat.txt',
      '09-result-summary.md',
    ],
  },
  {
    auditArea: 'ios-physical-background-and-region',
    sourceProofRef: 'test-results/tracking-physical-device-evidence-review-proof/proof.json',
    supportingProofRefs: [
      'test-results/tracking-physical-device-artifact-gate-proof/proof.json',
      'test-results/tracking-physical-device-evidence-review-proof/proof.json',
    ],
    proofRoot: 'output/tracking-plan-proof/ios-region-monitoring',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: [
      '00-run-metadata.json',
      '01-device-metadata.json',
      '02-authorization-state.json',
      '03-region-definition.json',
      '04-location-events.ndjson',
      '05-region-transitions.ndjson',
      '06-alert-decision.json',
      '07-screenshots',
      '08-xcode-test-log.txt',
      '09-result-summary.md',
    ],
  },
  {
    auditArea: 'manual-desktop-location',
    sourceProofRef:
      'output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing/20-platform-extension-inventory-proof.json',
    proofRoot: 'output/tracking-plan-proof/manual-desktop-location',
    requiredProofTier: 'P4_APPROVED_MANUAL_PROOF',
    requiredArtifacts: [
      '00-run-metadata.json',
      '01-desktop-location-provider-approval.json',
      '02-desktop-location-runtime-observation.json',
      '03-parent-ui-screenshot.png',
      '04-result-summary.md',
    ],
  },
  {
    auditArea: 'retention-product-settings-writable-runtime',
    sourceProofRef: 'test-results/tracking-full-product-ui-runtime-preflight-proof/proof.json',
    supportingProofRefs: [
      'test-results/tracking-retention-product-settings-writable-execution-proof/proof.json',
      'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
      'test-results/tracking-full-product-ui-runtime-preflight-proof/proof.json',
    ],
    proofRoot: 'output/tracking-plan-proof/product-parent-child-ui-runtime',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: ['04-retention-settings-production-write-result.png'],
  },
  {
    auditArea: 'child-device-runtime-execution',
    sourceProofRef: 'test-results/tracking-child-runtime-artifact-gate-proof/proof.json',
    proofRoot: 'output/tracking-plan-proof/child-runtime-delivery',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: [
      '00-run-metadata.json',
      '01-child-device-metadata.json',
      '02-delivery-envelope.json',
      '03-execution-result.json',
      '04-visible-child-runtime-snapshot.png',
      '05-parent-receipt.json',
      '06-runtime-observation-log.txt',
    ],
  },
  {
    auditArea: 'full-product-parent-child-ui-runtime',
    sourceProofRef: 'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
    supportingProofRefs: [
      'test-results/tracking-full-product-ui-runtime-artifact-gate-proof/proof.json',
      'test-results/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json',
      'output/tracking-plan-proof/tracking-full-product-ui-local-runtime-artifact-capture-proof/proof.json',
    ],
    proofRoot: 'output/tracking-plan-proof/product-parent-child-ui-runtime',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: [
      '01-parent-overview-runtime.png',
      '02-parent-device-detail-runtime.png',
      '03-parent-notification-history-preferences-runtime.png',
      '04-retention-settings-production-write-result.png',
      '05-child-device-rendered-check-in-runtime.png',
      '06-child-device-rendered-location-consent-runtime.png',
      '07-child-device-safe-help-response-runtime.png',
      '08-cross-surface-accessibility-report.json',
      '09-product-ui-end-to-end-trace.json',
    ],
  },
  {
    auditArea: 'authority-enrolled-hard-control-runtime',
    sourceProofRef: 'test-results/tracking-authority-runtime-artifact-gate-proof/proof.json',
    proofRoot: 'output/tracking-plan-proof/authority-runtime',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: [
      '01-android-device-owner-enrollment.json',
      '02-android-managed-profile-enrollment.json',
      '03-ios-family-controls-entitlement.json',
      '04-ios-app-review-approval.json',
      '05-desktop-managed-policy-proof.json',
      '06-parent-visible-authority-status.png',
    ],
  },
  {
    auditArea: 'provider-delivery-receipt-runtime',
    sourceProofRef: 'test-results/tracking-provider-delivery-artifact-gate-proof/proof.json',
    proofRoot: 'output/tracking-plan-proof/provider-delivery-runtime',
    requiredProofTier: 'P4_MANUAL_PROVIDER_RUNTIME',
    requiredArtifacts: [
      '01-provider-send-request.json',
      '02-provider-response.json',
      '03-delivery-receipt-webhook.json',
      '04-parent-visible-receipt.png',
      '05-provider-audit-log.txt',
    ],
  },
  {
    auditArea: 'retention-product-runtime-enforcement',
    sourceProofRef: 'test-results/tracking-retention-runtime-artifact-gate-proof/proof.json',
    proofRoot: 'output/tracking-plan-proof/tracking-retention',
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    requiredArtifacts: ['platform-runtime-retention-enforcement.json'],
  },
  {
    auditArea: 'production-durable-workers-and-storage',
    sourceProofRef: 'test-results/tracking-production-worker-runtime-artifact-gate-proof/proof.json',
    proofRoot: 'output/tracking-plan-proof/tracking-production',
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    requiredArtifacts: [
      'location-upload-worker-runtime.json',
      'retention-cleanup-worker-runtime.json',
      'notification-outbox-worker-runtime.json',
      'escalation-timeout-worker-runtime.json',
      'provider-receipt-worker-runtime.json',
      'child-device-delivery-worker-runtime.json',
      'authority-status-worker-runtime.json',
      'audit-durable-storage-runtime.json',
    ],
  },
  {
    auditArea: 'escalation-runtime-workers-and-storage',
    sourceProofRef: 'test-results/tracking-escalation-runtime-artifact-gate-proof/proof.json',
    proofRoot: 'output/tracking-plan-proof/tracking-escalation-runtime',
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    requiredArtifacts: [
      'quiet-hours-timer-runtime.json',
      'provider-retry-worker-runtime.json',
      'parent-notification-history-runtime.json',
      'durable-escalation-storage-runtime.json',
      'emergency-auto-contact-policy-runtime.json',
    ],
  },
] as const;

export function buildTrackingClaimAuditProof(
  generatedAt: string,
  inventories: readonly TrackingClaimAuditInventory[]
): TrackingClaimAuditProof {
  const rows = RequiredTrackingClaimAuditPlans.map((plan) => claimAuditRow(generatedAt, plan, inventories));

  return TrackingClaimAuditProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-claim-audit-proof',
    generatedAt,
    requiredProofTier: 'P4_REAL_RUNTIME_HANDOFF',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    currentStatus: 'manual_required',
    rows,
    summary: summarizeRows(rows),
    productClaims: {
      physicalDeviceBehaviorClaimed: false,
      manualDesktopLocationClaimed: false,
      childDeviceRuntimeClaimed: false,
      fullProductUiClaimed: false,
      authorityClaimed: false,
      providerDeliveryClaimed: false,
      retentionProductRuntimeClaimed: false,
      productionWorkerClaimed: false,
      escalationRuntimeClaimed: false,
      productReadyClaimed: false,
    },
  });
}

function claimAuditRow(
  generatedAt: string,
  plan: (typeof RequiredTrackingClaimAuditPlans)[number],
  inventories: readonly TrackingClaimAuditInventory[]
): TrackingClaimAuditRow {
  const inventory = inventories.find((candidate) => candidate.auditArea === plan.auditArea);
  const presentArtifactSet = new Set(inventory?.presentArtifacts ?? []);
  const presentArtifacts = plan.requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = plan.requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const artifactSetComplete = missingArtifacts.length === 0;

  return TrackingClaimAuditRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-claim-audit-${plan.auditArea}`,
    generatedAt,
    auditArea: plan.auditArea,
    sourceProofRef: plan.sourceProofRef,
    supportingProofRefs: supportingProofRefsForPlan(plan),
    proofRoot: plan.proofRoot,
    requiredProofTier: plan.requiredProofTier,
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: artifactSetComplete ? 'artifact-set-present-review-required' : 'manual-required',
    acceptanceCriteria: acceptanceCriteriaForPlan(plan),
    manualValidationCommands: manualValidationCommandsForPlan(),
    artifactAcceptanceNotes: artifactAcceptanceNotesForPlan(plan),
    requiredArtifacts: [...plan.requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    auditRefs: [`tracking-claim-audit-${plan.auditArea}-audit`],
    artifactSetComplete,
    claimApproved: false,
    physicalDeviceBehaviorClaimed: false,
    manualDesktopLocationClaimed: false,
    childDeviceRuntimeClaimed: false,
    fullProductUiClaimed: false,
    authorityClaimed: false,
    providerDeliveryClaimed: false,
    retentionProductRuntimeClaimed: false,
    productionWorkerClaimed: false,
    escalationRuntimeClaimed: false,
    productClaimReady: false,
  });
}

function supportingProofRefsForPlan(plan: (typeof RequiredTrackingClaimAuditPlans)[number]): readonly string[] {
  return 'supportingProofRefs' in plan ? plan.supportingProofRefs : [plan.sourceProofRef];
}

function acceptanceCriteriaForPlan(plan: (typeof RequiredTrackingClaimAuditPlans)[number]): readonly string[] {
  return [
    `Collect every required artifact under ${plan.proofRoot} before review.`,
    `Keep required proof tier ${plan.requiredProofTier}; local P3 artifacts cannot approve the claim.`,
    `Cite source proof ${plan.sourceProofRef} and all supporting proof refs in the final handoff.`,
    acceptanceCriterionForProofTier(plan.requiredProofTier),
  ];
}

function acceptanceCriterionForProofTier(
  requiredProofTier: (typeof RequiredTrackingClaimAuditPlans)[number]['requiredProofTier']
) {
  if (requiredProofTier === 'P4_PHYSICAL_DEVICE') {
    return 'Use real device or enrolled child runtime evidence with metadata, logs, screenshots, and transition or execution rows.';
  }
  if (requiredProofTier === 'P4_APPROVED_MANUAL_PROOF') {
    return 'Use an approved manual provider observation record plus parent-visible screenshot and result summary.';
  }
  if (requiredProofTier === 'P4_MANUAL_PROVIDER_RUNTIME') {
    return 'Use real provider request, response, webhook receipt, parent-visible receipt, and provider audit log artifacts.';
  }
  return 'Use deployed worker/runtime artifacts plus durable storage evidence from the production environment.';
}

function manualValidationCommandsForPlan(): readonly string[] {
  return [
    'node scripts/test/tracking-claim-audit-proof.mjs',
    'node scripts/test/tracking-product-readiness-closure-proof.mjs',
    'node scripts/test/tracking-real-runtime-handoff-proof.mjs',
  ];
}

function artifactAcceptanceNotesForPlan(plan: (typeof RequiredTrackingClaimAuditPlans)[number]): readonly string[] {
  return [
    `Required artifacts: ${plan.requiredArtifacts.length}.`,
    `Proof root: ${plan.proofRoot}.`,
    `Status can move only to review-required when all required artifacts are present; claimApproved remains false here.`,
    `Missing artifacts stay blocking until the ${plan.requiredProofTier} evidence is produced outside local CI.`,
  ];
}

function summarizeRows(rows: readonly TrackingClaimAuditRow[]) {
  return {
    rowCount: rows.length,
    requiredArtifactCount: rows.reduce((count, row) => count + row.requiredArtifacts.length, 0),
    presentArtifactCount: rows.reduce((count, row) => count + row.presentArtifacts.length, 0),
    missingArtifactCount: rows.reduce((count, row) => count + row.missingArtifacts.length, 0),
    manualRequiredRowCount: rows.filter((row) => row.status === 'manual-required').length,
    artifactSetPresentReviewRequiredRowCount: rows.filter(
      (row) => row.status === 'artifact-set-present-review-required'
    ).length,
    physicalDeviceRequiredRowCount: rows.filter((row) => row.requiredProofTier === 'P4_PHYSICAL_DEVICE').length,
    approvedManualRequiredRowCount: rows.filter((row) => row.requiredProofTier === 'P4_APPROVED_MANUAL_PROOF').length,
    manualProviderRuntimeRequiredRowCount: rows.filter((row) => row.requiredProofTier === 'P4_MANUAL_PROVIDER_RUNTIME')
      .length,
    productionRuntimeRequiredRowCount: rows.filter((row) => row.requiredProofTier === 'P4_PRODUCTION_RUNTIME').length,
    acceptanceCriteriaCount: rows.reduce((count, row) => count + row.acceptanceCriteria.length, 0),
    manualValidationCommandCount: rows.reduce((count, row) => count + row.manualValidationCommands.length, 0),
    artifactAcceptanceNoteCount: rows.reduce((count, row) => count + row.artifactAcceptanceNotes.length, 0),
    approvedClaimCount: 0 as const,
    productReadyRowCount: 0 as const,
  };
}
