import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema,
  NonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/family-domain/reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

export const TrackingPhysicalDeviceArtifactPlatformSchema = Schema.Literal('android', 'ios');

export const TrackingPhysicalDeviceArtifactGateStatusSchema = Schema.Literal('manual-required', 'artifact-set-present');

export const TrackingPhysicalDeviceArtifactPathSchema = brandedNonEmptyStringSchema('TrackingPhysicalDeviceArtifactPath');

export const TrackingPhysicalDeviceArtifactGateRowIdSchema = brandedNonEmptyStringSchema('TrackingPhysicalDeviceArtifactGateRowId');

export const TrackingPhysicalDeviceArtifactGateRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingPhysicalDeviceArtifactGateRowIdSchema,
    generatedAt: ParentTimestampSchema,
    platform: TrackingPhysicalDeviceArtifactPlatformSchema,
    proofRoot: TrackingPhysicalDeviceArtifactPathSchema,
    requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingPhysicalDeviceArtifactGateStatusSchema,
    requiredArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    presentArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    missingArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    supportingStatusProofRef: TrackingPhysicalDeviceArtifactPathSchema,
    supportingStatusArtifacts: Schema.Array(TrackingPhysicalDeviceArtifactPathSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    acceptanceCriteria: Schema.Array(NonEmptyStringSchema),
    manualValidationCommands: Schema.Array(NonEmptyStringSchema),
    artifactAcceptanceNotes: Schema.Array(NonEmptyStringSchema),
    physicalArtifactSetComplete: Schema.Boolean,
    physicalDeviceStatusObserved: Schema.Boolean,
    physicalDeviceBehaviorClaimed: Schema.Literal(false),
    authorityEnrollmentClaimed: Schema.Literal(false),
    providerDeliveryClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.requiredArtifacts.length > 0 || 'Physical proof rows need artifacts'))
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Physical proof rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.status === 'artifact-set-present') === row.physicalArtifactSetComplete ||
          'Physical artifact set status must match completeness'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          (row.physicalArtifactSetComplete ? row.missingArtifacts.length === 0 : row.missingArtifacts.length > 0) ||
          'Physical artifact completeness must match missing artifact count'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.physicalDeviceStatusObserved === row.supportingStatusArtifacts.length > 0 ||
          'Physical device status observation must match supporting status artifacts'
      )
    )
    .pipe(Schema.filter((row) => row.acceptanceCriteria.length >= 4 || 'Physical proof rows need criteria'))
    .pipe(
      Schema.filter((row) => row.manualValidationCommands.length >= 4 || 'Physical proof rows need manual commands')
    )
    .pipe(
      Schema.filter((row) => row.artifactAcceptanceNotes.length >= 4 || 'Physical proof rows need acceptance notes')
    )
);

export const TrackingPhysicalDeviceArtifactGateProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-physical-device-artifact-gate-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingPhysicalDeviceArtifactGateRowSchema),
    proofClaims: Schema.Struct({
      androidPhysicalArtifactGateChecked: Schema.Literal(true),
      iosPhysicalArtifactGateChecked: Schema.Literal(true),
      noPhysicalDeviceBehaviorClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      physicalDeviceBehaviorClaimed: Schema.Literal(false),
      authorityEnrollmentClaimed: Schema.Literal(false),
      providerDeliveryClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length === RequiredTrackingPhysicalDeviceArtifactPlans.length &&
          proof.rows.some((row) => row.platform === 'android') &&
          proof.rows.some((row) => row.platform === 'ios')) ||
        'Physical device artifact gate must cover Android and iOS'
    )
  )
);

export type TrackingPhysicalDeviceArtifactGateProof = Infer<typeof TrackingPhysicalDeviceArtifactGateProofSchema>;
export type TrackingPhysicalDeviceArtifactGateRow = Infer<typeof TrackingPhysicalDeviceArtifactGateRowSchema>;

export interface TrackingPhysicalDeviceArtifactInventory {
  readonly platform: (typeof RequiredTrackingPhysicalDeviceArtifactPlans)[number]['platform'];
  readonly presentArtifacts: readonly string[];
  readonly supportingStatusProofRef?: string;
  readonly supportingStatusArtifacts?: readonly string[];
}

