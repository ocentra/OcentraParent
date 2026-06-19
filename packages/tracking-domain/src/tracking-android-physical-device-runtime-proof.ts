import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
const TrackingAndroidPhysicalDeviceRuntimeCountSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingAndroidPhysicalDeviceRuntimeRefSchema = brandedNonEmptyStringSchema('TrackingAndroidPhysicalDeviceRuntimeRef');

export const TrackingAndroidPhysicalDeviceRuntimeRowIdSchema = brandedNonEmptyStringSchema('TrackingAndroidPhysicalDeviceRuntimeRowId');

export const TrackingAndroidPhysicalDeviceRuntimeStatusSchema = Schema.Literal(
  'android-physical-device-runtime-observed-product-gaps-remain'
);

export const TrackingAndroidPhysicalDeviceRuntimeCategorySchema = Schema.Literal(
  'adb-runtime-output',
  'package-runtime',
  'foreground-service',
  'device-status',
  'ui-screenshot',
  'permission-state',
  'physical-location-runtime',
  'physical-geofence-runtime',
  'physical-route-observation',
  'validation-log'
);

export const RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs = [
  'test-results/tracking-android-physical-device-runtime-proof/00-device.json',
  'test-results/tracking-android-physical-device-runtime-proof/01-adb-connect.txt',
  'test-results/tracking-android-physical-device-runtime-proof/02-adb-install.txt',
  'test-results/tracking-android-physical-device-runtime-proof/03-prepare-device-for-launch.txt',
  'test-results/tracking-android-physical-device-runtime-proof/03-launch-activity.txt',
  'test-results/tracking-android-physical-device-runtime-proof/03-start-service.txt',
  'test-results/tracking-android-physical-device-runtime-proof/04-service-dump.txt',
  'test-results/tracking-android-physical-device-runtime-proof/05-activity-dump.txt',
  'test-results/tracking-android-physical-device-runtime-proof/06-window-dump.txt',
  'test-results/tracking-android-physical-device-runtime-proof/07-battery.txt',
  'test-results/tracking-android-physical-device-runtime-proof/08-connectivity.txt',
  'test-results/tracking-android-physical-device-runtime-proof/09-ui.xml',
  'test-results/tracking-android-physical-device-runtime-proof/10-screen.png',
  'test-results/tracking-android-physical-device-runtime-proof/11-logcat.txt',
  'test-results/tracking-android-physical-device-runtime-proof/12-package-dump.txt',
  'test-results/tracking-android-physical-device-runtime-proof/13-permission-state.json',
  'test-results/tracking-android-physical-device-runtime-proof/14-background-location-sample-prefs.xml',
  'test-results/tracking-android-physical-device-runtime-proof/15-geofence-transition-prefs.xml',
  'test-results/tracking-android-physical-device-runtime-proof/16-physical-route-observation.txt',
  'test-results/tracking-android-physical-device-runtime-proof/17-location-manager-state.txt',
] as const;

const TrackingAndroidPhysicalDeviceRuntimeArtifactRowSchema = Schema.Struct({
  artifactRef: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  category: TrackingAndroidPhysicalDeviceRuntimeCategorySchema,
  required: Schema.Literal(true),
  present: Schema.Boolean,
  byteSize: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
});

export const TrackingAndroidPhysicalDeviceRuntimeInputSchema = withParser(
  Schema.Struct({
    physicalDeviceProofRef: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    packageName: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    activityName: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    deviceSerial: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    androidRelease: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    androidSdk: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    productModel: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    productName: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    abi: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
    packageInstallObserved: Schema.Boolean,
    packageLaunchObserved: Schema.Boolean,
    foregroundServiceObserved: Schema.Boolean,
    uiLaunchTextObserved: Schema.Boolean,
    batteryDumpObserved: Schema.Boolean,
    connectivityDumpObserved: Schema.Boolean,
    foregroundPermissionGranted: Schema.Boolean,
    backgroundPermissionGranted: Schema.Boolean,
    geofenceRegistrationObserved: Schema.Boolean,
    systemProximityRegistrationObserved: Schema.Boolean,
    locationSampleObserved: Schema.Boolean,
    backgroundLocationSampleCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
    physicalRouteObservationWindowSeconds: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
    shellLocationInjectionAvailable: Schema.Boolean,
    localGeofenceTransitionCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
    localGeofenceDwellCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
    androidSystemGeofenceTransitionCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
    artifactRows: Schema.Array(TrackingAndroidPhysicalDeviceRuntimeArtifactRowSchema).pipe(
      Schema.minItems(RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.length)
    ),
  })
);

