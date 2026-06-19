import {
  type Infer,
  Schema,
  withParser,
  brandedNonEmptyStringSchema
} from '@ocentra-parent/schema-domain/effect';
import { ParentTimestampSchema } from '@ocentra-parent/schema-domain/family-reference-primitives';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';
import { TrackingRetentionSettingsProofRefSchema } from './tracking-retention-settings-read-model-proof';
const TrackingAndroidSystemGeofenceCounterSchema = Schema.Number.pipe(Schema.int(), Schema.nonNegative());

export const TrackingAndroidSystemGeofenceProofIdSchema = brandedNonEmptyStringSchema('TrackingAndroidSystemGeofenceProofId');

export const TrackingAndroidSystemGeofenceArtifactRefSchema = brandedNonEmptyStringSchema('TrackingAndroidSystemGeofenceArtifactRef');

export const TrackingAndroidSystemGeofenceBlockerSchema = Schema.Literal(
  'system-proximity-broadcast-counter-zero',
  'dwell-transition-not-observed',
  'physical-device-proof-required',
  'authority-proof-required'
);

export const TrackingAndroidSystemGeofenceBlockerRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    blockerProofId: TrackingAndroidSystemGeofenceProofIdSchema,
    generatedAt: ParentTimestampSchema,
    sourceAndroidEmulatorProofRef: TrackingRetentionSettingsProofRefSchema,
    auditRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    localListenerGeofenceTransitionCount: TrackingAndroidSystemGeofenceCounterSchema,
    localListenerGeofenceEnterCount: TrackingAndroidSystemGeofenceCounterSchema,
    localListenerGeofenceExitCount: TrackingAndroidSystemGeofenceCounterSchema,
    systemProximityRegistered: Schema.Literal(true),
    systemProximityTransitionCount: Schema.Literal(0),
    systemProximityEnterCount: Schema.Literal(0),
    systemProximityExitCount: Schema.Literal(0),
    localEvidenceArtifactRefs: Schema.Array(TrackingAndroidSystemGeofenceArtifactRefSchema),
    requiredRuntimeArtifactRefs: Schema.Array(TrackingAndroidSystemGeofenceArtifactRefSchema),
    presentRuntimeArtifactRefs: Schema.Array(TrackingAndroidSystemGeofenceArtifactRefSchema),
    missingRuntimeArtifactRefs: Schema.Array(TrackingAndroidSystemGeofenceArtifactRefSchema),
    runtimeArtifactSetComplete: Schema.Literal(false),
    blockerRefs: Schema.Array(TrackingAndroidSystemGeofenceBlockerSchema),
    appOwnedLocalGeofenceClaimed: Schema.Literal(true),
    androidSystemGeofenceDeliveryClaimed: Schema.Literal(false),
    dwellTransitionClaimed: Schema.Literal(false),
    backgroundLocationRuntimeClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
  })
    .pipe(Schema.filter((row) => row.auditRefs.length > 0 || 'Android geofence blocker rows need audit refs'))
    .pipe(
      Schema.filter(
        (row) =>
          row.blockerRefs.length === RequiredSystemGeofenceBlockers.length ||
          'Android geofence blocker rows need every blocker ref'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.localEvidenceArtifactRefs.length > 0 || 'Android geofence blocker rows need local evidence artifact refs'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.requiredRuntimeArtifactRefs.length ===
            row.presentRuntimeArtifactRefs.length + row.missingRuntimeArtifactRefs.length ||
          'Android geofence blocker rows must classify every runtime artifact ref'
      )
    )
);

export const TrackingAndroidSystemGeofenceBlockerProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-android-system-geofence-blocker-proof'),
    generatedAt: ParentTimestampSchema,
    rows: Schema.Array(TrackingAndroidSystemGeofenceBlockerRowSchema),
    proofClaims: Schema.Struct({
      localListenerGeofenceObserved: Schema.Literal(true),
      systemProximityRegistrationObserved: Schema.Literal(true),
      systemProximityDeliveryBlocked: Schema.Literal(true),
      noProductReadyClaim: Schema.Literal(true),
    }),
    productClaims: Schema.Struct({
      appOwnedLocalGeofenceClaimed: Schema.Literal(true),
      androidSystemGeofenceDeliveryClaimed: Schema.Literal(false),
      dwellTransitionClaimed: Schema.Literal(false),
      backgroundLocationRuntimeClaimed: Schema.Literal(false),
      physicalDeviceClaimed: Schema.Literal(false),
      authorityClaimed: Schema.Literal(false),
      productClaimReady: Schema.Literal(false),
    }),
  }).pipe(Schema.filter((proof) => proof.rows.length > 0 || 'Android geofence blocker proof needs a row'))
);

export type TrackingAndroidSystemGeofenceBlockerProof = Infer<typeof TrackingAndroidSystemGeofenceBlockerProofSchema>;

export const RequiredSystemGeofenceBlockers = [
  'system-proximity-broadcast-counter-zero',
  'dwell-transition-not-observed',
  'physical-device-proof-required',
  'authority-proof-required',
] as const;

