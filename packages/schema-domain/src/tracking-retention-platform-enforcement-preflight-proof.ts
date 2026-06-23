import { type Infer, Schema, withParser, brandedNonEmptyStringSchema } from './effect';
import { ParentTimestampSchema } from './family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import {
  RequiredTrackingRetentionRuntimeArtifactPlan,
  type TrackingRetentionRuntimeArtifactGateProof,
} from './tracking-retention-runtime-artifact-gate-proof';

export const TrackingRetentionPlatformEnforcementPreflightPlatformSchema = Schema.Literal(
  'android-device-policy',
  'ios-family-controls',
  'desktop-managed-policy'
);

export const TrackingRetentionPlatformEnforcementPreflightStatusSchema = Schema.Literal('manual-required');

export const TrackingRetentionPlatformEnforcementPreflightRowIdSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionPlatformEnforcementPreflightRowId'
);

export const TrackingRetentionPlatformEnforcementPreflightPathSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionPlatformEnforcementPreflightPath'
);

export const TrackingRetentionPlatformEnforcementPreflightArtifactRefSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionPlatformEnforcementPreflightArtifactRef'
);

export const TrackingRetentionPlatformEnforcementPreflightCommandSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionPlatformEnforcementPreflightCommand'
);

export const TrackingRetentionPlatformEnforcementPreflightCriterionSchema = brandedNonEmptyStringSchema(
  'TrackingRetentionPlatformEnforcementPreflightCriterion'
);

export const RequiredTrackingRetentionPlatformEnforcementArtifactRef =
  'tracking-retention/platform-runtime-retention-enforcement.json';

const RequiredTrackingRetentionPlatformEnforcementRows = [
  {
    rowId: 'tracking-retention-platform-enforcement-android-device-policy',
    platform: 'android-device-policy',
    acceptanceCriteria: [
      'Android device-owner or profile-owner retention policy write is observed on an enrolled child device.',
      'A retained location or geofence event remains queryable after the configured local retention boundary.',
      'The artifact includes the command, policy target, retained record id, timestamp, and platform status result.',
    ],
    manualValidationCommands: [
      'cmd /c npm run android:device:retention -- --enrolled-child --capture-artifact tracking-retention/platform-runtime-retention-enforcement.json',
      'adb shell dumpsys device_policy',
      'adb shell dumpsys activity service ocentra',
    ],
    requiredArtifacts: [
      'tracking-retention/platform-runtime-retention-enforcement/android-device-policy-write.json',
      'tracking-retention/platform-runtime-retention-enforcement/android-retained-record-observation.json',
    ],
    artifactAcceptanceNotes: [
      'Artifact must come from a real enrolled Android child device, not emulator-only replay.',
      'Artifact must prove retention enforcement behavior, not only package launch or local JSON generation.',
    ],
    auditRefs: ['tracking-retention-platform-enforcement-android-audit'],
  },
  {
    rowId: 'tracking-retention-platform-enforcement-ios-family-controls',
    platform: 'ios-family-controls',
    acceptanceCriteria: [
      'iOS Screen Time or Family Controls retention path is exercised on an entitled child device.',
      'A retained location or geofence event remains queryable after the configured local retention boundary.',
      'The artifact includes entitlement state, policy target, retained record id, timestamp, and platform status result.',
    ],
    manualValidationCommands: [
      'xcrun simctl list devices',
      'xcodebuild -scheme OcentraParentChild -destination generic/platform=iOS archive',
      'manual: capture entitled iOS child-device retention artifact tracking-retention/platform-runtime-retention-enforcement.json',
    ],
    requiredArtifacts: [
      'tracking-retention/platform-runtime-retention-enforcement/ios-family-controls-profile.json',
      'tracking-retention/platform-runtime-retention-enforcement/ios-retained-record-observation.json',
    ],
    artifactAcceptanceNotes: [
      'Artifact must come from an entitled iOS child device when credentials are available.',
      'Simulator package launch is not enough for retention enforcement product readiness.',
    ],
    auditRefs: ['tracking-retention-platform-enforcement-ios-audit'],
  },
  {
    rowId: 'tracking-retention-platform-enforcement-desktop-managed-policy',
    platform: 'desktop-managed-policy',
    acceptanceCriteria: [
      'Desktop managed-policy retention write is exercised through the real local service path.',
      'A retained location or geofence record remains queryable after the configured local retention boundary.',
      'The artifact includes the command, storage target, retained record id, timestamp, and service status result.',
    ],
    manualValidationCommands: [
      'cmd /c npm run dev:agent',
      'cmd /c npm run test:local -- --tracking-retention-platform-enforcement',
      'manual: capture desktop managed-policy retention artifact tracking-retention/platform-runtime-retention-enforcement.json',
    ],
    requiredArtifacts: [
      'tracking-retention/platform-runtime-retention-enforcement/desktop-managed-policy-write.json',
      'tracking-retention/platform-runtime-retention-enforcement/desktop-retained-record-observation.json',
    ],
    artifactAcceptanceNotes: [
      'Artifact must prove service-backed retention enforcement, not only domain schema acceptance.',
      'Artifact must remain separate from Android and iOS child-device authority claims.',
    ],
    auditRefs: ['tracking-retention-platform-enforcement-desktop-audit'],
  },
] as const;

