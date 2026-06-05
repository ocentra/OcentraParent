import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { TrackingPolicyAuditRefSchema, TrackingPolicySchemaVersion } from './tracking-location-policy-primitives';

const TrackingPlatformManualProofPlatformSchema = withParser(
  Schema.Literal('android', 'ios', 'windows', 'macos', 'linux', 'web')
);

const TrackingPlatformManualProofCapabilitySchema = withParser(
  Schema.Literal(
    'foreground-location',
    'background-location',
    'geofence-transition',
    'device-status',
    'child-runtime-delivery'
  )
);

const TrackingPlatformManualProofStateSchema = withParser(
  Schema.Literal('manual-required', 'authority-required', 'unavailable', 'not-claimed', 'scaffold-observed')
);

const TrackingPlatformManualProofDisplayStateSchema = withParser(
  Schema.Literal('show-manual-required', 'show-unavailable', 'show-not-claimed', 'show-scaffold-only')
);

const TrackingPlatformManualProofReasonSchema = withParser(
  Schema.Literal(
    'android-physical-background-proof-missing',
    'ios-core-location-entitlement-proof-missing',
    'desktop-precise-location-not-implemented',
    'web-child-agent-not-supported',
    'child-runtime-delivery-not-proved',
    'emulator-scaffold-not-physical-proof'
  )
);

const TrackingPlatformManualStateProofRowBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  platform: TrackingPlatformManualProofPlatformSchema,
  capability: TrackingPlatformManualProofCapabilitySchema,
  proofState: TrackingPlatformManualProofStateSchema,
  displayState: TrackingPlatformManualProofDisplayStateSchema,
  reason: TrackingPlatformManualProofReasonSchema,
  proofArtifactRefs: Schema.Array(TrackingPolicyAuditRefSchema),
  productClaimReady: Schema.Literal(false),
});

export const TrackingPlatformManualStateProofRowSchema = withParser(TrackingPlatformManualStateProofRowBaseSchema);

const TrackingPlatformManualStateProofSummaryBaseSchema = Schema.Struct({
  schemaVersion: Schema.Literal(TrackingPolicySchemaVersion),
  workpackId: Schema.Literal('31-platform-extension-checklists-and-proof-routing'),
  productClaimReady: Schema.Literal(false),
  rowCount: Schema.Number,
  manualRequiredCount: Schema.Number,
  unavailableCount: Schema.Number,
  notClaimedCount: Schema.Number,
  scaffoldObservedCount: Schema.Number,
  fakeCapabilityRows: Schema.Array(Schema.String),
  proofArtifactRefs: Schema.Array(TrackingPolicyAuditRefSchema),
});

export const TrackingPlatformManualStateProofSummarySchema = withParser(
  TrackingPlatformManualStateProofSummaryBaseSchema
);

export type TrackingPlatformManualStateProofRow = Infer<typeof TrackingPlatformManualStateProofRowBaseSchema>;
export type TrackingPlatformManualStateProofSummary = Infer<typeof TrackingPlatformManualStateProofSummaryBaseSchema>;

const ProofRoot = 'output/tracking-plan-proof/31-platform-extension-checklists-and-proof-routing';
const PlatformManualStateProof = `${ProofRoot}/22-platform-manual-state-proof.json`;

export function trackingPlatformManualStateProofRows() {
  return TrackingPlatformManualStateProofRows.map((row) => TrackingPlatformManualStateProofRowSchema.parse(row));
}

export function summarizeTrackingPlatformManualStateProof(
  rows: readonly TrackingPlatformManualStateProofRow[] = trackingPlatformManualStateProofRows()
) {
  const fakeCapabilityRows = rows
    .filter((row) => row.productClaimReady)
    .map((row) => `${row.platform}:${row.capability}`);

  return TrackingPlatformManualStateProofSummarySchema.parse({
    schemaVersion: TrackingPolicySchemaVersion,
    workpackId: '31-platform-extension-checklists-and-proof-routing',
    productClaimReady: false,
    rowCount: rows.length,
    manualRequiredCount: rows.filter((row) => row.proofState === 'manual-required').length,
    unavailableCount: rows.filter((row) => row.proofState === 'unavailable').length,
    notClaimedCount: rows.filter((row) => row.proofState === 'not-claimed').length,
    scaffoldObservedCount: rows.filter((row) => row.proofState === 'scaffold-observed').length,
    fakeCapabilityRows,
    proofArtifactRefs: Array.from(new Set(rows.flatMap((row) => row.proofArtifactRefs))),
  });
}

const TrackingPlatformManualStateProofRows = [
  manualRow(
    'android',
    'background-location',
    'manual-required',
    'show-manual-required',
    'android-physical-background-proof-missing',
    PlatformManualStateProof
  ),
  manualRow(
    'android',
    'geofence-transition',
    'manual-required',
    'show-manual-required',
    'android-physical-background-proof-missing',
    PlatformManualStateProof
  ),
  manualRow(
    'android',
    'device-status',
    'scaffold-observed',
    'show-scaffold-only',
    'emulator-scaffold-not-physical-proof',
    'test-results/tracking-plan-android-emulator-proof/proof.json'
  ),
  manualRow(
    'ios',
    'foreground-location',
    'manual-required',
    'show-manual-required',
    'ios-core-location-entitlement-proof-missing',
    PlatformManualStateProof
  ),
  manualRow(
    'ios',
    'background-location',
    'manual-required',
    'show-manual-required',
    'ios-core-location-entitlement-proof-missing',
    PlatformManualStateProof
  ),
  manualRow(
    'ios',
    'geofence-transition',
    'manual-required',
    'show-manual-required',
    'ios-core-location-entitlement-proof-missing',
    PlatformManualStateProof
  ),
  manualRow(
    'windows',
    'foreground-location',
    'not-claimed',
    'show-not-claimed',
    'desktop-precise-location-not-implemented',
    PlatformManualStateProof
  ),
  manualRow(
    'macos',
    'foreground-location',
    'manual-required',
    'show-manual-required',
    'desktop-precise-location-not-implemented',
    PlatformManualStateProof
  ),
  manualRow(
    'linux',
    'foreground-location',
    'unavailable',
    'show-unavailable',
    'desktop-precise-location-not-implemented',
    PlatformManualStateProof
  ),
  manualRow(
    'web',
    'child-runtime-delivery',
    'unavailable',
    'show-unavailable',
    'web-child-agent-not-supported',
    PlatformManualStateProof
  ),
  manualRow(
    'android',
    'child-runtime-delivery',
    'not-claimed',
    'show-not-claimed',
    'child-runtime-delivery-not-proved',
    PlatformManualStateProof
  ),
  manualRow(
    'ios',
    'child-runtime-delivery',
    'not-claimed',
    'show-not-claimed',
    'child-runtime-delivery-not-proved',
    PlatformManualStateProof
  ),
] as const;

function manualRow(
  platform: TrackingPlatformManualStateProofRow['platform'],
  capability: TrackingPlatformManualStateProofRow['capability'],
  proofState: TrackingPlatformManualStateProofRow['proofState'],
  displayState: TrackingPlatformManualStateProofRow['displayState'],
  reason: TrackingPlatformManualStateProofRow['reason'],
  proofArtifactRef: string
) {
  return {
    schemaVersion: TrackingPolicySchemaVersion,
    platform,
    capability,
    proofState,
    displayState,
    reason,
    proofArtifactRefs: [proofArtifactRef],
    productClaimReady: false,
  } as const;
}