export const LocalAndroidSystemGeofenceEvidenceArtifactRefs = [
  'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/05-geofence-transition-proof.json',
] as const;

export const RequiredAndroidSystemGeofenceRuntimeArtifactRefs = [
  'output/tracking-plan-proof/android-background-geofence/system-proximity-broadcast-transitions.ndjson',
  'output/tracking-plan-proof/android-background-geofence/system-dwell-transition-observations.ndjson',
  'output/tracking-plan-proof/android-background-geofence/physical-device-background-geofence-result.json',
  'output/tracking-plan-proof/android-background-geofence/authority-enrolled-geofence-runtime.json',
] as const;

export function buildTrackingAndroidSystemGeofenceBlockerProof(
  generatedAt: string,
  sourceAndroidEmulatorProofRef: string,
  androidEmulatorProof: unknown
): TrackingAndroidSystemGeofenceBlockerProof {
  const geofence = geofenceTransitionsFrom(androidEmulatorProof);
  return TrackingAndroidSystemGeofenceBlockerProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    proofMode: 'tracking-android-system-geofence-blocker-proof',
    generatedAt,
    rows: [blockerRow(generatedAt, sourceAndroidEmulatorProofRef, geofence)],
    proofClaims: {
      localListenerGeofenceObserved: true,
      systemProximityRegistrationObserved: true,
      systemProximityDeliveryBlocked: true,
      noProductReadyClaim: true,
    },
    productClaims: {
      appOwnedLocalGeofenceClaimed: true,
      androidSystemGeofenceDeliveryClaimed: false,
      dwellTransitionClaimed: false,
      backgroundLocationRuntimeClaimed: false,
      physicalDeviceClaimed: false,
      authorityClaimed: false,
      productClaimReady: false,
    },
  });
}

type GeofenceTransitions = {
  readonly transitionCount: number;
  readonly enterCount: number;
  readonly exitCount: number;
  readonly systemProximityRegistered: boolean;
  readonly systemProximityTransitionCount: number;
  readonly systemProximityEnterCount: number;
  readonly systemProximityExitCount: number;
};

function blockerRow(generatedAt: string, sourceAndroidEmulatorProofRef: string, geofence: GeofenceTransitions) {
  if (!geofence.systemProximityRegistered || geofence.transitionCount <= 0) {
    throw new Error('Android emulator proof must include local geofence rows and system proximity registration');
  }
  return TrackingAndroidSystemGeofenceBlockerRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    blockerProofId: 'tracking-android-system-geofence-delivery-blocked',
    generatedAt,
    sourceAndroidEmulatorProofRef,
    auditRefs: ['tracking-android-system-geofence-delivery-blocker-audit'],
    localListenerGeofenceTransitionCount: geofence.transitionCount,
    localListenerGeofenceEnterCount: geofence.enterCount,
    localListenerGeofenceExitCount: geofence.exitCount,
    systemProximityRegistered: true,
    systemProximityTransitionCount: geofence.systemProximityTransitionCount,
    systemProximityEnterCount: geofence.systemProximityEnterCount,
    systemProximityExitCount: geofence.systemProximityExitCount,
    localEvidenceArtifactRefs: [...LocalAndroidSystemGeofenceEvidenceArtifactRefs],
    requiredRuntimeArtifactRefs: [...RequiredAndroidSystemGeofenceRuntimeArtifactRefs],
    presentRuntimeArtifactRefs: [],
    missingRuntimeArtifactRefs: [...RequiredAndroidSystemGeofenceRuntimeArtifactRefs],
    runtimeArtifactSetComplete: false,
    blockerRefs: [...RequiredSystemGeofenceBlockers],
    appOwnedLocalGeofenceClaimed: true,
    androidSystemGeofenceDeliveryClaimed: false,
    dwellTransitionClaimed: false,
    backgroundLocationRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
    productClaimReady: false,
  });
}

function geofenceTransitionsFrom(androidEmulatorProof: unknown): GeofenceTransitions {
  const candidate = androidEmulatorProof as {
    readonly runtime?: { readonly geofenceTransitions?: Partial<GeofenceTransitions> };
  };
  const geofence = candidate.runtime?.geofenceTransitions;
  if (geofence === undefined) {
    throw new Error('Android emulator proof is missing runtime geofence transitions');
  }
  return {
    transitionCount: numberFrom(geofence.transitionCount),
    enterCount: numberFrom(geofence.enterCount),
    exitCount: numberFrom(geofence.exitCount),
    systemProximityRegistered: geofence.systemProximityRegistered === true,
    systemProximityTransitionCount: numberFrom(geofence.systemProximityTransitionCount),
    systemProximityEnterCount: numberFrom(geofence.systemProximityEnterCount),
    systemProximityExitCount: numberFrom(geofence.systemProximityExitCount),
  };
}

function numberFrom(value: unknown): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) {
    throw new Error(`Expected finite Android geofence counter, got ${String(value)}`);
  }
  return value;
}