const TrackingAndroidPhysicalDeviceRuntimeRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  rowId: TrackingAndroidPhysicalDeviceRuntimeRowIdSchema,
  generatedAt: ParentTimestampSchema,
  requiredProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  currentProofTier: Schema.Literal('P4_PHYSICAL_DEVICE'),
  status: TrackingAndroidPhysicalDeviceRuntimeStatusSchema,
  physicalDeviceProofRef: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  auditRefs: Schema.Array(TrackingPolicyAuditRefSchema).pipe(Schema.minItems(1)),
  packageName: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  activityName: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  deviceSerial: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  androidRelease: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  androidSdk: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  productModel: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  productName: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  abi: TrackingAndroidPhysicalDeviceRuntimeRefSchema,
  requiredArtifacts: Schema.Array(TrackingAndroidPhysicalDeviceRuntimeRefSchema).pipe(
    Schema.minItems(RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.length)
  ),
  presentArtifacts: Schema.Array(TrackingAndroidPhysicalDeviceRuntimeRefSchema),
  missingArtifacts: Schema.Array(TrackingAndroidPhysicalDeviceRuntimeRefSchema),
  artifactRows: Schema.Array(TrackingAndroidPhysicalDeviceRuntimeArtifactRowSchema).pipe(
    Schema.minItems(RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.length)
  ),
  packageInstallObserved: Schema.Literal(true),
  packageLaunchObserved: Schema.Literal(true),
  foregroundServiceObserved: Schema.Literal(true),
  uiLaunchTextObserved: Schema.Boolean,
  batteryDumpObserved: Schema.Literal(true),
  connectivityDumpObserved: Schema.Literal(true),
  foregroundPermissionGranted: Schema.Boolean,
  backgroundPermissionGranted: Schema.Boolean,
  geofenceRegistrationObserved: Schema.Boolean,
  systemProximityRegistrationObserved: Schema.Boolean,
  locationSampleObserved: Schema.Boolean,
  backgroundLocationSampleCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
  physicalRouteObservationWindowSeconds: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
  shellLocationInjectionAvailable: Schema.Boolean,
  localGeofenceTransitionCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
  localGeofenceDwellCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
  androidSystemGeofenceTransitionCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
  physicalDeviceRuntimeObserved: Schema.Literal(true),
  physicalLocationRuntimeClaimed: Schema.Literal(false),
  physicalGeofenceRuntimeClaimed: Schema.Literal(false),
  androidSystemGeofenceDeliveryClaimed: Schema.Literal(false),
  authorityProofClaimed: Schema.Literal(false),
  productionRuntimeClaimed: Schema.Literal(false),
  productClaimReady: Schema.Literal(false),
});

export const TrackingAndroidPhysicalDeviceRuntimeRowSchema = withParser(
  TrackingAndroidPhysicalDeviceRuntimeRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        androidPhysicalDeviceRuntimeRowIsHonest(row) ||
        'Expected Android physical-device runtime rows to prove only package/service/status evidence without product-ready tracking claims'
    )
  )
);

export const TrackingAndroidPhysicalDeviceRuntimeProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-android-physical-device-runtime-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingAndroidPhysicalDeviceRuntimeRowSchema).pipe(Schema.minItems(1)),
    summary: Schema.Struct({
      requiredArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      presentArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      missingArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      packageRuntimeArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      statusArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      physicalLocationArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      physicalGeofenceArtifactCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      backgroundLocationSampleCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      physicalRouteObservationWindowSeconds: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      shellLocationInjectionAvailable: Schema.Boolean,
      geofenceRegistrationObserved: Schema.Boolean,
      systemProximityRegistrationObserved: Schema.Boolean,
      localGeofenceTransitionCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      localGeofenceDwellCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      androidSystemGeofenceTransitionCount: TrackingAndroidPhysicalDeviceRuntimeCountSchema,
      physicalDeviceRuntimeObserved: Schema.Literal(true),
    }),
    proofClaims: Schema.Struct({
      adbPhysicalDeviceObserved: Schema.Literal(true),
      packageInstallObserved: Schema.Literal(true),
      packageLaunchObserved: Schema.Literal(true),
      foregroundServiceObserved: Schema.Literal(true),
      batteryAndConnectivityObserved: Schema.Literal(true),
      geofenceRegistrationObserved: Schema.Boolean,
      systemProximityRegistrationObserved: Schema.Boolean,
      noPhysicalLocationRuntimeClaim: Schema.Literal(true),
      noPhysicalGeofenceRuntimeClaim: Schema.Literal(true),
      noAndroidSystemGeofenceDeliveryClaim: Schema.Literal(true),
      noAuthorityClaim: Schema.Literal(true),
      noProductionClaim: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      physicalDeviceRuntimeObserved: Schema.Literal(true),
      physicalLocationRuntimeClaimed: Schema.Literal(false),
      physicalGeofenceRuntimeClaimed: Schema.Literal(false),
      androidSystemGeofenceDeliveryClaimed: Schema.Literal(false),
      authorityProofClaimed: Schema.Literal(false),
      productionRuntimeClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        proof.summary.requiredArtifactCount ===
          proof.summary.presentArtifactCount + proof.summary.missingArtifactCount ||
        'Android physical-device runtime summary must classify every required artifact'
    )
  )
);