export const TrackingRetentionPlatformEnforcementPreflightRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingRetentionPlatformEnforcementPreflightRowIdSchema,
    generatedAt: ParentTimestampSchema,
    platform: TrackingRetentionPlatformEnforcementPreflightPlatformSchema,
    requiredProofTier: Schema.Literal('P4_PRODUCTION_RUNTIME'),
    currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
    status: TrackingRetentionPlatformEnforcementPreflightStatusSchema,
    sourceRuntimeArtifactGateProofRef: TrackingRetentionPlatformEnforcementPreflightPathSchema,
    sourceMissingArtifactRef: TrackingRetentionPlatformEnforcementPreflightArtifactRefSchema,
    acceptanceCriteria: Schema.Array(TrackingRetentionPlatformEnforcementPreflightCriterionSchema),
    manualValidationCommands: Schema.Array(TrackingRetentionPlatformEnforcementPreflightCommandSchema),
    requiredArtifacts: Schema.Array(TrackingRetentionPlatformEnforcementPreflightArtifactRefSchema),
    presentArtifacts: Schema.Array(TrackingRetentionPlatformEnforcementPreflightArtifactRefSchema),
    missingArtifacts: Schema.Array(TrackingRetentionPlatformEnforcementPreflightArtifactRefSchema),
    artifactAcceptanceNotes: Schema.Array(TrackingRetentionPlatformEnforcementPreflightCriterionSchema),
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
    writableProductSettingsExecutionClaimed: Schema.Literal(false),
    childDeviceDeliveryClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
    productionWorkerClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) => row.acceptanceCriteria.length >= 3 || 'Retention platform preflight rows need acceptance criteria'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.manualValidationCommands.length >= 2 || 'Retention platform preflight rows need validation commands'
      )
    )
    .pipe(
      Schema.filter(
        (row) => row.requiredArtifacts.length > 0 || 'Retention platform preflight rows need required artifacts'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length ||
          'Retention platform preflight rows must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.missingArtifacts.length > 0 ||
          'Retention platform preflight rows are manual-required until platform artifacts exist'
      )
    )
);

export const TrackingRetentionPlatformEnforcementPreflightProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-retention-platform-enforcement-preflight-proof'),
    generatedAt: ParentTimestampSchema,
    sourceRuntimeArtifactGateProofRef: TrackingRetentionPlatformEnforcementPreflightPathSchema,
    sourceMissingArtifactRef: TrackingRetentionPlatformEnforcementPreflightArtifactRefSchema,
    rows: Schema.Array(TrackingRetentionPlatformEnforcementPreflightRowSchema),
    summary: Schema.Struct({
      rowCount: Schema.Number,
      manualRequiredRowCount: Schema.Number,
      requiredArtifactCount: Schema.Number,
      presentArtifactCount: Schema.Number,
      missingArtifactCount: Schema.Number,
      productReadyRowCount: Schema.Number,
    }),
    proofClaims: Schema.Struct({
      platformEnforcementPreflightGenerated: Schema.Literal(true),
      runtimeArtifactGateObserved: Schema.Literal(true),
      platformRuntimeArtifactStillMissing: Schema.Literal(true),
      noPlatformRuntimeRetentionEnforcementClaim: Schema.Literal(true),
      noWritableProductSettingsExecutionClaim: Schema.Literal(true),
      noChildDeviceDeliveryClaim: Schema.Literal(true),
      noPhysicalDeviceProofClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductionWorkerClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      platformRuntimeRetentionEnforcementClaimed: Schema.Literal(false),
      writableProductSettingsExecutionClaimed: Schema.Literal(false),
      childDeviceDeliveryClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionWorkerClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.rows.length === RequiredTrackingRetentionPlatformEnforcementRows.length ||
          'Retention platform preflight must cover every platform row'
      )
    )
    .pipe(
      Schema.filter(
        (proof) => proof.summary.rowCount === proof.rows.length || 'Retention platform preflight summary row mismatch'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.productReadyRowCount === 0 || 'Retention platform preflight cannot include product-ready rows'
      )
    )
);

export type TrackingRetentionPlatformEnforcementPreflightProof = Infer<
  typeof TrackingRetentionPlatformEnforcementPreflightProofSchema
>;
export type TrackingRetentionPlatformEnforcementPreflightRow = Infer<
  typeof TrackingRetentionPlatformEnforcementPreflightRowSchema
>;

