import { type Infer, Schema, withParser } from '@ocentra-parent/schema-domain/effect';
import { ParentControlCapabilityNameSchema, ParentControlCapabilityStatusSchema } from './capabilities';
import { ParentTimestampSchema } from './reference-primitives';
import {
  BrowserGameMobileCapabilityBoundarySchema,
  BrowserGameMobileCapabilityMatrixSchemaVersionSchema,
  BrowserGameMobileCapabilityReasonsSchema,
  type BrowserGameMobileCapabilitySurface,
  BrowserGameMobileCapabilitySurfaceSchema,
  BrowserGameMobileCapabilityStateSchema,
  BrowserGameMobilePlatformSchema,
  BrowserGameMobilePolicyScopeSchema,
  BrowserGameMobileProofRefsSchema,
  BrowserGameMobileProofStateSchema,
  BrowserGameMobileTargetKindSchema,
} from './browser-game-android-ios-capability-matrix-values';

const BrowserGameMobileCapabilityRowBaseSchema = Schema.Struct({
  platform: BrowserGameMobilePlatformSchema,
  surface: BrowserGameMobileCapabilitySurfaceSchema,
  targetKind: BrowserGameMobileTargetKindSchema,
  parentCapability: ParentControlCapabilityNameSchema,
  parentCapabilityStatus: ParentControlCapabilityStatusSchema,
  capabilityState: BrowserGameMobileCapabilityStateSchema,
  proofState: BrowserGameMobileProofStateSchema,
  policyScope: BrowserGameMobilePolicyScopeSchema,
  proofRefs: BrowserGameMobileProofRefsSchema,
  reasons: BrowserGameMobileCapabilityReasonsSchema,
  exactGameContentClaimed: Schema.Boolean,
  cloudStreamFrameAnalysisClaimed: Schema.Boolean,
  nativeGameControlClaimed: Schema.Boolean,
  nativeLauncherControlClaimed: Schema.Boolean,
  gameChatContentClaimed: Schema.Boolean,
  perGameCloudTitleClaimed: Schema.Boolean,
  runtimeSignalClaimed: Schema.Boolean,
  appStoreOrPurchaseControlClaimed: Schema.Boolean,
  uiDeliveredClaimed: Schema.Boolean,
  enforcementClaimed: Schema.Boolean,
});

type BrowserGameMobileCapabilityRowCandidate = Infer<typeof BrowserGameMobileCapabilityRowBaseSchema>;

export const BrowserGameMobileCapabilityRowSchema = withParser(
  BrowserGameMobileCapabilityRowBaseSchema.pipe(
    Schema.filter(
      (row) =>
        browserGameMobileCapabilityRowIsHonest(row) ||
        'Expected mobile browser-game capability row to stay manual-required, token-limited, entitlement-required, or unavailable without content, runtime, UI, native-game, cloud-title, or enforcement claims'
    )
  )
);

export const BrowserGameMobileCapabilityClaimBoundariesSchema = withParser(
  Schema.Struct({
    exactGameContent: Schema.Literal('not-claimed'),
    cloudStreamFrameAnalysis: Schema.Literal('not-claimed'),
    nativeGameControl: Schema.Literal('not-claimed'),
    nativeLauncherControl: Schema.Literal('not-claimed'),
    gameChatContent: Schema.Literal('not-claimed'),
    perGameCloudTitle: Schema.Literal('not-claimed'),
    runtimeSignals: Schema.Literal('not-claimed'),
    appStoreOrPurchaseControl: Schema.Literal('not-claimed'),
    uiDelivery: Schema.Literal('not-claimed'),
    enforcement: Schema.Literal('not-claimed'),
    reviewerSummary: BrowserGameMobileCapabilityBoundarySchema,
  })
);

const BrowserGameMobileCapabilityMatrixBaseSchema = Schema.Struct({
  schemaVersion: BrowserGameMobileCapabilityMatrixSchemaVersionSchema,
  generatedAt: ParentTimestampSchema,
  proofRefs: BrowserGameMobileProofRefsSchema,
  rows: Schema.Array(BrowserGameMobileCapabilityRowSchema),
  claimBoundaries: BrowserGameMobileCapabilityClaimBoundariesSchema,
});

type BrowserGameMobileCapabilityMatrixCandidate = Infer<typeof BrowserGameMobileCapabilityMatrixBaseSchema>;

export const BrowserGameMobileCapabilityMatrixSchema = withParser(
  BrowserGameMobileCapabilityMatrixBaseSchema.pipe(
    Schema.filter(
      (matrix) =>
        browserGameMobileCapabilityMatrixIsHonest(matrix) ||
        'Expected mobile browser-game capability matrix to include all Android/iOS surfaces exactly once without runtime or enforcement claims'
    )
  )
);

export const decodeBrowserGameMobileCapabilityMatrix = Schema.decodeUnknownSync(
  BrowserGameMobileCapabilityMatrixSchema
);

export type BrowserGameMobileCapabilityRow = Infer<typeof BrowserGameMobileCapabilityRowSchema>;
export type BrowserGameMobileCapabilityMatrix = Infer<typeof BrowserGameMobileCapabilityMatrixSchema>;

