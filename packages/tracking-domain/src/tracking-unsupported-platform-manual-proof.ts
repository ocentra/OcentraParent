import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import {
  TrackingPlatformProofRouteStateSchema,
  TrackingPolicyAuditRefSchema,
  TrackingPolicyReasonCodeSchema,
  TrackingPolicySchemaVersion,
} from './tracking-location-policy-primitives';

const TrackingUnsupportedPlatformManualProofTextSchema = Schema.String.pipe(Schema.minLength(1));

export const TrackingUnsupportedPlatformManualProofRowIdSchema = TrackingUnsupportedPlatformManualProofTextSchema.pipe(
  Schema.brand('TrackingUnsupportedPlatformManualProofRowId')
);

export const TrackingUnsupportedPlatformManualProofPlatformSchema = withParser(
  Schema.Literal('android', 'ios', 'windows', 'macos', 'linux', 'web')
);

export const TrackingUnsupportedPlatformManualProofSurfaceSchema = withParser(
  Schema.Literal(
    'foreground-location',
    'background-location',
    'geofence-transition',
    'desktop-os-location',
    'child-agent-location',
    'authority-hard-control',
    'parent-web-portal'
  )
);

export const TrackingUnsupportedPlatformManualProofRenderStateSchema = withParser(
  Schema.Literal('manual-required', 'authority-required', 'unavailable')
);

export const TrackingUnsupportedPlatformManualProofTierSchema = withParser(
  Schema.Literal(
    'P0_CONTRACT',
    'P2_HOSTED_CI',
    'P3_LOCAL_DEV_MACHINE',
    'P4_PHYSICAL_DEVICE',
    'P5_AUTHORITY_ENROLLED_DEVICE'
  )
);

export const TrackingUnsupportedPlatformManualProofRowSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    rowId: TrackingUnsupportedPlatformManualProofRowIdSchema,
    platform: TrackingUnsupportedPlatformManualProofPlatformSchema,
    surface: TrackingUnsupportedPlatformManualProofSurfaceSchema,
    requiredProofTier: TrackingUnsupportedPlatformManualProofTierSchema,
    currentProofTier: TrackingUnsupportedPlatformManualProofTierSchema,
    supportState: TrackingPlatformProofRouteStateSchema,
    renderedState: TrackingUnsupportedPlatformManualProofRenderStateSchema,
    manualProofCommand: TrackingUnsupportedPlatformManualProofTextSchema,
    proofArtifactRefs: Schema.Array(TrackingPolicyAuditRefSchema),
    reasonCodes: Schema.Array(TrackingPolicyReasonCodeSchema),
    fakeCapabilityRendered: Schema.Literal(false),
    productClaimReady: Schema.Literal(false),
    runtimeLocationClaimed: Schema.Literal(false),
    backgroundLocationClaimed: Schema.Literal(false),
    geofenceRuntimeClaimed: Schema.Literal(false),
    physicalDeviceClaimed: Schema.Literal(false),
    authorityClaimed: Schema.Literal(false),
  })
    .pipe(
      Schema.filter(
        (row) =>
          row.renderedState !== 'authority-required' ||
          row.supportState === 'real-device-required' ||
          'Authority-required rows must stay tied to a proof-gated platform support state'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.supportState !== 'platform-unsupported' ||
          row.renderedState === 'unavailable' ||
          'Unsupported platforms must render unavailable instead of unproved tracking capability'
      )
    )
    .pipe(
      Schema.filter(
        (row) =>
          row.supportState !== 'manual-required' ||
          row.renderedState === 'manual-required' ||
          'Manual-required tracking states must render manual-required'
      )
    )
);

export const TrackingUnsupportedPlatformManualProofSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
    generatedAt: TrackingUnsupportedPlatformManualProofTextSchema,
    rows: Schema.Array(TrackingUnsupportedPlatformManualProofRowSchema),
    allRowsAvoidFakeCapability: Schema.Literal(true),
    allRowsKeepProductClaimBlocked: Schema.Literal(true),
    portalScreenshotClaimed: Schema.Literal(false),
    physicalDeviceProofClaimed: Schema.Literal(false),
    authorityProofClaimed: Schema.Literal(false),
  }).pipe(
    Schema.filter(
      (proof) =>
        (proof.rows.length >= 6 && proof.rows.every((row) => !row.fakeCapabilityRendered && !row.productClaimReady)) ||
        'Unsupported platform manual proof needs at least six non-claiming rows'
    )
  )
);

