import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  TrackingPlatformProofRouteStateSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingPlatformManualRequiredTextSchema = Schema.String.pipe(Schema.minLength(1));

const TrackingPlatformManualRequiredCapabilitySchema = withParser(
  Schema.Literal('foreground-location', 'background-location', 'geofence', 'device-status')
);

const TrackingPlatformManualRequiredProofLevelSchema = withParser(
  Schema.Literal('ci-mechanical', 'emulator-scaffold', 'simulator-package', 'manual-required', 'authority-required')
);

const TrackingPlatformManualRequiredClaimStateSchema = withParser(
  Schema.Literal('proved', 'manual-required', 'authority-required', 'not-claimed')
);

const TrackingPlatformManualRequiredRouteStateSchema = withParser(
  Schema.Struct({
    capability: TrackingPlatformManualRequiredCapabilitySchema,
    routeState: TrackingPlatformProofRouteStateSchema,
    claimState: TrackingPlatformManualRequiredClaimStateSchema,
    proofLevel: TrackingPlatformManualRequiredProofLevelSchema,
    proofArtifactRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    manualRequiredReason: TrackingPolicyReasonCodeSchema,
    parentVisibleStatus: TrackingPlatformManualRequiredTextSchema,
    childSafeStatus: TrackingPlatformManualRequiredTextSchema,
    productClaimReady: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (row) =>
        row.claimState !== 'proved' ||
        row.proofArtifactRefs.length > 0 ||
        'Tracking platform proved rows need proof artifact references'
    ),
    Schema.filter(
      (row) =>
        row.claimState !== 'proved' ||
        row.routeState === 'contract-proved' ||
        'Tracking platform proved rows must use the contract-proved route state'
    ),
    Schema.filter(
      (row) =>
        row.routeState !== 'contract-proved' ||
        row.claimState === 'proved' ||
        'Tracking contract-proved route states must be represented as proved rows'
    ),
    Schema.filter(
      (row) =>
        row.claimState === 'proved' ||
        row.proofLevel === 'manual-required' ||
        row.proofLevel === 'authority-required' ||
        row.proofLevel === 'emulator-scaffold' ||
        row.proofLevel === 'simulator-package' ||
        'Unproved tracking platform rows must stay manual, authority, emulator, or simulator scoped'
    )
  )
);

export const TrackingPlatformManualRequiredProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    proofMode: Schema.Literal('tracking-platform-manual-required-proof'),
    generatedFrom: TrackingPlatformManualRequiredTextSchema,
    rows: Schema.Array(TrackingPlatformManualRequiredRouteStateSchema).pipe(Schema.minItems(1)),
    nonClaims: Schema.Struct({
      noAndroidForegroundLocationClaim: Schema.Literal(true),
      noAndroidBackgroundGeofenceClaim: Schema.Literal(true),
      noIosCoreLocationClaim: Schema.Literal(true),
      noIosBackgroundRegionClaim: Schema.Literal(true),
      noPhysicalDeviceClaim: Schema.Literal(true),
      noAuthorityEnrolledClaim: Schema.Literal(true),
      noProviderDeliveryClaim: Schema.Literal(true),
    }),
  }).pipe(
    Schema.filter(
      (proof) =>
        trackingManualRequiredProofHasRequiredRows(proof.rows) ||
        'Tracking platform manual-required proof must include Android and iOS foreground/background/geofence rows'
    ),
    Schema.filter(
      (proof) =>
        proof.rows.every((row) => !row.productClaimReady) ||
        'Tracking platform manual-required rows must not be product-claim ready'
    )
  )
);

export type TrackingPlatformManualRequiredProof = Infer<typeof TrackingPlatformManualRequiredProofSchema>;
export type TrackingPlatformManualRequiredRouteState = Infer<typeof TrackingPlatformManualRequiredRouteStateSchema>;