const RequiredBrowserGameMobileSurfaces = [
  'android-owned-browser-shell',
  'android-managed-webview-shell',
  'android-custom-tabs',
  'android-installed-browser-app',
  'android-cloud-gaming-browser-session',
  'android-device-owner-browser-policy',
  'ios-family-controls-authorization',
  'ios-safari-web-domain-token',
  'ios-application-token',
  'ios-managed-browser-shell',
  'ios-cloud-gaming-web-session',
  'ios-webclip-pwa',
] as const satisfies ReadonlyArray<BrowserGameMobileCapabilitySurface>;

function browserGameMobileCapabilityMatrixIsHonest(matrix: BrowserGameMobileCapabilityMatrixCandidate): boolean {
  const surfaces = new Set(matrix.rows.map((row) => row.surface));
  return (
    surfaces.size === matrix.rows.length && RequiredBrowserGameMobileSurfaces.every((surface) => surfaces.has(surface))
  );
}

function browserGameMobileCapabilityRowIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  if (browserGameMobileCapabilityRowClaimsRuntime(row)) {
    return false;
  }
  if (!browserGameMobileCapabilityRowHasExpectedPlatform(row)) {
    return false;
  }
  if (browserGameMobileCapabilityRowClaimsUnsupportedUpgrade(row)) {
    return false;
  }
  if (row.surface === 'android-owned-browser-shell' || row.surface === 'android-managed-webview-shell') {
    return (
      row.parentCapability === 'managed-browser-control' &&
      row.proofState === 'manual-device-proof-required' &&
      row.policyScope === 'owned-browser-shell-only' &&
      row.reasons.includes('owned-shell-required') &&
      row.reasons.includes('managed-browser-required')
    );
  }
  if (row.surface === 'android-custom-tabs') {
    return (
      row.parentCapability === 'managed-browser-control' &&
      row.policyScope === 'manual-review-only' &&
      row.reasons.includes('android-custom-tabs-not-owned') &&
      row.reasons.includes('unmanaged-exact-url-unavailable')
    );
  }
  if (row.surface === 'android-installed-browser-app') {
    return (
      row.parentCapability === 'package-lifecycle' &&
      row.policyScope === 'app-level-only' &&
      row.reasons.includes('unmanaged-exact-url-unavailable')
    );
  }
  if (row.surface === 'android-cloud-gaming-browser-session') {
    return browserGameMobileCloudSessionIsHonest(row);
  }
  if (row.surface === 'android-device-owner-browser-policy') {
    return (
      row.parentCapability === 'device-owner-policy' &&
      row.policyScope === 'manual-review-only' &&
      row.reasons.includes('device-owner-required')
    );
  }
  if (row.surface === 'ios-family-controls-authorization') {
    return (
      row.parentCapability === 'family-controls-entitlement' &&
      row.capabilityState === 'entitlement-required' &&
      row.proofState === 'platform-entitlement-required' &&
      row.policyScope === 'manual-review-only' &&
      row.reasons.includes('family-controls-entitlement-required')
    );
  }
  if (row.surface === 'ios-safari-web-domain-token') {
    return (
      row.parentCapability === 'screen-time-api' &&
      row.capabilityState === 'domain-token-limited' &&
      row.policyScope === 'web-domain-token-level' &&
      row.reasons.includes('web-domain-token-limited')
    );
  }
  if (row.surface === 'ios-application-token' || row.surface === 'ios-webclip-pwa') {
    return (
      row.parentCapability === 'family-controls-entitlement' &&
      row.policyScope === 'app-token-level' &&
      row.reasons.includes('opaque-application-token-required') &&
      row.reasons.includes('native-game-boundary')
    );
  }
  if (row.surface === 'ios-managed-browser-shell') {
    return (
      row.parentCapability === 'managed-browser-control' &&
      row.policyScope === 'owned-browser-shell-only' &&
      row.reasons.includes('owned-shell-required') &&
      row.reasons.includes('managed-browser-required')
    );
  }
  return browserGameMobileCloudSessionIsHonest(row);
}

function browserGameMobileCapabilityRowHasExpectedPlatform(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  if (row.surface.startsWith('android-')) {
    return row.platform === 'android';
  }
  if (row.surface.startsWith('ios-')) {
    return row.platform === 'ios';
  }
  return false;
}

function browserGameMobileCapabilityRowClaimsUnsupportedUpgrade(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return row.capabilityState === 'owned-browser-shell-capable-with-proof';
}

function browserGameMobileCloudSessionIsHonest(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.targetKind === 'cloud-gaming-web-session' &&
    row.capabilityState === 'manual-required' &&
    row.policyScope === 'manual-review-only' &&
    row.reasons.includes('cloud-title-unavailable') &&
    row.reasons.includes('content-frame-analysis-unavailable')
  );
}

function browserGameMobileCapabilityRowClaimsRuntime(row: BrowserGameMobileCapabilityRowCandidate): boolean {
  return (
    row.exactGameContentClaimed ||
    row.cloudStreamFrameAnalysisClaimed ||
    row.nativeGameControlClaimed ||
    row.nativeLauncherControlClaimed ||
    row.gameChatContentClaimed ||
    row.perGameCloudTitleClaimed ||
    row.runtimeSignalClaimed ||
    row.appStoreOrPurchaseControlClaimed ||
    row.uiDeliveredClaimed ||
    row.enforcementClaimed
  );
}