export type TrackingUnsupportedPlatformManualProofRow = Infer<typeof TrackingUnsupportedPlatformManualProofRowSchema>;
export type TrackingUnsupportedPlatformManualProof = Infer<typeof TrackingUnsupportedPlatformManualProofSchema>;

export function buildTrackingUnsupportedPlatformManualProof(
  generatedAt: string
): TrackingUnsupportedPlatformManualProof {
  return TrackingUnsupportedPlatformManualProofSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    generatedAt,
    rows: trackingUnsupportedPlatformManualProofRows(),
    allRowsAvoidFakeCapability: true,
    allRowsKeepProductClaimBlocked: true,
    portalScreenshotClaimed: false,
    physicalDeviceProofClaimed: false,
    authorityProofClaimed: false,
  });
}

export function trackingUnsupportedPlatformManualProofRows(): readonly TrackingUnsupportedPlatformManualProofRow[] {
  return [
    row(
      'tracking-android-background-manual',
      'android',
      'background-location',
      'P4_PHYSICAL_DEVICE',
      'manual-required'
    ),
    row('tracking-android-geofence-manual', 'android', 'geofence-transition', 'P4_PHYSICAL_DEVICE', 'manual-required'),
    row('tracking-ios-background-manual', 'ios', 'background-location', 'P4_PHYSICAL_DEVICE', 'manual-required'),
    row('tracking-ios-geofence-manual', 'ios', 'geofence-transition', 'P4_PHYSICAL_DEVICE', 'manual-required'),
    row(
      'tracking-desktop-os-location-manual',
      'windows',
      'desktop-os-location',
      'P3_LOCAL_DEV_MACHINE',
      'manual-required'
    ),
    row('tracking-web-child-agent-unavailable', 'web', 'child-agent-location', 'P4_PHYSICAL_DEVICE', 'unavailable'),
    row(
      'tracking-authority-hard-control-required',
      'android',
      'authority-hard-control',
      'P5_AUTHORITY_ENROLLED_DEVICE',
      'authority-required'
    ),
  ];
}

function row(
  rowId: string,
  platform: TrackingUnsupportedPlatformManualProofRow['platform'],
  surface: TrackingUnsupportedPlatformManualProofRow['surface'],
  requiredProofTier: TrackingUnsupportedPlatformManualProofRow['requiredProofTier'],
  renderedState: TrackingUnsupportedPlatformManualProofRow['renderedState']
): TrackingUnsupportedPlatformManualProofRow {
  return TrackingUnsupportedPlatformManualProofRowSchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    rowId,
    platform,
    surface,
    requiredProofTier,
    currentProofTier: currentTierFor(renderedState),
    supportState: supportStateFor(renderedState),
    renderedState,
    manualProofCommand: manualProofCommandFor(platform, surface, renderedState),
    proofArtifactRefs: [`tracking-${platform}-${surface}-manual-proof-plan`],
    reasonCodes: [`tracking-${platform}-${surface}-${renderedState}`],
    fakeCapabilityRendered: false,
    productClaimReady: false,
    runtimeLocationClaimed: false,
    backgroundLocationClaimed: false,
    geofenceRuntimeClaimed: false,
    physicalDeviceClaimed: false,
    authorityClaimed: false,
  });
}

function currentTierFor(
  renderedState: TrackingUnsupportedPlatformManualProofRow['renderedState']
): TrackingUnsupportedPlatformManualProofRow['currentProofTier'] {
  if (renderedState === 'unavailable') return 'P0_CONTRACT';
  return 'P3_LOCAL_DEV_MACHINE';
}

function supportStateFor(
  renderedState: TrackingUnsupportedPlatformManualProofRow['renderedState']
): TrackingUnsupportedPlatformManualProofRow['supportState'] {
  if (renderedState === 'unavailable') return 'platform-unsupported';
  if (renderedState === 'authority-required') return 'real-device-required';
  return 'manual-required';
}

function manualProofCommandFor(
  platform: TrackingUnsupportedPlatformManualProofRow['platform'],
  surface: TrackingUnsupportedPlatformManualProofRow['surface'],
  renderedState: TrackingUnsupportedPlatformManualProofRow['renderedState']
): string {
  if (renderedState === 'unavailable') {
    return `record ${platform} ${surface} unavailable tracking capability row`;
  }
  return `collect ${platform} ${surface} ${renderedState} tracking proof before product claim`;
}