export type TrackingAndroidPhysicalDeviceRuntimeInput = Infer<typeof TrackingAndroidPhysicalDeviceRuntimeInputSchema>;
export type TrackingAndroidPhysicalDeviceRuntimeProof = Infer<typeof TrackingAndroidPhysicalDeviceRuntimeProofSchema>;
type TrackingAndroidPhysicalDeviceRuntimeRowInput = Infer<typeof TrackingAndroidPhysicalDeviceRuntimeRowBaseSchema>;

export function buildTrackingAndroidPhysicalDeviceRuntimeProof(
  generatedAt: string,
  input: TrackingAndroidPhysicalDeviceRuntimeInput
): TrackingAndroidPhysicalDeviceRuntimeProof {
  const parsedInput = TrackingAndroidPhysicalDeviceRuntimeInputSchema.parse(input);
  const row = physicalDeviceRuntimeRow(generatedAt, parsedInput);
  const summary = summaryFrom(row);

  return TrackingAndroidPhysicalDeviceRuntimeProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-android-physical-device-runtime-proof',
    generatedAt,
    rows: [row],
    summary,
    proofClaims: {
      adbPhysicalDeviceObserved: true,
      packageInstallObserved: true,
      packageLaunchObserved: true,
      foregroundServiceObserved: true,
      batteryAndConnectivityObserved: true,
      geofenceRegistrationObserved: row.geofenceRegistrationObserved,
      systemProximityRegistrationObserved: row.systemProximityRegistrationObserved,
      noPhysicalLocationRuntimeClaim: true,
      noPhysicalGeofenceRuntimeClaim: true,
      noAndroidSystemGeofenceDeliveryClaim: true,
      noAuthorityClaim: true,
      noProductionClaim: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      physicalDeviceRuntimeObserved: true,
      physicalLocationRuntimeClaimed: false,
      physicalGeofenceRuntimeClaimed: false,
      androidSystemGeofenceDeliveryClaimed: false,
      authorityProofClaimed: false,
      productionRuntimeClaimed: false,
      productClaimReady: false,
    },
  });
}

function physicalDeviceRuntimeRow(generatedAt: string, input: TrackingAndroidPhysicalDeviceRuntimeInput) {
  const presentArtifacts = input.artifactRows
    .filter((artifact) => artifact.present)
    .map((artifact) => artifact.artifactRef);
  const missingArtifacts = input.artifactRows
    .filter((artifact) => !artifact.present)
    .map((artifact) => artifact.artifactRef);

  return TrackingAndroidPhysicalDeviceRuntimeRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId: 'tracking-android-physical-device-runtime',
    generatedAt,
    requiredProofTier: 'P4_PHYSICAL_DEVICE',
    currentProofTier: 'P4_PHYSICAL_DEVICE',
    status: 'android-physical-device-runtime-observed-product-gaps-remain',
    physicalDeviceProofRef: input.physicalDeviceProofRef,
    auditRefs: ['tracking-android-physical-device-runtime-audit'],
    packageName: input.packageName,
    activityName: input.activityName,
    deviceSerial: input.deviceSerial,
    androidRelease: input.androidRelease,
    androidSdk: input.androidSdk,
    productModel: input.productModel,
    productName: input.productName,
    abi: input.abi,
    requiredArtifacts: [...RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs],
    presentArtifacts,
    missingArtifacts,
    artifactRows: input.artifactRows,
    packageInstallObserved: input.packageInstallObserved,
    packageLaunchObserved: input.packageLaunchObserved,
    foregroundServiceObserved: input.foregroundServiceObserved,
    uiLaunchTextObserved: input.uiLaunchTextObserved,
    batteryDumpObserved: input.batteryDumpObserved,
    connectivityDumpObserved: input.connectivityDumpObserved,
    foregroundPermissionGranted: input.foregroundPermissionGranted,
    backgroundPermissionGranted: input.backgroundPermissionGranted,
    geofenceRegistrationObserved: input.geofenceRegistrationObserved,
    systemProximityRegistrationObserved: input.systemProximityRegistrationObserved,
    locationSampleObserved: input.locationSampleObserved,
    backgroundLocationSampleCount: input.backgroundLocationSampleCount,
    physicalRouteObservationWindowSeconds: input.physicalRouteObservationWindowSeconds,
    shellLocationInjectionAvailable: input.shellLocationInjectionAvailable,
    localGeofenceTransitionCount: input.localGeofenceTransitionCount,
    localGeofenceDwellCount: input.localGeofenceDwellCount,
    androidSystemGeofenceTransitionCount: input.androidSystemGeofenceTransitionCount,
    physicalDeviceRuntimeObserved: true,
    physicalLocationRuntimeClaimed: false,
    physicalGeofenceRuntimeClaimed: false,
    androidSystemGeofenceDeliveryClaimed: false,
    authorityProofClaimed: false,
    productionRuntimeClaimed: false,
    productClaimReady: false,
  });
}

