import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from './reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingAndroidEmulatorArtifactInventoryTextSchema = Schema.String.pipe(Schema.minLength(1));
const TrackingAndroidEmulatorArtifactInventoryCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingAndroidEmulatorArtifactInventoryRefSchema =
  TrackingAndroidEmulatorArtifactInventoryTextSchema.pipe(Schema.brand('TrackingAndroidEmulatorArtifactInventoryRef'));

export const TrackingAndroidEmulatorArtifactInventoryRowIdSchema =
  TrackingAndroidEmulatorArtifactInventoryTextSchema.pipe(
    Schema.brand('TrackingAndroidEmulatorArtifactInventoryRowId')
  );

export const TrackingAndroidEmulatorArtifactInventoryStatusSchema = Schema.Literal(
  'android-emulator-local-artifacts-present-physical-device-required'
);

export const TrackingAndroidEmulatorArtifactInventoryCategorySchema = Schema.Literal(
  'adb-runtime-output',
  'permission-ui',
  'location-runtime',
  'geofence-runtime',
  'device-status',
  'validation-log'
);

export const RequiredTrackingAndroidEmulatorArtifactRefs = [
  'test-results/tracking-plan-android-emulator-proof/proof.json',
  'test-results/tracking-plan-android-emulator-proof/01-adb-install.txt',
  'test-results/tracking-plan-android-emulator-proof/02-resolve-activity.txt',
  'test-results/tracking-plan-android-emulator-proof/03-launch-activity.txt',
  'test-results/tracking-plan-android-emulator-proof/13-foreground-location-permission-ux.json',
  'test-results/tracking-plan-android-emulator-proof/13-foreground-location-permission-ux.xml',
  'test-results/tracking-plan-android-emulator-proof/24-background-location-settings-page.json',
  'output/tracking-plan-proof/08-android-foreground-location-adapter/01-device-metadata.json',
  'output/tracking-plan-proof/08-android-foreground-location-adapter/03-runtime-location-evidence.json',
  'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json',
  'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/04-device-status-proof.json',
  'output/tracking-plan-proof/10-android-battery-connectivity-and-status-adapter/16-validation-commands.log',
] as const;

const TrackingAndroidEmulatorArtifactInventoryArtifactRowSchema = Schema.Struct({
  artifactRef: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  category: TrackingAndroidEmulatorArtifactInventoryCategorySchema,
  required: Schema.Literal(true),
  present: Schema.Boolean,
  byteSize: TrackingAndroidEmulatorArtifactInventoryCountSchema,
});

export const TrackingAndroidEmulatorArtifactInventoryInputSchema = withParser(
  Schema.Struct({
    sourceAndroidEmulatorProofRef: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    androidSdkRoot: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    androidProofStatus: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    packageName: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    activityName: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    deviceSerial: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    androidRelease: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    androidSdk: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    productModel: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    abi: TrackingAndroidEmulatorArtifactInventoryRefSchema,
    foregroundPermissionGranted: Schema.Boolean,
    backgroundPermissionGranted: Schema.Boolean,
    foregroundPermissionUxObserved: Schema.Boolean,
    backgroundSettingsPageObserved: Schema.Boolean,
    packageLaunchObserved: Schema.Boolean,
    foregroundServiceObserved: Schema.Boolean,
    localGeofenceTransitionCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
    localGeofenceDwellCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
    systemProximityRegistered: Schema.Boolean,
    systemProximityTransitionCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
    artifactRows: Schema.Array(TrackingAndroidEmulatorArtifactInventoryArtifactRowSchema).pipe(
      Schema.minItems(RequiredTrackingAndroidEmulatorArtifactRefs.length)
    ),
  })
);

const TrackingAndroidEmulatorArtifactInventoryRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  rowId: TrackingAndroidEmulatorArtifactInventoryRowIdSchema,
  generatedAt: ParentTimestampSchema,
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P3_LOCAL_DEV_MACHINE'),
  status: TrackingAndroidEmulatorArtifactInventoryStatusSchema,
  sourceAndroidEmulatorProofRef: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema).pipe(Schema.minItems(1)),
  androidSdkRoot: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  androidProofStatus: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  packageName: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  activityName: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  deviceSerial: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  androidRelease: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  androidSdk: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  productModel: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  abi: TrackingAndroidEmulatorArtifactInventoryRefSchema,
  requiredArtifacts: Schema.Array(TrackingAndroidEmulatorArtifactInventoryRefSchema).pipe(
    Schema.minItems(RequiredTrackingAndroidEmulatorArtifactRefs.length)
  ),
  presentArtifacts: Schema.Array(TrackingAndroidEmulatorArtifactInventoryRefSchema),
  missingArtifacts: Schema.Array(TrackingAndroidEmulatorArtifactInventoryRefSchema),
  artifactRows: Schema.Array(TrackingAndroidEmulatorArtifactInventoryArtifactRowSchema).pipe(
    Schema.minItems(RequiredTrackingAndroidEmulatorArtifactRefs.length)
  ),
  foregroundPermissionGranted: Schema.Boolean,
  backgroundPermissionGranted: Schema.Boolean,
  foregroundPermissionUxObserved: Schema.Boolean,
  backgroundSettingsPageObserved: Schema.Boolean,
  packageLaunchObserved: Schema.Literal(true),
  foregroundServiceObserved: Schema.Literal(true),
  localGeofenceTransitionCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
  localGeofenceDwellCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
  systemProximityRegistered: Schema.Literal(true),
  systemProximityTransitionCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
  emulatorArtifactInventoryComplete: Schema.Boolean,
  androidSystemGeofenceDeliveryClaimed: Schema.Literal(false),
  physicalDeviceProofClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionRuntimeClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingAndroidEmulatorArtifactInventoryRowSchema = withParser(
  TrackingAndroidEmulatorArtifactInventoryRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        androidEmulatorArtifactInventoryRowIsHonest(row) ||
        'Expected Android emulator artifact inventory rows to classify required local artifacts without claiming physical-device readiness'
    )
  )
);