export const RequiredTrackingPhysicalDeviceArtifactPlans = [
  {
    platform: 'android',
    proofRoot: 'output/tracking-plan-proof/android-background-geofence',
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
    acceptanceCriteria: [
      'Record a real child Android device run, not an emulator-only run.',
      'Capture foreground and background permission state before and after the geofence scenario.',
      'Show system geofence or background delivery evidence with timestamps, not only app-owned local listener rows.',
      'Keep product-ready false until parent-visible UI, authority, provider, and production rows are separately complete.',
    ],
    manualValidationCommands: [
      'adb devices -l',
      'adb shell dumpsys package com.ocentra.parent.child | findstr ACCESS_BACKGROUND_LOCATION',
      'adb logcat -d | findstr OcentraTracking',
      'node scripts/test/tracking-physical-device-artifact-gate-proof.mjs',
    ],
  },
  {
    platform: 'ios',
    proofRoot: 'output/tracking-plan-proof/ios-region-monitoring',
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
    acceptanceCriteria: [
      'Record a real child iOS device run, not simulator-only proof.',
      'Capture Core Location authorization, Always/background state, and region monitoring registration evidence.',
      'Show region/significant-change/visit delivery evidence with timestamps and parent alert decision refs.',
      'Keep product-ready false until entitlement/review, authority, provider, and production rows are separately complete.',
    ],
    manualValidationCommands: [
      'xcrun xctrace list devices',
      'xcodebuild test -scheme OcentraParentChildTracking -destination id=<physical-device-udid>',
      'xcrun simctl is not accepted for this row; attach physical-device logs instead',
      'node scripts/test/tracking-physical-device-artifact-gate-proof.mjs',
    ],
  },
] as const;

export function buildTrackingPhysicalDeviceArtifactGateProof(
  generatedAt: string,
  inventories: readonly TrackingPhysicalDeviceArtifactInventory[]
): TrackingPhysicalDeviceArtifactGateProof {
  return TrackingPhysicalDeviceArtifactGateProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-physical-device-artifact-gate-proof',
    generatedAt,
    rows: RequiredTrackingPhysicalDeviceArtifactPlans.map((plan) =>
      physicalArtifactRow(generatedAt, plan, inventories)
    ),
    proofClaims: {
      androidPhysicalArtifactGateChecked: true,
      iosPhysicalArtifactGateChecked: true,
      noPhysicalDeviceBehaviorClaim: true,
      noAuthorityClaim: true,
      noProviderDeliveryClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      physicalDeviceBehaviorClaimed: false,
      authorityEnrollmentClaimed: false,
      providerDeliveryClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function physicalArtifactRow(
  generatedAt: string,
  plan: (typeof RequiredTrackingPhysicalDeviceArtifactPlans)[number],
  inventories: readonly TrackingPhysicalDeviceArtifactInventory[]
): TrackingPhysicalDeviceArtifactGateRow {
  const inventory = inventories.find((candidate) => candidate.platform === plan.platform);
  const presentArtifactSet = new Set(inventory?.presentArtifacts ?? []);
  const presentArtifacts = plan.requiredArtifacts.filter((artifact) => presentArtifactSet.has(artifact));
  const missingArtifacts = plan.requiredArtifacts.filter((artifact) => !presentArtifactSet.has(artifact));
  const supportingStatusArtifacts = [...(inventory?.supportingStatusArtifacts ?? [])];
  const physicalArtifactSetComplete = missingArtifacts.length === 0;

  return TrackingPhysicalDeviceArtifactGateRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: `tracking-physical-device-artifacts-${plan.platform}`,
    generatedAt,
    platform: plan.platform,
    proofRoot: plan.proofRoot,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: physicalArtifactSetComplete ? 'artifact-set-present' : 'manual-required',
    requiredArtifacts: [...plan.requiredArtifacts],
    presentArtifacts,
    missingArtifacts,
    supportingStatusProofRef: inventory?.supportingStatusProofRef ?? `${plan.proofRoot}/status-support-not-collected`,
    supportingStatusArtifacts,
    auditRefs: [`tracking-physical-device-artifacts-${plan.platform}-audit`],
    acceptanceCriteria: [...plan.acceptanceCriteria],
    manualValidationCommands: [...plan.manualValidationCommands],
    artifactAcceptanceNotes: artifactAcceptanceNotesFor(plan.platform, plan.requiredArtifacts.length),
    physicalArtifactSetComplete,
    physicalDeviceStatusObserved: supportingStatusArtifacts.length > 0,
    physicalDeviceBehaviorClaimed: false,
    authorityEnrollmentClaimed: false,
    providerDeliveryClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}

function artifactAcceptanceNotesFor(
  platform: TrackingPhysicalDeviceArtifactInventory['platform'],
  artifactCount: number
) {
  return [
    `Required ${platform} physical artifacts: ${artifactCount}.`,
    'Artifact-set-present only means the required files exist in the proof root.',
    'Physical-device behavior remains unclaimed until a reviewer accepts the artifact contents.',
    'Product claims stay false for authority, provider delivery, production runtime, and product readiness in this gate.',
  ];
}