function summaryFrom(row: TrackingAndroidPhysicalDeviceRuntimeRowInput) {
  return {
    requiredArtifactCount: row.requiredArtifacts.length,
    presentArtifactCount: row.presentArtifacts.length,
    missingArtifactCount: row.missingArtifacts.length,
    packageRuntimeArtifactCount: row.artifactRows.filter((artifact) =>
      ['package-runtime', 'foreground-service', 'ui-screenshot'].includes(artifact.category)
    ).length,
    statusArtifactCount: row.artifactRows.filter((artifact) => artifact.category === 'device-status').length,
    physicalLocationArtifactCount: row.artifactRows.filter(
      (artifact) => artifact.category === 'physical-location-runtime'
    ).length,
    physicalGeofenceArtifactCount: row.artifactRows.filter(
      (artifact) => artifact.category === 'physical-geofence-runtime'
    ).length,
    backgroundLocationSampleCount: row.backgroundLocationSampleCount,
    physicalRouteObservationWindowSeconds: row.physicalRouteObservationWindowSeconds,
    shellLocationInjectionAvailable: row.shellLocationInjectionAvailable,
    geofenceRegistrationObserved: row.geofenceRegistrationObserved,
    systemProximityRegistrationObserved: row.systemProximityRegistrationObserved,
    localGeofenceTransitionCount: row.localGeofenceTransitionCount,
    localGeofenceDwellCount: row.localGeofenceDwellCount,
    androidSystemGeofenceTransitionCount: row.androidSystemGeofenceTransitionCount,
    physicalDeviceRuntimeObserved: true as const,
  };
}

function androidPhysicalDeviceRuntimeRowIsHonest(row: TrackingAndroidPhysicalDeviceRuntimeRowInput): boolean {
  return (
    requiredArtifactCoverageIsHonest(row) &&
    artifactPresenceIsHonest(row) &&
    androidPhysicalDeviceRuntimeClaimsAreHonest(row)
  );
}

function requiredArtifactCoverageIsHonest(row: TrackingAndroidPhysicalDeviceRuntimeRowInput): boolean {
  const requiredArtifactSet = new Set(row.requiredArtifacts.map((artifactRef) => String(artifactRef)));
  const artifactRowSet = new Set(row.artifactRows.map((artifact) => String(artifact.artifactRef)));
  return (
    RequiredTrackingAndroidPhysicalDeviceRuntimeArtifactRefs.every(
      (artifactRef) => requiredArtifactSet.has(artifactRef) && artifactRowSet.has(artifactRef)
    ) &&
    row.requiredArtifacts.length === row.presentArtifacts.length + row.missingArtifacts.length &&
    row.artifactRows.every((artifact) => artifact.required === true)
  );
}

function artifactPresenceIsHonest(row: TrackingAndroidPhysicalDeviceRuntimeRowInput): boolean {
  const requiredArtifactSet = new Set(row.requiredArtifacts.map((artifactRef) => String(artifactRef)));
  return (
    row.presentArtifacts.every((artifactRef) => requiredArtifactSet.has(String(artifactRef))) &&
    row.missingArtifacts.every((artifactRef) => requiredArtifactSet.has(String(artifactRef)))
  );
}

function androidPhysicalDeviceRuntimeClaimsAreHonest(row: TrackingAndroidPhysicalDeviceRuntimeRowInput): boolean {
  return (
    row.packageInstallObserved === true &&
    row.packageLaunchObserved === true &&
    row.foregroundServiceObserved === true &&
    row.batteryDumpObserved === true &&
    row.connectivityDumpObserved === true &&
    row.physicalLocationRuntimeClaimed === false &&
    row.physicalGeofenceRuntimeClaimed === false &&
    row.androidSystemGeofenceDeliveryClaimed === false &&
    row.authorityProofClaimed === false &&
    row.productionRuntimeClaimed === false &&
    row.productClaimReady === false
  );
}