export const TrackingAndroidEmulatorArtifactInventoryProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-android-emulator-artifact-inventory-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingAndroidEmulatorArtifactInventoryRowSchema).pipe(Schema.minItems(1)),
    summary: Schema.Struct({
      requiredArtifactCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      presentArtifactCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      missingArtifactCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      permissionUiArtifactCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      runtimeArtifactCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      localGeofenceTransitionCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      localGeofenceDwellCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      systemProximityTransitionCount: TrackingAndroidEmulatorArtifactInventoryCountSchema,
      emulatorArtifactInventoryComplete: Schema.Boolean,
    }),
    proofClaims: Schema.Struct({
      adbRuntimeArtifactsObserved: Schema.Literal(true),
      permissionUiArtifactsObserved: Schema.Literal(true),
      locationRuntimeArtifactsObserved: Schema.Literal(true),
      geofenceRuntimeArtifactsObserved: Schema.Literal(true),
      deviceStatusArtifactsObserved: Schema.Literal(true),
      noAndroidSystemGeofenceDeliveryClaim: Schema.Literal(true),
      noPhysicalDeviceClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      androidEmulatorArtifactInventoryComplete: Schema.Boolean,
      androidSystemGeofenceDeliveryClaimed: Schema.Literal(false),
      physicalDeviceProofClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  })
    .pipe(
      Schema.filter(
        (proof) =>
          proof.summary.requiredArtifactCount ===
            proof.summary.presentArtifactCount + proof.summary.missingArtifactCount ||
          'Android emulator artifact inventory summary must classify every required artifact'
      )
    )
    .pipe(
      Schema.filter(
        (proof) =>
          proof.rows.every(
            (row) =>
              row.emulatorArtifactInventoryComplete === proof.productClaims.androidEmulatorArtifactInventoryComplete
          ) || 'Android emulator artifact inventory rows and product claims must agree on artifact completeness'
      )
    )
);

export type TrackingAndroidEmulatorArtifactInventoryInput = Infer<
  typeof TrackingAndroidEmulatorArtifactInventoryInputSchema
>;
export type TrackingAndroidEmulatorArtifactInventoryProof = Infer<
  typeof TrackingAndroidEmulatorArtifactInventoryProofSchema
>;
type TrackingAndroidEmulatorArtifactInventoryRowInput = Infer<
  typeof TrackingAndroidEmulatorArtifactInventoryRowBaseSchema
>;