const TrackingPlatformManualRequiredProofFixtureInput = {
  schemaVersion: TrackingPolicySchemaVersion,
  proofMode: 'tracking-platform-manual-required-proof',
  generatedFrom: 'docs/plans/tracking-plan/workpacks/31-platform-extension-checklists-and-proof-routing.md',
  rows: [
    {
      capability: 'foreground-location',
      routeState: 'manual-required',
      claimState: 'manual-required',
      proofLevel: 'manual-required',
      proofArtifactRefs: ['output/tracking-plan-proof/08-android-foreground-location-adapter/manual-required.md'],
      manualRequiredReason: 'android-foreground-location-real-device-required',
      parentVisibleStatus: 'Android foreground location needs permission and adapter evidence before product claims.',
      childSafeStatus: 'Location sharing is not active until setup and permission proof are complete.',
      productClaimReady: false,
    },
    {
      capability: 'background-location',
      routeState: 'background-permission-required',
      claimState: 'manual-required',
      proofLevel: 'manual-required',
      proofArtifactRefs: [
        'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/manual-required.md',
      ],
      manualRequiredReason: 'android-background-location-real-device-required',
      parentVisibleStatus: 'Android background location remains manual-required until real background evidence exists.',
      childSafeStatus: 'Background location is not claimed without device setup and visible permission proof.',
      productClaimReady: false,
    },
    {
      capability: 'geofence',
      routeState: 'real-device-required',
      claimState: 'manual-required',
      proofLevel: 'manual-required',
      proofArtifactRefs: [
        'output/tracking-plan-proof/09-android-background-location-and-geofence-adapter/manual-required.md',
      ],
      manualRequiredReason: 'android-geofence-real-device-required',
      parentVisibleStatus: 'Android geofence transitions need real-device background proof before product claims.',
      childSafeStatus: 'Geofence alerts are not active until device permission and transition proof are complete.',
      productClaimReady: false,
    },
    {
      capability: 'device-status',
      routeState: 'contract-proved',
      claimState: 'proved',
      proofLevel: 'emulator-scaffold',
      proofArtifactRefs: ['test-results/tracking-plan-android-emulator-proof/proof.json'],
      manualRequiredReason: 'android-device-status-emulator-scaffold-only',
      parentVisibleStatus: 'Android package/service/battery/connectivity scaffold proof exists, not location proof.',
      childSafeStatus: 'Device status scaffold is visible, but location tracking is not active from this proof.',
      productClaimReady: false,
    },
    {
      capability: 'foreground-location',
      routeState: 'manual-required',
      claimState: 'manual-required',
      proofLevel: 'manual-required',
      proofArtifactRefs: ['output/tracking-plan-proof/11-ios-core-location-foreground-adapter/manual-required.md'],
      manualRequiredReason: 'ios-core-location-device-required',
      parentVisibleStatus: 'iOS Core Location needs Mac/device, entitlement, and permission evidence before claims.',
      childSafeStatus: 'iOS location sharing is not active until Apple-approved setup proof exists.',
      productClaimReady: false,
    },
    {
      capability: 'background-location',
      routeState: 'background-permission-required',
      claimState: 'authority-required',
      proofLevel: 'authority-required',
      proofArtifactRefs: [
        'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/manual-required.md',
      ],
      manualRequiredReason: 'ios-background-region-entitlement-required',
      parentVisibleStatus: 'iOS background region monitoring needs entitlement and device proof before claims.',
      childSafeStatus: 'Background location is not claimed without Apple-approved entitlement and device proof.',
      productClaimReady: false,
    },
    {
      capability: 'geofence',
      routeState: 'real-device-required',
      claimState: 'authority-required',
      proofLevel: 'authority-required',
      proofArtifactRefs: [
        'output/tracking-plan-proof/12-ios-background-region-significant-change-adapter/manual-required.md',
      ],
      manualRequiredReason: 'ios-region-monitoring-entitlement-required',
      parentVisibleStatus: 'iOS region/geofence behavior remains authority-required until entitlement proof exists.',
      childSafeStatus: 'Geofence alerts are not active on iOS until approved device proof exists.',
      productClaimReady: false,
    },
    {
      capability: 'device-status',
      routeState: 'not-claimed',
      claimState: 'not-claimed',
      proofLevel: 'simulator-package',
      proofArtifactRefs: ['test-results/tracking-plan-ios-simulator-proof/proof.json'],
      manualRequiredReason: 'ios-simulator-package-mechanics-only',
      parentVisibleStatus: 'iOS simulator package proof exists, but child-device status runtime is not claimed.',
      childSafeStatus: 'The iOS package can be routed for proof, but child-device tracking is not active.',
      productClaimReady: false,
    },
  ],
  nonClaims: {
    noAndroidForegroundLocationClaim: true,
    noAndroidBackgroundGeofenceClaim: true,
    noIosCoreLocationClaim: true,
    noIosBackgroundRegionClaim: true,
    noPhysicalDeviceClaim: true,
    noAuthorityEnrolledClaim: true,
    noProviderDeliveryClaim: true,
  },
} as const;

export function buildTrackingPlatformManualRequiredProof(): TrackingPlatformManualRequiredProof {
  return TrackingPlatformManualRequiredProofSchema.parse(TrackingPlatformManualRequiredProofFixtureInput);
}

export function summarizeTrackingPlatformManualRequiredProof(proof: TrackingPlatformManualRequiredProof) {
  const manualRows = proof.rows.filter((row) => row.claimState === 'manual-required');
  const authorityRows = proof.rows.filter((row) => row.claimState === 'authority-required');
  const productReadyRows = proof.rows.filter((row) => row.productClaimReady);

  return {
    rowCount: proof.rows.length,
    manualRequiredCount: manualRows.length,
    authorityRequiredCount: authorityRows.length,
    productClaimReadyCount: productReadyRows.length,
    androidRows: proof.rows.filter((row) => row.manualRequiredReason.startsWith('android-')).length,
    iosRows: proof.rows.filter((row) => row.manualRequiredReason.startsWith('ios-')).length,
    nonClaims: proof.nonClaims,
  };
}

function trackingManualRequiredProofHasRequiredRows(rows: ReadonlyArray<TrackingPlatformManualRequiredRouteState>) {
  const requiredReasons = new Set([
    'android-foreground-location-real-device-required',
    'android-background-location-real-device-required',
    'android-geofence-real-device-required',
    'ios-core-location-device-required',
    'ios-background-region-entitlement-required',
    'ios-region-monitoring-entitlement-required',
  ]);

  return (
    rows.every((row) => row.proofArtifactRefs.length > 0) &&
    [...requiredReasons].every((reason) => rows.some((row) => row.manualRequiredReason === reason))
  );
}