export const RequiredTrackingRetentionPlatformEnforcementPreflightPlan = {
  sourceRuntimeArtifactGateProofRef: 'test-results/tracking-retention-runtime-artifact-gate-proof/proof.json',
  sourceMissingArtifactRef: RequiredTrackingRetentionPlatformEnforcementArtifactRef,
  rows: RequiredTrackingRetentionPlatformEnforcementRows,
} as const;

export function buildTrackingRetentionPlatformEnforcementPreflightProof(
  generatedAt: string,
  runtimeArtifactGateProof: TrackingRetentionRuntimeArtifactGateProof
): TrackingRetentionPlatformEnforcementPreflightProof {
  assertRuntimeGateStillRequiresPlatformEnforcement(runtimeArtifactGateProof);
  const rows = RequiredTrackingRetentionPlatformEnforcementRows.map((row) => preflightRow(generatedAt, row));

  return TrackingRetentionPlatformEnforcementPreflightProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-retention-platform-enforcement-preflight-proof',
    generatedAt,
    sourceRuntimeArtifactGateProofRef:
      RequiredTrackingRetentionPlatformEnforcementPreflightPlan.sourceRuntimeArtifactGateProofRef,
    sourceMissingArtifactRef: RequiredTrackingRetentionPlatformEnforcementPreflightPlan.sourceMissingArtifactRef,
    rows,
    summary: {
      rowCount: rows.length,
      manualRequiredRowCount: rows.filter((row) => row.status === 'manual-required').length,
      requiredArtifactCount: rows.reduce((total, row) => total + row.requiredArtifacts.length, 0),
      presentArtifactCount: rows.reduce((total, row) => total + row.presentArtifacts.length, 0),
      missingArtifactCount: rows.reduce((total, row) => total + row.missingArtifacts.length, 0),
      productReadyRowCount: rows.filter((row) => row.productClaimReady).length,
    },
    proofClaims: {
      platformEnforcementPreflightGenerated: true,
      runtimeArtifactGateObserved: true,
      platformRuntimeArtifactStillMissing: true,
      noPlatformRuntimeRetentionEnforcementClaim: true,
      noWritableProductSettingsExecutionClaim: true,
      noChildDeviceDeliveryClaim: true,
      noPhysicalDeviceProofClaim: true,
      noAuthorityClaim: true,
      noProductionWorkerClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      platformRuntimeRetentionEnforcementClaimed: false,
      writableProductSettingsExecutionClaimed: false,
      childDeviceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionWorkerClaimed: false,
      productClaimReady: false,
    },
  });
}

function assertRuntimeGateStillRequiresPlatformEnforcement(
  runtimeArtifactGateProof: TrackingRetentionRuntimeArtifactGateProof
): void {
  const runtimeGateRow = runtimeArtifactGateProof.rows.find(
    (row) => row.proofRoot === RequiredTrackingRetentionRuntimeArtifactPlan.proofRoot
  );
  if (!runtimeGateRow) {
    throw new Error('Retention runtime artifact gate proof is missing the retention proof-root row.');
  }
  if (
    !runtimeGateRow.missingArtifacts.some(
      (artifact) => artifact === RequiredTrackingRetentionPlatformEnforcementArtifactRef
    )
  ) {
    throw new Error('Retention platform enforcement preflight requires the platform runtime artifact to be missing.');
  }
  if (runtimeArtifactGateProof.productClaims.productClaimReady) {
    throw new Error('Retention platform enforcement preflight cannot run against product-ready runtime claims.');
  }
}

function preflightRow(
  generatedAt: string,
  row: (typeof RequiredTrackingRetentionPlatformEnforcementRows)[number]
): TrackingRetentionPlatformEnforcementPreflightRow {
  return TrackingRetentionPlatformEnforcementPreflightRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: row.rowId,
    generatedAt,
    platform: row.platform,
    requiredProofTier: 'P4_PRODUCTION_RUNTIME',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'manual-required',
    sourceRuntimeArtifactGateProofRef:
      RequiredTrackingRetentionPlatformEnforcementPreflightPlan.sourceRuntimeArtifactGateProofRef,
    sourceMissingArtifactRef: RequiredTrackingRetentionPlatformEnforcementPreflightPlan.sourceMissingArtifactRef,
    acceptanceCriteria: [...row.acceptanceCriteria],
    manualValidationCommands: [...row.manualValidationCommands],
    requiredArtifacts: [...row.requiredArtifacts],
    presentArtifacts: [],
    missingArtifacts: [...row.requiredArtifacts],
    artifactAcceptanceNotes: [...row.artifactAcceptanceNotes],
    auditRefs: [...row.auditRefs],
    platformRuntimeRetentionEnforcementClaimed: false,
    writableProductSettingsExecutionClaimed: false,
    childDeviceDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionWorkerClaimed: false,
    productClaimReady: false,
  });
}