export function buildTrackingAndroidEmulatorArtifactInventoryProof(
  generatedAt: string,
  input: TrackingAndroidEmulatorArtifactInventoryInput
): TrackingAndroidEmulatorArtifactInventoryProof {
  const parsedInput = TrackingAndroidEmulatorArtifactInventoryInputSchema.parse(input);
  const row = artifactInventoryRow(generatedAt, parsedInput);
  const summary = summaryFrom(row);

  return TrackingAndroidEmulatorArtifactInventoryProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-android-emulator-artifact-inventory-proof',
    generatedAt,
    rows: [row],
    summary,
    proofClaims: {
      adbRuntimeArtifactsObserved: true,
      permissionUiArtifactsObserved: true,
      locationRuntimeArtifactsObserved: true,
      geofenceRuntimeArtifactsObserved: true,
      deviceStatusArtifactsObserved: true,
      noAndroidSystemGeofenceDeliveryClaim: true,
      noPhysicalDeviceClaim: true,
      noAuthorityClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      androidEmulatorArtifactInventoryComplete: row.emulatorArtifactInventoryComplete,
      androidSystemGeofenceDeliveryClaimed: false,
      physicalDeviceProofClaimed: false,
      authorityProofClaimed: false,
      productionRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function artifactInventoryRow(generatedAt: string, input: TrackingAndroidEmulatorArtifactInventoryInput) {
  const presentArtifacts = input.artifactRows
    .filter((artifact) => artifact.present)
    .map((artifact) => artifact.artifactRef);
  const missingArtifacts = input.artifactRows
    .filter((artifact) => !artifact.present)
    .map((artifact) => artifact.artifactRef);

  return TrackingAndroidEmulatorArtifactInventoryRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-android-emulator-artifact-inventory',
    generatedAt,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P3_LOCAL_DEV_MACHINE',
    status: 'android-emulator-local-artifacts-present-physical-device-required',
    sourceAndroidEmulatorProofRef: input.sourceAndroidEmulatorProofRef,
    auditRefs: ['tracking-android-emulator-artifact-inventory-audit'],
    androidSdkRoot: input.androidSdkRoot,
    androidProofStatus: input.androidProofStatus,
    packageName: input.packageName,
    activityName: input.activityName,
    deviceSerial: input.deviceSerial,
    androidRelease: input.androidRelease,
    androidSdk: input.androidSdk,
    productModel: input.productModel,
    abi: input.abi,
    requiredArtifacts: [...RequiredTrackingAndroidEmulatorArtifactRefs],
    presentArtifacts,
    missingArtifacts,
    artifactRows: input.artifactRows,
    foregroundPermissionGranted: input.foregroundPermissionGranted,
    backgroundPermissionGranted: input.backgroundPermissionGranted,
    foregroundPermissionUxObserved: input.foregroundPermissionUxObserved,
    backgroundSettingsPageObserved: input.backgroundSettingsPageObserved,
    packageLaunchObserved: input.packageLaunchObserved,
    foregroundServiceObserved: input.foregroundServiceObserved,
    localGeofenceTransitionCount: input.localGeofenceTransitionCount,
    localGeofenceDwellCount: input.localGeofenceDwellCount,
    systemProximityRegistered: input.systemProximityRegistered,
    systemProximityTransitionCount: input.systemProximityTransitionCount,
    emulatorArtifactInventoryComplete: missingArtifacts.length === 0,
    androidSystemGeofenceDeliveryClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
    productionRuntimeClaimed: false,
    productClaimReady: false,
  });
}

function summaryFrom(row: TrackingAndroidEmulatorArtifactInventoryRowInput) {
  return {
    requiredArtifactCount: row.requiredArtifacts.length,
    presentArtifactCount: row.presentArtifacts.length,
    missingArtifactCount: row.missingArtifacts.length,
    permissionUiArtifactCount: row.artifactRows.filter((artifact) => artifact.category === 'permission-ui').length,
    runtimeArtifactCount: row.artifactRows.filter((artifact) =>
      ['adb-runtime-output', 'location-runtime', 'geofence-runtime', 'device-status'].includes(artifact.category)
    ).length,
    localGeofenceTransitionCount: row.localGeofenceTransitionCount,
    localGeofenceDwellCount: row.localGeofenceDwellCount,
    systemProximityTransitionCount: row.systemProximityTransitionCount,
    emulatorArtifactInventoryComplete: row.emulatorArtifactInventoryComplete,
  };
}

function androidEmulatorArtifactInventoryRowIsHonest(row: TrackingAndroidEmulatorArtifactInventoryRowInput): boolean {
  const requiredArtifactSet = new Set(row.requiredArtifacts.map((artifactRef) => String(artifactRef)));
  const artifactRowSet = new Set(row.artifactRows.map((artifact) => String(artifact.artifactRef)));
  return (
    androidEmulatorArtifactRowsCoverRequirements(row, requiredArtifactSet, artifactRowSet) &&
    androidEmulatorRuntimeRowsAreObserved(row) &&
    androidEmulatorArtifactInventoryNonClaimsAreHonest(row)
  );
}

function androidEmulatorArtifactRowsCoverRequirements(
  row: TrackingAndroidEmulatorArtifactInventoryRowInput,
  requiredArtifactSet: ReadonlySet<string>,
  artifactRowSet: ReadonlySet<string>
): boolean {
  return (
    RequiredTrackingAndroidEmulatorArtifactRefs.every(
      (artifactRef) => requiredArtifactSet.has(artifactRef) && artifactRowSet.has(artifactRef)
    ) &&
    row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length &&
    row.artifactRows.every((artifact) => artifact.required === true) &&
    row.presentArtifacts.every((artifactRef) => requiredArtifactSet.has(String(artifactRef))) &&
    row.missingArtifacts.every((artifactRef) => requiredArtifactSet.has(String(artifactRef)))
  );
}

function androidEmulatorRuntimeRowsAreObserved(row: TrackingAndroidEmulatorArtifactInventoryRowInput): boolean {
  return (
    row.packageLaunchObserved === true &&
    row.foregroundServiceObserved === true &&
    row.localGeofenceTransitionCount > 0 &&
    row.localGeofenceDwellCount > 0 &&
    row.systemProximityRegistered === true
  );
}

function androidEmulatorArtifactInventoryNonClaimsAreHonest(
  row: TrackingAndroidEmulatorArtifactInventoryRowInput
): boolean {
  return (
    row.androidSystemGeofenceDeliveryClaimed === false &&
    row.physicalDeviceProofClaimed === false &&
    row.authorityProofClaimed === false &&
    row.productionRuntimeClaimed === false &&
    row.productClaimReady === false
  );
}
